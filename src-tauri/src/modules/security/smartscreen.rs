//! SmartScreen Tweaks
//!
//! Controls for Windows SmartScreen filter for apps, Edge, and Store.

use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_smartscreen_tweaks() -> Vec<Tweak> {
    vec![
        // Disable SmartScreen for Apps and Files
        Tweak {
            id: "sec_disable_smartscreen_apps".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🛡️ Disable SmartScreen for Apps".to_string(),
            description: "Disables SmartScreen filter for downloaded apps and files. Reduces protection against malware.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "EnableSmartScreen".to_string(),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer".to_string(),
                    key: "SmartScreenEnabled".to_string(),
                    value: RegistryValue::String("On".to_string()),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "EnableSmartScreen".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer".to_string(),
                    key: "SmartScreenEnabled".to_string(),
                    value: RegistryValue::String("Off".to_string()),
                },
            ],
        },

        // Disable SmartScreen for Microsoft Edge
        Tweak {
            id: "sec_disable_smartscreen_edge".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🌐 Disable SmartScreen for Edge".to_string(),
            description: "Disables SmartScreen in Microsoft Edge browser.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppContainer\\Storage\\microsoft.microsoftedge_8wekyb3d8bbwe\\MicrosoftEdge\\PhishingFilter".to_string(),
                    key: "EnabledV9".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppContainer\\Storage\\microsoft.microsoftedge_8wekyb3d8bbwe\\MicrosoftEdge\\PhishingFilter".to_string(),
                    key: "EnabledV9".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable SmartScreen for Store Apps
        Tweak {
            id: "sec_disable_smartscreen_store".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🏪 Disable SmartScreen for Store Apps".to_string(),
            description: "Disables web content evaluation for Microsoft Store apps.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AppHost".to_string(),
                    key: "EnableWebContentEvaluation".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AppHost".to_string(),
                    key: "EnableWebContentEvaluation".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable Application Reputation Check
        Tweak {
            id: "sec_disable_app_reputation".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "📋 Disable App Reputation Check".to_string(),
            description: "Stops Windows from checking app reputation online before execution.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\SmartScreen".to_string(),
                    key: "ConfigureAppInstallControlEnabled".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\SmartScreen".to_string(),
                    key: "ConfigureAppInstallControlEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable "Windows protected your PC" Popup
        Tweak {
            id: "sec_disable_protected_popup".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🚫 Disable 'Protected Your PC' Popup".to_string(),
            description: "Removes the blue warning screen when running unrecognized apps.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Attachments".to_string(),
                    key: "SaveZoneInformation".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Attachments".to_string(),
                    key: "SaveZoneInformation".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },
    ]
}
