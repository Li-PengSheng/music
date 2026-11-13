mod catalog;
mod catalog_commands;
#[allow(dead_code)]
mod collections;
mod controller;
#[allow(dead_code)]
mod db;
mod desktop;
#[allow(dead_code)]
mod library;
mod logging;
pub mod lyrics;
mod media;
mod metadata;
pub mod player;
use controller::{Action, Controller, PlayerSnapshot};
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
async fn player_artwork(state: tauri::State<'_, Shared>) -> Result<Option<String>, String> {
    let path = state.lock().map_err(|_| "播放器不可用")?.current_path();
    tauri::async_runtime::spawn_blocking(move || {
        use base64::Engine;
        use lofty::file::TaggedFileExt;
        let tagged = lofty::read_from_path(path?).ok()?;
        let picture = tagged.tags().iter().flat_map(|t| t.pictures()).next()?;
        let mime = picture.mime_type()?.as_str();
        if !["image/jpeg", "image/png", "image/webp", "image/gif"].contains(&mime)
            || picture.data().len() > 10 * 1024 * 1024
        {
            return None;
        }
        Some(format!(
            "data:{mime};base64,{}",
            base64::engine::general_purpose::STANDARD.encode(picture.data())
        ))
    })
    .await
    .map_err(|e| e.to_string())
}
pub fn run() {
    use tauri::Manager;
    tauri::Builder::default()
        .setup(|app| {
            let directory = app.path().app_data_dir()?;
            std::fs::create_dir_all(&directory)?;
            app.manage(Arc::new(catalog::Catalog::new(
                directory.join("library-tauri-v1.sqlite"),
            )));
            if let Ok(mut controller) = app.state::<Shared>().lock() {
                if let Err(error) = controller.attach_session(directory.join("playback-v1.json")) {
                    eprintln!("{error}");
                }
            }
            desktop::setup(app)?;
            media::setup(app);
            let handle = app.handle().clone();
            // Queue progression follows engine EOF on Rust's worker even with the UI hidden.
            let player = app.state::<Shared>().inner().clone();
            std::thread::Builder::new()
                .name("player-controller".into())
                .spawn(move || {
                    let mut last_path = None;
                    loop {
                        if handle
                            .state::<desktop::Desktop>()
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
                            desktop::update(&handle, &snapshot);
                            media::update(&handle, &snapshot);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                })?;
            Ok(())
        })
        .on_window_event(desktop::window_event)
        .manage(Arc::new(Mutex::new(Controller::new())))
        .invoke_handler(tauri::generate_handler![
            desktop::desktop_preferences,
            media::media_status,
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
            catalog_commands::library_query,
            catalog_commands::library_summary,
            catalog_commands::library_home,
            catalog_commands::library_scan,
            catalog_commands::library_progress,
            catalog_commands::library_cancel,
            catalog_commands::library_track,
            catalog_commands::library_audio_info,
            catalog_commands::library_cover,
            catalog_commands::library_play,
            catalog_commands::library_favorite,
            catalog_commands::library_playlist,
            catalog_commands::library_playlist_edit,
            catalog_commands::library_import,
            catalog_commands::library_queue,
            catalog_commands::library_queue_tracks
        ])
        .build(tauri::generate_context!())
        .expect("无法启动 Music Mine 桌面窗口")
        .run(|app, event| {
            if matches!(event, tauri::RunEvent::ExitRequested { .. }) {
                media::shutdown(app);
                desktop::shutdown(app);
            }
        });
}
