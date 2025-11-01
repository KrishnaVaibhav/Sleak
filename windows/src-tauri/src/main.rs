#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{GlobalShortcutManager, Manager, Window};

#[cfg(target_os = "windows")]
mod windows_overlay;

#[tauri::command]
fn toggle_overlay(window: Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;

        if windows_overlay::is_dimmed() {
            windows_overlay::show_overlay(hwnd).map_err(|e| e.to_string())?;
        } else {
            windows_overlay::hide_overlay(hwnd).map_err(|e| e.to_string())?;
        }
        let _ = window.show();
        let _ = window.set_focus();
    }

    Ok(())
}

#[tauri::command]
fn enable_overlay(window: Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        windows_overlay::hide_overlay(hwnd).map_err(|e| e.to_string())?;
        let _ = window.show();
    }
    Ok(())
}

#[tauri::command]
fn disable_overlay(window: Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        windows_overlay::show_overlay(hwnd).map_err(|e| e.to_string())?;
        let _ = window.show();
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
                    let hwnd = window.hwnd()?;
                    windows_overlay::apply_overlay_hints(hwnd)?;
                    #[cfg(debug_assertions)]
                    {
                        windows_overlay::show_overlay(hwnd).ok();
                        let _ = window.show();
                    }
                }

                let handle = app.handle();
                let shortcut_handle = handle.clone();
                handle
                    .global_shortcut_manager()
                    .register("Ctrl+Shift+Space", move || {
                        if let Some(window) = shortcut_handle.get_window("overlay") {
                            let _ = toggle_overlay(window);
                        }
                    })?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
