use crate::{playback::controller::Action, Shared};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{
    menu::{CheckMenuItem, IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

#[cfg(windows)]
#[path = "thumbbar.rs"]
mod thumbbar;

const LYRICS_OVERLAY: &str = "lyrics-overlay";
const LYRICS_ENTRY: &str = "index.html?window=lyrics";
const LYRICS_MIN_WIDTH: u32 = 360;
const LYRICS_MIN_HEIGHT: u32 = 92;

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub output_device: Option<String>,
    pub close_to_tray: bool,
    pub queue_open: bool,
    /// "dark", "light" or "system"; missing or unknown choices use light.
    pub theme: String,
    bounds: Option<(i32, i32, u32, u32)>,
    maximized: bool,
    pub lyrics_overlay_bounds: Option<(i32, i32, u32, u32)>,
    /// Pointer events pass through the lyrics window while this is true.
    pub lyrics_overlay_locked: bool,
    pub listenbrainz_token: Option<String>,
    pub listenbrainz_username: Option<String>,
    pub listenbrainz_enabled: bool,
    pub ai_enabled: bool,
    pub ai_base_url: Option<String>,
    pub ai_api_key: Option<String>,
    pub ai_model: Option<String>,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            output_device: None,
            close_to_tray: false,
            queue_open: false,
            theme: String::new(),
            bounds: None,
            maximized: false,
            lyrics_overlay_bounds: None,
            lyrics_overlay_locked: false,
            listenbrainz_token: None,
            listenbrainz_username: None,
            listenbrainz_enabled: false,
            ai_enabled: false,
            ai_base_url: Some("https://api.openai.com/v1".into()),
            ai_api_key: None,
            ai_model: Some("gpt-4o-mini".into()),
        }
    }
}
/// Native title bar theme; `None` lets Windows follow the system setting.
fn window_theme(theme: &str) -> Option<tauri::Theme> {
    match theme {
        "dark" => Some(tauri::Theme::Dark),
        "system" => None,
        _ => Some(tauri::Theme::Light),
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
    pub(crate) fn configure_ai(
        &self,
        patch: super::ai_agent::ConfigPatch,
    ) -> Result<Preferences, String> {
        let mut guard = self.preferences.lock().map_err(|_| "AI_CONFIG_LOCK")?;
        let next = super::ai_agent::apply_config(&guard, patch)?;
        self.save(&next).map_err(|_| "AI_CONFIG_SAVE".to_string())?;
        *guard = next.clone();
        Ok(next)
    }
    pub(crate) fn scrobble_preferences(&self) -> Result<Preferences, String> {
        self.preferences
            .lock()
            .map(|p| p.clone())
            .map_err(|_| "偏好不可用".into())
    }

    pub(crate) fn save_scrobble(
        &self,
        enabled: bool,
        username: Option<String>,
        token: Option<String>,
    ) -> Result<(), String> {
        let mut guard = self.preferences.lock().map_err(|_| "偏好不可用")?;
        let mut next = guard.clone();
        next.listenbrainz_enabled = enabled;
        next.listenbrainz_username = username;
        next.listenbrainz_token = token;
        // Never include serialization or filesystem contents in an IPC error.
        self.save(&next)
            .map_err(|_| "ListenBrainz 设置保存失败".to_string())?;
        *guard = next;
        Ok(())
    }
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
    next.listenbrainz_token = None;
    next.ai_api_key = None;
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
        items.lyrics_open.store(open, Ordering::SeqCst);
        items.lyrics_locked.store(locked, Ordering::SeqCst);
        let _ = items.lyrics.set_checked(open);
        let _ = set_unlock_visible(&items, locked);
        if let Ok(mut last) = items.last.lock() {
            last.clear();
        }
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
        #[cfg(windows)]
        thumbbar::refresh_after_show();
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
    install_tray(app, preferences.lyrics_overlay_locked)?;
    #[cfg(windows)]
    if let Some(window) = app.get_webview_window("main") {
        match window.hwnd() {
            Ok(hwnd) => thumbbar::attach(app.handle(), hwnd.0),
            Err(error) => eprintln!("读取主窗口句柄失败: {error}"),
        }
    }
    Ok(())
}
fn install_tray(
    app: &mut tauri::App,
    lyrics_locked: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = MenuItem::with_id(app, "status", "未在播放", false, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
    let sep_transport = PredefinedMenuItem::separator(app)?;
    let toggle = MenuItem::with_id(app, "toggle", "播放", false, None::<&str>)?;
    let stop = MenuItem::with_id(app, "stop", "停止", false, None::<&str>)?;
    let previous = MenuItem::with_id(app, "previous", "上一首", false, None::<&str>)?;
    let next = MenuItem::with_id(app, "next", "下一首", false, None::<&str>)?;
    let sep_lyrics = PredefinedMenuItem::separator(app)?;
    let lyrics = CheckMenuItem::with_id(app, "lyrics", "桌面歌词", true, false, None::<&str>)?;
    let unlock = MenuItem::with_id(app, "unlock-lyrics", "解锁桌面歌词穿透", true, None::<&str>)?;
    let mode_sequential =
        CheckMenuItem::with_id(app, "mode-sequential", "顺序播放", true, true, None::<&str>)?;
    let mode_shuffle =
        CheckMenuItem::with_id(app, "mode-shuffle", "随机播放", true, false, None::<&str>)?;
    let mode_repeat_one = CheckMenuItem::with_id(
        app,
        "mode-repeat_one",
        "单曲循环",
        true,
        false,
        None::<&str>,
    )?;
    let mode_repeat_all = CheckMenuItem::with_id(
        app,
        "mode-repeat_all",
        "列表循环",
        true,
        false,
        None::<&str>,
    )?;
    let mode = Submenu::with_id_and_items(
        app,
        "playback-mode",
        "播放模式",
        true,
        &[
            &mode_sequential,
            &mode_shuffle,
            &mode_repeat_one,
            &mode_repeat_all,
        ],
    )?;
    let sep_quit = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let mut rows: Vec<&dyn IsMenuItem<tauri::Wry>> = vec![
        &status,
        &show_item,
        &sep_transport,
        &toggle,
        &stop,
        &previous,
        &next,
        &sep_lyrics,
        &lyrics,
    ];
    if lyrics_locked {
        rows.push(&unlock);
    }
    rows.push(&mode);
    rows.push(&sep_quit);
    rows.push(&quit);
    let menu = Menu::with_items(app, &rows)?;
    TrayIconBuilder::with_id("music-mine")
        .icon(tauri::image::Image::from_bytes(include_bytes!(
            "../../icons/icon.png"
        ))?)
        .tooltip("Music Mine · 未在播放")
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
                toggle_main_window(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| on_tray_menu(app, event.id().as_ref()))
        .build(app)?;
    app.manage(TrayItems {
        status,
        toggle,
        stop,
        previous,
        next,
        lyrics,
        unlock,
        menu,
        unlock_shown: AtomicBool::new(lyrics_locked),
        mode_sequential,
        mode_shuffle,
        mode_repeat_one,
        mode_repeat_all,
        lyrics_open: AtomicBool::new(false),
        lyrics_locked: AtomicBool::new(lyrics_locked),
        last: Mutex::new(String::new()),
    });
    Ok(())
}
fn toggle_main_window(app: &tauri::AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let visible = window.is_visible().unwrap_or(false);
    let minimized = window.is_minimized().unwrap_or(false);
    if visible && !minimized {
        if let Err(error) = window.hide() {
            eprintln!("隐藏窗口失败: {error}");
        }
        return;
    }
    show(app);
}
fn on_tray_menu(app: &tauri::AppHandle, id: &str) {
    match id {
        "show" => show(app),
        "quit" => app.exit(0),
        "unlock-lyrics" => {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(error) = desktop_lyrics_window(app.clone(), None, Some(false)).await {
                    let _ = status_error(&app, &error);
                }
            });
        }
        "lyrics" => {
            let open = app
                .try_state::<TrayItems>()
                .is_some_and(|items| items.lyrics_open.load(Ordering::SeqCst));
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(error) = desktop_lyrics_window(app.clone(), Some(!open), None).await {
                    let _ = status_error(&app, &error);
                }
            });
        }
        "toggle" => dispatch_playback(app, PlaybackRequest::Toggle),
        "stop" => dispatch_playback(app, PlaybackRequest::Stop),
        "previous" => dispatch_playback(app, PlaybackRequest::Previous),
        "next" => dispatch_playback(app, PlaybackRequest::Next),
        "mode-sequential" | "mode-shuffle" | "mode-repeat_one" | "mode-repeat_all" => {
            dispatch_playback(
                app,
                PlaybackRequest::Mode(id.trim_start_matches("mode-").to_string()),
            );
        }
        _ => {}
    }
}
struct TrayItems {
    status: MenuItem<tauri::Wry>,
    toggle: MenuItem<tauri::Wry>,
    stop: MenuItem<tauri::Wry>,
    previous: MenuItem<tauri::Wry>,
    next: MenuItem<tauri::Wry>,
    lyrics: CheckMenuItem<tauri::Wry>,
    unlock: MenuItem<tauri::Wry>,
    menu: Menu<tauri::Wry>,
    unlock_shown: AtomicBool,
    mode_sequential: CheckMenuItem<tauri::Wry>,
    mode_shuffle: CheckMenuItem<tauri::Wry>,
    mode_repeat_one: CheckMenuItem<tauri::Wry>,
    mode_repeat_all: CheckMenuItem<tauri::Wry>,
    lyrics_open: AtomicBool,
    lyrics_locked: AtomicBool,
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
        .map(|track| track.title.as_str())
        .unwrap_or("");
    let artist = s
        .current_track
        .as_ref()
        .map(|track| track.artist.as_str())
        .unwrap_or("");
    let has_track = s.current_track.is_some();
    let lyrics_open = items.lyrics_open.load(Ordering::SeqCst);
    let lyrics_locked = items.lyrics_locked.load(Ordering::SeqCst);
    let query = TrayQuery {
        state: s.playback_state,
        title,
        artist,
        has_track,
        queue_len: s.queue.len(),
        mode: &s.playback_mode,
    };
    let face = tray_face(&query);
    let key = tray_signature(&query, lyrics_open, lyrics_locked);
    let unchanged = items
        .last
        .lock()
        .ok()
        .is_some_and(|last| !tray_signature_changed(&last, &key));
    #[cfg(windows)]
    thumbbar::sync(app, thumb_flags(s.playback_state, has_track, s.queue.len()));
    if unchanged {
        return;
    }
    let result = (|| {
        items.status.set_text(menu_label(&face.status))?;
        if let Some(tray) = app.tray_by_id("music-mine") {
            tray.set_tooltip(Some(face.tooltip.as_str()))?;
        }
        items.toggle.set_text(face.toggle)?;
        items.toggle.set_enabled(face.can_toggle)?;
        items.stop.set_enabled(face.can_stop)?;
        items.previous.set_enabled(face.can_skip)?;
        items.next.set_enabled(face.can_skip)?;
        items.lyrics.set_checked(lyrics_open)?;
        set_unlock_visible(&items, lyrics_locked)?;
        let [sequential, shuffle, repeat_one, repeat_all] = face.marks;
        items.mode_sequential.set_checked(sequential)?;
        items.mode_shuffle.set_checked(shuffle)?;
        items.mode_repeat_one.set_checked(repeat_one)?;
        items.mode_repeat_all.set_checked(repeat_all)?;
        Ok::<(), tauri::Error>(())
    })();
    if let Err(error) = result {
        eprintln!("更新托盘失败: {error}");
        return;
    }
    if let Ok(mut last) = items.last.lock() {
        *last = key;
    };
}
pub fn shutdown(app: &tauri::AppHandle) {
    let state = app.state::<Desktop>();
    if state.exiting.swap(true, Ordering::SeqCst) {
        return;
    }
    #[cfg(windows)]
    thumbbar::detach();
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

const THUMB_PX: usize = 16;
const LABEL_CHARS: usize = 80;

enum PlaybackRequest {
    Toggle,
    Stop,
    Previous,
    Next,
    Mode(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThumbGlyph {
    Previous,
    Play,
    Pause,
    Stop,
    Next,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ThumbFlags {
    playing: bool,
    toggle: bool,
    stop: bool,
    skip: bool,
}

struct TrayQuery<'a> {
    state: &'a str,
    title: &'a str,
    artist: &'a str,
    has_track: bool,
    queue_len: usize,
    mode: &'a str,
}

struct TrayFace {
    status: String,
    tooltip: String,
    toggle: &'static str,
    can_toggle: bool,
    can_stop: bool,
    can_skip: bool,
    marks: [bool; 4],
}

fn dispatch_playback(app: &tauri::AppHandle, request: PlaybackRequest) {
    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let result = (|| {
            let state = app.state::<Shared>();
            let mut controller = state.lock().map_err(|error| error.to_string())?;
            let action = match request {
                PlaybackRequest::Toggle => {
                    if controller.execute(Action::State)?.playback_state == "playing" {
                        Action::Pause
                    } else {
                        Action::Play
                    }
                }
                PlaybackRequest::Stop => Action::Stop,
                PlaybackRequest::Previous => Action::Previous,
                PlaybackRequest::Next => Action::Next,
                PlaybackRequest::Mode(mode) => Action::Mode(mode),
            };
            let snapshot = controller.execute(action)?;
            drop(controller);
            update(&app, &snapshot);
            crate::platform::media::update(&app, &snapshot);
            Ok::<(), String>(())
        })();
        if let Err(error) = result {
            let _ = status_error(&app, &error);
        }
    });
}

fn thumb_request(id: u32) -> Option<PlaybackRequest> {
    match id {
        1001 => Some(PlaybackRequest::Previous),
        1002 => Some(PlaybackRequest::Toggle),
        1003 => Some(PlaybackRequest::Stop),
        1004 => Some(PlaybackRequest::Next),
        _ => None,
    }
}

fn thumb_tip(id: u32, playing: bool) -> Option<&'static str> {
    match id {
        1001 => Some("上一首"),
        1002 if playing => Some("暂停"),
        1002 => Some("播放"),
        1003 => Some("停止"),
        1004 => Some("下一首"),
        _ => None,
    }
}

fn thumb_flags(state: &str, has_track: bool, queue_len: usize) -> ThumbFlags {
    ThumbFlags {
        playing: state == "playing",
        toggle: has_track || state == "playing",
        stop: state == "playing" || state == "paused",
        skip: queue_len > 0,
    }
}

fn mode_marks(mode: &str) -> [bool; 4] {
    [
        mode == "sequential",
        mode == "shuffle",
        mode == "repeat_one",
        mode == "repeat_all",
    ]
}

fn clip_chars(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let mut clipped: String = text.chars().take(max).collect();
    clipped.push('…');
    clipped
}

fn track_label(title: &str, artist: &str) -> String {
    let title = title.trim();
    let artist = artist.trim();
    let combined = match (title.is_empty(), artist.is_empty()) {
        (true, true) => "未命名".to_string(),
        (false, true) => title.to_string(),
        (true, false) => artist.to_string(),
        (false, false) => format!("{title} - {artist}"),
    };
    clip_chars(&combined, LABEL_CHARS)
}

fn menu_label(text: &str) -> String {
    text.replace('&', "&&")
}

fn tray_face(query: &TrayQuery<'_>) -> TrayFace {
    let idle = query.state == "idle" || !query.has_track;
    let label = track_label(query.title, query.artist);
    let (status, tooltip) = if idle {
        ("未在播放".to_string(), "Music Mine · 未在播放".to_string())
    } else {
        (
            format!("♪ {label}"),
            clip_chars(&format!("Music Mine · {label}"), 120),
        )
    };
    TrayFace {
        status,
        tooltip,
        toggle: if query.state == "playing" {
            "暂停"
        } else {
            "播放"
        },
        can_toggle: query.has_track || query.state == "playing",
        can_stop: query.state == "playing" || query.state == "paused",
        can_skip: query.queue_len > 0,
        marks: mode_marks(query.mode),
    }
}

fn tray_signature(query: &TrayQuery<'_>, lyrics_open: bool, lyrics_locked: bool) -> String {
    let label = if query.state == "idle" || !query.has_track {
        "未在播放".to_string()
    } else {
        track_label(query.title, query.artist)
    };
    format!(
        "{}\n{label}\n{}\n{}\n{}\n{lyrics_open}\n{lyrics_locked}",
        query.state, query.has_track, query.queue_len, query.mode
    )
}

fn tray_signature_changed(previous: &str, next: &str) -> bool {
    previous != next
}

const GLYPH_MIN: usize = 2;
const GLYPH_MAX: usize = 13;
const UNLOCK_INDEX: usize = 9;

fn set_unlock_visible(items: &TrayItems, locked: bool) -> tauri::Result<()> {
    if items.unlock_shown.load(Ordering::SeqCst) == locked {
        return Ok(());
    }
    if locked {
        items.menu.insert(&items.unlock, UNLOCK_INDEX)?;
    } else {
        items.menu.remove(&items.unlock)?;
    }
    items.unlock_shown.store(locked, Ordering::SeqCst);
    Ok(())
}

fn thumb_rgba(glyph: ThumbGlyph) -> [u8; THUMB_PX * THUMB_PX * 4] {
    let mut mask = [[false; THUMB_PX]; THUMB_PX];
    match glyph {
        ThumbGlyph::Play => play_triangle(&mut mask),
        ThumbGlyph::Previous => skip_mark(&mut mask, true),
        ThumbGlyph::Next => skip_mark(&mut mask, false),
        ThumbGlyph::Pause => {
            fill_rect(&mut mask, GLYPH_MIN, 5, GLYPH_MIN, GLYPH_MAX);
            fill_rect(&mut mask, 10, GLYPH_MAX, GLYPH_MIN, GLYPH_MAX);
        }
        ThumbGlyph::Stop => fill_rect(&mut mask, GLYPH_MIN, GLYPH_MAX, GLYPH_MIN, GLYPH_MAX),
    }
    rasterize(&mask)
}

fn center_distance(y: usize) -> usize {
    if y < 8 {
        7 - y
    } else {
        y - 8
    }
}

fn play_triangle(mask: &mut [[bool; THUMB_PX]; THUMB_PX]) {
    for y in GLYPH_MIN..=GLYPH_MAX {
        let x1 = GLYPH_MAX - center_distance(y) * 11 / 5;
        fill_rect(mask, GLYPH_MIN, x1, y, y);
    }
}

fn skip_mark(mask: &mut [[bool; THUMB_PX]; THUMB_PX], previous: bool) {
    if previous {
        fill_rect(mask, GLYPH_MIN, 4, GLYPH_MIN, GLYPH_MAX);
        for y in GLYPH_MIN..=GLYPH_MAX {
            let x0 = 6 + center_distance(y) * 7 / 5;
            fill_rect(mask, x0, GLYPH_MAX, y, y);
        }
    } else {
        fill_rect(mask, 11, GLYPH_MAX, GLYPH_MIN, GLYPH_MAX);
        for y in GLYPH_MIN..=GLYPH_MAX {
            let x1 = 9 - center_distance(y) * 7 / 5;
            fill_rect(mask, GLYPH_MIN, x1, y, y);
        }
    }
}

fn fill_rect(mask: &mut [[bool; THUMB_PX]; THUMB_PX], x0: usize, x1: usize, y0: usize, y1: usize) {
    for row in mask.iter_mut().take(y1 + 1).skip(y0) {
        for cell in row.iter_mut().take(x1 + 1).skip(x0) {
            *cell = true;
        }
    }
}

fn rasterize(mask: &[[bool; THUMB_PX]; THUMB_PX]) -> [u8; THUMB_PX * THUMB_PX * 4] {
    let mut pixels = [0u8; THUMB_PX * THUMB_PX * 4];
    for y in 0..THUMB_PX {
        for x in 0..THUMB_PX {
            let filled = mask[y][x];
            let outline = !filled && outline_of(mask, x, y);
            let (red, green, blue, alpha) = if filled {
                (255, 255, 255, 255)
            } else if outline {
                (16, 16, 16, 255)
            } else {
                (0, 0, 0, 0)
            };
            let index = (y * THUMB_PX + x) * 4;
            pixels[index] = red;
            pixels[index + 1] = green;
            pixels[index + 2] = blue;
            pixels[index + 3] = alpha;
        }
    }
    pixels
}

fn outline_of(mask: &[[bool; THUMB_PX]; THUMB_PX], x: usize, y: usize) -> bool {
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if (0..THUMB_PX as i32).contains(&nx)
                && (0..THUMB_PX as i32).contains(&ny)
                && mask[ny as usize][nx as usize]
            {
                return true;
            }
        }
    }
    false
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
    fn missing_theme_defaults_to_light_and_saved_choices_are_preserved() {
        let old: Preferences = serde_json::from_str(r#"{"close_to_tray":true}"#).unwrap();
        assert_eq!(window_theme(&old.theme), Some(tauri::Theme::Light));
        assert_eq!(window_theme("dark"), Some(tauri::Theme::Dark));
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
    fn ai_configuration_persists_and_rejects_invalid_changes_without_overwriting_other_preferences()
    {
        use super::super::ai_agent::ConfigPatch;
        let path = std::env::temp_dir().join(format!(
            "mm-ai-config-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let p = Preferences {
            listenbrainz_username: Some("fixture-user".into()),
            ..Preferences::default()
        };
        let state = Desktop {
            lyrics_operation: Mutex::new(()),
            preferences: Mutex::new(p),
            path: path.clone(),
            exiting: AtomicBool::new(false),
        };
        state
            .configure_ai(ConfigPatch {
                enabled: Some(true),
                base_url: Some("http://localhost:11434/v1".into()),
                api_key: Some("fixture-secret-key".into()),
                model: Some("fixture-model".into()),
            })
            .unwrap();
        let restored: Preferences = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert!(restored.ai_enabled);
        assert_eq!(restored.ai_api_key.as_deref(), Some("fixture-secret-key"));
        assert_eq!(
            restored.listenbrainz_username.as_deref(),
            Some("fixture-user")
        );
        let before = std::fs::read(&path).unwrap();
        assert!(state
            .configure_ai(ConfigPatch {
                base_url: Some("file:///secret".into()),
                ..Default::default()
            })
            .is_err());
        assert_eq!(std::fs::read(&path).unwrap(), before);
        state
            .configure_ai(ConfigPatch {
                model: Some("another-model".into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            state.scrobble_preferences().unwrap().ai_api_key.as_deref(),
            Some("fixture-secret-key")
        );
        state
            .configure_ai(ConfigPatch {
                api_key: Some(String::new()),
                ..Default::default()
            })
            .unwrap();
        let cleared: Preferences = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert!(cleared.ai_api_key.is_none());
        assert!(!cleared.ai_enabled);
        std::fs::remove_file(path).unwrap();
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
            ..Preferences::default()
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
    fn query<'a>(
        state: &'a str,
        title: &'a str,
        artist: &'a str,
        has_track: bool,
        queue_len: usize,
        mode: &'a str,
    ) -> TrayQuery<'a> {
        TrayQuery {
            state,
            title,
            artist,
            has_track,
            queue_len,
            mode,
        }
    }
    #[test]
    fn tray_summary_dedup_and_thumb_pixels() {
        let playing = tray_face(&query("playing", "雨", "周杰伦", true, 3, "shuffle"));
        assert_eq!(playing.status, "♪ 雨 - 周杰伦");
        assert_eq!(playing.tooltip, "Music Mine · 雨 - 周杰伦");
        assert_eq!(playing.toggle, "暂停");
        assert!(playing.can_toggle && playing.can_stop && playing.can_skip);
        assert_eq!(playing.marks, [false, true, false, false]);
        let idle = tray_face(&query("idle", "雨", "周杰伦", false, 0, "sequential"));
        assert_eq!(idle.status, "未在播放");
        assert_eq!(idle.tooltip, "Music Mine · 未在播放");
        assert_eq!(idle.toggle, "播放");
        assert!(!idle.can_toggle && !idle.can_stop && !idle.can_skip);
        assert_eq!(idle.marks, [true, false, false, false]);
        let paused = tray_face(&query("paused", "雨", "", true, 1, "repeat_one"));
        assert_eq!(paused.status, "♪ 雨");
        assert_eq!(paused.tooltip, "Music Mine · 雨");
        assert_eq!(paused.toggle, "播放");
        assert!(paused.can_toggle && paused.can_stop && paused.can_skip);
        assert_eq!(paused.marks, [false, false, true, false]);
        let stopped_with_queue = tray_face(&query("idle", "", "", false, 2, "repeat_all"));
        assert!(!stopped_with_queue.can_stop && stopped_with_queue.can_skip);
        assert_eq!(stopped_with_queue.marks, [false, false, false, true]);
        let artist_only = tray_face(&query("paused", "  ", "陈奕迅", true, 1, "sequential"));
        assert_eq!(artist_only.status, "♪ 陈奕迅");
        let unnamed = tray_face(&query("playing", " ", " ", true, 1, "nope"));
        assert_eq!(unnamed.status, "♪ 未命名");
        assert_eq!(unnamed.marks, [false, false, false, false]);
        let key = tray_signature(
            &query("playing", "雨", "周杰伦", true, 3, "shuffle"),
            true,
            false,
        );
        assert!(!tray_signature_changed(&key, &key));
        assert!(tray_signature_changed(
            &key,
            &tray_signature(
                &query("paused", "雨", "周杰伦", true, 3, "shuffle"),
                true,
                false
            )
        ));
        assert!(tray_signature_changed(
            &key,
            &tray_signature(
                &query("playing", "晴天", "周杰伦", true, 3, "shuffle"),
                true,
                false
            )
        ));
        assert!(tray_signature_changed(
            &key,
            &tray_signature(
                &query("playing", "雨", "周杰伦", true, 3, "repeat_all"),
                true,
                false
            )
        ));
        assert!(tray_signature_changed(
            &key,
            &tray_signature(
                &query("playing", "雨", "周杰伦", true, 3, "shuffle"),
                false,
                false
            )
        ));
        assert!(tray_signature_changed(
            &key,
            &tray_signature(
                &query("playing", "雨", "周杰伦", true, 3, "shuffle"),
                true,
                true
            )
        ));
        assert!(!tray_signature_changed(
            &key,
            &tray_signature(
                &query("playing", "雨", "周杰伦", true, 3, "shuffle"),
                true,
                false
            )
        ));
        assert_eq!(menu_label("A & B"), "A && B");
        let long = "音".repeat(81);
        let clipped = tray_face(&query("playing", &long, "", true, 1, "sequential"));
        assert!(clipped.status.starts_with("♪ "));
        assert!(clipped.status.ends_with('…'));
        assert_eq!(clipped.status.chars().count(), 83);
        let playing_flags = thumb_flags("playing", true, 0);
        let paused_flags = thumb_flags("paused", true, 2);
        assert!(
            playing_flags.playing
                && playing_flags.toggle
                && playing_flags.stop
                && !playing_flags.skip
        );
        assert!(!paused_flags.playing && paused_flags.stop && paused_flags.skip);
        assert_ne!(playing_flags, paused_flags);
        assert!(matches!(
            thumb_request(1001),
            Some(PlaybackRequest::Previous)
        ));
        assert!(matches!(thumb_request(1002), Some(PlaybackRequest::Toggle)));
        assert!(matches!(thumb_request(1003), Some(PlaybackRequest::Stop)));
        assert!(matches!(thumb_request(1004), Some(PlaybackRequest::Next)));
        assert!(thumb_request(1000).is_none());
        assert_eq!(thumb_tip(1002, true), Some("暂停"));
        assert_eq!(thumb_tip(1002, false), Some("播放"));
        assert_eq!(thumb_tip(1001, false), Some("上一首"));
        assert_eq!(thumb_tip(1004, true), Some("下一首"));
        let mut seen = Vec::new();
        for glyph in [
            ThumbGlyph::Previous,
            ThumbGlyph::Play,
            ThumbGlyph::Pause,
            ThumbGlyph::Stop,
            ThumbGlyph::Next,
        ] {
            let pixels = thumb_rgba(glyph);
            assert_eq!(pixels.len(), THUMB_PX * THUMB_PX * 4);
            assert!(pixels.chunks(4).any(|pixel| pixel[3] > 0));
            assert!(pixels
                .chunks(4)
                .any(|pixel| pixel[3] == 255 && pixel[0] > 200));
            assert!(pixels
                .chunks(4)
                .any(|pixel| pixel[3] == 255 && pixel[0] < 40));
            assert!(pixels
                .chunks(4)
                .all(|pixel| pixel[3] == 0 || pixel[3] == 255));
            let (width, height) = solid_span(&pixels);
            assert!(
                width >= 12 && height >= 12,
                "thumb glyph occupies {width}x{height}"
            );
            seen.push(pixels);
        }
        for index in 0..seen.len() {
            for other in (index + 1)..seen.len() {
                assert_ne!(seen[index], seen[other]);
            }
        }
    }
    fn solid_span(pixels: &[u8]) -> (usize, usize) {
        let mut min_x = THUMB_PX;
        let mut min_y = THUMB_PX;
        let mut max_x = 0;
        let mut max_y = 0;
        for y in 0..THUMB_PX {
            for x in 0..THUMB_PX {
                let pixel = &pixels[(y * THUMB_PX + x) * 4..];
                if pixel[3] == 255 && pixel[0] > 200 {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }
        (max_x - min_x + 1, max_y - min_y + 1)
    }

    #[test]
    fn thumb_triangles_point_in_the_transport_direction() {
        let white = [255, 255, 255, 255];
        let pixel = |pixels: &[u8], x: usize, y: usize| {
            let offset = (y * 16 + x) * 4;
            pixels[offset..offset + 4].to_vec()
        };
        let play = thumb_rgba(ThumbGlyph::Play);
        assert_eq!(pixel(&play, 2, 2), white); // upright base at the left
        assert_ne!(pixel(&play, 13, 2), white);
        assert_eq!(pixel(&play, 13, 7), white); // tip at the right
        let next = thumb_rgba(ThumbGlyph::Next);
        assert_eq!(pixel(&next, 2, 2), white);
        assert_ne!(pixel(&next, 9, 2), white);
        assert_eq!(pixel(&next, 9, 7), white);
        let previous = thumb_rgba(ThumbGlyph::Previous);
        assert_eq!(pixel(&previous, 13, 2), white);
        assert_ne!(pixel(&previous, 6, 2), white);
        assert_eq!(pixel(&previous, 6, 7), white);
        for y in 0..16 {
            for x in 0..16 {
                assert_eq!(pixel(&previous, x, y), pixel(&next, 15 - x, y));
            }
        }
    }
}
