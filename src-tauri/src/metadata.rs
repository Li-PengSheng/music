use std::path::Path;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::{Accessor, ItemKey};

use crate::library::{AlbumSource, Track};

fn clean<S: Into<String>>(value: Option<S>) -> Option<String> {
    value
        .map(|s| s.into().trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Read metadata from an audio file using lofty. Tags missing from the primary
/// tag are looked up in every other tag the file carries, because some files
/// keep title/album in a secondary tag (e.g. ID3v1 next to ID3v2, or RIFF INFO
/// in a WAV).
pub fn read_track(path: &Path, id: i64) -> Option<Track> {
    let tagged = lofty::read_from_path(path).ok()?;
    let duration_ms = tagged.properties().duration().as_millis() as u64;

    let mut title = None;
    let mut artist = None;
    let mut album = None;
    let mut album_artist = None;
    let mut genre = None;
    let mut year = None;
    let mut track_number = None;
    let mut disc_number = None;

    // Primary tag first, then any remaining tag as a fallback.
    let primary = tagged.primary_tag().map(|t| t as *const _);
    let mut tags: Vec<&lofty::tag::Tag> = Vec::new();
    if let Some(p) = tagged.primary_tag() {
        tags.push(p);
    }
    for t in tagged.tags() {
        if Some(t as *const _) != primary {
            tags.push(t);
        }
    }

    for t in tags {
        if title.is_none() {
            title = clean(t.title());
        }
        if artist.is_none() {
            artist = clean(t.artist());
        }
        if album.is_none() {
            album = clean(t.album());
        }
        if album_artist.is_none() {
            album_artist = clean(t.get_string(&ItemKey::AlbumArtist));
        }
        if genre.is_none() {
            genre = clean(t.genre());
        }
        if year.is_none() {
            year = t.year();
        }
        if track_number.is_none() {
            track_number = t.track();
        }
        if disc_number.is_none() {
            disc_number = t.disk();
        }
    }

    // Untagged files fall back to the file name (and its leading track number).
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未知歌曲");
    let (file_title, file_track_no) = split_track_number(stem);

    let has_album_tag = album.is_some();
    Some(Track {
        id,
        path: path.to_string_lossy().to_string(),
        title: title.unwrap_or(file_title),
        artist: artist.unwrap_or_else(|| "未知艺术家".to_string()),
        // Album stays empty when the file has no album tag; the scanner decides
        // whether the containing folder is a better guess.
        album: album.unwrap_or_default(),
        album_artist: album_artist.unwrap_or_default(),
        genre: genre.unwrap_or_default(),
        year,
        track_number: track_number.or(file_track_no),
        disc_number,
        duration_ms,
        album_source: if has_album_tag {
            AlbumSource::Tag
        } else {
            AlbumSource::Unknown
        },
        // Filled in by the database on first insert, preserved on rescans.
        added_at: 0,
    })
}

/// Split a leading track number off a file name: "01 - Title", "02. Title",
/// "03 Title" and "04_Title" all yield (title, Some(number)).
fn split_track_number(stem: &str) -> (String, Option<u32>) {
    let s = stem.trim();
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() || digits.len() > 3 {
        return (s.to_string(), None);
    }
    let rest = &s[digits.len()..];
    let trimmed = rest
        .trim_start_matches([' ', '-', '.', '_', '　'])
        .trim_start();
    let number = digits.parse::<u32>().ok();
    if trimmed.is_empty() {
        return (s.to_string(), number);
    }
    (trimmed.to_string(), number)
}

/// Extract the first embedded cover image (bytes) from a file's tags.
pub fn read_cover(path: &Path) -> Option<Vec<u8>> {
    let tagged = lofty::read_from_path(path).ok()?;
    for tag in tagged.tags() {
        if let Some(p) = tag.pictures().iter().find(|p| !p.data().is_empty()) {
            return Some(p.data().to_vec());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::split_track_number;

    #[test]
    fn splits_track_numbers() {
        assert_eq!(split_track_number("01 - Night"), ("Night".into(), Some(1)));
        assert_eq!(split_track_number("02. Sea"), ("Sea".into(), Some(2)));
        assert_eq!(split_track_number("03 Song"), ("Song".into(), Some(3)));
        assert_eq!(split_track_number("04_Song"), ("Song".into(), Some(4)));
        assert_eq!(split_track_number("Song"), ("Song".into(), None));
        // Four leading digits are a year, not a track number.
        assert_eq!(split_track_number("1984"), ("1984".into(), None));
        assert_eq!(split_track_number("1000 days"), ("1000 days".into(), None));
    }
}
