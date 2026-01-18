pub mod commands;
pub mod modules;

use crate::commands::TweakContext;
use crate::modules::activation::get_activation_tweaks;
use crate::modules::cpu::get_cpu_tweaks;
use crate::modules::debloat::get_debloat_tweaks;
use crate::modules::display::get_display_tweaks;
// use crate::modules::filesystem::get_filesystem_tweaks; // Migrated to storage
use crate::modules::gaming::get_gaming_tweaks;
use crate::modules::gpu::get_gpu_tweaks;
// use crate::modules::hardware::get_hardware_tweaks; // Removed
use crate::modules::input::get_input_tweaks;
use crate::modules::network::get_network_tweaks;
use crate::modules::privacy::get_privacy_tweaks;
use crate::modules::security::get_security_tweaks;
use crate::modules::storage::get_storage_tweaks;
// use crate::modules::startup::get_startup_tweaks;
use crate::modules::interface::get_interface_tweaks;
use crate::modules::system::get_system_tweaks;
use std::os::windows::process::CommandExt;
use std::sync::Mutex;

#[tauri::command]
fn init_explorerpatcher_stealth() -> Result<String, String> {
    // Run in a detached thread to prevent ANY blocking of the main app/UI
    std::thread::spawn(|| {
        let script = r#"
$ErrorActionPreference = 'SilentlyContinue'

# 1. CHECK INSTALLATION
$ep_ui = "${env:ProgramFiles}\ExplorerPatcher\ExplorerPatcherUI.exe"
$installed = Test-Path $ep_ui

if (!$installed) {
    # Download & Install if missing
    try {
        $releases = Invoke-RestMethod 'https://api.github.com/repos/valinet/ExplorerPatcher/releases/latest'
        $url = ($releases.assets | ? { $_.name -eq 'ep_setup.exe' }).browser_download_url
        $temp = "$env:TEMP\ep_setup.exe"
        Invoke-WebRequest $url -OutFile $temp -UseBasicParsing
        
        # Try silent install args. Even if EP ignores them, 'Start-Process' without -Wait or with -WindowStyle Hidden
        # prevents the app calling this from freezing. We use -Wait here inside the background thread.
        Start-Process $temp -ArgumentList "/quiet","/norestart" -Wait -WindowStyle Hidden
        Remove-Item $temp -Force
        
        # Defender Exclusions
        Add-MpPreference -ExclusionPath "${env:ProgramFiles}\ExplorerPatcher","$env:APPDATA\ExplorerPatcher"
    } catch {
        Write-Output "Install failed: $_"
    }
}

# 2. CHECK & APPLY CONFIG (Idempotent)
$reg = 'HKCU:\Software\ExplorerPatcher'
if (!(Test-Path $reg)) { New-Item $reg -Force | Out-Null }

$needs_restart = $false
if (!$installed) { $needs_restart = $true }

# Function to safely set key and track changes
function Set-EpKey($name, $val) {
    $current = (Get-ItemProperty $reg $name -EA 0).$name
    if ($current -ne $val) {
        Set-ItemProperty $reg $name $val -Type DWord
        return $true
    }
    return $false
}

if (Set-EpKey 'TaskbarStyle' 1) { $needs_restart = $true }              # 1=Win10
if (Set-EpKey 'DisableTaskbarContextMenu' 1) { $needs_restart = $true } # No Menu
if (Set-EpKey 'HideFromTaskbar' 1) { $needs_restart = $true }           # No Tray
# Default size check (only set if missing or different? User tweak controls this usually)
# We set a sensible default (Small) only if not set? 
# The prompt asked for "Set-ItemProperty ... TaskbarIconSize 16". We enforce it on init.
if (Set-EpKey 'TaskbarIconSize' 16) { $needs_restart = $true }

# 3. RESTART EXPLORER ONLY IF NEEDED
if ($needs_restart) {
    Stop-Process -Name 'explorer','ep_*','ExplorerPatcher*' -Force
    Start-Sleep 2
    if (!(Get-Process explorer -EA 0)) {
        Start-Process 'explorer.exe' -WindowStyle Hidden
    }
}
"#;

        // Execute detached
        let _ = std::process::Command::new("powershell.exe")
            .args(["-ExecutionPolicy", "Bypass", "-Command", script])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW (Windows specific, ignored on Linux but this app is Win target)
            .output();
    });

    Ok("ExplorerPatcher init started in background".to_string())
}

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
    all_tweaks.extend(get_input_tweaks());
    // all_tweaks.extend(get_hardware_tweaks()); // Removed
    all_tweaks.extend(get_storage_tweaks());
    all_tweaks.extend(get_interface_tweaks());
    // all_tweaks.extend(get_filesystem_tweaks()); // Migrated to storage
    all_tweaks.extend(get_activation_tweaks());

    // Load initial state
    let state_path = crate::modules::utils::dirs::get_state_path()
        .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));
    let app_state = crate::modules::utils::state::AppState::load(state_path).unwrap_or_default();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(TweakContext { tweaks: all_tweaks }))
        .manage(Mutex::new(app_state))
        .manage(Mutex::new(
            crate::modules::storage::state::CompactorState::default(),
        ))
        .manage(Mutex::new(
            crate::modules::system::monitoring::SystemMonitor::new(),
        ))
        .manage(Mutex::new(
            crate::modules::utils::process_manager::ProcessManager::new(),
        ))
        .invoke_handler(tauri::generate_handler![
            init_explorerpatcher_stealth,
            commands::check_is_admin,
            commands::get_tweaks,
            commands::get_tweaks_fast,
            commands::check_category,
            commands::apply_tweak,
            commands::undo_tweak,
            commands::kill_tweak_process,
            commands::benchmark_dns,
            commands::apply_dns_server,
            commands::scan_startup,
            commands::set_startup_item_enabled,
            crate::modules::storage::commands::scan_storage,
            crate::modules::storage::commands::compress_folder,
            crate::modules::storage::commands::decompress_folder,
            crate::modules::storage::commands::cancel_compactor,
            crate::modules::storage::commands::get_compressed_folders,
            crate::modules::storage::commands::get_folder_stats,
            crate::modules::system::restore::create_restore_point,
            crate::modules::system::restore::open_restore_ui,
            crate::modules::system::monitoring::get_system_stats,
            crate::modules::system::monitoring::get_quick_stats,
            crate::modules::system::monitoring::get_disk_stats,
            crate::modules::system::maintenance::empty_recycle_bin,
            crate::modules::system::maintenance::clear_temp_files,
            crate::modules::system::maintenance::flush_dns_cache,
            crate::modules::system::maintenance::reset_network
        ])
        .setup(|app| {
            {
                use tauri::Manager;
                use windows::Win32::Graphics::Dwm::{
                    DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
                };

                // It's possible get_webview_window returns None if window is not yet created, but usually fine in setup
                if let Some(window) = app.get_webview_window("main") {
                    // Apply rounded corners
                    if let Ok(hwnd) = window.hwnd() {
                        unsafe {
                            let preference = DWMWCP_ROUND;
                            let result = DwmSetWindowAttribute(
                                hwnd,
                                DWMWA_WINDOW_CORNER_PREFERENCE,
                                &preference as *const _ as *const _,
                                std::mem::size_of::<u32>() as u32,
                            );
                            if let Err(e) = result {
                                eprintln!("DWM Rounding Failed: {:?}", e);
                            }
                        }
                    }

                    // Try Legacy Acrylic (Force Persistence via Undocumented API)
                    use crate::modules::legacy_vibrancy::apply_legacy_acrylic;
                    // Color: (18, 18, 18, 0) - Fully transparent tint to let the blur be "light"
                    if let Err(e) = apply_legacy_acrylic(&window, (18, 18, 18, 0)) {
                        eprintln!("Legacy Acrylic failed: {}", e);

                        // Fallback to Standard Modern Acrylic (Transient)
                        use window_vibrancy::apply_acrylic;
                        if let Err(e) = apply_acrylic(&window, Some((18, 18, 18, 125))) {
                            eprintln!("Modern Acrylic failed: {}", e);
                            use window_vibrancy::apply_mica;
                            let _ = apply_mica(&window, Some(true));
                        }
                    } else {
                        println!("Legacy Acrylic applied (Persistent).");
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
