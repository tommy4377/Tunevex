use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Windows Core Telemetry
/// From: disallow-data-collection.yml, disable-diagnostic-tracing.yml, disable-ceip.yml
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Comprehensive Windows Telemetry Disable
        // From: disallow-data-collection.yml
        // ============================================
        Tweak {
            id: "priv_disable_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Windows Telemetry".to_string(),
            description: "Sets AllowTelemetry=0 in all locations, disables DiagTrack service, clears telemetry logs.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "MaxTelemetryAllowed".to_string(),
                },
                // Delete Policy Keys to restore default behavior
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowDeviceNameInTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                },
                // Re-enable EventTranscript
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Diagnostics\\DiagTrack\\EventTranscriptKey".to_string(),
                    key: "EnableEventTranscript".to_string(),
                    value: RegistryValue::DWord(1),
                },
                // Re-enable DiagTrack Autologger
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\WMI\\Autologger\\Diagtrack-Listener".to_string(),
                    key: "Start".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                key: "AllowTelemetry".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                // AllowTelemetry in DataCollection (multiple locations)
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "MaxTelemetryAllowed".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowDeviceNameInTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                // Disable EventTranscript
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Diagnostics\\DiagTrack\\EventTranscriptKey".to_string(),
                    key: "EnableEventTranscript".to_string(),
                    value: RegistryValue::DWord(0),
                },
                // Disable DiagTrack Autologger
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\WMI\\Autologger\\Diagtrack-Listener".to_string(),
                    key: "Start".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },
        
        // CEIP - kept in privacy (registry-based)
        Tweak {
            id: "priv_disable_ceip".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Customer Experience Improvement Program".to_string(),
            description: "Disables Windows CEIP data collection via registry.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\SQMClient\\Windows".to_string(),
                key: "CEIPEnable".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Disable Windows Error Reporting
        Tweak {
            id: "priv_disable_wer".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Windows Error Reporting".to_string(),
            description: "Prevents Windows from sending error reports to Microsoft.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::ServiceSetMode {
                    name: "WerSvc".to_string(),
                    mode: "Manual".to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                key: "Disabled".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::ServiceDisable {
                    name: "WerSvc".to_string(),
                }
            ]
        },
        
        // Disable Input Telemetry
        Tweak {
            id: "priv_disable_input_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Input Telemetry".to_string(),
            description: "Disables inking and typing data collection for personalization.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitInkCollection".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitTextCollection".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization\\TrainedDataStore".to_string(),
                    key: "HarvestContacts".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Personalization\\Settings".to_string(),
                    key: "AcceptedPrivacyPolicy".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                key: "RestrictImplicitInkCollection".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitInkCollection".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitTextCollection".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization\\TrainedDataStore".to_string(),
                    key: "HarvestContacts".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Personalization\\Settings".to_string(),
                    key: "AcceptedPrivacyPolicy".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // Note: NVIDIA Telemetry is consolidated in priv_nvidia_telemetry (privacy/apps.rs)
        // Keeping only that one to avoid duplicate conflicts
    ]
}
