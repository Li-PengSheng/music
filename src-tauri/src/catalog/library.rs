use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::metadata;

/// Where a track's album value came from. Grouping must not treat a folder
/// guess the same way as a real album tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlbumSource {
    /// Read from the file's album tag.
    Tag,
    /// Tag missing; the containing folder was used (it holds several songs).
    Folder,
    /// Tag missing and the folder had only this song.
    Unknown,
}

impl AlbumSource {
    pub fn as_str(self) -> &'static str {
        match self {
            AlbumSource::Tag => "tag",
            AlbumSource::Folder => "folder",
            AlbumSource::Unknown => "unknown",
        }
    }
    pub fn parse(s: &str) -> AlbumSource {
        match s {
            "folder" => AlbumSource::Folder,
            "unknown" => AlbumSource::Unknown,
            _ => AlbumSource::Tag,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub genre: String,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub duration_ms: u64,
    pub album_source: AlbumSource,
    /// Unix time the track first entered the library (0 = assigned on insert).
    pub added_at: i64,
}

#[derive(Default)]
pub struct Library {
    pub tracks: Vec<Track>,
    pub folders: Vec<String>,
}

const EXTENSIONS: &[&str] = &["mp3", "flac", "wav"];

/// Recursively scan a folder for supported audio files and read their tags.
/// Returns tracks with id = 0 (the database assigns real ids on reload).
pub fn scan_folder(folder: &Path) -> Vec<Track> {
    let mut out = Vec::new();
    walk(folder, &mut out);
    fill_album_fallbacks(&mut out);
    sort_tracks(&mut out);
    out
}

fn walk(dir: &Path, out: &mut Vec<Track>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else if is_supported(&path) {
            if let Some(t) = metadata::read_track(&path, 0) {
                out.push(t);
            }
        }
    }
}

/// Files without an album tag are grouped by their containing folder when that
/// folder holds several songs (the usual "one folder per album" layout). A
/// folder with a single song would otherwise create a fake album per track, so
/// those land in one "未知专辑" bucket instead.
pub fn fill_album_fallbacks(tracks: &mut [Track]) {
    let mut per_folder: HashMap<PathBuf, usize> = HashMap::new();
    for t in tracks.iter() {
        if let Some(parent) = Path::new(&t.path).parent() {
            *per_folder.entry(parent.to_path_buf()).or_default() += 1;
        }
    }
    for t in tracks.iter_mut() {
        if t.album.trim().is_empty() {
            let parent = Path::new(&t.path).parent().map(|p| p.to_path_buf());
            let count = parent
                .as_ref()
                .and_then(|p| per_folder.get(p).copied())
                .unwrap_or(1);
            let folder_name = parent
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .map(|s| s.trim().to_string());
            match (count >= 2, folder_name) {
                (true, Some(name)) if !name.is_empty() => {
                    t.album = name;
                    t.album_source = AlbumSource::Folder;
                }
                _ => {
                    t.album = "未知专辑".to_string();
                    t.album_source = AlbumSource::Unknown;
                }
            }
        }
    }
}

/// Case/space/punctuation-insensitive form used for comparing album names.
/// Keeps distinct titles distinct; only folds cosmetic differences.
pub fn normalize_key(s: &str) -> String {
    let mut out = String::new();
    let mut last_space = false;
    for ch in s.trim().to_lowercase().chars() {
        let c = match ch {
            '’' | '‘' | '`' | '´' => '\'',
            '“' | '”' => '"',
            '（' => '(',
            '）' => ')',
            '：' => ':',
            '，' => ',',
            '、' => ',',
            '　' => ' ',
            _ => ch,
        };
        if c.is_whitespace() {
            if !last_space {
                out.push(' ');
            }
            last_space = true;
        } else {
            out.push(c);
            last_space = false;
        }
    }
    out
}

/// The album name a track is grouped under. Empty album tags become a single
/// "未知专辑" group rather than one album per song.
pub fn album_name(t: &Track) -> String {
    let album = t.album.trim();
    if album.is_empty() {
        "未知专辑".to_string()
    } else {
        album.to_string()
    }
}

pub const VARIOUS_ARTISTS: &str = "Various Artists";

/// One album as the UI shows it: a name, a performing artist label and the
/// tracks in play order.
#[derive(Clone, Debug)]
pub struct AlbumGroup {
    /// Stable identity for selection; derived from name + artist label.
    pub key: String,
    pub name: String,
    pub artist: String,
    pub is_compilation: bool,
    pub source: AlbumSource,
    /// Track indices (into the slice passed to `group_albums`), play order.
    pub tracks: Vec<usize>,
    pub disc_count: usize,
}

/// Album grouping rules, driven by real metadata:
///
/// * A real album tag groups by album name (ignoring case/space/punctuation)
///   **plus** the album-artist value, so two different artists' same-named
///   albums stay apart.
/// * No album-artist tag: one shared artist labels the album; several artists
///   mean a compilation, labelled "Various Artists" — unless the album name is
///   also a song title, which marks same-named singles (e.g. covers) that must
///   stay separate per artist.
/// * An album guessed from a folder groups by that folder, never mixed with a
///   same-named tag album elsewhere.
/// * Untagged single files share one "未知专辑" group.
pub fn group_albums(tracks: &[Track]) -> Vec<AlbumGroup> {
    use std::collections::BTreeMap;

    // Bucket 1: what the metadata says the album is.
    let mut buckets: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, t) in tracks.iter().enumerate() {
        let base = match t.album_source {
            AlbumSource::Tag => format!("tag:{}", normalize_key(&album_name(t))),
            AlbumSource::Folder => {
                let parent = std::path::Path::new(&t.path)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                format!("folder:{}", normalize_key(&parent))
            }
            AlbumSource::Unknown => "untagged".to_string(),
        };
        buckets.entry(base).or_default().push(i);
    }

    let mut out: Vec<AlbumGroup> = Vec::new();
    for (base, indices) in buckets {
        let source = tracks[indices[0]].album_source;
        let name = album_name(&tracks[indices[0]]);

        let mut album_artists: Vec<String> = Vec::new();
        let mut artists: Vec<String> = Vec::new();
        for &i in &indices {
            let aa = tracks[i].album_artist.trim();
            if !aa.is_empty()
                && !album_artists
                    .iter()
                    .any(|x| normalize_key(x) == normalize_key(aa))
            {
                album_artists.push(aa.to_string());
            }
            let ar = tracks[i].artist.trim();
            if !ar.is_empty()
                && !artists
                    .iter()
                    .any(|x| normalize_key(x) == normalize_key(ar))
            {
                artists.push(ar.to_string());
            }
        }

        // Same-named singles: the album name is also one of the song titles.
        let name_is_a_title = indices
            .iter()
            .any(|&i| normalize_key(&tracks[i].title) == normalize_key(&name));

        let single_label: Option<String> = match source {
            AlbumSource::Folder => Some(if artists.len() == 1 {
                artists[0].clone()
            } else if artists.is_empty() {
                "未知艺术家".to_string()
            } else {
                VARIOUS_ARTISTS.to_string()
            }),
            AlbumSource::Unknown => Some(if artists.len() == 1 {
                artists[0].clone()
            } else if artists.is_empty() {
                "未知艺术家".to_string()
            } else {
                "多位艺术家".to_string()
            }),
            AlbumSource::Tag => {
                if album_artists.len() == 1 {
                    Some(album_artists[0].clone())
                } else if album_artists.len() > 1 {
                    // Explicitly different album artists: two albums that happen
                    // to share a name. Split, never merge into a fake compilation.
                    None
                } else if artists.len() == 1 {
                    Some(artists[0].clone())
                } else if artists.is_empty() {
                    Some("未知艺术家".to_string())
                } else if name_is_a_title {
                    None // same-named singles: split per artist below
                } else {
                    Some(VARIOUS_ARTISTS.to_string())
                }
            }
        };

        match single_label {
            Some(label) => out.push(make_group(&base, name, label, source, indices, tracks)),
            None => {
                // Split same-named singles per performing artist.
                let mut by_artist: BTreeMap<String, (String, Vec<usize>)> = BTreeMap::new();
                for &i in &indices {
                    let label = {
                        let aa = tracks[i].album_artist.trim();
                        if !aa.is_empty() {
                            aa.to_string()
                        } else if !tracks[i].artist.trim().is_empty() {
                            tracks[i].artist.trim().to_string()
                        } else {
                            "未知艺术家".to_string()
                        }
                    };
                    by_artist
                        .entry(normalize_key(&label))
                        .or_insert_with(|| (label, Vec::new()))
                        .1
                        .push(i);
                }
                for (_, (label, idx)) in by_artist {
                    out.push(make_group(&base, name.clone(), label, source, idx, tracks));
                }
            }
        }
    }

    out.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then(a.artist.to_lowercase().cmp(&b.artist.to_lowercase()))
    });
    out
}

fn make_group(
    base: &str,
    name: String,
    artist: String,
    source: AlbumSource,
    mut indices: Vec<usize>,
    tracks: &[Track],
) -> AlbumGroup {
    indices.sort_by_key(|&i| {
        (
            tracks[i].disc_number.unwrap_or(1),
            tracks[i].track_number.unwrap_or(u32::MAX),
            tracks[i].title.to_lowercase(),
        )
    });
    let disc_count = {
        let mut discs: Vec<u32> = indices
            .iter()
            .filter_map(|&i| tracks[i].disc_number)
            .collect();
        discs.sort_unstable();
        discs.dedup();
        discs.len().max(1)
    };
    let is_compilation = artist == VARIOUS_ARTISTS;
    AlbumGroup {
        key: format!("{base}|{}", normalize_key(&artist)),
        name,
        artist,
        is_compilation,
        source,
        tracks: indices,
        disc_count,
    }
}

/// The performer shown for an album: the album-artist tag when present, the
/// single shared artist when all tracks agree, otherwise "多位艺术家".
pub fn album_artist_label(album_artist: &str, artists: &[String]) -> String {
    if !album_artist.trim().is_empty() {
        return album_artist.trim().to_string();
    }
    match artists.len() {
        0 => "未知艺术家".to_string(),
        1 => artists[0].clone(),
        _ => "多位艺术家".to_string(),
    }
}

/// Sortable fields offered by the song list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortKey {
    Title,
    Artist,
    Album,
    Duration,
    AddedAt,
}

/// Sort track indices in place. Kept pure so the ordering rules are testable.
pub fn sort_track_indices(tracks: &[Track], indices: &mut [usize], key: SortKey, ascending: bool) {
    indices.sort_by(|a, b| {
        let (ta, tb) = (&tracks[*a], &tracks[*b]);
        let ord = match key {
            SortKey::Title => ta
                .title
                .to_lowercase()
                .cmp(&tb.title.to_lowercase())
                .then(ta.artist.to_lowercase().cmp(&tb.artist.to_lowercase())),
            SortKey::Artist => ta
                .artist
                .to_lowercase()
                .cmp(&tb.artist.to_lowercase())
                .then(ta.album.to_lowercase().cmp(&tb.album.to_lowercase()))
                .then(ta.disc_number.cmp(&tb.disc_number))
                .then(ta.track_number.cmp(&tb.track_number)),
            SortKey::Album => ta
                .album
                .to_lowercase()
                .cmp(&tb.album.to_lowercase())
                .then(ta.disc_number.cmp(&tb.disc_number))
                .then(ta.track_number.cmp(&tb.track_number)),
            SortKey::Duration => ta.duration_ms.cmp(&tb.duration_ms),
            SortKey::AddedAt => ta.added_at.cmp(&tb.added_at),
        };
        if ascending {
            ord
        } else {
            ord.reverse()
        }
    });
}

pub fn sort_tracks(tracks: &mut [Track]) {
    tracks.sort_by(|a, b| {
        a.artist
            .to_lowercase()
            .cmp(&b.artist.to_lowercase())
            .then(a.album.to_lowercase().cmp(&b.album.to_lowercase()))
            .then(a.disc_number.cmp(&b.disc_number))
            .then(a.track_number.cmp(&b.track_number))
            .then(a.title.to_lowercase().cmp(&b.title.to_lowercase()))
    });
}

pub fn is_supported(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn track(path: &str, album: &str, artist: &str) -> Track {
        Track {
            id: 0,
            path: path.into(),
            title: "t".into(),
            artist: artist.into(),
            album: album.into(),
            album_artist: String::new(),
            genre: String::new(),
            year: None,
            track_number: None,
            disc_number: None,
            duration_ms: 1000,
            album_source: if album.is_empty() {
                AlbumSource::Unknown
            } else {
                AlbumSource::Tag
            },
            added_at: 0,
        }
    }

    fn titled(mut t: Track, title: &str) -> Track {
        t.title = title.into();
        t
    }

    #[test]
    fn album_artist_label_prefers_the_tag_then_the_shared_artist() {
        assert_eq!(
            album_artist_label("Band", &["A".into(), "B".into()]),
            "Band"
        );
        assert_eq!(album_artist_label("", &["A".into()]), "A");
        assert_eq!(
            album_artist_label("", &["A".into(), "B".into()]),
            "多位艺术家"
        );
        assert_eq!(album_artist_label("", &[]), "未知艺术家");
    }

    #[test]
    fn untagged_albums_use_folder_only_when_it_holds_several_tracks() {
        let mut tracks = vec![
            track("D:\\A\\01.wav", "", "X"),
            track("D:\\A\\02.wav", "", "X"),
            track("D:\\Singles\\only.wav", "", "Y"),
            track("D:\\Tagged\\03.wav", "Real Album", "Z"),
        ];
        fill_album_fallbacks(&mut tracks);
        assert_eq!(tracks[0].album, "A");
        assert_eq!(tracks[0].album_source, AlbumSource::Folder);
        assert_eq!(tracks[1].album, "A");
        assert_eq!(tracks[2].album, "未知专辑");
        assert_eq!(tracks[2].album_source, AlbumSource::Unknown);
        assert_eq!(tracks[3].album, "Real Album");
        assert_eq!(tracks[3].album_source, AlbumSource::Tag);
    }

    #[test]
    fn a_compilation_becomes_one_various_artists_album() {
        let tracks = vec![
            titled(
                track("D:\\OST\\1.flac", "Some Soundtrack", "Singer A"),
                "Opening",
            ),
            titled(
                track("D:\\OST\\2.flac", "Some Soundtrack", "Singer B"),
                "Theme",
            ),
            titled(
                track("D:\\OST\\3.flac", "Some Soundtrack", "Singer C"),
                "Ending",
            ),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].artist, VARIOUS_ARTISTS);
        assert!(groups[0].is_compilation);
        assert_eq!(groups[0].tracks.len(), 3);
    }

    #[test]
    fn same_named_singles_stay_separate_per_artist() {
        // The album name is also the song title: two covers, not one album.
        let tracks = vec![
            titled(track("D:\\1.flac", "牵丝戏", "银临"), "牵丝戏"),
            titled(track("D:\\2.flac", "牵丝戏", "黄龄"), "牵丝戏 (梦幻西游版)"),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 2, "{groups:?}");
    }

    #[test]
    fn same_album_name_with_different_album_artists_stays_apart() {
        let mut a = titled(track("D:\\1.flac", "Greatest Hits", "Band A"), "One");
        a.album_artist = "Band A".into();
        let mut b = titled(track("D:\\2.flac", "Greatest Hits", "Band B"), "Two");
        b.album_artist = "Band B".into();
        let groups = group_albums(&[a, b]);
        assert_eq!(groups.len(), 2, "{groups:?}");
    }

    #[test]
    fn album_artist_tag_keeps_a_compilation_together() {
        let mut a = titled(track("D:\\1.flac", "Comp", "Singer A"), "One");
        a.album_artist = "Various Artists".into();
        let mut b = titled(track("D:\\2.flac", "Comp", "Singer B"), "Two");
        b.album_artist = "Various Artists".into();
        let groups = group_albums(&[a, b]);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert!(groups[0].is_compilation);
    }

    #[test]
    fn cosmetic_album_differences_do_not_split_an_album() {
        let tracks = vec![
            titled(track("D:\\1.flac", "LOVE  Story", "Band"), "One"),
            titled(track("D:\\2.flac", " love story", "Band"), "Two"),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].tracks.len(), 2);
    }

    #[test]
    fn folder_albums_do_not_merge_with_same_named_tag_albums() {
        let mut folder_track = titled(track("D:\\Music\\Holiday\\01.flac", "", "X"), "One");
        folder_track.album = "Holiday".into();
        folder_track.album_source = AlbumSource::Folder;
        let tag_track = titled(track("D:\\Other\\2.flac", "Holiday", "Y"), "Two");
        let groups = group_albums(&[folder_track, tag_track]);
        assert_eq!(groups.len(), 2, "{groups:?}");
    }

    #[test]
    fn untagged_single_files_share_one_unknown_album_group() {
        let tracks = vec![
            titled(track("D:\\S1\\a.wav", "", "A"), "One"),
            titled(track("D:\\S2\\b.wav", "", "B"), "Two"),
        ];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].name, "未知专辑");
    }

    #[test]
    fn sorting_orders_by_each_field_in_both_directions() {
        let mut a = titled(track("D:\\1.flac", "A", "Beta"), "Zebra");
        a.duration_ms = 3000;
        a.added_at = 20;
        let mut b = titled(track("D:\\2.flac", "A", "Alpha"), "apple");
        b.duration_ms = 1000;
        b.added_at = 10;
        let mut c = titled(track("D:\\3.flac", "A", "Gamma"), "Mango");
        c.duration_ms = 2000;
        c.added_at = 30;
        let tracks = vec![a, b, c];

        let order = |key, asc| {
            let mut idx: Vec<usize> = (0..tracks.len()).collect();
            sort_track_indices(&tracks, &mut idx, key, asc);
            idx
        };
        assert_eq!(order(SortKey::Title, true), vec![1, 2, 0]); // apple, Mango, Zebra
        assert_eq!(order(SortKey::Title, false), vec![0, 2, 1]);
        assert_eq!(order(SortKey::Artist, true), vec![1, 0, 2]); // Alpha, Beta, Gamma
        assert_eq!(order(SortKey::Duration, true), vec![1, 2, 0]);
        assert_eq!(order(SortKey::Duration, false), vec![0, 2, 1]);
        assert_eq!(order(SortKey::AddedAt, true), vec![1, 0, 2]);
        assert_eq!(order(SortKey::AddedAt, false), vec![2, 0, 1]);
        assert_eq!(order(SortKey::Album, true), vec![0, 1, 2]); // same album: stable
    }

    #[test]
    fn multi_disc_albums_order_by_disc_then_track() {
        let mut t1 = titled(track("D:\\1.flac", "Big", "Band"), "Disc2 Track1");
        t1.disc_number = Some(2);
        t1.track_number = Some(1);
        let mut t2 = titled(track("D:\\2.flac", "Big", "Band"), "Disc1 Track2");
        t2.disc_number = Some(1);
        t2.track_number = Some(2);
        let mut t3 = titled(track("D:\\3.flac", "Big", "Band"), "Disc1 Track1");
        t3.disc_number = Some(1);
        t3.track_number = Some(1);
        let tracks = vec![t1, t2, t3];
        let groups = group_albums(&tracks);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].disc_count, 2);
        let order: Vec<&str> = groups[0]
            .tracks
            .iter()
            .map(|&i| tracks[i].title.as_str())
            .collect();
        assert_eq!(order, vec!["Disc1 Track1", "Disc1 Track2", "Disc2 Track1"]);
    }
}
