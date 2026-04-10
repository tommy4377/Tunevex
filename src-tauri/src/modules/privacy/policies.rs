use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// System & Group Policies
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // Disable OOBE Privacy Experience
        Tweak {
            id: "priv_disable_oobe_privacy".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable OOBE Privacy Experience".to_string(),
            description: "Disables the Out of Box Experience privacy configuration prompts on updates.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\OOBE".to_string(),
                    key: "DisablePrivacyExperience".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\OOBE".to_string(),
                key: "DisablePrivacyExperience".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\OOBE".to_string(),
                    key: "DisablePrivacyExperience".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // Disallow Microsoft Accounts
        Tweak {
            id: "priv_disallow_ms_accounts".to_string(),
            category: TweakCategory::Privacy,
            name: "Restrict Microsoft Accounts".to_string(),
            description: "Prevents users from adding Microsoft accounts, encouraging local accounts for privacy.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "NoConnectedUser".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                key: "NoConnectedUser".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "NoConnectedUser".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Disable RSoP Logging
        Tweak {
            id: "priv_disable_rsop".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable RSoP Logging".to_string(),
            description: "Disables logging of Group Policy settings (Resultant Set of Policy) for privacy.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "RSoPLogging".to_string(),
                    value: RegistryValue::DWord(1), // Re-enable logging on revert
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                key: "RSoPLogging".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "RSoPLogging".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // Disable Experimentation
        Tweak {
            id: "priv_disable_experimentation".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Microsoft Experimentation".to_string(),
            description: "Prevents Microsoft from using your computer as a test platform for new features.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\AllowExperimentation".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\AllowExperimentation".to_string(),
                key: "Value".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\AllowExperimentation".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Disable Smart App Control
        Tweak {
            id: "priv_disable_smart_app_control".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Smart App Control".to_string(),
            description: "Disables Smart App Control which can slow down app launching and sends usage data to Microsoft.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\CI\\Policy".to_string(),
                    key: "VerifiedAndReputablePolicyState".to_string(),
                    value: RegistryValue::DWord(1), // Default usually 1 (Enforce) or 0 (Off) depending on setup
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\CI\\Policy".to_string(),
                key: "VerifiedAndReputablePolicyState".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\CI\\Policy".to_string(),
                    key: "VerifiedAndReputablePolicyState".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
    ]
}
