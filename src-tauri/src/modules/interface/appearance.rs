//! Appearance Tweaks Module

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_appearance_tweaks() -> Vec<Tweak> {
    vec![Tweak {
        id: "interface_dark_mode".to_string(),
        category: TweakCategory::InterfaceUx,
        name: "Full Dark Mode".to_string(),
        description: "Force dark mode for system and apps.".to_string(),
        warning_level: WarningLevel::Safe,
        requires_restart: false,
        tweak_type: TweakType::Toggle,
        enabled: false,
        check: Some(TweakCheck::Registry {
            root_key: "HKCU".to_string(),
            path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize".to_string(),
            key: "AppsUseLightTheme".to_string(),
            expected_value: RegistryValue::DWord(0),
        }),
        revert_operations: Some(vec![
            TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize".to_string(),
                key: "AppsUseLightTheme".to_string(),
                value: RegistryValue::DWord(1),
            },
            TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize".to_string(),
                key: "SystemUsesLightTheme".to_string(),
                value: RegistryValue::DWord(1),
            },
        ]),
        operations: vec![
            TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize".to_string(),
                key: "AppsUseLightTheme".to_string(),
                value: RegistryValue::DWord(0),
            },
            TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize".to_string(),
                key: "SystemUsesLightTheme".to_string(),
                value: RegistryValue::DWord(0),
            },
        ],
    }]
}
