use crate::{catalog::*, collections, execute, metadata, Action, PlayerSnapshot, Shared};
use base64::Engine;
use std::sync::{atomic::Ordering, Arc};
type State<'a> = tauri::State<'a, Arc<Catalog>>;
async fn work<T: Send + 'static>(
    catalog: Arc<Catalog>,
    f: impl FnOnce(&Catalog) -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(move || f(&catalog))
        .await
        .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn library_query(query: Query, state: State<'_>) -> Result<Page, String> {
    work(state.inner().clone(), move |s| s.query(query)).await
}
#[tauri::command]
pub async fn library_summary(state: State<'_>) -> Result<Summary, String> {
    work(state.inner().clone(), |s| s.summary()).await
}
#[tauri::command]
pub fn library_progress(state: State<'_>) -> Progress {
    state.progress.lock().unwrap().clone()
}
#[tauri::command]
pub fn library_cancel(state: State<'_>) {
    state.cancel.store(true, Ordering::Relaxed);
}
#[tauri::command]
pub async fn library_scan(
    state: State<'_>,
    covers: tauri::State<'_, Arc<crate::covers::Store>>,
) -> Result<(), String> {
    let folder = tauri::async_runtime::spawn_blocking(|| rfd::FileDialog::new().pick_folder())
        .await
        .map_err(|e| e.to_string())?;
    let Some(folder) = folder else {
        return Ok(());
    };
    let catalog = state.inner().clone();
    let covers = covers.inner().clone();
    {
        let mut p = catalog.progress.lock().unwrap();
        if p.running {
            return Err("扫描已经在进行".into());
        }
        *p = Progress {
            running: true,
            ..Default::default()
        };
    }
    catalog.cancel.store(false, Ordering::Relaxed);
    covers.clear_memory();
    tauri::async_runtime::spawn_blocking(move || {
        let result = catalog.scan(folder);
        let mut p = catalog.progress.lock().unwrap();
        p.running = false;
        if let Err(e) = result {
            p.errors.push(e);
        }
    });
    Ok(())
}
#[tauri::command]
pub async fn library_track(id: i64, state: State<'_>) -> Result<crate::library::Track, String> {
    work(state.inner().clone(), move |s| s.track(id)).await
}
#[derive(serde::Serialize)]
pub struct AudioInfo {
    format: String,
    sample_rate: Option<u32>,
    channels: Option<u8>,
    bitrate: Option<u32>,
}
/// Read-only file inspection; no library schema changes or audio engine work.
#[tauri::command]
pub async fn library_audio_info(id: i64, state: State<'_>) -> Result<AudioInfo, String> {
    work(state.inner().clone(), move |s| {
        let track = s.track(id)?;
        read_audio_info(std::path::Path::new(&track.path))
    })
    .await
}
fn read_audio_info(path: &std::path::Path) -> Result<AudioInfo, String> {
    use lofty::file::{AudioFile, TaggedFileExt};
    let file = lofty::read_from_path(path).map_err(|e| e.to_string())?;
    let properties = file.properties();
    Ok(AudioInfo {
        format: format!("{:?}", file.file_type()),
        sample_rate: properties.sample_rate().filter(|v| *v > 0),
        channels: properties.channels().filter(|v| *v > 0),
        bitrate: properties.audio_bitrate().filter(|v| *v > 0),
    })
}
#[cfg(test)]
mod audio_info_tests {
    use super::*;
    #[test]
    fn reads_real_properties_without_changing_fixture() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.wav");
        let before = std::fs::read(&path).unwrap();
        let info = read_audio_info(&path).unwrap();
        assert_eq!(info.format, "Wav");
        let rate = u32::from_le_bytes(before[24..28].try_into().unwrap());
        let channels = u16::from_le_bytes(before[22..24].try_into().unwrap());
        assert_eq!(info.sample_rate, Some(rate));
        assert_eq!(info.channels, Some(channels as u8));
        assert_eq!(std::fs::read(&path).unwrap(), before);
        assert!(read_audio_info(&path.with_extension("missing")).is_err());
    }
}
#[tauri::command]
pub async fn library_cover(id: i64, state: State<'_>) -> Result<Option<String>, String> {
    work(state.inner().clone(), move |s| {
        let t = s.track(id)?;
        Ok(
            metadata::read_cover(std::path::Path::new(&t.path)).and_then(|bytes| {
                let mime = if bytes.starts_with(b"\x89PNG") {
                    "image/png"
                } else if bytes.starts_with(b"\xff\xd8") {
                    "image/jpeg"
                } else {
                    return None;
                };
                if bytes.len() > 8 * 1024 * 1024 {
                    return None;
                }
                Some(format!(
                    "data:{mime};base64,{}",
                    base64::engine::general_purpose::STANDARD.encode(bytes)
                ))
            }),
        )
    })
    .await
}
#[tauri::command]
pub async fn library_thumb(
    id: i64,
    edge: u32,
    state: State<'_>,
    covers: tauri::State<'_, Arc<crate::covers::Store>>,
) -> Result<Option<String>, String> {
    let catalog = state.inner().clone();
    let covers = covers.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let Some(track) = catalog.track_present(id)? else {
            return Ok(None);
        };
        Ok(covers.for_track(track.id, std::path::Path::new(&track.path), edge))
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn library_queue_thumb(
    path: String,
    edge: u32,
    state: State<'_>,
    covers: tauri::State<'_, Arc<crate::covers::Store>>,
) -> Result<Option<String>, String> {
    let catalog = state.inner().clone();
    let covers = covers.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let Some(track) = catalog.track_by_path(&path)? else {
            return Ok(None);
        };
        Ok(covers.for_track(track.id, std::path::Path::new(&track.path), edge))
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn library_play(
    id: i64,
    state: State<'_>,
    player: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    let track = work(state.inner().clone(), move |s| s.track(id)).await?;
    execute(player.inner().clone(), Action::Start(track.path.into())).await
}
#[tauri::command]
pub async fn library_queue(
    action: String,
    id: Option<i64>,
    path: Option<String>,
    state: State<'_>,
    player: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    let action = match action.as_str() {
        "next" => Action::Next,
        "previous" => Action::Previous,
        "clear" => Action::ClearUpcoming,
        "remove" => Action::Remove(path.ok_or("缺少歌曲路径")?),
        "add" | "play_next" => {
            let t = work(state.inner().clone(), move |s| {
                s.track(id.ok_or("缺少歌曲 ID")?)
            })
            .await?;
            let track = queue_track(t);
            if action == "play_next" {
                Action::PlayNext(track)
            } else {
                Action::Enqueue(track)
            }
        }
        _ => return Err("不支持的队列操作".into()),
    };
    execute(player.inner().clone(), action).await
}
fn queue_track(t: crate::library::Track) -> crate::controller::Track {
    crate::controller::Track {
        path: t.path,
        title: t.title,
        artist: t.artist,
        album: t.album,
        duration_ms: t.duration_ms,
    }
}
/// Plays (replacing the queue) or appends a list of library tracks, e.g. an album or playlist.
#[tauri::command]
pub async fn library_queue_tracks(
    ids: Vec<i64>,
    replace: bool,
    state: State<'_>,
    player: tauri::State<'_, Shared>,
) -> Result<PlayerSnapshot, String> {
    if ids.len() > 20_000 {
        return Err("一次加入的歌曲过多".into());
    }
    let tracks = work(state.inner().clone(), move |s| s.tracks_by_ids(&ids)).await?;
    if tracks.is_empty() {
        return Err("没有可播放的歌曲".into());
    }
    let tracks = tracks.into_iter().map(queue_track).collect();
    let action = if replace {
        Action::Replace(tracks)
    } else {
        Action::EnqueueMany(tracks)
    };
    execute(player.inner().clone(), action).await
}
#[tauri::command]
pub async fn library_favorite(id: i64, on: bool, state: State<'_>) -> Result<(), String> {
    work(state.inner().clone(), move |s| {
        let _guard = s.operation.lock().map_err(|e| e.to_string())?;
        collections::set_favorite(&s.connection()?, id, on)
    })
    .await
}
#[tauri::command]
pub async fn library_playlist(
    name: Option<String>,
    playlist_id: Option<i64>,
    track_id: Option<i64>,
    state: State<'_>,
) -> Result<Option<i64>, String> {
    work(state.inner().clone(), move |s| {
        let _guard = s.operation.lock().map_err(|e| e.to_string())?;
        let conn = s.connection()?;
        if let Some(name) = name {
            return collections::create_playlist(&conn, &name).map(Some);
        } else if let (Some(p), Some(t)) = (playlist_id, track_id) {
            collections::add_to_playlist(&conn, p, t)?;
        } else {
            return Err("请选择歌单和歌曲".into());
        }
        Ok(None)
    })
    .await
}
/// Rename, delete or remove a track from a playlist by its ID. Only SQLite rows change;
/// library tracks and audio files are never touched.
#[tauri::command]
pub async fn library_playlist_edit(
    action: String,
    playlist_id: i64,
    name: Option<String>,
    track_id: Option<i64>,
    state: State<'_>,
) -> Result<(), String> {
    work(state.inner().clone(), move |s| {
        let _guard = s.operation.lock().map_err(|e| e.to_string())?;
        let conn = s.connection()?;
        match action.as_str() {
            "rename" => {
                collections::rename_playlist(&conn, playlist_id, &name.ok_or("缺少歌单名称")?)
            }
            "delete" => collections::delete_playlist(&conn, playlist_id),
            "remove_track" => collections::remove_from_playlist(
                &conn,
                playlist_id,
                track_id.ok_or("缺少歌曲 ID")?,
            ),
            _ => Err("不支持的歌单操作".into()),
        }
    })
    .await
}
#[tauri::command]
pub async fn library_import(state: State<'_>) -> Result<Option<ImportReport>, String> {
    let file = tauri::async_runtime::spawn_blocking(|| {
        rfd::FileDialog::new()
            .set_title("选择旧版 SQLite 数据库（只读安全备份）")
            .add_filter("SQLite", &["db", "sqlite", "sqlite3"])
            .pick_file()
    })
    .await
    .map_err(|e| e.to_string())?;
    match file {
        Some(file) => {
            work(state.inner().clone(), move |s| {
                s.import_copy(&file).map(Some)
            })
            .await
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn library_home(state: State<'_>) -> Result<Home, String> {
    work(state.inner().clone(), |s| s.home()).await
}
