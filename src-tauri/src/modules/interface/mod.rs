//! Interface & UX Tweaks Module

use crate::modules::types::{TweakType, 
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel,
};

pub fn get_interface_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // C.11: Show File Extensions
        // ============================================
        Tweak {
            id: "interface_show_file_extensions".to_string(),
            category: TweakCategory::Interface,
            name: "📄 Show File Extensions".to_string(),
            description:
                "Shows file extensions in File Explorer (e.g., document.docx instead of document).

Security benefit: Prevents malware from disguising .exe files as documents."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "HideFileExt".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "HideFileExt".to_string(),
                value: RegistryValue::DWord(1), // Hidden (default)
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "HideFileExt".to_string(),
                value: RegistryValue::DWord(0), // Show extensions
            }],
        },
        // ============================================
        // C.12: Compact File Explorer View
        // ============================================
        Tweak {
            id: "interface_compact_mode".to_string(),
            category: TweakCategory::Interface,
            name: "📐 Enable Compact File Explorer".to_string(),
            description: "Enables compact mode in File Explorer for more items per page.

Reduces padding between items to fit more files on screen.
Useful for high-resolution displays or power users."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "UseCompactMode".to_string(),
                expected_value: RegistryValue::DWord(1), // Compact mode
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "UseCompactMode".to_string(),
                value: RegistryValue::DWord(0), // Standard mode
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "UseCompactMode".to_string(),
                value: RegistryValue::DWord(1), // Compact mode
            }],
        },
        // ============================================
        // Show Hidden Files
        // ============================================
        Tweak {
            id: "interface_show_hidden_files".to_string(),
            category: TweakCategory::Interface,
            name: "👁️ Show Hidden Files".to_string(),
            description: "Shows hidden files and folders in File Explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "Hidden".to_string(),
                expected_value: RegistryValue::DWord(1), // Show hidden files
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "Hidden".to_string(),
                value: RegistryValue::DWord(2), // Don't show hidden
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "Hidden".to_string(),
                value: RegistryValue::DWord(1), // Show hidden files
            }],
        },
        // ============================================
        // Show System Files
        // ============================================
        Tweak {
            id: "interface_show_system_files".to_string(),
            category: TweakCategory::Interface,
            name: "⚙️ Show Protected System Files".to_string(),
            description: "Shows protected operating system files in File Explorer.

WARNING: Be careful not to modify or delete system files!"
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "ShowSuperHidden".to_string(),
                expected_value: RegistryValue::DWord(1), // Show system files
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "ShowSuperHidden".to_string(),
                value: RegistryValue::DWord(0), // Hide system files (default)
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced"
                    .to_string(),
                key: "ShowSuperHidden".to_string(),
                value: RegistryValue::DWord(1), // Show system files
            }],
        },
    ]
}
