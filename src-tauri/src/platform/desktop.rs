use crate::{playback::controller::Action, Shared};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub close_to_tray: bool,
    pub queue_open: bool,
    /// "dark", "light" or "system"; files written before themes existed read as dark.
    pub theme: String,
    bounds: Option<(i32, i32, u32, u32)>,
    maximized: bool,
}
/// Native title bar theme; `None` lets Windows follow the system setting.
fn window_theme(theme: &str) -> Option<tauri::Theme> {
    match theme {
        "light" => Some(tauri::Theme::Light),
        "system" => None,
        _ => Some(tauri::Theme::Dark),
    }
}
fn window_background(theme: tauri::Theme) -> tauri::window::Color {
    match theme {
        tauri::Theme::Light => tauri::window::Color(0xf3, 0xf5, 0xf1, 0xff),
        _ => tauri::window::Color(0x12, 0x14, 0x14, 0xff),
    }
}
pub struct Desktop {
    preferences: Mutex<Preferences>,
    path: std::path::PathBuf,
    pub exiting: AtomicBool,
}
impl Desktop {
    fn save(&self, value: &Preferences) -> Result<(), String> {
        let data = serde_json::to_vec(value).map_err(|e| e.to_string())?;
        let temp = self.path.with_extension("tmp");
        std::fs::write(&temp, data).map_err(|e| e.to_string())?;
        std::fs::rename(temp, &self.path).map_err(|e| e.to_string())
    }
}
#[tauri::command]
pub fn desktop_preferences(
    app: tauri::AppHandle,
    close_to_tray: Option<bool>,
    queue_open: Option<bool>,
    theme: Option<String>,
) -> Result<Preferences, String> {
    let state = app.state::<Desktop>();
    let mut guard = state.preferences.lock().map_err(|e| e.to_string())?;
    let mut next = guard.clone();
    if let Some(v) = close_to_tray {
        next.close_to_tray = v;
    }
    if let Some(v) = queue_open {
        next.queue_open = v;
    }
    if let Some(v) = theme {
        if !matches!(v.as_str(), "dark" | "light" | "system") {
            return Err(format!("未知的外观主题: {v}"));
        }
        next.theme = v;
    }
    state.save(&next)?;
    *guard = next.clone();
    Ok(next)
}
// Require the entire window to fit one monitor's work area; disconnected monitors
// and changed DPI therefore fall back to the current primary work area.
fn fits(bounds: (i32, i32, u32, u32), area: (i32, i32, u32, u32)) -> bool {
    let (x, y, w, h) = bounds;
    let (ax, ay, aw, ah) = area;
    w >= 320
        && h >= 240
        && x >= ax
        && y >= ay
        && i64::from(x) + i64::from(w) <= i64::from(ax) + i64::from(aw)
        && i64::from(y) + i64::from(h) <= i64::from(ay) + i64::from(ah)
}
pub fn ensure_visible(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    let pos = window.outer_position()?;
    let size = window.outer_size()?;
    let monitors = window.available_monitors()?;
    if window.is_maximized()? {
        let center_x = i64::from(pos.x) + i64::from(size.width) / 2;
        let center_y = i64::from(pos.y) + i64::from(size.height) / 2;
        if monitors.iter().any(|m| {
            let a = m.work_area();
            center_x >= i64::from(a.position.x)
                && center_x < i64::from(a.position.x) + i64::from(a.size.width)
                && center_y >= i64::from(a.position.y)
                && center_y < i64::from(a.position.y) + i64::from(a.size.height)
        }) {
            return Ok(());
        }
        window.unmaximize()?;
        if let Some(m) = window.primary_monitor()? {
            window.set_position(m.work_area().position)?;
        }
        return window.maximize();
    }
    if !monitors.iter().any(|m| {
        let a = m.work_area();
        fits(
            (pos.x, pos.y, size.width, size.height),
            (a.position.x, a.position.y, a.size.width, a.size.height),
        )
    }) {
        if let Some(m) = window.primary_monitor()? {
            let a = m.work_area();
            let inner = window.inner_size()?;
            let frame_width = size.width.saturating_sub(inner.width);
            let frame_height = size.height.saturating_sub(inner.height);
            let max_width = a.size.width.saturating_sub(frame_width).max(1);
            let max_height = a.size.height.saturating_sub(frame_height).max(1);
            let scale = window.scale_factor()?;
            window.set_min_size(Some(tauri::PhysicalSize::new(
                ((900.0 * scale) as u32).min(max_width),
                ((640.0 * scale) as u32).min(max_height),
            )))?;
            window.set_size(tauri::PhysicalSize::new(
                inner.width.min(max_width),
                inner.height.min(max_height),
            ))?;
            window.set_position(a.position)?;
        }
    }
    Ok(())
}
pub fn show(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let result = (|| {
            w.unminimize()?;
            ensure_visible(&w)?;
            w.show()?;
            w.set_focus()
        })();
        if let Err(e) = result {
            eprintln!("恢复窗口失败: {e}");
        }
    }
}
pub fn window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    if window.label() != "main" {
        return;
    }
    let Some(state) = window.try_state::<Desktop>() else {
        return;
    };
    if matches!(event, tauri::WindowEvent::ScaleFactorChanged { .. }) {
        let app = window.app_handle().clone();
        let handle = app.clone();
        if let Err(e) = app.run_on_main_thread(move || {
            if let Some(w) = handle.get_webview_window("main") {
                if let Err(e) = ensure_visible(&w) {
                    eprintln!("调整显示范围失败: {e}");
                }
            }
        }) {
            eprintln!("调整 DPI 失败: {e}");
        }
    }
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if !state.exiting.load(Ordering::SeqCst)
            && state.preferences.lock().is_ok_and(|p| p.close_to_tray)
        {
            // Only prevent closing after hiding succeeds: a failed tray operation
            // must never strand an invisible process.
            if window.hide().is_ok() {
                api.prevent_close();
            }
        }
    }
    if matches!(
        event,
        tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_)
    ) && !window.is_minimized().unwrap_or(true)
    {
        if let Ok(mut p) = state.preferences.lock() {
            p.maximized = window.is_maximized().unwrap_or(false);
            if !p.maximized {
                if let (Ok(pos), Ok(size)) = (window.outer_position(), window.inner_size()) {
                    p.bounds = Some((pos.x, pos.y, size.width, size.height));
                }
            }
            if let Err(e) = state.save(&p) {
                eprintln!("保存窗口状态失败: {e}");
            }
        }
    }
}
pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let path = app.path().app_data_dir()?.join("desktop-v1.json");
    let preferences = match std::fs::read(&path) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => Preferences::default(),
    };
    app.manage(Desktop {
        preferences: Mutex::new(preferences.clone()),
        path,
        exiting: AtomicBool::new(false),
    });
    if let Some(w) = app.get_webview_window("main") {
        w.set_theme(window_theme(&preferences.theme))?;
        w.set_background_color(Some(window_background(w.theme()?)))?;
        if let Some((x, y, width, height)) = preferences.bounds {
            w.set_size(tauri::PhysicalSize::new(
                width.clamp(900, 16384),
                height.clamp(640, 16384),
            ))?;
            w.set_position(tauri::PhysicalPosition::new(x, y))?;
        }
        ensure_visible(&w)?;
        if preferences.maximized {
            w.maximize()?;
        }
    }
    let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let status = MenuItem::with_id(app, "status", "未选择歌曲", false, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "播放", false, None::<&str>)?;
    let previous = MenuItem::with_id(app, "previous", "上一首", false, None::<&str>)?;
    let next = MenuItem::with_id(app, "next", "下一首", false, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&show_item, &status, &toggle, &previous, &next, &quit],
    )?;
    TrayIconBuilder::with_id("music-mine")
        .icon(tauri::image::Image::from_bytes(include_bytes!(
            "../../icons/icon.png"
        ))?)
        .tooltip("Music Mine")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if matches!(
                event,
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
            ) {
                show(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => show(app),
            "quit" => app.exit(0),
            id => {
                let app = app.clone();
                let id = id.to_string();
                tauri::async_runtime::spawn_blocking(move || {
                    let result = (|| {
                        let state = app.state::<Shared>();
                        let mut c = state.lock().map_err(|e| e.to_string())?;
                        let action = match id.as_str() {
                            "toggle" => {
                                if c.execute(Action::State)?.playback_state == "playing" {
                                    Action::Pause
                                } else {
                                    Action::Play
                                }
                            }
                            "previous" => Action::Previous,
                            "next" => Action::Next,
                            _ => return Ok(()),
                        };
                        c.execute(action)?;
                        Ok::<_, String>(())
                    })();
                    if let Err(e) = result {
                        let _ = status_error(&app, &e);
                    }
                });
            }
        })
        .build(app)?;
    app.manage(TrayItems {
        status,
        toggle,
        previous,
        next,
        last: Mutex::new(String::new()),
    });
    Ok(())
}
struct TrayItems {
    status: MenuItem<tauri::Wry>,
    toggle: MenuItem<tauri::Wry>,
    previous: MenuItem<tauri::Wry>,
    next: MenuItem<tauri::Wry>,
    last: Mutex<String>,
}
fn status_error(app: &tauri::AppHandle, error: &str) -> tauri::Result<()> {
    app.state::<TrayItems>()
        .status
        .set_text(format!("操作失败: {error}"))
}
pub fn update(app: &tauri::AppHandle, s: &crate::playback::controller::PlayerSnapshot) {
    let items = app.state::<TrayItems>();
    let title = s
        .current_track
        .as_ref()
        .map(|t| t.title.as_str())
        .unwrap_or("未选择歌曲");
    let key = format!("{}:{title}:{}", s.playback_state, s.queue.len());
    if let Ok(mut last) = items.last.lock() {
        if *last == key {
            return;
        }
        *last = key;
    }
    let result = (|| {
        items.status.set_text(format!(
            "{} · {title}",
            match s.playback_state {
                "playing" => "正在播放",
                "paused" => "已暂停",
                _ => "已停止",
            }
        ))?;
        items.toggle.set_text(if s.playback_state == "playing" {
            "暂停"
        } else {
            "播放"
        })?;
        items.toggle.set_enabled(s.current_track.is_some())?;
        items.previous.set_enabled(!s.queue.is_empty())?;
        items.next.set_enabled(!s.queue.is_empty())
    })();
    if let Err(e) = result {
        eprintln!("更新托盘失败: {e}");
    }
}
pub fn shutdown(app: &tauri::AppHandle) {
    let state = app.state::<Desktop>();
    if state.exiting.swap(true, Ordering::SeqCst) {
        return;
    }
    if let Ok(mut c) = app.state::<Shared>().lock() {
        if let Err(e) = c.flush_session() {
            eprintln!("保存播放状态失败: {e}");
        }
        if let Err(e) = c.release_output() {
            eprintln!("释放音频输出失败: {e}");
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_defaults_to_exit() {
        let p: Preferences = serde_json::from_str("{}").unwrap();
        assert!(!p.close_to_tray);
        assert!(!p.queue_open);
    }
    #[test]
    fn existing_users_keep_the_dark_title_bar() {
        let old: Preferences = serde_json::from_str(r#"{"close_to_tray":true}"#).unwrap();
        assert_eq!(window_theme(&old.theme), Some(tauri::Theme::Dark));
        assert_eq!(window_theme("light"), Some(tauri::Theme::Light));
        assert_eq!(window_theme("system"), None);
    }
    #[test]
    fn disconnected_and_negative_monitors() {
        assert!(!fits((1920, 0, 900, 640), (0, 0, 1920, 1080)));
        assert!(fits((-1800, 0, 900, 640), (-1920, 0, 1920, 1080)));
        assert!(!fits((0, 0, 2000, 1400), (0, 0, 1920, 1080)));
    }
    #[test]
    fn preferences_replace_existing_file() {
        let directory =
            std::env::temp_dir().join(format!("music-mine-desktop-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let state = Desktop {
            preferences: Mutex::new(Preferences::default()),
            path: directory.join("desktop.json"),
            exiting: AtomicBool::new(false),
        };
        state.save(&Preferences::default()).unwrap();
        let next = Preferences {
            close_to_tray: true,
            queue_open: true,
            ..Default::default()
        };
        state.save(&next).unwrap();
        let restored: Preferences =
            serde_json::from_slice(&std::fs::read(&state.path).unwrap()).unwrap();
        assert!(restored.close_to_tray && restored.queue_open);
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn shutdown_acknowledges_resource_release() {
        let player = crate::audio::player::Player::new(None);
        player.shutdown().unwrap();
        assert!(player.synchronize().is_err());
    }
    #[test]
    fn preferences_roundtrip() {
        let p = Preferences {
            close_to_tray: true,
            queue_open: true,
            theme: "light".into(),
            bounds: Some((-1200, 30, 1000, 700)),
            maximized: true,
        };
        let q: Preferences = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
        assert_eq!(p.bounds, q.bounds);
        assert_eq!(q.theme, "light");
        assert!(q.maximized && q.queue_open && q.close_to_tray);
    }
}
