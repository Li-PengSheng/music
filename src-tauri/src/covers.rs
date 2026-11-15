//! Bounded cover thumbnails. Decoding happens on the caller's blocking thread,
//! never on the audio callback. Disk files live under the app cache directory.
use std::collections::{HashMap, VecDeque};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Instant, UNIX_EPOCH};

use base64::Engine;

const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;
const MAX_EDGE: u32 = 960;
const MIN_EDGE: u32 = 32;
const MAX_PIXELS: u32 = 8_192;
/// A 4000 x 4000 RGBA picture needs 64 MB; two decode slots bound the peak.
const MAX_DECODE_ALLOC: u64 = 80 * 1024 * 1024;
/// WebP and GIF are not decoded here; small ones go to the WebView unchanged.
const MAX_PASSTHROUGH_BYTES: usize = 512 * 1024;
const MEMORY_ENTRIES: usize = 240;
const MEMORY_BYTES: usize = 24 * 1024 * 1024;
const DISK_BYTES: u64 = 64 * 1024 * 1024;
const DECODE_SLOTS: u32 = 2;
/// Disk entries written by an older policy (which stored read failures as "no cover") are dropped.
const CACHE_VERSION: &str = "v2";

#[derive(Default)]
pub struct Stats {
    pub requests: AtomicU64,
    pub decoded: AtomicU64,
    pub passthrough: AtomicU64,
    pub memory_hits: AtomicU64,
    pub disk_hits: AtomicU64,
    /// Definitive "no usable cover" results, split by reason below.
    pub missing: AtomicU64,
    pub no_picture: AtomicU64,
    pub unsupported: AtomicU64,
    pub too_large: AtomicU64,
    pub corrupt: AtomicU64,
    /// File missing or unreadable right now. Never cached.
    pub transient: AtomicU64,
    pub coalesced: AtomicU64,
    pub bytes_out: AtomicU64,
    pub decode_us: AtomicU64,
    pub decode_max_us: AtomicU64,
    pub slot_wait_us: AtomicU64,
}

impl Stats {
    pub fn snapshot(&self) -> serde_json::Value {
        let n = |v: &AtomicU64| v.load(Ordering::Relaxed);
        serde_json::json!({
            "requests": n(&self.requests),
            "decoded": n(&self.decoded),
            "passthrough": n(&self.passthrough),
            "memoryHits": n(&self.memory_hits),
            "diskHits": n(&self.disk_hits),
            "coalesced": n(&self.coalesced),
            "noPicture": n(&self.no_picture),
            "unsupported": n(&self.unsupported),
            "tooLarge": n(&self.too_large),
            "corrupt": n(&self.corrupt),
            "transient": n(&self.transient),
            "bytesOut": n(&self.bytes_out),
            "decodeMs": n(&self.decode_us) / 1000,
            "decodeMaxMs": n(&self.decode_max_us) / 1000,
            "slotWaitMs": n(&self.slot_wait_us) / 1000,
        })
    }
}

/// The cover cannot be produced now but may be later. Returned to the page as a code, never cached.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverError {
    FileMissing,
    Unreadable,
}

impl CoverError {
    pub fn code(self) -> &'static str {
        match self {
            CoverError::FileMissing => "cover-file-missing",
            CoverError::Unreadable => "cover-unreadable",
        }
    }
}

/// Definitive reasons a file has no usable cover. These are cached until the file changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Absent {
    NoPicture,
    Unsupported,
    TooLarge,
    Corrupt,
}

struct Image {
    mime: &'static str,
    bytes: Vec<u8>,
}

type Cached = Option<Arc<Image>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Key {
    /// 1 = library row id, 2 = playback path that is not a library row.
    kind: u8,
    id: u64,
    mtime_ns: u128,
    len: u64,
    edge: u32,
}

struct Slots {
    free: Mutex<u32>,
    cv: Condvar,
}

struct SlotGuard<'a> {
    slots: &'a Slots,
}

impl Drop for SlotGuard<'_> {
    fn drop(&mut self) {
        let mut free = self.slots.free.lock().unwrap_or_else(|e| e.into_inner());
        *free += 1;
        self.slots.cv.notify_one();
    }
}

impl Slots {
    fn new(n: u32) -> Self {
        Self {
            free: Mutex::new(n),
            cv: Condvar::new(),
        }
    }
    fn enter(&self) -> SlotGuard<'_> {
        let mut free = self.free.lock().unwrap_or_else(|e| e.into_inner());
        while *free == 0 {
            free = self.cv.wait(free).unwrap_or_else(|e| e.into_inner());
        }
        *free -= 1;
        SlotGuard { slots: self }
    }
}

enum FlightState {
    Pending,
    Ready(Result<Cached, CoverError>),
}

struct Flight {
    state: Mutex<FlightState>,
    cv: Condvar,
}

impl Flight {
    fn pending() -> Self {
        Self {
            state: Mutex::new(FlightState::Pending),
            cv: Condvar::new(),
        }
    }
    fn wait(&self) -> Result<Cached, CoverError> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            match &*state {
                FlightState::Ready(value) => return value.clone(),
                FlightState::Pending => {
                    state = self.cv.wait(state).unwrap_or_else(|e| e.into_inner());
                }
            }
        }
    }
    fn finish(&self, value: Result<Cached, CoverError>) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        *state = FlightState::Ready(value);
        self.cv.notify_all();
    }
}

struct Memory {
    map: HashMap<Key, Cached>,
    order: VecDeque<Key>,
    bytes: usize,
}

impl Memory {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            bytes: 0,
        }
    }
    fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
        self.bytes = 0;
    }
    fn get(&mut self, key: &Key) -> Option<Cached> {
        if !self.map.contains_key(key) {
            return None;
        }
        self.order.retain(|item| item != key);
        self.order.push_back(key.clone());
        Some(self.map.get(key).cloned().unwrap_or(None))
    }
    fn put(&mut self, key: Key, value: Cached) {
        if let Some(old) = self.map.remove(&key) {
            self.bytes = self
                .bytes
                .saturating_sub(old.as_ref().map(|b| b.bytes.len()).unwrap_or(0));
            self.order.retain(|item| item != &key);
        }
        self.bytes += value.as_ref().map(|b| b.bytes.len()).unwrap_or(0);
        self.map.insert(key.clone(), value);
        self.order.push_back(key);
        while self.map.len() > MEMORY_ENTRIES || self.bytes > MEMORY_BYTES {
            let Some(old) = self.order.pop_front() else {
                break;
            };
            if let Some(value) = self.map.remove(&old) {
                self.bytes = self
                    .bytes
                    .saturating_sub(value.as_ref().map(|b| b.bytes.len()).unwrap_or(0));
            }
        }
    }
}

pub struct Store {
    dir: PathBuf,
    memory: Mutex<Memory>,
    inflight: Mutex<HashMap<Key, Arc<Flight>>>,
    slots: Slots,
    writes: AtomicU64,
    pub stats: Stats,
}

impl Store {
    pub fn new(dir: PathBuf) -> Self {
        let _ = std::fs::create_dir_all(&dir);
        let stale = dir.clone();
        std::thread::spawn(move || drop_old_versions(&stale));
        Self {
            dir,
            memory: Mutex::new(Memory::new()),
            inflight: Mutex::new(HashMap::new()),
            slots: Slots::new(DECODE_SLOTS),
            writes: AtomicU64::new(0),
            stats: Stats::default(),
        }
    }
    pub fn clear_memory(&self) {
        self.memory
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
    }
    /// Thumbnail for a track row already loaded from the library. `path` is the stored path.
    /// `Ok(None)` means the file has no usable cover; errors are worth retrying later.
    pub fn for_track(&self, id: i64, path: &Path, edge: u32) -> Result<Option<String>, CoverError> {
        self.render(1, id as u64, path, edge)
    }
    /// Current playback file. The path comes from the player, not from a frontend argument.
    pub fn for_playback(&self, path: &Path, edge: u32) -> Result<Option<String>, CoverError> {
        self.render(2, path_key(path), path, edge)
    }
    fn render(
        &self,
        kind: u8,
        id: u64,
        path: &Path,
        edge: u32,
    ) -> Result<Option<String>, CoverError> {
        self.stats.requests.fetch_add(1, Ordering::Relaxed);
        let edge = edge.clamp(MIN_EDGE, MAX_EDGE);
        let meta = std::fs::metadata(path).map_err(|e| {
            self.stats.transient.fetch_add(1, Ordering::Relaxed);
            if e.kind() == std::io::ErrorKind::NotFound {
                CoverError::FileMissing
            } else {
                CoverError::Unreadable
            }
        })?;
        if !meta.is_file() {
            self.stats.transient.fetch_add(1, Ordering::Relaxed);
            return Err(CoverError::FileMissing);
        }
        let mtime_ns = meta
            .modified()
            .ok()
            .and_then(|m| m.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let key = Key {
            kind,
            id,
            mtime_ns,
            len: meta.len(),
            edge,
        };
        let Some(image) = self.load(key, path, edge)? else {
            return Ok(None);
        };
        self.stats
            .bytes_out
            .fetch_add(image.bytes.len() as u64, Ordering::Relaxed);
        Ok(Some(data_url(image.mime, &image.bytes)))
    }
    fn load(&self, key: Key, path: &Path, edge: u32) -> Result<Cached, CoverError> {
        if let Some(hit) = self
            .memory
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&key)
        {
            self.stats.memory_hits.fetch_add(1, Ordering::Relaxed);
            return Ok(hit);
        }
        let (flight, leader) = {
            let mut inflight = self.inflight.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(existing) = inflight.get(&key) {
                (existing.clone(), false)
            } else {
                let created = Arc::new(Flight::pending());
                inflight.insert(key.clone(), created.clone());
                (created, true)
            }
        };
        if !leader {
            self.stats.coalesced.fetch_add(1, Ordering::Relaxed);
            let value = flight.wait();
            self.finish_wait(&key);
            return value;
        }
        let value = match self.read_disk(&key) {
            Some(hit) => {
                self.stats.disk_hits.fetch_add(1, Ordering::Relaxed);
                Ok(hit)
            }
            None => {
                let waited = Instant::now();
                let _slot = self.slots.enter();
                self.stats
                    .slot_wait_us
                    .fetch_add(waited.elapsed().as_micros() as u64, Ordering::Relaxed);
                let started = Instant::now();
                let encoded = encode_file(path, edge, &self.stats);
                let spent = started.elapsed().as_micros() as u64;
                self.stats.decode_us.fetch_add(spent, Ordering::Relaxed);
                self.stats.decode_max_us.fetch_max(spent, Ordering::Relaxed);
                match encoded {
                    Ok(Ok(image)) => {
                        let shared = Arc::new(image);
                        self.write_disk(&key, Some(&shared));
                        Ok(Some(shared))
                    }
                    Ok(Err(reason)) => {
                        self.count_absent(reason);
                        self.write_disk(&key, None);
                        Ok(None)
                    }
                    Err(error) => {
                        self.stats.transient.fetch_add(1, Ordering::Relaxed);
                        Err(error)
                    }
                }
            }
        };
        if let Ok(cached) = &value {
            self.memory
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .put(key.clone(), cached.clone());
        }
        flight.finish(value.clone());
        self.inflight
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&key);
        value
    }
    fn finish_wait(&self, key: &Key) {
        let mut inflight = self.inflight.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(flight) = inflight.get(key) {
            let done = matches!(
                &*flight.state.lock().unwrap_or_else(|e| e.into_inner()),
                FlightState::Ready(_)
            );
            if done {
                inflight.remove(key);
            }
        }
    }
    fn count_absent(&self, reason: Absent) {
        self.stats.missing.fetch_add(1, Ordering::Relaxed);
        let counter = match reason {
            Absent::NoPicture => &self.stats.no_picture,
            Absent::Unsupported => &self.stats.unsupported,
            Absent::TooLarge => &self.stats.too_large,
            Absent::Corrupt => &self.stats.corrupt,
        };
        counter.fetch_add(1, Ordering::Relaxed);
    }
    fn file_path(&self, key: &Key, ext: &str) -> PathBuf {
        self.dir.join(format!(
            "{CACHE_VERSION}-{}-{}-{}-{}-{}.{}",
            key.kind, key.id, key.mtime_ns, key.len, key.edge, ext
        ))
    }
    fn read_disk(&self, key: &Key) -> Option<Cached> {
        for (ext, mime) in IMAGE_FILES {
            let file = self.file_path(key, ext);
            if !file.is_file() {
                continue;
            }
            if let Ok(bytes) = std::fs::read(&file) {
                if !bytes.is_empty() {
                    return Some(Some(Arc::new(Image { mime, bytes })));
                }
            }
            let _ = std::fs::remove_file(&file);
        }
        if self.file_path(key, "none").is_file() {
            return Some(None);
        }
        None
    }
    fn write_disk(&self, key: &Key, image: Option<&Image>) {
        let (path, payload): (PathBuf, &[u8]) = match image {
            Some(image) => {
                let ext = IMAGE_FILES
                    .iter()
                    .find(|(_, mime)| *mime == image.mime)
                    .map(|(ext, _)| *ext)
                    .unwrap_or("jpg");
                (self.file_path(key, ext), &image.bytes)
            }
            None => (self.file_path(key, "none"), &[]),
        };
        if std::fs::write(&path, payload).is_ok() {
            let n = self.writes.fetch_add(1, Ordering::Relaxed);
            if n.is_multiple_of(16) {
                trim_disk(&self.dir);
            }
        }
    }
}

fn trim_disk(dir: &Path) {
    let mut files = Vec::new();
    let mut total = 0u64;
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_file() {
            continue;
        }
        let Ok(modified) = meta.modified() else {
            continue;
        };
        let stamp = modified
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        total += meta.len();
        files.push((stamp, entry.path(), meta.len()));
    }
    if total <= DISK_BYTES {
        return;
    }
    files.sort_by_key(|item| item.0);
    for (_, path, len) in files {
        if total <= DISK_BYTES * 3 / 4 {
            break;
        }
        if std::fs::remove_file(path).is_ok() {
            total = total.saturating_sub(len);
        }
    }
}

fn path_key(path: &Path) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in path.to_string_lossy().as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

const IMAGE_FILES: [(&str, &str); 3] = [
    ("jpg", "image/jpeg"),
    ("webp", "image/webp"),
    ("gif", "image/gif"),
];

/// Removes disk entries without the current version prefix. Runs once, off the setup thread.
fn drop_old_versions(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let prefix = format!("{CACHE_VERSION}-");
    for entry in entries.flatten() {
        let name = entry.file_name();
        if !name.to_string_lossy().starts_with(&prefix) && entry.path().is_file() {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

fn data_url(mime: &str, bytes: &[u8]) -> String {
    format!(
        "data:{mime};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}

fn encode_file(path: &Path, edge: u32, stats: &Stats) -> Result<Result<Image, Absent>, CoverError> {
    use crate::metadata::CoverReadError;
    match crate::metadata::read_cover_checked(path) {
        Ok(Some(source)) => Ok(encode_source(&source, edge, stats)),
        Ok(None) => Ok(Err(Absent::NoPicture)),
        Err(CoverReadError::Tags) => Ok(Err(Absent::Unsupported)),
        Err(CoverReadError::Io) => Err(CoverError::Unreadable),
    }
}

/// JPEG thumbnail of a JPEG or PNG picture. Other inputs give `None`.
#[cfg(test)]
fn encode_bytes(source: &[u8], edge: u32, stats: &Stats) -> Option<Vec<u8>> {
    encode_source(source, edge, stats)
        .ok()
        .filter(|image| image.mime == "image/jpeg")
        .map(|image| image.bytes)
}

fn encode_source(source: &[u8], edge: u32, stats: &Stats) -> Result<Image, Absent> {
    if source.is_empty() {
        return Err(Absent::NoPicture);
    }
    if source.len() > MAX_SOURCE_BYTES {
        return Err(Absent::TooLarge);
    }
    let format = image::guess_format(source).map_err(|_| Absent::Corrupt)?;
    let passthrough = match format {
        image::ImageFormat::WebP => Some("image/webp"),
        image::ImageFormat::Gif => Some("image/gif"),
        image::ImageFormat::Jpeg | image::ImageFormat::Png => None,
        _ => return Err(Absent::Unsupported),
    };
    if let Some(mime) = passthrough {
        if source.len() > MAX_PASSTHROUGH_BYTES {
            return Err(Absent::TooLarge);
        }
        stats.passthrough.fetch_add(1, Ordering::Relaxed);
        return Ok(Image {
            mime,
            bytes: source.to_vec(),
        });
    }
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_PIXELS);
    limits.max_image_height = Some(MAX_PIXELS);
    limits.max_alloc = Some(MAX_DECODE_ALLOC);
    let mut reader = image::ImageReader::with_format(Cursor::new(source), format);
    reader.limits(limits);
    stats.decoded.fetch_add(1, Ordering::Relaxed);
    let image = reader.decode().map_err(|e| match e {
        image::ImageError::Limits(_) => Absent::TooLarge,
        _ => Absent::Corrupt,
    })?;
    let fitted = fit(image, edge);
    let rgb = fitted.into_rgb8();
    let mut output = Cursor::new(Vec::new());
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 80);
    encoder
        .encode(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|_| Absent::Corrupt)?;
    let bytes = output.into_inner();
    if bytes.is_empty() {
        return Err(Absent::Corrupt);
    }
    Ok(Image {
        mime: "image/jpeg",
        bytes,
    })
}

fn fit(image: image::DynamicImage, edge: u32) -> image::DynamicImage {
    let width = image.width().max(1);
    let height = image.height().max(1);
    let longest = width.max(height);
    if longest <= edge {
        return image;
    }
    let target_w = (u64::from(width) * u64::from(edge) / u64::from(longest)).max(1) as u32;
    let target_h = (u64::from(height) * u64::from(edge) / u64::from(longest)).max(1) as u32;
    // Area sampling: a 3000 px cover down to a thumbnail is far cheaper than a Triangle filter.
    image.thumbnail(target_w, target_h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, Rgb, RgbImage};

    fn jpeg(width: u32, height: u32, color: [u8; 3], quality: u8) -> Vec<u8> {
        let mut image = RgbImage::new(width, height);
        for pixel in image.pixels_mut() {
            *pixel = Rgb(color);
        }
        let mut output = Cursor::new(Vec::new());
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
        encoder
            .encode(
                image.as_raw(),
                width,
                height,
                image::ExtendedColorType::Rgb8,
            )
            .unwrap();
        output.into_inner()
    }

    fn dimensions(bytes: &[u8]) -> (u32, u32) {
        let image = image::load_from_memory(bytes).unwrap();
        (image.width(), image.height())
    }

    #[test]
    fn thumbnail_keeps_aspect_and_does_not_upscale() {
        let stats = Stats::default();
        let wide = encode_bytes(&jpeg(200, 100, [20, 40, 200], 90), 40, &stats).unwrap();
        assert_eq!(dimensions(&wide), (40, 20));
        let small = encode_bytes(&jpeg(10, 10, [1, 2, 3], 90), 96, &stats).unwrap();
        assert_eq!(dimensions(&small), (10, 10));
        assert_eq!(stats.decoded.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn corrupt_and_oversized_images_are_rejected() {
        let stats = Stats::default();
        assert!(encode_bytes(b"not an image", 96, &stats).is_none());
        assert!(encode_bytes(&vec![0u8; MAX_SOURCE_BYTES + 1], 96, &stats).is_none());
        assert_eq!(stats.decoded.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn small_thumbnail_is_far_smaller_than_the_original_ipc_payload() {
        let mut image = RgbImage::new(1600, 1200);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = Rgb([(x % 251) as u8, (y % 241) as u8, ((x * 3 + y) % 255) as u8]);
        }
        let mut original = Cursor::new(Vec::new());
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut original, 95)
            .encode(image.as_raw(), 1600, 1200, image::ExtendedColorType::Rgb8)
            .unwrap();
        let original = original.into_inner();
        let before = data_url("image/jpeg", &original).len();
        let stats = Stats::default();
        let thumb = encode_bytes(&original, 96, &stats).unwrap();
        let after = data_url("image/jpeg", &thumb).len();
        eprintln!(
            "cover-bytes original_ipc={before} thumb_ipc={after} thumb_jpeg={}",
            thumb.len()
        );
        assert!(
            after * 8 < before,
            "thumb {after} should be much smaller than {before}"
        );
        assert_eq!(dimensions(&thumb).0.max(dimensions(&thumb).1), 96);
        assert_eq!(stats.decoded.load(Ordering::Relaxed), 1);
    }

    fn fixture_copy(dir: &Path, name: &str) -> PathBuf {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.flac");
        let before = std::fs::read(&source).unwrap();
        let dest = dir.join(name);
        std::fs::copy(&source, &dest).unwrap();
        assert_eq!(
            std::fs::read(&source).unwrap(),
            before,
            "fixture must stay unchanged"
        );
        dest
    }

    fn embed(path: &Path, picture: Vec<u8>) {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        use lofty::picture::{MimeType, Picture, PictureType};
        let mut tagged = lofty::read_from_path(path).unwrap();
        let picture =
            Picture::new_unchecked(PictureType::CoverFront, Some(MimeType::Jpeg), None, picture);
        if let Some(tag) = tagged.primary_tag_mut() {
            tag.remove_picture_type(PictureType::CoverFront);
            tag.push_picture(picture);
        } else {
            let mut tag = lofty::tag::Tag::new(tagged.primary_tag_type());
            tag.push_picture(picture);
            tagged.insert_tag(tag);
        }
        tagged.save_to_path(path, WriteOptions::default()).unwrap();
    }

    #[test]
    fn memory_and_disk_cache_decode_once_and_drop_when_the_file_changes() {
        let dir = std::env::temp_dir().join(format!("mm-covers-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let audio = fixture_copy(&dir, "song.flac");
        embed(&audio, jpeg(80, 40, [220, 10, 10], 90));
        let store = Store::new(dir.join("cache"));
        let first = store.for_track(7, &audio, 32).unwrap().unwrap();
        let second = store.for_track(7, &audio, 32).unwrap().unwrap();
        assert_eq!(first, second);
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 1);
        assert!(store.stats.memory_hits.load(Ordering::Relaxed) >= 1);
        let reopened = Store::new(dir.join("cache"));
        assert!(reopened.for_track(7, &audio, 32).unwrap().is_some());
        assert_eq!(reopened.stats.decoded.load(Ordering::Relaxed), 0);
        assert!(reopened.stats.disk_hits.load(Ordering::Relaxed) >= 1);
        store.clear_memory();
        std::thread::sleep(std::time::Duration::from_millis(20));
        embed(&audio, jpeg(80, 40, [10, 220, 10], 90));
        let changed = store.for_track(7, &audio, 32).unwrap().unwrap();
        assert_ne!(first, changed);
        assert!(store.stats.decoded.load(Ordering::Relaxed) >= 2);
        let plain = fixture_copy(&dir, "plain.flac");
        assert!(store.for_track(8, &plain, 32).unwrap().is_none());
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 2);
        assert!(store.for_track(8, &plain, 32).unwrap().is_none());
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn same_title_files_keep_their_own_covers() {
        let dir = std::env::temp_dir().join(format!("mm-covers-pair-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let red = fixture_copy(&dir, "red.flac");
        let green = fixture_copy(&dir, "green.flac");
        embed(&red, jpeg(60, 40, [230, 20, 20], 95));
        embed(&green, jpeg(40, 60, [20, 230, 20], 95));
        let store = Store::new(dir.join("cache"));
        let left = store.for_track(1, &red, 40).unwrap().unwrap();
        let right = store.for_track(2, &green, 40).unwrap().unwrap();
        assert_ne!(left, right);
        let red_px = image::load_from_memory(&{
            // The stored JPEG is inside the data URL; compare the decoded thumb colors.
            let raw = store.for_track(1, &red, 40).unwrap().unwrap();
            let b64 = raw.split_once(',').unwrap().1;
            base64::engine::general_purpose::STANDARD
                .decode(b64)
                .unwrap()
        })
        .unwrap();
        let green_px = image::load_from_memory(&{
            let raw = right;
            let b64 = raw.split_once(',').unwrap().1;
            base64::engine::general_purpose::STANDARD
                .decode(b64)
                .unwrap()
        })
        .unwrap();
        let red_sample = red_px.get_pixel(0, 0);
        let green_sample = green_px.get_pixel(0, 0);
        assert!(red_sample[0] > red_sample[1]);
        assert!(green_sample[1] > green_sample[0]);
        if std::env::var_os("MM_EXPORT_COVERS").is_some() {
            let out = std::env::temp_dir().join("music-mine-cover-audio");
            let data = std::env::temp_dir().join("music-mine-cover-data");
            let _ = std::fs::remove_dir_all(&out);
            let _ = std::fs::remove_dir_all(&data);
            std::fs::create_dir_all(&out).unwrap();
            let exported_red = fixture_copy(&out, "red.flac");
            let exported_green = fixture_copy(&out, "green.flac");
            let _plain = fixture_copy(&out, "plain.flac");
            embed(&exported_red, jpeg(80, 60, [230, 20, 20], 95));
            embed(&exported_green, jpeg(60, 80, [20, 230, 20], 95));
            let catalog = crate::catalog::Catalog::new(data.join("library-tauri-v1.sqlite"));
            catalog.scan(out.clone()).unwrap();
            eprintln!("exported {} {}", out.display(), data.display());
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn concurrent_requests_share_one_decode() {
        let dir = std::env::temp_dir().join(format!("mm-covers-flight-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let audio = fixture_copy(&dir, "song.flac");
        embed(&audio, jpeg(120, 80, [8, 16, 32], 90));
        let store = Arc::new(Store::new(dir.join("cache")));
        let mut threads = Vec::new();
        for _ in 0..8 {
            let store = store.clone();
            let audio = audio.clone();
            threads.push(std::thread::spawn(move || store.for_track(3, &audio, 48)));
        }
        let results: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
        assert!(results
            .iter()
            .all(|item| item == &results[0] && matches!(item, Ok(Some(_)))));
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 1);
        assert!(store.stats.coalesced.load(Ordering::Relaxed) >= 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn embed_picture(
        path: &Path,
        kind: lofty::picture::PictureType,
        mime: lofty::picture::MimeType,
        data: Vec<u8>,
    ) {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        let mut tagged = lofty::read_from_path(path).unwrap();
        if tagged.primary_tag().is_none() {
            tagged.insert_tag(lofty::tag::Tag::new(tagged.primary_tag_type()));
        }
        let tag = tagged.primary_tag_mut().unwrap();
        tag.push_picture(lofty::picture::Picture::new_unchecked(
            kind,
            Some(mime),
            None,
            data,
        ));
        tagged.save_to_path(path, WriteOptions::default()).unwrap();
    }

    fn cache_names(dir: &Path) -> Vec<String> {
        let mut names: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn outcomes_distinguish_formats_limits_and_corruption() {
        let stats = Stats::default();
        let webp = base64::engine::general_purpose::STANDARD
            .decode("UklGRhoAAABXRUJQVlA4TA0AAAAvAAAAEAcQERGIiP4HAA==")
            .unwrap();
        let gif = base64::engine::general_purpose::STANDARD
            .decode("R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==")
            .unwrap();
        assert_eq!(encode_source(&webp, 96, &stats).unwrap().mime, "image/webp");
        assert_eq!(encode_source(&gif, 96, &stats).unwrap().mime, "image/gif");
        let mut big_webp = webp.clone();
        big_webp.resize(MAX_PASSTHROUGH_BYTES + 1, 0);
        assert_eq!(
            encode_source(&big_webp, 96, &stats).err(),
            Some(Absent::TooLarge)
        );
        let mut broken = jpeg(64, 64, [1, 2, 3], 90);
        broken.truncate(40);
        assert_eq!(
            encode_source(&broken, 96, &stats).err(),
            Some(Absent::Corrupt)
        );
        assert_eq!(
            encode_source(b"not an image", 96, &stats).err(),
            Some(Absent::Corrupt)
        );
        assert_eq!(
            encode_source(b"BM\0\0\0\0\0\0\0\0\0\0\0\0", 96, &stats).err(),
            Some(Absent::Unsupported)
        );
        assert_eq!(
            encode_source(&vec![0u8; MAX_SOURCE_BYTES + 1], 96, &stats).err(),
            Some(Absent::TooLarge)
        );
        // 3000 x 3000 RGBA needs 36 MB to decode; the old 32 MB limit cached it as "no cover".
        let rgba = image::RgbaImage::from_pixel(3000, 3000, image::Rgba([200, 30, 30, 255]));
        let mut png = Cursor::new(Vec::new());
        rgba.write_to(&mut png, image::ImageFormat::Png).unwrap();
        let thumb = encode_source(png.get_ref(), 112, &stats).unwrap();
        assert_eq!(thumb.mime, "image/jpeg");
        assert_eq!(dimensions(&thumb.bytes), (112, 112));
    }

    #[test]
    fn transient_failures_are_not_cached_as_no_cover() {
        let dir = std::env::temp_dir().join(format!("mm-covers-transient-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let store = Store::new(dir.join("cache"));
        let audio = dir.join("later.flac");
        assert_eq!(store.for_track(5, &audio, 64), Err(CoverError::FileMissing));
        assert!(cache_names(&dir.join("cache")).is_empty());
        fixture_copy(&dir, "later.flac");
        embed(&audio, jpeg(80, 80, [9, 9, 200], 90));
        assert!(store.for_track(5, &audio, 64).unwrap().is_some());
        let plain = fixture_copy(&dir, "plain.flac");
        assert_eq!(store.for_track(6, &plain, 64), Ok(None));
        let names = cache_names(&dir.join("cache"));
        assert!(names.iter().all(|n| n.starts_with("v2-")), "{names:?}");
        assert_eq!(names.iter().filter(|n| n.ends_with(".none")).count(), 1);
        assert_eq!(store.stats.transient.load(Ordering::Relaxed), 1);
        assert_eq!(store.stats.no_picture.load(Ordering::Relaxed), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn old_cache_entries_are_dropped_and_front_cover_wins() {
        use lofty::picture::{MimeType, PictureType};
        let dir = std::env::temp_dir().join(format!("mm-covers-front-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cache = dir.join("cache");
        std::fs::create_dir_all(&cache).unwrap();
        std::fs::write(cache.join("1-4-0-0-112.none"), b"").unwrap();
        let audio = fixture_copy(&dir, "two.flac");
        embed_picture(
            &audio,
            PictureType::Other,
            MimeType::Jpeg,
            jpeg(40, 40, [230, 10, 10], 95),
        );
        embed_picture(
            &audio,
            PictureType::CoverFront,
            MimeType::Jpeg,
            jpeg(40, 40, [10, 230, 10], 95),
        );
        let store = Store::new(cache.clone());
        let url = store.for_track(4, &audio, 40).unwrap().unwrap();
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(url.split_once(',').unwrap().1)
            .unwrap();
        let pixel = image::load_from_memory(&bytes).unwrap().get_pixel(5, 5);
        assert!(pixel[1] > pixel[0], "front cover expected, got {pixel:?}");
        for _ in 0..50 {
            if !cache.join("1-4-0-0-112.none").exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(!cache.join("1-4-0-0-112.none").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
