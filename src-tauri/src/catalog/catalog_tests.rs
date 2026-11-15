use super::*;

#[test]
fn scan_real_formats_search_and_cancel() {
    let dir = std::env::temp_dir().join(format!("mm-catalog-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    state
        .scan(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();
    let page = state.query(Query::default()).unwrap();
    assert_eq!(page.total, 3);
    state
        .scan(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();
    assert_eq!(
        state.query(Query::default()).unwrap().total,
        3,
        "rescans must not duplicate paths"
    );
    assert!(page.tracks.iter().all(|t| t.duration_ms > 2800));
    let query = Query {
        search: "tone".into(),
        limit: 1,
        offset: 1,
        ..Default::default()
    };
    assert_eq!(state.query(query).unwrap().tracks.len(), 1);
    assert_eq!(
        state
            .query(Query {
                search: "%".into(),
                ..Default::default()
            })
            .unwrap()
            .total,
        0
    );
    state.cancel.store(true, Ordering::Relaxed);
    state.scan(PathBuf::from("missing")).unwrap();
    assert!(state.progress.lock().unwrap().cancelled);
    assert_eq!(state.query(Query::default()).unwrap().total, 3);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn wal_backup_preserves_ids_collections_and_settings() {
    let dir = std::env::temp_dir().join(format!("mm-backup-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("old.db");
    let conn = db::open(&source).unwrap();
    conn.execute_batch("PRAGMA journal_mode=WAL;").unwrap();
    let track = metadata::read_track(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.flac"),
        0,
    )
    .unwrap();
    db::upsert_tracks(&conn, &[track]).unwrap();
    let second = metadata::read_track(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.mp3"),
        0,
    )
    .unwrap();
    db::upsert_tracks(&conn, &[second]).unwrap();
    let id = db::load_tracks(&conn).unwrap()[0].id;
    let second_id = db::load_tracks(&conn).unwrap()[1].id;
    collections::set_favorite(&conn, id, true).unwrap();
    let playlist = collections::create_playlist(&conn, "保留歌单").unwrap();
    collections::add_to_playlist(&conn, playlist, id).unwrap();
    collections::add_to_playlist(&conn, playlist, second_id).unwrap();
    collections::move_in_playlist(&conn, playlist, 0, 1).unwrap();
    db::add_folder(&conn, "C:/legacy-music").unwrap();
    db::set_setting(&conn, "session_v2", "{\"queue\":[]}").unwrap();
    let dest = Catalog::new(dir.join("new.db"));
    let report = dest.import_copy(&source).unwrap();
    assert_eq!(
        (report.tracks, report.favorites, report.playlists),
        (2, 1, 1)
    );
    assert!(*dest.initialized.lock().unwrap());
    assert_eq!(dest.summary().unwrap().tracks, 2);
    assert_eq!(dest.initialization_count.load(Ordering::SeqCst), 1);
    let copied = db::open(&dest.path).unwrap();
    assert_eq!(
        collections::playlist_track_ids(&copied, playlist),
        vec![second_id, id]
    );
    assert_eq!(db::load_folders(&copied).unwrap(), vec!["C:/legacy-music"]);
    assert_eq!(
        db::get_setting(&copied, "session_v2"),
        Some("{\"queue\":[]}".into())
    );
    assert!(dest.import_copy(&source).is_err());
    assert_eq!(db::load_tracks(&conn).unwrap().len(), 2);
    drop(copied);
    drop(conn);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn nested_scan_reports_corrupt_files_and_preserves_audio_bytes() {
    let dir = std::env::temp_dir().join(format!("mm-readonly-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("album/nested")).unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tone.wav");
    let expected = std::fs::read(source).unwrap();
    let audio = dir.join("album/nested/01 - Read only.WAV");
    std::fs::write(&audio, &expected).unwrap();
    std::fs::write(dir.join("album/broken.mp3"), b"broken").unwrap();
    std::fs::write(dir.join("notes.txt"), b"ignore").unwrap();
    let catalog = Catalog::new(dir.join("catalog.db"));
    catalog.scan(dir.join("album")).unwrap();
    let page = catalog.query(Query::default()).unwrap();
    assert_eq!(page.total, 1);
    assert_eq!(page.tracks[0].title, "Read only");
    assert_eq!(page.tracks[0].track_number, Some(1));
    assert_eq!(catalog.progress.lock().unwrap().errors.len(), 1);
    assert_eq!(std::fs::read(audio).unwrap(), expected);
    std::fs::remove_dir_all(dir).unwrap();
}
/// The built-in “本地” collection is the unfiltered library query: it follows imports and removals
/// and stays independent of a user playlist that has the same name.
#[test]
fn local_collection_tracks_the_library_and_leaves_user_playlists_alone() {
    let dir = std::env::temp_dir().join(format!("mm-local-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    let local = || {
        state
            .query(Query {
                sort: "added".into(),
                ..Default::default()
            })
            .unwrap()
            .total
    };
    assert_eq!(local(), 0);
    let conn = state.connection().unwrap();
    let user = collections::create_playlist(&conn, "本地").unwrap();
    let other = collections::create_playlist(&conn, "夜晚").unwrap();
    drop(conn);
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    state.scan(folder.clone()).unwrap();
    assert_eq!(local(), 3, "new imports appear without copying rows");
    let conn = state.connection().unwrap();
    let first = db::load_tracks(&conn).unwrap()[0].id;
    collections::add_to_playlist(&conn, other, first).unwrap();
    let in_playlist = |id| {
        state
            .query(Query {
                playlist: Some(id),
                ..Default::default()
            })
            .unwrap()
            .total
    };
    assert_eq!(
        in_playlist(user),
        0,
        "same-name user playlist keeps its own (empty) membership"
    );
    assert_eq!(in_playlist(other), 1);
    let rows: i64 = conn
        .query_row("SELECT COUNT(*) FROM playlist_tracks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(
        rows, 1,
        "no membership rows are written for the built-in collection"
    );
    collections::delete_playlist(&conn, user).unwrap();
    assert_eq!(
        local(),
        3,
        "deleting a playlist never removes library tracks"
    );
    db::remove_folder(&conn, &folder.to_string_lossy()).unwrap();
    drop(conn);
    assert_eq!(local(), 0, "removed library files leave the collection");
    assert_eq!(in_playlist(other), 0);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn listening_history_is_real_bounded_and_persistent() {
    let dir = std::env::temp_dir().join(format!("mm-home-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    state
        .scan(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .unwrap();
    assert!(state.home().unwrap().recent.is_empty());
    state.record_listen("not-a-library-track").unwrap();
    assert!(state.home().unwrap().recent.is_empty());
    let track = state.query(Query::default()).unwrap().tracks.remove(0);
    state.record_listen(&track.path).unwrap();
    state.record_listen(&track.path).unwrap();
    let reopened = Catalog::new(state.path.clone());
    let home = reopened.home().unwrap();
    assert_eq!(home.recent.len(), 1);
    assert_eq!(home.recent[0].id, track.id);
    assert_eq!(home.artists[0], (track.artist, 2));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn same_title_resolves_by_exact_path() {
    let dir = std::env::temp_dir().join(format!("mm-path-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let state = Catalog::new(dir.join("library.db"));
    let conn = state.connection().unwrap();
    for (path, album) in [("C:\\one.flac", "Album A"), ("C:\\two.flac", "Album B")] {
        conn.execute(
            "INSERT INTO tracks (path,title,artist,album,album_artist,genre,duration_ms,album_source,added_at)
             VALUES (?1,'Same Song','Same Artist',?2,'','',1000,'tag',1)",
            [path, album],
        )
        .unwrap();
    }
    drop(conn);
    let first = state.track_by_path("C:\\one.flac").unwrap().unwrap();
    let second = state.track_by_path("C:\\two.flac").unwrap().unwrap();
    assert_ne!(first.id, second.id);
    assert_eq!(first.title, second.title);
    assert_ne!(first.album, second.album);
    assert!(state.track_by_path("C:\\missing.flac").unwrap().is_none());
    assert!(state.track_by_path("").unwrap().is_none());
    let _ = std::fs::remove_dir_all(dir);
}

fn initialization_test_dir(label: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("mm-init-{label}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn concurrent_first_access_initializes_once() {
    let dir = initialization_test_dir("concurrent");
    let catalog = std::sync::Arc::new(Catalog::new(dir.join("library.db")));
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let catalog = catalog.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                assert_eq!(catalog.query(Query::default()).unwrap().total, 0);
                assert_eq!(catalog.summary().unwrap().tracks, 0);
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(catalog.initialization_count.load(Ordering::SeqCst), 1);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hot_connections_preserve_zero_added_at_and_configure_pragmas() {
    let dir = initialization_test_dir("hot");
    let catalog = Catalog::new(dir.join("library.db"));
    let conn = catalog.connection().unwrap();
    conn.execute("INSERT INTO tracks (path,title,artist,album,album_artist,genre,duration_ms,added_at) VALUES ('fixture.flac','Song','Artist','Album','','',1000,0)", []).unwrap();
    let id = conn.last_insert_rowid();
    drop(conn);
    for _ in 0..3 {
        assert_eq!(
            catalog.query(Query::default()).unwrap().tracks[0].added_at,
            0
        );
        assert_eq!(catalog.track(id).unwrap().added_at, 0);
        assert_eq!(catalog.tracks_by_ids(&[id]).unwrap()[0].added_at, 0);
        assert_eq!(catalog.summary().unwrap().tracks, 1);
        catalog.record_listen("fixture.flac").unwrap();
        assert_eq!(catalog.home().unwrap().recent[0].added_at, 0);
        assert_eq!(
            catalog
                .track_by_path("fixture.flac")
                .unwrap()
                .unwrap()
                .added_at,
            0
        );
        assert_eq!(catalog.track_present(id).unwrap().unwrap().added_at, 0);
    }
    assert_eq!(catalog.initialization_count.load(Ordering::SeqCst), 1);
    for conn in [
        catalog.connection().unwrap(),
        db::open_light(&catalog.path).unwrap(),
        db::open_read(&catalog.path).unwrap(),
        db::open(&catalog.path).unwrap(),
    ] {
        assert_eq!(
            conn.query_row("PRAGMA foreign_keys", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            1
        );
        assert_eq!(
            conn.query_row("PRAGMA busy_timeout", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            2000
        );
    }
    assert!(catalog.track(id).unwrap().added_at > 0);
    std::fs::remove_file(&catalog.path).unwrap();
    assert!(catalog.connection().is_err());
    assert!(!catalog.path.exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn initialization_failure_is_retryable_and_poison_is_reported() {
    let dir = initialization_test_dir("retry");
    let parent = dir.join("blocked");
    std::fs::write(&parent, b"not a directory").unwrap();
    let catalog = std::sync::Arc::new(Catalog::new(parent.join("library.db")));
    assert!(catalog.connection().is_err());
    assert!(!*catalog.initialized.lock().unwrap());
    std::fs::remove_file(&parent).unwrap();
    assert_eq!(catalog.summary().unwrap().tracks, 0);
    assert!(*catalog.initialized.lock().unwrap());
    let poisoned = catalog.clone();
    assert!(std::thread::spawn(move || {
        let _guard = poisoned.initialized.lock().unwrap();
        panic!("intentional initialization lock poison");
    })
    .join()
    .is_err());
    let error = catalog.connection().expect_err("poison must fail");
    assert!(error.contains("初始化锁已毒化"));
    std::fs::remove_dir_all(dir).unwrap();
}
