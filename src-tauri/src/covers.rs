//! Bounded cover thumbnails. Decoding happens on the caller's blocking thread,
//! never on the audio callback. Disk files live under the app cache directory.
use std::collections::{HashMap, VecDeque};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::UNIX_EPOCH;

use base64::Engine;

const MAX_SOURCE_BYTES: usize = 8 * 1024 * 1024;
const MAX_EDGE: u32 = 960;
const MIN_EDGE: u32 = 32;
const MAX_PIXELS: u32 = 8_192;
const MAX_DECODE_ALLOC: u64 = 32 * 1024 * 1024;
const MEMORY_ENTRIES: usize = 240;
const MEMORY_BYTES: usize = 24 * 1024 * 1024;
const DISK_BYTES: u64 = 64 * 1024 * 1024;
const DECODE_SLOTS: u32 = 2;

#[derive(Default)]
pub struct Stats {
    pub decoded: AtomicU64,
    pub memory_hits: AtomicU64,
    pub disk_hits: AtomicU64,
    pub missing: AtomicU64,
    pub coalesced: AtomicU64,
    pub bytes_out: AtomicU64,
}

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
    Ready(Option<Arc<Vec<u8>>>),
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
    fn wait(&self) -> Option<Arc<Vec<u8>>> {
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
    fn finish(&self, value: Option<Arc<Vec<u8>>>) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        *state = FlightState::Ready(value);
        self.cv.notify_all();
    }
}

struct Memory {
    map: HashMap<Key, Option<Arc<Vec<u8>>>>,
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
    fn get(&mut self, key: &Key) -> Option<Option<Arc<Vec<u8>>>> {
        if !self.map.contains_key(key) {
            return None;
        }
        self.order.retain(|item| item != key);
        self.order.push_back(key.clone());
        Some(self.map.get(key).cloned().unwrap_or(None))
    }
    fn put(&mut self, key: Key, value: Option<Arc<Vec<u8>>>) {
        if let Some(old) = self.map.remove(&key) {
            self.bytes = self
                .bytes
                .saturating_sub(old.as_ref().map(|b| b.len()).unwrap_or(0));
            self.order.retain(|item| item != &key);
        }
        self.bytes += value.as_ref().map(|b| b.len()).unwrap_or(0);
        self.map.insert(key.clone(), value);
        self.order.push_back(key);
        while self.map.len() > MEMORY_ENTRIES || self.bytes > MEMORY_BYTES {
            let Some(old) = self.order.pop_front() else {
                break;
            };
            if let Some(value) = self.map.remove(&old) {
                self.bytes = self
                    .bytes
                    .saturating_sub(value.as_ref().map(|b| b.len()).unwrap_or(0));
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
    pub fn for_track(&self, id: i64, path: &Path, edge: u32) -> Option<String> {
        self.render(1, id as u64, path, edge)
    }
    /// Current playback file. The path comes from the player, not from a frontend argument.
    pub fn for_playback(&self, path: &Path, edge: u32) -> Option<String> {
        self.render(2, path_key(path), path, edge)
    }
    fn render(&self, kind: u8, id: u64, path: &Path, edge: u32) -> Option<String> {
        let edge = edge.clamp(MIN_EDGE, MAX_EDGE);
        let meta = std::fs::metadata(path).ok()?;
        if !meta.is_file() {
            return None;
        }
        let mtime_ns = meta
            .modified()
            .ok()?
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_nanos();
        let key = Key {
            kind,
            id,
            mtime_ns,
            len: meta.len(),
            edge,
        };
        let bytes = self.load(key, path, edge)?;
        self.stats
            .bytes_out
            .fetch_add(bytes.len() as u64, Ordering::Relaxed);
        Some(data_url(&bytes))
    }
    fn load(&self, key: Key, path: &Path, edge: u32) -> Option<Arc<Vec<u8>>> {
        if let Some(hit) = self
            .memory
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&key)
        {
            self.stats.memory_hits.fetch_add(1, Ordering::Relaxed);
            return hit;
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
                hit
            }
            None => {
                let _slot = self.slots.enter();
                let encoded = encode_file(path, edge, &self.stats);
                if encoded.is_none() {
                    self.stats.missing.fetch_add(1, Ordering::Relaxed);
                }
                let shared = encoded.map(Arc::new);
                self.write_disk(&key, shared.as_deref());
                shared
            }
        };
        self.memory
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .put(key.clone(), value.clone());
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
    fn file_path(&self, key: &Key, ext: &str) -> PathBuf {
        self.dir.join(format!(
            "{}-{}-{}-{}-{}.{}",
            key.kind, key.id, key.mtime_ns, key.len, key.edge, ext
        ))
    }
    fn read_disk(&self, key: &Key) -> Option<Option<Arc<Vec<u8>>>> {
        let jpeg = self.file_path(key, "jpg");
        if jpeg.is_file() {
            if let Ok(bytes) = std::fs::read(&jpeg) {
                if !bytes.is_empty() {
                    return Some(Some(Arc::new(bytes)));
                }
            }
            let _ = std::fs::remove_file(&jpeg);
        }
        if self.file_path(key, "none").is_file() {
            return Some(None);
        }
        None
    }
    fn write_disk(&self, key: &Key, bytes: Option<&Vec<u8>>) {
        let path = match bytes {
            Some(bytes) if !bytes.is_empty() => self.file_path(key, "jpg"),
            _ => self.file_path(key, "none"),
        };
        let payload = bytes.cloned().unwrap_or_default();
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

fn data_url(bytes: &[u8]) -> String {
    format!(
        "data:image/jpeg;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}

fn encode_file(path: &Path, edge: u32, stats: &Stats) -> Option<Vec<u8>> {
    let source = crate::metadata::read_cover(path)?;
    encode_bytes(&source, edge, stats)
}

pub fn encode_bytes(source: &[u8], edge: u32, stats: &Stats) -> Option<Vec<u8>> {
    if source.is_empty() || source.len() > MAX_SOURCE_BYTES {
        return None;
    }
    let format = image::guess_format(source).ok()?;
    if !matches!(format, image::ImageFormat::Jpeg | image::ImageFormat::Png) {
        return None;
    }
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_PIXELS);
    limits.max_image_height = Some(MAX_PIXELS);
    limits.max_alloc = Some(MAX_DECODE_ALLOC);
    let mut reader = image::ImageReader::with_format(Cursor::new(source), format);
    reader.limits(limits);
    stats.decoded.fetch_add(1, Ordering::Relaxed);
    let image = reader.decode().ok()?;
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
        .ok()?;
    let bytes = output.into_inner();
    if bytes.is_empty() {
        None
    } else {
        Some(bytes)
    }
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
    image.resize(target_w, target_h, image::imageops::FilterType::Triangle)
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
        let before = data_url(&original).len();
        let stats = Stats::default();
        let thumb = encode_bytes(&original, 96, &stats).unwrap();
        let after = data_url(&thumb).len();
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
        let first = store.for_track(7, &audio, 32).unwrap();
        let second = store.for_track(7, &audio, 32).unwrap();
        assert_eq!(first, second);
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 1);
        assert!(store.stats.memory_hits.load(Ordering::Relaxed) >= 1);
        let reopened = Store::new(dir.join("cache"));
        assert!(reopened.for_track(7, &audio, 32).is_some());
        assert_eq!(reopened.stats.decoded.load(Ordering::Relaxed), 0);
        assert!(reopened.stats.disk_hits.load(Ordering::Relaxed) >= 1);
        store.clear_memory();
        std::thread::sleep(std::time::Duration::from_millis(20));
        embed(&audio, jpeg(80, 40, [10, 220, 10], 90));
        let changed = store.for_track(7, &audio, 32).unwrap();
        assert_ne!(first, changed);
        assert!(store.stats.decoded.load(Ordering::Relaxed) >= 2);
        let plain = fixture_copy(&dir, "plain.flac");
        assert!(store.for_track(8, &plain, 32).is_none());
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 2);
        assert!(store.for_track(8, &plain, 32).is_none());
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
        let left = store.for_track(1, &red, 40).unwrap();
        let right = store.for_track(2, &green, 40).unwrap();
        assert_ne!(left, right);
        let red_px = image::load_from_memory(&{
            // The stored JPEG is inside the data URL; compare the decoded thumb colors.
            let raw = store.for_track(1, &red, 40).unwrap();
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
            .all(|item| item == &results[0] && item.is_some()));
        assert_eq!(store.stats.decoded.load(Ordering::Relaxed), 1);
        assert!(store.stats.coalesced.load(Ordering::Relaxed) >= 1);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
