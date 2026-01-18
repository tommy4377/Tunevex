//! Taskbar Tweaks Module (ExplorerPatcher Integration)
//!
//! CRITICAL VALUES:
//! - TaskbarStyle: 0=Win11, 1=Win10(deprecated), 2=Win10(EP mode) ← USE 2!
//! - Position byte 12: 00=Left, 01=Top, 02=Right, 03=Bottom
//! - ShellExtensionEnabled: 0 to prevent File Explorer crashes

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Synchronous explorer restart with longer delay
const RESTART_EXPLORER: &str = r#"
Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 3
Start-Process explorer
"#;

/// Position check script - checks both StuckRects3 and StuckRectsLegacy
fn position_check_script(expected_byte: u8) -> String {
    format!(
        r#"
$stuck3 = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3'
$stuckLegacy = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRectsLegacy'

$settings = $null
if (Test-Path $stuck3) {{
    $settings = (Get-ItemProperty $stuck3 -Name 'Settings' -EA SilentlyContinue).Settings
}}
if ($null -eq $settings -and (Test-Path $stuckLegacy)) {{
    $settings = (Get-ItemProperty $stuckLegacy -Name 'Settings' -EA SilentlyContinue).Settings
}}

if ($null -eq $settings) {{ 'False'; exit }}
if ($settings[12] -eq {}) {{ 'True' }} else {{ 'False' }}
"#,
        expected_byte
    )
}

/// Position set script - supports both StuckRects3 and StuckRectsLegacy
fn position_set_script(position_byte: u8) -> String {
    format!(
        r#"
$stuck3 = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3'
$stuckLegacy = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRectsLegacy'

$regPath = $null
$settings = $null

if (Test-Path $stuck3) {{
    $regPath = $stuck3
    $settings = (Get-ItemProperty $regPath -Name 'Settings' -EA SilentlyContinue).Settings
}}

if ($null -eq $settings -and (Test-Path $stuckLegacy)) {{
    $regPath = $stuckLegacy
    $settings = (Get-ItemProperty $regPath -Name 'Settings' -EA SilentlyContinue).Settings
}}

if ($null -ne $settings -and $null -ne $regPath) {{
    $settings[12] = {}
    Set-ItemProperty -Path $regPath -Name 'Settings' -Value $settings -Type Binary
}}
"#,
        position_byte
    )
}

pub fn get_taskbar_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // WIN10 TASKBAR STYLE (EP Mode = TaskbarStyle 2)
        // CRITICAL: Value 2, not 1!
        // ============================================
        Tweak {
            id: "ep_style_win10".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Windows 10 Taskbar".to_string(),
            description: "Classic Win10 taskbar (requires ExplorerPatcher). Uses EP mode for stability.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$path = 'HKCU:\Software\ExplorerPatcher'
if (!(Test-Path $path)) { 'False'; exit }
$style = (Get-ItemProperty $path -Name 'TaskbarStyle' -EA SilentlyContinue).TaskbarStyle
# Check for value 2 (EP mode) OR value 1 (legacy)
if ($style -eq 2 -or $style -eq 1) { 'True' } else { 'False' }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = 'HKCU:\Software\ExplorerPatcher'
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }

# CRITICAL: Use value 2 for Windows 10 (ExplorerPatcher) mode
Set-ItemProperty -Path $path -Name 'TaskbarStyle' -Value 2 -Type DWord -Force

# Disable shell extension for File Explorer to prevent crashes
Set-ItemProperty -Path $path -Name 'ShellExtensionEnabled' -Value 0 -Type DWord -Force
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = 'HKCU:\Software\ExplorerPatcher'
if (Test-Path $path) {
    Set-ItemProperty -Path $path -Name 'TaskbarStyle' -Value 0 -Type DWord -Force
}
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },

        // ============================================
        // TASKBAR POSITION - TOP (byte 12 = 01)
        // Requires EP mode (TaskbarStyle=2)
        // ============================================
        Tweak {
            id: "taskbar_pos_top".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar: Top".to_string(),
            description: "Move taskbar to top (requires Win10 style active).".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: position_check_script(1),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                // First ensure EP mode is active
                TweakOperation::Powershell {
                    script: r#"
$epPath = 'HKCU:\Software\ExplorerPatcher'
if (!(Test-Path $epPath)) { New-Item -Path $epPath -Force | Out-Null }
Set-ItemProperty -Path $epPath -Name 'TaskbarStyle' -Value 2 -Type DWord -Force
Set-ItemProperty -Path $epPath -Name 'ShellExtensionEnabled' -Value 0 -Type DWord -Force
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: position_set_script(1), // 01 = Top
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: position_set_script(3), // 03 = Bottom
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },

        // TASKBAR POSITION - LEFT (byte 12 = 00)
        Tweak {
            id: "taskbar_pos_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar: Left".to_string(),
            description: "Move taskbar to left side.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: position_check_script(0),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$epPath = 'HKCU:\Software\ExplorerPatcher'
if (!(Test-Path $epPath)) { New-Item -Path $epPath -Force | Out-Null }
Set-ItemProperty -Path $epPath -Name 'TaskbarStyle' -Value 2 -Type DWord -Force
Set-ItemProperty -Path $epPath -Name 'ShellExtensionEnabled' -Value 0 -Type DWord -Force
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: position_set_script(0),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: position_set_script(3),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },

        // TASKBAR POSITION - RIGHT (byte 12 = 02)
        Tweak {
            id: "taskbar_pos_right".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar: Right".to_string(),
            description: "Move taskbar to right side.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: position_check_script(2),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$epPath = 'HKCU:\Software\ExplorerPatcher'
if (!(Test-Path $epPath)) { New-Item -Path $epPath -Force | Out-Null }
Set-ItemProperty -Path $epPath -Name 'TaskbarStyle' -Value 2 -Type DWord -Force
Set-ItemProperty -Path $epPath -Name 'ShellExtensionEnabled' -Value 0 -Type DWord -Force
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: position_set_script(2),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: position_set_script(3),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },

        // ============================================
        // SMALL ICONS (Native key)
        // ============================================
        Tweak {
            id: "taskbar_icons_small".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Small Icons".to_string(),
            description: "Use smaller taskbar icons.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarSmallIcons".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarSmallIcons".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarSmallIcons".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },

        // ============================================
        // ALIGNMENT - LEFT (Native, works immediately)
        // ============================================
        Tweak {
            id: "taskbar_align_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Align Left".to_string(),
            description: "Align taskbar icons to the left.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(0),
            }],
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(1),
            }]),
        },

        // ============================================
        // NEVER COMBINE (Native)
        // ============================================
        Tweak {
            id: "taskbar_never_combine".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Never Combine".to_string(),
            description: "Show separate button for each window.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarGlomLevel".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarGlomLevel".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                    key: "TaskbarGlomLevel".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },

        // ============================================
        // EMERGENCY RECOVERY - Fixes crash loops
        // ============================================
        Tweak {
            id: "ep_emergency_recovery".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "EP: Emergency Recovery".to_string(),
            description: "Cleans ExplorerPatcher registry if crashes occur.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Delete corrupted EP registry
Remove-Item 'HKCU:\Software\ExplorerPatcher' -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\ExplorerPatcher' -Recurse -Force -ErrorAction SilentlyContinue
# Restart explorer
Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 3
Start-Process explorer
"#.to_string(),
                },
            ],
            revert_operations: None,
        },

        // ============================================
        // FULL RESET - Restores all defaults
        // ============================================
        Tweak {
            id: "ep_full_reset".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Reset All Taskbar".to_string(),
            description: "Restore Windows 11 defaults.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Delete EP registries
Remove-Item 'HKCU:\Software\ExplorerPatcher' -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\ExplorerPatcher' -Recurse -Force -ErrorAction SilentlyContinue

# Reset native settings
$adv = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
Set-ItemProperty $adv -Name 'TaskbarAl' -Value 1 -Force -ErrorAction SilentlyContinue
Set-ItemProperty $adv -Name 'TaskbarSmallIcons' -Value 0 -Force -ErrorAction SilentlyContinue
Set-ItemProperty $adv -Name 'TaskbarGlomLevel' -Value 0 -Force -ErrorAction SilentlyContinue

# Reset position to bottom in both StuckRects locations
$stuck3 = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3'
$stuckLegacy = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRectsLegacy'

if (Test-Path $stuck3) {
    $settings = (Get-ItemProperty $stuck3 -Name 'Settings' -EA SilentlyContinue).Settings
    if ($settings) {
        $settings[12] = 3
        Set-ItemProperty $stuck3 -Name 'Settings' -Value $settings -Type Binary
    }
}

if (Test-Path $stuckLegacy) {
    $settings = (Get-ItemProperty $stuckLegacy -Name 'Settings' -EA SilentlyContinue).Settings
    if ($settings) {
        $settings[12] = 3
        Set-ItemProperty $stuckLegacy -Name 'Settings' -Value $settings -Type Binary
    }
}
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: None,
        },
    ]
}
