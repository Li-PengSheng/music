use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::Manager;
#[cfg(windows)]
use windows::{
    core::HSTRING,
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, SystemMediaTransportControls,
        SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
    },
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream, RandomAccessStreamReference},
    Win32::System::WinRT::ISystemMediaTransportControlsInterop,
};

pub struct Media {
    stopped: AtomicBool,
    status: Mutex<String>,
    #[cfg(windows)]
    controls: Option<(SystemMediaTransportControls, EventRegistrationToken)>,
    last: Mutex<String>,
    #[cfg(windows)]
    thumbnail: Mutex<ThumbnailCache<RandomAccessStreamReference>>,
}

/// One track (including a failed/no-cover result), not one entry per poll.
#[cfg(any(windows, test))]
struct ThumbnailCache<T> {
    path: Option<String>,
    value: Option<T>,
}
#[cfg(any(windows, test))]
impl<T> Default for ThumbnailCache<T> {
    fn default() -> Self {
        Self {
            path: None,
            value: None,
        }
    }
}
#[cfg(any(windows, test))]
impl<T> ThumbnailCache<T> {
    fn update(
        &mut self,
        path: Option<&str>,
        load: impl FnOnce(&std::path::Path) -> Option<T>,
    ) -> Option<&T> {
        if self.path.as_deref() != path {
            self.path = path.map(str::to_owned);
            self.value = path.and_then(|path| load(std::path::Path::new(path)));
        }
        self.value.as_ref()
    }
}

#[cfg(any(windows, test))]
fn thumbnail_path<'a>(state: &str, path: Option<&'a str>) -> Option<&'a str> {
    if matches!(state, "playing" | "paused") {
        path
    } else {
        None
    }
}

/// Reuse Store's bounded JPEG/PNG thumbnail and its memory/disk cache. Its
/// WebView-only GIF/WebP passthrough may have unbounded dimensions, so omit it.
#[cfg(any(windows, test))]
fn prepare_thumbnail(
    store: &crate::artwork::covers::Store,
    path: &std::path::Path,
) -> Option<Vec<u8>> {
    use base64::Engine;
    let url = store.for_playback(path, 320).ok()??;
    let encoded = url.strip_prefix("data:image/jpeg;base64,")?;
    // Bound the WinRT payload even if a disk cache entry is damaged.
    if encoded.len() > 700_000 {
        return None;
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;
    if bytes.is_empty() || bytes.len() > 512 * 1024 {
        return None;
    }
    Some(bytes)
}

#[cfg(windows)]
fn thumbnail_reference(bytes: &[u8]) -> windows::core::Result<RandomAccessStreamReference> {
    let stream = InMemoryRandomAccessStream::new()?;
    let writer = DataWriter::CreateDataWriter(&stream)?;
    writer.WriteBytes(bytes)?;
    writer.StoreAsync()?.get()?;
    writer.DetachStream()?;
    stream.Seek(0)?;
    RandomAccessStreamReference::CreateFromStream(&stream)
}
#[tauri::command]
pub fn media_status(app: tauri::AppHandle) -> Result<String, String> {
    app.state::<Media>()
        .status
        .lock()
        .map(|s| s.clone())
        .map_err(|e| e.to_string())
}
pub fn setup(app: &mut tauri::App) {
    #[cfg(windows)]
    let result = (|| -> windows::core::Result<_> {
        let window = app
            .get_webview_window("main")
            .ok_or_else(windows::core::Error::from_win32)?;
        let hwnd = window
            .hwnd()
            .map_err(|_| windows::core::Error::from_win32())?;
        let factory: ISystemMediaTransportControlsInterop = windows::core::factory::<
            SystemMediaTransportControls,
            ISystemMediaTransportControlsInterop,
        >()?;
        // Desktop WinRT interop uses our existing top-level HWND, never a MediaPlayer.
        let controls: SystemMediaTransportControls =
            unsafe { factory.GetForWindow(windows::Win32::Foundation::HWND(hwnd.0))? };
        controls.SetIsPlayEnabled(true)?;
        controls.SetIsPauseEnabled(true)?;
        controls.SetIsNextEnabled(true)?;
        controls.SetIsPreviousEnabled(true)?;
        let handle = app.handle().clone();
        let token = controls.ButtonPressed(&TypedEventHandler::new(
            move |_, args: &Option<SystemMediaTransportControlsButtonPressedEventArgs>| {
                if let Some(args) = args {
                    let action = match args.Button()? {
                        SystemMediaTransportControlsButton::Play => {
                            crate::playback::controller::Action::Play
                        }
                        SystemMediaTransportControlsButton::Pause => {
                            crate::playback::controller::Action::Pause
                        }
                        SystemMediaTransportControlsButton::Next => {
                            crate::playback::controller::Action::Next
                        }
                        SystemMediaTransportControlsButton::Previous => {
                            crate::playback::controller::Action::Previous
                        }
                        _ => return Ok(()),
                    };
                    let handle = handle.clone();
                    tauri::async_runtime::spawn_blocking(move || {
                        if handle.state::<Media>().stopped.load(Ordering::SeqCst) {
                            return;
                        }
                        let result = handle
                            .state::<crate::Shared>()
                            .lock()
                            .map_err(|e| e.to_string())
                            .and_then(|mut c| c.execute(action));
                        if let Err(e) = result {
                            if let Ok(mut s) = handle.state::<Media>().status.lock() {
                                *s = format!("SMTC 操作失败：{e}");
                            }
                        }
                    });
                }
                Ok(())
            },
        ))?;
        Ok((controls, token))
    })();
    #[cfg(windows)]
    let (controls, status) = match result {
        Ok(c) => (Some(c), "SMTC 已连接（含专辑封面）".to_string()),
        Err(e) => (None, format!("SMTC 不可用：{e}")),
    };
    #[cfg(not(windows))]
    let status = "仅支持 Windows SMTC".to_string();
    app.state::<std::sync::Arc<crate::infrastructure::logging::LogBuffer>>()
        .push(format!("[smtc] {status}"));
    app.manage(Media {
        stopped: AtomicBool::new(false),
        #[cfg(windows)]
        controls,
        status: Mutex::new(status),
        last: Mutex::new(String::new()),
        #[cfg(windows)]
        thumbnail: Mutex::new(ThumbnailCache::default()),
    });
}
pub fn update(app: &tauri::AppHandle, s: &crate::playback::controller::PlayerSnapshot) {
    let state = app.state::<Media>();
    let key = format!(
        "{}:{}:{}",
        s.playback_state,
        serde_json::to_string(&s.current_track).unwrap_or_default(),
        s.queue.len()
    );
    let Ok(mut last) = state.last.lock() else {
        return;
    };
    if state.stopped.load(Ordering::SeqCst) {
        return;
    }
    if *last == key {
        return;
    }
    #[cfg(windows)]
    if let Some((c, _)) = &state.controls {
        let result = (|| -> windows::core::Result<()> {
            c.SetIsEnabled(s.current_track.is_some())?;
            c.SetPlaybackStatus(match s.playback_state {
                "playing" => MediaPlaybackStatus::Playing,
                "paused" => MediaPlaybackStatus::Paused,
                _ => MediaPlaybackStatus::Stopped,
            })?;
            c.SetIsNextEnabled(!s.queue.is_empty())?;
            c.SetIsPreviousEnabled(!s.queue.is_empty())?;
            let display = c.DisplayUpdater()?;
            display.ClearAll()?;
            if let Some(t) = &s.current_track {
                display.SetType(MediaPlaybackType::Music)?;
                let music = display.MusicProperties()?;
                music.SetTitle(&HSTRING::from(&t.title))?;
                music.SetArtist(&HSTRING::from(&t.artist))?;
                music.SetAlbumTitle(&HSTRING::from(&t.album))?;
            }
            // ClearAll above prevents stale art on no-cover/idle transitions.
            // Pausing or queue metadata updates only reattach the cached WinRT
            // reference; decoding and stream creation happen on path changes.
            if let Ok(mut cache) = state.thumbnail.lock() {
                let path = thumbnail_path(
                    s.playback_state,
                    s.current_track.as_ref().map(|t| t.path.as_str()),
                );
                if let Some(reference) = cache.update(path, |path| {
                    let store = app.state::<std::sync::Arc<crate::artwork::covers::Store>>();
                    let bytes = prepare_thumbnail(&store, path)?;
                    thumbnail_reference(&bytes).ok()
                }) {
                    // Art is optional: a thumbnail failure must never skip Update
                    // for title/artist or change the player's transport state.
                    if let Err(error) = display.SetThumbnail(reference) {
                        eprintln!("SMTC 封面同步失败，保留文本元数据：{error}");
                    }
                }
            }
            display.Update()
        })();
        if let Err(e) = result {
            if let Ok(mut status) = state.status.lock() {
                *status = format!("SMTC 同步失败：{e}");
            }
            return;
        }
    }
    *last = key;
}
pub fn shutdown(app: &tauri::AppHandle) {
    let state = app.state::<Media>();
    let Ok(_last) = state.last.lock() else { return };
    if state.stopped.swap(true, Ordering::SeqCst) {
        return;
    }
    #[cfg(windows)]
    if let Some((c, token)) = &app.state::<Media>().controls {
        if let Err(e) = c
            .RemoveButtonPressed(*token)
            .and_then(|_| c.SetIsEnabled(false))
        {
            eprintln!("关闭 SMTC 失败：{e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn thumbnail_path_deduplicates_polls_pause_and_missing_covers_but_clears_idle() {
        let mut cache = ThumbnailCache::<Vec<u8>>::default();
        let mut reads = 0;
        for state in ["playing", "playing", "paused", "playing"] {
            let image = cache.update(thumbnail_path(state, Some("a.flac")), |_| {
                reads += 1;
                Some(vec![1, 2, 3])
            });
            assert_eq!(image, Some(&vec![1, 2, 3]));
        }
        assert_eq!(reads, 1);
        assert_eq!(
            cache.update(Some("b.flac"), |_| {
                reads += 1;
                None
            }),
            None
        );
        assert_eq!(
            cache.update(Some("b.flac"), |_| panic!("no repeated failed read")),
            None
        );
        assert_eq!(reads, 2);
        assert_eq!(
            cache.update(thumbnail_path("idle", Some("b.flac")), |_| panic!(
                "idle cannot read files"
            )),
            None
        );
        assert_eq!(
            cache.update(thumbnail_path("paused", Some("a.flac")), |_| Some(vec![4])),
            Some(&vec![4])
        );
        assert_eq!(
            cache.update(thumbnail_path("playing", None), |_| panic!(
                "no track cannot read files"
            )),
            None
        );
    }

    #[test]
    fn thumbnail_bytes_reuse_bounded_cover_cache_and_never_change_audio() {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        use lofty::picture::{MimeType, Picture, PictureType};
        use std::io::Cursor;
        let directory =
            std::env::temp_dir().join(format!("music-mine-smtc-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let original = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.flac");
        let source_before = std::fs::read(&original).unwrap();
        let covered = directory.join("covered.flac");
        std::fs::copy(&original, &covered).unwrap();
        let image = image::RgbImage::from_pixel(1280, 640, image::Rgb([20, 80, 160]));
        let mut picture = Cursor::new(Vec::new());
        image
            .write_to(&mut picture, image::ImageFormat::Png)
            .unwrap();
        let mut tagged = lofty::read_from_path(&covered).unwrap();
        tagged
            .primary_tag_mut()
            .unwrap()
            .push_picture(Picture::new_unchecked(
                PictureType::CoverFront,
                Some(MimeType::Png),
                None,
                picture.into_inner(),
            ));
        tagged
            .save_to_path(&covered, WriteOptions::default())
            .unwrap();
        let covered_before = std::fs::read(&covered).unwrap();
        let store = crate::artwork::covers::Store::new(directory.join("cache"));
        let bytes = prepare_thumbnail(&store, &covered).unwrap();
        let thumb = image::load_from_memory(&bytes).unwrap();
        assert_eq!((thumb.width(), thumb.height()), (320, 160));
        assert!(bytes.len() < 512 * 1024);
        assert_eq!(prepare_thumbnail(&store, &covered).unwrap(), bytes);
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 1);
        assert_eq!(prepare_thumbnail(&store, &original), None);
        assert_eq!(
            prepare_thumbnail(&store, &directory.join("missing.flac")),
            None
        );
        assert_eq!(std::fs::read(&original).unwrap(), source_before);
        assert_eq!(std::fs::read(&covered).unwrap(), covered_before);
        // A corrupt embedded picture degrades to no thumbnail, without panic or
        // touching the audio file during preparation.
        let mut corrupt = lofty::read_from_path(&covered).unwrap();
        let tag = corrupt.primary_tag_mut().unwrap();
        tag.remove_picture_type(PictureType::CoverFront);
        tag.push_picture(Picture::new_unchecked(
            PictureType::CoverFront,
            Some(MimeType::Jpeg),
            None,
            vec![0xff, 0xd8, 0xff, 0x00],
        ));
        corrupt
            .save_to_path(&covered, WriteOptions::default())
            .unwrap();
        let corrupt_before = std::fs::read(&covered).unwrap();
        assert_eq!(prepare_thumbnail(&store, &covered), None);
        assert_eq!(std::fs::read(&covered).unwrap(), corrupt_before);
        std::fs::remove_dir_all(&directory).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn thumbnail_reference_keeps_written_bytes_alive_after_writer_and_stream_drop() {
        use windows::Storage::Streams::DataReader;
        let bytes = [1u8, 2, 3, 4, 5];
        let reference = thumbnail_reference(&bytes).unwrap();
        let stream = reference.OpenReadAsync().unwrap().get().unwrap();
        assert_eq!(stream.Size().unwrap(), 5);
        let reader = DataReader::CreateDataReader(&stream).unwrap();
        assert_eq!(reader.LoadAsync(5).unwrap().get().unwrap(), 5);
        let mut actual = [0u8; 5];
        reader.ReadBytes(&mut actual).unwrap();
        assert_eq!(actual, bytes);
    }
}
