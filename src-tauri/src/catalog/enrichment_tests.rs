use super::*;

#[test]
fn ignored_migration_preserves_old_cache_and_setting_is_persistent_and_validated() {
    let f = Fixture::new();
    let conn = f.catalog.connection().unwrap();
    conn.execute_batch("DROP TABLE enrichment_cache;
        CREATE TABLE enrichment_cache(track_id INTEGER PRIMARY KEY,query_key TEXT NOT NULL,payload_json TEXT NOT NULL,fetched_at INTEGER NOT NULL);").unwrap();
    conn.execute(
        "INSERT INTO enrichment_cache VALUES(?1,'old-key','old-payload',123)",
        [f.id],
    )
    .unwrap();
    let migrated = Catalog::new(f.catalog.path.clone());
    let db = migrated.connection().unwrap();
    let old: (String, i64) = db
        .query_row(
            "SELECT payload_json,ignored FROM enrichment_cache WHERE track_id=?1",
            [f.id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(old, ("old-payload".into(), 0));
    set_ignored(&migrated, f.id, true).unwrap();
    assert!(
        track_statuses(&Catalog::new(f.catalog.path.clone()), &[f.id]).unwrap()[0]
            .enrichment_ignored
    );
    set_ignored(&migrated, f.id, false).unwrap();
    assert!(!track_statuses(&migrated, &[f.id]).unwrap()[0].enrichment_ignored);
    assert_eq!(
        db.query_row(
            "SELECT payload_json FROM enrichment_cache WHERE track_id=?1",
            [f.id],
            |r| r.get::<_, String>(0)
        )
        .unwrap(),
        "old-payload"
    );
    assert_eq!(
        set_ignored(&migrated, 0, true).unwrap_err(),
        "METADATA_IDS_INVALID"
    );
    assert_eq!(
        set_ignored(&migrated, 999, true).unwrap_err(),
        "METADATA_TRACK_MISSING"
    );
    assert_eq!(
        db.query_row("SELECT COUNT(*) FROM enrichment_cache", [], |r| r
            .get::<_, i64>(0))
            .unwrap(),
        1
    );
}

#[test]
fn ignored_marker_survives_candidate_refresh_and_successful_apply_until_explicit_restore() {
    let f = Fixture::new();
    set_ignored(&f.catalog, f.id, true).unwrap();
    for force in [false, true] {
        search_cached(&f.catalog, f.id, None, None, force, |_| {
            Ok(cached_result("Fresh"))
        })
        .unwrap();
        assert!(track_statuses(&f.catalog, &[f.id]).unwrap()[0].enrichment_ignored);
    }
    apply_prepared(&f.catalog, f.request(), None).unwrap();
    assert!(track_statuses(&f.catalog, &[f.id]).unwrap()[0].enrichment_ignored);
    let mut fetched = false;
    search_cached(&f.catalog, f.id, None, None, false, |_| {
        fetched = true;
        Ok(cached_result("After apply"))
    })
    .unwrap();
    assert!(
        fetched,
        "apply must invalidate candidates while retaining ignored"
    );
    set_ignored(&f.catalog, f.id, false).unwrap();
    assert!(!track_statuses(&f.catalog, &[f.id]).unwrap()[0].enrichment_ignored);
}

#[test]
fn library_query_filters_ignored_and_orders_newest_first_with_stable_pagination() {
    use super::super::Query;
    let f = Fixture::new();
    let conn = f.catalog.connection().unwrap();
    for name in ["new-a.flac", "new-b.flac"] {
        let mut track = fixture_track(&f.root.join(name));
        track.added_at = 500;
        super::super::db::upsert_tracks(&conn, &[track]).unwrap();
    }
    let page = f.catalog.query(Query::default()).unwrap();
    assert_eq!(
        page.tracks.iter().map(|t| t.added_at).collect::<Vec<_>>(),
        vec![500, 500, 123]
    );
    assert!(page.tracks[0].id > page.tracks[1].id);
    let ignored = page.tracks[0].id;
    set_ignored(&f.catalog, ignored, true).unwrap();
    let pending = f
        .catalog
        .query(Query {
            enrichment_ignored: Some(false),
            limit: 1,
            offset: 1,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(pending.total, 2);
    assert_eq!(pending.tracks[0].id, f.id);
    let ignored_page = f
        .catalog
        .query(Query {
            enrichment_ignored: Some(true),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(ignored_page.total, 1);
    assert_eq!(ignored_page.tracks[0].id, ignored);
    assert_eq!(f.catalog.query(Query::default()).unwrap().total, 3);
}

fn cached_result(title: &str) -> EnrichmentCandidates {
    EnrichmentCandidates { title_query: "Song".into(), artist_query: "Singer".into(), tags: parse_itunes(&serde_json::json!({"results":[{"trackName":title,"artistName":"Singer","collectionName":"Album"}]})).unwrap(), lyrics: vec![], errors: vec![] }
}

#[test]
fn candidate_cache_hits_without_fetch_and_query_change_or_force_refresh_replaces_it() {
    let f = Fixture::new();
    let calls = std::cell::Cell::new(0);
    let fetch = |_: &[(String, String)]| {
        calls.set(calls.get() + 1);
        Ok(cached_result("First"))
    };
    search_cached(
        &f.catalog,
        f.id,
        Some("Song".into()),
        Some("Singer".into()),
        false,
        fetch,
    )
    .unwrap();
    let reopened = Catalog::new(f.catalog.path.clone());
    let hit = search_cached(
        &reopened,
        f.id,
        Some(" SONG ".into()),
        Some(" singer ".into()),
        false,
        |_| panic!("cache hit must not fetch"),
    )
    .unwrap();
    assert_eq!(hit.tags[0].title, "First");
    assert_eq!(calls.get(), 1);
    let changed = search_cached(
        &f.catalog,
        f.id,
        Some("Different".into()),
        Some("Singer".into()),
        false,
        |_| {
            calls.set(calls.get() + 1);
            Ok(cached_result("Changed"))
        },
    )
    .unwrap();
    assert_eq!(changed.tags[0].title, "Changed");
    let forced = search_cached(
        &f.catalog,
        f.id,
        Some("Different".into()),
        Some("Singer".into()),
        true,
        |_| {
            calls.set(calls.get() + 1);
            Ok(cached_result("Forced"))
        },
    )
    .unwrap();
    assert_eq!(forced.tags[0].title, "Forced");
    assert_eq!(calls.get(), 3);
    assert_eq!(
        f.catalog
            .connection()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM enrichment_cache", [], |row| row
                .get::<_, i64>(0))
            .unwrap(),
        1
    );
}

#[test]
fn expired_future_and_corrupt_candidate_cache_are_refetched() {
    let f = Fixture::new();
    let calls = std::cell::Cell::new(0);
    let query = || {
        search_cached(&f.catalog, f.id, None, None, false, |_| {
            calls.set(calls.get() + 1);
            Ok(cached_result("Fresh"))
        })
    };
    query().unwrap();
    let conn = f.catalog.connection().unwrap();
    for timestamp in [
        super::super::db::now_secs() - 7 * 24 * 60 * 60,
        super::super::db::now_secs() + 3600,
    ] {
        conn.execute("UPDATE enrichment_cache SET fetched_at=?1", [timestamp])
            .unwrap();
        query().unwrap();
    }
    conn.execute("UPDATE enrichment_cache SET payload_json='broken json'", [])
        .unwrap();
    query().unwrap();
    assert_eq!(calls.get(), 4);
}

#[test]
fn successful_apply_and_folder_removal_clear_candidate_cache_without_deleting_files() {
    let f = Fixture::new();
    search_cached(&f.catalog, f.id, None, None, false, |_| {
        Ok(cached_result("Cached"))
    })
    .unwrap();
    apply_prepared(&f.catalog, f.request(), None).unwrap();
    let conn = f.catalog.connection().unwrap();
    assert_eq!(
        conn.query_row("SELECT COUNT(*) FROM enrichment_cache", [], |r| r
            .get::<_, i64>(0))
            .unwrap(),
        0
    );
    search_cached(&f.catalog, f.id, None, None, false, |_| {
        Ok(cached_result("Cached"))
    })
    .unwrap();
    conn.execute(
        "INSERT INTO enrichment_cache(track_id,query_key,payload_json,fetched_at) VALUES(999,'orphan','{}',1)",
        [],
    )
    .unwrap();
    super::super::db::add_folder(&conn, f.root.to_str().unwrap()).unwrap();
    let before = fs::read(&f.audio).unwrap();
    super::super::db::remove_folder(&conn, f.root.to_str().unwrap()).unwrap();
    assert_eq!(
        conn.query_row("SELECT COUNT(*) FROM enrichment_cache", [], |r| r
            .get::<_, i64>(0))
            .unwrap(),
        0
    );
    assert_eq!(fs::read(&f.audio).unwrap(), before);
}

#[test]
fn migration_adds_cache_to_existing_library_and_late_fetch_cannot_reinsert_a_removed_track() {
    let f = Fixture::new();
    let conn = f.catalog.connection().unwrap();
    conn.execute("DROP TABLE enrichment_cache", []).unwrap();
    let reopened = Catalog::new(f.catalog.path.clone());
    assert_eq!(reopened.track(f.id).unwrap().title, "Old title");
    assert_eq!(
        reopened
            .connection()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM enrichment_cache", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        0
    );
    let error = search_cached(&reopened, f.id, None, None, false, |_| {
        assert!(
            reopened.operation.try_lock().is_ok(),
            "fetch must not hold the catalog operation lock"
        );
        reopened
            .connection()
            .unwrap()
            .execute("DELETE FROM tracks WHERE id=?1", [f.id])
            .unwrap();
        Ok(cached_result("Late"))
    })
    .err()
    .unwrap();
    assert_eq!(error, "METADATA_TRACK_MISSING");
    assert_eq!(
        reopened
            .connection()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM enrichment_cache", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        0
    );
}

#[test]
fn itunes_json_maps_tags_genre_year_and_restricted_600px_artwork() {
    let input = serde_json::json!({"results":[{"kind":"song","trackName":"无情画","artistName":"王唯旖","collectionName":"无情画 - Single","collectionArtistName":"王唯旖","releaseDate":"2023-03-04T00:00:00Z","trackNumber":1,"primaryGenreName":"Mandopop","artworkUrl100":"https://is1-ssl.mzstatic.com/image/thumb/Music/a/b/c/100x100bb.jpg"}]});
    let tags = parse_itunes(&input).unwrap();
    assert_eq!(tags[0].source, CandidateSource::ITunes);
    assert_eq!(tags[0].year, Some(2023));
    assert_eq!(tags[0].genre.as_deref(), Some("Mandopop"));
    assert!(tags[0]
        .cover_url
        .as_ref()
        .unwrap()
        .ends_with("/600x600bb.jpg"));
    assert_eq!(tags[0].cover_preview_url, tags[0].cover_url);
    assert!(parse_itunes(&serde_json::json!({})).is_err());
    for url in [
        "https://evil.test/a/100x100bb.jpg",
        "https://mzstatic.com.evil.test/image/thumb/a/100x100bb.jpg",
        "https://user@is1-ssl.mzstatic.com/image/thumb/a/100x100bb.jpg",
        "https://is1-ssl.mzstatic.com/image/thumb/a/100x100bb.jpg?secret=x",
    ] {
        assert!(itunes_artwork(url).is_none(), "{url}");
    }
}

#[test]
fn apple_cover_url_is_validated_before_download_and_genre_writes_preserve_audio_by_default() {
    let f = Fixture::new();
    let before = fs::read(&f.audio).unwrap();
    let mut p = f.request();
    p.save_cover = true;
    p.cover_url = Some("https://evil.test/image/thumb/a/600x600bb.jpg".into());
    assert_eq!(apply(&f.catalog, p).unwrap_err(), "METADATA_COVER_URL");
    assert_eq!(fs::read(&f.audio).unwrap(), before);
    let mut p = f.request();
    p.save_cover = true;
    p.dedicated_cover = true;
    p.cover_url = Some("https://is1-ssl.mzstatic.com/image/thumb/a/600x600bb.jpg".into());
    p.tags.as_mut().unwrap().genre = Some("Mandopop".into());
    let track = apply_prepared(&f.catalog, p, Some(png())).unwrap();
    assert_eq!(track.genre, "Mandopop");
    assert_eq!(f.catalog.track(f.id).unwrap().genre, "Mandopop");
    assert_eq!(fs::read(&f.audio).unwrap(), before);
    assert!(f.audio.with_extension("cover.jpg").exists());
}

#[test]
fn bracket_suffixes_retry_only_after_all_original_variants_are_empty() {
    for (raw, expected) in [
        ("Stay With Me (纯音乐版)", "Stay With Me"),
        ("歌曲（完整收录篇）", "歌曲"),
        ("Song [0.8x]", "Song"),
        ("Song (Live) [0.8x]", "Song"),
        ("Song (Live (2024))", "Song"),
        ("(Only title)", "(Only title)"),
        ("A (middle) B", "A (middle) B"),
        ("Song (unclosed", "Song (unclosed"),
    ] {
        assert_eq!(strip_title_suffix(raw), expected);
    }
    let variants = vec![("Stay (纯音乐版)".into(), "Artist".into())];
    let mut calls = Vec::new();
    let result = search_variants(&variants, |title, artist| {
        calls.push((title.to_owned(), artist.to_owned()));
        Ok(if title == "Stay" { vec![1] } else { vec![] })
    })
    .unwrap();
    assert_eq!(calls.len(), 2);
    assert_eq!(result.query, Some(("Stay".into(), "Artist".into())));
    let mut calls = 0;
    search_variants(&variants, |_, _| {
        calls += 1;
        Ok(vec![1])
    })
    .unwrap();
    assert_eq!(calls, 1);
}

#[test]
fn provider_failures_merge_independently_and_lrclib_backfills_when_tag_sources_are_empty() {
    let itunes = parse_itunes(&serde_json::json!({"results":[{"trackName":"Song","artistName":"Singer","collectionName":"Album"}]})).unwrap();
    let lyrics = parse_lyrics(&serde_json::json!([{"id":7,"trackName":"无情画","artistName":"王唯旖","albumName":"无情画 - Single","syncedLyrics":"[00:01.00]歌词"}])).unwrap();
    let result = merge_sources(
        Err("METADATA_MB_NETWORK".into()),
        Ok(VariantResults {
            values: itunes.clone(),
            query: None,
        }),
        Ok(VariantResults {
            values: lyrics,
            query: None,
        }),
        ("query".into(), "artist".into()),
    );
    assert_eq!(result.tags.len(), 1);
    assert!(result.errors.is_empty());
    let lyrics = parse_lyrics(&serde_json::json!([{"id":7,"trackName":"无情画","artistName":"王唯旖","albumName":"无情画 - Single","syncedLyrics":"[00:01.00]歌词"}])).unwrap();
    let result = merge_sources(
        Err("METADATA_MB_NETWORK".into()),
        Err("METADATA_ITUNES_NETWORK".into()),
        Ok(VariantResults {
            values: lyrics,
            query: None,
        }),
        ("query".into(), "artist".into()),
    );
    assert_eq!(result.tags[0].source, CandidateSource::Lrclib);
    assert_eq!(result.tags[0].album, "无情画 - Single");
    assert!(result.errors.is_empty());
    let result = merge_sources(
        Err("METADATA_MB_NETWORK".into()),
        Err("METADATA_ITUNES_NETWORK".into()),
        Ok(VariantResults {
            values: vec![],
            query: None,
        }),
        ("q".into(), "a".into()),
    );
    assert_eq!(
        result.errors,
        vec!["METADATA_MB_NETWORK", "METADATA_ITUNES_NETWORK"]
    );
    let result = merge_sources(
        Ok(VariantResults {
            values: itunes.clone(),
            query: None,
        }),
        Ok(VariantResults {
            values: itunes,
            query: None,
        }),
        Err("METADATA_LRC_NETWORK".into()),
        ("q".into(), "a".into()),
    );
    assert_eq!(result.tags.len(), 1);
    assert!(result.errors.is_empty());
}

#[test]
fn missing_artist_splits_both_orders_and_three_dash_forms_without_overriding_manual_artist() {
    let mut t = fixture_track(Path::new("fallback.flac"));
    t.artist = "未知艺术家".into();
    for title in ["无情画-王唯旖", "无情画－王唯旖", "无情画—王唯旖"] {
        let variants = query_variants(&t, Some(title.into()), Some("未知艺术家".into())).unwrap();
        assert_eq!(
            variants,
            vec![
                ("无情画".into(), "王唯旖".into()),
                ("王唯旖".into(), "无情画".into())
            ]
        );
    }
    assert_eq!(
        query_variants(&t, Some("周杰伦 - 晴天".into()), None).unwrap()[0],
        ("晴天".into(), "周杰伦".into())
    );
    assert_eq!(
        query_variants(&t, Some("Song-Remix".into()), Some("Manual artist".into())).unwrap(),
        vec![("Song-Remix".into(), "Manual artist".into())]
    );
    t.path = "01 无情画-王唯旖.flac".into();
    t.title = "无情画-王唯旖".into();
    assert_eq!(
        query_variants(&t, None, None).unwrap()[0],
        ("无情画".into(), "王唯旖".into())
    );
    t.artist = "".into();
    assert_eq!(
        query_variants(&t, Some("No dash".into()), Some("".into())).unwrap(),
        vec![("No dash".into(), "".into())]
    );
}

#[test]
fn query_orientation_falls_back_only_on_empty_results_and_checks_filename() {
    let mut t = fixture_track(Path::new("01 无情画-王唯旖.flac"));
    t.artist = String::new();
    let variants = query_variants(&t, None, None).unwrap();
    assert_eq!(variants[0], ("无情画".into(), "王唯旖".into()));
    let mut calls = Vec::new();
    let result = search_variants(&variants, |title, artist| {
        calls.push((title.to_owned(), artist.to_owned()));
        Ok(if calls.len() == 2 {
            vec!["reverse match"]
        } else {
            Vec::new()
        })
    })
    .unwrap();
    assert_eq!(calls, variants);
    assert_eq!(result.values, vec!["reverse match"]);
    assert_eq!(result.query, Some(variants[1].clone()));
    let mut calls = 0;
    let result = search_variants(&variants, |_, _| {
        calls += 1;
        Ok(vec!["primary match"])
    })
    .unwrap();
    assert_eq!(calls, 1);
    assert_eq!(result.values, vec!["primary match"]);
    assert_eq!(result.query, Some(variants[0].clone()));
    t.path = "01 - 周杰伦 - 晴天.flac".into();
    t.title = String::new();
    assert_eq!(
        query_variants(&t, None, None).unwrap()[0],
        ("晴天".into(), "周杰伦".into())
    );
}

#[test]
fn completeness_reads_local_tags_images_and_sidecars_without_network_or_writes() {
    let f = Fixture::new();
    let before = fs::read(&f.audio).unwrap();
    f.catalog.connection().unwrap().execute("UPDATE tracks SET artist='未知艺术家',album='未知专辑',album_source='unknown' WHERE id=?1", [f.id]).unwrap();
    let status = track_statuses(&f.catalog, &[f.id]).unwrap().remove(0);
    assert!(status.missing_artist && status.missing_album);
    assert_eq!(status.missing_cover, Some(true));
    assert_eq!(status.missing_lyrics, Some(true));
    fs::write(f.audio.with_extension("lrc"), "[00:01.00]Local").unwrap();
    fs::write(f.audio.with_extension("cover.jpg"), png()).unwrap();
    let status = track_statuses(&f.catalog, &[f.id]).unwrap().remove(0);
    assert_eq!(status.missing_cover, Some(false));
    assert_eq!(status.missing_lyrics, Some(false));
    assert_eq!(fs::read(&f.audio).unwrap(), before);
    assert!(track_statuses(&f.catalog, &vec![f.id; 201]).is_err());
}

#[test]
fn dedicated_covers_do_not_change_a_neighbors_cover_or_overwrite_shared_cover() {
    let f = Fixture::new();
    fs::write(f.root.join("cover.jpg"), "keep shared").unwrap();
    let neighbor = f.root.join("neighbor.flac");
    fs::copy(&f.audio, &neighbor).unwrap();
    let store = crate::artwork::covers::Store::new(f.root.join("cache"));
    assert!(store.for_playback(&neighbor, 96).unwrap().is_none());
    let mut p = f.request();
    p.save_cover = true;
    p.dedicated_cover = true;
    p.release_mbid = Some("12345678-1234-1234-1234-123456789abc".into());
    apply_prepared(&f.catalog, p, Some(png())).unwrap();
    assert_eq!(
        fs::read_to_string(f.root.join("cover.jpg")).unwrap(),
        "keep shared"
    );
    assert!(f.audio.with_extension("cover.jpg").exists());
    assert!(store.for_track(f.id, &f.audio, 96).unwrap().is_some());
    assert!(store.for_playback(&neighbor, 96).unwrap().is_none());
}

#[test]
fn filename_keywords_only_fill_missing_tags_and_blank_queries() {
    assert_eq!(
        filename_keywords(Path::new("01 - 银临 - 牵丝戏.flac")),
        ("牵丝戏".into(), "银临".into())
    );
    assert_eq!(
        filename_keywords(Path::new("Solo.mp3")),
        ("Solo".into(), String::new())
    );
    let mut t = fixture_track(Path::new("银临 - 牵丝戏.flac"));
    t.title = "银临 - 牵丝戏".into();
    t.artist = "未知艺术家".into();
    assert_eq!(
        query_terms(&t, Some(" ".into()), None).unwrap(),
        ("牵丝戏".into(), "银临".into())
    );
    assert_eq!(
        query_terms(&t, Some("编辑标题".into()), Some("编辑艺人".into())).unwrap(),
        ("编辑标题".into(), "编辑艺人".into())
    );
    t.title = "本地标题".into();
    t.artist = "本地艺人".into();
    assert_eq!(
        query_terms(&t, None, None).unwrap(),
        ("本地标题".into(), "本地艺人".into())
    );
}

#[test]
fn candidates_parse_service_shapes_and_keep_only_synced_lyrics() {
    let mb = serde_json::json!({"recordings":[{"title":"Song", "artist-credit":[{"name":"Singer","joinphrase":" & "},{"artist":{"name":"Guest"}}], "releases":[{"id":"12345678-1234-1234-1234-123456789abc", "title":"Album","date":"2024-01-01","artist-credit":[{"name":"Band"}],"media":[{"track":[{"number":"03"}]}]}]}]});
    let tags = parse_recordings(&mb).unwrap();
    assert_eq!(tags[0].artist, "Singer & Guest");
    assert_eq!(tags[0].album_artist, "Band");
    assert_eq!(tags[0].year, Some(2024));
    assert_eq!(tags[0].track_number, Some(3));
    assert!(tags[0]
        .cover_preview_url
        .as_ref()
        .unwrap()
        .ends_with("/front-250"));
    let lrc = serde_json::json!([{"id":1,"trackName":"Plain", "syncedLyrics":null},{"id":2,"trackName":"Song","artistName":"Singer","albumName":"Album","syncedLyrics":"[00:01.00]Hello\n[00:02.00]World"},{"id":3,"syncedLyrics":"plain text"}]);
    let lyrics = parse_lyrics(&lrc).unwrap();
    assert_eq!(lyrics.len(), 1);
    assert_eq!(lyrics[0].synced_lyrics, "[00:01.00]Hello\n[00:02.00]World");
    assert!(parse_recordings(&serde_json::json!({})).is_err());
    assert!(parse_lyrics(&serde_json::json!({})).is_err());
}

fn fixture_track(path: &Path) -> Track {
    Track {
        id: 0,
        path: path.to_string_lossy().into_owned(),
        title: "Old title".into(),
        artist: "Old artist".into(),
        album: "Old album".into(),
        album_artist: String::new(),
        genre: "Keep genre".into(),
        year: None,
        track_number: None,
        disc_number: Some(2),
        duration_ms: 3000,
        album_source: AlbumSource::Tag,
        added_at: 123,
    }
}
struct Fixture {
    root: std::path::PathBuf,
    catalog: Catalog,
    id: i64,
    audio: std::path::PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mm-enrichment-{}-{}",
            std::process::id(),
            UNIQUE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        let audio = root.join("fixture.flac");
        std::fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.flac"),
            &audio,
        )
        .unwrap();
        let catalog = Catalog::new(root.join("fixture.sqlite"));
        super::super::db::upsert_tracks(&catalog.connection().unwrap(), &[fixture_track(&audio)])
            .unwrap();
        let id = catalog
            .query(super::super::Query::default())
            .unwrap()
            .tracks[0]
            .id;
        Self {
            root,
            catalog,
            id,
            audio,
        }
    }
    fn request(&self) -> ApplyEnrichmentRequest {
        ApplyEnrichmentRequest {
            track_id: self.id,
            tags: Some(TagCandidate {
                source: CandidateSource::MusicBrainz,
                title: "New title".into(),
                artist: "New artist".into(),
                album: "New album".into(),
                album_artist: "Band".into(),
                year: Some(2025),
                track_number: Some(7),
                release_mbid: None,
                cover_preview_url: None,
                cover_preview: None,
                cover_url: None,
                genre: None,
            }),
            update_library_tags: true,
            synced_lyrics: Some("[00:01.00]New line".into()),
            save_lyrics: true,
            save_cover: false,
            release_mbid: None,
            write_audio_tags: false,
            overwrite_cover: false,
            overwrite_lyrics: false,
            dedicated_cover: false,
            cover_url: None,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        assert!(self.root.starts_with(std::env::temp_dir()));
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn confirmed_apply_updates_sqlite_groups_and_atomic_lrc_without_touching_audio() {
    let f = Fixture::new();
    let before = std::fs::read(&f.audio).unwrap();
    let track = apply_prepared(&f.catalog, f.request(), None).unwrap();
    assert_eq!(track.title, "New title");
    assert_eq!(track.genre, "Keep genre");
    assert_eq!(track.disc_number, Some(2));
    assert_eq!(track.added_at, 123);
    assert_eq!(std::fs::read(&f.audio).unwrap(), before);
    assert_eq!(
        std::fs::read_to_string(f.audio.with_extension("lrc")).unwrap(),
        "[00:01.00]New line"
    );
    assert_eq!(f.catalog.summary().unwrap().artists[0].name, "New artist");
    assert_eq!(f.catalog.summary().unwrap().albums[0].name, "New album");
    assert_eq!(
        std::fs::read_dir(&f.root)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().starts_with(".musicmine-"))
            .count(),
        0
    );
}

#[test]
fn existing_sidecar_is_preserved_until_explicit_overwrite() {
    let f = Fixture::new();
    std::fs::write(f.audio.with_extension("lrc"), "original").unwrap();
    assert_eq!(
        apply_prepared(&f.catalog, f.request(), None).unwrap_err(),
        "METADATA_FILE_EXISTS"
    );
    assert_eq!(f.catalog.track(f.id).unwrap().title, "Old title");
    assert_eq!(
        std::fs::read_to_string(f.audio.with_extension("lrc")).unwrap(),
        "original"
    );
    let mut request = f.request();
    request.overwrite_lyrics = true;
    apply_prepared(&f.catalog, request, None).unwrap();
    assert!(std::fs::read_to_string(f.audio.with_extension("lrc"))
        .unwrap()
        .contains("New line"));
}

#[test]
fn sqlite_failure_rolls_back_published_sidecars_and_audio() {
    let f = Fixture::new();
    f.catalog.connection().unwrap().execute_batch("CREATE TRIGGER reject_enrichment BEFORE UPDATE ON tracks BEGIN SELECT RAISE(ABORT,'fixture rejection'); END;").unwrap();
    let before = std::fs::read(&f.audio).unwrap();
    let mut request = f.request();
    request.write_audio_tags = true;
    assert!(apply_prepared(&f.catalog, request, None).is_err());
    assert_eq!(std::fs::read(&f.audio).unwrap(), before);
    assert!(!f.audio.with_extension("lrc").exists());
    assert_eq!(f.catalog.track(f.id).unwrap().title, "Old title");
}

#[test]
fn explicit_audio_opt_in_writes_tags_only_on_copied_fixture() {
    let f = Fixture::new();
    let mut request = f.request();
    request.write_audio_tags = true;
    apply_prepared(&f.catalog, request, None).unwrap();
    let tags = super::super::metadata::read_track(&f.audio, 0).unwrap();
    assert_eq!(tags.title, "New title");
    assert_eq!(tags.album_artist, "Band");
    assert_eq!(tags.track_number, Some(7));
}

#[test]
fn unchecked_outputs_are_not_written_and_invalid_requests_are_rejected() {
    let f = Fixture::new();
    let mut request = f.request();
    request.save_lyrics = false;
    apply_prepared(&f.catalog, request, None).unwrap();
    assert!(!f.audio.with_extension("lrc").exists());
    let mut request = f.request();
    request.track_id = -1;
    assert!(apply_prepared(&f.catalog, request, None).is_err());
    let mut request = f.request();
    request.synced_lyrics = Some("not lrc".into());
    assert!(apply_prepared(&f.catalog, request, None).is_err());
}

#[test]
fn cover_validation_rejects_oversize_fake_images_and_unsafe_urls() {
    assert!(validate_image(&vec![0; MAX_IMAGE + 1]).is_err());
    assert!(validate_image(b"\xff\xd8not an actual jpeg").is_err());
    for url in [
        "http://coverartarchive.org/release/id/front",
        "https://evil.test/a.jpg",
        "https://coverartarchive.org.evil.test/x",
        "https://user@archive.org/x",
        "https://127.0.0.1/x",
        "https://archive.org:444/x",
        "https://archive.org/x?secret=y",
    ] {
        assert!(
            crate::platform::metadata_http::validate_cover_url(url).is_err(),
            "{url}"
        );
    }
    assert!(crate::platform::metadata_http::validate_cover_url(
        "https://archive.org/download/mbid-123/a.jpg"
    )
    .is_ok());
}

fn png() -> Vec<u8> {
    let image = image::RgbImage::from_pixel(640, 640, image::Rgb([24, 90, 120]));
    let mut bytes = Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

#[test]
fn cover_sidecar_invalidates_negative_cache_without_audio_write_and_embeds_only_with_opt_in() {
    let f = Fixture::new();
    let store = crate::artwork::covers::Store::new(f.root.join("cache"));
    assert!(store.for_track(f.id, &f.audio, 96).unwrap().is_none());
    let before = fs::read(&f.audio).unwrap();
    let mut request = f.request();
    request.save_cover = true;
    request.release_mbid = Some("12345678-1234-1234-1234-123456789abc".into());
    apply_prepared(&f.catalog, request, Some(png())).unwrap();
    let saved = fs::read(f.root.join("cover.jpg")).unwrap();
    assert_eq!(
        image::guess_format(&saved).unwrap(),
        image::ImageFormat::Jpeg
    );
    assert_eq!(validate_image(&saved).unwrap().width(), 640);
    assert_eq!(fs::read(&f.audio).unwrap(), before);
    assert!(store.for_track(f.id, &f.audio, 96).unwrap().is_some());
    let mut request = f.request();
    request.save_cover = true;
    request.release_mbid = Some("12345678-1234-1234-1234-123456789abc".into());
    assert_eq!(
        apply_prepared(&f.catalog, request, Some(png())).unwrap_err(),
        "METADATA_FILE_EXISTS"
    );
    assert_eq!(fs::read(f.root.join("cover.jpg")).unwrap(), saved);
    let mut request = f.request();
    request.save_cover = true;
    request.release_mbid = Some("12345678-1234-1234-1234-123456789abc".into());
    request.overwrite_cover = true;
    request.overwrite_lyrics = true;
    request.write_audio_tags = true;
    apply_prepared(&f.catalog, request, Some(png())).unwrap();
    assert!(super::super::metadata::read_cover(&f.audio).is_some());
}

#[test]
fn publish_new_never_overwrites_files_created_after_preflight() {
    let f = Fixture::new();
    let target = f.audio.with_extension("lrc");
    let mut staged = StagedFile::new(target.clone(), false).unwrap();
    staged.write(b"new").unwrap();
    fs::write(&target, "other program").unwrap();
    assert_eq!(staged.publish().unwrap_err(), "METADATA_FILE_EXISTS");
    assert_eq!(fs::read_to_string(target).unwrap(), "other program");
}

#[test]
fn failed_database_update_restores_preexisting_lyrics_and_cover() {
    let f = Fixture::new();
    fs::write(f.audio.with_extension("lrc"), "original lrc").unwrap();
    fs::write(f.root.join("cover.jpg"), "original cover").unwrap();
    f.catalog.connection().unwrap().execute_batch("CREATE TRIGGER reject_enrichment BEFORE UPDATE ON tracks BEGIN SELECT RAISE(ABORT,'fixture rejection'); END;").unwrap();
    let mut p = f.request();
    p.save_cover = true;
    p.release_mbid = Some("12345678-1234-1234-1234-123456789abc".into());
    p.overwrite_cover = true;
    p.overwrite_lyrics = true;
    assert_eq!(
        apply_prepared(&f.catalog, p, Some(png())).unwrap_err(),
        "METADATA_DATABASE"
    );
    assert_eq!(
        fs::read_to_string(f.audio.with_extension("lrc")).unwrap(),
        "original lrc"
    );
    assert_eq!(
        fs::read_to_string(f.root.join("cover.jpg")).unwrap(),
        "original cover"
    );
}

#[test]
fn invalid_audio_never_publishes_prepared_lyrics_or_changes_original() {
    let f = Fixture::new();
    fs::write(&f.audio, "invalid audio fixture").unwrap();
    let mut p = f.request();
    p.write_audio_tags = true;
    assert_eq!(
        apply_prepared(&f.catalog, p, None).unwrap_err(),
        "METADATA_AUDIO_UNSUPPORTED"
    );
    assert!(!f.audio.with_extension("lrc").exists());
    assert_eq!(
        fs::read_to_string(&f.audio).unwrap(),
        "invalid audio fixture"
    );
    assert_eq!(f.catalog.track(f.id).unwrap().title, "Old title");
}

#[test]
fn candidate_counts_and_malformed_identifiers_are_bounded() {
    let recordings = vec![
        serde_json::json!({"title":"A","releases":[{"id":"../../bad","media":[{"track-offset":u64::MAX}]}]});
        10
    ];
    let tags = parse_recordings(&serde_json::json!({"recordings":recordings})).unwrap();
    assert_eq!(tags.len(), 5);
    assert!(tags[0].release_mbid.is_none());
    assert!(tags[0].cover_preview_url.is_none());
    assert!(tags[0].track_number.is_none());
    let lyrics = vec![serde_json::json!({"syncedLyrics":"[00:01.00]A"}); 10];
    assert_eq!(parse_lyrics(&serde_json::json!(lyrics)).unwrap().len(), 3);
}

#[test]
fn missing_audio_rejects_sidecar_writes_and_an_audio_opt_in_alone_is_not_a_selection() {
    let f = Fixture::new();
    fs::remove_file(&f.audio).unwrap();
    assert!(apply_prepared(&f.catalog, f.request(), None).is_err());
    assert!(!f.audio.with_extension("lrc").exists());
    assert_eq!(f.catalog.track(f.id).unwrap().title, "Old title");
    let mut p = f.request();
    p.update_library_tags = false;
    p.save_lyrics = false;
    p.write_audio_tags = true;
    assert_eq!(
        validate_request(&p).unwrap_err(),
        "METADATA_SELECTION_INVALID"
    );
}
