use music_mine_desktop::player::{analyze_file, PlaybackState, Player};
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn encoded_formats_decode_and_play_with_real_device_controls() {
    let player = Player::new(None);
    player.set_volume(0.0);
    player.synchronize().unwrap();
    for name in ["tone.mp3", "tone.flac", "tone.wav"] {
        let report = analyze_file(&fixture(name), 5.0).unwrap();
        assert!(!report.is_silent(), "{name}: {report:?}");
        assert!(report.peak > 0.35 && report.rms > 0.2);
        assert!((report.seconds - 3.0).abs() < 0.25);
        player.prepare(fixture(name), 3000, 0);
        let ready = player.synchronize().unwrap();
        assert_eq!(ready.state, PlaybackState::Paused, "{ready:?}");
        assert!(ready.error.is_none(), "{ready:?}");
        player.resume();
        player.synchronize().unwrap();
        let deadline = Instant::now() + Duration::from_secs(4);
        while player.status().position < Duration::from_millis(150) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(
            player.status().position >= Duration::from_millis(150),
            "{name}"
        );
        player.pause();
        let paused = player.synchronize().unwrap();
        std::thread::sleep(Duration::from_millis(120));
        assert_eq!(
            player.status().position,
            paused.position,
            "{name}: paused clock moved"
        );
        player.seek(1.0);
        let sought = player.synchronize().unwrap();
        assert!(
            (sought.position.as_secs_f64() - 1.0).abs() < 0.1,
            "{sought:?}"
        );
        player.set_volume(0.25);
        assert_eq!(player.synchronize().unwrap().volume, 0.25);
        player.set_volume(0.0);
        player.resume();
        player.synchronize().unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while player.status().state != PlaybackState::Stopped && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(
            player.status().state,
            PlaybackState::Stopped,
            "{name}: did not end"
        );
        assert!(player.status().error.is_none());
        player.stop();
        let stopped = player.synchronize().unwrap();
        assert_eq!(stopped.position, Duration::ZERO);
    }
}

#[test]
fn damaged_file_reports_failure_without_progress() {
    let path = std::env::temp_dir().join(format!(
        "music-mine-tauri-broken-{}.mp3",
        std::process::id()
    ));
    std::fs::write(&path, b"not an audio file").unwrap();
    let player = Player::new(None);
    player.play(path.clone(), 3000);
    let status = player.synchronize().unwrap();
    assert_eq!(status.state, PlaybackState::Stopped);
    assert!(status.error.is_some());
    assert_eq!(status.position, Duration::ZERO);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn rapid_track_changes_leave_only_the_last_track() {
    let player = Player::new(None);
    player.set_volume(0.0);
    for name in ["tone.mp3", "tone.wav", "tone.flac"] {
        player.prepare(fixture(name), 3000, 0);
    }
    let status = player.synchronize().unwrap();
    assert_eq!(
        status.current_path,
        Some(fixture("tone.flac").to_string_lossy().into_owned())
    );
    assert_eq!(status.state, PlaybackState::Paused);
    assert_eq!(status.position, Duration::ZERO);
}
