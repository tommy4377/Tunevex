use crate::modules::types::{
    FileOp, RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType,
    WarningLevel,
};

/// Application Telemetry (NVIDIA, Office, VS, Chrome, Firefox, etc.)
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // NVIDIA Telemetry (from scripts + research)
        // ============================================
        Tweak {
            id: "priv_nvidia_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable NVIDIA Telemetry".to_string(),
            description: "Disables NVIDIA telemetry: services, scheduled tasks, registry keys. From privacy.sexy scripts.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\NvControlPanel2\\Client".to_string(), key: "OptInOrOutPreference".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID44231".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID64640".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID66610".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm\\Global\\Startup".to_string(), key: "SendTelemetryData".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::ServiceSetMode { name: "NvTelemetryContainer".to_string(), mode: "Auto".to_string() },
                TweakOperation::Command { cmd: "sc".to_string(), args: vec!["start".to_string(), "NvTelemetryContainer".to_string()] },
                TweakOperation::ScheduledTaskEnable { path: "\\".to_string(), name: "NvTmMon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\".to_string(), name: "NvTmRep_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\".to_string(), name: "NvTmRepOnLogon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}".to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::ServiceDisabled { name: "NvTelemetryContainer".to_string() }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\NvControlPanel2\\Client".to_string(), key: "OptInOrOutPreference".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID44231".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID64640".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\NVIDIA Corporation\\Global\\FTS".to_string(), key: "EnableRID66610".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm\\Global\\Startup".to_string(), key: "SendTelemetryData".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::ServiceDisable { name: "NvTelemetryContainer".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\".to_string(), name: "NvTmMon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\".to_string(), name: "NvTmRep_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\".to_string(), name: "NvTmRepOnLogon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}".to_string() },
                TweakOperation::Command { cmd: "cmd".to_string(), args: vec!["/c".to_string(), "rmdir".to_string(), "/s".to_string(), "/q".to_string(), "%ProgramData%\\NVIDIA Corporation\\NVTelemetry".to_string()] },
                TweakOperation::Command { cmd: "cmd".to_string(), args: vec!["/c".to_string(), "rmdir".to_string(), "/s".to_string(), "/q".to_string(), "%ProgramData%\\NVIDIA Corporation\\CrashDumps".to_string()] }
            ]
        },
        
        // Office Telemetry
        Tweak {
            id: "priv_office_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Microsoft Office Telemetry".to_string(),
            description: "Disables Office telemetry, CEIP, and customer data collection.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "sendcustomerdata".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\common\\clienttelemetry".to_string(),
                    key: "sendtelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "qmenable".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Office\\Common\\ClientTelemetry".to_string(),
                    key: "DisableTelemetry".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                key: "sendcustomerdata".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "sendcustomerdata".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\common\\clienttelemetry".to_string(),
                    key: "sendtelemetry".to_string(),
                    value: RegistryValue::DWord(3),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "qmenable".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Office\\Common\\ClientTelemetry".to_string(),
                    key: "DisableTelemetry".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Visual Studio Telemetry
        Tweak {
            id: "priv_vs_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Visual Studio Telemetry".to_string(),
            description: "Disables Visual Studio telemetry, VSCEIP, feedback, and IntelliCode collection.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\14.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\15.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\16.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\17.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\14.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\15.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\16.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\17.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\SQM".to_string(), key: "OptIn".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\VisualStudio\\Telemetry".to_string(), key: "TurnOffSwitch".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\Feedback".to_string(), key: "DisableFeedbackDialog".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\Feedback".to_string(), key: "DisableEmailInput".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\Feedback".to_string(), key: "DisableScreenshotCapture".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\IntelliCode".to_string(), key: "DisableRemoteAnalysis".to_string() },
                TweakOperation::ServiceSetMode { name: "VSStandardCollectorService150".to_string(), mode: "Manual".to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\SQM".to_string(),
                key: "OptIn".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\14.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\15.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\16.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Wow6432Node\\Microsoft\\VSCommon\\17.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\14.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\15.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\16.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Microsoft\\VSCommon\\17.0\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\SQM".to_string(), key: "OptIn".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\VisualStudio\\Telemetry".to_string(), key: "TurnOffSwitch".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\Feedback".to_string(), key: "DisableFeedbackDialog".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\Feedback".to_string(), key: "DisableEmailInput".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\Feedback".to_string(), key: "DisableScreenshotCapture".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\VisualStudio\\IntelliCode".to_string(), key: "DisableRemoteAnalysis".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::ServiceDisable { name: "VSStandardCollectorService150".to_string() }
            ]
        },
        
        // VS Code Telemetry
        Tweak {
            id: "priv_vscode_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable VS Code Telemetry".to_string(),
            description: "Modifies VS Code settings.json to disable telemetry, crash reports, and experiments.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::FileOperation(FileOp::Write {
                    path: "%APPDATA%\\Code\\User\\settings.json".to_string(),
                    content: r#"{
    "telemetry.telemetryLevel": "all",
    "telemetry.enableTelemetry": true,
    "telemetry.enableCrashReporter": true,
    "workbench.enableExperiments": true,
    "update.showReleaseNotes": true
}"#.to_string(),
                })
            ]),
            tweak_type: TweakType::Action, enabled: false,
            check: None,
            operations: vec![
                TweakOperation::FileOperation(FileOp::Write {
                    path: "%APPDATA%\\Code\\User\\settings.json".to_string(),
                    content: r#"{
    "telemetry.telemetryLevel": "off",
    "telemetry.enableTelemetry": false,
    "telemetry.enableCrashReporter": false,
    "workbench.enableExperiments": false,
    "update.showReleaseNotes": false
}"#.to_string(),
                })
            ]
        },
        
        // .NET CLI Telemetry
        Tweak {
            id: "priv_dotnet_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable .NET CLI Telemetry".to_string(),
            description: "Sets DOTNET_CLI_TELEMETRY_OPTOUT environment variable.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment".to_string(), key: "DOTNET_CLI_TELEMETRY_OPTOUT".to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment".to_string(),
                key: "DOTNET_CLI_TELEMETRY_OPTOUT".to_string(),
                expected_value: RegistryValue::String("1".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment".to_string(), key: "DOTNET_CLI_TELEMETRY_OPTOUT".to_string(), value: RegistryValue::String("1".to_string()) }
            ]
        },
        
        // PowerShell Telemetry
        Tweak {
            id: "priv_powershell_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable PowerShell Telemetry".to_string(),
            description: "Sets POWERSHELL_TELEMETRY_OPTOUT environment variable.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment".to_string(), key: "POWERSHELL_TELEMETRY_OPTOUT".to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment".to_string(),
                key: "POWERSHELL_TELEMETRY_OPTOUT".to_string(),
                expected_value: RegistryValue::String("1".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment".to_string(), key: "POWERSHELL_TELEMETRY_OPTOUT".to_string(), value: RegistryValue::String("1".to_string()) }
            ]
        },
        
        // Chrome Telemetry
        Tweak {
            id: "priv_chrome_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Chrome Telemetry".to_string(),
            description: "Disables Chrome metrics reporting and software reporter tool.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "MetricsReportingEnabled".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupReportingEnabled".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupEnabled".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                key: "MetricsReportingEnabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "MetricsReportingEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupReportingEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Firefox Telemetry (registry for enterprise)
        Tweak {
            id: "priv_firefox_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Firefox Telemetry (Enterprise)".to_string(),
            description: "Sets Firefox enterprise policies to disable telemetry, studies, and crash reports.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableFirefoxStudies".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                key: "DisableTelemetry".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableTelemetry".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableFirefoxStudies".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // ============================================
        // Background Apps
        // ============================================
        Tweak {
            id: "priv_disable_background_apps".to_string(), // Renamed from cpu_
            category: TweakCategory::Privacy,
            name: "Disable Background Apps".to_string(),
            description: "Globally disables background app execution to minimize resource usage.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\BackgroundAccessApplications".to_string(),
                    key: "GlobalUserDisabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search".to_string(),
                    key: "BackgroundAppGlobalToggle".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\BackgroundAccessApplications".to_string(),
                key: "GlobalUserDisabled".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\BackgroundAccessApplications".to_string(),
                    key: "GlobalUserDisabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search".to_string(),
                    key: "BackgroundAppGlobalToggle".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
    ]
}
