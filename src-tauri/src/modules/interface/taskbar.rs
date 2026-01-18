//! Taskbar Tweaks Module (ExplorerPatcher Integration)
//!
//! Position uses native StuckRects3 byte array (byte 12).
//! Style/advanced features require ExplorerPatcher.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Synchronous explorer restart
const RESTART_EXPLORER: &str = r#"
Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2
Start-Process explorer
"#;

/// Check if ExplorerPatcher is installed
const CHECK_EP_INSTALLED: &str = r#"
$ep = "${env:ProgramFiles}\ExplorerPatcher\ep_setup.exe"
if (Test-Path $ep) { 'True' } else { 'False' }
"#;

/// Get current position from StuckRects3 (byte 12: 00=Left, 01=Top, 02=Right, 03=Bottom)
fn position_check_script(expected_byte: u8) -> String {
    format!(
        r#"
$settings = (Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3' -Name 'Settings' -EA SilentlyContinue).Settings
if ($null -eq $settings) {{ 'False'; exit }}
$pos = $settings[12]
if ($pos -eq {}) {{ 'True' }} else {{ 'False' }}
"#,
        expected_byte
    )
}

/// Set position in StuckRects3 byte array
fn position_set_script(position_byte: u8) -> String {
    format!(
        r#"
$regPath = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3'
$settings = (Get-ItemProperty $regPath -Name 'Settings').Settings
$settings[12] = {}
Set-ItemProperty $regPath -Name 'Settings' -Value $settings
"#,
        position_byte
    )
}

pub fn get_taskbar_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // EP INSTALLATION (Required for Win10 style features)
        // ============================================
        Tweak {
            id: "ep_install".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Install ExplorerPatcher".to_string(),
            description: "Enables Win10 taskbar features on Win11. Downloads and installs EP.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: CHECK_EP_INSTALLED.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$ErrorActionPreference = 'SilentlyContinue'
$epPath = "${env:ProgramFiles}\ExplorerPatcher\ep_setup.exe"
if (!(Test-Path $epPath)) {
    $releases = Invoke-RestMethod 'https://api.github.com/repos/valinet/ExplorerPatcher/releases/latest'
    $url = ($releases.assets | Where-Object { $_.name -eq 'ep_setup.exe' }).browser_download_url
    $temp = "$env:TEMP\ep_setup.exe"
    Invoke-WebRequest $url -OutFile $temp -UseBasicParsing
    Start-Process $temp -ArgumentList '/VERYSILENT','/NORESTART','/SUPPRESSMSGBOXES' -Wait -WindowStyle Hidden
    Remove-Item $temp -Force -ErrorAction SilentlyContinue
    Add-MpPreference -ExclusionPath "${env:ProgramFiles}\ExplorerPatcher" -ErrorAction SilentlyContinue
}
"#.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$uninstaller = "${env:ProgramFiles}\ExplorerPatcher\ep_setup.exe"
if (Test-Path $uninstaller) {
    Start-Process $uninstaller -ArgumentList '/uninstall','/VERYSILENT' -Wait -WindowStyle Hidden
}
"#.to_string(),
                },
            ]),
        },

        // ============================================
        // WIN10 TASKBAR STYLE (requires EP)
        // ============================================
        Tweak {
            id: "ep_style_win10".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Windows 10 Taskbar".to_string(),
            description: "Classic Win10 taskbar with labels (requires EP installed).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$val = (Get-ItemProperty 'HKCU:\Software\ExplorerPatcher' -Name 'TaskbarStyle' -EA SilentlyContinue).TaskbarStyle
if ($val -eq 1) { 'True' } else { 'False' }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = 'HKCU:\Software\ExplorerPatcher'
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name 'TaskbarStyle' -Value 1 -Type DWord -Force
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Set-ItemProperty 'HKCU:\Software\ExplorerPatcher' -Name 'TaskbarStyle' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
"#.to_string(),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },

        // ============================================
        // TASKBAR POSITION - TOP (StuckRects3 byte 12 = 01)
        // ============================================
        Tweak {
            id: "taskbar_pos_top".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar: Top".to_string(),
            description: "Move taskbar to top of screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: position_check_script(1), // 01 = Top
                expected_output: "True".to_string(),
            }),
            operations: vec![
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

        // TASKBAR POSITION - LEFT (StuckRects3 byte 12 = 00)
        Tweak {
            id: "taskbar_pos_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar: Left".to_string(),
            description: "Move taskbar to left side of screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: position_check_script(0), // 00 = Left
                expected_output: "True".to_string(),
            }),
            operations: vec![
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

        // TASKBAR POSITION - RIGHT (StuckRects3 byte 12 = 02)
        Tweak {
            id: "taskbar_pos_right".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar: Right".to_string(),
            description: "Move taskbar to right side of screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: position_check_script(2), // 02 = Right
                expected_output: "True".to_string(),
            }),
            operations: vec![
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
            description: "Use smaller taskbar icons (works with Win10 style).".to_string(),
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
        // ALIGNMENT - LEFT (Native, works without EP)
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
        // FULL RESET - Delete all EP settings + restore defaults
        // ============================================
        Tweak {
            id: "ep_full_reset".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Reset All Taskbar".to_string(),
            description: "Delete ExplorerPatcher settings and restore Windows 11 defaults.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Delete EP registry
Remove-Item 'HKCU:\Software\ExplorerPatcher' -Recurse -Force -ErrorAction SilentlyContinue
# Delete EP AppData
Remove-Item "$env:APPDATA\ExplorerPatcher" -Recurse -Force -ErrorAction SilentlyContinue
# Reset native settings
$adv = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
Set-ItemProperty $adv -Name 'TaskbarAl' -Value 1 -Force -ErrorAction SilentlyContinue
Set-ItemProperty $adv -Name 'TaskbarSmallIcons' -Value 0 -Force -ErrorAction SilentlyContinue
Set-ItemProperty $adv -Name 'TaskbarGlomLevel' -Value 0 -Force -ErrorAction SilentlyContinue
# Reset position to bottom
$regPath = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3'
$settings = (Get-ItemProperty $regPath -Name 'Settings' -EA SilentlyContinue).Settings
if ($settings) {
    $settings[12] = 3
    Set-ItemProperty $regPath -Name 'Settings' -Value $settings
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
