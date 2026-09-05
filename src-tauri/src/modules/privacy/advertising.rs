use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Advertising & Tracking
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // Advertising ID
        Tweak {
            id: "priv_disable_advertising_id".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Advertising ID".to_string(),
            description: "Prevents apps from using advertising ID for personalized ads.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo".to_string(),
                    key: "DisabledByGroupPolicy".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo".to_string(),
                key: "DisabledByGroupPolicy".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo".to_string(),
                    key: "DisabledByGroupPolicy".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // Disable Sync Provider Notifications (File Explorer Ads)
        Tweak {
            id: "priv_disable_sync_notifs".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Sync Provider Notifications".to_string(),
            description: "Disables notifications/ads from OneDrive and other sync providers in File Explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "ShowSyncProviderNotifications".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                key: "ShowSyncProviderNotifications".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "ShowSyncProviderNotifications".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // Tailored Experiences
        Tweak {
            id: "priv_disable_tailored".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Tailored Experiences".to_string(),
            description: "Stops Microsoft from using diagnostic data for personalized tips and ads.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Privacy".to_string(),
                    key: "TailoredExperiencesWithDiagnosticDataEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\CloudContent".to_string(),
                    key: "DisableTailoredExperiencesWithDiagnosticData".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\CloudContent".to_string(),
                key: "DisableTailoredExperiencesWithDiagnosticData".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Privacy".to_string(),
                    key: "TailoredExperiencesWithDiagnosticDataEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\CloudContent".to_string(),
                    key: "DisableTailoredExperiencesWithDiagnosticData".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // Suggested Content
        Tweak {
            id: "priv_disable_suggestions".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Suggested Content".to_string(),
            description: "Removes Microsoft suggestions, tips, and spotlight from Start and lock screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338388Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338389Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-310093Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SoftLandingEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "RotatingLockScreenEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                key: "SubscribedContent-338388Enabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338388Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338389Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-310093Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SoftLandingEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "RotatingLockScreenEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // Comprehensive Privacy All-in-One
        Tweak {
            id: "priv_all_in_one".to_string(),
            category: TweakCategory::Privacy,
            name: "Privacy Hardening All-in-One".to_string(),
            description: "Comprehensive privacy settings: disables feedback, handwriting reports, Copilot, Recall, web content, and more.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Siuf\\Rules".to_string(), key: "NumberOfSIUFInPeriod".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\HandwritingErrorReports".to_string(), key: "PreventHandwritingErrorReports".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI".to_string(), key: "DisableAIDataAnalysis".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot".to_string(), key: "TurnOffWindowsCopilot".to_string() },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(), key: "EnableSmartScreen".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer".to_string(), key: "DisableSearchBoxSuggestions".to_string() },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-338393Enabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-353694Enabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-353696Enabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-338388Enabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-338389Enabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-310093Enabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-314563Enabled".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SystemPaneSuggestionsEnabled".to_string(), value: RegistryValue::DWord(1) },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MultiRegistry {
                checks: vec![
                    crate::modules::types::RegistryCheck { root_key: "HKCU".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot".to_string(), key: "TurnOffWindowsCopilot".to_string(), expected_value: RegistryValue::DWord(1) },
                    crate::modules::types::RegistryCheck { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI".to_string(), key: "DisableAIDataAnalysis".to_string(), expected_value: RegistryValue::DWord(1) },
                    crate::modules::types::RegistryCheck { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(), key: "EnableSmartScreen".to_string(), expected_value: RegistryValue::DWord(0) },
                ]
            }),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Siuf\\Rules".to_string(), key: "NumberOfSIUFInPeriod".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\HandwritingErrorReports".to_string(), key: "PreventHandwritingErrorReports".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI".to_string(), key: "DisableAIDataAnalysis".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot".to_string(), key: "TurnOffWindowsCopilot".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKLM".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(), key: "EnableSmartScreen".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer".to_string(), key: "DisableSearchBoxSuggestions".to_string(), value: RegistryValue::DWord(1) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-338393Enabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-353694Enabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-353696Enabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-338388Enabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-338389Enabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-310093Enabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SubscribedContent-314563Enabled".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(), key: "SystemPaneSuggestionsEnabled".to_string(), value: RegistryValue::DWord(0) },
            ]
        },
    ]
}
