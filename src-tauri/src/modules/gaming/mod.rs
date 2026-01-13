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
            description: "Globally disables fullscreen optimizations (FSO).

⚠️ MODERN ADVICE (2024+):
FSO has improved significantly and is now RECOMMENDED for:
- Variable Refresh Rate (VRR/G-Sync/FreeSync) monitors
- HDR displays
- Multi-monitor setups
- Alt-Tab without black screens

Consider disabling ONLY if:
- You experience input lag in competitive games
- Games stutter or have frame pacing issues with FSO on
- You don't use VRR or HDR

Disabling FSO may BREAK VRR and HDR functionality!".to_string(),
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
            description: "Disables Windows network throttling (10 packets/ms limit). Reduces online gaming latency by 10-30ms. Also sets SystemResponsiveness to 0 for maximum foreground priority.".to_string(),
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
            description: "Disables Windows visual effects and animations for maximum performance.

Disables:
- Window animations
- Taskbar animations
- Smooth scrolling
- Tooltip fade
- Menu fade/slide effects

Recommended for gaming systems where every frame counts.".to_string(),
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
                TweakOperation::Powershell {
                    script: r#"
# Restore default visual effects (Let Windows choose)
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects" -Name "VisualFXSetting" -Value 0 -Type DWord -Force -EA 0
# Remove custom UserPreferencesMask to use defaults
Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "UserPreferencesMask" -EA 0
Write-Host "Visual effects restored to Windows defaults" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Set visual effects to "Best Performance"
# VisualFXSetting: 0=Let Windows choose, 1=Best appearance, 2=Best performance, 3=Custom
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects" -Name "VisualFXSetting" -Value 2 -Type DWord -Force -EA 0

# Disable individual animations
$explorerAdvanced = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $explorerAdvanced -Name "TaskbarAnimations" -Value 0 -Type DWord -Force -EA 0

# Disable menu animations
$desktop = "HKCU:\Control Panel\Desktop"
Set-ItemProperty -Path $desktop -Name "MenuShowDelay" -Value "0" -Force -EA 0

# Disable window animations
$windowMetrics = "HKCU:\Control Panel\Desktop\WindowMetrics"
Set-ItemProperty -Path $windowMetrics -Name "MinAnimate" -Value "0" -Force -EA 0

# UserPreferencesMask for performance (disables most visual effects)
# This is a binary value that controls many visual settings
$perfMask = [byte[]](0x90,0x12,0x03,0x80,0x10,0x00,0x00,0x00)
Set-ItemProperty -Path $desktop -Name "UserPreferencesMask" -Value $perfMask -Type Binary -Force -EA 0

Write-Host "Visual effects disabled for best performance" -ForegroundColor Green
Write-Host "You may need to restart Explorer or log off for all changes to apply" -ForegroundColor Yellow
"#.to_string(),
                }
            ],
        },

        // ============================================
        // MMCSS Disable Option (B.21)
        // ============================================
        Tweak {
            id: "gaming_disable_mmcss".to_string(),
            category: TweakCategory::GameOptimizations,
            name: "Disable MMCSS Service".to_string(),
            description: "Disables Multimedia Class Scheduler Service (MMCSS).

Some systems perform better without MMCSS interference:
- Older CPUs with limited cores
- Systems with aggressive custom power plans
- Specific game engine issues

WARNING: This may HURT performance on most modern systems.
MMCSS normally helps by boosting audio/video thread priority.
Only disable if you've tested and confirmed improvement.".to_string(),
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
                TweakOperation::Powershell {
                    script: r#"
Set-Service -Name "MMCSS" -StartupType Automatic -EA 0
Start-Service -Name "MMCSS" -EA 0
Write-Host "MMCSS service re-enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Stop-Service -Name "MMCSS" -Force -EA 0
Set-Service -Name "MMCSS" -StartupType Disabled -EA 0
Write-Host "MMCSS service disabled" -ForegroundColor Yellow
Write-Host "Restart required for changes to take effect" -ForegroundColor Cyan
"#.to_string(),
                }
            ],
        },
    ]);
    tweaks
}
