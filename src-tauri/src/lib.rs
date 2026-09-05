pub mod commands;
pub mod modules;

use crate::modules::activation::get_activation_tweaks;
use crate::modules::cpu::get_cpu_tweaks;
use crate::modules::debloat::get_debloat_tweaks;
use crate::modules::display::get_display_tweaks;
use crate::modules::tweaks::TweakContext;
use crate::modules::types::{Tweak, TweakType, WarningLevel};
// use crate::modules::filesystem::get_filesystem_tweaks; // Migrated to storage
use crate::modules::gaming::get_gaming_tweaks;
use crate::modules::gpu::get_gpu_tweaks;
// use crate::modules::hardware::get_hardware_tweaks; // Removed
use crate::modules::input::get_input_tweaks;
use crate::modules::interface::get_interface_tweaks;
use crate::modules::network::get_network_tweaks;
use crate::modules::privacy::get_privacy_tweaks;
use crate::modules::security::get_security_tweaks;
use crate::modules::startup::boot::get_boot_tweaks;
use crate::modules::storage::get_storage_tweaks;
use crate::modules::system::get_system_tweaks;
use std::sync::Mutex;

/// Build the single catalog consumed by the UI, command layer, and AI.
///
/// Duplicate/conflicting entries stay excluded, but advanced controls remain
/// available to power users behind the application's dangerous-tweak guard.
fn build_tweak_catalog() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    tweaks.extend(get_network_tweaks());
    tweaks.extend(get_security_tweaks());
    tweaks.extend(get_privacy_tweaks());
    tweaks.extend(get_cpu_tweaks());
    tweaks.extend(get_debloat_tweaks());
    tweaks.extend(get_display_tweaks());
    tweaks.extend(get_system_tweaks());
    tweaks.extend(get_boot_tweaks());
    tweaks.extend(get_gaming_tweaks());
    tweaks.extend(get_gpu_tweaks());
    tweaks.extend(get_input_tweaks());
    tweaks.extend(get_storage_tweaks());
    tweaks.extend(get_interface_tweaks());
    tweaks.extend(get_activation_tweaks());

    const CATALOG_EXCLUSIONS: &[&str] = &[
        // Duplicate/conflicting implementations.
        "cpu_legacy_boot_menu",
        "cpu_disable_all_power_saving",
        "sys_disable_fth",
        "sys_game_priority",
        "privacy_powershell_telemetry",
        "privacy_disable_uwp_background",
        "sec_disable_wer",
        "sec_disable_wer_service",
        "sec_disable_problem_dialog",
        "sec_disable_corp_wer",
        "priv_all_in_one",
        "sec_block_driver_updates",
        "priv_disable_reserved_storage",
        "sec_disable_all_mitigations",
        "input_keyboard_speed",
        "net_disable_ipv6",
        "gaming_disable_visual_effects",
    ];
    tweaks.retain(|tweak| !CATALOG_EXCLUSIONS.contains(&tweak.id.as_str()));

    // These controls have legitimate diagnostic, lab, compatibility, or
    // specialist use cases, but can materially reduce security/stability or
    // remove Windows components. They must never be included by bulk "safe"
    // actions or AI recommendations and require an explicit acknowledgement.
    const POWER_USER_CONTROLS: &[&str] = &[
        "cpu_disable_vbs",
        "sec_disable_realtime",
        "sec_disable_cloud",
        "sec_disable_samples",
        "sec_dev_exclusions",
        "sec_disable_pua",
        "sec_disable_defender",
        "sec_disable_firewall",
        "sec_fw_whitelist",
        "sec_disable_cpu_mitigations",
        "sec_disable_dep",
        "sec_disable_hvci",
        "sec_disable_sehop",
        "sec_disable_cfg",
        "sec_uac_lower",
        "sec_uac_no_secure",
        "sec_disable_uac",
        "sec_uac_admin_mode",
        "sec_uac_auto_elevate",
        "sec_disable_smartscreen_apps",
        "sec_disable_smartscreen_edge",
        "sec_disable_smartscreen_store",
        "sec_disable_app_reputation",
        "sec_disable_protected_popup",
        "sec_disable_windows_update",
        "sec_disable_update_medic",
        "sec_disable_uac_virtualization",
        "sec_disable_auto_maintenance",
        "sec_disable_hello",
        "sec_disable_lockscreen",
        "priv_disable_smart_app_control",
        // Unsupported, misleading, or potentially destabilizing optimizations.
        "cpu_tsc_sync",
        "cpu_disable_dynamic_tick",
        "cpu_disable_hpet",
        "cpu_dpc_latency",
        "cpu_processor_check_interval",
        "cpu_disable_idle",
        "cpu_disable_acpi_devices",
        "cpu_disable_paging_executive",
        "cpu_disable_prefetch",
        "cpu_disable_svchost_split",
        "cpu_disable_page_combining",
        "cpu_intel_disable_tsx",
        "sys_irq8_priority",
        "sys_responsiveness",
        "sys_no_lazy_mode",
        "sys_gpu_mmcss_priority",
        "mem_large_system_cache",
        "mem_fixed_pagefile",
        "mem_disable_compression",
        "mem_svchost_split",
        "system_msi_global_safe",
        "display_timer_resolution",
        "display_enable_vrr",
        "display_no_gpu_scaling",
        "display_enable_hdr",
        "display_nvidia_low_latency",
        "display_8bit_color",
        "storage_native_nvme_driver",
        "storage_enable_write_cache",
        "storage_disable_scheduled_defrag",
        "storage_ntfs_tunneling",
        "debloat_remove_edge_full",
        "debloat_remove_store",
        "debloat_gaming",
        "ctx_merge_as_trustedinstaller",
        "priv_vscode_telemetry",
        "interface_auto_end_tasks",
        "input_keyboard_data_queue_size",
        "gaming_disable_mmcss",
        "gaming_disable_gameinput",
    ];
    for tweak in &mut tweaks {
        if POWER_USER_CONTROLS.contains(&tweak.id.as_str()) {
            tweak.warning_level = WarningLevel::Dangerous;
            tweak.description.push_str(power_user_risk_note(&tweak.id));
        }
    }

    // App/package removal cannot honestly promise rollback. Present it as a
    // one-shot, caution-level action rather than a stateful toggle.
    const IRREVERSIBLE_REMOVALS: &[&str] = &[
        "debloat_gaming",
        "debloat_remove_edge_full",
        "debloat_ms_common",
        "debloat_ms_comm",
        "debloat_ms_outlook",
        "debloat_ms_phonelink",
        "debloat_onedrive",
        "debloat_thirdparty",
        "debloat_hp",
        "debloat_dell",
        "debloat_lenovo",
        "debloat_asus",
        "debloat_msi",
        "debloat_acer",
        "debloat_razer",
        "debloat_norton",
    ];
    for tweak in &mut tweaks {
        if IRREVERSIBLE_REMOVALS.contains(&tweak.id.as_str()) {
            tweak.tweak_type = TweakType::Action;
            if matches!(tweak.warning_level, WarningLevel::Safe) {
                tweak.warning_level = WarningLevel::Careful;
            }
            tweak.check = None;
            tweak.revert_operations = None;
            tweak.description.push_str(
                " This removal is not automatically reversible; reinstall affected software manually if needed.",
            );
        }
    }

    tweaks
}

fn power_user_risk_note(id: &str) -> &'static str {
    match id {
        "sec_disable_realtime"
        | "sec_disable_cloud"
        | "sec_disable_samples"
        | "sec_dev_exclusions"
        | "sec_disable_pua"
        | "sec_disable_defender" => " POWER USER RISK: Reduces malware detection or creates scanning blind spots. Use only for a specific, temporary compatibility or test requirement and restore protection afterward.",
        "sec_disable_firewall" | "sec_fw_whitelist" => " POWER USER RISK: Reduces network traffic filtering. Use only on a controlled network with another firewall or a narrowly defined troubleshooting reason.",
        "sec_disable_cpu_mitigations"
        | "sec_disable_dep"
        | "sec_disable_hvci"
        | "sec_disable_sehop"
        | "sec_disable_cfg"
        | "cpu_disable_vbs"
        | "priv_disable_smart_app_control" => " POWER USER RISK: Disables an exploit or code-integrity boundary. This can improve compatibility in narrow cases but materially increases compromise risk.",
        "sec_uac_lower"
        | "sec_uac_no_secure"
        | "sec_disable_uac"
        | "sec_uac_admin_mode"
        | "sec_uac_auto_elevate"
        | "sec_disable_uac_virtualization" => " POWER USER RISK: Weakens UAC isolation or elevation prompts. Apply only when the resulting elevation behavior is explicitly required and understood.",
        "sec_disable_smartscreen_apps"
        | "sec_disable_smartscreen_edge"
        | "sec_disable_smartscreen_store"
        | "sec_disable_app_reputation"
        | "sec_disable_protected_popup" => " POWER USER RISK: Removes reputation-based warnings for downloaded or untrusted content. Verify software provenance independently before applying.",
        "sec_disable_windows_update" | "sec_disable_update_medic" => " POWER USER RISK: Prevents or impairs security and reliability updates. Intended only for tightly managed servicing windows; re-enable promptly.",
        "cpu_tsc_sync"
        | "cpu_disable_dynamic_tick"
        | "cpu_disable_hpet"
        | "cpu_dpc_latency"
        | "cpu_processor_check_interval"
        | "cpu_disable_idle"
        | "cpu_disable_acpi_devices"
        | "display_timer_resolution" => " POWER USER RISK: Changes timing, boot, interrupt, power, or ACPI behavior. Benefits are hardware/workload-specific and regressions may include instability, higher power use, or boot problems.",
        "cpu_disable_paging_executive"
        | "cpu_disable_prefetch"
        | "cpu_disable_svchost_split"
        | "cpu_disable_page_combining"
        | "cpu_intel_disable_tsx"
        | "sys_irq8_priority"
        | "sys_responsiveness"
        | "sys_no_lazy_mode"
        | "sys_gpu_mmcss_priority"
        | "mem_large_system_cache"
        | "mem_fixed_pagefile"
        | "mem_disable_compression"
        | "mem_svchost_split"
        | "system_msi_global_safe" => " POWER USER RISK: Alters low-level scheduling, memory, cache, service-host, or interrupt policy. Measure before and after; revert if the target workload does not demonstrably improve.",
        "display_enable_vrr"
        | "display_no_gpu_scaling"
        | "display_enable_hdr"
        | "display_nvidia_low_latency"
        | "display_8bit_color" => " POWER USER RISK: Forces display or driver behavior that may be unsupported by the active GPU, monitor, or driver and can cause visual defects or instability.",
        "storage_native_nvme_driver"
        | "storage_enable_write_cache"
        | "storage_disable_scheduled_defrag"
        | "storage_ntfs_tunneling" => " POWER USER RISK: Changes the storage stack or maintenance policy. A crash or power loss may risk data, and disabling Optimize Drives also disables scheduled SSD retrim.",
        "debloat_remove_edge_full" | "debloat_remove_store" | "debloat_gaming" => " POWER USER RISK: Removes Windows packages and may break dependent apps, games, updates, or WebView content. Reinstallation may require Windows repair or the Microsoft Store.",
        "ctx_merge_as_trustedinstaller" => " POWER USER RISK: Adds a path for importing registry files with TrustedInstaller authority. A malicious or incorrect .reg file can compromise the operating system.",
        "priv_vscode_telemetry" => " POWER USER RISK: Rewrites VS Code's settings file. Back up custom settings before running this one-shot action.",
        "interface_auto_end_tasks" => " POWER USER RISK: Windows may terminate unresponsive applications during shutdown before they save data.",
        "input_keyboard_data_queue_size" => " POWER USER RISK: A very small keyboard input queue can drop input under heavy system load; use only with latency measurements.",
        "gaming_disable_mmcss" | "gaming_disable_gameinput" => " POWER USER RISK: Disables Windows gaming scheduling or input infrastructure and can break games, controllers, audio, or capture behavior.",
        "sec_disable_auto_maintenance" | "sec_disable_hello" | "sec_disable_lockscreen" => " POWER USER RISK: Disables a Windows maintenance or authentication feature and may reduce reliability or account protection.",
        _ => " POWER USER RISK: This advanced control can reduce system security, stability, compatibility, or recoverability. Apply only for a specific measured reason and with a recovery plan.",
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let all_tweaks = build_tweak_catalog();

    // Load initial state
    let state_path = crate::modules::utils::dirs::get_state_path()
        .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));
    let app_state = crate::modules::utils::state::AppState::load(state_path).unwrap_or_default();

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init());

    #[cfg(debug_assertions)]
    let builder = builder.plugin(
        tauri_plugin_mcp_bridge::Builder::new()
            .bind_address("127.0.0.1")
            .build(),
    );

    builder
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
            commands::check_is_admin,
            commands::get_tweaks,
            commands::get_tweaks_fast,
            commands::check_category,
            crate::modules::profiles::export_tweak_profile,
            crate::modules::profiles::preview_tweak_profile,
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
            crate::modules::system::maintenance::reset_network,
            commands::save_gemini_key,
            commands::get_gemini_key_status,
            commands::delete_gemini_key,
            commands::test_gemini_connection,
            commands::ai_analyze,
            commands::ai_chat,
            commands::ai_diagnose,
            commands::get_ai_memory_context,
            commands::record_ai_memory,
            commands::get_full_ai_memory,
            commands::clear_ai_memory,
            commands::save_chat,
            commands::list_chats,
            commands::load_chat,
            commands::delete_chat,
            commands::ai_scan_startup,
            commands::ai_apply_startup_recommendations,
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

#[cfg(test)]
mod catalog_tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn catalog_ids_are_unique_and_toggles_are_reversible() {
        let catalog = build_tweak_catalog();
        let mut ids = HashSet::new();
        for tweak in catalog {
            assert!(
                ids.insert(tweak.id.clone()),
                "duplicate tweak id: {}",
                tweak.id
            );
            if matches!(tweak.tweak_type, TweakType::Toggle) {
                assert!(
                    tweak.check.is_some(),
                    "toggle has no state check: {}",
                    tweak.id
                );
                assert!(
                    tweak.revert_operations.is_some(),
                    "toggle has no rollback: {}",
                    tweak.id
                );
            }
        }
    }

    #[test]
    fn registry_values_have_one_catalog_owner() {
        let catalog = build_tweak_catalog();
        let mut owners = std::collections::HashMap::<String, String>::new();
        let mut collisions = Vec::new();
        for tweak in catalog {
            for operation in &tweak.operations {
                let target = match operation {
                    crate::modules::types::TweakOperation::RegistrySet {
                        root_key,
                        path,
                        key,
                        ..
                    }
                    | crate::modules::types::TweakOperation::RegistryDelete {
                        root_key,
                        path,
                        key,
                    } => Some(format!(
                        "{}\\{}\\{}",
                        root_key.to_lowercase(),
                        path.to_lowercase(),
                        key.to_lowercase()
                    )),
                    _ => None,
                };
                if let Some(target) = target {
                    if let Some(owner) = owners.insert(target.clone(), tweak.id.clone()) {
                        if owner != tweak.id {
                            collisions.push(format!("{target}: {owner} / {}", tweak.id));
                        }
                    }
                }
            }
        }
        let documented_power_user_overlaps = [
            "hklm\\software\\microsoft\\windows\\currentversion\\policies\\system\\promptonsecuredesktop: sec_uac_lower / sec_uac_no_secure",
            "hklm\\software\\microsoft\\windows\\currentversion\\policies\\system\\consentpromptbehavioradmin: sec_uac_lower / sec_uac_admin_mode",
            "hklm\\system\\currentcontrolset\\control\\deviceguard\\scenarios\\hypervisorenforcedcodeintegrity\\enabled: sec_disable_hvci / cpu_disable_vbs",
        ];
        let unexpected = collisions
            .iter()
            .filter(|collision| !documented_power_user_overlaps.contains(&collision.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        assert!(
            unexpected.is_empty(),
            "unexpected conflicting registry ownership:\n{}",
            unexpected.join("\n")
        );
    }

    #[test]
    fn catalog_covers_every_ui_system_category() {
        let catalog = build_tweak_catalog();
        let mut categories = std::collections::HashMap::<String, usize>::new();
        for tweak in &catalog {
            *categories
                .entry(format!("{:?}", tweak.category))
                .or_default() += 1;
        }
        for expected in [
            "Network",
            "SecurityPrivacy",
            "Privacy",
            "CpuPerformance",
            "DebloatTelemetry",
            "DisplayMonitor",
            "System",
            "StartupServices",
            "GameOptimizations",
            "GpuOptimization",
            "MouseInput",
            "FileSystem",
            "InterfaceUx",
            "Activation",
        ] {
            assert!(
                categories.get(expected).is_some_and(|count| *count > 0),
                "empty UI category: {expected}"
            );
        }
        println!("catalog_total={} categories={categories:?}", catalog.len());
    }

    #[test]
    fn every_catalog_command_is_allow_listed() {
        for tweak in build_tweak_catalog() {
            for operation in tweak.operations.iter().chain(
                tweak
                    .revert_operations
                    .as_deref()
                    .unwrap_or_default()
                    .iter(),
            ) {
                if let crate::modules::types::TweakOperation::Command { cmd, .. } = operation {
                    crate::modules::utils::security::validate_command(cmd)
                        .unwrap_or_else(|error| panic!("{} uses {cmd}: {error}", tweak.id));
                }
            }
        }
    }

    #[test]
    fn dangerous_controls_are_explicitly_documented_power_user_controls() {
        let dangerous = build_tweak_catalog()
            .into_iter()
            .filter(|tweak| matches!(tweak.warning_level, WarningLevel::Dangerous))
            .map(|tweak| {
                assert!(
                    tweak.description.contains("POWER USER RISK:"),
                    "dangerous control has no explicit risk note: {}",
                    tweak.id
                );
                tweak.id
            })
            .collect::<Vec<_>>();
        assert!(
            !dangerous.is_empty(),
            "expected the advanced power-user catalog to contain guarded controls"
        );
    }
}
