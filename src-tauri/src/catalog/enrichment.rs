//! Explicit metadata enrichment. Network runs before the catalog operation lock.
use std::fs::{self, OpenOptions};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use base64::Engine;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::{Accessor, ItemKey, Tag};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::library::{AlbumSource, Track};
use super::Catalog;
use crate::platform::metadata_http;

pub const MAX_IMAGE: usize = 5 * 1024 * 1024;
const MAX_LRC: usize = 256 * 1024;
static UNIQUE: AtomicU64 = AtomicU64::new(0);
static NETWORK_OPERATION: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CandidateSource {
    #[default]
    MusicBrainz,
    #[serde(rename = "iTunes")]
    ITunes,
    #[serde(rename = "LRCLIB")]
    Lrclib,
    Manual,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagCandidate {
    #[serde(default)]
    pub source: CandidateSource,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub release_mbid: Option<String>,
    pub cover_preview_url: Option<String>,
    pub cover_preview: Option<String>,
    pub cover_url: Option<String>,
    pub genre: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LyricsCandidate {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub preview: String,
    pub synced_lyrics: String,
}
#[derive(Serialize, Deserialize)]
pub struct EnrichmentCandidates {
    pub title_query: String,
    pub artist_query: String,
    pub tags: Vec<TagCandidate>,
    pub lyrics: Vec<LyricsCandidate>,
    pub errors: Vec<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyEnrichmentRequest {
    pub track_id: i64,
    pub tags: Option<TagCandidate>,
    #[serde(default)]
    pub update_library_tags: bool,
    pub synced_lyrics: Option<String>,
    #[serde(default)]
    pub save_lyrics: bool,
    #[serde(default)]
    pub save_cover: bool,
    pub release_mbid: Option<String>,
    #[serde(default)]
    pub write_audio_tags: bool,
    #[serde(default)]
    pub overwrite_cover: bool,
    #[serde(default)]
    pub overwrite_lyrics: bool,
    #[serde(default)]
    pub dedicated_cover: bool,
    pub cover_url: Option<String>,
}

fn err(code: &str) -> String {
    code.to_string()
}
pub fn valid_mbid(id: &str) -> bool {
    id.len() == 36
        && id.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
}
pub fn filename_keywords(path: &Path) -> (String, String) {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let (stem, _) = super::metadata::split_track_number(stem);
    match stem.split_once(" - ") {
        Some((artist, title)) if !artist.trim().is_empty() && !title.trim().is_empty() => {
            (title.trim().into(), artist.trim().into())
        }
        _ => (stem, String::new()),
    }
}
fn query_terms(
    t: &Track,
    title: Option<String>,
    artist: Option<String>,
) -> Result<(String, String), String> {
    let path = Path::new(&t.path);
    let (file_title, file_artist) = filename_keywords(path);
    let missing_title = t.title.trim().is_empty()
        || t.title == "未知歌曲"
        || path.file_stem().and_then(|s| s.to_str()) == Some(t.title.as_str())
        || super::metadata::split_track_number(
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
        )
        .0 == t.title;
    let title = title.filter(|s| !s.trim().is_empty()).unwrap_or_else(|| {
        if missing_title {
            file_title
        } else {
            t.title.clone()
        }
    });
    let artist = artist.filter(|s| !s.trim().is_empty()).unwrap_or_else(|| {
        if t.artist.trim().is_empty() || t.artist == "未知艺术家" {
            file_artist
        } else {
            t.artist.clone()
        }
    });
    for s in [&title, &artist] {
        if s.len() > 512 || s.chars().any(char::is_control) {
            return Err(err("METADATA_QUERY_INVALID"));
        }
    }
    if title.trim().is_empty() {
        return Err(err("METADATA_QUERY_INVALID"));
    }
    Ok((title.trim().into(), artist.trim().into()))
}
fn artist_missing(value: &str) -> bool {
    value.trim().is_empty() || value.trim() == "未知艺术家"
}
fn split_pair(value: &str) -> Option<(String, String, bool)> {
    let separators: Vec<_> = value
        .char_indices()
        .filter(|(_, c)| matches!(c, '-' | '－' | '—'))
        .collect();
    let &(index, separator) = separators
        .iter()
        .find(|(i, c)| {
            value[..*i].ends_with(char::is_whitespace)
                && value[*i + c.len_utf8()..].starts_with(char::is_whitespace)
        })
        .or_else(|| separators.first())?;
    let left = value[..index].trim();
    let right = value[index + separator.len_utf8()..].trim();
    if left.is_empty() || right.is_empty() {
        return None;
    }
    let spaced = value[..index].ends_with(char::is_whitespace)
        && value[index + separator.len_utf8()..].starts_with(char::is_whitespace);
    Some((left.into(), right.into(), spaced))
}
fn query_variants(
    t: &Track,
    title: Option<String>,
    artist: Option<String>,
) -> Result<Vec<(String, String)>, String> {
    let missing = artist
        .as_deref()
        .map(artist_missing)
        .unwrap_or_else(|| artist_missing(&t.artist));
    if !missing {
        return query_terms(t, title, artist).map(|terms| vec![terms]);
    }
    let raw = title
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&t.title);
    let stem = Path::new(&t.path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let filename_title = super::metadata::split_track_number(stem).0;
    let raw = if raw.trim().is_empty() || raw == "未知歌曲" || raw == stem {
        filename_title.as_str()
    } else {
        raw
    };
    if raw.len() > 512 || raw.chars().any(char::is_control) {
        return Err(err("METADATA_QUERY_INVALID"));
    }
    let from_filename = || {
        let stem = Path::new(&t.path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        split_pair(&super::metadata::split_track_number(stem).0)
    };
    let pair = split_pair(raw).or_else(|| {
        if title
            .as_deref()
            .is_none_or(|value| value.trim().is_empty() || value == t.title)
        {
            from_filename()
        } else {
            None
        }
    });
    if let Some((left, right, spaced)) = pair {
        let first = if spaced {
            (right.clone(), left.clone())
        } else {
            (left.clone(), right.clone())
        };
        let second = if spaced { (left, right) } else { (right, left) };
        return Ok(if first == second {
            vec![first]
        } else {
            vec![first, second]
        });
    }
    let (title, _) = query_terms(t, title, Some(String::new()))?;
    Ok(vec![(title, String::new())])
}

#[derive(Debug, Serialize)]
pub struct MetadataStatus {
    pub track_id: i64,
    pub enrichment_ignored: bool,
    pub missing_artist: bool,
    pub missing_album: bool,
    pub missing_cover: Option<bool>,
    pub missing_lyrics: Option<bool>,
}
pub fn cover_paths(path: &Path) -> [PathBuf; 2] {
    [
        path.with_extension("cover.jpg"),
        path.with_file_name("cover.jpg"),
    ]
}
fn local_file_missing(path: &Path) -> Option<bool> {
    match fs::metadata(path) {
        Ok(meta) => Some(!meta.is_file()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Some(true),
        Err(_) => None,
    }
}
pub fn track_statuses(catalog: &Catalog, ids: &[i64]) -> Result<Vec<MetadataStatus>, String> {
    if ids.len() > 200 || ids.iter().any(|id| *id <= 0) {
        return Err(err("METADATA_IDS_INVALID"));
    }
    let tracks = catalog
        .tracks_by_ids(ids)
        .map_err(|_| err("METADATA_DATABASE"))?;
    let conn = catalog.connection().map_err(|_| err("METADATA_DATABASE"))?;
    let mut stmt = conn.prepare("SELECT track_id FROM enrichment_cache WHERE ignored<>0 AND track_id IN (SELECT value FROM json_each(?1))").map_err(|_| err("METADATA_DATABASE"))?;
    let ignored = stmt
        .query_map(
            [serde_json::to_string(ids).map_err(|_| err("METADATA_IDS_INVALID"))?],
            |r| r.get::<_, i64>(0),
        )
        .map_err(|_| err("METADATA_DATABASE"))?
        .collect::<Result<std::collections::HashSet<_>, _>>()
        .map_err(|_| err("METADATA_DATABASE"))?;
    Ok(tracks
        .into_iter()
        .map(|t| {
            let path = Path::new(&t.path);
            let missing_cover = crate::artwork::covers::has_usable_cover(path)
                .ok()
                .map(|has_cover| !has_cover);
            MetadataStatus {
                track_id: t.id,
                enrichment_ignored: ignored.contains(&t.id),
                missing_artist: artist_missing(&t.artist),
                missing_album: t.album.trim().is_empty()
                    || t.album == "未知专辑"
                    || t.album_source != AlbumSource::Tag,
                missing_cover,
                missing_lyrics: local_file_missing(&path.with_extension("lrc")),
            }
        })
        .collect())
}
/// A catalog-only preference. No HTTP or audio/sidecar writes occur here.
pub fn set_ignored(catalog: &Catalog, track_id: i64, ignored: bool) -> Result<(), String> {
    if track_id <= 0 {
        return Err(err("METADATA_IDS_INVALID"));
    }
    let _guard = catalog.operation.lock().map_err(|_| err("METADATA_BUSY"))?;
    catalog
        .track(track_id)
        .map_err(|_| err("METADATA_TRACK_MISSING"))?;
    catalog
        .connection()
        .map_err(|_| err("METADATA_DATABASE"))?
        .execute(
            "INSERT INTO enrichment_cache(track_id,query_key,payload_json,fetched_at,ignored)
         VALUES(?1,'','',0,?2) ON CONFLICT(track_id) DO UPDATE SET ignored=excluded.ignored",
            rusqlite::params![track_id, ignored],
        )
        .map_err(|_| err("METADATA_DATABASE"))?;
    Ok(())
}
fn text(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or_default().trim().to_string()
}
fn credit(v: &Value) -> String {
    v.as_array()
        .map(|a| {
            a.iter()
                .map(|v| {
                    let name = v["name"]
                        .as_str()
                        .or_else(|| v["artist"]["name"].as_str())
                        .unwrap_or_default();
                    format!("{name}{}", v["joinphrase"].as_str().unwrap_or_default())
                })
                .collect::<String>()
                .trim()
                .to_string()
        })
        .unwrap_or_default()
}
pub fn parse_recordings(v: &Value) -> Result<Vec<TagCandidate>, String> {
    let recordings = v["recordings"]
        .as_array()
        .ok_or_else(|| err("METADATA_MB_RESPONSE"))?;
    Ok(recordings
        .iter()
        .filter_map(|r| {
            let title = text(r, "title");
            if title.is_empty() {
                return None;
            }
            let release = &r["releases"][0];
            let id = text(release, "id");
            let release_mbid = valid_mbid(&id).then_some(id);
            let artist = credit(&r["artist-credit"]);
            let album_artist = credit(&release["artist-credit"]);
            let date = release["date"]
                .as_str()
                .or_else(|| r["first-release-date"].as_str())
                .unwrap_or_default();
            let track_number = release["media"][0]["track"][0]["number"]
                .as_str()
                .and_then(|n| n.parse().ok())
                .or_else(|| {
                    release["media"][0]["track-offset"]
                        .as_u64()
                        .and_then(|n| n.checked_add(1))
                        .and_then(|n| u32::try_from(n).ok())
                });
            Some(TagCandidate {
                source: CandidateSource::MusicBrainz,
                title,
                artist,
                album: text(release, "title"),
                album_artist: if album_artist.is_empty() {
                    credit(&r["artist-credit"])
                } else {
                    album_artist
                },
                year: date.get(..4).and_then(|y| y.parse().ok()),
                track_number,
                cover_preview_url: release_mbid
                    .as_ref()
                    .map(|id| format!("https://coverartarchive.org/release/{id}/front-250")),
                release_mbid,
                cover_preview: None,
                cover_url: None,
                genre: None,
            })
        })
        .take(5)
        .collect())
}
fn valid_lrc(s: &str) -> bool {
    !s.is_empty() && s.len() <= MAX_LRC && !s.contains('\0') && crate::lyrics::parse_lrc(s).synced
}
pub fn parse_lyrics(v: &Value) -> Result<Vec<LyricsCandidate>, String> {
    let entries = v.as_array().ok_or_else(|| err("METADATA_LRC_RESPONSE"))?;
    Ok(entries
        .iter()
        .filter_map(|v| {
            let lrc = v["syncedLyrics"].as_str()?;
            if !valid_lrc(lrc) {
                return None;
            }
            Some(LyricsCandidate {
                id: v["id"].as_i64().unwrap_or_default(),
                title: text(v, "trackName"),
                artist: text(v, "artistName"),
                album: text(v, "albumName"),
                preview: lrc.lines().take(5).collect::<Vec<_>>().join("\n"),
                synced_lyrics: lrc.into(),
            })
        })
        .take(3)
        .collect())
}
pub fn itunes_artwork(raw: &str) -> Option<String> {
    let parsed = ureq::get(raw).request_url().ok()?;
    let mut url = parsed.as_url().clone();
    if url.scheme() == "http" {
        url.set_scheme("https").ok()?;
    }
    let (prefix, name) = url.path().rsplit_once('/')?;
    if !name.starts_with("100x100") {
        return None;
    }
    let path = format!("{prefix}/600x600bb.jpg");
    url.set_path(&path);
    metadata_http::validate_itunes_cover_url(url.as_str()).ok()?;
    Some(url.to_string())
}
pub fn parse_itunes(v: &Value) -> Result<Vec<TagCandidate>, String> {
    let results = v["results"]
        .as_array()
        .ok_or_else(|| err("METADATA_ITUNES_RESPONSE"))?;
    Ok(results
        .iter()
        .filter_map(|v| {
            if v["kind"].as_str().is_some_and(|kind| kind != "song") {
                return None;
            }
            let title = text(v, "trackName");
            if title.is_empty() {
                return None;
            }
            let artist = text(v, "artistName");
            let album_artist = text(v, "collectionArtistName");
            let cover_url = v["artworkUrl100"].as_str().and_then(itunes_artwork);
            Some(TagCandidate {
                source: CandidateSource::ITunes,
                title,
                artist: artist.clone(),
                album: text(v, "collectionName"),
                album_artist: if album_artist.is_empty() {
                    artist
                } else {
                    album_artist
                },
                year: v["releaseDate"]
                    .as_str()
                    .and_then(|s| s.get(..4))
                    .and_then(|s| s.parse().ok()),
                track_number: v["trackNumber"]
                    .as_u64()
                    .and_then(|n| u32::try_from(n).ok())
                    .filter(|n| *n > 0),
                release_mbid: None,
                cover_preview_url: cover_url.clone(),
                cover_preview: None,
                cover_url,
                genre: v["primaryGenreName"]
                    .as_str()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string),
            })
        })
        .take(5)
        .collect())
}
fn merge_sources(
    mb: Result<VariantResults<TagCandidate>, String>,
    itunes: Result<VariantResults<TagCandidate>, String>,
    lrc: Result<VariantResults<LyricsCandidate>, String>,
    fallback: (String, String),
) -> EnrichmentCandidates {
    let tag_networks_failed = mb
        .as_ref()
        .err()
        .is_some_and(|e| e == "METADATA_MB_NETWORK")
        && itunes
            .as_ref()
            .err()
            .is_some_and(|e| e == "METADATA_ITUNES_NETWORK");
    let mut tag_errors = Vec::new();
    let mut tags = Vec::new();
    let mut query = None;
    let mut seen = std::collections::HashSet::new();
    // Prefer the direct Apple artwork on otherwise identical releases, while retaining MB-only versions.
    for result in [itunes, mb] {
        match result {
            Ok(result) => {
                if !result.values.is_empty() && query.is_none() {
                    query = result.query;
                }
                for tag in result.values {
                    let key = (
                        super::library::normalize_key(&tag.title),
                        super::library::normalize_key(&tag.artist),
                        super::library::normalize_key(&tag.album),
                    );
                    if seen.insert(key) && tags.len() < 10 {
                        tags.push(tag);
                    }
                }
            }
            Err(e) => tag_errors.push(e),
        }
    }
    let lyrics = match lrc {
        Ok(result) => {
            if query.is_none() {
                query = result.query;
            }
            result.values
        }
        Err(_) => Vec::new(),
    };
    if tags.is_empty() {
        if let Some(lyric) = lyrics
            .iter()
            .find(|l| !l.title.is_empty() && !l.artist.is_empty())
        {
            tags.push(TagCandidate {
                source: CandidateSource::Lrclib,
                title: lyric.title.clone(),
                artist: lyric.artist.clone(),
                album: lyric.album.clone(),
                album_artist: lyric.artist.clone(),
                year: None,
                track_number: None,
                release_mbid: None,
                cover_preview_url: None,
                cover_preview: None,
                cover_url: None,
                genre: None,
            });
        }
    }
    let errors = if tags.is_empty() && tag_networks_failed {
        tag_errors.reverse();
        tag_errors
    } else {
        Vec::new()
    };
    let (title_query, artist_query) = query.unwrap_or(fallback);
    EnrichmentCandidates {
        title_query,
        artist_query,
        tags,
        lyrics,
        errors,
    }
}
pub fn search(
    catalog: &Catalog,
    track_id: i64,
    title: Option<String>,
    artist: Option<String>,
    force_refresh: bool,
) -> Result<EnrichmentCandidates, String> {
    search_cached(
        catalog,
        track_id,
        title,
        artist,
        force_refresh,
        search_uncached,
    )
}
const CACHE_TTL: i64 = 7 * 24 * 60 * 60;
const CACHE_MAX_BYTES: usize = 16 * 1024 * 1024;
fn query_key(variants: &[(String, String)]) -> Result<String, String> {
    let (title, artist) = variants
        .first()
        .ok_or_else(|| err("METADATA_QUERY_INVALID"))?;
    serde_json::to_string(&(
        super::library::normalize_key(title),
        super::library::normalize_key(artist),
    ))
    .map_err(|_| err("METADATA_QUERY_INVALID"))
}
fn search_cached(
    catalog: &Catalog,
    track_id: i64,
    title: Option<String>,
    artist: Option<String>,
    force: bool,
    fetch: impl FnOnce(&[(String, String)]) -> Result<EnrichmentCandidates, String>,
) -> Result<EnrichmentCandidates, String> {
    let track = catalog
        .track(track_id)
        .map_err(|_| err("METADATA_TRACK_MISSING"))?;
    let variants = query_variants(&track, title.clone(), artist.clone())?;
    let key = query_key(&variants)?;
    if !force {
        let conn = catalog
            .connection()
            .map_err(|_| err("METADATA_CACHE_READ"))?;
        let now = super::db::now_secs();
        let cached = conn.query_row(
            "SELECT payload_json FROM enrichment_cache
             WHERE track_id=?1 AND query_key=?2 AND fetched_at<=?3 AND fetched_at>?4
             AND length(CAST(payload_json AS BLOB))<=?5",
            rusqlite::params![
                track_id,
                key,
                now,
                now.saturating_sub(CACHE_TTL),
                CACHE_MAX_BYTES as i64
            ],
            |row| row.get::<_, String>(0),
        );
        match cached {
            Ok(payload) => {
                if let Ok(result) = serde_json::from_str::<EnrichmentCandidates>(&payload) {
                    return Ok(result);
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {}
            Err(_) => return Err(err("METADATA_CACHE_READ")),
        }
    }
    // No catalog or SQLite guard survives this call into the blocking network worker.
    let result = fetch(&variants)?;
    let _guard = catalog.operation.lock().map_err(|_| err("METADATA_BUSY"))?;
    let current = catalog
        .track(track_id)
        .map_err(|_| err("METADATA_TRACK_MISSING"))?;
    if current.path != track.path || query_key(&query_variants(&current, title, artist)?)? != key {
        return Err(err("METADATA_QUERY_CHANGED"));
    }
    // Transport failures must not poison an otherwise useful seven-day cache.
    if result.errors.is_empty() {
        let payload = serde_json::to_string(&result).map_err(|_| err("METADATA_CACHE_WRITE"))?;
        if payload.len() <= CACHE_MAX_BYTES {
            catalog.connection().map_err(|_| err("METADATA_CACHE_WRITE"))?.execute(
                "INSERT INTO enrichment_cache(track_id,query_key,payload_json,fetched_at)
                 VALUES(?1,?2,?3,?4) ON CONFLICT(track_id) DO UPDATE SET
                 query_key=excluded.query_key,payload_json=excluded.payload_json,fetched_at=excluded.fetched_at",
                rusqlite::params![track_id, key, payload, super::db::now_secs()],
            ).map_err(|_| err("METADATA_CACHE_WRITE"))?;
        }
    }
    Ok(result)
}
fn search_uncached(variants: &[(String, String)]) -> Result<EnrichmentCandidates, String> {
    let _network = NETWORK_OPERATION
        .try_lock()
        .map_err(|_| err("METADATA_BUSY"))?;
    let client = metadata_http::Client::new();
    // Independent providers: one failure must not discard the other's candidates.
    let (recordings, itunes, lyrics) = std::thread::scope(|scope| {
        let recordings = scope.spawn(|| {
            search_variants(variants, |title, artist| {
                client
                    .recordings(title, artist)
                    .and_then(|v| parse_recordings(&v))
            })
        });
        let itunes = scope.spawn(|| {
            search_variants(variants, |title, artist| {
                client.itunes(title, artist).and_then(|v| parse_itunes(&v))
            })
        });
        let lyrics = scope.spawn(|| {
            search_variants(variants, |title, artist| {
                client.lyrics(title, artist).and_then(|v| parse_lyrics(&v))
            })
        });
        (
            recordings
                .join()
                .unwrap_or_else(|_| Err(err("METADATA_MB_RESPONSE"))),
            itunes
                .join()
                .unwrap_or_else(|_| Err(err("METADATA_ITUNES_RESPONSE"))),
            lyrics
                .join()
                .unwrap_or_else(|_| Err(err("METADATA_LRC_RESPONSE"))),
        )
    });
    let mut result = merge_sources(recordings, itunes, lyrics, variants[0].clone());
    let mut previews: std::collections::HashMap<String, Option<String>> =
        std::collections::HashMap::new();
    for tag in &mut result.tags {
        let key = tag.cover_url.clone().or_else(|| tag.release_mbid.clone());
        if let Some(key) = key {
            if let Some(preview) = previews.get(&key) {
                tag.cover_preview = preview.clone();
                continue;
            }
            let bytes = if let Some(url) = &tag.cover_url {
                client.apple_cover(url)
            } else {
                client.cover(&key, true)
            };
            if let Ok(bytes) = bytes {
                if let Ok(image) = validate_image(&bytes) {
                    let thumb = image.thumbnail(250, 250).to_rgb8();
                    let mut output = Vec::new();
                    if image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 80)
                        .encode_image(&thumb)
                        .is_ok()
                    {
                        tag.cover_preview = Some(format!(
                            "data:image/jpeg;base64,{}",
                            base64::engine::general_purpose::STANDARD.encode(output)
                        ));
                    }
                }
            }
            previews.insert(key, tag.cover_preview.clone());
        }
    }
    Ok(result)
}
pub fn strip_title_suffix(title: &str) -> String {
    let mut value = title.trim();
    loop {
        if !value.ends_with([')', '）', ']']) {
            break;
        }
        let mut stack = Vec::new();
        let mut start = None;
        for (index, ch) in value.char_indices().rev() {
            match ch {
                ')' => stack.push('('),
                '）' => stack.push('（'),
                ']' => stack.push('['),
                '(' | '（' | '[' => {
                    if stack.pop() != Some(ch) {
                        break;
                    }
                    if stack.is_empty() {
                        start = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(index) = start else { break };
        let prefix = value[..index].trim_end();
        if prefix.is_empty() {
            break;
        }
        value = prefix;
    }
    value.into()
}
struct VariantResults<T> {
    values: Vec<T>,
    query: Option<(String, String)>,
}
fn search_variants<T>(
    variants: &[(String, String)],
    mut search: impl FnMut(&str, &str) -> Result<Vec<T>, String>,
) -> Result<VariantResults<T>, String> {
    let mut attempts = variants.to_vec();
    for (title, artist) in variants {
        let stripped = strip_title_suffix(title);
        let pair = (stripped, artist.clone());
        if !attempts.contains(&pair) {
            attempts.push(pair);
        }
    }
    for (title, artist) in &attempts {
        let results = search(title, artist)?;
        if !results.is_empty() {
            return Ok(VariantResults {
                values: results,
                query: Some((title.clone(), artist.clone())),
            });
        }
    }
    Ok(VariantResults {
        values: Vec::new(),
        query: None,
    })
}
pub fn validate_image(bytes: &[u8]) -> Result<image::DynamicImage, String> {
    if bytes.is_empty() || bytes.len() > MAX_IMAGE {
        return Err(err("METADATA_IMAGE_SIZE"));
    }
    let format = image::guess_format(bytes).map_err(|_| err("METADATA_IMAGE_INVALID"))?;
    if !matches!(format, image::ImageFormat::Jpeg | image::ImageFormat::Png) {
        return Err(err("METADATA_IMAGE_INVALID"));
    }
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(8192);
    limits.max_image_height = Some(8192);
    limits.max_alloc = Some(80 * 1024 * 1024);
    let mut reader = image::ImageReader::with_format(Cursor::new(bytes), format);
    reader.limits(limits);
    reader.decode().map_err(|_| err("METADATA_IMAGE_INVALID"))
}
fn validate_request(p: &ApplyEnrichmentRequest) -> Result<(), String> {
    if p.track_id <= 0 || !(p.update_library_tags || p.save_cover || p.save_lyrics) {
        return Err(err("METADATA_SELECTION_INVALID"));
    }
    if p.update_library_tags || (p.write_audio_tags && p.tags.is_some()) {
        let t = p
            .tags
            .as_ref()
            .ok_or_else(|| err("METADATA_TAGS_MISSING"))?;
        if t.title.trim().is_empty()
            || [&t.title, &t.artist, &t.album, &t.album_artist]
                .iter()
                .any(|s| s.len() > 1024 || s.chars().any(char::is_control))
            || t.genre
                .as_ref()
                .is_some_and(|s| s.len() > 1024 || s.chars().any(char::is_control))
            || t.year.is_some_and(|y| y > 9999)
            || t.track_number.is_some_and(|n| n == 0 || n > 9999)
        {
            return Err(err("METADATA_TAGS_INVALID"));
        }
    }
    if p.save_lyrics && !p.synced_lyrics.as_ref().is_some_and(|s| valid_lrc(s)) {
        return Err(err("METADATA_LRC_INVALID"));
    }
    if p.save_cover {
        if let Some(url) = &p.cover_url {
            metadata_http::validate_itunes_cover_url(url)?;
        } else if !p.release_mbid.as_ref().is_some_and(|id| valid_mbid(id)) {
            return Err(err("METADATA_COVER_INVALID"));
        }
    }
    Ok(())
}
pub fn apply(catalog: &Catalog, payload: ApplyEnrichmentRequest) -> Result<Track, String> {
    validate_request(&payload)?;
    catalog
        .track(payload.track_id)
        .map_err(|_| err("METADATA_TRACK_MISSING"))?;
    let cover = if payload.save_cover {
        let _network = NETWORK_OPERATION
            .try_lock()
            .map_err(|_| err("METADATA_BUSY"))?;
        let client = metadata_http::Client::new();
        Some(if let Some(url) = &payload.cover_url {
            client.apple_cover(url)?
        } else {
            client.cover(
                payload
                    .release_mbid
                    .as_deref()
                    .ok_or_else(|| err("METADATA_COVER_INVALID"))?,
                false,
            )?
        })
    } else {
        None
    };
    apply_prepared(catalog, payload, cover)
}

/// Each output is prepared beside the destination; never write tags into the original file.
struct StagedFile {
    target: PathBuf,
    temp: PathBuf,
    backup: Option<PathBuf>,
    published: bool,
    preserve_backup: bool,
}
fn temp_file(target: &Path) -> Result<(PathBuf, std::fs::File), String> {
    let parent = target
        .parent()
        .ok_or_else(|| err("METADATA_FILE_INVALID"))?;
    for _ in 0..32 {
        let name = format!(
            ".musicmine-{}-{}-{}.tmp",
            std::process::id(),
            super::db::now_secs(),
            UNIQUE.fetch_add(1, Ordering::Relaxed)
        );
        let path = parent.join(name);
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => return Ok((path, file)),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err(err("METADATA_FILE_UNWRITABLE")),
        }
    }
    Err(err("METADATA_FILE_UNWRITABLE"))
}
fn regular_file(path: &Path) -> Result<bool, String> {
    match fs::symlink_metadata(path) {
        Ok(m) if m.is_file() && !m.file_type().is_symlink() => {
            if m.permissions().readonly() {
                return Err(err("METADATA_FILE_UNWRITABLE"));
            }
            Ok(true)
        }
        Ok(_) => Err(err("METADATA_FILE_INVALID")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(err("METADATA_FILE_UNWRITABLE")),
    }
}
impl StagedFile {
    fn new(target: PathBuf, overwrite: bool) -> Result<Self, String> {
        let exists = regular_file(&target)?;
        if exists && !overwrite {
            return Err(err("METADATA_FILE_EXISTS"));
        }
        let (temp, file) = temp_file(&target)?;
        drop(file);
        let mut staged = Self {
            target,
            temp,
            backup: None,
            published: false,
            preserve_backup: false,
        };
        if exists {
            let (backup, file) = temp_file(&staged.target)?;
            drop(file);
            staged.backup = Some(backup.clone());
            fs::copy(&staged.target, backup).map_err(|_| err("METADATA_FILE_UNWRITABLE"))?;
        }
        Ok(staged)
    }
    fn write(&self, bytes: &[u8]) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.temp)
            .map_err(|_| err("METADATA_FILE_UNWRITABLE"))?;
        file.write_all(bytes)
            .and_then(|_| file.sync_all())
            .map_err(|_| err("METADATA_FILE_UNWRITABLE"))
    }
    fn publish(&mut self) -> Result<(), String> {
        if self.backup.is_some() {
            regular_file(&self.target)?;
            fs::rename(&self.temp, &self.target).map_err(|_| err("METADATA_FILE_REPLACE"))?;
        } else {
            crate::platform::atomic_files::publish_new(&self.temp, &self.target).map_err(|e| {
                err(if e.kind() == std::io::ErrorKind::AlreadyExists {
                    "METADATA_FILE_EXISTS"
                } else {
                    "METADATA_FILE_REPLACE"
                })
            })?;
        }
        self.published = true;
        Ok(())
    }
    fn rollback(&mut self) -> Result<(), String> {
        if self.published {
            let result = match &self.backup {
                Some(backup) => fs::rename(backup, &self.target),
                None => fs::remove_file(&self.target),
            };
            if result.is_err() {
                self.preserve_backup = true;
                return Err(err("METADATA_ROLLBACK_FAILED"));
            }
            self.published = false;
        }
        Ok(())
    }
}
impl Drop for StagedFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.temp);
        if !self.preserve_backup {
            if let Some(path) = &self.backup {
                let _ = fs::remove_file(path);
            }
        }
    }
}
fn apply_prepared(
    catalog: &Catalog,
    p: ApplyEnrichmentRequest,
    cover: Option<Vec<u8>>,
) -> Result<Track, String> {
    validate_request(&p)?;
    let _guard = catalog.operation.lock().map_err(|_| err("METADATA_BUSY"))?;
    let mut track = catalog
        .track(p.track_id)
        .map_err(|_| err("METADATA_TRACK_MISSING"))?;
    let path = PathBuf::from(&track.path);
    if p.save_lyrics || p.save_cover || p.write_audio_tags {
        let source = fs::symlink_metadata(&path).map_err(|_| err("METADATA_FILE_INVALID"))?;
        if !source.is_file() || source.file_type().is_symlink() {
            return Err(err("METADATA_FILE_INVALID"));
        }
    }
    let image = if p.save_cover {
        Some(validate_image(
            cover
                .as_deref()
                .ok_or_else(|| err("METADATA_COVER_INVALID"))?,
        )?)
    } else {
        None
    };
    if p.update_library_tags {
        let t = p
            .tags
            .as_ref()
            .ok_or_else(|| err("METADATA_TAGS_MISSING"))?;
        track.title = t.title.trim().into();
        track.artist = t.artist.trim().into();
        track.album = t.album.trim().into();
        track.album_artist = t.album_artist.trim().into();
        track.year = t.year;
        track.track_number = t.track_number;
        if let Some(genre) = &t.genre {
            track.genre = genre.trim().into();
        }
        track.album_source = AlbumSource::Tag;
    }
    let mut staged = Vec::new();
    if p.save_lyrics {
        let file = StagedFile::new(path.with_extension("lrc"), p.overwrite_lyrics)?;
        file.write(
            p.synced_lyrics
                .as_deref()
                .ok_or_else(|| err("METADATA_LRC_INVALID"))?
                .as_bytes(),
        )?;
        staged.push(file);
    }
    let mut jpeg = Vec::new();
    if let Some(image) = image {
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, 95)
            .encode_image(&image.to_rgb8())
            .map_err(|_| err("METADATA_IMAGE_INVALID"))?;
        if jpeg.len() > MAX_IMAGE {
            return Err(err("METADATA_IMAGE_SIZE"));
        }
        let target = if p.dedicated_cover {
            path.with_extension("cover.jpg")
        } else {
            path.with_file_name("cover.jpg")
        };
        let file = StagedFile::new(target, p.overwrite_cover)?;
        file.write(&jpeg)?;
        staged.push(file);
    }
    if p.write_audio_tags && (p.update_library_tags || p.save_cover) {
        if !regular_file(&path)? || !super::library::is_supported(&path) {
            return Err(err("METADATA_AUDIO_UNSUPPORTED"));
        }
        // Opening writable is a preflight only; no bytes are changed here.
        drop(
            OpenOptions::new()
                .write(true)
                .open(&path)
                .map_err(|_| err("METADATA_FILE_UNWRITABLE"))?,
        );
        let mut tagged =
            lofty::read_from_path(&path).map_err(|_| err("METADATA_AUDIO_UNSUPPORTED"))?;
        let tag_type = tagged.primary_tag_type();
        if !tagged.file_type().supports_tag_type(tag_type) {
            return Err(err("METADATA_AUDIO_UNSUPPORTED"));
        }
        if tagged.primary_tag().is_none() {
            tagged.insert_tag(Tag::new(tag_type));
        }
        let tag = tagged
            .primary_tag_mut()
            .ok_or_else(|| err("METADATA_AUDIO_UNSUPPORTED"))?;
        if let Some(t) = &p.tags {
            // Text selection controls audio text too; cover-only applies do not rewrite text.
            if p.update_library_tags {
                tag.set_title(t.title.clone());
                tag.set_artist(t.artist.clone());
                tag.set_album(t.album.clone());
                if let Some(genre) = &t.genre {
                    tag.set_genre(genre.clone());
                }
                tag.insert_text(ItemKey::AlbumArtist, t.album_artist.clone());
                if let Some(y) = t.year {
                    tag.set_year(y);
                } else {
                    tag.remove_year();
                }
                if let Some(n) = t.track_number {
                    tag.set_track(n);
                } else {
                    tag.remove_track();
                }
            }
        }
        if p.save_cover {
            let mut picture = lofty::picture::Picture::from_reader(&mut Cursor::new(&jpeg))
                .map_err(|_| err("METADATA_IMAGE_INVALID"))?;
            picture.set_pic_type(lofty::picture::PictureType::CoverFront);
            tag.remove_picture_type(lofty::picture::PictureType::CoverFront);
            tag.push_picture(picture);
        }
        let file = StagedFile::new(path.clone(), true)?;
        fs::copy(&path, &file.temp).map_err(|_| err("METADATA_FILE_UNWRITABLE"))?;
        tagged
            .save_to_path(&file.temp, lofty::config::WriteOptions::default())
            .map_err(|_| err("METADATA_AUDIO_WRITE"))?;
        lofty::probe::Probe::open(&file.temp)
            .and_then(|p| Ok(p.guess_file_type()?))
            .and_then(|p| p.read())
            .map_err(|_| err("METADATA_AUDIO_WRITE"))?;
        OpenOptions::new()
            .write(true)
            .open(&file.temp)
            .and_then(|f| f.sync_all())
            .map_err(|_| err("METADATA_AUDIO_WRITE"))?;
        staged.push(file);
    }
    let mut conn = catalog.connection().map_err(|_| err("METADATA_DATABASE"))?;
    let tx = conn.transaction().map_err(|_| err("METADATA_DATABASE"))?;
    let result = (|| {
        for file in &mut staged {
            file.publish()?;
        }
        if p.update_library_tags {
            let changed = tx.execute("UPDATE tracks SET title=?1,artist=?2,album=?3,album_artist=?4,year=?5,track_number=?6,genre=?7,album_source='tag' WHERE id=?8", rusqlite::params![track.title, track.artist, track.album, track.album_artist, track.year, track.track_number, track.genre, track.id]).map_err(|_| err("METADATA_DATABASE"))?;
            if changed != 1 {
                return Err(err("METADATA_TRACK_MISSING"));
            }
        }
        tx.execute(
            "DELETE FROM enrichment_cache WHERE track_id=?1 AND ignored=0",
            [track.id],
        )
        .map_err(|_| err("METADATA_CACHE_WRITE"))?;
        tx.execute("UPDATE enrichment_cache SET query_key='',payload_json='',fetched_at=0 WHERE track_id=?1 AND ignored<>0", [track.id])
            .map_err(|_| err("METADATA_CACHE_WRITE"))?;
        tx.commit().map_err(|_| err("METADATA_DATABASE"))?;
        Ok(track)
    })();
    if result.is_err() {
        let mut rollback_failed = false;
        for file in staged.iter_mut().rev() {
            if file.rollback().is_err() {
                rollback_failed = true;
            }
        }
        if rollback_failed {
            return Err(err("METADATA_ROLLBACK_FAILED"));
        }
    }
    result
}

#[cfg(test)]
#[path = "enrichment_tests.rs"]
mod tests;
