//! File System Tweaks Module
//!
//! NTFS optimizations for improved disk performance.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_filesystem_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Disable 8.3 Short Filename Creation
        // ============================================
        Tweak {
            id: "fs_disable_8dot3".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable 8.3 Short Filenames".to_string(),
            description: "Disables legacy DOS 8.3 short filename creation on NTFS.

Modern software doesn't need these legacy names.
Improves file creation speed and reduces disk overhead.
Safe for Windows 10/11 systems.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$result = fsutil behavior query disable8dot3
if ($result -match "1$" -or $result -match "NtfsDisable8dot3NameCreation *= *1") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["behavior".to_string(), "set".to_string(), "disable8dot3".to_string(), "0".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["behavior".to_string(), "set".to_string(), "disable8dot3".to_string(), "1".to_string()],
            }],
        },
        // ============================================
        // Disable Last Access Timestamp
        // ============================================
        Tweak {
            id: "fs_disable_last_access".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable Last Access Timestamp".to_string(),
            description: "Disables updating file/folder last access time on read operations.

Reduces disk writes significantly, especially with many small files.
Used by AtlasOS and other performance-focused Windows tweaks.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$result = fsutil behavior query disablelastaccess
if ($result -match "1$" -or $result -match "DisableLastAccess *= *1") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["behavior".to_string(), "set".to_string(), "disablelastaccess".to_string(), "0".to_string()],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["behavior".to_string(), "set".to_string(), "disablelastaccess".to_string(), "1".to_string()],
            }],
        },
        // ============================================
        // NTFS Memory Usage
        // ============================================
        Tweak {
            id: "fs_ntfs_memory_usage".to_string(),
            category: TweakCategory::FileSystem,
            name: "Increase NTFS Memory Cache".to_string(),
            description: "Increases NTFS paged pool memory usage.

Allows NTFS to use more RAM for caching file metadata.
Improves performance on drives with many files.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\FileSystem".to_string(),
                key: "NtfsMemoryUsage".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\FileSystem".to_string(),
                key: "NtfsMemoryUsage".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\FileSystem".to_string(),
                key: "NtfsMemoryUsage".to_string(),
                value: RegistryValue::DWord(2),
            }],
        },
        // ============================================
        // Disable NTFS Tunneling
        // ============================================
        Tweak {
            id: "fs_disable_tunneling".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable NTFS File Tunneling".to_string(),
            description: "Disables NTFS file tunneling (metadata preservation on delete/recreate).

Reduces overhead when deleting and recreating files with same name.
Safe tweak recommended by performance guides.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\FileSystem".to_string(),
                key: "MaximumTunnelEntries".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\FileSystem".to_string(),
                key: "MaximumTunnelEntries".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\FileSystem".to_string(),
                key: "MaximumTunnelEntries".to_string(),
                value: RegistryValue::DWord(0),
            }],
        },
    ]
}
