//! Gaming Optimizations Module
//!
//! Consolidated tweaks from System and CPU modules:
//! - Game Bar & DVR (App capture, overlay)
//! - Game Mode
//! - Fullscreen Optimizations
//! - MMCSS Priority for Games

use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub mod xbox;

pub fn get_gaming_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();
    tweaks.extend(xbox::get_xbox_tweaks());
    tweaks.extend(vec![
        // ============================================
        // Game Bar & DVR
        // ============================================
        Tweak {
            id: "gaming_disable_gamebar".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "🎮 Disable Xbox Game Bar".to_string(),
            description: "Completely disables Xbox Game Bar overlay, startup panel, and tips.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "ShowStartupPanel".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "UseNexusForGameBarEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "AutoGameModeEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "ShowStartupPanel".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "UseNexusForGameBarEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "AutoGameModeEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "GamePanelStartupTipIndex".to_string(),
                    value: RegistryValue::DWord(3),
                },
            ]
        },

        Tweak {
            id: "gaming_disable_gamedvr".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "📹 Disable Game DVR".to_string(),
            description: "Disables background recording, capturing, and broadcasting (GameDVR).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "System\\GameConfigStore".to_string(),
                    key: "GameDVR_Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR".to_string(),
                    key: "AppCaptureEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR".to_string(),
                    key: "AllowGameDVR".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "System\\GameConfigStore".to_string(),
                    key: "GameDVR_Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR".to_string(),
                    key: "AppCaptureEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR".to_string(),
                    key: "AllowGameDVR".to_string(),
                    value: RegistryValue::DWord(0),
                },
                // Additional capture settings
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR".to_string(),
                    key: "AudioCaptureEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR".to_string(),
                    key: "CursorCaptureEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // ============================================
        // Game Mode & FSO
        // ============================================
        Tweak {
            id: "gaming_enable_gamemode".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "🎯 Enable Game Mode".to_string(),
            description: "Enables Windows Game Mode to prioritize games and minimize background activity.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "AllowAutoGameMode".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "AutoGameModeEnabled".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "AllowAutoGameMode".to_string(),
                    value: RegistryValue::DWord(1),
                },
                // Some versions use this too
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                    key: "AutoGameModeEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        Tweak {
            id: "gaming_disable_fso".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "🖥️ Disable Fullscreen Optimizations".to_string(),
            description: "Globally disables fullscreen optimizations (FSO) for better exclusive fullscreen support and less input lag.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "System\\GameConfigStore".to_string(),
                    key: "GameDVR_FSEBehaviorMode".to_string(),
                    value: RegistryValue::DWord(0), // Default
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "System\\GameConfigStore".to_string(),
                    key: "GameDVR_DXGIHonorFSEWindowsCompatible".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "System\\GameConfigStore".to_string(),
                    key: "GameDVR_FSEBehaviorMode".to_string(),
                    value: RegistryValue::DWord(2), // Disable FSO
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "System\\GameConfigStore".to_string(),
                    key: "GameDVR_HonorUserFSEBehaviorMode".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "System\\GameConfigStore".to_string(),
                    key: "GameDVR_DXGIHonorFSEWindowsCompatible".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // ============================================
        // Priorities
        // ============================================
        Tweak {
            id: "gaming_mmcss_priority".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "⚖️ Optimize Game Process Priority".to_string(),
            description: "Sets MMCSS Games task to high priority for better CPU scheduling in games.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "GPU Priority".to_string(),
                    value: RegistryValue::DWord(8),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Priority".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Scheduling Category".to_string(),
                    value: RegistryValue::String("Medium".to_string()),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "GPU Priority".to_string(),
                    value: RegistryValue::DWord(8), // Keep 8 or set higher? 8 is reasonable High
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Priority".to_string(),
                    value: RegistryValue::DWord(6),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "Scheduling Category".to_string(),
                    value: RegistryValue::String("High".to_string()),
                },
                 TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "SFIO Priority".to_string(),
                    value: RegistryValue::String("High".to_string()),
                },
            ]
        },
    ]);
    tweaks
}
