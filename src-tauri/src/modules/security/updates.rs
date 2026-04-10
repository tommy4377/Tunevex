use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_update_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "sec_disable_windows_update".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Windows Update Services".to_string(),
            description: "Completely disables Windows Update services and tasks. WARNING: You will receive NO security updates!".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "wuauserv".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "UsoSvc".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "WaaSMedicSvc".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "BITS".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "DoSvc".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "uhssvc".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "InstallService".to_string(), mode: "Manual".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\InstallService".to_string(), name: "ScanForUpdates".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\InstallService".to_string(), name: "ScanForUpdatesAsUser".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\InstallService".to_string(), name: "SmartRetry".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\UpdateOrchestrator".to_string(), name: "Schedule Scan".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\WindowsUpdate".to_string(), name: "Scheduled Start".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsUpdate".to_string(), key: "DisableWindowsUpdateAccess".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MultiServiceDisabled {
                names: vec![
                    "wuauserv".to_string(),
                    "UsoSvc".to_string(),
                    "WaaSMedicSvc".to_string(),
                    "BITS".to_string(),
                    "DoSvc".to_string(),
                    "uhssvc".to_string(),
                    "InstallService".to_string(),
                ]
            }),
            operations: vec![
                TweakOperation::ServiceDisable { name: "wuauserv".to_string() },
                TweakOperation::ServiceDisable { name: "UsoSvc".to_string() },
                TweakOperation::ServiceDisable { name: "WaaSMedicSvc".to_string() },
                TweakOperation::ServiceDisable { name: "BITS".to_string() },
                TweakOperation::ServiceDisable { name: "DoSvc".to_string() },
                TweakOperation::ServiceDisable { name: "uhssvc".to_string() },
                TweakOperation::ServiceDisable { name: "InstallService".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\InstallService".to_string(), name: "ScanForUpdates".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\InstallService".to_string(), name: "ScanForUpdatesAsUser".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\InstallService".to_string(), name: "SmartRetry".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\UpdateOrchestrator".to_string(), name: "Schedule Scan".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\WindowsUpdate".to_string(), name: "Scheduled Start".to_string() },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsUpdate".to_string(), key: "DisableWindowsUpdateAccess".to_string(), value: RegistryValue::DWord(1) },
            ]
        },

        // ============================================
        // Disable Automatic Driver Installation
        // ============================================
        Tweak {
            id: "sec_disable_auto_driver_update".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Automatic Driver Updates".to_string(),
            description: "Prevents Windows Update from automatically installing drivers. Keeps your custom GPU/NIC drivers.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsUpdate".to_string(), key: "ExcludeWUDriversInQualityUpdate".to_string() },
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsUpdate".to_string(),
                key: "ExcludeWUDriversInQualityUpdate".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsUpdate".to_string(), key: "ExcludeWUDriversInQualityUpdate".to_string(), value: RegistryValue::DWord(1) },
            ]
        }
    ]
}
