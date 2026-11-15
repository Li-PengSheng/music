//! Isolated large cover corpus for queue cover measurements. Never touches a user library:
//! every audio file is a copy of the repository fixtures with a generated picture.
//!
//! `MM_COVER_CORPUS=<dir> cargo test --release cover_corpus -- --ignored --nocapture`
//! writes `<dir>/music` and scans it into `<dir>/app-data/library-tauri-v1.sqlite`.
//! With `MM_COVER_CORPUS_RECOVER=1` it only replaces the pictures of the first five files,
//! keeping their names, to exercise cache invalidation.
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage, Rgba, RgbaImage};
use lofty::config::WriteOptions;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::picture::{MimeType, Picture, PictureType};
use lofty::tag::{Accessor, Tag};

const WEBP_1X1: &str = "UklGRhoAAABXRUJQVlA4TA0AAAAvAAAAEAcQERGIiP4HAA==";
const GIF_1X1: &str = "R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==";

struct Seed(u32);
impl Seed {
    fn next(&mut self) -> u8 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.0 >> 24) as u8
    }
}

/// Smooth gradient with mild noise: realistic JPEG sizes without multi-megabyte noise.
fn picture_rgb(edge: u32, seed: u32) -> RgbImage {
    let mut noise = Seed(seed);
    let base = [
        (seed * 53 % 256) as u8,
        (seed * 97 % 256) as u8,
        (seed * 29 % 256) as u8,
    ];
    RgbImage::from_fn(edge, edge, |x, y| {
        let n = noise.next() / 16;
        Rgb([
            base[0]
                .wrapping_add((x * 255 / edge) as u8 / 2)
                .wrapping_add(n),
            base[1]
                .wrapping_add((y * 255 / edge) as u8 / 2)
                .wrapping_add(n),
            base[2]
                .wrapping_add(((x + y) * 127 / edge) as u8)
                .wrapping_add(n),
        ])
    })
}

fn jpeg(edge: u32, seed: u32) -> Vec<u8> {
    let image = picture_rgb(edge, seed);
    let mut out = Cursor::new(Vec::new());
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 88)
        .encode(image.as_raw(), edge, edge, image::ExtendedColorType::Rgb8)
        .unwrap();
    out.into_inner()
}

fn png_rgb(edge: u32, seed: u32) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    picture_rgb(edge, seed)
        .write_to(&mut out, image::ImageFormat::Png)
        .unwrap();
    out.into_inner()
}

fn png_rgba(edge: u32, seed: u32, noisy: bool) -> Vec<u8> {
    let mut noise = Seed(seed);
    let image = RgbaImage::from_fn(edge, edge, |x, y| {
        if noisy {
            Rgba([noise.next(), noise.next(), noise.next(), 255])
        } else {
            Rgba([(x % 256) as u8, (y % 256) as u8, (seed % 256) as u8, 200])
        }
    });
    let mut out = Cursor::new(Vec::new());
    image.write_to(&mut out, image::ImageFormat::Png).unwrap();
    out.into_inner()
}

fn b64(text: &str) -> Vec<u8> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(text)
        .unwrap()
}

fn embed(
    path: &Path,
    title: &str,
    album: &str,
    artist: &str,
    picture: Option<(Vec<u8>, MimeType)>,
) {
    let mut tagged = lofty::read_from_path(path).unwrap();
    if tagged.primary_tag().is_none() {
        tagged.insert_tag(Tag::new(tagged.primary_tag_type()));
    }
    let tag = tagged.primary_tag_mut().unwrap();
    tag.set_title(title.into());
    tag.set_album(album.into());
    tag.set_artist(artist.into());
    tag.remove_picture_type(PictureType::CoverFront);
    if let Some((data, mime)) = picture {
        tag.push_picture(Picture::new_unchecked(
            PictureType::CoverFront,
            Some(mime),
            None,
            data,
        ));
    }
    tagged.save_to_path(path, WriteOptions::default()).unwrap();
}

struct Entry {
    kind: &'static str,
    folder: String,
    file: String,
    picture: Option<(Vec<u8>, MimeType)>,
}

fn entries() -> Vec<Entry> {
    let mut out = Vec::new();
    let mut push = |kind: &'static str, picture: Option<(Vec<u8>, MimeType)>| {
        let i = out.len();
        out.push(Entry {
            kind,
            folder: kind.to_string(),
            file: format!("{i:03} {kind}"),
            picture,
        });
    };
    for i in 0..194u32 {
        push(
            "jpeg-small",
            Some((jpeg([300, 500, 800][i as usize % 3], i), MimeType::Jpeg)),
        );
    }
    for i in 0..50u32 {
        push("jpeg-3000", Some((jpeg(3000, 1000 + i), MimeType::Jpeg)));
    }
    for i in 0..20u32 {
        push("png-1000", Some((png_rgb(1000, 2000 + i), MimeType::Png)));
    }
    for i in 0..8u32 {
        push(
            "png-3000-rgba",
            Some((png_rgba(3000, 3000 + i, false), MimeType::Png)),
        );
    }
    for i in 0..4u32 {
        push(
            "png-noise-13mb",
            Some((png_rgba(1800, 4000 + i, true), MimeType::Png)),
        );
    }
    for _ in 0..10 {
        push(
            "webp",
            Some((b64(WEBP_1X1), MimeType::Unknown("image/webp".into()))),
        );
    }
    for _ in 0..5 {
        push("gif", Some((b64(GIF_1X1), MimeType::Gif)));
    }
    for i in 0..5u8 {
        let mut broken = vec![0xff, 0xd8, 0xff, 0xe0];
        broken.extend(std::iter::repeat_n(i, 4096));
        push("corrupt-jpeg", Some((broken, MimeType::Jpeg)));
    }
    for _ in 0..30 {
        push("no-cover", None);
    }
    for i in 0..10u32 {
        out.push(Entry {
            kind: "same-name",
            folder: format!("same-name/{i:02}"),
            file: "Same Name".into(),
            picture: Some((jpeg(400, 5000 + i), MimeType::Jpeg)),
        });
    }
    out
}

#[test]
#[ignore]
fn cover_corpus() {
    let Some(root) = std::env::var_os("MM_COVER_CORPUS").map(PathBuf::from) else {
        eprintln!("MM_COVER_CORPUS is not set; nothing generated");
        return;
    };
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let music = root.join("music");
    if std::env::var_os("MM_COVER_CORPUS_RECOVER").is_some() {
        for (i, entry) in entries().into_iter().take(5).enumerate() {
            let ext = if i % 2 == 0 { "flac" } else { "mp3" };
            let path = music
                .join(&entry.folder)
                .join(format!("{}.{ext}", entry.file));
            let title = format!("Cover {i:03} {}", entry.kind);
            embed(
                &path,
                &title,
                "Album jpeg-small 0",
                "Corpus Artist 0",
                Some((jpeg(500, 9000 + i as u32), MimeType::Jpeg)),
            );
        }
        eprintln!("recovered 5 pictures under {}", music.display());
        return;
    }
    assert!(
        !music.exists(),
        "corpus already exists: {}",
        music.display()
    );
    let data = root.join("app-data");
    std::fs::create_dir_all(&data).unwrap();
    let mut kinds = std::collections::BTreeMap::<&str, (usize, usize)>::new();
    for (i, entry) in entries().into_iter().enumerate() {
        let ext = if i % 2 == 0 { "flac" } else { "mp3" };
        let dir = music.join(&entry.folder);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("{}.{ext}", entry.file));
        std::fs::copy(fixtures.join(format!("tone.{ext}")), &path).unwrap();
        let title = format!("Cover {i:03} {}", entry.kind);
        let album = format!("Album {} {}", entry.kind, i / 12);
        let artist = format!("Corpus Artist {}", i % 9);
        let bytes = entry.picture.as_ref().map(|p| p.0.len()).unwrap_or(0);
        embed(&path, &title, &album, &artist, entry.picture);
        let slot = kinds.entry(entry.kind).or_default();
        slot.0 += 1;
        slot.1 = slot.1.max(bytes);
    }
    let catalog = crate::catalog::Catalog::new(data.join("library-tauri-v1.sqlite"));
    catalog.scan(music.clone()).unwrap();
    for (kind, (count, max)) in kinds {
        eprintln!("corpus {kind}: {count} files, largest picture {max} bytes");
    }
    eprintln!("corpus music={} data={}", music.display(), data.display());
}
