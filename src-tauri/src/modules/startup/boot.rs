use crate::modules::startup::types::{AutostartSource, SafetyRating, StartupItem};
use crate::modules::startup::{assess_safety, utils};
use winreg::enums::*;
use winreg::RegKey;

pub fn scan() -> Vec<StartupItem> {
    let mut items = Vec::new();
    items.extend(scan_boot_execute());
    items.extend(scan_appinit());
    items.extend(scan_winlogon());
    items.extend(scan_ifeo()); // Image File Execution Options
    items
}

fn scan_boot_execute() -> Vec<StartupItem> {
    let mut items = Vec::new();
    // HKLM\SYSTEM\CurrentControlSet\Control\Session Manager -> BootExecute (MultiString)
    if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager")
    {
        if let Ok(val) = key.get_value::<Vec<String>, _>("BootExecute") {
            for v in val {
                items.push(StartupItem {
                    id: format!("BOOT:{}", v),
                    name: v.clone(),
                    category: "Boot".to_string(),
                    subcategory: "BootExecute".to_string(),
                    location: "HKLM\\...\\Session Manager".to_string(),
                    command: v,
                    enabled: true,
                    publisher: Some("Microsoft".to_string()),
                    description: Some("Native Boot Program".to_string()),
                    source: AutostartSource::Registry,
                    safety_rating: SafetyRating::Safe, // Usually autochek
                    file_exists: true,
                });
            }
        }
    }
    items
}

fn scan_appinit() -> Vec<StartupItem> {
    let mut items = Vec::new();
    let paths = vec![
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows",
            "AppInit_DLLs",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Wow6432Node\Microsoft\Windows NT\CurrentVersion\Windows",
            "AppInit_DLLs (32-bit)",
        ),
    ];

    for (hive, path, subcat) in paths {
        let root = RegKey::predef(hive);
        if let Ok(key) = root.open_subkey(path) {
            if let Ok(val) = key.get_value::<String, _>("AppInit_DLLs") {
                if !val.is_empty() {
                    // Can be comma or space separated
                    let dlls: Vec<&str> = val.split([',', ' ']).filter(|s| !s.is_empty()).collect();
                    for dll in dlls {
                        let (publ, desc) = utils::get_file_info(dll);
                        items.push(StartupItem {
                            id: format!("APPINIT:{}:{}", subcat, dll),
                            name: dll.to_string(),
                            category: "AppInit".to_string(),
                            subcategory: subcat.to_string(),
                            location: path.to_string(),
                            command: dll.to_string(),
                            enabled: true,
                            publisher: publ,
                            description: desc,
                            source: AutostartSource::Registry,
                            safety_rating: SafetyRating::Unknown, // High risk location
                            file_exists: true,
                        });
                    }
                }
            }
        }
    }
    items
}

fn scan_winlogon() -> Vec<StartupItem> {
    let mut items = Vec::new();
    // Shell and Userinit
    let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon";
    if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(path) {
        for val_name in ["Shell", "Userinit"] {
            if let Ok(val) = key.get_value::<String, _>(val_name) {
                // Shell is normally "explorer.exe", Userinit is "C:\Windows\system32\userinit.exe,"
                // Anything extra is suspicious
                let parts: Vec<&str> = val
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();
                for part in parts {
                    let (publ, desc) = utils::get_file_info(part);
                    let rating = assess_safety(part, publ.as_deref());

                    items.push(StartupItem {
                        id: format!("WINLOGON:{}:{}", val_name, part),
                        name: part.to_string(),
                        category: "Winlogon".to_string(),
                        subcategory: val_name.to_string(),
                        location: path.to_string(),
                        command: part.to_string(),
                        enabled: true,
                        publisher: publ,
                        description: desc,
                        source: AutostartSource::Registry,
                        safety_rating: rating,
                        file_exists: true,
                    });
                }
            }
        }
    }
    items
}

fn scan_ifeo() -> Vec<StartupItem> {
    let mut items = Vec::new();
    let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options";
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = root.open_subkey(path) {
        for key_name in key.enum_keys().flatten() {
            if let Ok(sub) = key.open_subkey(&key_name) {
                if let Ok(debugger) = sub.get_value::<String, _>("Debugger") {
                    // HIJACK detected
                    let (publ, desc) = utils::get_file_info(&debugger);

                    items.push(StartupItem {
                        id: format!("IFEO:{}", key_name),
                        name: key_name.clone(),
                        category: "ImageHijack".to_string(),
                        subcategory: "Debugger Hijack".to_string(),
                        location: format!("{}\\{}", path, key_name),
                        command: debugger.clone(),
                        enabled: true,
                        publisher: publ,
                        description: desc,
                        source: AutostartSource::Registry,
                        safety_rating: SafetyRating::Dangerous, // Usually malicious if not dev tool
                        file_exists: true,
                    });
                }
            }
        }
    }
    items
}

pub fn toggle_boot_item(id: &str, enable: bool) -> Result<(), String> {
    if id.starts_with("IFEO:") {
        toggle_ifeo_item(id, enable)
    } else if id.starts_with("APPINIT:") {
        toggle_appinit_item(id, enable)
    } else if id.starts_with("BOOT:") {
        toggle_bootexec_item(id, enable)
    } else {
        Err(format!("Unknown boot item type: {}", id))
    }
}

fn toggle_ifeo_item(id: &str, enable: bool) -> Result<(), String> {
    let exe_name = id.strip_prefix("IFEO:").ok_or("Invalid IFEO ID format")?;

    let path = format!(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{}",
        exe_name
    );

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm
        .open_subkey_with_flags(&path, KEY_ALL_ACCESS)
        .map_err(|e| format!("Cannot open IFEO key: {}", e))?;

    if enable {
        // Re-enabling IFEO is dangerous - refuse
        return Err("Cannot re-enable IFEO debugger hijacks - too dangerous. \
                    If this was legitimate software, reinstall it."
            .to_string());
    }

    // Backup before deletion
    if let Ok(debugger) = key.get_value::<String, _>("Debugger") {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let backup_path = format!(r"Software\TommyTweaker\Backups\IFEO\{}", exe_name);
        if let Ok((backup_key, _)) = hkcu.create_subkey(&backup_path) {
            let _ = backup_key.set_value("Debugger", &debugger);
        }
    }

    // Delete the debugger value
    key.delete_value("Debugger")
        .map_err(|e| format!("Cannot remove debugger: {}", e))?;

    Ok(())
}

fn toggle_appinit_item(id: &str, enable: bool) -> Result<(), String> {
    let is_32bit = id.contains("(32-bit)");
    let path = if is_32bit {
        r"SOFTWARE\Wow6432Node\Microsoft\Windows NT\CurrentVersion\Windows"
    } else {
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows"
    };

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey_with_flags(path, KEY_ALL_ACCESS) {
        if enable {
            return Err("Cannot re-enable AppInit_DLLs - too dangerous".to_string());
        }

        // Backup
        if let Ok(current) = key.get_value::<String, _>("AppInit_DLLs") {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let backup_path = if is_32bit {
                r"Software\TommyTweaker\Backups\AppInit32"
            } else {
                r"Software\TommyTweaker\Backups\AppInit64"
            };
            if let Ok((backup_key, _)) = hkcu.create_subkey(backup_path) {
                let _ = backup_key.set_value("AppInit_DLLs", &current);
            }
        }

        // Clear
        key.set_value("AppInit_DLLs", &"")
            .map_err(|e| e.to_string())?;
        let _ = key.set_value("LoadAppInit_DLLs", &0u32); // Disable loading
        Ok(())
    } else {
        Err("Could not open AppInit registry key".to_string())
    }
}

fn toggle_bootexec_item(_id: &str, _enable: bool) -> Result<(), String> {
    Err("BootExecute items cannot be toggled safely. \
         Use 'msconfig' or 'autoruns' for manual editing."
        .to_string())
}

// ============================================
// BOOT TWEAKS - Part 2 additions
// ============================================

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_boot_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "boot_ignore_failures".to_string(),
            category: TweakCategory::StartupServices,
            name: "Disable Auto Repair on Boot Failure".to_string(),
            description: "Prevents Windows from entering the automatic repair loop after a crash or hard reset. Boots normally instead of showing the recovery screen.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "bcdedit".to_string(),
                args: vec!["/enum".to_string(), "{current}".to_string()],
                contains: "bootstatuspolicy".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "bcdedit".to_string(),
                args: vec!["/deletevalue".to_string(), "{current}".to_string(), "bootstatuspolicy".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "bcdedit".to_string(),
                args: vec!["/set".to_string(), "{current}".to_string(), "bootstatuspolicy".to_string(), "IgnoreAllFailures".to_string()],
            }],
        },
        Tweak {
            id: "boot_highest_mode".to_string(),
            category: TweakCategory::StartupServices,
            name: "Maximum Resolution in Boot Environment".to_string(),
            description: "Forces the UEFI boot environment and Safe Mode to use the maximum supported resolution instead of low-resolution fallback.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "bcdedit".to_string(),
                args: vec!["/enum".to_string(), "{globalsettings}".to_string()],
                contains: "highestmode".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "bcdedit".to_string(),
                args: vec!["/deletevalue".to_string(), "{globalsettings}".to_string(), "highestmode".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "bcdedit".to_string(),
                args: vec!["/set".to_string(), "{globalsettings}".to_string(), "highestmode".to_string(), "true".to_string()],
            }],
        },
        Tweak {
            id: "boot_disable_animations".to_string(),
            category: TweakCategory::StartupServices,
            name: "Disable Boot Logo & Loading Spinner".to_string(),
            description: "Hides the OEM manufacturer logo and Windows loading spinner during boot. Uses undocumented BCD flags from AtlasOS.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "bcdedit".to_string(),
                args: vec!["/enum".to_string(), "{globalsettings}".to_string()],
                contains: "16000067".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "{globalsettings}".to_string(), "custom:16000067".to_string()],
                },
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "{globalsettings}".to_string(), "custom:16000069".to_string()],
                },
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "{globalsettings}".to_string(), "custom:16000067".to_string(), "true".to_string()],
                },
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "{globalsettings}".to_string(), "custom:16000069".to_string(), "true".to_string()],
                },
            ],
        },
        Tweak {
            id: "boot_f8_legacy_menu".to_string(),
            category: TweakCategory::StartupServices,
            name: "Enable F8 Legacy Boot Menu".to_string(),
            description: "Restores the classic F8 Advanced Boot Options text menu (Safe Mode, Debug Mode, etc.) disabled since Windows 8.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "bcdedit".to_string(),
                args: vec!["/enum".to_string(), "{current}".to_string()],
                contains: "legacy".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "bcdedit".to_string(),
                args: vec!["/set".to_string(), "{current}".to_string(), "bootmenupolicy".to_string(), "standard".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "bcdedit".to_string(),
                args: vec!["/set".to_string(), "{current}".to_string(), "bootmenupolicy".to_string(), "legacy".to_string()],
            }],
        },
        Tweak {
            id: "boot_disable_fast_startup".to_string(),
            category: TweakCategory::StartupServices,
            name: "Disable Fast Startup".to_string(),
            description: "Disables Windows hybrid boot. Fast Startup causes issues with dual boot, hardware detection, and Windows Update. Ensures a clean full kernel reload on every boot.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\Session Manager\Power".to_string(),
                key: "HiberbootEnabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\Session Manager\Power".to_string(),
                key: "HiberbootEnabled".to_string(),
                value: RegistryValue::DWord(1),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\Session Manager\Power".to_string(),
                key: "HiberbootEnabled".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
        Tweak {
            id: "boot_real_time_universal".to_string(),
            category: TweakCategory::StartupServices,
            name: "Hardware Clock = UTC (Dual Boot Fix)".to_string(),
            description: "Sets the hardware clock to UTC. Required when dual-booting with Linux to prevent the system clock from being wrong after switching OS.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\TimeZoneInformation".to_string(),
                key: "RealTimeIsUniversal".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\TimeZoneInformation".to_string(),
                key: "RealTimeIsUniversal".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\TimeZoneInformation".to_string(),
                key: "RealTimeIsUniversal".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "boot_verbose_status".to_string(),
            category: TweakCategory::StartupServices,
            name: "Verbose Boot/Shutdown Status Messages".to_string(),
            description: "Shows detailed status messages (e.g. 'Stopping Windows Update...') during boot and shutdown instead of just the spinning circle.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System".to_string(),
                key: "VerboseStatus".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System".to_string(),
                key: "VerboseStatus".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System".to_string(),
                key: "VerboseStatus".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "boot_modern_standby_network".to_string(),
            category: TweakCategory::StartupServices,
            name: "Disable Network on Modern Standby (S0)".to_string(),
            description: "Prevents the network adapter from staying active during S0 modern sleep. Fixes laptops overheating in bags while suspended.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powercfg".to_string(),
                args: vec!["/q".to_string(), "SCHEME_CURRENT".to_string(), "f15576e8-98b7-4186-b944-eafa664402d9".to_string(), "f15576e8-98b7-4186-b944-eafa664402d9".to_string()],
                contains: "0x00000000".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setacvalueindex".to_string(), "SCHEME_CURRENT".to_string(), "f15576e8-98b7-4186-b944-eafa664402d9".to_string(), "f15576e8-98b7-4186-b944-eafa664402d9".to_string(), "1".to_string()],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setacvalueindex".to_string(), "SCHEME_CURRENT".to_string(), "f15576e8-98b7-4186-b944-eafa664402d9".to_string(), "f15576e8-98b7-4186-b944-eafa664402d9".to_string(), "0".to_string()],
                },
                TweakOperation::Command {
                    cmd: "powercfg".to_string(),
                    args: vec!["/setactive".to_string(), "SCHEME_CURRENT".to_string()],
                },
            ],
        },
    ]
}
