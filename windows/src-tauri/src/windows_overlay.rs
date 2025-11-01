use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_CLOAK, DWMWA_EXCLUDED_FROM_PEEK};
use windows::Win32::Graphics::Gdi::COLORREF;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, VK_SPACE};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetLayeredWindowAttributes, SetWindowDisplayAffinity, SetWindowLongPtrW,
    SetWindowPos, ShowWindow, GWL_EXSTYLE, HWND_TOPMOST, LWA_ALPHA, SW_HIDE, SW_SHOW,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, WDA_EXCLUDEFROMCAPTURE, WINDOW_EX_STYLE,
    WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
};

static HOTKEY_ID: i32 = 0xC1AA;
static HOTKEY_REGISTERED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

fn ensure_success(result: BOOL, context: &'static str) -> Result<()> {
    if result == BOOL(0) {
        Err(anyhow!("{}", context))
    } else {
        Ok(())
    }
}

/// Apply the Win32 hints that keep the overlay stealthy during screen-sharing.
pub fn apply_overlay_hints(hwnd: HWND) -> Result<()> {
    unsafe {
        let current = WINDOW_EX_STYLE(GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32);
        let mut desired = current;
        desired |= WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
        desired &= !WS_EX_APPWINDOW;

        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, desired.0 as isize);

        // Keep the overlay in the Alt+Tab list suppressed and mark it as topmost.
        ensure_success(
            SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER,
            ),
            "SetWindowPos failed",
        )?;

        ensure_success(
            SetLayeredWindowAttributes(hwnd, COLORREF(0), 1, LWA_ALPHA),
            "SetLayeredWindowAttributes failed",
        )?;

        ensure_success(
            SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE),
            "SetWindowDisplayAffinity rejected the request",
        )?;

        // Cloak for legacy peeks (Aero Peek/taskbar preview).
        let cloak: u32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_CLOAK,
            &cloak as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_EXCLUDED_FROM_PEEK,
            &cloak as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );

        Ok(())
    }
}

/// Register a global hotkey (Ctrl+Shift+Space) to toggle the overlay while debugging.
pub fn register_toggle_hotkey(hwnd: HWND) -> Result<()> {
    unsafe {
        let mut guard = HOTKEY_REGISTERED.lock().unwrap();
        if *guard {
            return Ok(());
        }

        let modifiers = MOD_CONTROL.0 | MOD_SHIFT.0 | MOD_NOREPEAT.0;

        ensure_success(
            RegisterHotKey(
                hwnd,
                HOTKEY_ID,
                modifiers,
                VK_SPACE.0 as u32,
            ),
            "RegisterHotKey failed",
        )?;

        *guard = true;
        Ok(())
    }
}

pub fn unregister_toggle_hotkey(hwnd: HWND) {
    unsafe {
        let mut guard = HOTKEY_REGISTERED.lock().unwrap();
        if *guard {
            let _ = UnregisterHotKey(hwnd, HOTKEY_ID);
            *guard = false;
        }
    }
}

pub fn show_overlay(hwnd: HWND) -> Result<()> {
    unsafe {
        ensure_success(ShowWindow(hwnd, SW_SHOW), "ShowWindow failed")?;
        ensure_success(
            SetLayeredWindowAttributes(hwnd, COLORREF(0), 230, LWA_ALPHA),
            "SetLayeredWindowAttributes failed",
        )?;
        Ok(())
    }
}

pub fn hide_overlay(hwnd: HWND) -> Result<()> {
    unsafe {
        ensure_success(
            SetLayeredWindowAttributes(hwnd, COLORREF(0), 1, LWA_ALPHA),
            "SetLayeredWindowAttributes failed",
        )?;
        ensure_success(ShowWindow(hwnd, SW_HIDE), "ShowWindow failed")?;
        Ok(())
    }
}
