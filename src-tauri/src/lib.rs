mod artwork;
mod audio;
mod catalog;
mod commands;
mod infrastructure;
mod platform;
mod playback;
#[cfg(test)]
mod test_support;

pub use audio::player;
pub use playback::lyrics;

use playback::controller::{Action, Controller, PlayerSnapshot};
use std::sync::{Arc, Mutex};
type Shared = Arc<Mutex<Controller>>;
async fn execute(state: Shared, action: Action) -> Result<PlayerSnapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        state
            .lock()
            .map_err(|_| "播放器控制器不可用".to_string())?
            .execute(action)
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
async fn player_get_state(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::State).await
}
#[tauri::command]
async fn player_open_file(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    let path = tauri::async_runtime::spawn_blocking(|| {
        rfd::FileDialog::new()
            .add_filter("音频 MP3 / FLAC / WAV", &["mp3", "flac", "wav"])
            .pick_file()
    })
    .await
    .map_err(|e| e.to_string())?;
    execute(
        state.inner().clone(),
        path.map(Action::Open).unwrap_or(Action::State),
    )
    .await
}
#[tauri::command]
async fn player_play(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Play).await
}
#[tauri::command]
async fn player_pause(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Pause).await
}
#[tauri::command]
async fn player_stop(state: tauri::State<'_, Shared>) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Stop).await
}
#[tauri::command]
async fn player_seek(
    position_ms: f64,
    state: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Seek(position_ms)).await
}
#[tauri::command]
async fn player_set_volume(
    volume: f64,
    state: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    execute(state.inner().clone(), Action::Volume(volume)).await
}
#[derive(serde::Serialize)]
struct BridgeReply {
    nonce: String,
    backend: &'static str,
    snapshot: PlayerSnapshot,
}
#[tauri::command]
async fn bridge_probe(
    nonce: String,
    state: tauri::State<'_, Shared>,
) -> Result<BridgeReply, String> {
    if nonce.len() > 128 {
        return Err("验证参数过长".into());
    }
    Ok(BridgeReply {
        nonce,
        backend: "rust",
        snapshot: execute(state.inner().clone(), Action::State).await?,
    })
}
#[tauri::command]
async fn player_configure(
    mode: Option<String>,
    offset: Option<i64>,
    path: Option<String>,
    target: Option<usize>,
    state: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    let action = if let Some(mode) = mode {
        Action::Mode(mode)
    } else if let Some(offset) = offset {
        Action::Offset(offset)
    } else if let Some(path) = path {
        if let Some(target) = target {
            Action::Reorder(path, target)
        } else {
            Action::Select(path)
        }
    } else {
        return Err("缺少操作参数".into());
    };
    execute(state.inner().clone(), action).await
}
#[tauri::command]
async fn player_lyrics(state: tauri::State<'_, Shared>) -> Result<Option<lyrics::Lyrics>, String> {
    let path = state.lock().map_err(|_| "播放器不可用")?.current_path();
    tauri::async_runtime::spawn_blocking(move || path.and_then(|p| lyrics::load_for_track(&p)))
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
async fn player_artwork(
    state: tauri::State<'_, Shared>,
    catalog: tauri::State<'_, Arc<catalog::Catalog>>,
    covers: tauri::State<'_, Arc<artwork::covers::Store>>,
) -> Result<Option<String>, String> {
    let path = state.lock().map_err(|_| "播放器不可用")?.current_path();
    let Some(path) = path else {
        return Ok(None);
    };
    let path_text = path.to_string_lossy().to_string();
    let catalog = catalog.inner().clone();
    let covers = covers.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let edge = 960;
        let cover = match catalog.track_by_path(&path_text)? {
            Some(track) => covers.for_track(track.id, std::path::Path::new(&track.path), edge),
            None => covers.for_playback(&path, edge),
        };
        cover.map_err(|e| e.code().to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
pub fn run() {
    use tauri::Manager;
    let context = tauri::generate_context!();
    if let Some(root) = portable_root_from(context.config()) {
        if let Err(error) = ensure_writable(&root)
            .and_then(|_| ensure_writable(&root.join("caches")))
            .and_then(|_| ensure_writable(&root.join("logs")))
        {
            report_portable_error(&error);
            std::process::exit(1);
        }
    }
    match tauri::Builder::default()
        .setup(|app| {
            let directory = app.path().app_data_dir()?;
            let cache = app.path().app_cache_dir()?;
            let log = app.path().app_log_dir()?;
            if let Err(error) = ensure_writable(&directory)
                .and_then(|_| ensure_writable(&cache))
                .and_then(|_| ensure_writable(&log))
            {
                return Err(std::io::Error::other(error).into());
            }
            app.manage(Arc::new(catalog::Catalog::new(
                directory.join("library-tauri-v1.sqlite"),
            )));
            app.manage(Arc::new(artwork::covers::Store::new(cache.join("covers"))));
            if let Ok(mut controller) = app.state::<Shared>().lock() {
                if let Err(error) = controller.attach_session(directory.join("playback-v1.json")) {
                    eprintln!("{error}");
                }
            }
            platform::desktop::setup(app)?;
            platform::media::setup(app);
            let handle = app.handle().clone();
            // Queue progression follows engine EOF on Rust's worker even with the UI hidden.
            let player = app.state::<Shared>().inner().clone();
            std::thread::Builder::new()
                .name("player-controller".into())
                .spawn(move || {
                    let mut last_path = None;
                    loop {
                        if handle
                            .state::<platform::desktop::Desktop>()
                            .exiting
                            .load(std::sync::atomic::Ordering::SeqCst)
                        {
                            break;
                        }
                        let snapshot = player
                            .lock()
                            .ok()
                            .and_then(|mut c| c.execute(Action::State).ok());
                        if let Some(snapshot) = snapshot {
                            if snapshot.playback_state == "playing"
                                && snapshot.current_track.as_ref().map(|t| &t.path)
                                    != last_path.as_ref()
                            {
                                if let Some(track) = &snapshot.current_track {
                                    if let Err(e) = handle
                                        .state::<Arc<catalog::Catalog>>()
                                        .record_listen(&track.path)
                                    {
                                        eprintln!("记录播放失败：{e}");
                                    }
                                }
                                last_path = snapshot.current_track.as_ref().map(|t| t.path.clone());
                            }
                            if snapshot.playback_state == "idle" {
                                last_path = None;
                            }
                            platform::desktop::update(&handle, &snapshot);
                            platform::media::update(&handle, &snapshot);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                })?;
            Ok(())
        })
        .on_window_event(platform::desktop::window_event)
        .manage(Arc::new(Mutex::new(Controller::new())))
        .invoke_handler(tauri::generate_handler![
            platform::desktop::desktop_preferences,
            platform::media::media_status,
            player_configure,
            player_lyrics,
            player_artwork,
            bridge_probe,
            player_get_state,
            player_open_file,
            player_play,
            player_pause,
            player_stop,
            player_seek,
            player_set_volume,
            commands::catalog::library_query,
            commands::catalog::library_summary,
            commands::catalog::library_home,
            commands::catalog::library_scan,
            commands::catalog::library_progress,
            commands::catalog::library_cancel,
            commands::catalog::library_track,
            commands::catalog::library_audio_info,
            commands::catalog::library_cover,
            commands::catalog::library_thumb,
            commands::catalog::library_queue_thumb,
            commands::catalog::library_cover_stats,
            commands::catalog::library_play,
            commands::catalog::library_favorite,
            commands::catalog::library_playlist,
            commands::catalog::library_playlist_edit,
            commands::catalog::library_import,
            commands::catalog::library_queue,
            commands::catalog::library_queue_tracks
        ])
        .build(context)
    {
        Ok(built) => built.run(|app, event| {
            if matches!(event, tauri::RunEvent::ExitRequested { .. }) {
                platform::media::shutdown(app);
                platform::desktop::shutdown(app);
            }
        }),
        Err(error) => {
            report_portable_error(&error.to_string());
            std::process::exit(1);
        }
    }
}

fn report_portable_error(error: &str) {
    let description = if error.contains("app-data") {
        format!(
            "无法写入程序旁边的 app-data 目录，Music Mine 没有启动。\n便携版不会改用 Windows 用户目录中的曲库或设置。\n请把整个文件夹放到可以写入的位置后重试。\n\n{error}"
        )
    } else {
        format!("Music Mine 没有启动。\n\n{error}")
    };
    let _ = rfd::MessageDialog::new()
        .set_title("Music Mine")
        .set_level(rfd::MessageLevel::Error)
        .set_description(description)
        .show();
}

fn portable_root_from(config: &tauri::Config) -> Option<std::path::PathBuf> {
    let configured = match config.app.app_directories_override.as_ref() {
        Some(tauri::utils::config::AppDirectoriesOverride::Root(path)) => path,
        _ => return None,
    };
    let exe = std::env::current_exe().ok()?;
    exe_relative_root(configured, exe.parent()?)
}

fn exe_relative_root(
    configured: &std::path::Path,
    exe_dir: &std::path::Path,
) -> Option<std::path::PathBuf> {
    if configured.is_absolute() {
        return None;
    }
    let text = configured.to_string_lossy();
    if text.starts_with('$') || text == "." || text == "./" || text == ".\\" {
        return None;
    }
    let relative = text.trim_start_matches("./").trim_start_matches(".\\");
    if relative.is_empty() {
        return None;
    }
    Some(exe_dir.join(relative))
}

fn ensure_writable(dir: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("无法创建 {}：{e}", dir.display()))?;
    let probe = dir.join(".write-probe");
    std::fs::write(&probe, b"1").map_err(|e| format!("无法写入 {}：{e}", dir.display()))?;
    let _ = std::fs::remove_file(&probe);
    Ok(())
}

#[cfg(test)]
mod portable_dir_tests {
    use super::{ensure_writable, exe_relative_root};
    use std::path::{Path, PathBuf};

    #[test]
    fn writable_probe_leaves_no_marker() {
        let dir = std::env::temp_dir().join(format!("music-mine-write-{}", std::process::id()));
        ensure_writable(&dir).expect("temp dir");
        assert!(!dir.join(".write-probe").exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn portable_root_is_beside_the_exe_and_rejects_the_exe_directory() {
        let exe_dir = Path::new(r"D:\Music Mine");
        assert_eq!(
            exe_relative_root(Path::new("./app-data"), exe_dir),
            Some(PathBuf::from(r"D:\Music Mine\app-data"))
        );
        assert_eq!(exe_relative_root(Path::new("."), exe_dir), None);
        assert_eq!(exe_relative_root(Path::new("./"), exe_dir), None);
        assert_eq!(
            exe_relative_root(Path::new(r"E:\absolute\app-data"), exe_dir),
            None
        );
    }
}
