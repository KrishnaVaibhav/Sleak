#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};

#[cfg(target_os = "windows")]
mod windows_overlay;

#[cfg(target_os = "windows")]
use tauri::WindowExtWindows;

#[tauri::command]
fn toggle_overlay(window: Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd();

        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            windows_overlay::hide_overlay(hwnd).map_err(|e| e.to_string())?;
            let _ = window.hide();
        } else {
            windows_overlay::show_overlay(hwnd).map_err(|e| e.to_string())?;
            let _ = window.show();
            let _ = window.set_focus();
        }
    }

    Ok(())
}

#[tauri::command]
fn enable_overlay(window: Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd();
        windows_overlay::show_overlay(hwnd).map_err(|e| e.to_string())?;
        let _ = window.show();
    }
    Ok(())
}

#[tauri::command]
fn disable_overlay(window: Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd();
        windows_overlay::hide_overlay(hwnd).map_err(|e| e.to_string())?;
        let _ = window.hide();
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            toggle_overlay,
            enable_overlay,
            disable_overlay
        ])
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                if let Some(window) = app.get_window("overlay") {
                    let hwnd = window.hwnd();
                    windows_overlay::apply_overlay_hints(hwnd)?;
                    windows_overlay::register_toggle_hotkey(hwnd)?;
                }
            }
            Ok(())
        })
        .on_window_event(|event| {
            #[cfg(target_os = "windows")]
            if let tauri::WindowEvent::Destroyed = event.event() {
                let hwnd = event.window().hwnd();
                windows_overlay::unregister_toggle_hotkey(hwnd);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
