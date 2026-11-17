use crate::audio::player::{PlaybackEvent, PlaybackState, Player};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metadata_refresh_preserves_transport_and_revises_current_text_and_queue() {
        let mut c = Controller::new();
        c.execute(Action::Open(fixture("tone.flac"))).unwrap();
        let before = c.execute(Action::Seek(500.)).unwrap();
        let mut track = before.current_track.clone().unwrap();
        track.title = "Enriched title".into();
        let next = c.execute(Action::RefreshMetadata(track)).unwrap();
        assert_eq!(next.playback_state, before.playback_state);
        assert_eq!(next.position_ms, before.position_ms);
        assert_eq!(next.listen_session, before.listen_session);
        assert_eq!(next.listen_seek, before.listen_seek);
        assert_eq!(next.metadata_revision, before.metadata_revision + 1);
        assert_eq!(next.current_track.unwrap().title, "Enriched title");
        assert_eq!(next.queue[0].title, "Enriched title");
    }
    #[test]
    fn output_device_action_keeps_queue_track_volume_and_transport() {
        if std::env::var_os("MUSIC_MINE_NULL_OUTPUT").is_none() {
            return;
        }
        let mut c = Controller::new();
        assert_eq!(c.execute(Action::State).unwrap().volume, 0.2);
        c.execute(Action::Volume(0.2)).unwrap();
        c.execute(Action::Open(fixture("tone.flac"))).unwrap();
        let before = c.execute(Action::Seek(700.)).unwrap();
        let after = c
            .execute(Action::OutputDevice(Some("unplugged".into())))
            .unwrap();
        assert_eq!(after.playback_state, "paused");
        assert_eq!(after.track_id, before.track_id);
        assert_eq!(after.position_ms, before.position_ms);
        assert_eq!(after.queue.len(), before.queue.len());
        assert_eq!(after.volume, before.volume);
        assert!(after.revision > before.revision);
        assert_eq!(after.output_device_id.as_deref(), Some("null"));
        c.execute(Action::Play).unwrap();
        let after = c.execute(Action::OutputDevice(None)).unwrap();
        assert_eq!(after.playback_state, "playing");
        assert!(after.position_ms >= 700);
        c.execute(Action::Stop).unwrap();
    }
    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }
    #[test]
    fn engine_eof_drives_sequential_shuffle_and_repeat_modes() {
        for mode in ["sequential", "shuffle", "repeat_one", "repeat_all"] {
            let mut c = Controller::new();
            c.execute(Action::Volume(0.)).unwrap();
            c.execute(Action::Open(fixture("tone.flac"))).unwrap();
            let first = c.track.clone().unwrap();
            c.execute(Action::Open(fixture("tone.mp3"))).unwrap();
            let last = c.track.clone().unwrap();
            c.execute(Action::Mode(mode.into())).unwrap();
            let duration = last.duration_ms;
            c.execute(Action::Seek(duration.saturating_sub(120) as f64))
                .unwrap();
            c.execute(Action::Play).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(250));
            c.execute(Action::Volume(0.)).unwrap(); // volume changes must not swallow EOF
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
            loop {
                std::thread::sleep(std::time::Duration::from_millis(30));
                let snapshot = c.execute(Action::State).unwrap();
                let done = match mode {
                    "sequential" => snapshot.playback_state == "idle",
                    "repeat_one" => {
                        snapshot.track_id == Some(last.path.clone())
                            && snapshot.position_ms < duration.saturating_sub(500)
                    }
                    _ => snapshot.track_id == Some(first.path.clone()),
                };
                if done {
                    break;
                }
                assert!(
                    std::time::Instant::now() < deadline,
                    "EOF did not apply mode {mode}"
                );
            }
            c.execute(Action::Stop).unwrap();
        }
    }
    #[test]
    fn session_restores_queue_position_mode_and_pauses() {
        let dir = std::env::temp_dir().join(format!("music-mine-session-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        let _ = std::fs::remove_file(&path);
        let mut c = Controller::new();
        c.attach_session(path.clone()).unwrap();
        c.execute(Action::Volume(0.)).unwrap();
        c.execute(Action::Open(fixture("tone.flac"))).unwrap();
        c.execute(Action::Seek(500.)).unwrap();
        c.execute(Action::Mode("repeat_all".into())).unwrap();
        c.execute(Action::Offset(-300)).unwrap();
        let mut restored = Controller::new();
        restored.attach_session(path.clone()).unwrap();
        let state = restored.execute(Action::State).unwrap();
        assert_eq!(state.playback_state, "paused");
        assert_eq!(state.playback_mode, "repeat_all");
        assert_eq!(state.lyrics_offset_ms, -300);
        assert_eq!(state.queue.len(), 1);
        assert!(state.position_ms.abs_diff(500) < 50);
        assert_eq!(state.volume, 0.);
        restored.execute(Action::Offset(100)).unwrap(); // replacing an existing session on Windows
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn queue_reorders_inserts_next_and_respects_modes() {
        let mut c = Controller::new();
        let make = |path: &str| Track {
            path: path.into(),
            title: path.into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        c.queue = vec![make("a"), make("b"), make("c")];
        c.track = Some(make("a"));
        c.execute(Action::Reorder("c".into(), 0)).unwrap();
        assert_eq!(
            c.queue.iter().map(|t| t.path.as_str()).collect::<Vec<_>>(),
            vec!["c", "a", "b"]
        );
        c.execute(Action::PlayNext(make("c"))).unwrap();
        assert_eq!(
            c.queue.iter().map(|t| t.path.as_str()).collect::<Vec<_>>(),
            vec!["a", "c", "b"]
        );
        assert_eq!(c.next_index(false), Some(1));
        assert_eq!(c.next_index(true), None);
        c.mode = "repeat_all".into();
        assert_eq!(c.next_index(true), Some(2));
        c.mode = "shuffle".into();
        for _ in 0..100 {
            assert_ne!(c.next_index(false), Some(0));
        }
        assert!(c.execute(Action::Mode("invalid".into())).is_err());
        assert!(c.execute(Action::Reorder("a".into(), 8)).is_err());
    }
    #[test]
    fn bulk_queue_actions_dedupe_and_clear_keeps_current() {
        let mut c = Controller::new();
        let make = |path: &str| Track {
            path: path.into(),
            title: path.into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        c.queue = vec![make("a"), make("b")];
        c.track = Some(make("b"));
        c.execute(Action::EnqueueMany(vec![
            make("c"),
            make("a"),
            make("c"),
            make("d"),
        ]))
        .unwrap();
        assert_eq!(
            c.queue.iter().map(|t| t.path.as_str()).collect::<Vec<_>>(),
            vec!["a", "b", "c", "d"]
        );
        let state = c.execute(Action::ClearUpcoming).unwrap();
        assert_eq!(state.queue.len(), 1);
        assert_eq!(state.queue[0].path, "b");
        c.track = None;
        assert!(c.execute(Action::ClearUpcoming).unwrap().queue.is_empty());
        assert!(c.execute(Action::Replace(Vec::new())).is_err());
    }
    #[test]
    fn replace_queue_starts_first_real_file() {
        let mut c = Controller::new();
        c.execute(Action::Volume(0.0)).unwrap();
        let track = |name: &str| Track {
            path: fixture(name).to_string_lossy().into_owned(),
            title: name.into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        let state = c
            .execute(Action::Replace(vec![
                track("tone.wav"),
                track("tone.flac"),
                track("tone.wav"),
            ]))
            .unwrap();
        assert_eq!(state.queue.len(), 2);
        assert_eq!(state.track_id, Some(track("tone.wav").path));
        assert_eq!(state.playback_state, "playing");
        c.execute(Action::Stop).unwrap();
    }
    #[test]
    fn queue_uses_existing_engine_for_selection_next_and_previous() {
        let mut c = Controller::new();
        c.execute(Action::Volume(0.0)).unwrap();
        let first = c.execute(Action::Open(fixture("tone.flac"))).unwrap();
        let first_track = first.current_track.unwrap();
        let second = Track {
            path: fixture("tone.mp3").to_string_lossy().into_owned(),
            title: "second".into(),
            artist: String::new(),
            album: String::new(),
            duration_ms: 3000,
        };
        let s = c.execute(Action::Enqueue(second.clone())).unwrap();
        assert_eq!(s.queue.len(), 2);
        assert_eq!(
            c.execute(Action::Enqueue(second.clone()))
                .unwrap()
                .queue
                .len(),
            2
        );
        let next = c.execute(Action::Next).unwrap();
        assert_eq!(next.track_id, Some(second.path.clone()));
        assert_eq!(next.playback_state, "playing");
        let previous = c.execute(Action::Previous).unwrap();
        assert_eq!(previous.track_id, Some(first_track.path));
        c.execute(Action::Stop).unwrap();
        assert_eq!(
            c.execute(Action::Remove(second.path)).unwrap().queue.len(),
            1
        );
    }
}
#[derive(Clone, Serialize)]
pub struct PlayerSnapshot {
    #[serde(skip)]
    pub(crate) listen_session: u64,
    #[serde(skip)]
    pub(crate) listen_seek: u64,
    revision: u64,
    pub metadata_revision: u64,
    track_id: Option<String>,
    pub current_track: Option<Track>,
    pub playback_state: &'static str,
    pub(crate) position_ms: u64,
    duration_ms: u64,
    volume: f32,
    pub playback_mode: String,
    lyrics_offset_ms: i64,
    output_device_id: Option<String>,
    pub(crate) error: Option<String>,
    pub queue: Vec<Track>,
}
pub enum Action {
    State,
    /// Refresh catalog text without reopening the file, seeking or starting a listen session.
    RefreshMetadata(Track),
    OutputDevice(Option<String>),
    Open(PathBuf),
    Start(PathBuf),
    Select(String),
    Play,
    Pause,
    Stop,
    Seek(f64),
    Volume(f64),
    Enqueue(Track),
    EnqueueMany(Vec<Track>),
    /// Replaces the queue with these tracks (deduplicated by path) and starts the first.
    Replace(Vec<Track>),
    /// Keeps only the current track in the queue.
    ClearUpcoming,
    Remove(String),
    Next,
    Previous,
    Mode(String),
    Reorder(String, usize),
    PlayNext(Track),
    Offset(i64),
}
pub struct Controller {
    listen_session: u64,
    listen_seek: u64,
    engine: Player,
    track: Option<Track>,
    metadata_revision: u64,
    revision: u64,
    queue: Vec<Track>,
    mode: String,
    offset: i64,
    session_path: Option<PathBuf>,
    saved: String,
    last_save: std::time::Instant,
    random: u64,
}
impl Controller {
    pub fn new() -> Self {
        Self {
            listen_session: 0,
            listen_seek: 0,
            engine: Player::new(None),
            track: None,
            metadata_revision: 0,
            revision: 0,
            queue: Vec::new(),
            mode: "sequential".into(),
            offset: 0,
            session_path: None,
            saved: String::new(),
            last_save: std::time::Instant::now(),
            random: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
                | 1,
        }
    }
    pub fn execute(&mut self, action: Action) -> Result<PlayerSnapshot, String> {
        let is_state = matches!(action, Action::State);
        let replaces_transport = matches!(
            action,
            Action::Open(_)
                | Action::Start(_)
                | Action::Select(_)
                | Action::Play
                | Action::Pause
                | Action::Stop
                | Action::Seek(_)
                | Action::Next
                | Action::Previous
        );
        if matches!(action, Action::State) {
            let mut finished = false;
            while let Some(event) = self.engine.poll() {
                if matches!(event, PlaybackEvent::Finished) {
                    finished = true;
                }
            }
            if finished {
                if self.mode == "repeat_one" {
                    if let Some(t) = &self.track {
                        return self.execute(Action::Start(t.path.clone().into()));
                    }
                }
                if let Some(index) = self.next_index(false) {
                    return self.execute(Action::Start(self.queue[index].path.clone().into()));
                }
            }
        }

        let mut prepared_status = None;
        match action {
            Action::RefreshMetadata(track) => {
                for item in &mut self.queue {
                    if item.path == track.path {
                        *item = track.clone();
                    }
                }
                if self
                    .track
                    .as_ref()
                    .is_some_and(|item| item.path == track.path)
                {
                    self.track = Some(track);
                    self.metadata_revision = self.metadata_revision.wrapping_add(1);
                }
            }
            Action::Select(path) => {
                if !self.queue.iter().any(|t| t.path == path) {
                    return Err("歌曲不在队列中".into());
                }
                return self.execute(Action::Start(path.into()));
            }
            Action::Start(path) => {
                self.execute(Action::Open(path))?;
                return self.execute(Action::Play);
            }
            Action::Enqueue(track) => {
                if !self.queue.iter().any(|t| t.path == track.path) {
                    self.queue.push(track);
                }
            }
            Action::EnqueueMany(tracks) => {
                for track in tracks {
                    if !self.queue.iter().any(|t| t.path == track.path) {
                        self.queue.push(track);
                    }
                }
            }
            Action::Replace(tracks) => {
                let mut queue: Vec<Track> = Vec::with_capacity(tracks.len());
                for track in tracks {
                    if !queue.iter().any(|t| t.path == track.path) {
                        queue.push(track);
                    }
                }
                let first = queue.first().ok_or("没有可播放的歌曲")?.path.clone();
                self.queue = queue;
                return self.execute(Action::Start(first.into()));
            }
            Action::ClearUpcoming => {
                let current = self.track.as_ref().map(|t| t.path.clone());
                self.queue.retain(|t| Some(&t.path) == current.as_ref());
            }
            Action::Remove(path) => {
                self.queue.retain(|t| t.path != path);
            }
            Action::Next | Action::Previous => {
                let index = self
                    .next_index(matches!(action, Action::Previous))
                    .ok_or("已经到达队列边界")?;
                let path = PathBuf::from(&self.queue[index].path);
                self.execute(Action::Open(path))?;
                return self.execute(Action::Play);
            }
            Action::Mode(mode) => {
                if !["sequential", "shuffle", "repeat_one", "repeat_all"].contains(&mode.as_str()) {
                    return Err("无效播放模式".into());
                }
                self.mode = mode;
            }
            Action::Offset(value) => {
                if !(-60_000..=60_000).contains(&value) {
                    return Err("歌词偏移范围为 ±60 秒".into());
                }
                self.offset = value;
            }
            Action::Reorder(path, target) => {
                if target >= self.queue.len() {
                    return Err("无效队列位置".into());
                }
                let from = self
                    .queue
                    .iter()
                    .position(|t| t.path == path)
                    .ok_or("歌曲不在队列中")?;
                let track = self.queue.remove(from);
                self.queue.insert(target, track);
            }
            Action::PlayNext(track) => {
                self.queue.retain(|t| t.path != track.path);
                let index = self
                    .track
                    .as_ref()
                    .and_then(|t| self.queue.iter().position(|q| q.path == t.path))
                    .map_or(0, |i| i + 1);
                self.queue.insert(index, track);
            }
            Action::State => {}
            Action::Open(path) => {
                let extension = path
                    .extension()
                    .and_then(|v| v.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                if !["mp3", "flac", "wav"].contains(&extension.as_str()) {
                    return Err("请选择 MP3、FLAC 或 WAV 文件".into());
                }
                let file =
                    lofty::read_from_path(&path).map_err(|e| format!("读取音频失败：{e}"))?;
                let text = |field: u8| {
                    file.primary_tag()
                        .into_iter()
                        .chain(file.tags().iter())
                        .find_map(|tag| {
                            let value = match field {
                                0 => tag.title(),
                                1 => tag.artist(),
                                _ => tag.album(),
                            };
                            value.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty())
                        })
                };
                let track = Track {
                    path: path.to_string_lossy().into_owned(),
                    title: text(0).unwrap_or_else(|| {
                        path.file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned()
                    }),
                    artist: text(1).unwrap_or_else(|| "未知歌手".into()),
                    album: text(2).unwrap_or_default(),
                    duration_ms: file.properties().duration().as_millis() as u64,
                };
                self.engine.prepare(path, track.duration_ms, 0);
                let status = self.engine.synchronize()?;
                self.track = if status.current_path.as_deref() == Some(track.path.as_str()) {
                    if !self.queue.iter().any(|t| t.path == track.path) {
                        self.queue.push(track.clone());
                    }
                    Some(track)
                } else {
                    None
                };
                prepared_status = Some(status);
                self.listen_session = self.listen_session.wrapping_add(1);
            }
            Action::Play => {
                let track = self.track.as_ref().ok_or("请先添加真实音频文件")?;
                match self.engine.status().state {
                    PlaybackState::Paused => self.engine.resume(),
                    PlaybackState::Stopped => {
                        self.listen_session = self.listen_session.wrapping_add(1);
                        self.engine
                            .play(PathBuf::from(&track.path), track.duration_ms);
                    }
                    PlaybackState::Playing => {}
                }
            }
            Action::Pause => self.engine.pause(),
            Action::Stop => self.engine.stop(),
            Action::Seek(ms) => {
                let status = self.engine.status();
                if !ms.is_finite() || ms < 0.0 || ms > status.duration.as_millis() as f64 {
                    return Err("跳转位置超出歌曲范围".into());
                }
                if status.state == PlaybackState::Stopped {
                    return Err("请先播放歌曲再跳转".into());
                }
                self.engine.seek(ms / 1000.0);
                self.listen_seek = self.listen_seek.wrapping_add(1);
            }
            Action::Volume(value) => {
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err("音量应在 0 到 100% 之间".into());
                }
                self.engine.set_volume(value as f32);
            }
            Action::OutputDevice(device) => self.engine.set_output_device(device),
        }
        let status = match prepared_status {
            Some(status) => status,
            None => self.engine.synchronize()?,
        };
        // Only transport changes invalidate old EOF. Queue/config/volume actions preserve it.
        if replaces_transport {
            while self.engine.poll().is_some() {}
        }
        self.revision += 1;
        let snapshot = PlayerSnapshot {
            listen_session: self.listen_session,
            listen_seek: self.listen_seek,
            revision: self.revision,
            metadata_revision: self.metadata_revision,
            track_id: self.track.as_ref().map(|t| t.path.clone()),
            current_track: self.track.clone(),
            playback_state: match status.state {
                PlaybackState::Stopped => "idle",
                PlaybackState::Playing => "playing",
                PlaybackState::Paused => "paused",
            },
            position_ms: status.position.as_millis() as u64,
            duration_ms: status.duration.as_millis() as u64,
            volume: status.volume,
            playback_mode: self.mode.clone(),
            lyrics_offset_ms: self.offset,
            output_device_id: status.output.map(|o| o.device),
            error: status.error,
            queue: self.queue.clone(),
        };
        if !is_state || self.last_save.elapsed().as_secs() >= 2 {
            self.persist(&snapshot)?;
        }
        Ok(snapshot)
    }
}

#[derive(Serialize, Deserialize)]
struct Session {
    queue: Vec<Track>,
    current: Option<Track>,
    position: u64,
    volume: f32,
    mode: String,
    offset: i64,
}
impl Controller {
    fn next_index(&mut self, previous: bool) -> Option<usize> {
        let len = self.queue.len();
        if len == 0 {
            return None;
        }
        let current = self
            .track
            .as_ref()
            .and_then(|t| self.queue.iter().position(|q| q.path == t.path));
        let Some(i) = current else {
            return Some(0);
        };
        if self.mode == "shuffle" && len > 1 {
            self.random ^= self.random << 13;
            self.random ^= self.random >> 7;
            self.random ^= self.random << 17;
            return Some((i + 1 + self.random as usize % (len - 1)) % len);
        }
        if previous {
            i.checked_sub(1)
                .or_else(|| (self.mode == "repeat_all").then_some(len - 1))
        } else if i + 1 < len {
            Some(i + 1)
        } else {
            (self.mode == "repeat_all").then_some(0)
        }
    }
    pub fn attach_session(&mut self, path: PathBuf) -> Result<(), String> {
        if path.exists() {
            let session: Session =
                serde_json::from_slice(&std::fs::read(&path).map_err(|e| e.to_string())?)
                    .map_err(|e| format!("播放会话读取失败：{e}"))?;
            self.queue = session.queue;
            if ["sequential", "shuffle", "repeat_one", "repeat_all"]
                .contains(&session.mode.as_str())
            {
                self.mode = session.mode;
            }
            self.offset = session.offset.clamp(-60_000, 60_000);
            if session.volume.is_finite() {
                self.engine.set_volume(session.volume.clamp(0., 1.));
            }
            if let Some(track) = session.current {
                if std::path::Path::new(&track.path).is_file() {
                    self.engine.prepare(
                        track.path.clone().into(),
                        track.duration_ms,
                        session.position.min(track.duration_ms),
                    );
                    self.engine.synchronize()?;
                    self.track = Some(track);
                }
            }
        }
        self.session_path = Some(path);
        Ok(())
    }
    fn persist(&mut self, s: &PlayerSnapshot) -> Result<(), String> {
        let Some(path) = &self.session_path else {
            return Ok(());
        };
        let json = serde_json::to_string(&Session {
            queue: self.queue.clone(),
            current: self.track.clone(),
            position: s.position_ms,
            volume: s.volume,
            mode: self.mode.clone(),
            offset: self.offset,
        })
        .map_err(|e| e.to_string())?;
        if json != self.saved {
            let temp = path.with_extension("tmp");
            std::fs::write(&temp, &json).map_err(|e| format!("保存播放会话失败：{e}"))?;
            std::fs::rename(&temp, path).map_err(|e| format!("替换播放会话失败：{e}"))?;
            self.saved = json;
        }
        self.last_save = std::time::Instant::now();
        Ok(())
    }
    pub fn release_output(&mut self) -> Result<(), String> {
        self.engine.shutdown()
    }
    pub fn flush_session(&mut self) -> Result<(), String> {
        let snapshot = self.execute(Action::State)?;
        self.persist(&snapshot)
    }
    pub fn current_path(&self) -> Option<PathBuf> {
        self.track.as_ref().map(|t| t.path.clone().into())
    }
    pub fn log_buffer(&self) -> std::sync::Arc<crate::infrastructure::logging::LogBuffer> {
        self.engine.log_buffer()
    }
}
