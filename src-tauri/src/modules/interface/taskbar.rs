//! Taskbar Tweaks Module (ExplorerPatcher Integration)
//!
//! Controls taskbar style, position, size, and behavior via ExplorerPatcher and native registry keys.
//! Registry keys verified from ExplorerPatcher documentation and source.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Non-blocking explorer restart script using PowerShell Jobs
const RESTART_EXPLORER: &str = r#"
$null = Start-Job -ScriptBlock {
    Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 2
    Start-Process explorer
}
"#;

pub fn get_taskbar_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // TASKBAR STYLE: Win10 vs Win11
        // HKCU\Software\ExplorerPatcher\TaskbarStyle
        // 0 = Windows 11 (default), 1 = Windows 10
        // ============================================
        Tweak {
            id: "ep_style_win10".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Windows 10 Taskbar".to_string(),
            description: "Use classic Windows 10 taskbar style (enables position/size options)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "TaskbarStyle".to_string(),
                expected_value: RegistryValue::DWord(1), // 1 = Win10
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(0), // 0 = Win11
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },
        // ============================================
        // TASKBAR POSITION - TOP
        // HKCU\Software\ExplorerPatcher\MMTaskbarPosition
        // 0 = Bottom, 1 = Left, 2 = Top, 3 = Right
        // ============================================
        Tweak {
            id: "taskbar_pos_top".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar Position: Top".to_string(),
            description: "Move taskbar to top of screen (requires Win10 style).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "MMTaskbarPosition".to_string(),
                expected_value: RegistryValue::DWord(2), // 2 = Top
            }),
            operations: vec![
                // Force Win10 style first (required for position)
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "MMTaskbarPosition".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "MMTaskbarPosition".to_string(),
                    value: RegistryValue::DWord(0), // 0 = Bottom
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },
        // TASKBAR POSITION - LEFT
        Tweak {
            id: "taskbar_pos_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar Position: Left".to_string(),
            description: "Move taskbar to left side (requires Win10 style).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "MMTaskbarPosition".to_string(),
                expected_value: RegistryValue::DWord(1), // 1 = Left
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "MMTaskbarPosition".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "MMTaskbarPosition".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },
        // TASKBAR POSITION - RIGHT
        Tweak {
            id: "taskbar_pos_right".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar Position: Right".to_string(),
            description: "Move taskbar to right side (requires Win10 style).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\ExplorerPatcher".to_string(),
                key: "MMTaskbarPosition".to_string(),
                expected_value: RegistryValue::DWord(3), // 3 = Right
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "MMTaskbarPosition".to_string(),
                    value: RegistryValue::DWord(3),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "MMTaskbarPosition".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },
        // ============================================
        // TASKBAR ICON SIZE - SMALL
        // Native Windows key: TaskbarSmallIcons
        // 0 = Large (default), 1 = Small
        // ============================================
        Tweak {
            id: "taskbar_icons_small".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Small Taskbar Icons".to_string(),
            description: "Use small icons on taskbar (requires Win10 style).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarSmallIcons".to_string(),
                expected_value: RegistryValue::DWord(1), // 1 = Small
            }),
            operations: vec![
                // Force Win10 style first
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\ExplorerPatcher".to_string(),
                    key: "TaskbarStyle".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
                        .to_string(),
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
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
                        .to_string(),
                    key: "TaskbarSmallIcons".to_string(),
                    value: RegistryValue::DWord(0), // 0 = Large
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },
        // ============================================
        // TASKBAR ALIGNMENT - LEFT
        // Native Windows key: TaskbarAl
        // 0 = Left, 1 = Center (default)
        // ============================================
        Tweak {
            id: "taskbar_align_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Align Icons Left".to_string(),
            description: "Align taskbar icons to the left side.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false, // Takes effect immediately
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                expected_value: RegistryValue::DWord(0), // 0 = Left
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
                value: RegistryValue::DWord(1), // 1 = Center
            }]),
        },
        // ============================================
        // NEVER COMBINE TASKBAR BUTTONS
        // Native Windows key: TaskbarGlomLevel
        // 0 = Always combine, 1 = When full, 2 = Never
        // ============================================
        Tweak {
            id: "taskbar_never_combine".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Never Combine Buttons".to_string(),
            description: "Show separate buttons for each window (requires Win10 style)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarGlomLevel".to_string(),
                expected_value: RegistryValue::DWord(2), // 2 = Never
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
                        .to_string(),
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
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
                        .to_string(),
                    key: "TaskbarGlomLevel".to_string(),
                    value: RegistryValue::DWord(0), // 0 = Always
                },
                TweakOperation::Powershell {
                    script: RESTART_EXPLORER.to_string(),
                },
            ]),
        },
    ]
}
