use anyhow::{anyhow, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetLayeredWindowAttributes, SetWindowDisplayAffinity, SetWindowLongPtrW,
    SetWindowPos, ShowWindow, GWL_EXSTYLE, HWND_TOPMOST, LWA_ALPHA, SW_SHOW,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, WDA_EXCLUDEFROMCAPTURE, WINDOW_EX_STYLE,
    WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
};

static OVERLAY_DIMMED: AtomicBool = AtomicBool::new(false);

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
            SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA),
            "SetLayeredWindowAttributes failed",
        )?;

        ensure_success(
            SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE),
            "SetWindowDisplayAffinity rejected the request",
        )?;

        Ok(())
    }
}

pub fn show_overlay(hwnd: HWND) -> Result<()> {
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
        OVERLAY_DIMMED.store(false, Ordering::SeqCst);
        ensure_success(
            SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA),
            "SetLayeredWindowAttributes failed",
        )?;
        Ok(())
    }
}

pub fn hide_overlay(hwnd: HWND) -> Result<()> {
    unsafe {
        ensure_success(
            SetLayeredWindowAttributes(hwnd, 0, 80, LWA_ALPHA),
            "SetLayeredWindowAttributes failed",
        )?;
        let _ = ShowWindow(hwnd, SW_SHOW);
        OVERLAY_DIMMED.store(true, Ordering::SeqCst);
        Ok(())
    }
}

pub fn is_dimmed() -> bool {
    OVERLAY_DIMMED.load(Ordering::SeqCst)
}
