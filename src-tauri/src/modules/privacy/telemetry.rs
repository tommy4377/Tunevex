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
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
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

        // Disable Scheduled Telemetry Tasks
        Tweak {
            id: "priv_disable_telemetry_tasks".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Telemetry Scheduled Tasks".to_string(),
            description: "Disables Microsoft Compatibility Appraiser, ProgramDataUpdater, and other telemetry tasks.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\Application Experience\\Microsoft Compatibility Appraiser".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\Application Experience\\ProgramDataUpdater".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\Autochk\\Proxy".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\Customer Experience Improvement Program\\UsbCeip".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\DiskDiagnostic\\Microsoft-Windows-DiskDiagnosticDataCollector".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\Feedback\\Siuf\\DmClient".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\Feedback\\Siuf\\DmClientOnScenarioDownload".to_string(), "/ENABLE".to_string()] },
                TweakOperation::Command { cmd: "schtasks".to_string(), args: vec!["/Change".to_string(), "/TN".to_string(), "\\Microsoft\\Windows\\PI\\Sqm-Tasks".to_string(), "/ENABLE".to_string()] },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::ScheduledTaskDisabled { name: "\\Microsoft\\Windows\\Application Experience\\Microsoft Compatibility Appraiser".to_string() }),
            operations: vec![
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Application Experience".to_string(), name: "Microsoft Compatibility Appraiser".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Application Experience".to_string(), name: "ProgramDataUpdater".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Autochk".to_string(), name: "Proxy".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Customer Experience Improvement Program".to_string(), name: "Consolidator".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Customer Experience Improvement Program".to_string(), name: "UsbCeip".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\DiskDiagnostic".to_string(), name: "Microsoft-Windows-DiskDiagnosticDataCollector".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Feedback\\Siuf".to_string(), name: "DmClient".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Feedback\\Siuf".to_string(), name: "DmClientOnScenarioDownload".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\PI".to_string(), name: "Sqm-Tasks".to_string() }
            ]
        },
        // Disable NVIDIA Telemetry
        Tweak {
            id: "priv_disable_nvidia_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable NVIDIA Telemetry".to_string(),
            description: "Disables NVIDIA Telemetry Container service and scheduled tasks (NvTmMon, NvTmRep).".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "NvTelemetryContainer".to_string(), mode: "Auto".to_string() },
                TweakOperation::Command { cmd: "sc".to_string(), args: vec!["start".to_string(), "NvTelemetryContainer".to_string()] },
                TweakOperation::ScheduledTaskEnable { path: "\\".to_string(), name: "NvTmMon".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\".to_string(), name: "NvTmRep".to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::ServiceDisabled { name: "NvTelemetryContainer".to_string() }),
            operations: vec![
                TweakOperation::ServiceDisable { name: "NvTelemetryContainer".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\".to_string(), name: "NvTmMon".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\".to_string(), name: "NvTmRep".to_string() },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\NvControlPanel2\\Client".to_string(), key: "OptInOrOutPreference".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID44231".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID64640".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID66610".to_string(), value: RegistryValue::DWord(0) }
            ]
        },
    ]
}
