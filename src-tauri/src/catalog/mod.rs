pub(crate) mod collections;
mod db;
pub(crate) mod library;
pub(crate) mod metadata;
use library::Track;
const FIELDS: &str = "t.id,t.path,t.title,t.artist,t.album,t.album_artist,t.genre,t.year,t.track_number,t.duration_ms,t.disc_number,t.album_source,t.added_at";
fn row_track(r: &rusqlite::Row<'_>) -> rusqlite::Result<Track> {
    Ok(Track {
        id: r.get(0)?,
        path: r.get(1)?,
        title: r.get(2)?,
        artist: r.get(3)?,
        album: r.get(4)?,
        album_artist: r.get(5)?,
        genre: r.get(6)?,
        year: r.get(7)?,
        track_number: r.get(8)?,
        duration_ms: r.get(9)?,
        disc_number: r.get(10)?,
        album_source: library::AlbumSource::parse(&r.get::<_, String>(11)?),
        added_at: r.get(12)?,
    })
}
// Compare stable legacy data before/after migrating the backup, including membership order.
fn legacy_snapshot(conn: &Connection) -> Result<Vec<Vec<Vec<String>>>, String> {
    let mut snapshot = Vec::new();
    for (table, fields) in [
        (
            "tracks",
            "id,path,title,artist,album,album_artist,genre,year,track_number,duration_ms",
        ),
        ("favorites", "*"),
        ("playlists", "*"),
        ("playlist_tracks", "*"),
        ("folders", "*"),
        ("settings", "*"),
    ] {
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
                [table],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        if !exists {
            if table == "tracks" {
                return Err("所选文件不是支持的旧版音乐库".into());
            }
            snapshot.push(vec![]);
            continue;
        }
        let mut statement = conn
            .prepare(&format!("SELECT {fields} FROM {table} ORDER BY 1"))
            .map_err(|e| e.to_string())?;
        let columns = statement.column_count();
        let mut rows = statement.query([]).map_err(|e| e.to_string())?;
        let mut values = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            values.push(
                (0..columns)
                    .map(|i| row.get_ref(i).map(|v| format!("{v:?}")))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.to_string())?,
            );
        }
        values.sort();
        snapshot.push(values);
    }
    Ok(snapshot)
}
use rusqlite::{params, Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

pub struct Catalog {
    initialized: Mutex<bool>,
    #[cfg(test)]
    initialization_count: std::sync::atomic::AtomicUsize,
    pub path: PathBuf,
    pub progress: Mutex<Progress>,
    pub cancel: AtomicBool,
    pub operation: Mutex<()>,
}
#[derive(Clone, Default, Serialize)]
pub struct Progress {
    pub running: bool,
    pub visited: usize,
    pub imported: usize,
    pub current: String,
    pub cancelled: bool,
    pub errors: Vec<String>,
}
#[derive(Deserialize)]
#[serde(default)]
pub struct Query {
    pub search: String,
    pub sort: String,
    pub offset: usize,
    pub limit: usize,
    pub favorite: bool,
    pub playlist: Option<i64>,
    pub album: Option<String>,
    pub artist: Option<String>,
}
impl Default for Query {
    fn default() -> Self {
        Self {
            search: String::new(),
            sort: "added".into(),
            offset: 0,
            limit: 100,
            favorite: false,
            playlist: None,
            album: None,
            artist: None,
        }
    }
}
#[derive(Serialize)]
pub struct Home {
    pub recent: Vec<Track>,
    pub artists: Vec<(String, i64)>,
}
#[derive(Serialize)]
pub struct Page {
    pub tracks: Vec<Track>,
    pub total: usize,
    pub favorites: Vec<i64>,
}
#[derive(Serialize)]
pub struct Group {
    pub key: String,
    pub name: String,
    pub artist: String,
    pub count: usize,
    pub cover_id: i64,
}
#[derive(Serialize)]
pub struct Summary {
    pub tracks: usize,
    pub albums: Vec<Group>,
    pub artists: Vec<Group>,
    pub folders: Vec<String>,
    pub playlists: Vec<Playlist>,
}
#[derive(Serialize)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub count: usize,
}
#[derive(Serialize, PartialEq, Debug)]
pub struct ImportReport {
    pub tracks: i64,
    pub favorites: i64,
    pub playlists: i64,
    pub entries: i64,
    pub warnings: Vec<String>,
}
impl Catalog {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            initialized: Mutex::new(false),
            #[cfg(test)]
            initialization_count: std::sync::atomic::AtomicUsize::new(0),
            progress: Mutex::new(Progress::default()),
            cancel: AtomicBool::new(false),
            operation: Mutex::new(()),
        }
    }
    fn ensure_initialized(&self) -> Result<(), String> {
        let mut initialized = self
            .initialized
            .lock()
            .map_err(|_| "曲库初始化锁已毒化，无法安全初始化".to_string())?;
        if !*initialized {
            #[cfg(test)]
            self.initialization_count.fetch_add(1, Ordering::SeqCst);
            drop(db::open(&self.path)?);
            *initialized = true;
        }
        Ok(())
    }
    pub fn connection(&self) -> Result<Connection, String> {
        self.ensure_initialized()?;
        db::open_light(&self.path)
    }
    pub fn record_listen(&self, path: &str) -> Result<(), String> {
        self.connection()?.execute("INSERT INTO listening_history(track_id,last_played,plays) SELECT id,strftime('%s','now'),1 FROM tracks WHERE path=?1 ON CONFLICT(track_id) DO UPDATE SET last_played=excluded.last_played,plays=plays+1", [path]).map_err(|e|e.to_string())?;
        Ok(())
    }
    pub fn home(&self) -> Result<Home, String> {
        let conn = self.connection()?;
        let mut stmt=conn.prepare(&format!("SELECT {FIELDS} FROM tracks t JOIN listening_history h ON h.track_id=t.id ORDER BY h.last_played DESC,t.id DESC LIMIT 12")).map_err(|e|e.to_string())?;
        let recent = stmt
            .query_map([], row_track)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        let mut stmt=conn.prepare("SELECT t.artist,SUM(h.plays) FROM tracks t JOIN listening_history h ON h.track_id=t.id GROUP BY t.artist ORDER BY SUM(h.plays) DESC,t.artist LIMIT 8").map_err(|e|e.to_string())?;
        let artists = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(Home { recent, artists })
    }
    pub fn track(&self, id: i64) -> Result<Track, String> {
        self.connection()?
            .query_row(
                &format!("SELECT {FIELDS} FROM tracks t WHERE t.id=?1"),
                [id],
                row_track,
            )
            .map_err(|e| format!("读取歌曲失败：{e}"))
    }
    /// A missing row is `Ok(None)`. Database failures stay errors.
    pub fn track_present(&self, id: i64) -> Result<Option<Track>, String> {
        self.lookup(&format!("SELECT {FIELDS} FROM tracks t WHERE t.id=?1"), &id)
            .map_err(|e| format!("读取歌曲失败：{e}"))
    }
    /// Exact library path. Titles are never consulted. Unknown paths are `Ok(None)`.
    pub fn track_by_path(&self, path: &str) -> Result<Option<Track>, String> {
        if path.is_empty() || path.len() > 1024 {
            return Ok(None);
        }
        self.lookup(
            &format!("SELECT {FIELDS} FROM tracks t WHERE t.path=?1"),
            &path,
        )
    }
    /// One-row lookup after initialization, with a lightweight read-write fallback.
    fn lookup(&self, sql: &str, param: &dyn rusqlite::ToSql) -> Result<Option<Track>, String> {
        self.ensure_initialized()?;
        let run = |conn: &Connection| match conn.query_row(sql, [param], row_track) {
            Ok(track) => Ok(Some(track)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        };
        if let Ok(conn) = db::open_read(&self.path) {
            if let Ok(found) = run(&conn) {
                return Ok(found);
            }
        }
        run(&db::open_light(&self.path)?).map_err(|e| e.to_string())
    }
    /// Loads tracks in the given order over one connection, skipping unknown IDs.
    pub fn tracks_by_ids(&self, ids: &[i64]) -> Result<Vec<Track>, String> {
        let conn = self.connection()?;
        let mut stmt = conn
            .prepare(&format!("SELECT {FIELDS} FROM tracks t WHERE t.id=?1"))
            .map_err(|e| e.to_string())?;
        let mut tracks = Vec::with_capacity(ids.len());
        for id in ids {
            match stmt.query_row([id], row_track) {
                Ok(t) => tracks.push(t),
                Err(rusqlite::Error::QueryReturnedNoRows) => {}
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(tracks)
    }
    pub fn query(&self, q: Query) -> Result<Page, String> {
        let conn = self.connection()?;
        // All user values are bound parameters; LIKE metacharacters are literal search text.
        let pattern = format!(
            "%{}%",
            q.search
                .replace('!', "!!")
                .replace('%', "!%")
                .replace('_', "!_")
        );
        let order = match q.sort.as_str() {
            "title" => "t.title COLLATE NOCASE,t.id",
            "artist" => "t.artist COLLATE NOCASE,t.id",
            "album" => "t.album COLLATE NOCASE,t.disc_number,t.track_number,t.id",
            "duration" => "t.duration_ms,t.id",
            _ => "t.added_at DESC,t.id DESC",
        };
        let filter = "FROM tracks t WHERE (t.title LIKE ?1 ESCAPE '!' OR t.artist LIKE ?1 ESCAPE '!' OR t.album LIKE ?1 ESCAPE '!') AND (?2=0 OR EXISTS(SELECT 1 FROM favorites f WHERE f.track_id=t.id)) AND (?3 IS NULL OR EXISTS(SELECT 1 FROM playlist_tracks p WHERE p.track_id=t.id AND p.playlist_id=?3)) AND (?4 IS NULL OR t.id IN (SELECT value FROM json_each(?4))) AND (?5 IS NULL OR t.artist=?5)";
        let tracks = if q.album.is_some() {
            db::load_tracks(&conn)?
        } else {
            Vec::new()
        };
        let album_ids = q.album.as_ref().map(|key| {
            let indices = library::group_albums(&tracks)
                .into_iter()
                .find(|g| &g.key == key)
                .map(|g| g.tracks)
                .unwrap_or_default();
            serde_json::to_string(
                &indices
                    .into_iter()
                    .map(|i| tracks[i].id)
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        });
        let total = conn
            .query_row(
                &format!("SELECT COUNT(*) {filter}"),
                params![pattern, q.favorite, q.playlist, album_ids, q.artist],
                |r| r.get::<_, usize>(0),
            )
            .map_err(|e| e.to_string())?;
        let order = if q.playlist.is_some() {
            "(SELECT position FROM playlist_tracks p WHERE p.track_id=t.id AND p.playlist_id=?3)"
        } else {
            order
        };
        let mut stmt = conn
            .prepare(&format!(
                "SELECT {FIELDS} {filter} ORDER BY {order} LIMIT ?6 OFFSET ?7"
            ))
            .map_err(|e| e.to_string())?;
        let tracks = stmt
            .query_map(
                params![
                    pattern,
                    q.favorite,
                    q.playlist,
                    album_ids,
                    q.artist,
                    q.limit.clamp(1, 200),
                    q.offset.min(i64::MAX as usize)
                ],
                row_track,
            )
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(Page {
            tracks,
            total,
            favorites: collections::favorite_ids(&conn).into_iter().collect(),
        })
    }
    pub fn summary(&self) -> Result<Summary, String> {
        let conn = self.connection()?;
        let tracks = db::load_tracks(&conn)?;
        let albums = library::group_albums(&tracks)
            .into_iter()
            .map(|g| Group {
                key: g.key,
                name: g.name,
                artist: g.artist,
                count: g.tracks.len(),
                cover_id: tracks[g.tracks[0]].id,
            })
            .collect();
        let mut artists: std::collections::BTreeMap<String, Vec<&Track>> = Default::default();
        for t in &tracks {
            artists.entry(t.artist.clone()).or_default().push(t);
        }
        Ok(Summary {
            tracks: tracks.len(),
            albums,
            artists: artists
                .into_iter()
                .map(|(name, ts)| Group {
                    key: name.clone(),
                    name,
                    artist: String::new(),
                    count: ts.len(),
                    cover_id: ts[0].id,
                })
                .collect(),
            folders: db::load_folders(&conn)?,
            playlists: collections::list_playlists(&conn)
                .into_iter()
                .map(|p| Playlist {
                    id: p.id,
                    name: p.name,
                    count: p.track_count,
                })
                .collect(),
        })
    }
    fn error(&self, value: String) {
        let mut p = self.progress.lock().unwrap();
        if p.errors.len() < 100 {
            p.errors.push(value);
        }
    }
    pub fn scan(&self, folder: PathBuf) -> Result<(), String> {
        let _guard = self.operation.lock().map_err(|e| e.to_string())?;
        let conn = self.connection()?;
        if folder.is_dir() {
            db::add_folder(&conn, &folder.to_string_lossy())?;
        }
        let mut dirs = vec![folder];
        while let Some(dir) = dirs.pop() {
            if self.cancel.load(Ordering::Relaxed) {
                self.progress.lock().unwrap().cancelled = true;
                break;
            }
            let entries = match std::fs::read_dir(&dir) {
                Ok(e) => e,
                Err(e) => {
                    self.error(format!("{}：{e}", dir.display()));
                    continue;
                }
            };
            let mut batch = Vec::new();
            for entry in entries {
                if self.cancel.load(Ordering::Relaxed) {
                    self.progress.lock().unwrap().cancelled = true;
                    break;
                }
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        self.error(e.to_string());
                        continue;
                    }
                };
                let path = entry.path();
                let ty = match entry.file_type() {
                    Ok(t) => t,
                    Err(e) => {
                        self.error(format!("{}：{e}", path.display()));
                        continue;
                    }
                };
                // Never traverse links/junctions, avoiding recursive cycles and duplicate imports.
                if ty.is_symlink() {
                    continue;
                }
                if ty.is_dir() {
                    dirs.push(path);
                    continue;
                }
                if !library::is_supported(&path) {
                    continue;
                }
                {
                    let mut p = self.progress.lock().unwrap();
                    p.visited += 1;
                    p.current = path.to_string_lossy().into_owned();
                }
                match metadata::read_track(&path, 0) {
                    Some(t) => batch.push(t),
                    None => self.error(format!("无法读取元数据：{}", path.display())),
                }
            }
            library::fill_album_fallbacks(&mut batch);
            db::upsert_tracks(&conn, &batch)?;
            self.progress.lock().unwrap().imported += batch.len();
        }
        Ok(())
    }
    pub fn import_copy(&self, source: &Path) -> Result<ImportReport, String> {
        let _guard = self
            .operation
            .try_lock()
            .map_err(|_| "扫描或导入正在进行")?;
        let conn = self.connection()?;
        for table in ["tracks", "favorites", "playlists", "settings", "folders"] {
            let count: i64 = conn
                .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| r.get(0))
                .map_err(|e| e.to_string())?;
            if count > 0 {
                return Err("仅可导入到空曲库；现有新版数据不会被覆盖".into());
            }
        }
        drop(conn);
        let src = Connection::open_with_flags(source, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|e| e.to_string())?;
        let temp = self.path.with_extension("importing.db");
        if temp.exists() {
            return Err("存在未完成的导入副本，请先检查 importing.db".into());
        }
        let result = (|| {
            let mut dest = Connection::open(&temp).map_err(|e| e.to_string())?;
            {
                let backup =
                    rusqlite::backup::Backup::new(&src, &mut dest).map_err(|e| e.to_string())?;
                backup
                    .run_to_completion(128, std::time::Duration::from_millis(5), None)
                    .map_err(|e| e.to_string())?;
            }
            let integrity: String = dest
                .query_row("PRAGMA integrity_check", [], |r| r.get(0))
                .map_err(|e| e.to_string())?;
            if integrity != "ok" {
                return Err(format!("旧库完整性检查失败：{integrity}"));
            }
            let before = legacy_snapshot(&dest)?;
            drop(dest);
            // Only the consistent backup is migrated, never the source connection.
            let dest = db::open(&temp)?;
            if legacy_snapshot(&dest)? != before {
                return Err("迁移前后歌曲、收藏、歌单顺序或设置不一致，已拒绝导入".into());
            }
            let count = |table: &str| {
                dest.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| {
                    r.get::<_, i64>(0)
                })
                .map_err(|e| e.to_string())
            };
            let mut report = ImportReport {
                tracks: count("tracks")?,
                favorites: count("favorites")?,
                playlists: count("playlists")?,
                entries: count("playlist_tracks")?,
                warnings: vec![],
            };
            db::load_tracks(&dest)?;
            let bad: i64 = dest
                .query_row("SELECT COUNT(*) FROM pragma_foreign_key_check", [], |r| {
                    r.get(0)
                })
                .map_err(|e| e.to_string())?;
            if bad != 0 {
                return Err(format!(
                    "旧库有 {bad} 个无效关联，副本已拒绝导入；原库保持不变"
                ));
            }
            if db::get_setting(&dest, "session_v2").is_some() {
                report.warnings.push("旧版播放队列、循环/随机模式和上次播放位置已原样保留在 session_v2，尚未自动恢复；不会静默丢弃。".into());
            }
            dest.execute_batch("PRAGMA wal_checkpoint(TRUNCATE); PRAGMA journal_mode=DELETE;")
                .map_err(|e| e.to_string())?;
            drop(dest);
            std::fs::rename(&temp, &self.path).map_err(|e| e.to_string())?;
            Ok(report)
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(&temp);
        }
        result
    }
}
#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;
