//! Taskbar Tweaks Module
//!
//! Controls taskbar position, size, icon alignment, and behavior.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_taskbar_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Taskbar Icon Alignment - Left
        // ============================================
        Tweak {
            id: "taskbar_align_left".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Align Taskbar Icons Left".to_string(),
            description: "Moves taskbar icons to the left side (Windows 10 style).".to_string(),
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
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(1), // Center (default)
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarAl".to_string(),
                value: RegistryValue::DWord(0), // Left
            }],
        },
        // ============================================
        // Taskbar Size - Small
        // ============================================
        Tweak {
            id: "taskbar_size_small".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Small Taskbar".to_string(),
            description: "Reduces taskbar height. May not work on Windows 11 24H2+.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarSi".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Remove-ItemProperty -Path $path -Name "TaskbarSi" -ErrorAction SilentlyContinue
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $path -Name "TaskbarSi" -Value 0 -Type DWord -Force
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }],
        },
        // ============================================
        // Taskbar Size - Large
        // ============================================
        Tweak {
            id: "taskbar_size_large".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Large Taskbar".to_string(),
            description: "Increases taskbar height. May not work on Windows 11 24H2+.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarSi".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Remove-ItemProperty -Path $path -Name "TaskbarSi" -ErrorAction SilentlyContinue
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $path -Name "TaskbarSi" -Value 2 -Type DWord -Force
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }],
        },
        // ============================================
        // Taskbar on Top
        // ============================================
        Tweak {
            id: "taskbar_position_top".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Taskbar on Top".to_string(),
            description: "Moves taskbar to top of screen. May not work on Windows 11 24H2+."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings -EA 0).Settings
if ($data -and $data[12] -eq 0x01) { "True" } else { "False" }
"#
                .to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings).Settings
$data[12] = 0x03
Set-ItemProperty -Path $path -Name Settings -Value $data -Type Binary -Force
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3"
$data = (Get-ItemProperty -Path $path -Name Settings).Settings
$data[12] = 0x01
Set-ItemProperty -Path $path -Name Settings -Value $data -Type Binary -Force
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }],
        },
        // ============================================
        // End Task in Taskbar
        // ============================================
        Tweak {
            id: "taskbar_end_task".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "End Task in Taskbar".to_string(),
            description: "Adds 'End Task' option when right-clicking taskbar apps.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarEndTask".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarEndTask".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKCU".to_string(),
                path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
                key: "TaskbarEndTask".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // ============================================
        // Never Combine Taskbar Buttons
        // ============================================
        Tweak {
            id: "taskbar_never_combine".to_string(),
            category: TweakCategory::InterfaceUx,
            name: "Never Combine Taskbar".to_string(),
            description: "Shows separate buttons for each window (Win10 style).".to_string(),
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
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $path -Name "TaskbarGlomLevel" -Value 0 -Type DWord -Force
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$path = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
Set-ItemProperty -Path $path -Name "TaskbarGlomLevel" -Value 2 -Type DWord -Force
Stop-Process -Name "explorer" -Force -EA 0; Start-Sleep 1; Start-Process "explorer.exe"
"#
                .to_string(),
            }],
        },
    ]
}
