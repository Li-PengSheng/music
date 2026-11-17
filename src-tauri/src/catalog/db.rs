use std::path::Path;

use rusqlite::Connection;

use super::library::{AlbumSource, Track};

pub fn open(path: &Path) -> Result<Connection, String> {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    configure_connection(&conn)?;
    init(&conn)?;
    Ok(conn)
}

fn configure_connection(conn: &Connection) -> Result<(), String> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|e| e.to_string())?;
    conn.busy_timeout(std::time::Duration::from_secs(2))
        .map_err(|e| e.to_string())
}

/// Opens an initialized database without creating it or running migrations.
pub fn open_light(path: &Path) -> Result<Connection, String> {
    let conn = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)
        .map_err(|e| e.to_string())?;
    configure_connection(&conn)?;
    Ok(conn)
}

/// Read-only connection for hot lookups: no schema checks and no write lock per call.
pub fn open_read(path: &Path) -> Result<Connection, String> {
    let flags =
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX;
    let conn = Connection::open_with_flags(path, flags).map_err(|e| e.to_string())?;
    configure_connection(&conn)?;
    Ok(conn)
}

#[cfg(test)]
pub fn open_in_memory() -> Connection {
    let conn = Connection::open_in_memory().expect("open in-memory db");
    configure_connection(&conn).expect("configure in-memory db");
    init(&conn).expect("init in-memory db");
    conn
}

fn init(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY,
            path TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            album TEXT NOT NULL,
            album_artist TEXT NOT NULL,
            genre TEXT NOT NULL,
            year INTEGER,
            track_number INTEGER,
            duration_ms INTEGER NOT NULL,
            disc_number INTEGER,
            album_source TEXT NOT NULL DEFAULT 'tag',
            added_at INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS listening_history (track_id INTEGER PRIMARY KEY REFERENCES tracks(id) ON DELETE CASCADE, last_played INTEGER NOT NULL, plays INTEGER NOT NULL);
        CREATE TABLE IF NOT EXISTS scrobble_outbox (
            id INTEGER PRIMARY KEY,
            artist TEXT NOT NULL,
            title TEXT NOT NULL,
            album TEXT NOT NULL,
            duration_ms INTEGER NOT NULL,
            listened_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            username TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_scrobble_owner ON scrobble_outbox(username,id);
        CREATE TABLE IF NOT EXISTS folders (
            path TEXT PRIMARY KEY
        );
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS favorites (
            track_id INTEGER PRIMARY KEY REFERENCES tracks(id) ON DELETE CASCADE,
            added_at INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS playlist_tracks (
            playlist_id INTEGER NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
            track_id INTEGER NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
            position INTEGER NOT NULL,
            added_at INTEGER NOT NULL,
            PRIMARY KEY (playlist_id, track_id)
        );
        CREATE INDEX IF NOT EXISTS idx_playlist_tracks ON playlist_tracks(playlist_id, position);",
    )
    .map_err(|e| e.to_string())?;
    // Migration for databases created before V0.2 / V0.3.
    ensure_column(conn, "tracks", "disc_number", "INTEGER")?;
    ensure_column(
        conn,
        "tracks",
        "album_source",
        "TEXT NOT NULL DEFAULT 'tag'",
    )?;
    ensure_column(conn, "tracks", "added_at", "INTEGER NOT NULL DEFAULT 0")?;
    // Existing libraries get a stable "added" time so sorting works.
    conn.execute(
        "UPDATE tracks SET added_at = strftime('%s','now') WHERE added_at = 0",
        [],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn ensure_column(conn: &Connection, table: &str, column: &str, decl: &str) -> Result<(), String> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA table_info({table})"))
        .map_err(|e| e.to_string())?;
    let names: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(1))
        .map_err(|e| e.to_string())?
        .flatten()
        .collect();
    if names.iter().any(|n| n.eq_ignore_ascii_case(column)) {
        return Ok(());
    }
    conn.execute(
        &format!("ALTER TABLE {table} ADD COLUMN {column} {decl}"),
        [],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_tracks(conn: &Connection) -> Result<Vec<Track>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, path, title, artist, album, album_artist, genre, year, track_number, duration_ms, disc_number, album_source, added_at
             FROM tracks
             ORDER BY artist COLLATE NOCASE, album COLLATE NOCASE, disc_number, track_number, title COLLATE NOCASE",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Track {
                id: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                album_artist: row.get(5)?,
                genre: row.get(6)?,
                year: row.get::<_, Option<i64>>(7)?.map(|v| v as u32),
                track_number: row.get::<_, Option<i64>>(8)?.map(|v| v as u32),
                duration_ms: row.get::<_, i64>(9)? as u64,
                disc_number: row.get::<_, Option<i64>>(10)?.map(|v| v as u32),
                album_source: AlbumSource::parse(&row.get::<_, String>(11).unwrap_or_default()),
                added_at: row.get::<_, i64>(12).unwrap_or(0),
            })
        })
        .map_err(|e| e.to_string())?;
    let mut tracks = Vec::new();
    for r in rows {
        tracks.push(r.map_err(|e| e.to_string())?);
    }
    Ok(tracks)
}

pub fn upsert_tracks(conn: &Connection, tracks: &[Track]) -> Result<(), String> {
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    for t in tracks {
        tx.execute(
            "INSERT INTO tracks (path, title, artist, album, album_artist, genre, year, track_number, duration_ms, disc_number, album_source, added_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)
             ON CONFLICT(path) DO UPDATE SET
               title=excluded.title, artist=excluded.artist, album=excluded.album,
               album_artist=excluded.album_artist, genre=excluded.genre, year=excluded.year,
               track_number=excluded.track_number, duration_ms=excluded.duration_ms,
               disc_number=excluded.disc_number, album_source=excluded.album_source",
            rusqlite::params![
                t.path,
                t.title,
                t.artist,
                t.album,
                t.album_artist,
                t.genre,
                t.year.map(|v| v as i64),
                t.track_number.map(|v| v as i64),
                t.duration_ms as i64,
                t.disc_number.map(|v| v as i64),
                t.album_source.as_str(),
                if t.added_at > 0 {
                    t.added_at
                } else {
                    now_secs()
                }
            ],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}

pub fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn load_folders(conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT path FROM folders ORDER BY path")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(|e| e.to_string())?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

pub fn add_folder(conn: &Connection, path: &str) -> Result<(), String> {
    conn.execute("INSERT OR IGNORE INTO folders (path) VALUES (?1)", [path])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn remove_folder(conn: &Connection, path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("缺少文件夹路径".into());
    }
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM folders WHERE path = ?1", [path])
        .map_err(|e| e.to_string())?;
    let normalized = path.replace('/', "\\");
    let normalized = normalized.trim_end_matches('\\');
    let prefix = format!("{normalized}\\")
        .replace('!', "!!")
        .replace('%', "!%")
        .replace('_', "!_");
    tx.execute(
        "DELETE FROM tracks WHERE replace(path, '/', char(92)) = ?1 COLLATE NOCASE
         OR replace(path, '/', char(92)) LIKE ?2 ESCAPE '!'",
        rusqlite::params![normalized, format!("{prefix}%")],
    )
    .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())
}

pub fn get_setting(conn: &Connection, key: &str) -> Option<String> {
    conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
        r.get::<_, String>(0)
    })
    .ok()
}

#[cfg(test)]
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1,?2)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        [key, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::library::Track;
    use super::*;

    fn sample_track(path: &str, title: &str) -> Track {
        Track {
            id: 0,
            path: path.to_string(),
            title: title.to_string(),
            artist: "A".into(),
            album: "B".into(),
            album_artist: String::new(),
            genre: String::new(),
            year: Some(2000),
            track_number: Some(1),
            disc_number: Some(1),
            duration_ms: 123456,
            album_source: AlbumSource::Tag,
            added_at: 0,
        }
    }

    #[test]
    fn remove_folder_treats_wildcards_and_boundaries_as_literal_paths() {
        let conn = open_in_memory();
        add_folder(&conn, r"C:\Music_100%").unwrap();
        let paths = [
            r"C:\Music_100%\a.flac",
            r"C:\Music_100%\nested\b.flac",
            r"C:\MusicA100X\keep.flac",
            r"C:\Music_100%2\keep.flac",
            r"C:/Music_100%/nested/c.flac",
        ];
        for path in paths {
            upsert_tracks(&conn, &[sample_track(path, "Track")]).unwrap();
        }
        remove_folder(&conn, r"C:\Music_100%").unwrap();
        let remaining: Vec<_> = load_tracks(&conn)
            .unwrap()
            .into_iter()
            .map(|t| t.path)
            .collect();
        assert_eq!(remaining, vec![paths[2], paths[3]]);
        assert!(load_folders(&conn).unwrap().is_empty());
    }

    #[test]
    fn remove_folder_rolls_back_if_track_deletion_fails() {
        let conn = open_in_memory();
        add_folder(&conn, r"C:\Music").unwrap();
        upsert_tracks(&conn, &[sample_track(r"C:\Music\a.flac", "Track")]).unwrap();
        conn.execute_batch("CREATE TRIGGER block_delete BEFORE DELETE ON tracks BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
        assert!(remove_folder(&conn, r"C:\Music").is_err());
        assert_eq!(load_folders(&conn).unwrap(), vec![r"C:\Music"]);
        assert_eq!(load_tracks(&conn).unwrap().len(), 1);
    }

    #[test]
    fn roundtrip_tracks_and_settings() {
        let conn = open_in_memory();
        upsert_tracks(
            &conn,
            &[
                sample_track("C:\\Music\\a.mp3", "First"),
                sample_track("C:\\Music\\b.flac", "Second"),
            ],
        )
        .unwrap();
        let loaded = load_tracks(&conn).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].title, "First");
        assert!(loaded[0].id != 0);

        // Upserting the same path updates rather than duplicates.
        upsert_tracks(&conn, &[sample_track("C:\\Music\\a.mp3", "FirstUpdated")]).unwrap();
        let reloaded = load_tracks(&conn).unwrap();
        assert_eq!(reloaded.len(), 2);
        assert!(reloaded.iter().any(|t| t.title == "FirstUpdated"));

        set_setting(&conn, "volume", "0.5").unwrap();
        assert_eq!(get_setting(&conn, "volume").as_deref(), Some("0.5"));

        add_folder(&conn, "C:\\Music").unwrap();
        assert_eq!(load_folders(&conn).unwrap(), vec!["C:\\Music".to_string()]);
        remove_folder(&conn, "C:\\Music").unwrap();
        assert!(load_folders(&conn).unwrap().is_empty());
    }
}

#[cfg(test)]
mod migration_tests {
    use super::*;

    /// A V0.2 database must keep every track when opened by V0.3: new tables
    /// appear, new columns are added, and existing rows survive untouched.
    #[test]
    fn upgrading_a_v02_database_preserves_the_library() {
        let dir = std::env::temp_dir().join("music-mine-tests-migration");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("v02-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);
        {
            let conn = Connection::open(&path).unwrap();
            conn.execute_batch(
                "CREATE TABLE tracks (
                    id INTEGER PRIMARY KEY, path TEXT UNIQUE NOT NULL, title TEXT NOT NULL,
                    artist TEXT NOT NULL, album TEXT NOT NULL, album_artist TEXT NOT NULL,
                    genre TEXT NOT NULL, year INTEGER, track_number INTEGER,
                    duration_ms INTEGER NOT NULL, disc_number INTEGER,
                    album_source TEXT NOT NULL DEFAULT 'tag');
                 CREATE TABLE folders (path TEXT PRIMARY KEY);
                 CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
            )
            .unwrap();
            conn.execute(
                "INSERT INTO tracks (path,title,artist,album,album_artist,genre,year,track_number,duration_ms,disc_number,album_source)
                 VALUES ('C:\\Music\\a.flac','Song','Artist','Album','','Pop',2001,1,1234,NULL,'tag')",
                [],
            )
            .unwrap();
            conn.execute(
                "INSERT INTO settings (key,value) VALUES ('session_v2','{\"queue\":[]}')",
                [],
            )
            .unwrap();
        }

        let conn = open(&path).unwrap();
        let tracks = load_tracks(&conn).unwrap();
        assert_eq!(tracks.len(), 1, "existing library rows must survive");
        assert_eq!(tracks[0].title, "Song");
        assert!(tracks[0].added_at > 0, "migration assigns an added time");
        assert_eq!(
            get_setting(&conn, "session_v2").as_deref(),
            Some("{\"queue\":[]}")
        );
        // The new tables exist and work.
        let favorites: i64 = conn
            .query_row("SELECT COUNT(*) FROM favorites", [], |r| r.get(0))
            .unwrap();
        let playlists: i64 = conn
            .query_row("SELECT COUNT(*) FROM playlists", [], |r| r.get(0))
            .unwrap();
        assert_eq!((favorites, playlists), (0, 0));
    }
}

#[cfg(test)]
mod scan_tests {
    use super::super::library;
    use super::super::metadata;

    #[test]
    fn scans_a_wav_and_reads_its_duration() {
        let dir = std::env::temp_dir().join("music-mine-tests-scan");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("nested")).unwrap();
        let wav = dir.join("nested").join("tone.wav");
        std::fs::copy(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.wav"),
            &wav,
        )
        .unwrap();
        // Files we don't support must be ignored.
        std::fs::write(dir.join("notes.txt"), b"hello").unwrap();

        let tracks = library::scan_folder(&dir);
        assert_eq!(tracks.len(), 1, "expected only the wav file");
        let t = &tracks[0];
        assert_eq!(t.title, "tone", "falls back to the file name");
        assert!(
            t.duration_ms >= 2900 && t.duration_ms <= 3100,
            "duration {}",
            t.duration_ms
        );
        assert!(metadata::read_cover(std::path::Path::new(&t.path)).is_none());
    }
}
