use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// System Maintenance & Storage Privacy
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // Configure Storage Sense (auto cleanup)
        Tweak {
            id: "priv_config_storage_sense".to_string(),
            category: TweakCategory::Privacy,
            name: "Configure Storage Sense".to_string(),
            description: "Enables Storage Sense to auto-clean temp files monthly without touching Downloads or Recycle Bin.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(),
                    key: "01".to_string(),
                    value: RegistryValue::DWord(0), // Disable Storage Sense
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(),
                key: "01".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "01".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "1024".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "2048".to_string(), value: RegistryValue::DWord(30) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "04".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "32".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "02".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "128".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "08".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy".to_string(), key: "256".to_string(), value: RegistryValue::DWord(0) }
            ]
        },

        // Disable Reserved Storage (DISM)
        Tweak {
            id: "priv_disable_reserved_storage".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Reserved Storage".to_string(),
            description: "Disables Windows reserved storage for updates (saves ~7GB disk space).".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "dism".to_string(),
                    args: vec!["/Online".to_string(), "/Set-ReservedStorageState".to_string(), "/State:Enabled".to_string()],
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "dism".to_string(),
                args: vec!["/Online".to_string(), "/Get-ReservedStorageState".to_string()],
                contains: "Disabled".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "dism".to_string(),
                    args: vec!["/Online".to_string(), "/Set-ReservedStorageState".to_string(), "/State:Disabled".to_string()],
                }
            ]
        },

        // Disable Automatic Maintenance WakeUp
        Tweak {
            id: "priv_disable_maintenance_wakeup".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Maintenance Wake-up".to_string(),
            description: "Prevents Windows from waking your PC for automatic maintenance.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Task Scheduler\\Maintenance".to_string(),
                    key: "WakeUp".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Task Scheduler\\Maintenance".to_string(),
                key: "WakeUp".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Task Scheduler\\Maintenance".to_string(),
                    key: "WakeUp".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
    ]
}
