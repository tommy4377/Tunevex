//! User Account Control (UAC) Tweaks
//!
//! Controls for UAC prompts, elevation, and admin approval mode.

use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_uac_tweaks() -> Vec<Tweak> {
    vec![
        // Lower UAC Level
        Tweak {
            id: "sec_uac_lower".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "📉 Lower UAC Level".to_string(),
            description: "Sets UAC to 'notify only when apps try to make changes' without dimming desktop.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "ConsentPromptBehaviorAdmin".to_string(),
                    value: RegistryValue::DWord(5),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "PromptOnSecureDesktop".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "ConsentPromptBehaviorAdmin".to_string(),
                    value: RegistryValue::DWord(5), // 5 = Prompt for consent on non-secure desktop
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "PromptOnSecureDesktop".to_string(),
                    value: RegistryValue::DWord(0), // 0 = Don't dim desktop
                },
            ],
        },

        // Disable Secure Desktop
        Tweak {
            id: "sec_uac_no_secure".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "🖥️ Disable Secure Desktop for UAC".to_string(),
            description: "UAC prompts appear on regular desktop instead of secure (dimmed) desktop.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "PromptOnSecureDesktop".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "PromptOnSecureDesktop".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },

        // Disable UAC Prompts Completely
        Tweak {
            id: "sec_disable_uac".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "⛔ Disable UAC Prompts".to_string(),
            description: "Disables all UAC elevation prompts. Apps elevate silently. DANGEROUS: Malware can run elevated without warning.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "EnableLUA".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "EnableLUA".to_string(),
                    value: RegistryValue::DWord(0), // 0 = Disable UAC
                },
            ],
        },

        // Disable Admin Approval Mode
        Tweak {
            id: "sec_uac_admin_mode".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "👑 Disable Admin Approval Mode".to_string(),
            description: "Admin accounts run with full privileges without prompts. Requires reboot.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "FilterAdministratorToken".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "ConsentPromptBehaviorAdmin".to_string(),
                    value: RegistryValue::DWord(5),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "FilterAdministratorToken".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "ConsentPromptBehaviorAdmin".to_string(),
                    value: RegistryValue::DWord(0), // 0 = Elevate without prompting
                },
            ],
        },

        // Auto-Elevate Known Apps
        Tweak {
            id: "sec_uac_auto_elevate".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "⬆️ Auto-Elevate Known Apps".to_string(),
            description: "Allows Windows to auto-elevate known Microsoft applications without prompts.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "EnableInstallerDetection".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System".to_string(),
                    key: "EnableInstallerDetection".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },
    ]
}
