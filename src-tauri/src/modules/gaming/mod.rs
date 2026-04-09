//! Gaming Optimizations Module
//!
//! Consolidated tweaks from System and CPU modules:
//! - Game Bar & DVR (App capture, overlay)
//! - Game Mode
//! - Fullscreen Optimizations
//! - MMCSS Priority for Games

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

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
            name: "Disable Xbox Game Bar".to_string(),
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
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$panel = Get-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\GameBar" -Name "ShowStartupPanel" -ErrorAction SilentlyContinue
$capture = Get-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR" -Name "AppCaptureEnabled" -ErrorAction SilentlyContinue
if (($panel.ShowStartupPanel -eq 0) -and ($capture.AppCaptureEnabled -eq 0)) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
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
                TweakOperation::ServiceDisable { name: "XblAuthManager".to_string() },
                TweakOperation::ServiceDisable { name: "XblGameSave".to_string() },
                TweakOperation::ServiceDisable { name: "XboxGipSvc".to_string() },
                TweakOperation::ServiceDisable { name: "XboxNetApiSvc".to_string() },
            ]
        },

        Tweak {
            id: "gaming_disable_gamedvr".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable Game DVR".to_string(),
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
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "System\\GameConfigStore".to_string(),
                key: "GameDVR_Enabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
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
            name: "Enable Game Mode".to_string(),
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
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\GameBar".to_string(),
                key: "AllowAutoGameMode".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
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
            name: "Disable Fullscreen Optimizations".to_string(),
            description: "Globally disables fullscreen optimizations (FSO). May improve input latency in competitive games but can break VRR/HDR.".to_string(),
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
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "System\\GameConfigStore".to_string(),
                key: "GameDVR_FSEBehaviorMode".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
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
            name: "Optimize Game Process Priority".to_string(),
            description: "Sets MMCSS Games task to high priority for better CPU scheduling in games.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                    key: "GPU Priority".to_string(),
                    value: RegistryValue::DWord(2), // Windows default is 2, not 8
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
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games".to_string(),
                key: "Priority".to_string(),
                expected_value: RegistryValue::DWord(6),
            }),
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

        // ============================================
        // NEW: Network Throttling Disable
        // ============================================
        Tweak {
            id: "gaming_disable_network_throttling".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable Network Throttling".to_string(),
            description: "Disables network throttling and maximizes foreground priority. Reduces online gaming latency.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                key: "NetworkThrottlingIndex".to_string(),
                expected_value: RegistryValue::DWord(0xFFFFFFFF),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "NetworkThrottlingIndex".to_string(),
                    value: RegistryValue::DWord(10),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "SystemResponsiveness".to_string(),
                    value: RegistryValue::DWord(20),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "NetworkThrottlingIndex".to_string(),
                    value: RegistryValue::DWord(0xFFFFFFFF), // Disable throttling completely
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile".to_string(),
                    key: "SystemResponsiveness".to_string(),
                    value: RegistryValue::DWord(0), // 100% to foreground apps
                },
            ],
        },

        // ============================================
        // Visual Effects Disable (B.20)
        // ============================================
        Tweak {
            id: "gaming_disable_visual_effects".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable Visual Effects".to_string(),
            description: "Disables Windows animations and visual effects for best performance.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects".to_string(),
                key: "VisualFXSetting".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects".to_string(), key: "VisualFXSetting".to_string() },
                TweakOperation::RegistryDelete { root_key: "HKCU".to_string(), path: "Control Panel\\Desktop".to_string(), key: "UserPreferencesMask".to_string() },
            ]),
            operations: vec![
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects".to_string(), key: "VisualFXSetting".to_string(), value: RegistryValue::DWord(2) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(), key: "TaskbarAnimations".to_string(), value: RegistryValue::DWord(0) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "Control Panel\\Desktop".to_string(), key: "MenuShowDelay".to_string(), value: RegistryValue::String("0".to_string()) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "Control Panel\\Desktop\\WindowMetrics".to_string(), key: "MinAnimate".to_string(), value: RegistryValue::String("0".to_string()) },
                TweakOperation::RegistrySet { root_key: "HKCU".to_string(), path: "Control Panel\\Desktop".to_string(), key: "UserPreferencesMask".to_string(), value: RegistryValue::Binary(vec![0x90,0x12,0x03,0x80,0x10,0x00,0x00,0x00]) },
            ],
        },

        // ============================================
        // MMCSS Disable Option (B.21)
        // ============================================
        Tweak {
            id: "gaming_disable_mmcss".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable MMCSS Service".to_string(),
            description: "Disables MMCSS service. Only use if you've tested and confirmed improvement on your system.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$svc = Get-Service -Name "MMCSS" -EA 0
if ($svc.StartType -eq 'Disabled') { 'True' } else { 'False' }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "MMCSS".to_string(), mode: "auto".to_string() }
            ]),
            operations: vec![
                TweakOperation::ServiceDisable { name: "MMCSS".to_string() }
            ],
        },
    ]);
    tweaks
}
