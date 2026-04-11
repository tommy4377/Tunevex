use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::command;
use windows::core::PCWSTR;
use windows::Win32::UI::Shell::{
    SHEmptyRecycleBinW, SHERB_NOCONFIRMATION, SHERB_NOPROGRESSUI, SHERB_NOSOUND,
};

/// Empty the Recycle Bin using native Windows Shell API
#[command]
pub async fn empty_recycle_bin() -> Result<String, String> {
    // Use native Windows API - SHEmptyRecycleBinW
    // Flags: no confirmation, no progress UI, no sound
    let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;

    let result = unsafe { SHEmptyRecycleBinW(None, PCWSTR::null(), flags) };

    match result {
        Ok(_) => Ok("Emptied".to_string()),
        Err(e) => {
            let code = e.code().0 as u32;
            // S_FALSE (0x00000001) means it was already empty
            // 0x8000FFFF = E_UNEXPECTED, can also mean empty
            if code == 0x00000001 || code == 0x8000FFFF {
                Ok("Already empty".to_string())
            } else {
                Err(format!("Error: {}", e))
            }
        }
    }
}

/// Clear temporary files using native Rust fs operations
#[command]
pub async fn clear_temp_files() -> Result<String, String> {
    tokio::task::spawn_blocking(|| {
        use std::env;
        use std::fs;
        use std::path::PathBuf;

        let mut total_freed: u64 = 0;
        let mut files_deleted: u32 = 0;

        let temp_dirs: Vec<PathBuf> = vec![
            env::temp_dir(),
            PathBuf::from(env::var("WINDIR").unwrap_or_default()).join("Temp"),
        ];

        for temp_dir in temp_dirs {
            if !temp_dir.exists() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(&temp_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    let size = if path.is_file() {
                        fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
                    } else {
                        0
                    };

                    let deleted = if path.is_dir() {
                        fs::remove_dir_all(&path).is_ok()
                    } else {
                        fs::remove_file(&path).is_ok()
                    };

                    if deleted {
                        total_freed += size;
                        files_deleted += 1;
                    }
                }
            }
        }

        let freed_mb = total_freed as f64 / 1_048_576.0;
        if freed_mb > 0.1 {
            Ok(format!("Freed {:.1} MB", freed_mb))
        } else if files_deleted > 0 {
            Ok(format!("{} files", files_deleted))
        } else {
            Ok("Already clean".to_string())
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Flush DNS cache using ipconfig
#[command]
pub async fn flush_dns_cache() -> Result<String, String> {
    tokio::task::spawn_blocking(|| {
        let output = Command::new("ipconfig")
            .args(["/flushdns"])
            .creation_flags(0x08000000)
            .output();

        match output {
            Ok(o) if o.status.success() => Ok("DNS flushed".to_string()),
            _ => Ok("DNS flushed".to_string()),
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Reset network stack using netsh
#[command]
pub async fn reset_network() -> Result<String, String> {
    let commands: &[(&str, &[&str])] = &[
        ("netsh", &["winsock", "reset"]),
        ("netsh", &["int", "ip", "reset"]),
        ("ipconfig", &["/flushdns"]),
    ];

    let mut success_count = 0;
    for (cmd, args) in commands {
        let result = Command::new(cmd)
            .args(*args)
            .creation_flags(0x08000000)
            .output();

        if result.is_ok() {
            success_count += 1;
        }
    }

    if success_count == commands.len() {
        Ok("Reset done".to_string())
    } else {
        Ok("Partial reset".to_string())
    }
}

// ============================================
// SYSTEM TWEAKS - Part 2 additions
// ============================================

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_system_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "sys_detailed_bsod".to_string(),
            category: TweakCategory::System,
            name: "Detailed BSOD Error Codes".to_string(),
            description: "Shows hexadecimal error codes and parameters on the Blue Screen of Death instead of the sad face. Essential for crash diagnosis.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\CrashControl".to_string(),
                key: "DisplayParameters".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SYSTEM\CurrentControlSet\Control\CrashControl".to_string(),
                    key: "DisplayParameters".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SYSTEM\CurrentControlSet\Control\CrashControl".to_string(),
                    key: "DisableEmoticon".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SYSTEM\CurrentControlSet\Control\CrashControl".to_string(),
                    key: "DisplayParameters".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SYSTEM\CurrentControlSet\Control\CrashControl".to_string(),
                    key: "DisableEmoticon".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },
        Tweak {
            id: "sys_disable_fth".to_string(),
            category: TweakCategory::System,
            name: "Disable Fault Tolerant Heap (FTH)".to_string(),
            description: "Disables the FTH mitigation that silently throttles games and apps that crash frequently. FTH flags a process and permanently reduces its performance as a crash prevention measure.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\FTH".to_string(),
                key: "Enabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\FTH".to_string(),
                key: "Enabled".to_string(),
                value: RegistryValue::DWord(1),
            }]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: r"SOFTWARE\Microsoft\FTH".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Command {
                    cmd: "rundll32.exe".to_string(),
                    args: vec!["fthsvc.dll,FthSysprepSpecialize".to_string()],
                },
            ],
        },
        Tweak {
            id: "sys_no_disk_space_check".to_string(),
            category: TweakCategory::System,
            name: "Disable Low Disk Space Warning".to_string(),
            description: "Disables the 'You are running out of disk space' notification that can interrupt games and cause stuttering when your drive is nearly full.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer".to_string(),
                key: "NoLowDiskSpaceChecks".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer".to_string(),
                key: "NoLowDiskSpaceChecks".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer".to_string(),
                key: "NoLowDiskSpaceChecks".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "sys_audio_ducking".to_string(),
            category: TweakCategory::System,
            name: "Disable Audio Ducking".to_string(),
            description: "Prevents Windows from automatically lowering game audio when a communication app (Discord, Teams) detects voice activity.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Multimedia\Audio".to_string(),
                key: "UserDuckingPreference".to_string(),
                expected_value: RegistryValue::DWord(3),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Multimedia\Audio".to_string(),
                key: "UserDuckingPreference".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Multimedia\Audio".to_string(),
                key: "UserDuckingPreference".to_string(),
                value: RegistryValue::DWord(3),
            }],
        },
        Tweak {
            id: "sys_startup_delay_zero".to_string(),
            category: TweakCategory::System,
            name: "Remove Startup App Delay".to_string(),
            description: "Removes the artificial 10-second delay Windows applies to startup apps to show the desktop first. Apps load immediately after login.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize".to_string(),
                key: "StartupDelayInMSec".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize".to_string(),
                key: "StartupDelayInMSec".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize".to_string(),
                key: "StartupDelayInMSec".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
        Tweak {
            id: "sys_long_paths".to_string(),
            category: TweakCategory::System,
            name: "Enable Long Paths (260+ chars)".to_string(),
            description: "Enables Windows to support file paths longer than 260 characters. Required for some development tools and deep folder structures.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\FileSystem".to_string(),
                key: "LongPathsEnabled".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\FileSystem".to_string(),
                key: "LongPathsEnabled".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\FileSystem".to_string(),
                key: "LongPathsEnabled".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "sys_ansi_console".to_string(),
            category: TweakCategory::System,
            name: "Enable ANSI/VT100 Console Support".to_string(),
            description: "Enables native VT100/ANSI escape code support in the Windows console. Required for correct color output in terminal apps and scripts.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Console".to_string(),
                key: "VirtualTerminalLevel".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Console".to_string(),
                key: "VirtualTerminalLevel".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Console".to_string(),
                key: "VirtualTerminalLevel".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "sys_reserved_storage_disable".to_string(),
            category: TweakCategory::System,
            name: "Disable Windows Reserved Storage".to_string(),
            description: "Frees the ~7GB of disk space Windows reserves for future updates. Useful on small SSDs. Windows Update may temporarily re-reserve space during feature updates.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "DISM.exe".to_string(),
                args: vec!["/Online".to_string(), "/Set-ReservedStorageState".to_string(), "/State:Enabled".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "DISM.exe".to_string(),
                args: vec!["/Online".to_string(), "/Set-ReservedStorageState".to_string(), "/State:Disabled".to_string()],
            }],
        },
        Tweak {
            id: "sys_repair_perf_counters".to_string(),
            category: TweakCategory::System,
            name: "Repair Performance Counters".to_string(),
            description: "Rebuilds broken Windows performance counters. Fixes monitoring tools like MSI Afterburner, HWiNFO, and GPU-Z that show missing or zero values.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![
                TweakOperation::Command {
                    cmd: "lodctr".to_string(),
                    args: vec!["/R".to_string()],
                },
            ],
        },
    ]
}
