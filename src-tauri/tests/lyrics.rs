//! Lyrics loading from disk: sidecar `.lrc`, encodings, and embedded tags.
//! Uses temp copies of the fixtures; no original music is touched.

use std::path::{Path, PathBuf};

use music_mine_desktop::lyrics;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("music-mine-lyrics-{tag}"));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[test]
fn sidecar_lrc_is_loaded_and_synced() {
    let dir = temp_dir("sidecar");
    let audio = dir.join("song.live.flac");
    std::fs::write(&audio, b"not really audio").unwrap(); // sidecar path only
    std::fs::write(
        dir.join("song.live.lrc"),
        "[ti:Demo]\n[offset:+100]\n[00:01.00]Line one\n[00:02.50]Line two\n",
    )
    .unwrap();

    let loaded = lyrics::load_for_track(&audio).expect("sidecar lyrics");
    assert_eq!(loaded.source, "song.live.lrc");
    assert!(loaded.synced);
    assert_eq!(loaded.lines.len(), 2);
    assert_eq!(loaded.lines[0].time_ms, 1_100);
    assert_eq!(loaded.lines[1].text, "Line two");
    assert_eq!(loaded.index_at(2_600), Some(1));
}

#[test]
fn gbk_encoded_sidecar_is_decoded() {
    let dir = temp_dir("gbk");
    let audio = dir.join("cjk.flac");
    std::fs::write(&audio, b"x").unwrap();
    let text = "[00:01.00]你好，世界\n[00:03.00]再见\n";
    let (gbk, _, _) = encoding_rs::GBK.encode(text);
    std::fs::write(dir.join("cjk.lrc"), gbk.as_ref()).unwrap();

    let loaded = lyrics::load_for_track(&audio).expect("gbk lyrics");
    assert_eq!(loaded.lines[0].text, "你好，世界");
    assert_eq!(loaded.lines[1].text, "再见");
}

#[test]
fn embedded_lyrics_are_read_from_tags() {
    let dir = temp_dir("embedded");
    let audio = dir.join("tagged.flac");
    std::fs::copy(fixture("tone.flac"), &audio).unwrap();

    {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        use lofty::tag::ItemKey;
        let mut tagged = lofty::read_from_path(&audio).expect("read copy");
        let tag = tagged.primary_tag_mut().expect("flac has a tag");
        tag.insert_text(
            ItemKey::Lyrics,
            "[00:00.50]Embedded one\n[00:02.00]Embedded two".to_string(),
        );
        tagged
            .save_to_path(&audio, WriteOptions::default())
            .expect("write tag");
    }

    let loaded = lyrics::load_for_track(&audio).expect("embedded lyrics");
    assert_eq!(loaded.source, "内嵌歌词");
    assert!(loaded.synced);
    assert_eq!(loaded.lines.len(), 2);
    assert_eq!(loaded.lines[1].text, "Embedded two");
    assert_eq!(loaded.index_at(2_100), Some(1));
}

#[test]
fn sidecar_wins_over_embedded_and_unsynced_text_still_shows() {
    let dir = temp_dir("priority");
    let audio = dir.join("both.flac");
    std::fs::copy(fixture("tone.flac"), &audio).unwrap();
    {
        use lofty::config::WriteOptions;
        use lofty::file::{AudioFile, TaggedFileExt};
        use lofty::tag::ItemKey;
        let mut tagged = lofty::read_from_path(&audio).unwrap();
        let tag = tagged.primary_tag_mut().unwrap();
        tag.insert_text(ItemKey::Lyrics, "[00:00.20]embedded".to_string());
        tagged
            .save_to_path(&audio, WriteOptions::default())
            .unwrap();
    }
    std::fs::write(dir.join("both.lrc"), "only plain text\nsecond line\n").unwrap();

    let loaded = lyrics::load_for_track(&audio).expect("lyrics");
    assert_eq!(loaded.source, "both.lrc");
    assert!(!loaded.synced, "plain text has no timestamps");
    assert_eq!(loaded.lines.len(), 2);
    assert_eq!(loaded.index_at(1_000), None);
}

#[test]
fn a_track_without_lyrics_returns_none() {
    let dir = temp_dir("none");
    let audio = dir.join("empty.flac");
    std::fs::copy(fixture("tone.flac"), &audio).unwrap();
    let _ = std::fs::remove_file(dir.join("empty.lrc"));
    assert!(lyrics::load_for_track(&audio).is_none());
}

#[test]
fn a_large_lyrics_file_is_still_fast_to_parse() {
    // 6000 timestamped lines: parsing must stay well under a frame budget.
    let mut text = String::new();
    for i in 0..6000 {
        let total_ms = i * 100;
        let minutes = total_ms / 60_000;
        let seconds = (total_ms % 60_000) / 1000;
        let millis = total_ms % 1000;
        text.push_str(&format!(
            "[{minutes:02}:{seconds:02}.{millis:03}]line {i}\n"
        ));
    }
    let start = std::time::Instant::now();
    let parsed = lyrics::parse_lrc(&text);
    let elapsed = start.elapsed();
    assert_eq!(parsed.lines.len(), 6000);
    assert!(
        elapsed < std::time::Duration::from_millis(500),
        "parsing 6000 lines took {elapsed:?}"
    );
    assert_eq!(
        parsed
            .index_at(30_050)
            .map(|i| parsed.lines[i].text.clone()),
        Some("line 300".to_string())
    );
}
