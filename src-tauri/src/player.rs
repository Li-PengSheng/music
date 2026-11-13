//! Playback engine.
//!
//! Pipeline: file → Symphonia probe/decode → downmix to stereo → linear
//! resample to the output device's rate → interleaved output channels → ring
//! buffer → cpal/WASAPI callback → speakers.
//!
//! The UI's position comes from `frames_played`, which only the audio callback
//! increments, so the progress bar always reflects audio that really reached
//! the device. If the output device cannot be opened, playback refuses to
//! start and the error is reported instead of faking progress.

use std::collections::VecDeque;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;

use crate::logging::LogBuffer;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutputInfo {
    pub device: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: String,
}

#[derive(Clone, Debug)]
pub struct PlayerStatus {
    pub state: PlaybackState,
    pub position: Duration,
    pub duration: Duration,
    pub current_path: Option<String>,
    pub volume: f32,
    pub output: Option<OutputInfo>,
    pub error: Option<String>,
    pub underruns: u64,
}

/// Event sent from the engine to the UI.
#[derive(Clone, Debug)]
pub enum PlaybackEvent {
    /// A track played to its end (buffers drained).
    Finished,
    /// Something went wrong; the message is user-facing.
    Failed(String),
}

enum Cmd {
    Play {
        path: PathBuf,
        duration_ms: u64,
    },
    /// Load a track paused at a position (session restore). Nothing is heard
    /// until the person presses play.
    Prepare {
        path: PathBuf,
        duration_ms: u64,
        position_ms: u64,
    },
    Pause,
    Resume,
    Seek {
        seconds: f64,
    },
    Stop,
    SetVolume {
        volume: f32,
    },
    ClearError,
    Synchronize(Sender<PlayerStatus>),
    Shutdown(Sender<()>),
}

pub struct Player {
    cmd_tx: Sender<Cmd>,
    event_rx: Receiver<PlaybackEvent>,
    status: Arc<Mutex<PlayerStatus>>,
    logs: Arc<LogBuffer>,
}

impl Player {
    pub fn new(log_path: Option<PathBuf>) -> Player {
        let logs = Arc::new(LogBuffer::new(log_path));
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let status = Arc::new(Mutex::new(PlayerStatus {
            state: PlaybackState::Stopped,
            position: Duration::ZERO,
            duration: Duration::ZERO,
            current_path: None,
            volume: 1.0,
            output: None,
            error: None,
            underruns: 0,
        }));
        let status_thread = status.clone();
        let logs_thread = logs.clone();
        std::thread::Builder::new()
            .name("audio-engine".into())
            .spawn(move || engine_loop(cmd_rx, event_tx, status_thread, logs_thread))
            .expect("spawn audio engine");
        Player {
            cmd_tx,
            event_rx,
            status,
            logs,
        }
    }

    /// FIFO barrier acknowledged after publishing the actual engine status.
    pub fn synchronize(&self) -> Result<PlayerStatus, String> {
        let (tx, rx) = mpsc::channel();
        self.cmd_tx
            .send(Cmd::Synchronize(tx))
            .map_err(|_| "音频线程已退出".to_string())?;
        rx.recv_timeout(Duration::from_secs(15))
            .map_err(|_| "音频引擎响应超时".to_string())
    }

    pub fn status(&self) -> PlayerStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn poll(&self) -> Option<PlaybackEvent> {
        self.event_rx.try_recv().ok()
    }

    pub fn log_lines(&self) -> Vec<String> {
        self.logs.lines()
    }

    pub fn play(&self, path: PathBuf, duration_ms: u64) {
        let _ = self.cmd_tx.send(Cmd::Play { path, duration_ms });
    }
    /// Load a track paused at `position_ms` without producing sound.
    pub fn prepare(&self, path: PathBuf, duration_ms: u64, position_ms: u64) {
        let _ = self.cmd_tx.send(Cmd::Prepare {
            path,
            duration_ms,
            position_ms,
        });
    }
    pub fn pause(&self) {
        let _ = self.cmd_tx.send(Cmd::Pause);
    }
    pub fn resume(&self) {
        let _ = self.cmd_tx.send(Cmd::Resume);
    }
    pub fn seek(&self, seconds: f64) {
        let _ = self.cmd_tx.send(Cmd::Seek { seconds });
    }
    pub fn shutdown(&self) -> Result<(), String> {
        let (tx, rx) = mpsc::channel();
        self.cmd_tx
            .send(Cmd::Shutdown(tx))
            .map_err(|_| "音频线程已退出".to_string())?;
        rx.recv_timeout(Duration::from_secs(15))
            .map_err(|_| "释放音频资源超时".to_string())
    }
    pub fn stop(&self) {
        let _ = self.cmd_tx.send(Cmd::Stop);
    }
    pub fn set_volume(&self, volume: f32) {
        let _ = self.cmd_tx.send(Cmd::SetVolume { volume });
    }
    pub fn clear_error(&self) {
        let _ = self.cmd_tx.send(Cmd::ClearError);
    }
}

// ---------------------------------------------------------------- shared state

struct Shared {
    ring: Mutex<VecDeque<f32>>,
    capacity: usize,
    channels: usize,
    rate: u32,
    volume_bits: AtomicU32,
    paused: AtomicBool,
    /// True while a track is loaded (playing or draining its tail). The
    /// playback clock only advances then.
    clock_on: AtomicBool,
    /// True while audio is expected, so the callback can count underruns.
    active: AtomicBool,
    frames_played: AtomicU64,
    underruns: AtomicU64,
    stream_dead: AtomicBool,
    capture: Mutex<Option<Vec<f32>>>,
    peak_bits: AtomicU32,
}

impl Shared {
    fn new(rate: u32, channels: usize, volume: f32) -> Shared {
        let capacity = (rate as usize * channels * 3) / 10; // 300 ms
        Shared {
            ring: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
            channels,
            rate,
            volume_bits: AtomicU32::new(volume.to_bits()),
            paused: AtomicBool::new(false),
            clock_on: AtomicBool::new(false),
            active: AtomicBool::new(false),
            frames_played: AtomicU64::new(0),
            underruns: AtomicU64::new(0),
            stream_dead: AtomicBool::new(false),
            capture: Mutex::new(None),
            peak_bits: AtomicU32::new(0.0f32.to_bits()),
        }
    }

    fn clear_ring(&self) {
        self.ring.lock().unwrap().clear();
    }

    fn fill(&self) -> usize {
        self.ring.lock().unwrap().len()
    }

    fn reset_clock(&self) {
        self.frames_played.store(0, Ordering::SeqCst);
    }
}

struct Output {
    // The stream stops when dropped, so it must stay alive with the engine.
    _stream: cpal::Stream,
    shared: Arc<Shared>,
    info: OutputInfo,
}

fn open_output(logs: &Arc<LogBuffer>, volume: f32) -> Result<Output, String> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| "系统没有可用的音频输出设备".to_string())?;
    let device_name = device.name().unwrap_or_else(|_| "未知设备".to_string());
    logs.push(format!("音频输出设备: {device_name}"));

    // The device's own default config is what Windows uses in shared mode and
    // is the most reliable choice; fall back to every supported config.
    let mut candidates: Vec<cpal::SupportedStreamConfig> = Vec::new();
    match device.default_output_config() {
        Ok(cfg) => {
            logs.push(format!(
                "设备默认格式: {} Hz, {} 声道, {:?}",
                cfg.sample_rate().0,
                cfg.channels(),
                cfg.sample_format()
            ));
            candidates.push(cfg);
        }
        Err(e) => logs.push(format!("读取设备默认格式失败: {e}")),
    }
    if let Ok(list) = device.supported_output_configs() {
        for c in list {
            let rate = 48_000u32.clamp(c.min_sample_rate().0, c.max_sample_rate().0);
            let cfg = c.with_sample_rate(cpal::SampleRate(rate));
            if !candidates.iter().any(|x| {
                x.sample_rate() == cfg.sample_rate()
                    && x.channels() == cfg.channels()
                    && x.sample_format() == cfg.sample_format()
            }) {
                candidates.push(cfg);
            }
        }
    }
    // Prefer float, stereo, 48 kHz — but only after the device default.
    candidates.sort_by_key(|c| {
        (
            c.sample_format() != SampleFormat::F32,
            c.channels() != 2,
            c.sample_rate().0 != 48_000,
        )
    });

    let mut last_err = String::from("没有可尝试的输出格式");
    for cand in candidates {
        let format = cand.sample_format();
        let config = cand.config();
        let shared = Arc::new(Shared::new(
            config.sample_rate.0,
            config.channels as usize,
            volume,
        ));
        let built = match format {
            SampleFormat::F32 => build::<f32>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::F64 => build::<f64>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::I16 => build::<i16>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::I32 => build::<i32>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::I8 => build::<i8>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::U8 => build::<u8>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::U16 => build::<u16>(&device, &config, shared.clone(), logs.clone()),
            SampleFormat::U32 => build::<u32>(&device, &config, shared.clone(), logs.clone()),
            other => {
                last_err = format!("不支持的采样格式: {other}");
                continue;
            }
        };
        match built {
            Ok(stream) => {
                if let Err(e) = stream.play() {
                    last_err = format!("启动音频流失败: {e}");
                    logs.push(format!("{last_err} ({format:?})"));
                    continue;
                }
                logs.push(format!(
                    "音频流已启动: {} Hz, {} 声道, {format:?}",
                    config.sample_rate.0, config.channels
                ));
                return Ok(Output {
                    _stream: stream,
                    shared,
                    info: OutputInfo {
                        device: device_name,
                        sample_rate: config.sample_rate.0,
                        channels: config.channels,
                        sample_format: format!("{format:?}"),
                    },
                });
            }
            Err(e) => {
                last_err = e;
                logs.push(format!("无法用 {format:?} 打开音频流: {last_err}"));
            }
        }
    }
    Err(format!("无法启动音频输出: {last_err}"))
}

/// Result of loading a track into the engine.
struct Started {
    decoder: Option<Decoder>,
    draining_since: Option<Instant>,
    log_line: String,
}

/// Open a file, seek to `position`, pre-fill the device buffer and put the
/// engine in the requested state. Shared by Play and Prepare (session restore).
fn begin_track(
    output: &mut Option<Output>,
    logs: &Arc<LogBuffer>,
    volume: f32,
    path: &Path,
    duration_ms: u64,
    position: Duration,
    paused: bool,
) -> Result<Started, String> {
    if output.is_none() {
        *output = Some(open_output(logs, volume)?);
    }
    let out = output.as_ref().unwrap();
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut d = Decoder::open(path, out.shared.rate, out.shared.channels)
        .map_err(|e| format!("无法播放「{name}」：{e}"))?;

    let log_line = format!(
        "开始播放: {name} | {} | {} Hz | {} 声道 | 预计 {} ms | 输出 {} Hz / {} 声道 | 起点 {:.1}s",
        d.codec,
        d.source_rate,
        d.source_channels,
        duration_ms,
        out.shared.rate,
        out.shared.channels,
        position.as_secs_f64()
    );

    out.shared.paused.store(true, Ordering::SeqCst);
    out.shared.clear_ring();
    out.shared.reset_clock();
    if position > Duration::ZERO {
        let secs = position.as_secs_f64();
        d.seek(secs)?;
        out.shared
            .frames_played
            .store((secs * out.shared.rate as f64) as u64, Ordering::SeqCst);
    }

    // Hold the device while the first chunk is decoded so playback never
    // starts on an empty buffer.
    out.shared.paused.store(true, Ordering::SeqCst);
    let target = out.shared.capacity;
    let mut filled = 0usize;
    let mut hit_end = false;
    while filled < target {
        match d.next_output() {
            Some(chunk) => {
                filled += chunk.len();
                let mut ring = out.shared.ring.lock().unwrap();
                ring.extend(chunk);
            }
            None => {
                hit_end = true;
                break;
            }
        }
    }
    if filled == 0 {
        // Probed fine but produced no audio: report it instead of pretending.
        out.shared.paused.store(false, Ordering::SeqCst);
        out.shared.clock_on.store(false, Ordering::SeqCst);
        out.shared.active.store(false, Ordering::SeqCst);
        out.shared.clear_ring();
        return Err(format!("无法播放「{name}」：文件中没有可解码的音频数据"));
    }
    out.shared.clock_on.store(true, Ordering::SeqCst);
    out.shared.active.store(!paused, Ordering::SeqCst);
    out.shared.paused.store(paused, Ordering::SeqCst);
    Ok(Started {
        decoder: if hit_end { None } else { Some(d) },
        draining_since: if hit_end { Some(Instant::now()) } else { None },
        log_line,
    })
}

fn build<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    shared: Arc<Shared>,
    logs: Arc<LogBuffer>,
) -> Result<cpal::Stream, String>
where
    T: SizedSample + FromSample<f32> + Send + 'static,
{
    let error_shared = shared.clone();
    let error_logs = logs;
    let err_fn = move |e: cpal::StreamError| {
        error_logs.push(format!("音频流错误: {e}"));
        error_shared.stream_dead.store(true, Ordering::SeqCst);
    };
    let data_fn = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        fill_output(data, &shared);
    };
    device
        .build_output_stream(config, data_fn, err_fn, None)
        .map_err(|e| e.to_string())
}

/// The audio callback. Public only so the self-test can drive it.
fn fill_output<T>(data: &mut [T], shared: &Shared)
where
    T: SizedSample + FromSample<f32>,
{
    let channels = shared.channels.max(1);
    if shared.paused.load(Ordering::Relaxed) {
        for s in data.iter_mut() {
            *s = T::from_sample(0.0f32);
        }
        return;
    }
    let volume = f32::from_bits(shared.volume_bits.load(Ordering::Relaxed)).clamp(0.0, 1.0);
    let mut peak = 0.0f32;
    let mut missing = 0usize;
    let capturing = shared.capture.lock().map(|c| c.is_some()).unwrap_or(false);
    let mut captured: Vec<f32> = if capturing {
        Vec::with_capacity(data.len())
    } else {
        Vec::new()
    };
    {
        let mut ring = shared.ring.lock().unwrap();
        for s in data.iter_mut() {
            match ring.pop_front() {
                Some(v) => {
                    let v = (v * volume).clamp(-1.0, 1.0);
                    peak = peak.max(v.abs());
                    *s = T::from_sample(v);
                    if capturing {
                        captured.push(v);
                    }
                }
                None => {
                    missing += 1;
                    *s = T::from_sample(0.0f32);
                    if capturing {
                        captured.push(0.0);
                    }
                }
            }
        }
    }
    if missing > 0 && shared.active.load(Ordering::Relaxed) {
        shared.underruns.fetch_add(1, Ordering::Relaxed);
    }
    if capturing {
        if let Ok(mut cap) = shared.capture.lock() {
            if let Some(buf) = cap.as_mut() {
                let room = 480_000usize.saturating_sub(buf.len());
                buf.extend(captured.into_iter().take(room));
            }
        }
    }
    shared.peak_bits.store(peak.to_bits(), Ordering::Relaxed);
    if shared.clock_on.load(Ordering::Relaxed) {
        shared
            .frames_played
            .fetch_add((data.len() / channels) as u64, Ordering::Relaxed);
    }
}

// ------------------------------------------------------------------- engine

fn engine_loop(
    cmd_rx: Receiver<Cmd>,
    events: Sender<PlaybackEvent>,
    status: Arc<Mutex<PlayerStatus>>,
    logs: Arc<LogBuffer>,
) {
    let mut output: Option<Output> = None;
    let mut decoder: Option<Decoder> = None;
    let mut state = PlaybackState::Stopped;
    let mut duration = Duration::ZERO;
    let mut current_path: Option<String> = None;
    let mut volume = 1.0f32;
    let mut error: Option<String> = None;
    let mut draining_since: Option<Instant> = None;

    // Aggregate decoded levels for a once-a-second log line.
    let mut window_start = Instant::now();
    let mut win_peak = 0.0f32;
    let mut win_sq = 0.0f64;
    let mut win_n = 0u64;

    logs.push("播放引擎已启动");

    // Open the device up front so a missing/blocked output is visible before
    // the person presses play, instead of failing silently later.
    match open_output(&logs, volume) {
        Ok(o) => output = Some(o),
        Err(e) => {
            logs.push(format!("音频输出不可用: {e}"));
            error = Some(format!("音频输出不可用：{e}"));
        }
    }

    loop {
        let mut acknowledgement = None;
        // ------------------------------------------------------ commands
        loop {
            match cmd_rx.try_recv() {
                Ok(Cmd::Play { path, duration_ms }) => {
                    match begin_track(
                        &mut output,
                        &logs,
                        volume,
                        &path,
                        duration_ms,
                        Duration::ZERO,
                        false,
                    ) {
                        Ok(started) => {
                            logs.push(started.log_line);
                            decoder = started.decoder;
                            draining_since = started.draining_since;
                            state = PlaybackState::Playing;
                            duration = Duration::from_millis(duration_ms);
                            current_path = Some(path.to_string_lossy().to_string());
                            error = None;
                            window_start = Instant::now();
                            win_peak = 0.0;
                            win_sq = 0.0;
                            win_n = 0;
                        }
                        Err(msg) => {
                            logs.push(msg.clone());
                            error = Some(msg.clone());
                            decoder = None;
                            state = PlaybackState::Stopped;
                            duration = Duration::ZERO;
                            current_path = None;
                            let _ = events.send(PlaybackEvent::Failed(msg));
                        }
                    }
                }
                Ok(Cmd::Prepare {
                    path,
                    duration_ms,
                    position_ms,
                }) => {
                    let position = Duration::from_millis(position_ms);
                    match begin_track(
                        &mut output,
                        &logs,
                        volume,
                        &path,
                        duration_ms,
                        position,
                        true,
                    ) {
                        Ok(started) => {
                            logs.push(format!("恢复会话: {}", started.log_line));
                            decoder = started.decoder;
                            draining_since = started.draining_since;
                            state = PlaybackState::Paused;
                            duration = Duration::from_millis(duration_ms);
                            current_path = Some(path.to_string_lossy().to_string());
                            error = None;
                        }
                        Err(msg) => {
                            logs.push(msg.clone());
                            error = Some(msg.clone());
                            decoder = None;
                            state = PlaybackState::Stopped;
                            duration = Duration::ZERO;
                            current_path = None;
                            let _ = events.send(PlaybackEvent::Failed(msg));
                        }
                    }
                }
                Ok(Cmd::Pause) => {
                    if state == PlaybackState::Playing {
                        state = PlaybackState::Paused;
                        if let Some(o) = &output {
                            o.shared.paused.store(true, Ordering::SeqCst);
                            o.shared.active.store(false, Ordering::SeqCst);
                        }
                        logs.push("已暂停");
                    }
                }
                Ok(Cmd::Resume) => {
                    if state == PlaybackState::Paused {
                        state = PlaybackState::Playing;
                        if let Some(o) = &output {
                            o.shared.paused.store(false, Ordering::SeqCst);
                            o.shared.active.store(true, Ordering::SeqCst);
                        }
                        logs.push("继续播放");
                    }
                }
                Ok(Cmd::Seek { seconds }) => {
                    if let Some(path) = current_path.as_ref() {
                        let target = seconds.max(0.0);
                        if duration > Duration::ZERO && target >= duration.as_secs_f64() {
                            decoder = None;
                            draining_since = None;
                            state = PlaybackState::Stopped;
                            error = None;
                            if let Some(o) = &output {
                                o.shared.paused.store(true, Ordering::SeqCst);
                                o.shared.clear_ring();
                                o.shared.clock_on.store(false, Ordering::SeqCst);
                                o.shared.active.store(false, Ordering::SeqCst);
                                o.shared.frames_played.store(
                                    (duration.as_secs_f64() * o.shared.rate as f64) as u64,
                                    Ordering::SeqCst,
                                );
                            }
                            continue;
                        }
                        match begin_track(
                            &mut output,
                            &logs,
                            volume,
                            Path::new(path),
                            duration.as_millis() as u64,
                            Duration::from_secs_f64(target),
                            state == PlaybackState::Paused,
                        ) {
                            Ok(started) => {
                                decoder = started.decoder;
                                draining_since = started.draining_since;
                                error = None;
                                logs.push(format!("跳转到 {target:.1}s"));
                            }
                            Err(msg) => {
                                state = PlaybackState::Stopped;
                                decoder = None;
                                error = Some(msg.clone());
                                let _ = events.send(PlaybackEvent::Failed(msg));
                            }
                        }
                    } else {
                        error = Some("歌曲已进入结尾，请重新播放后跳转".into());
                    }
                }
                Ok(Cmd::Shutdown(reply)) => {
                    drop(decoder);
                    drop(output);
                    let _ = reply.send(());
                    return;
                }
                Ok(Cmd::Stop) => {
                    decoder = None;
                    state = PlaybackState::Stopped;
                    duration = Duration::ZERO;
                    current_path = None;
                    draining_since = None;
                    if let Some(o) = &output {
                        o.shared.clear_ring();
                        o.shared.reset_clock();
                        o.shared.paused.store(false, Ordering::SeqCst);
                        o.shared.clock_on.store(false, Ordering::SeqCst);
                        o.shared.active.store(false, Ordering::SeqCst);
                    }
                }
                Ok(Cmd::SetVolume { volume: v }) => {
                    volume = v.clamp(0.0, 1.0);
                    if let Some(o) = &output {
                        o.shared
                            .volume_bits
                            .store(volume.to_bits(), Ordering::Relaxed);
                    }
                }
                Ok(Cmd::ClearError) => {
                    error = None;
                }
                Ok(Cmd::Synchronize(reply)) => {
                    acknowledgement = Some(reply);
                    break;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }

        // Failed switches/seeks must not drain a previous song as if it succeeded.
        if state == PlaybackState::Stopped && error.is_some() {
            draining_since = None;
            if let Some(o) = &output {
                o.shared.paused.store(true, Ordering::SeqCst);
                o.shared.clock_on.store(false, Ordering::SeqCst);
                o.shared.active.store(false, Ordering::SeqCst);
                o.shared.clear_ring();
                o.shared.reset_clock();
            }
        }

        // A dead stream (device removed, format lost) must not look like silence.
        if let Some(o) = &output {
            if o.shared.stream_dead.swap(false, Ordering::SeqCst) {
                let msg = format!("音频输出已中断（{}），请检查设备后重试。", o.info.device);
                logs.push(msg.clone());
                error = Some(msg.clone());
                state = PlaybackState::Stopped;
                decoder = None;
                output = None;
                let _ = events.send(PlaybackEvent::Failed(msg));
            }
        }

        // -------------------------------------------------------- decoding
        if state == PlaybackState::Playing {
            if let Some(o) = &output {
                if decoder.is_some() && o.shared.fill() < o.shared.capacity {
                    let chunk = decoder.as_mut().and_then(|d| d.next_output());
                    match chunk {
                        Some(samples) => {
                            let mut ring = o.shared.ring.lock().unwrap();
                            for s in &samples {
                                ring.push_back(*s);
                                win_peak = win_peak.max(s.abs());
                                win_sq += (*s as f64) * (*s as f64);
                            }
                            win_n += samples.len() as u64;
                        }
                        None => {
                            // End of stream: let the buffered tail play out.
                            // No more audio is coming, so stop counting
                            // underruns for the (intentionally) empty buffer.
                            draining_since = Some(Instant::now());
                            decoder = None;
                            o.shared.active.store(false, Ordering::SeqCst);
                        }
                    }
                } else {
                    std::thread::sleep(Duration::from_millis(2));
                }
            }
        }

        // -------------------------------------------------------- draining
        if state == PlaybackState::Playing && draining_since.is_some() {
            let empty = output
                .as_ref()
                .map(|o| o.shared.fill() == 0)
                .unwrap_or(true);
            if empty {
                logs.push("播放到结尾");
                draining_since = None;
                state = PlaybackState::Stopped;
                if let Some(o) = &output {
                    o.shared.active.store(false, Ordering::SeqCst);
                    o.shared.clock_on.store(false, Ordering::SeqCst);
                }
                let _ = events.send(PlaybackEvent::Finished);
            }
        }

        // ---------------------------------------------------- level logging
        if state == PlaybackState::Playing && window_start.elapsed() >= Duration::from_secs(1) {
            if win_n > 0 {
                let rms = (win_sq / win_n as f64).sqrt() as f32;
                let (buffered, capacity) = output
                    .as_ref()
                    .map(|o| {
                        (
                            (o.shared.fill() / o.shared.channels.max(1))
                                .min(o.shared.capacity / o.shared.channels.max(1)),
                            o.shared.capacity / o.shared.channels.max(1),
                        )
                    })
                    .unwrap_or((0, 0));
                logs.push(format!(
                    "PCM 电平: 峰值 {:.3} / RMS {:.3} | 缓冲区 {buffered}/{capacity} 帧 | 欠载 {}",
                    win_peak,
                    rms,
                    output
                        .as_ref()
                        .map(|o| o.shared.underruns.load(Ordering::Relaxed))
                        .unwrap_or(0)
                ));
            }
            window_start = Instant::now();
            win_peak = 0.0;
            win_sq = 0.0;
            win_n = 0;
        }

        // --------------------------------------------------------- status
        {
            let (position, out_info, underruns) = match &output {
                Some(o) => (
                    Duration::from_secs_f64(
                        o.shared.frames_played.load(Ordering::Relaxed) as f64
                            / o.shared.rate as f64,
                    ),
                    Some(o.info.clone()),
                    o.shared.underruns.load(Ordering::Relaxed),
                ),
                None => (Duration::ZERO, None, 0),
            };
            let position = if duration > Duration::ZERO && position > duration {
                duration
            } else {
                position
            };
            let mut st = status.lock().unwrap();
            *st = PlayerStatus {
                state,
                position,
                duration,
                current_path: current_path.clone(),
                volume,
                output: out_info,
                error: error.clone(),
                underruns,
            };
        }

        if let Some(reply) = acknowledgement {
            let _ = reply.send(status.lock().unwrap().clone());
        }
        std::thread::sleep(Duration::from_millis(3));
    }
}

// ------------------------------------------------------------------ decoder

struct Decoder {
    format: Box<dyn symphonia::core::formats::FormatReader>,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    track_id: u32,
    source_rate: u32,
    source_channels: usize,
    out_rate: u32,
    out_channels: usize,
    resampler: StreamResampler,
    codec: String,
}

impl Decoder {
    fn open(path: &Path, out_rate: u32, out_channels: usize) -> Result<Decoder, String> {
        let file = File::open(path).map_err(|e| format!("打开文件失败: {e}"))?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }
        let probed = symphonia::default::get_probe()
            .format(
                &hint,
                mss,
                &FormatOptions {
                    enable_gapless: true,
                    ..Default::default()
                },
                &MetadataOptions::default(),
            )
            .map_err(|e| format!("无法识别的音频格式: {e}"))?;
        let format = probed.format;
        let track = format
            .default_track()
            .ok_or_else(|| "文件中没有音频轨道".to_string())?;
        let track_id = track.id;
        let params = track.codec_params.clone();
        let codec = format!("{:?}", params.codec);
        let source_rate = params.sample_rate.unwrap_or(44_100);
        let source_channels = params.channels.map(|c| c.count()).unwrap_or(2);
        let decoder = symphonia::default::get_codecs()
            .make(&params, &DecoderOptions::default())
            .map_err(|e| format!("无法创建解码器: {e}"))?;
        Ok(Decoder {
            format,
            decoder,
            track_id,
            source_rate,
            source_channels,
            out_rate,
            out_channels: out_channels.max(1),
            resampler: StreamResampler::new(source_rate as f64, out_rate as f64),
            codec,
        })
    }

    fn seek(&mut self, seconds: f64) -> Result<(), String> {
        let secs = seconds.max(0.0);
        let time = Time::new(secs as u64, secs.fract());
        self.format
            .seek(
                SeekMode::Accurate,
                SeekTo::Time {
                    time,
                    track_id: Some(self.track_id),
                },
            )
            .map_err(|e| format!("跳转失败：{e}"))?;
        self.decoder.reset();
        self.resampler = StreamResampler::new(self.source_rate as f64, self.out_rate as f64);
        Ok(())
    }

    /// Decode packets until some audio is ready. Returns interleaved samples in
    /// the output channel layout, or None at end of stream.
    fn next_output(&mut self) -> Option<Vec<f32>> {
        loop {
            let packet = match self.format.next_packet() {
                Ok(p) => p,
                Err(_) => return None,
            };
            if packet.track_id() != self.track_id {
                continue;
            }
            let decoded = match self.decoder.decode(&packet) {
                Ok(d) => d,
                // A damaged packet is not the end of the file.
                Err(e) => {
                    eprintln!("[player] decode error (skipping packet): {e}");
                    continue;
                }
            };
            let spec = *decoded.spec();
            let frames = decoded.frames();
            if frames == 0 {
                continue;
            }
            let channels = spec.channels.count().max(1);
            let mut sbuf = SampleBuffer::<f32>::new(frames as u64, spec);
            sbuf.copy_interleaved_ref(decoded);
            let samples = sbuf.samples();

            // Downmix to stereo, resample, then map to the device's channels.
            let mut out: Vec<f32> = Vec::new();
            let mut stage = Vec::with_capacity(frames * 2);
            for f in 0..frames {
                let base = f * channels;
                let (l, r) = match channels {
                    1 => {
                        let m = samples[base];
                        (m, m)
                    }
                    2 => (samples[base], samples[base + 1]),
                    _ => {
                        let c = samples.get(base + 2).copied().unwrap_or(0.0);
                        (samples[base] + 0.707 * c, samples[base + 1] + 0.707 * c)
                    }
                };
                self.resampler.process([l, r], &mut stage);
            }
            match self.out_channels {
                1 => {
                    for pair in stage.as_chunks::<2>().0 {
                        out.push((pair[0] + pair[1]) * 0.5);
                    }
                }
                2 => out = stage,
                n => {
                    for pair in stage.as_chunks::<2>().0 {
                        out.push(pair[0]);
                        out.push(pair[1]);
                        out.extend(std::iter::repeat_n(0.0, n - 2));
                    }
                }
            }
            if !out.is_empty() {
                return Some(out);
            }
        }
    }
}

/// Streaming linear resampler (stereo). Carries the previous frame across calls.
struct StreamResampler {
    ratio: f64,
    next_t: f64,
    prev: Option<[f32; 2]>,
    i: u64,
}

impl StreamResampler {
    fn new(in_rate: f64, out_rate: f64) -> Self {
        Self {
            ratio: in_rate / out_rate,
            next_t: 0.0,
            prev: None,
            i: 0,
        }
    }

    fn process(&mut self, cur: [f32; 2], out: &mut Vec<f32>) {
        match self.prev {
            None => self.prev = Some(cur),
            Some(p) => {
                while self.next_t < self.i as f64 {
                    let local = (self.next_t - (self.i as f64 - 1.0)) as f32;
                    let l = p[0] + (cur[0] - p[0]) * local;
                    let r = p[1] + (cur[1] - p[1]) * local;
                    out.push(l);
                    out.push(r);
                    self.next_t += self.ratio;
                }
                self.prev = Some(cur);
            }
        }
        self.i += 1;
    }
}

// ------------------------------------------------------- diagnostics API

#[derive(Clone, Debug)]
pub struct AudioReport {
    pub codec: String,
    pub sample_rate: u32,
    pub channels: usize,
    pub frames_out: u64,
    pub seconds: f32,
    pub peak: f32,
    pub rms: f32,
}

impl AudioReport {
    pub fn is_silent(&self) -> bool {
        self.peak < 0.001
    }
}

/// Decode a file with the real engine decoder (no device needed) and measure
/// the PCM it produces. Used by tests and troubleshooting.
pub fn analyze_file(path: &Path, max_seconds: f32) -> Result<AudioReport, String> {
    let mut d = Decoder::open(path, 48_000, 2)?;
    let limit = (max_seconds.max(0.1) * 48_000.0 * 2.0) as usize;
    let mut peak = 0.0f32;
    let mut sum_sq = 0.0f64;
    let mut n = 0usize;
    let mut frames = 0u64;
    while n < limit {
        match d.next_output() {
            Some(chunk) => {
                for s in &chunk {
                    peak = peak.max(s.abs());
                    sum_sq += (*s as f64) * (*s as f64);
                }
                n += chunk.len();
                frames += (chunk.len() / 2) as u64;
            }
            None => break,
        }
    }
    Ok(AudioReport {
        codec: d.codec.clone(),
        sample_rate: d.source_rate,
        channels: d.source_channels,
        frames_out: frames,
        seconds: frames as f32 / 48_000.0,
        peak,
        rms: if n > 0 {
            (sum_sq / n as f64).sqrt() as f32
        } else {
            0.0
        },
    })
}

#[derive(Clone, Debug)]
pub struct SelfTestReport {
    pub device: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub format: String,
    pub frames_delivered: u64,
    pub peak: f32,
}

/// Open the real output device, feed it a 440 Hz tone through the engine's
/// ring buffer, and report what the audio callback actually wrote to the
/// device. Proves audio data reaches WASAPI (it cannot prove the speakers are
/// turned up).
pub fn output_selftest(millis: u64) -> Result<SelfTestReport, String> {
    let logs = Arc::new(LogBuffer::new(None));
    let out = open_output(&logs, 1.0)?;
    // Route the callback's samples into a capture buffer for inspection.
    out.shared
        .capture
        .lock()
        .unwrap()
        .replace(Vec::with_capacity(48_000 * 2));

    let rate = out.shared.rate as f32;
    let channels = out.shared.channels;
    // Pre-fill ~50 ms of tone so the first callbacks are not silent.
    {
        let mut ring = out.shared.ring.lock().unwrap();
        for i in 0..(rate as usize * channels / 20) {
            let t = i as f32 / (rate * channels as f32);
            ring.push_back((2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5);
        }
    }
    out.shared.paused.store(false, Ordering::SeqCst);
    out.shared.clock_on.store(true, Ordering::SeqCst);
    out.shared.active.store(true, Ordering::SeqCst);

    // Keep the ring fed while the device pulls frames in real time.
    let deadline = Instant::now() + Duration::from_millis(millis);
    let mut t = 0.0f32; // seconds
    while Instant::now() < deadline {
        {
            let mut ring = out.shared.ring.lock().unwrap();
            while ring.len() + channels <= out.shared.capacity {
                let v = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
                for _ in 0..channels {
                    ring.push_back(v);
                }
                t += 1.0 / rate;
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    let captured = out
        .shared
        .capture
        .lock()
        .unwrap()
        .take()
        .unwrap_or_default();
    let peak = captured.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    Ok(SelfTestReport {
        device: out.info.device.clone(),
        sample_rate: out.info.sample_rate,
        channels: out.info.channels,
        format: out.info.sample_format.clone(),
        frames_delivered: out.shared.frames_played.load(Ordering::SeqCst),
        peak,
    })
}

// -------------------------------------------------------------------- tests

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Write a small stereo 16-bit PCM WAV file for testing.
    pub fn write_test_wav(path: &Path, rate: u32, seconds: f32) {
        let frames = (rate as f32 * seconds) as u32;
        let mut data = Vec::new();
        for i in 0..frames {
            let t = i as f32 / rate as f32;
            let v = (2.0 * std::f32::consts::PI * 440.0 * t).sin();
            let s = (v * 12000.0) as i16;
            data.extend_from_slice(&s.to_le_bytes());
            data.extend_from_slice(&s.to_le_bytes());
        }
        let byte_rate = rate * 4;
        let mut f = Vec::new();
        f.extend_from_slice(b"RIFF");
        f.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        f.extend_from_slice(b"WAVE");
        f.extend_from_slice(b"fmt ");
        f.extend_from_slice(&16u32.to_le_bytes());
        f.extend_from_slice(&1u16.to_le_bytes());
        f.extend_from_slice(&2u16.to_le_bytes());
        f.extend_from_slice(&rate.to_le_bytes());
        f.extend_from_slice(&byte_rate.to_le_bytes());
        f.extend_from_slice(&4u16.to_le_bytes());
        f.extend_from_slice(&16u16.to_le_bytes());
        f.extend_from_slice(b"data");
        f.extend_from_slice(&(data.len() as u32).to_le_bytes());
        f.extend_from_slice(&data);
        std::fs::write(path, f).unwrap();
    }

    #[test]
    fn resampler_upsamples_two_to_one() {
        let mut r = StreamResampler::new(24000.0, 48000.0);
        let mut out = Vec::new();
        for v in [[0.0f32, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0]] {
            r.process(v, &mut out);
        }
        assert_eq!(out.len(), 12);
        assert_eq!(out[0], 0.0);
        assert!((out[2] - 0.5).abs() < 1e-6);
        assert_eq!(out[4], 1.0);
        assert_eq!(out[8], 2.0);
        assert!((out[10] - 2.5).abs() < 1e-6);
    }

    #[test]
    fn resampler_passthrough() {
        let mut r = StreamResampler::new(48000.0, 48000.0);
        let mut out = Vec::new();
        for v in [[1.0f32, 2.0], [3.0, 4.0], [5.0, 6.0]] {
            r.process(v, &mut out);
        }
        assert_eq!(out, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn decoded_wav_is_not_silent_and_has_expected_length() {
        let dir = std::env::temp_dir().join("music-mine-tests-decode");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tone.wav");
        write_test_wav(&path, 44100, 1.0);

        let report = analyze_file(&path, 2.0).unwrap();
        assert!(!report.is_silent(), "decoded PCM is silent: {report:?}");
        assert!(report.peak > 0.2, "peak too low: {report:?}");
        assert!(report.rms > 0.1, "rms too low: {report:?}");
        assert!(
            (report.seconds - 1.0).abs() < 0.05,
            "duration off: {report:?}"
        );
    }

    #[test]
    fn seeks_within_wav() {
        let dir = std::env::temp_dir().join("music-mine-tests-seek");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("long.wav");
        write_test_wav(&path, 44100, 3.0);

        let mut d = Decoder::open(&path, 48_000, 2).unwrap();
        d.seek(2.0).unwrap();
        let first = d.next_output().expect("audio after seek");
        assert!(!first.is_empty());
        assert!(first.iter().any(|s| s.abs() > 0.01), "silence after seek");
        assert!(first.iter().all(|s| s.is_finite()));
    }

    #[test]
    fn callback_moves_samples_to_the_device_buffer() {
        // Drives the actual callback logic without a device.
        let shared = Shared::new(48_000, 2, 1.0);
        shared.active.store(true, Ordering::SeqCst);
        shared.clock_on.store(true, Ordering::SeqCst);
        {
            let mut ring = shared.ring.lock().unwrap();
            for i in 0..480 {
                ring.push_back(if i % 2 == 0 { 0.5 } else { -0.5 });
            }
        }
        let mut data = vec![0.0f32; 192];
        fill_output(&mut data, &shared);
        assert_eq!(data.len(), 192);
        assert!(data.iter().any(|s| s.abs() > 0.4), "callback wrote silence");
        assert_eq!(shared.frames_played.load(Ordering::SeqCst), 96);
    }

    #[test]
    fn pause_writes_silence_and_freezes_the_clock() {
        let shared = Shared::new(48_000, 2, 1.0);
        shared.active.store(true, Ordering::SeqCst);
        shared.paused.store(true, Ordering::SeqCst);
        {
            let mut ring = shared.ring.lock().unwrap();
            ring.push_back(0.8);
            ring.push_back(0.8);
        }
        let mut data = vec![0.0f32; 64];
        fill_output(&mut data, &shared);
        assert!(data.iter().all(|s| *s == 0.0));
        assert_eq!(shared.frames_played.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn volume_scales_the_actual_callback_samples() {
        let shared = Shared::new(48_000, 2, 0.25);
        shared.ring.lock().unwrap().extend([0.8, -0.8]);
        let mut samples = [0.0f32; 2];
        fill_output(&mut samples, &shared);
        assert!((samples[0] - 0.2).abs() < 0.0001);
        assert!((samples[1] + 0.2).abs() < 0.0001);
    }

    #[test]
    fn paused_short_track_keeps_its_buffer_until_resumed() {
        let path =
            std::env::temp_dir().join(format!("music-mine-tauri-short-{}.wav", std::process::id()));
        write_test_wav(&path, 44100, 0.1);
        let player = Player::new(None);
        player.set_volume(0.0);
        player.prepare(path.clone(), 100, 0);
        assert_eq!(player.synchronize().unwrap().state, PlaybackState::Paused);
        std::thread::sleep(Duration::from_millis(2200));
        assert_eq!(player.synchronize().unwrap().state, PlaybackState::Paused);
        player.resume();
        player.synchronize().unwrap();
        std::thread::sleep(Duration::from_millis(400));
        assert_eq!(player.synchronize().unwrap().state, PlaybackState::Stopped);
        drop(player);
        std::thread::sleep(Duration::from_millis(20));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn real_output_device_delivers_audio() {
        // Skips (with a visible message) only when the machine has no device.
        match output_selftest(400) {
            Ok(r) => {
                eprintln!("selftest: {r:?}");
                assert!(r.frames_delivered > 0, "device never pulled frames");
                assert!(r.peak > 0.05, "device received silence: {r:?}");
            }
            Err(e) => eprintln!("SKIPPED (no output device): {e}"),
        }
    }
}
