pub mod commands;
pub mod modules;

use crate::commands::TweakContext;
use crate::modules::cpu::get_cpu_tweaks;
use crate::modules::debloat::get_debloat_tweaks;
use crate::modules::display::get_display_tweaks;
use crate::modules::gaming::get_gaming_tweaks;
use crate::modules::gpu::get_gpu_tweaks;
use crate::modules::hardware::get_hardware_tweaks;
use crate::modules::network::get_network_tweaks;
use crate::modules::privacy::get_privacy_tweaks;
use crate::modules::security::get_security_tweaks;
use crate::modules::storage::get_storage_tweaks;
// use crate::modules::startup::get_startup_tweaks;
use crate::modules::system::get_system_tweaks;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut all_tweaks = Vec::new();
    all_tweaks.extend(get_network_tweaks());
    all_tweaks.extend(get_security_tweaks());
    all_tweaks.extend(get_privacy_tweaks());
    all_tweaks.extend(get_cpu_tweaks());
    all_tweaks.extend(get_debloat_tweaks());
    all_tweaks.extend(get_display_tweaks());
    all_tweaks.extend(get_system_tweaks());
    // all_tweaks.extend(get_startup_tweaks()); // Startup is now a separate manager
    all_tweaks.extend(get_gaming_tweaks());
    all_tweaks.extend(get_gpu_tweaks());
    all_tweaks.extend(get_hardware_tweaks());
    all_tweaks.extend(get_storage_tweaks());

    // Load initial state
    let state_path = crate::modules::utils::dirs::get_state_path()
        .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));
    let app_state = crate::modules::utils::state::AppState::load(state_path).unwrap_or_default();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(TweakContext { tweaks: all_tweaks }))
        .manage(Mutex::new(app_state))
        .invoke_handler(tauri::generate_handler![
            commands::check_is_admin,
            commands::get_tweaks,
            commands::apply_tweak,
            commands::undo_tweak,
            commands::benchmark_dns,
            commands::apply_dns_server,
            commands::scan_startup,
            commands::set_startup_item_enabled,
            crate::modules::storage::commands::scan_storage,
            crate::modules::storage::commands::compress_folder,
            crate::modules::storage::commands::decompress_folder,
            crate::modules::storage::commands::get_compressed_folders,
            crate::modules::storage::commands::get_folder_stats
        ])
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
                use windows::Win32::Graphics::Dwm::{
                    DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
                };

                // It's possible get_webview_window returns None if window is not yet created, but usually fine in setup
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(hwnd) = window.hwnd() {
                        unsafe {
                            let preference = DWMWCP_ROUND;
                            let _ = DwmSetWindowAttribute(
                                hwnd,
                                DWMWA_WINDOW_CORNER_PREFERENCE,
                                &preference as *const _ as *const _,
                                std::mem::size_of::<u32>() as u32,
                            );
                        }
                    }
                }
            }

            // Global Mouse Hook (rdev)
            use std::thread;
            use tauri::{Emitter, Manager};

            let app_handle = app.handle().clone();

            thread::spawn(move || {
                // Listen for global mouse events
                // rdev::listen blocks, so we run in thread
                if let Err(error) = rdev::listen(move |event| {
                    if let rdev::EventType::ButtonPress(button) = event.event_type {
                        // Check if main window is focused to avoid interfering with other apps
                        if let Some(window) = app_handle.get_webview_window("main") {
                            if window.is_focused().unwrap_or(false) {
                                match button {
                                    // On Windows, XBUTTON1 is often Unknown(1) and XBUTTON2 is Unknown(2) in rdev
                                    rdev::Button::Unknown(1) => {
                                        let _ = window.emit("app-navigation", "back");
                                    }
                                    rdev::Button::Unknown(2) => {
                                        let _ = window.emit("app-navigation", "forward");
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }) {
                    eprintln!("Error in global mouse hook: {:?}", error);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
