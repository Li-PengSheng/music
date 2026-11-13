use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::Manager;
#[cfg(windows)]
use windows::{
    core::HSTRING,
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, SystemMediaTransportControls,
        SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
    },
    Win32::System::WinRT::ISystemMediaTransportControlsInterop,
};

pub struct Media {
    stopped: AtomicBool,
    status: Mutex<String>,
    #[cfg(windows)]
    controls: Option<(SystemMediaTransportControls, EventRegistrationToken)>,
    last: Mutex<String>,
}
#[tauri::command]
pub fn media_status(app: tauri::AppHandle) -> Result<String, String> {
    app.state::<Media>()
        .status
        .lock()
        .map(|s| s.clone())
        .map_err(|e| e.to_string())
}
pub fn setup(app: &mut tauri::App) {
    #[cfg(windows)]
    let result = (|| -> windows::core::Result<_> {
        let window = app
            .get_webview_window("main")
            .ok_or_else(windows::core::Error::from_win32)?;
        let hwnd = window
            .hwnd()
            .map_err(|_| windows::core::Error::from_win32())?;
        let factory: ISystemMediaTransportControlsInterop = windows::core::factory::<
            SystemMediaTransportControls,
            ISystemMediaTransportControlsInterop,
        >()?;
        // Desktop WinRT interop uses our existing top-level HWND, never a MediaPlayer.
        let controls: SystemMediaTransportControls =
            unsafe { factory.GetForWindow(windows::Win32::Foundation::HWND(hwnd.0))? };
        controls.SetIsPlayEnabled(true)?;
        controls.SetIsPauseEnabled(true)?;
        controls.SetIsNextEnabled(true)?;
        controls.SetIsPreviousEnabled(true)?;
        let handle = app.handle().clone();
        let token = controls.ButtonPressed(&TypedEventHandler::new(
            move |_, args: &Option<SystemMediaTransportControlsButtonPressedEventArgs>| {
                if let Some(args) = args {
                    let action = match args.Button()? {
                        SystemMediaTransportControlsButton::Play => crate::controller::Action::Play,
                        SystemMediaTransportControlsButton::Pause => {
                            crate::controller::Action::Pause
                        }
                        SystemMediaTransportControlsButton::Next => crate::controller::Action::Next,
                        SystemMediaTransportControlsButton::Previous => {
                            crate::controller::Action::Previous
                        }
                        _ => return Ok(()),
                    };
                    let handle = handle.clone();
                    tauri::async_runtime::spawn_blocking(move || {
                        if handle.state::<Media>().stopped.load(Ordering::SeqCst) {
                            return;
                        }
                        let result = handle
                            .state::<crate::Shared>()
                            .lock()
                            .map_err(|e| e.to_string())
                            .and_then(|mut c| c.execute(action));
                        if let Err(e) = result {
                            if let Ok(mut s) = handle.state::<Media>().status.lock() {
                                *s = format!("SMTC 操作失败：{e}");
                            }
                        }
                    });
                }
                Ok(())
            },
        ))?;
        Ok((controls, token))
    })();
    #[cfg(windows)]
    let (controls, status) = match result {
        Ok(c) => (
            Some(c),
            "SMTC 已连接，等待实机媒体键验收；封面暂未接入".to_string(),
        ),
        Err(e) => (None, format!("SMTC 不可用：{e}")),
    };
    #[cfg(not(windows))]
    let status = "仅支持 Windows SMTC".to_string();
    app.manage(Media {
        stopped: AtomicBool::new(false),
        #[cfg(windows)]
        controls,
        status: Mutex::new(status),
        last: Mutex::new(String::new()),
    });
}
pub fn update(app: &tauri::AppHandle, s: &crate::controller::PlayerSnapshot) {
    let state = app.state::<Media>();
    let key = format!(
        "{}:{}:{}",
        s.playback_state,
        serde_json::to_string(&s.current_track).unwrap_or_default(),
        s.queue.len()
    );
    let Ok(mut last) = state.last.lock() else {
        return;
    };
    if state.stopped.load(Ordering::SeqCst) {
        return;
    }
    if *last == key {
        return;
    }
    #[cfg(windows)]
    if let Some((c, _)) = &state.controls {
        let result = (|| -> windows::core::Result<()> {
            c.SetIsEnabled(s.current_track.is_some())?;
            c.SetPlaybackStatus(match s.playback_state {
                "playing" => MediaPlaybackStatus::Playing,
                "paused" => MediaPlaybackStatus::Paused,
                _ => MediaPlaybackStatus::Stopped,
            })?;
            c.SetIsNextEnabled(!s.queue.is_empty())?;
            c.SetIsPreviousEnabled(!s.queue.is_empty())?;
            let display = c.DisplayUpdater()?;
            display.ClearAll()?;
            if let Some(t) = &s.current_track {
                display.SetType(MediaPlaybackType::Music)?;
                let music = display.MusicProperties()?;
                music.SetTitle(&HSTRING::from(&t.title))?;
                music.SetArtist(&HSTRING::from(&t.artist))?;
                music.SetAlbumTitle(&HSTRING::from(&t.album))?;
            }
            display.Update()
        })();
        if let Err(e) = result {
            if let Ok(mut status) = state.status.lock() {
                *status = format!("SMTC 同步失败：{e}");
            }
            return;
        }
    }
    *last = key;
}
pub fn shutdown(app: &tauri::AppHandle) {
    let state = app.state::<Media>();
    let Ok(_last) = state.last.lock() else { return };
    if state.stopped.swap(true, Ordering::SeqCst) {
        return;
    }
    #[cfg(windows)]
    if let Some((c, token)) = &app.state::<Media>().controls {
        if let Err(e) = c
            .RemoveButtonPressed(*token)
            .and_then(|_| c.SetIsEnabled(false))
        {
            eprintln!("关闭 SMTC 失败：{e}");
        }
    }
}
