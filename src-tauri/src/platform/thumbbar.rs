//! Windows 7+ taskbar thumbnail toolbar. The COM object stays on the UI thread.
use super::{ThumbFlags, ThumbGlyph};
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HANDLE, HWND, LPARAM, LRESULT, TRUE, WPARAM},
        Graphics::Gdi::{
            CreateBitmap, CreateDIBSection, DeleteObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
            DIB_RGB_COLORS, HDC,
        },
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::{
            Shell::{
                DefSubclassProc, ITaskbarList3, RemoveWindowSubclass, SetWindowSubclass,
                TaskbarList, THBF_DISABLED, THBF_ENABLED, THBN_CLICKED, THB_FLAGS, THB_ICON,
                THB_TOOLTIP, THUMBBUTTON,
            },
            WindowsAndMessaging::{
                ChangeWindowMessageFilterEx, CreateIconIndirect, RegisterWindowMessageW, HICON,
                ICONINFO, MSGFLT_ALLOW, SIZE_MINIMIZED, WM_COMMAND, WM_SHOWWINDOW, WM_SIZE,
            },
        },
    },
};

const SUBCLASS_ID: usize = 0x4D4D_5442;
const PREVIOUS: u32 = 1001;
const TOGGLE: u32 = 1002;
const STOP: u32 = 1003;
const NEXT: u32 = 1004;

struct Icons {
    previous: HICON,
    play: HICON,
    pause: HICON,
    stop: HICON,
    next: HICON,
}

struct Slot {
    list: ITaskbarList3,
    icons: Icons,
    added: bool,
    applied: ThumbFlags,
}

thread_local! {
    static SLOT: RefCell<Option<Slot>> = const { RefCell::new(None) };
}

static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
static CREATED: AtomicU32 = AtomicU32::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static ADDING: AtomicBool = AtomicBool::new(false);
static HWND_RAW: AtomicIsize = AtomicIsize::new(0);
static REQUESTED: Mutex<Option<ThumbFlags>> = Mutex::new(None);
static COMPLAINED: AtomicBool = AtomicBool::new(false);
static MINIMIZED: AtomicBool = AtomicBool::new(false);

pub fn attach(app: &tauri::AppHandle, hwnd_ptr: *mut core::ffi::c_void) {
    let _ = APP.set(app.clone());
    HWND_RAW.store(hwnd_ptr as isize, Ordering::SeqCst);
    let hwnd = HWND(hwnd_ptr);
    unsafe {
        let name: Vec<u16> = "TaskbarButtonCreated"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let message = RegisterWindowMessageW(PCWSTR(name.as_ptr()));
        if message != 0 {
            CREATED.store(message, Ordering::SeqCst);
            let _ = ChangeWindowMessageFilterEx(hwnd, message, MSGFLT_ALLOW, None);
        }
        let installed = SetWindowSubclass(hwnd, Some(subclass), SUBCLASS_ID, 0);
        if installed.0 != 0 {
            INSTALLED.store(true, Ordering::SeqCst);
        } else {
            eprintln!("任务栏按钮窗口钩子安装失败");
        }
    }
    if let Err(error) = apply() {
        eprintln!("任务栏按钮初始化将等待系统通知: {error}");
    }
}

pub fn refresh_after_show() {
    let raw = HWND_RAW.load(Ordering::SeqCst);
    if raw == 0 {
        return;
    }
    let _ = mount(HWND(raw as *mut core::ffi::c_void), true);
}

pub fn detach() {
    if !INSTALLED.swap(false, Ordering::SeqCst) {
        return;
    }
    let raw = HWND_RAW.swap(0, Ordering::SeqCst);
    if raw == 0 {
        return;
    }
    unsafe {
        let _ = RemoveWindowSubclass(
            HWND(raw as *mut core::ffi::c_void),
            Some(subclass),
            SUBCLASS_ID,
        );
    }
}

pub fn sync(app: &tauri::AppHandle, flags: ThumbFlags) {
    {
        let mut requested = REQUESTED.lock().unwrap_or_else(|error| error.into_inner());
        if *requested == Some(flags) {
            return;
        }
        *requested = Some(flags);
    }
    let queued = app.run_on_main_thread(|| {
        if let Err(error) = apply() {
            if !COMPLAINED.swap(true, Ordering::SeqCst) {
                eprintln!("更新任务栏按钮失败: {error}");
            }
            if let Ok(mut requested) = REQUESTED.lock() {
                requested.take();
            }
        } else {
            COMPLAINED.store(false, Ordering::SeqCst);
        }
    });
    if queued.is_err() {
        if let Ok(mut requested) = REQUESTED.lock() {
            requested.take();
        }
    }
}

unsafe extern "system" fn subclass(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _id: usize,
    _data: usize,
) -> LRESULT {
    let created = CREATED.load(Ordering::SeqCst);
    if created != 0 && msg == created {
        if let Err(error) = mount(hwnd, true) {
            eprintln!("任务栏按钮创建通知后注册失败: {error}");
        }
    } else if msg == WM_SHOWWINDOW && wparam.0 != 0 {
        // A borderless window drops its thumbnail toolbar when it is shown again.
        let _ = mount(hwnd, true);
    } else if msg == WM_SIZE {
        let code = (wparam.0 & 0xffff) as u32;
        if code == SIZE_MINIMIZED {
            MINIMIZED.store(true, Ordering::SeqCst);
        } else if MINIMIZED.swap(false, Ordering::SeqCst) {
            let _ = mount(hwnd, true);
        }
    } else if msg == WM_COMMAND {
        let command = (wparam.0 & 0xffff) as u32;
        let notify = ((wparam.0 >> 16) & 0xffff) as u32;
        if notify == THBN_CLICKED {
            if let Some(request) = super::thumb_request(command) {
                if let Some(app) = APP.get() {
                    super::dispatch_playback(app, request);
                }
            }
            return LRESULT(0);
        }
    }
    DefSubclassProc(hwnd, msg, wparam, lparam)
}

fn apply() -> Result<(), String> {
    let raw = HWND_RAW.load(Ordering::SeqCst);
    if raw == 0 {
        return Err("主窗口句柄还没有登记".into());
    }
    let hwnd = HWND(raw as *mut core::ffi::c_void);
    if slot_added() {
        update_buttons(hwnd)
    } else {
        mount(hwnd, false)
    }
}

fn requested() -> ThumbFlags {
    REQUESTED
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .unwrap_or_default()
}

fn mount(hwnd: HWND, force: bool) -> Result<(), String> {
    if ADDING.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let result = (|| {
        ensure_slot()?;
        let flags = requested();
        let prepared = SLOT.with(|cell| {
            let slot = cell.borrow();
            let slot = slot.as_ref()?;
            Some((slot.list.clone(), buttons_for(&slot.icons, flags)))
        });
        let Some((list, buttons)) = prepared else {
            return Err("任务栏按钮尚未创建".into());
        };
        let already = slot_added();
        if already && !force {
            return push_buttons(hwnd, flags);
        }
        match unsafe { list.ThumbBarAddButtons(hwnd, &buttons) } {
            Ok(()) => {
                mark_added(flags);
                Ok(())
            }
            Err(error) if already => push_buttons(hwnd, flags).map_err(|_| error.to_string()),
            Err(error) => Err(error.to_string()),
        }
    })();
    ADDING.store(false, Ordering::SeqCst);
    result
}

fn mark_added(flags: ThumbFlags) {
    SLOT.with(|cell| {
        if let Some(slot) = cell.borrow_mut().as_mut() {
            slot.added = true;
            slot.applied = flags;
        }
    });
}

fn push_buttons(hwnd: HWND, flags: ThumbFlags) -> Result<(), String> {
    let prepared = SLOT.with(|cell| {
        let slot = cell.borrow();
        let slot = slot.as_ref()?;
        Some((slot.list.clone(), buttons_for(&slot.icons, flags)))
    });
    let Some((list, buttons)) = prepared else {
        return Err("任务栏按钮尚未创建".into());
    };
    unsafe { list.ThumbBarUpdateButtons(hwnd, &buttons) }.map_err(|error| error.to_string())?;
    SLOT.with(|cell| {
        if let Some(slot) = cell.borrow_mut().as_mut() {
            slot.added = true;
            slot.applied = flags;
        }
    });
    Ok(())
}

fn update_buttons(hwnd: HWND) -> Result<(), String> {
    let flags = requested();
    let prepared = SLOT.with(|cell| {
        let slot = cell.borrow();
        let slot = slot.as_ref()?;
        if !slot.added || slot.applied == flags {
            return None;
        }
        Some((slot.list.clone(), buttons_for(&slot.icons, flags)))
    });
    let Some((list, buttons)) = prepared else {
        return Ok(());
    };
    unsafe { list.ThumbBarUpdateButtons(hwnd, &buttons) }.map_err(|error| error.to_string())?;
    SLOT.with(|cell| {
        if let Some(slot) = cell.borrow_mut().as_mut() {
            slot.applied = flags;
        }
    });
    Ok(())
}

fn slot_added() -> bool {
    SLOT.with(|cell| cell.borrow().as_ref().is_some_and(|slot| slot.added))
}

fn ensure_slot() -> Result<(), String> {
    if SLOT.with(|cell| cell.borrow().is_some()) {
        return Ok(());
    }
    let list: ITaskbarList3 = unsafe {
        CoCreateInstance(
            &TaskbarList,
            None::<&windows::core::IUnknown>,
            CLSCTX_INPROC_SERVER,
        )
    }
    .map_err(|error| error.to_string())?;
    unsafe { list.HrInit() }.map_err(|error| error.to_string())?;
    let icons = create_icons()?;
    SLOT.with(|cell| {
        *cell.borrow_mut() = Some(Slot {
            list,
            icons,
            added: false,
            applied: ThumbFlags::default(),
        });
    });
    Ok(())
}

fn buttons_for(icons: &Icons, flags: ThumbFlags) -> [THUMBBUTTON; 4] {
    let toggle_icon = if flags.playing {
        icons.pause
    } else {
        icons.play
    };
    let tip = |id| super::thumb_tip(id, flags.playing).unwrap_or("");
    [
        button(PREVIOUS, icons.previous, tip(PREVIOUS), flags.skip),
        button(TOGGLE, toggle_icon, tip(TOGGLE), flags.toggle),
        button(STOP, icons.stop, tip(STOP), flags.stop),
        button(NEXT, icons.next, tip(NEXT), flags.skip),
    ]
}

fn button(id: u32, icon: HICON, tip: &str, enabled: bool) -> THUMBBUTTON {
    THUMBBUTTON {
        dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
        iId: id,
        hIcon: icon,
        szTip: tip_units(tip),
        dwFlags: if enabled { THBF_ENABLED } else { THBF_DISABLED },
        ..THUMBBUTTON::default()
    }
}

fn tip_units(text: &str) -> [u16; 260] {
    let mut units = [0u16; 260];
    for (index, unit) in text.encode_utf16().take(259).enumerate() {
        units[index] = unit;
    }
    units
}

fn create_icons() -> Result<Icons, String> {
    Ok(Icons {
        previous: hicon(ThumbGlyph::Previous)?,
        play: hicon(ThumbGlyph::Play)?,
        pause: hicon(ThumbGlyph::Pause)?,
        stop: hicon(ThumbGlyph::Stop)?,
        next: hicon(ThumbGlyph::Next)?,
    })
}

fn hicon(glyph: ThumbGlyph) -> Result<HICON, String> {
    icon_from_rgba(&super::thumb_rgba(glyph))
}

fn icon_from_rgba(rgba: &[u8; 1024]) -> Result<HICON, String> {
    let mut bgra = [0u8; 1024];
    for index in (0..1024).step_by(4) {
        bgra[index] = rgba[index + 2];
        bgra[index + 1] = rgba[index + 1];
        bgra[index + 2] = rgba[index];
        bgra[index + 3] = rgba[index + 3];
    }
    let info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: 16,
            biHeight: -16,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            biSizeImage: 1024,
            ..BITMAPINFOHEADER::default()
        },
        ..BITMAPINFO::default()
    };
    let mut bits: *mut core::ffi::c_void = std::ptr::null_mut();
    let color = unsafe {
        CreateDIBSection(
            HDC(std::ptr::null_mut()),
            &info,
            DIB_RGB_COLORS,
            &mut bits,
            HANDLE(std::ptr::null_mut()),
            0,
        )
    }
    .map_err(|error| error.to_string())?;
    if bits.is_null() {
        let _ = unsafe { DeleteObject(color) };
        return Err("图标像素缓冲区为空".into());
    }
    unsafe { std::ptr::copy_nonoverlapping(bgra.as_ptr(), bits.cast(), 1024) };
    let mask_bits = [0u8; 32];
    let mask = unsafe { CreateBitmap(16, 16, 1, 1, Some(mask_bits.as_ptr().cast())) };
    if mask.is_invalid() {
        let _ = unsafe { DeleteObject(color) };
        return Err("图标蒙版创建失败".into());
    }
    let icon_info = ICONINFO {
        fIcon: TRUE,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: mask,
        hbmColor: color,
    };
    let icon = unsafe { CreateIconIndirect(&icon_info) };
    let _ = unsafe { DeleteObject(color) };
    let _ = unsafe { DeleteObject(mask) };
    icon.map_err(|error| error.to_string())
}
