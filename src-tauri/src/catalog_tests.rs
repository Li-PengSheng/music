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
