//! Local lyrics: reading (embedded / sidecar `.lrc`), decoding and parsing
//! standard LRC, and the timing logic that maps a playback position to a line.
//!
//! This module has no UI and no player state: it is pure data + text work, so
//! it can be tested on its own. The player's real position is passed in by the
//! caller every frame.

use std::path::{Path, PathBuf};

use lofty::file::TaggedFileExt;
use lofty::tag::ItemKey;

/// One lyric line. `time_ms` is -1 for files without any timestamps.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct LyricLine {
    pub time_ms: i64,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize)]
pub struct Lyrics {
    pub lines: Vec<LyricLine>,
    /// `[offset:+/-ms]` applied to every timestamp.
    pub offset_ms: i64,
    /// True when at least one line carries a LRC timestamp.
    pub synced: bool,
    /// Where the text came from, for the UI ("内嵌歌词" / "song.lrc").
    pub source: String,
}

impl Lyrics {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Index of the line that is playing at `position_ms`, or None before the
    /// first timestamp.
    pub fn index_at(&self, position_ms: i64) -> Option<usize> {
        if !self.synced || self.lines.is_empty() {
            return None;
        }
        // Lines are sorted; find the last one with time <= position.
        let idx = self.lines.partition_point(|l| l.time_ms <= position_ms);
        if idx == 0 {
            None
        } else {
            Some(idx - 1)
        }
    }

    /// The time of the line that should be highlighted, for seek buttons.
    pub fn time_of(&self, index: usize) -> Option<i64> {
        self.lines.get(index).map(|l| l.time_ms)
    }
}

// ------------------------------------------------------------------ parsing

/// Parse LRC text. Accepts `[mm:ss]`, `[mm:ss.xx]`, `[mm:ss.xxx]`, several
/// timestamps on one line, and the `[offset:...]` tag. Lines without any
/// timestamp produce an unsynchronised lyric sheet.
pub fn parse_lrc(text: &str) -> Lyrics {
    let mut lines: Vec<LyricLine> = Vec::new();
    let mut offset_ms: i64 = 0;
    let mut any_stamp = false;

    for raw in text.lines() {
        let line = raw.trim_matches(['\u{feff}', '\r', '\n']).trim();
        if line.is_empty() {
            continue;
        }
        let mut rest = line;
        let mut stamps: Vec<i64> = Vec::new();
        // Consume leading [..] tags: timestamps or metadata.
        while let Some(start) = rest.strip_prefix('[') {
            let Some(end) = start.find(']') else {
                break;
            };
            let tag = &start[..end];
            let after = &start[end + 1..];
            if let Some(ms) = parse_timestamp(tag) {
                stamps.push(ms);
                rest = after;
                continue;
            }
            // Metadata like [ti:...], [offset:+500].
            if let Some((key, value)) = tag.split_once(':') {
                if key.eq_ignore_ascii_case("offset") {
                    let v = value.trim();
                    let sign = if v.starts_with('-') { -1 } else { 1 };
                    let digits: String = v.chars().filter(|c| c.is_ascii_digit()).collect();
                    if let Ok(n) = digits.parse::<i64>() {
                        offset_ms = sign * n;
                    }
                }
            }
            rest = after;
        }
        let text_part = rest.trim().to_string();
        if stamps.is_empty() {
            if !text_part.is_empty() {
                lines.push(LyricLine {
                    time_ms: -1,
                    text: text_part,
                });
            }
            continue;
        }
        any_stamp = true;
        for ms in stamps {
            lines.push(LyricLine {
                time_ms: ms,
                text: text_part.clone(),
            });
        }
    }

    let synced = any_stamp;
    if synced {
        // Apply the offset and sort; several tags on one line mean repeats.
        for l in lines.iter_mut() {
            if l.time_ms >= 0 {
                l.time_ms = l.time_ms.saturating_add(offset_ms);
            }
        }
        lines.retain(|l| l.time_ms >= 0);
        lines.sort_by_key(|l| l.time_ms);
    } else {
        for l in lines.iter_mut() {
            l.time_ms = -1;
        }
    }

    Lyrics {
        lines,
        offset_ms,
        synced,
        source: String::new(),
    }
}

/// `mm:ss`, `mm:ss.xx`, `mm:ss.xxx`, `mm:ss:xx` → milliseconds.
fn parse_timestamp(tag: &str) -> Option<i64> {
    let (minutes, rest) = tag.split_once(':')?;
    let minutes: i64 = minutes.trim().parse().ok()?;
    let (seconds, fraction) = match rest.split_once(['.', ':']) {
        Some((s, f)) => (s, Some(f)),
        None => (rest, None),
    };
    let seconds: i64 = seconds.trim().parse().ok()?;
    if minutes < 0 || !(0..60).contains(&seconds) {
        return None;
    }
    let frac_ms = match fraction {
        None => 0,
        Some(f) => {
            if !f.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            let digits: String = f.chars().take(3).collect();
            let value: i64 = digits.parse().unwrap_or(0);
            match digits.len() {
                0 => 0,
                1 => value * 100,
                2 => value * 10,
                _ => value / 10_i64.pow(digits.len().saturating_sub(3) as u32),
            }
        }
    };
    minutes
        .checked_mul(60_000)?
        .checked_add(seconds * 1000)?
        .checked_add(frac_ms)
}

// ------------------------------------------------------------------ decoding

/// Decode lyric bytes: UTF-8 (with or without BOM), UTF-16 with BOM, UTF-16
/// without BOM when the zero-byte pattern is obvious, else charset detection
/// (GBK/GB18030, Shift_JIS, Big5, …).
pub fn decode_bytes(bytes: &[u8]) -> String {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8_lossy(&bytes[3..]).to_string();
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        let (text, _, _) = encoding_rs::UTF_16LE.decode(&bytes[2..]);
        return text.replace('\u{feff}', "");
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let (text, _, _) = encoding_rs::UTF_16BE.decode(&bytes[2..]);
        return text.replace('\u{feff}', "");
    }
    // UTF-16 without a BOM: ASCII-heavy text has a zero byte every other byte.
    let sample = &bytes[..bytes.len().min(512)];
    let even_zeros = sample.iter().step_by(2).filter(|b| **b == 0).count();
    let odd_zeros = sample
        .iter()
        .skip(1)
        .step_by(2)
        .filter(|b| **b == 0)
        .count();
    let pairs = (sample.len() / 2).max(1);
    if odd_zeros > pairs / 3 && even_zeros < pairs / 10 {
        let (text, _, _) = encoding_rs::UTF_16LE.decode(bytes);
        return text.replace('\u{feff}', "");
    }
    if even_zeros > pairs / 3 && odd_zeros < pairs / 10 {
        let (text, _, _) = encoding_rs::UTF_16BE.decode(bytes);
        return text.replace('\u{feff}', "");
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        return text.to_string();
    }
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);
    let (text, _, _) = encoding.decode(bytes);
    text.to_string()
}

// ------------------------------------------------------------------- loading

const MAX_LYRIC_FILE: u64 = 4 * 1024 * 1024;

/// Load lyrics for a track: a same-name `.lrc` next to the file first (usually
/// the synced one), then the lyrics embedded in the audio tags. Files are only
/// read, never written.
pub fn load_for_track(path: &Path) -> Option<Lyrics> {
    if let Some(sidecar) = sidecar_path(path) {
        if let Ok(meta) = std::fs::metadata(&sidecar) {
            if meta.len() <= MAX_LYRIC_FILE {
                if let Ok(bytes) = std::fs::read(&sidecar) {
                    let text = decode_bytes(&bytes);
                    let mut lyrics = parse_lrc(&text);
                    if !lyrics.is_empty() {
                        lyrics.source = sidecar
                            .file_name()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "lrc".to_string());
                        return Some(lyrics);
                    }
                }
            }
        }
    }
    embedded_lyrics(path)
}

fn sidecar_path(audio: &Path) -> Option<PathBuf> {
    let candidate = audio.with_extension("lrc");
    if candidate.is_file() {
        return Some(candidate);
    }
    // Some collections store an upper-case extension; try it explicitly for
    // case-sensitive filesystems.
    let upper = audio.with_extension("LRC");
    if upper.is_file() {
        return Some(upper);
    }
    None
}

fn embedded_lyrics(path: &Path) -> Option<Lyrics> {
    let tagged = lofty::read_from_path(path).ok()?;
    for tag in tagged.tags() {
        let text = tag.get_string(&ItemKey::Lyrics).map(|s| s.to_string());
        if let Some(text) = text {
            if text.trim().is_empty() {
                continue;
            }
            let mut lyrics = parse_lrc(&text);
            if !lyrics.is_empty() {
                lyrics.source = "内嵌歌词".to_string();
                return Some(lyrics);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_timestamps_and_sorts_them() {
        let lrc = "[00:12.50]Second\n[00:00.00]First\n[00:20.123]Third\n";
        let lyrics = parse_lrc(lrc);
        assert!(lyrics.synced);
        assert_eq!(lyrics.lines.len(), 3);
        assert_eq!(lyrics.lines[0].text, "First");
        assert_eq!(lyrics.lines[0].time_ms, 0);
        assert_eq!(lyrics.lines[1].time_ms, 12_500);
        assert_eq!(lyrics.lines[2].time_ms, 20_123);
    }

    #[test]
    fn one_line_with_several_timestamps_repeats_the_text() {
        let lyrics = parse_lrc("[00:05.00][01:30.00]Chorus\n[00:10.00]Verse\n");
        let times: Vec<i64> = lyrics.lines.iter().map(|l| l.time_ms).collect();
        assert_eq!(times, vec![5_000, 10_000, 90_000]);
        assert_eq!(lyrics.lines[0].text, "Chorus");
        assert_eq!(lyrics.lines[2].text, "Chorus");
    }

    #[test]
    fn offset_tag_shifts_all_timestamps() {
        let lyrics = parse_lrc("[offset:+500]\n[00:01.00]A\n[00:02.00]B\n");
        assert_eq!(lyrics.offset_ms, 500);
        assert_eq!(lyrics.lines[0].time_ms, 1_500);
        assert_eq!(lyrics.lines[1].time_ms, 2_500);

        let earlier = parse_lrc("[offset:-250]\n[00:01.00]A\n");
        assert_eq!(earlier.lines[0].time_ms, 750);
    }

    #[test]
    fn metadata_tags_and_blank_lines_are_ignored() {
        let lrc = "[ti:Song]\n[ar:Artist]\n[al:Album]\n\n[00:01.00]Hello\n";
        let lyrics = parse_lrc(lrc);
        assert_eq!(lyrics.lines.len(), 1);
        assert_eq!(lyrics.lines[0].text, "Hello");
        // "Song" must not become a lyric line.
        assert!(!lyrics.lines.iter().any(|l| l.text == "Song"));
    }

    #[test]
    fn text_without_timestamps_is_unsynchronised() {
        let lyrics = parse_lrc("Just a plain lyric\nSecond line\n");
        assert!(!lyrics.synced);
        assert_eq!(lyrics.lines.len(), 2);
        assert_eq!(lyrics.index_at(10_000), None);
    }

    #[test]
    fn index_at_handles_boundaries_and_empty_positions() {
        let lyrics = parse_lrc("[00:01.00]One\n[00:02.00]Two\n[00:03.00]Three\n");
        assert_eq!(lyrics.index_at(0), None);
        assert_eq!(lyrics.index_at(999), None);
        assert_eq!(lyrics.index_at(1_000), Some(0));
        assert_eq!(lyrics.index_at(1_999), Some(0));
        assert_eq!(lyrics.index_at(2_000), Some(1));
        assert_eq!(lyrics.index_at(99_000), Some(2));
    }

    #[test]
    fn pause_and_seek_are_reproducible() {
        let lyrics = parse_lrc("[00:01.00]One\n[00:05.00]Two\n");
        // A paused position maps to the same line no matter how often asked.
        for _ in 0..3 {
            assert_eq!(lyrics.index_at(5_000), Some(1));
        }
        // Seeking backwards moves the highlight back.
        assert_eq!(lyrics.index_at(1_200), Some(0));
    }

    #[test]
    fn fractional_timestamps_of_various_precision() {
        let lyrics = parse_lrc("[00:00.5]a\n[00:01.05]b\n[00:02.005]c\n[00:03:25]d\n");
        let times: Vec<i64> = lyrics.lines.iter().map(|l| l.time_ms).collect();
        assert_eq!(times, vec![500, 1_050, 2_005, 3_250]);
    }

    #[test]
    fn decodes_utf8_bom_utf16_and_gbk() {
        let text = "[00:01.00]你好，世界\n";
        let mut utf8_bom = vec![0xEF, 0xBB, 0xBF];
        utf8_bom.extend_from_slice(text.as_bytes());
        assert_eq!(decode_bytes(&utf8_bom), text);

        let mut utf16 = vec![0xFF, 0xFE];
        for unit in text.encode_utf16() {
            utf16.extend_from_slice(&unit.to_le_bytes());
        }
        assert_eq!(decode_bytes(&utf16), text);

        let (gbk, _, _) = encoding_rs::GBK.encode(text);
        assert_eq!(decode_bytes(&gbk), text);

        // UTF-16LE without a BOM, detected by the zero-byte pattern.
        let mut raw16 = Vec::new();
        for unit in text.encode_utf16() {
            raw16.extend_from_slice(&unit.to_le_bytes());
        }
        assert_eq!(decode_bytes(&raw16), text);
    }

    #[test]
    fn rejects_overflow_and_decodes_japanese_and_big5() {
        assert!(!parse_lrc("[9223372036854775807:00]bad").synced);
        for (encoding, text) in [
            (
                encoding_rs::SHIFT_JIS,
                "[00:01.00]こんにちは、世界。音楽を聴いています。",
            ),
            (
                encoding_rs::BIG5,
                "[00:01.00]你好世界，這是我們喜歡的音樂。",
            ),
        ] {
            let (bytes, _, _) = encoding.encode(text);
            assert_eq!(decode_bytes(&bytes), text);
        }
    }
    #[test]
    fn garbage_input_does_not_panic() {
        let _ = decode_bytes(&[0, 159, 146, 150, 255, 1, 2, 3]);
        let _ = parse_lrc("");
        let _ = parse_lrc("[]\n[::]\n[99:99.99]x");
    }
}
