use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Privacy Settings (activity, tracking, sync, permissions)
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // Activity History
        Tweak {
            id: "priv_disable_activity_history".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Activity History".to_string(),
            description: "Prevents Windows from collecting activity history and sending to Microsoft.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "EnableActivityFeed".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "PublishUserActivities".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "UploadUserActivities".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                key: "EnableActivityFeed".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "EnableActivityFeed".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "PublishUserActivities".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "UploadUserActivities".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // App Launch Tracking & Instrumentation
        Tweak {
            id: "priv_disable_app_tracking".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable App Launch Tracking".to_string(),
            description: "Stops Windows from tracking app launches and using instrumentation data.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "Start_TrackProgs".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer".to_string(),
                    key: "NoInstrumentation".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer".to_string(),
                key: "NoInstrumentation".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "Start_TrackProgs".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer".to_string(),
                    key: "NoInstrumentation".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Location Services
        Tweak {
            id: "priv_disable_location".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Location Tracking".to_string(),
            description: "Disables location services and sensor permissions.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableLocation".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableWindowsLocationProvider".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                key: "DisableLocation".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableLocation".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableWindowsLocationProvider".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Disable Cloud Sync
        Tweak {
            id: "priv_disable_cloud_sync".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Settings Sync".to_string(),
            description: "Prevents Windows from syncing your settings to Microsoft cloud.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSync".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSyncUserOverride".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                key: "DisableSettingSync".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSync".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSyncUserOverride".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Online Speech Recognition
        Tweak {
            id: "priv_disable_speech".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Online Speech Recognition".to_string(),
            description: "Prevents sending voice data to Microsoft for cloud speech recognition.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Speech_OneCore\\Settings\\OnlineSpeechPrivacy".to_string(),
                    key: "HasAccepted".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Speech_OneCore\\Settings\\OnlineSpeechPrivacy".to_string(),
                key: "HasAccepted".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Speech_OneCore\\Settings\\OnlineSpeechPrivacy".to_string(),
                    key: "HasAccepted".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // App Permissions
        Tweak {
            id: "priv_app_permissions".to_string(),
            category: TweakCategory::Privacy,
            name: "Restrict App Permissions".to_string(),
            description: "Denies app access to diagnostics, location, and user account information globally.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\appDiagnostics".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Allow".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\location".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Allow".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\userAccountInformation".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Allow".to_string()),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\appDiagnostics".to_string(),
                key: "Value".to_string(),
                expected_value: RegistryValue::String("Deny".to_string()),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\appDiagnostics".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Deny".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\location".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Deny".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\userAccountInformation".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Deny".to_string()),
                },
            ]
        },
        
        // Disable Website Language Access
        Tweak {
            id: "priv_disable_lang_list".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Website Language Access".to_string(),
            description: "Prevents websites from accessing your language list to reduce fingerprinting.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\International\\User Profile".to_string(),
                    key: "HttpAcceptLanguageOptOut".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Control Panel\\International\\User Profile".to_string(),
                key: "HttpAcceptLanguageOptOut".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\International\\User Profile".to_string(),
                    key: "HttpAcceptLanguageOptOut".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // Disable Lockscreen Camera
        Tweak {
            id: "priv_lockscreen_camera".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Lockscreen Camera".to_string(),
            description: "Disables camera access on the lockscreen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization".to_string(),
                    key: "NoLockScreenCamera".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization".to_string(),
                key: "NoLockScreenCamera".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization".to_string(),
                    key: "NoLockScreenCamera".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // Configure Windows Media Player
        Tweak {
            id: "priv_config_wmp".to_string(),
            category: TweakCategory::Privacy,
            name: "Configure WMP Privacy".to_string(),
            description: "Disables Windows Media Player DRM internet access, usage tracking, and first-run wizard.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\WMDRM".to_string(),
                    key: "DisableOnline".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "AcceptedPrivacyStatement".to_string(),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "UsageTracking".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                key: "UsageTracking".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\WMDRM".to_string(),
                    key: "DisableOnline".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "AcceptedPrivacyStatement".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "UsageTracking".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Disable Performance Track
        Tweak {
            id: "priv_disable_perftrack".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Performance Tracking".to_string(),
            description: "Disables tracking of responsiveness events for privacy.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WDI\\{9c5a40da-b965-4fc3-8781-88dd50a6299d}".to_string(),
                    key: "ScenarioExecutionEnabled".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WDI\\{9c5a40da-b965-4fc3-8781-88dd50a6299d}".to_string(),
                key: "ScenarioExecutionEnabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WDI\\{9c5a40da-b965-4fc3-8781-88dd50a6299d}".to_string(),
                    key: "ScenarioExecutionEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Disable Bing Web Search
        Tweak {
            id: "priv_disable_bing_search".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Bing Web Search".to_string(),
            description: "Disables Bing web search and Cortana in Windows Search.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search".to_string(), key: "BingSearchEnabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search".to_string(), key: "CortanaConsent".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer".to_string(), key: "DisableSearchBoxSuggestions".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search".to_string(), key: "DisableWebSearch".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search".to_string(), key: "ConnectedSearchUseWeb".to_string() }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search".to_string(),
                key: "BingSearchEnabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search".to_string(), key: "BingSearchEnabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search".to_string(), key: "CortanaConsent".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer".to_string(), key: "DisableSearchBoxSuggestions".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search".to_string(), key: "DisableWebSearch".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search".to_string(), key: "ConnectedSearchUseWeb".to_string(), value: RegistryValue::DWord(0) }
            ]
        },
        
        // Clear MRU History
        Tweak {
            id: "priv_clear_mru".to_string(),
            category: TweakCategory::Privacy,
            name: "Clear Recent Files & MRU History".to_string(),
            description: "Clears all Most Recently Used (MRU) lists for privacy.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: None, tweak_type: TweakType::Action, enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Command { cmd: "cmd".to_string(), args: vec!["/c".to_string(), "del".to_string(), "/q".to_string(), "/f".to_string(), "/s".to_string(), "%APPDATA%\\Microsoft\\Windows\\Recent\\AutomaticDestinations\\*".to_string()] },
                TweakOperation::Command { cmd: "cmd".to_string(), args: vec!["/c".to_string(), "del".to_string(), "/q".to_string(), "/f".to_string(), "/s".to_string(), "%APPDATA%\\Microsoft\\Windows\\Recent\\CustomDestinations\\*".to_string()] },
                TweakOperation::Command { cmd: "cmd".to_string(), args: vec!["/c".to_string(), "del".to_string(), "/q".to_string(), "/f".to_string(), "/s".to_string(), "%APPDATA%\\Microsoft\\Windows\\Recent\\*".to_string()] },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Regedit".to_string(), key: "".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\LastVisitedPidlMRU".to_string(), key: "".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\OpenSaveMRU".to_string(), key: "".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\RecentDocs".to_string(), key: "".to_string() }
            ]
        },

        // ============================================
        // NEW PART 2 PRIVACY TWEAKS
        // ============================================
        Tweak {
            id: "privacy_powershell_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable PowerShell 7 Telemetry".to_string(),
            description: "Sets the POWERSHELL_TELEMETRY_OPTOUT machine environment variable to stop PowerShell 7 from sending usage data to Microsoft.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment".to_string(),
                key: "POWERSHELL_TELEMETRY_OPTOUT".to_string(),
                expected_value: RegistryValue::String("1".to_string()),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment".to_string(),
                key: "POWERSHELL_TELEMETRY_OPTOUT".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment".to_string(),
                key: "POWERSHELL_TELEMETRY_OPTOUT".to_string(),
                value: RegistryValue::String("1".to_string()),
            }],
        },
        Tweak {
            id: "privacy_disable_consumer_features".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Windows Consumer Features".to_string(),
            description: "Prevents Windows from silently installing sponsored apps like TikTok and Candy Crush in the Start Menu.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent".to_string(),
                key: "DisableWindowsConsumerFeatures".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent".to_string(),
                key: "DisableWindowsConsumerFeatures".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent".to_string(),
                key: "DisableWindowsConsumerFeatures".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "privacy_disable_wifi_sense".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Wi-Fi Sense".to_string(),
            description: "Disables automatic connection to suggested open hotspots and sharing of Wi-Fi passwords with contacts.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\PolicyManager\default\WiFi\AllowAutoConnectToWiFiSenseHotspots".to_string(),
                key: "value".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\PolicyManager\default\WiFi\AllowAutoConnectToWiFiSenseHotspots".to_string(),
                key: "value".to_string(),
                value: RegistryValue::DWord(1),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Microsoft\PolicyManager\default\WiFi\AllowAutoConnectToWiFiSenseHotspots".to_string(),
                key: "value".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
        Tweak {
            id: "privacy_disable_recall".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Windows Recall".to_string(),
            description: "Disables Windows Recall, the AI feature that periodically screenshots your screen to build a searchable history. Copilot+ PCs only.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI".to_string(),
                key: "DisableAIDataAnalysis".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI".to_string(),
                key: "DisableAIDataAnalysis".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI".to_string(),
                key: "DisableAIDataAnalysis".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        Tweak {
            id: "privacy_disable_cross_device_resume".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Cross-Device Resume".to_string(),
            description: "Disables the Windows 11 24H2+ feature that syncs your activity between your PC and phone via Microsoft Phone Link.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume\Configuration".to_string(),
                key: "IsResumeAllowed".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume\Configuration".to_string(),
                key: "IsResumeAllowed".to_string(),
                value: RegistryValue::DWord(1),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume\Configuration".to_string(),
                key: "IsResumeAllowed".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
    ]
}
