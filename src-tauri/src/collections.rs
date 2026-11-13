//! User collections: favourites and custom playlists (V0.3).
//!
//! All state lives in SQLite next to the library, so it survives restarts and
//! database migrations. Deleting a playlist only removes rows in
//! `playlists` / `playlist_tracks`; the `tracks` table (and therefore the music
//! files on disk) is never touched.

use std::collections::HashSet;

use rusqlite::{params, Connection};

use crate::db::now_secs;

// ------------------------------------------------------------------ favourites

pub fn favorite_ids(conn: &Connection) -> HashSet<i64> {
    let mut stmt = match conn.prepare("SELECT track_id FROM favorites") {
        Ok(s) => s,
        Err(_) => return HashSet::new(),
    };
    stmt.query_map([], |r| r.get::<_, i64>(0))
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
}

pub fn is_favorite(conn: &Connection, track_id: i64) -> bool {
    conn.query_row(
        "SELECT 1 FROM favorites WHERE track_id = ?1",
        [track_id],
        |_| Ok(true),
    )
    .unwrap_or(false)
}

pub fn set_favorite(conn: &Connection, track_id: i64, on: bool) -> Result<(), String> {
    if on {
        conn.execute(
            "INSERT OR IGNORE INTO favorites (track_id, added_at) VALUES (?1, ?2)",
            params![track_id, now_secs()],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute("DELETE FROM favorites WHERE track_id = ?1", [track_id])
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Returns the new state.
pub fn toggle_favorite(conn: &Connection, track_id: i64) -> Result<bool, String> {
    let now = is_favorite(conn, track_id);
    set_favorite(conn, track_id, !now)?;
    Ok(!now)
}

/// Favourite / un-favourite many songs in one transaction (one disk sync for a
/// selection of thousands).
pub fn set_favorites(conn: &Connection, ids: &[i64], on: bool) -> Result<usize, String> {
    if ids.is_empty() {
        return Ok(0);
    }
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    let mut changed = 0usize;
    {
        let mut stmt = tx
            .prepare(if on {
                "INSERT OR IGNORE INTO favorites (track_id, added_at) VALUES (?1, ?2)"
            } else {
                "DELETE FROM favorites WHERE track_id = ?1"
            })
            .map_err(|e| e.to_string())?;
        let now = now_secs();
        for id in ids {
            let rows = if on {
                stmt.execute(params![id, now])
            } else {
                stmt.execute([id])
            }
            .map_err(|e| e.to_string())?;
            changed += rows;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(changed)
}

// ------------------------------------------------------------------- playlists

#[derive(Clone, Debug, PartialEq)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub track_count: usize,
}

pub fn list_playlists(conn: &Connection) -> Vec<Playlist> {
    let mut stmt = match conn.prepare(
        "SELECT p.id, p.name, p.created_at, COUNT(pt.track_id)
         FROM playlists p
         LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id
         GROUP BY p.id
         ORDER BY p.created_at, p.id",
    ) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    stmt.query_map([], |r| {
        Ok(Playlist {
            id: r.get(0)?,
            name: r.get(1)?,
            created_at: r.get(2)?,
            track_count: r.get::<_, i64>(3)? as usize,
        })
    })
    .map(|rows| rows.flatten().collect())
    .unwrap_or_default()
}

fn ensure_unique_name(conn: &Connection, name: &str, except: Option<i64>) -> Result<(), String> {
    let taken: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM playlists WHERE name = ?1 COLLATE NOCASE AND id IS NOT ?2)",
            params![name, except],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    if taken {
        return Err(format!("已存在名为「{name}」的歌单"));
    }
    Ok(())
}

pub fn create_playlist(conn: &Connection, name: &str) -> Result<i64, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("歌单名称不能为空".to_string());
    }
    ensure_unique_name(conn, name, None)?;
    conn.execute(
        "INSERT INTO playlists (name, created_at) VALUES (?1, ?2)",
        params![name, now_secs()],
    )
    .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn rename_playlist(conn: &Connection, id: i64, name: &str) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("歌单名称不能为空".to_string());
    }
    ensure_unique_name(conn, name, Some(id))?;
    let changed = conn
        .execute(
            "UPDATE playlists SET name = ?1 WHERE id = ?2",
            params![name, id],
        )
        .map_err(|e| e.to_string())?;
    if changed == 0 {
        return Err("歌单不存在或已被删除".to_string());
    }
    Ok(())
}

/// Deletes the playlist and its membership rows. Tracks and files stay.
pub fn delete_playlist(conn: &Connection, id: i64) -> Result<(), String> {
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM playlist_tracks WHERE playlist_id = ?1", [id])
        .map_err(|e| e.to_string())?;
    let changed = tx
        .execute("DELETE FROM playlists WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    if changed == 0 {
        return Err("歌单不存在或已被删除".to_string());
    }
    tx.commit().map_err(|e| e.to_string())
}

pub fn playlist_track_ids(conn: &Connection, playlist_id: i64) -> Vec<i64> {
    let mut stmt = match conn
        .prepare("SELECT track_id FROM playlist_tracks WHERE playlist_id = ?1 ORDER BY position")
    {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    stmt.query_map([playlist_id], |r| r.get::<_, i64>(0))
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
}

/// Adds a track at the end. Returns false when it was already in the playlist.
pub fn add_to_playlist(conn: &Connection, playlist_id: i64, track_id: i64) -> Result<bool, String> {
    let next_pos: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(position) + 1, 0) FROM playlist_tracks WHERE playlist_id = ?1",
            [playlist_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    let changed = conn
        .execute(
            "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position, added_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![playlist_id, track_id, next_pos, now_secs()],
        )
        .map_err(|e| e.to_string())?;
    Ok(changed > 0)
}

/// Append many tracks in order, skipping those already in the playlist, in one
/// transaction. Returns how many were added.
pub fn add_many_to_playlist(
    conn: &Connection,
    playlist_id: i64,
    ids: &[i64],
) -> Result<usize, String> {
    if ids.is_empty() {
        return Ok(0);
    }
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    let mut next_pos: i64 = tx
        .query_row(
            "SELECT COALESCE(MAX(position) + 1, 0) FROM playlist_tracks WHERE playlist_id = ?1",
            [playlist_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    let now = now_secs();
    let mut added = 0usize;
    {
        let mut stmt = tx
            .prepare(
                "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position, added_at)
                 VALUES (?1, ?2, ?3, ?4)",
            )
            .map_err(|e| e.to_string())?;
        for id in ids {
            let rows = stmt
                .execute(params![playlist_id, id, next_pos, now])
                .map_err(|e| e.to_string())?;
            if rows > 0 {
                added += 1;
                next_pos += 1;
            }
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(added)
}

pub fn remove_from_playlist(
    conn: &Connection,
    playlist_id: i64,
    track_id: i64,
) -> Result<(), String> {
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    tx.execute(
        "DELETE FROM playlist_tracks WHERE playlist_id = ?1 AND track_id = ?2",
        params![playlist_id, track_id],
    )
    .map_err(|e| e.to_string())?;
    renumber(&tx, playlist_id)?;
    tx.commit().map_err(|e| e.to_string())
}

/// Moves the entry at `from` to index `to` (clamped), renumbering positions.
pub fn move_in_playlist(
    conn: &Connection,
    playlist_id: i64,
    from: usize,
    to: usize,
) -> Result<(), String> {
    let ids = playlist_track_ids(conn, playlist_id);
    if from >= ids.len() {
        return Ok(());
    }
    let to = to.min(ids.len().saturating_sub(1));
    if from == to {
        return Ok(());
    }
    let mut order = ids;
    let moved = order.remove(from);
    order.insert(to, moved);
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    for (pos, id) in order.iter().enumerate() {
        tx.execute(
            "UPDATE playlist_tracks SET position = ?1 WHERE playlist_id = ?2 AND track_id = ?3",
            params![pos as i64, playlist_id, id],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}

fn renumber(conn: &Connection, playlist_id: i64) -> Result<(), String> {
    let ids = playlist_track_ids(conn, playlist_id);
    for (pos, id) in ids.iter().enumerate() {
        conn.execute(
            "UPDATE playlist_tracks SET position = ?1 WHERE playlist_id = ?2 AND track_id = ?3",
            params![pos as i64, playlist_id, id],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::library::{AlbumSource, Track};

    fn seed_tracks(conn: &Connection, count: i64) -> Vec<i64> {
        let mut ids = Vec::new();
        for i in 0..count {
            let t = Track {
                id: 0,
                path: format!("C:\\Music\\{i}.flac"),
                title: format!("Song {i}"),
                artist: "Artist".into(),
                album: "Album".into(),
                album_artist: String::new(),
                genre: "Pop".into(),
                year: None,
                track_number: Some(i as u32 + 1),
                disc_number: None,
                duration_ms: 1000,
                album_source: AlbumSource::Tag,
                added_at: 0,
            };
            db::upsert_tracks(conn, &[t]).unwrap();
            ids.push(conn.last_insert_rowid());
        }
        ids
    }

    #[test]
    fn favorites_toggle_and_persist() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 3);
        assert!(!is_favorite(&conn, ids[0]));
        assert!(toggle_favorite(&conn, ids[0]).unwrap());
        assert!(is_favorite(&conn, ids[0]));
        assert_eq!(favorite_ids(&conn).len(), 1);
        assert!(!toggle_favorite(&conn, ids[0]).unwrap());
        assert!(favorite_ids(&conn).is_empty());

        set_favorite(&conn, ids[1], true).unwrap();
        set_favorite(&conn, ids[1], true).unwrap(); // idempotent
        assert_eq!(favorite_ids(&conn).len(), 1);
    }

    #[test]
    fn playlist_crud_and_order() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 4);
        let pl = create_playlist(&conn, "  夜晚  ").unwrap();
        assert_eq!(list_playlists(&conn)[0].name, "夜晚");

        for id in &ids[0..3] {
            assert!(add_to_playlist(&conn, pl, *id).unwrap());
        }
        // Adding twice does not duplicate.
        assert!(!add_to_playlist(&conn, pl, ids[0]).unwrap());
        assert_eq!(playlist_track_ids(&conn, pl), ids[0..3].to_vec());

        move_in_playlist(&conn, pl, 0, 2).unwrap();
        assert_eq!(playlist_track_ids(&conn, pl), vec![ids[1], ids[2], ids[0]]);

        remove_from_playlist(&conn, pl, ids[2]).unwrap();
        assert_eq!(playlist_track_ids(&conn, pl), vec![ids[1], ids[0]]);

        rename_playlist(&conn, pl, "新しい名前 🎵").unwrap();
        assert_eq!(list_playlists(&conn)[0].name, "新しい名前 🎵");
        assert_eq!(list_playlists(&conn)[0].track_count, 2);

        delete_playlist(&conn, pl).unwrap();
        assert!(list_playlists(&conn).is_empty());
        assert!(playlist_track_ids(&conn, pl).is_empty());
        // The library itself is untouched.
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 4);
    }

    #[test]
    fn deleting_a_track_removes_it_from_collections() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 2);
        let pl = create_playlist(&conn, "P").unwrap();
        add_to_playlist(&conn, pl, ids[0]).unwrap();
        set_favorite(&conn, ids[0], true).unwrap();

        db::remove_folder(&conn, "C:\\Music").unwrap();
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 0);
        assert!(playlist_track_ids(&conn, pl).is_empty());
        assert!(favorite_ids(&conn).is_empty());
    }

    #[test]
    fn batch_favourites_and_playlist_append() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 4);
        let added = set_favorites(&conn, &ids[0..3], true).unwrap();
        assert_eq!(added, 3);
        assert_eq!(favorite_ids(&conn).len(), 3);
        // Re-favouriting already-favourited rows changes nothing.
        assert_eq!(set_favorites(&conn, &ids[0..3], true).unwrap(), 0);
        assert_eq!(set_favorites(&conn, &ids[0..2], false).unwrap(), 2);
        assert_eq!(favorite_ids(&conn), [ids[2]].into_iter().collect());

        let pl = create_playlist(&conn, "批量").unwrap();
        assert_eq!(add_many_to_playlist(&conn, pl, &ids).unwrap(), 4);
        assert_eq!(
            add_many_to_playlist(&conn, pl, &ids[1..3]).unwrap(),
            0,
            "already in playlist"
        );
        assert_eq!(playlist_track_ids(&conn, pl), ids);
    }

    #[test]
    fn duplicate_names_and_missing_playlists_report_errors() {
        let conn = db::open_in_memory();
        let ids = seed_tracks(&conn, 2);
        let local = create_playlist(&conn, "本地").unwrap();
        let other = create_playlist(&conn, "Night").unwrap();
        assert!(create_playlist(&conn, " 本地 ").is_err());
        assert!(create_playlist(&conn, "night").is_err());
        assert!(rename_playlist(&conn, other, "本地").is_err());
        rename_playlist(&conn, other, "NIGHT").unwrap();
        add_to_playlist(&conn, local, ids[0]).unwrap();

        delete_playlist(&conn, local).unwrap();
        assert_eq!(list_playlists(&conn).len(), 1);
        assert!(delete_playlist(&conn, local).is_err());
        assert!(rename_playlist(&conn, local, "x").is_err());
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 2);
        assert!(create_playlist(&conn, "本地").is_ok());
    }

    #[test]
    fn deleted_playlist_stays_deleted_after_reopening_the_database() {
        let path = std::env::temp_dir().join(format!(
            "music-mine-playlist-delete-{}.sqlite",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        {
            let conn = db::open(&path).unwrap();
            let ids = seed_tracks(&conn, 1);
            let pl = create_playlist(&conn, "本地").unwrap();
            add_to_playlist(&conn, pl, ids[0]).unwrap();
            delete_playlist(&conn, pl).unwrap();
        }
        let conn = db::open(&path).unwrap();
        assert!(list_playlists(&conn).is_empty());
        assert_eq!(db::load_tracks(&conn).unwrap().len(), 1);
        drop(conn);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn empty_playlist_names_are_rejected() {
        let conn = db::open_in_memory();
        assert!(create_playlist(&conn, "   ").is_err());
        let pl = create_playlist(&conn, "ok").unwrap();
        assert!(rename_playlist(&conn, pl, "").is_err());
        assert_eq!(list_playlists(&conn)[0].name, "ok");
    }
}
