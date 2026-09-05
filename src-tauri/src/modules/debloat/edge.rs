//! Microsoft Edge debloat tweaks

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "debloat_edge_sidebar".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Edge Sidebar & Discover".to_string(),
            description: "Disables Edge sidebar, Discover button, and related bloat features.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "HubsSidebarEnabled".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "WebWidgetAllowed".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "EdgeShoppingAssistantEnabled".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(),
                key: "HubsSidebarEnabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "HubsSidebarEnabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "WebWidgetAllowed".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "EdgeShoppingAssistantEnabled".to_string(), value: RegistryValue::DWord(0) },
            ]
        },

        Tweak {
            id: "debloat_edge_startup".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Edge First Run & Welcome".to_string(),
            description: "Disables Edge first run experience and welcome page.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "HideFirstRunExperience".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "RunStartUpSystemCheck".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(),
                key: "HideFirstRunExperience".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "HideFirstRunExperience".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "RunStartUpSystemCheck".to_string(), value: RegistryValue::DWord(0) },
            ]
        },

        Tweak {
            id: "debloat_edge_sync".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Edge Sync & Cloud Features".to_string(),
            description: "Disables Edge sync, collections, and cloud features.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "SyncDisabled".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "EdgeCollectionsEnabled".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(),
                key: "SyncDisabled".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "SyncDisabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "EdgeCollectionsEnabled".to_string(), value: RegistryValue::DWord(0) },
            ]
        },

        Tweak {
            id: "debloat_edge_telemetry".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Edge Telemetry".to_string(),
            description: "Disables Edge telemetry, usage data, and personalization.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "PersonalizationReportingEnabled".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "UserFeedbackAllowed".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "MetricsReportingEnabled".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(),
                key: "MetricsReportingEnabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "PersonalizationReportingEnabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "UserFeedbackAllowed".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge".to_string(), key: "MetricsReportingEnabled".to_string(), value: RegistryValue::DWord(0) },
            ]
        },

        Tweak {
            id: "debloat_edge_autostart".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Edge Auto-Start".to_string(),
            description: "Prevents Edge from starting automatically at login.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge\\Main".to_string(), key: "PreventFirstRunPage".to_string() },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run".to_string(), key: "MicrosoftEdgeAutoLaunch".to_string(), value: RegistryValue::Binary(vec![0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]) },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Edge\\Main".to_string(),
                key: "PreventFirstRunPage".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Edge\\Main".to_string(), key: "PreventFirstRunPage".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::Command { cmd: "reg".to_string(), args: vec!["delete".to_string(), "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run".to_string(), "/v".to_string(), "MicrosoftEdgeAutoLaunch".to_string(), "/f".to_string()] },
            ]
        },
    ]
}
