use crate::{playback::controller::Action, Shared};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

const LYRICS_OVERLAY: &str = "lyrics-overlay";
const LYRICS_ENTRY: &str = "index.html?window=lyrics";
const LYRICS_MIN_WIDTH: u32 = 360;
const LYRICS_MIN_HEIGHT: u32 = 92;

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub output_device: Option<String>,
    pub close_to_tray: bool,
    pub queue_open: bool,
    /// "dark", "light" or "system"; files written before themes existed read as dark.
    pub theme: String,
    bounds: Option<(i32, i32, u32, u32)>,
    maximized: bool,
    pub lyrics_overlay_bounds: Option<(i32, i32, u32, u32)>,
    /// Pointer events pass through the lyrics window while this is true.
    pub lyrics_overlay_locked: bool,
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
    lyrics_operation: Mutex<()>,
    preferences: Mutex<Preferences>,
    path: std::path::PathBuf,
    pub exiting: AtomicBool,
}
impl Desktop {
    pub fn configure_output(
        &self,
        controller: &mut crate::playback::controller::Controller,
        device: Option<String>,
    ) -> Result<crate::playback::controller::PlayerSnapshot, String> {
        let mut guard = self.preferences.lock().map_err(|e| e.to_string())?;
        let mut next = guard.clone();
        next.output_device = device.filter(|name| !name.trim().is_empty());
        let snapshot = controller.execute(Action::OutputDevice(next.output_device.clone()))?;
        if snapshot.error.is_some() {
            return Ok(snapshot);
        }
        if let Err(error) = self.save(&next) {
            let _ = controller.execute(Action::OutputDevice(guard.output_device.clone()));
            return Err(format!("保存音频设备偏好失败：{error}"));
        }
        *guard = next;
        Ok(snapshot)
    }
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

#[derive(Clone, Serialize)]
struct LyricsOverlayState {
    open: bool,
    locked: bool,
}

fn publish_lyrics(app: &tauri::AppHandle, open: bool) {
    let locked = app
        .try_state::<Desktop>()
        .and_then(|state| {
            state
                .preferences
                .lock()
                .ok()
                .map(|prefs| prefs.lyrics_overlay_locked)
        })
        .unwrap_or(false);
    let _ = app.emit(LYRICS_OVERLAY, LyricsOverlayState { open, locked });
    if let Some(items) = app.try_state::<TrayItems>() {
        let _ = items.unlock.set_enabled(locked);
    }
}

/// The whole overlay must sit in one work area. Sizes below the window minimum are rejected.
fn lyrics_overlay_fits(bounds: (i32, i32, u32, u32), area: (i32, i32, u32, u32)) -> bool {
    let (x, y, width, height) = bounds;
    let (ax, ay, aw, ah) = area;
    width >= LYRICS_MIN_WIDTH
        && height >= LYRICS_MIN_HEIGHT
        && width <= 16384
        && height <= 16384
        && x >= ax
        && y >= ay
        && i64::from(x) + i64::from(width) <= i64::from(ax) + i64::from(aw)
        && i64::from(y) + i64::from(height) <= i64::from(ay) + i64::from(ah)
}

fn restore_lyrics_bounds(
    window: &tauri::WebviewWindow,
    bounds: Option<(i32, i32, u32, u32)>,
) -> tauri::Result<()> {
    let Some(bounds) = bounds else {
        return window.center();
    };
    let fits_monitor = window.available_monitors()?.iter().any(|monitor| {
        let area = monitor.work_area();
        lyrics_overlay_fits(
            bounds,
            (
                area.position.x,
                area.position.y,
                area.size.width,
                area.size.height,
            ),
        )
    });
    if !fits_monitor {
        return window.center();
    }
    let (x, y, width, height) = bounds;
    window.set_size(tauri::PhysicalSize::new(width, height))?;
    window.set_position(tauri::PhysicalPosition::new(x, y))
}

fn create_lyrics_window(
    app: &tauri::AppHandle,
    bounds: Option<(i32, i32, u32, u32)>,
) -> Result<tauri::WebviewWindow, String> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        LYRICS_OVERLAY,
        tauri::WebviewUrl::App(LYRICS_ENTRY.into()),
    )
    .title("桌面歌词")
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .shadow(false)
    .resizable(true)
    .focused(false)
    .visible(false)
    .inner_size(640.0, 108.0)
    .min_inner_size(LYRICS_MIN_WIDTH as f64, LYRICS_MIN_HEIGHT as f64)
    .background_color(tauri::window::Color(0, 0, 0, 0))
    .build()
    .map_err(|error| format!("无法创建桌面歌词窗口：{error}"))?;
    restore_lyrics_bounds(&window, bounds)
        .map_err(|error| format!("无法放置桌面歌词窗口：{error}"))?;
    Ok(window)
}

#[tauri::command]
pub async fn desktop_lyrics_window(
    app: tauri::AppHandle,
    open: Option<bool>,
    locked: Option<bool>,
) -> Result<bool, String> {
    // WebView2 creation must not block the Windows event loop. Serialize on the
    // blocking pool so simultaneous toggles cannot create the same label twice.
    tauri::async_runtime::spawn_blocking(move || update_lyrics_window(app, open, locked))
        .await
        .map_err(|error| format!("桌面歌词窗口任务失败：{error}"))?
}

fn update_lyrics_window(
    app: tauri::AppHandle,
    open: Option<bool>,
    locked: Option<bool>,
) -> Result<bool, String> {
    let state = app.state::<Desktop>();
    let _operation = state.lyrics_operation.lock().map_err(|e| e.to_string())?;
    let (locked_now, bounds) = {
        let state = app.state::<Desktop>();
        let mut guard = state
            .preferences
            .lock()
            .map_err(|error| error.to_string())?;
        if let Some(value) = locked {
            let previous = guard.lyrics_overlay_locked;
            guard.lyrics_overlay_locked = value;
            if let Err(error) = state.save(&guard) {
                guard.lyrics_overlay_locked = previous;
                return Err(error);
            }
        }
        (guard.lyrics_overlay_locked, guard.lyrics_overlay_bounds)
    };
    let mut window = app.get_webview_window(LYRICS_OVERLAY);
    if window.is_none() && open == Some(true) {
        window = Some(create_lyrics_window(&app, bounds)?);
    }
    if let Some(window) = window.as_ref() {
        window
            .set_ignore_cursor_events(locked_now)
            .map_err(|error| format!("无法设置桌面歌词鼠标穿透：{error}"))?;
        match open {
            Some(true) => window
                .show()
                .map_err(|error| format!("无法显示桌面歌词：{error}"))?,
            Some(false) => window
                .hide()
                .map_err(|error| format!("无法隐藏桌面歌词：{error}"))?,
            None => {}
        }
    }
    let visible = window
        .as_ref()
        .is_some_and(|window| window.is_visible().unwrap_or(false));
    publish_lyrics(&app, visible);
    Ok(visible)
}

fn ensure_lyrics_on_screen(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    let pos = window.outer_position()?;
    let size = window.inner_size()?;
    let bounds = (pos.x, pos.y, size.width, size.height);
    let fits = window.available_monitors()?.iter().any(|monitor| {
        let area = monitor.work_area();
        lyrics_overlay_fits(
            bounds,
            (
                area.position.x,
                area.position.y,
                area.size.width,
                area.size.height,
            ),
        )
    });
    if !fits {
        window.center()?;
    }
    Ok(())
}

fn lyrics_overlay_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    let Some(state) = window.try_state::<Desktop>() else {
        return;
    };
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if !state.exiting.load(Ordering::SeqCst) && window.hide().is_ok() {
            api.prevent_close();
            publish_lyrics(window.app_handle(), false);
        }
        return;
    }
    if matches!(event, tauri::WindowEvent::ScaleFactorChanged { .. }) {
        let app = window.app_handle().clone();
        let handle = app.clone();
        if let Err(error) = app.run_on_main_thread(move || {
            if let Some(lyrics) = handle.get_webview_window(LYRICS_OVERLAY) {
                if let Err(error) = ensure_lyrics_on_screen(&lyrics) {
                    eprintln!("调整桌面歌词显示范围失败: {error}");
                }
            }
        }) {
            eprintln!("调整桌面歌词 DPI 失败: {error}");
        }
    }
    if matches!(
        event,
        tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_)
    ) && window.is_visible().unwrap_or(false)
    {
        if let (Ok(mut prefs), Ok(pos), Ok(size)) = (
            state.preferences.lock(),
            window.outer_position(),
            window.inner_size(),
        ) {
            if size.width >= LYRICS_MIN_WIDTH && size.height >= LYRICS_MIN_HEIGHT {
                prefs.lyrics_overlay_bounds = Some((pos.x, pos.y, size.width, size.height));
                if let Err(error) = state.save(&prefs) {
                    eprintln!("保存桌面歌词窗口失败: {error}");
                }
            }
        }
    }
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
    if window.label() == LYRICS_OVERLAY {
        lyrics_overlay_event(window, event);
        return;
    }
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
                return;
            }
        }
        if !state.exiting.load(Ordering::SeqCst) {
            // A hidden lyrics WebView keeps the event loop alive. Closing main
            // must request application exit so the existing shutdown hook flushes
            // the session and releases output before disposing all windows.
            api.prevent_close();
            window.app_handle().exit(0);
            return;
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
    // Apply the output preference before the playback session is restored.
    app.state::<Shared>()
        .lock()
        .map_err(|_| "播放器控制器不可用")?
        .execute(Action::OutputDevice(preferences.output_device.clone()))?;
    app.manage(Desktop {
        lyrics_operation: Mutex::new(()),
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
    let unlock = MenuItem::with_id(
        app,
        "unlock-lyrics",
        "解锁桌面歌词",
        preferences.lyrics_overlay_locked,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[
            &show_item, &status, &toggle, &previous, &next, &unlock, &quit,
        ],
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
            "unlock-lyrics" => {
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(error) = desktop_lyrics_window(app.clone(), None, Some(false)).await
                    {
                        let _ = status_error(&app, &error);
                    }
                });
            }
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
        unlock,
        last: Mutex::new(String::new()),
    });
    Ok(())
}
struct TrayItems {
    status: MenuItem<tauri::Wry>,
    toggle: MenuItem<tauri::Wry>,
    previous: MenuItem<tauri::Wry>,
    next: MenuItem<tauri::Wry>,
    unlock: MenuItem<tauri::Wry>,
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
    if let Some(window) = app.get_webview_window(LYRICS_OVERLAY) {
        let _ = window.close();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_defaults_to_exit() {
        let p: Preferences = serde_json::from_str("{}").unwrap();
        assert!(!p.close_to_tray);
        assert!(!p.queue_open);
        assert_eq!(p.output_device, None);
        assert!(!p.lyrics_overlay_locked);
        assert_eq!(p.lyrics_overlay_bounds, None);
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
            lyrics_operation: Mutex::new(()),
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
    fn output_preference_saves_and_rolls_back_on_write_failure() {
        if std::env::var_os("MUSIC_MINE_NULL_OUTPUT").is_none() {
            return;
        }
        let directory = std::env::temp_dir().join(format!(
            "music-mine-output-preference-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).unwrap();
        let mut controller = crate::playback::controller::Controller::new();
        let state = Desktop {
            lyrics_operation: Mutex::new(()),
            preferences: Mutex::new(Preferences::default()),
            path: directory.join("desktop.json"),
            exiting: AtomicBool::new(false),
        };
        state
            .configure_output(&mut controller, Some("Disconnected headphones".into()))
            .unwrap();
        let saved: Preferences =
            serde_json::from_slice(&std::fs::read(&state.path).unwrap()).unwrap();
        assert_eq!(
            saved.output_device.as_deref(),
            Some("Disconnected headphones")
        );
        assert!(!saved.close_to_tray);
        state
            .configure_output(&mut controller, Some("  ".into()))
            .unwrap();
        let saved: Preferences =
            serde_json::from_slice(&std::fs::read(&state.path).unwrap()).unwrap();
        assert_eq!(saved.output_device, None);
        let failing = Desktop {
            path: directory.join("missing/desktop.json"),
            ..state
        };
        assert!(failing
            .configure_output(&mut controller, Some("null".into()))
            .err()
            .unwrap()
            .contains("保存音频设备偏好失败"));
        assert_eq!(failing.preferences.lock().unwrap().output_device, None);
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
            output_device: Some("Headphones".into()),
            bounds: Some((-1200, 30, 1000, 700)),
            maximized: true,
            lyrics_overlay_locked: true,
            lyrics_overlay_bounds: Some((40, 80, 760, 136)),
        };
        let q: Preferences = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
        assert_eq!(p.bounds, q.bounds);
        assert_eq!(q.theme, "light");
        assert_eq!(q.output_device.as_deref(), Some("Headphones"));
        assert!(q.maximized && q.queue_open && q.close_to_tray);
        assert!(q.lyrics_overlay_locked);
        assert_eq!(q.lyrics_overlay_bounds, Some((40, 80, 760, 136)));
    }
    #[test]
    fn old_desktop_files_default_the_lyrics_overlay() {
        let old: Preferences = serde_json::from_str(
            r#"{"close_to_tray":true,"queue_open":true,"bounds":[10,20,1280,800]}"#,
        )
        .unwrap();
        assert!(old.close_to_tray && old.queue_open);
        assert_eq!(old.bounds, Some((10, 20, 1280, 800)));
        assert!(!old.lyrics_overlay_locked);
        assert_eq!(old.lyrics_overlay_bounds, None);
        let explicit: Preferences =
            serde_json::from_str(r#"{"lyrics_overlay_locked":false,"lyrics_overlay_bounds":null}"#)
                .unwrap();
        assert!(!explicit.lyrics_overlay_locked);
        assert_eq!(explicit.lyrics_overlay_bounds, None);
        assert!(LYRICS_ENTRY.contains("?window=lyrics"));
    }
    #[test]
    fn lyrics_overlay_bounds_must_fit_one_work_area() {
        let desktop = (0, 0, 1920, 1080);
        assert!(lyrics_overlay_fits((0, 0, 760, 136), desktop));
        assert!(lyrics_overlay_fits(
            (0, 0, LYRICS_MIN_WIDTH, LYRICS_MIN_HEIGHT),
            desktop
        ));
        assert!(lyrics_overlay_fits((1560, 944, 360, 136), desktop));
        assert!(!lyrics_overlay_fits(
            (0, 0, LYRICS_MIN_WIDTH - 1, 136),
            desktop
        ));
        assert!(!lyrics_overlay_fits(
            (0, 0, 760, LYRICS_MIN_HEIGHT - 1),
            desktop
        ));
        assert!(!lyrics_overlay_fits((1561, 944, 360, 136), desktop));
        assert!(!lyrics_overlay_fits((1800, 0, 760, 136), desktop));
        assert!(!lyrics_overlay_fits((0, 0, 20_000, 136), desktop));
        let left = (-1920, 0, 1920, 1080);
        assert!(lyrics_overlay_fits((-1800, 40, 760, 136), left));
        assert!(!lyrics_overlay_fits((40, 40, 760, 136), left));
    }
}
