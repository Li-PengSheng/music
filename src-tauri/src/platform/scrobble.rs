use super::desktop::{Desktop, Preferences};
use crate::{
    catalog::{self, library::Track as LibraryTrack},
    infrastructure::logging::LogBuffer,
    playback::controller::{Action, Track},
    Shared,
};
use rusqlite::{params, Connection};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc, Arc, Mutex,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tauri::Manager;

fn threshold(duration_ms: u64) -> Option<u64> {
    (duration_ms >= 30_000).then_some((duration_ms / 2).min(240_000))
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[derive(Default)]
struct Tracker {
    session: Option<u64>,
    seek: u64,
    position: u64,
    playing: bool,
    played: u64,
    sent: bool,
    started: i64,
    announced: bool,
}
impl Tracker {
    /// Pure observation. No I/O; seek serial catches even short forward/back jumps.
    fn observe(
        &mut self,
        session: u64,
        seek: u64,
        position: u64,
        playing: bool,
        elapsed: u64,
        timestamp: i64,
    ) -> bool {
        let new = self.session != Some(session);
        if new {
            *self = Self {
                session: Some(session),
                started: timestamp,
                ..Self::default()
            };
        } else if self.playing && playing && self.seek == seek {
            // Never credit wall time while stalled, or a jump larger than elapsed time.
            self.played = self
                .played
                .saturating_add(position.saturating_sub(self.position).min(elapsed));
        }
        self.seek = seek;
        self.position = position;
        self.playing = playing;
        new
    }
    fn ready(&self, duration: u64) -> bool {
        !self.sent && threshold(duration).is_some_and(|n| self.played >= n)
    }
}

#[derive(Clone)]
struct Listen {
    id: i64,
    artist: String,
    title: String,
    album: String,
    duration_ms: u64,
    listened_at: i64,
}
impl Listen {
    fn from_track(track: &Track, listened_at: i64) -> Self {
        Self {
            id: 0,
            artist: track.artist.chars().take(512).collect(),
            title: track.title.chars().take(512).collect(),
            album: track.album.chars().take(512).collect(),
            duration_ms: track.duration_ms,
            listened_at,
        }
    }
    fn payload(&self, playing_now: bool) -> Value {
        let mut value = json!({"track_metadata": {
            "artist_name": self.artist, "track_name": self.title, "release_name": self.album,
            "additional_info": {"duration_ms": self.duration_ms, "submission_client": "MusicMine", "submission_client_version": "0.1.0"}
        }});
        if !playing_now {
            value["listened_at"] = json!(self.listened_at);
        }
        value
    }
}
fn enqueue(conn: &Connection, username: &str, listen: &Listen) -> Result<(), String> {
    conn.execute("INSERT INTO scrobble_outbox (artist,title,album,duration_ms,listened_at,created_at,username) VALUES (?1,?2,?3,?4,?5,?6,?7)", params![listen.artist,listen.title,listen.album,listen.duration_ms,listen.listened_at,now(),username]).map_err(|_| "OUTBOX_WRITE".to_string())?;
    Ok(())
}
fn pending(conn: &Connection, username: &str) -> Result<Vec<Listen>, String> {
    let mut stmt = conn.prepare("SELECT id,artist,title,album,duration_ms,listened_at FROM scrobble_outbox WHERE username=?1 ORDER BY id LIMIT 50").map_err(|_| "OUTBOX_READ")?;
    let rows = stmt
        .query_map([username], |r| {
            Ok(Listen {
                id: r.get(0)?,
                artist: r.get(1)?,
                title: r.get(2)?,
                album: r.get(3)?,
                duration_ms: r.get(4)?,
                listened_at: r.get(5)?,
            })
        })
        .map_err(|_| "OUTBOX_READ")?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_| "OUTBOX_READ".into())
}
fn acknowledge(conn: &mut Connection, listens: &[Listen]) -> Result<(), String> {
    let tx = conn.transaction().map_err(|_| "OUTBOX_ACK")?;
    for listen in listens {
        tx.execute("DELETE FROM scrobble_outbox WHERE id=?1", [listen.id])
            .map_err(|_| "OUTBOX_ACK")?;
    }
    tx.commit().map_err(|_| "OUTBOX_ACK".into())
}
fn flush_outbox(
    conn: &mut Connection,
    username: &str,
    send: impl FnOnce(Value) -> Result<(), String>,
) -> Result<(), String> {
    let listens = pending(conn, username)?;
    if listens.is_empty() {
        return Ok(());
    }
    send(
        json!({"listen_type":if listens.len() == 1 {"single"} else {"import"}, "payload":listens.iter().map(|l| l.payload(false)).collect::<Vec<_>>()}),
    )?;
    acknowledge(conn, &listens)
}

#[derive(Serialize)]
pub struct Config {
    enabled: bool,
    username: Option<String>,
    token_configured: bool,
    pending_count: u64,
}
fn config(p: &Preferences, conn: &Connection) -> Result<Config, String> {
    let count = conn
        .query_row(
            "SELECT COUNT(*) FROM scrobble_outbox WHERE username=?1",
            [p.listenbrainz_username.as_deref().unwrap_or("")],
            |r| r.get(0),
        )
        .map_err(|_| "OUTBOX_READ")?;
    Ok(Config {
        enabled: p.listenbrainz_enabled,
        username: p.listenbrainz_username.clone(),
        token_configured: p.listenbrainz_token.is_some(),
        pending_count: count,
    })
}
type Reply<T> = mpsc::SyncSender<Result<T, String>>;
enum Job {
    Configure {
        enabled: Option<bool>,
        username: Option<String>,
        token: Option<String>,
        revision: u64,
        reply: Reply<Config>,
    },
    Recommendations(Reply<Vec<LibraryTrack>>),
    PlayingNow {
        username: String,
        session: u64,
        listen: Listen,
    },
}
pub struct Scrobbler {
    jobs: mpsc::SyncSender<Job>,
    revision: AtomicU64,
    collection_revision: AtomicU64,
    commit: Mutex<()>,
}
fn collection_changed(a: &Preferences, b: &Preferences) -> bool {
    a.listenbrainz_enabled != b.listenbrainz_enabled
        || a.listenbrainz_username != b.listenbrainz_username
}

#[derive(Default)]
struct Claims {
    session: Option<u64>,
    owners: HashSet<String>,
}
impl Claims {
    fn contains(&mut self, session: u64, owner: &str) -> bool {
        if self.session != Some(session) {
            self.session = Some(session);
            self.owners.clear();
        }
        self.owners.contains(owner)
    }
    fn insert(&mut self, session: u64, owner: &str) {
        self.contains(session, owner);
        self.owners.insert(owner.into());
    }
}

fn preferences(app: &tauri::AppHandle) -> Result<Preferences, String> {
    app.state::<Desktop>().scrobble_preferences()
}
fn active(p: &Preferences) -> bool {
    p.listenbrainz_enabled && p.listenbrainz_token.is_some() && p.listenbrainz_username.is_some()
}
fn exiting(app: &tauri::AppHandle) -> bool {
    app.state::<Desktop>()
        .exiting
        .load(std::sync::atomic::Ordering::SeqCst)
}
fn log(app: &tauri::AppHandle, operation: &str, code: &str) {
    // Only fixed operation names and sanitized codes. Never log URLs, bodies, headers or credentials.
    let p = preferences(app).ok();
    app.state::<Arc<LogBuffer>>().push_redacted(
        &format!("[listenbrainz] {operation} {code}"),
        p.as_ref().and_then(|p| p.listenbrainz_token.as_deref()),
    );
}

#[tauri::command]
pub async fn listenbrainz_config(
    app: tauri::AppHandle,
    enabled: Option<bool>,
    username: Option<String>,
    token: Option<String>,
) -> Result<Config, String> {
    tauri::async_runtime::spawn_blocking(move || {
        // Query and disable are local operations, even when a request is in flight.
        if username.is_none() && token.is_none() {
            let service = app.state::<Scrobbler>();
            let _guard = service.commit.lock().map_err(|_| "配置锁不可用")?;
            let mut p = preferences(&app)?;
            if let Some(enabled) = enabled {
                if enabled && (p.listenbrainz_token.is_none() || p.listenbrainz_username.is_none())
                {
                    return Err("启用前请保存并验证 ListenBrainz Token".into());
                }
                service.revision.fetch_add(1, Ordering::SeqCst);
                let changed = p.listenbrainz_enabled != enabled;
                p.listenbrainz_enabled = enabled;
                app.state::<Desktop>().save_scrobble(
                    enabled,
                    p.listenbrainz_username.clone(),
                    p.listenbrainz_token.clone(),
                )?;
                if changed {
                    service.collection_revision.fetch_add(1, Ordering::SeqCst);
                }
                log(&app, "config", if enabled { "ENABLED" } else { "DISABLED" });
            }
            return config(&p, &app.state::<Arc<catalog::Catalog>>().connection()?);
        }
        let (tx, rx) = mpsc::sync_channel(1);
        let service = app.state::<Scrobbler>();
        let revision = {
            let _guard = service.commit.lock().map_err(|_| "配置锁不可用")?;
            service.revision.fetch_add(1, Ordering::SeqCst) + 1
        };
        service
            .jobs
            .try_send(Job::Configure {
                enabled,
                username,
                token,
                revision,
                reply: tx,
            })
            .map_err(|_| "ListenBrainz 后台忙，请稍后重试".to_string())?;
        rx.recv()
            .map_err(|_| "ListenBrainz 后台已停止".to_string())?
    })
    .await
    .map_err(|_| "ListenBrainz 配置任务失败".to_string())?
}
#[tauri::command]
pub async fn listenbrainz_recommendations(
    app: tauri::AppHandle,
) -> Result<Vec<LibraryTrack>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        if !active(&preferences(&app)?) {
            return Ok(Vec::new());
        }
        let (tx, rx) = mpsc::sync_channel(1);
        app.state::<Scrobbler>()
            .jobs
            .try_send(Job::Recommendations(tx))
            .map_err(|_| "ListenBrainz 后台忙，请稍后重试".to_string())?;
        rx.recv()
            .map_err(|_| "ListenBrainz 后台已停止".to_string())?
    })
    .await
    .map_err(|_| "ListenBrainz 推荐任务失败".to_string())?
}

#[derive(Clone)]
struct Candidate {
    title: String,
    artist: String,
}
fn normalized(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
fn match_local(conn: &Connection, candidates: &[Candidate]) -> Result<Vec<LibraryTrack>, String> {
    let tracks = catalog::db::load_tracks(conn)?;
    let mut seen = HashSet::new();
    Ok(candidates
        .iter()
        .filter_map(|c| {
            if c.title.is_empty() || c.artist.is_empty() {
                return None;
            }
            tracks.iter().find(|t| {
                normalized(&t.title) == normalized(&c.title)
                    && normalized(&t.artist) == normalized(&c.artist)
            })
        })
        .filter(|t| seen.insert(t.id))
        .cloned()
        .collect())
}
fn encode(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn recent_candidates(v: &Value) -> Vec<Candidate> {
    v.pointer("/payload/listens")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .take(50)
        .filter_map(|v| {
            let m = v.get("track_metadata")?;
            Some(Candidate {
                title: m.get("track_name")?.as_str()?.into(),
                artist: m.get("artist_name")?.as_str()?.into(),
            })
        })
        .collect()
}

struct Http {
    agent: ureq::Agent,
    next: Instant,
}
fn retry_delay(
    status: u16,
    remaining: Option<&str>,
    reset: Option<&str>,
    retry: Option<&str>,
) -> Duration {
    let seconds = |s: Option<&str>| s.and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
    let limit = if status == 429 || remaining == Some("0") {
        seconds(reset).max(seconds(retry)).max(2)
    } else {
        0
    };
    Duration::from_millis(1100).max(Duration::from_secs(limit.max(seconds(retry))))
}
#[derive(Clone, Copy)]
enum RequestKind {
    Active,
    Validation(u64),
    PlayingNow(u64),
}
fn check_request(app: &tauri::AppHandle, p: &Preferences, kind: RequestKind) -> Result<(), String> {
    if exiting(app) {
        return Err("STOPPED".into());
    }
    match kind {
        RequestKind::Validation(revision) => {
            if app.state::<Scrobbler>().revision.load(Ordering::SeqCst) != revision {
                return Err("CONFIG_CHANGED".into());
            }
        }
        RequestKind::Active | RequestKind::PlayingNow(_) => {
            if !same_active(&preferences(app)?, p) {
                return Err("CONFIG_CHANGED".into());
            }
            if let RequestKind::PlayingNow(session) = kind {
                let current = app
                    .state::<Shared>()
                    .lock()
                    .ok()
                    .and_then(|mut c| c.execute(Action::State).ok());
                if !current
                    .is_some_and(|s| s.listen_session == session && s.playback_state == "playing")
                {
                    return Err("STALE_PLAYING_NOW".into());
                }
            }
        }
    }
    Ok(())
}
impl Http {
    fn new() -> Self {
        Self {
            agent: ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(10))
                .redirects(0)
                .user_agent("MusicMine/0.1.0")
                .build(),
            next: Instant::now(),
        }
    }
    fn request(
        &mut self,
        app: &tauri::AppHandle,
        p: &Preferences,
        path: &str,
        payload: Option<Value>,
        kind: RequestKind,
    ) -> Result<Value, String> {
        while Instant::now() < self.next {
            check_request(
                app,
                p,
                match kind {
                    RequestKind::PlayingNow(_) => RequestKind::Active,
                    other => other,
                },
            )?;
            std::thread::sleep(Duration::from_millis(50));
        }
        check_request(app, p, kind)?;
        let url = format!("https://api.listenbrainz.org/1/{path}");
        let request = self
            .agent
            .request(if payload.is_some() { "POST" } else { "GET" }, &url)
            .set(
                "Authorization",
                &format!("Token {}", p.listenbrainz_token.as_deref().unwrap_or("")),
            );
        let response = match payload {
            Some(v) => request.send_json(v),
            None => request.call(),
        };
        let response = match response {
            Ok(r) | Err(ureq::Error::Status(_, r)) => r,
            Err(_) => {
                self.next = Instant::now() + Duration::from_millis(1100);
                log(app, "request", "NETWORK");
                return Err("NETWORK".into());
            }
        };
        let status = response.status();
        self.next = Instant::now()
            .checked_add(retry_delay(
                status,
                response.header("X-RateLimit-Remaining"),
                response.header("X-RateLimit-Reset-In"),
                response.header("Retry-After"),
            ))
            .unwrap_or_else(|| Instant::now() + Duration::from_secs(31_536_000));
        log(app, "request", &format!("HTTP_{status}"));
        if !(200..300).contains(&status) {
            return Err(format!("HTTP_{status}"));
        }
        if status == 204 {
            return Ok(Value::Null);
        }
        let result = decode_response(response);
        if let Err(code) = &result {
            log(app, "response", code);
        }
        result
    }
}
fn decode_response(response: ureq::Response) -> Result<Value, String> {
    // Bound untrusted allocation; errors never include response text or credentials.
    use std::io::Read;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(2_000_001)
        .read_to_end(&mut bytes)
        .map_err(|_| "BODY_READ")?;
    if bytes.len() > 2_000_000 {
        return Err("BODY_LIMIT".into());
    }
    serde_json::from_slice(&bytes).map_err(|_| "JSON".into())
}
fn same_active(a: &Preferences, b: &Preferences) -> bool {
    active(a)
        && a.listenbrainz_username == b.listenbrainz_username
        && a.listenbrainz_token == b.listenbrainz_token
}
fn configure(
    app: &tauri::AppHandle,
    http: &mut Http,
    enabled: Option<bool>,
    username: Option<String>,
    token: Option<String>,
    revision: u64,
) -> Result<Config, String> {
    let mut p = preferences(app)?;
    if let Some(v) = enabled {
        p.listenbrainz_enabled = v;
    }
    if let Some(v) = username {
        let v = v.trim();
        if v.len() > 128 || v.chars().any(char::is_control) {
            return Err("用户名无效".into());
        }
        p.listenbrainz_username = (!v.is_empty()).then(|| v.to_string());
    }
    if let Some(v) = token {
        let v = v.trim();
        if v.len() > 256 || !v.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
            return Err("Token 格式无效".into());
        }
        p.listenbrainz_token = (!v.is_empty()).then(|| v.to_string());
    }
    if p.listenbrainz_token.is_some() {
        let result = http.request(
            app,
            &p,
            "validate-token",
            None,
            RequestKind::Validation(revision),
        )?;
        if result.get("valid").and_then(Value::as_bool) != Some(true) {
            return Err("Token 无效，设置未保存".into());
        }
        let owner = result
            .get("user_name")
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .ok_or("验证响应缺少用户名")?;
        if p.listenbrainz_username
            .as_deref()
            .is_some_and(|u| u != owner)
        {
            return Err("Token 所属用户名不一致，设置未保存".into());
        }
        p.listenbrainz_username = Some(owner.into());
    } else if p.listenbrainz_enabled {
        return Err("启用前请填写 ListenBrainz Token".into());
    }
    let service = app.state::<Scrobbler>();
    let _guard = service.commit.lock().map_err(|_| "配置锁不可用")?;
    if service.revision.load(Ordering::SeqCst) != revision {
        return Err("配置已变化，请重新保存".into());
    }
    let changed = collection_changed(&preferences(app)?, &p);
    app.state::<Desktop>().save_scrobble(
        p.listenbrainz_enabled,
        p.listenbrainz_username.clone(),
        p.listenbrainz_token.clone(),
    )?;
    if changed {
        service.collection_revision.fetch_add(1, Ordering::SeqCst);
    }
    log(app, "config", "SAVED token=***");
    config(&p, &app.state::<Arc<catalog::Catalog>>().connection()?)
}

fn fetch_candidates(
    app: &tauri::AppHandle,
    http: &mut Http,
    p: &Preferences,
) -> Result<Vec<Candidate>, String> {
    let user = encode(p.listenbrainz_username.as_deref().ok_or("NO_USERNAME")?);
    let result = http.request(
        app,
        p,
        &format!("cf/recommendation/user/{user}/recording?count=50"),
        None,
        RequestKind::Active,
    );
    let recommendations = match result {
        Ok(v) => v,
        Err(e) if e == "HTTP_404" => Value::Null,
        Err(e) => return Err(e),
    };
    let mbids: Vec<&str> = recommendations
        .pointer("/payload/mbids")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .take(50)
        .filter_map(|v| v.get("recording_mbid")?.as_str())
        .collect();
    if !mbids.is_empty() {
        let metadata = http.request(
            app,
            p,
            "metadata/recording/",
            Some(json!({"recording_mbids":mbids,"inc":"artist release"})),
            RequestKind::Active,
        )?;
        let candidates: Vec<Candidate> = mbids
            .iter()
            .filter_map(|id| {
                let m = metadata.get(*id)?;
                let title = m
                    .pointer("/recording/name")
                    .or_else(|| m.pointer("/recording/title"))
                    .or_else(|| m.get("recording_name"))?
                    .as_str()?;
                let artist = m.pointer("/artist/name")?.as_str()?;
                Some(Candidate {
                    title: title.into(),
                    artist: artist.into(),
                })
            })
            .collect();
        if !candidates.is_empty() {
            return Ok(candidates);
        }
    }
    // Accounts without generated CF results still get a local shelf from recent public listens.
    Ok(recent_candidates(&http.request(
        app,
        p,
        &format!("user/{user}/listens?count=50"),
        None,
        RequestKind::Active,
    )?))
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::sync_channel(32);
    app.manage(Scrobbler {
        jobs: tx.clone(),
        revision: AtomicU64::new(0),
        collection_revision: AtomicU64::new(0),
        commit: Mutex::new(()),
    });
    let handle = app.handle().clone();
    let catalog = app.state::<Arc<catalog::Catalog>>().inner().clone();
    // Database initialization is outside playback locks and uses only the resolved app data path.
    drop(catalog.connection()?);
    let player = app.state::<Shared>().inner().clone();
    let observer_app = handle.clone();
    let observer_catalog = catalog.clone();
    std::thread::Builder::new()
        .name("listenbrainz-observer".into())
        .spawn(move || {
            let mut tracker = Tracker::default();
            let mut owner = None;
            let mut config_revision = 0;
            let mut claims = Claims::default();
            let mut outbox_retry = Instant::now();
            let mut last = Instant::now();
            while !exiting(&observer_app) {
                let elapsed = last.elapsed().as_millis() as u64;
                last = Instant::now();
                if let Ok(p) = preferences(&observer_app) {
                    if active(&p) {
                        let revision = observer_app
                            .state::<Scrobbler>()
                            .collection_revision
                            .load(Ordering::SeqCst);
                        if owner != p.listenbrainz_username || config_revision != revision {
                            tracker = Tracker::default();
                            owner = p.listenbrainz_username.clone();
                            config_revision = revision;
                        }
                        // This guard is dropped before all SQLite, logging or network work.
                        let snapshot = player
                            .try_lock()
                            .ok()
                            .and_then(|mut c| c.execute(Action::State).ok());
                        if let Some(s) = snapshot {
                            if let Some(track) = s
                                .current_track
                                .as_ref()
                                .filter(|_| s.playback_state != "idle")
                            {
                                tracker.observe(
                                    s.listen_session,
                                    s.listen_seek,
                                    s.position_ms,
                                    s.playback_state == "playing",
                                    elapsed,
                                    now(),
                                );
                                tracker.sent |= claims
                                    .contains(s.listen_session, owner.as_deref().unwrap_or(""));
                                if tracker.playing && !tracker.announced {
                                    tracker.started = now();
                                    tracker.announced = tx
                                        .try_send(Job::PlayingNow {
                                            username: owner.clone().unwrap_or_default(),
                                            session: s.listen_session,
                                            listen: Listen::from_track(track, tracker.started),
                                        })
                                        .is_ok();
                                }
                                if tracker.ready(track.duration_ms)
                                    && Instant::now() >= outbox_retry
                                {
                                    let result = (|| {
                                        let service = observer_app.state::<Scrobbler>();
                                        let _guard =
                                            service.commit.lock().map_err(|_| "CONFIG_LOCK")?;
                                        if service.collection_revision.load(Ordering::SeqCst)
                                            != config_revision
                                            || !same_active(&preferences(&observer_app)?, &p)
                                        {
                                            return Err("CONFIG_CHANGED".into());
                                        }
                                        let c = observer_catalog.connection()?;
                                        enqueue(
                                            &c,
                                            owner.as_deref().unwrap_or(""),
                                            &Listen::from_track(track, tracker.started),
                                        )
                                    })();
                                    match result {
                                        Ok(()) => {
                                            tracker.sent = true;
                                            claims.insert(
                                                s.listen_session,
                                                owner.as_deref().unwrap_or(""),
                                            );
                                            log(&observer_app, "outbox", "QUEUED");
                                        }
                                        Err(_) => {
                                            outbox_retry = Instant::now() + Duration::from_secs(2);
                                            log(&observer_app, "outbox", "WRITE_FAILED");
                                        }
                                    }
                                }
                            } else {
                                tracker = Tracker::default();
                            }
                        }
                    } else {
                        tracker = Tracker::default();
                        owner = None;
                    }
                }
                std::thread::sleep(Duration::from_millis(200));
            }
        })?;
    std::thread::Builder::new().name("listenbrainz-network".into()).spawn(move || {
        let mut http = Http::new();
        let mut cache: Option<(String,Instant,Vec<Candidate>)> = None;
        let mut retry = Instant::now();
        let mut failures = 0u32;
        let mut recommendation_failure: Option<(String,Instant)> = None;
        while !exiting(&handle) {
            if let Ok(job) = rx.recv_timeout(Duration::from_millis(200)) {
                match job {
                    Job::Configure { enabled, username, token, revision, reply } => {
                        let result = configure(&handle,&mut http,enabled,username,token,revision);
                        if result.is_ok() { cache = None; recommendation_failure = None; retry = Instant::now(); failures = 0; }
                        if result.is_err() { log(&handle,"config","FAILED token=***"); }
                        let _ = reply.send(result);
                    }
                    Job::PlayingNow { username, session, listen } => {
                        if let Ok(p) = preferences(&handle) {
                            let current = handle.state::<Shared>().try_lock().ok().and_then(|mut c| c.execute(Action::State).ok());
                            if active(&p) && p.listenbrainz_username.as_deref() == Some(&username) && current.is_some_and(|s| s.listen_session == session && s.playback_state == "playing") {
                                let _ = http.request(&handle,&p,"submit-listens",Some(json!({"listen_type":"playing_now","payload":[listen.payload(true)]})),RequestKind::PlayingNow(session));
                            }
                        }
                    }
                    Job::Recommendations(reply) => {
                        let result = (|| {
                            let p = preferences(&handle)?;
                            if !active(&p) { return Ok(Vec::new()); }
                            let user = p.listenbrainz_username.clone().unwrap_or_default();
                            if recommendation_failure.as_ref().is_some_and(|(u,t)| u == &user && t.elapsed() < Duration::from_secs(30)) { return Ok(Vec::new()); }
                            if !cache.as_ref().is_some_and(|(u,t,_)| u == &user && t.elapsed() < Duration::from_secs(600)) {
                                let candidates = match fetch_candidates(&handle,&mut http,&p) {
                                    Ok(c) => c,
                                    Err(e) => { recommendation_failure = Some((user,Instant::now())); return Err(e); }
                                };
                                cache = Some((user,Instant::now(),candidates));
                            }
                            if !same_active(&preferences(&handle)?,&p) { return Ok(Vec::new()); }
                            match_local(&catalog.connection()?, cache.as_ref().map(|(_,_,c)| c.as_slice()).unwrap_or(&[]))
                        })();
                        let _ = reply.send(result);
                    }
                }
            }
            if Instant::now() >= retry {
                if let Ok(p) = preferences(&handle) {
                    if active(&p) {
                        let result = (|| {
                            let mut conn = catalog.connection()?;
                            flush_outbox(&mut conn,p.listenbrainz_username.as_deref().unwrap_or(""), |payload| http.request(&handle,&p,"submit-listens",Some(payload),RequestKind::Active).map(|_| ()))
                        })();
                        if result.is_ok() { failures = 0; } else { failures = failures.saturating_add(1); log(&handle,"outbox","RETAINED_RETRY"); }
                    }
                }
                retry = Instant::now() + Duration::from_secs(if failures == 0 { 2 } else { (2u64.saturating_pow(failures.min(10))).min(900) });
            }
        }
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untrusted_response_errors_never_expose_a_body_and_oversized_bodies_are_rejected() {
        let response = ureq::Response::new(200, "OK", "private-token invalid-json").unwrap();
        assert_eq!(decode_response(response).unwrap_err(), "JSON");
        let huge = "x".repeat(2_000_001);
        assert_eq!(
            decode_response(ureq::Response::new(200, "OK", &huge).unwrap()).unwrap_err(),
            "BODY_LIMIT"
        );
    }

    #[test]
    fn same_account_saves_and_failed_requests_do_not_restart_collection_or_duplicate_a_session() {
        let mut p = Preferences::default();
        p.listenbrainz_enabled = true;
        p.listenbrainz_username = Some("alice".into());
        p.listenbrainz_token = Some("first-token".into());
        let mut saved = p.clone();
        saved.listenbrainz_token = Some("replacement-token".into());
        assert!(!collection_changed(&p, &saved));
        let mut claims = Claims::default();
        assert!(!claims.contains(7, "alice"));
        claims.insert(7, "alice");
        // Observer may reset progress on disable/re-enable or change accounts,
        // but one completed listen for this owner/session remains claimed.
        assert!(claims.contains(7, "alice"));
        assert!(!claims.contains(7, "bob"));
        claims.insert(7, "bob");
        assert!(claims.contains(7, "alice"));
        saved.listenbrainz_enabled = false;
        assert!(collection_changed(&p, &saved));
        assert!(claims.contains(7, "alice"));
        assert!(!claims.contains(8, "alice"));
    }

    fn listen() -> Listen {
        Listen {
            id: 0,
            artist: "Artist".into(),
            title: "Song".into(),
            album: "Album".into(),
            duration_ms: 180_000,
            listened_at: 1_700_000_000,
        }
    }

    #[test]
    fn pauses_seeks_stalls_and_replays_do_not_fabricate_listening_time() {
        let mut t = Tracker::default();
        t.observe(1, 0, 0, true, 0, 100);
        t.observe(1, 0, 10_000, true, 10_000, 110);
        assert_eq!(t.played, 10_000);
        t.observe(1, 0, 10_000, false, 50_000, 160);
        t.observe(1, 0, 10_000, true, 50_000, 210);
        assert_eq!(t.played, 10_000);
        t.observe(1, 1, 150_000, true, 200, 211); // forward seek
        t.observe(1, 2, 200, true, 200, 212); // backward seek, same session
        assert_eq!(t.played, 10_000);
        t.observe(1, 2, 200, true, 10_000, 222); // output stalled
        assert_eq!(t.played, 10_000);
        t.observe(1, 2, 80_200, true, 80_000, 302);
        assert!(t.ready(180_000));
        t.sent = true;
        assert!(!t.ready(180_000));
        t.observe(2, 2, 0, true, 200, 303); // same track replay, new session
        assert_eq!(t.played, 0);
        assert!(!t.ready(180_000));
        assert_eq!(t.started, 303);
    }

    #[test]
    fn queue_survives_failures_and_only_acknowledges_success_for_its_owner() {
        let mut conn = crate::catalog::db::open_in_memory();
        enqueue(&conn, "alice", &listen()).unwrap();
        enqueue(&conn, "bob", &listen()).unwrap();
        assert!(flush_outbox(&mut conn, "alice", |_| Err("HTTP_429".into())).is_err());
        assert_eq!(pending(&conn, "alice").unwrap().len(), 1);
        flush_outbox(&mut conn, "alice", |payload| {
            assert_eq!(payload["listen_type"], "single");
            assert_eq!(payload["payload"][0]["listened_at"], 1_700_000_000);
            Ok(())
        })
        .unwrap();
        assert!(pending(&conn, "alice").unwrap().is_empty());
        assert_eq!(pending(&conn, "bob").unwrap().len(), 1);
        enqueue(&conn, "bob", &listen()).unwrap();
        flush_outbox(&mut conn, "bob", |payload| {
            assert_eq!(payload["listen_type"], "import");
            assert_eq!(payload["payload"].as_array().unwrap().len(), 2);
            Ok(())
        })
        .unwrap();
        assert!(pending(&conn, "bob").unwrap().is_empty());
    }

    #[test]
    fn persisted_queue_reopens_after_an_offline_restart() {
        let path = std::env::temp_dir().join(format!(
            "music-mine-outbox-{}-{}.sqlite",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        {
            let conn = crate::catalog::db::open(&path).unwrap();
            enqueue(&conn, "alice", &listen()).unwrap();
        }
        {
            let conn = crate::catalog::db::open(&path).unwrap();
            assert_eq!(pending(&conn, "alice").unwrap()[0].title, "Song");
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn token_is_persistable_but_public_config_contains_only_presence() {
        let conn = crate::catalog::db::open_in_memory();
        let mut p = Preferences::default();
        p.listenbrainz_token = Some("secret-token-never-log".into());
        p.listenbrainz_username = Some("alice".into());
        let public = serde_json::to_string(&config(&p, &conn).unwrap()).unwrap();
        assert!(!public.contains("secret-token"));
        assert!(public.contains("\"token_configured\":true"));
        assert!(!active(&p)); // credentials alone never opt in
        assert!(serde_json::to_string(&p)
            .unwrap()
            .contains("secret-token-never-log"));
        let old: Preferences = serde_json::from_str("{}").unwrap();
        assert!(!old.listenbrainz_enabled);
        assert!(old.listenbrainz_token.is_none());
    }

    #[test]
    fn playing_now_omits_timestamp_and_rate_limit_never_retries_early() {
        assert_eq!(
            retry_delay(503, None, None, Some("60")),
            Duration::from_secs(60)
        );
        assert!(listen().payload(true).get("listened_at").is_none());
        assert_eq!(
            retry_delay(200, Some("10"), None, None),
            Duration::from_millis(1100)
        );
        assert_eq!(
            retry_delay(429, None, Some("45"), Some("60")),
            Duration::from_secs(60)
        );
        assert_eq!(
            retry_delay(200, Some("0"), Some("120"), None),
            Duration::from_secs(120)
        );
        assert!(retry_delay(429, None, Some("invalid"), None) >= Duration::from_millis(1100));
        assert_eq!(encode("用户 /?#"), "%E7%94%A8%E6%88%B7%20%2F%3F%23");
    }

    #[test]
    fn local_match_requires_artist_and_title_and_deduplicates_in_remote_order() {
        let conn = crate::catalog::db::open_in_memory();
        conn.execute("INSERT INTO tracks (id,path,title,artist,album,album_artist,genre,duration_ms) VALUES (1,'fixture-a',' Song ','ARTIST','','','',180000),(2,'fixture-b','Song','Someone Else','','','',180000)", []).unwrap();
        let candidates = vec![
            Candidate {
                title: "song".into(),
                artist: "Artist".into(),
            },
            Candidate {
                title: "song".into(),
                artist: "Artist".into(),
            },
            Candidate {
                title: "Song".into(),
                artist: "Missing".into(),
            },
        ];
        let matched = match_local(&conn, &candidates).unwrap();
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].id, 1);
        conn.execute("DELETE FROM tracks WHERE id=1", []).unwrap();
        assert!(match_local(&conn, &candidates).unwrap().is_empty());
        let recent = recent_candidates(
            &json!({"payload":{"listens":[{"track_metadata":{"artist_name":"Artist","track_name":"Song"}}]}}),
        );
        assert_eq!(recent[0].title, "Song");
    }

    #[test]
    fn thresholds_require_thirty_seconds_and_use_half_or_four_minutes() {
        assert_eq!(threshold(29_999), None);
        assert_eq!(threshold(30_000), Some(15_000));
        assert_eq!(threshold(180_000), Some(90_000));
        assert_eq!(threshold(480_000), Some(240_000));
        assert_eq!(threshold(900_000), Some(240_000));
    }

    #[test]
    fn fresh_and_existing_schema_have_a_durable_outbox() {
        let conn = crate::catalog::db::open_in_memory();
        conn.execute("INSERT INTO scrobble_outbox (artist,title,album,duration_ms,listened_at,created_at,username) VALUES ('Artist','Song','Album',180000,1700000000,1700000090,'owner')", []).unwrap();
        assert_eq!(
            conn.query_row("SELECT COUNT(*) FROM scrobble_outbox", [], |r| r
                .get::<_, i64>(0))
                .unwrap(),
            1
        );
    }
}
