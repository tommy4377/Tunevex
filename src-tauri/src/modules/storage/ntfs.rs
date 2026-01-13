use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Returns all NTFS filesystem tweaks
pub fn get_ntfs_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "storage_ntfs_lastaccess".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable NTFS Last Access Time".to_string(),
            description:
                "Disables last access time updates on files for improved performance and privacy."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec![
                    "behavior".to_string(),
                    "set".to_string(),
                    "disablelastaccess".to_string(),
                    "2".to_string(),
                ], // 2 = System Managed (Default)
            }]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$res = fsutil behavior query disablelastaccess
if ($res -match "1") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec![
                    "behavior".to_string(),
                    "set".to_string(),
                    "disablelastaccess".to_string(),
                    "1".to_string(),
                ],
            }],
        },
        Tweak {
            id: "storage_ntfs_8dot3".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable NTFS 8.3 Name Creation".to_string(),
            description: "Disables legacy 8.3 short filename creation for improved performance."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["8dot3name".to_string(), "set".to_string(), "2".to_string()], // 2 = Volume Default
            }]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$res = fsutil behavior query 8dot3name
if ($res -match "1") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["8dot3name".to_string(), "set".to_string(), "1".to_string()],
            }],
        },

        // ============================================
        // C.5: Native NVMe Driver (Windows 11 24H2+)
        // ============================================
        Tweak {
            id: "storage_native_nvme_driver".to_string(),
            category: TweakCategory::FileSystem,
            name: "Enable Native NVMe Driver (24H2+)".to_string(),
            description: "Enables the new native Windows NVMe driver for improved performance.

Only works on Windows 11 24H2 or newer.
Provides lower latency and better IOPS for NVMe SSDs.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\FeatureManagement\\Overrides\\0\\1176759950".to_string(),
                key: "EnabledState".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\FeatureManagement\\Overrides\\0\\1176759950".to_string(),
                    key: "EnabledState".to_string(),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\FeatureManagement\\Overrides\\0\\1176759950".to_string(),
                    key: "EnabledState".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],
        },

        // ============================================
        // C.7: Storage Write-Cache
        // ============================================
        Tweak {
            id: "storage_enable_write_cache".to_string(),
            category: TweakCategory::FileSystem,
            name: "Enable Disk Write Caching".to_string(),
            description: "Enables write caching on all physical disks for improved write performance.

WARNING: Data loss may occur during power failure.
Recommended only for systems with UPS or laptops with good battery.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$disks = Get-PhysicalDisk
if (($disks | Where-Object { $_.WriteCacheEnabled -ne $true }).Count -eq 0) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Get-PhysicalDisk | Set-PhysicalDisk -WriteCacheEnabled $false
Write-Host "Write caching disabled on all physical disks" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Get-PhysicalDisk | Set-PhysicalDisk -WriteCacheEnabled $true
Write-Host "Write caching enabled on all physical disks" -ForegroundColor Green
Write-Host "Ensure you have a UPS to prevent data loss on power failure" -ForegroundColor Yellow
"#.to_string(),
                }
            ],
        },

        // ============================================
        // C.8: Disable Scheduled Defrag
        // ============================================
        Tweak {
            id: "storage_disable_scheduled_defrag".to_string(),
            category: TweakCategory::FileSystem,
            name: "🛑 Disable Scheduled Defragmentation".to_string(),
            description: "Disables the Windows scheduled defragmentation task.

Recommended for SSDs where defrag is unnecessary and can cause extra writes.
Windows should auto-detect SSDs, but this ensures it's disabled.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
if ((Get-ScheduledTask -TaskName "ScheduledDefrag" -TaskPath "\Microsoft\Windows\Defrag\" -ErrorAction SilentlyContinue).State -match 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Enable-ScheduledTask -TaskPath "\Microsoft\Windows\Defrag\" -TaskName "ScheduledDefrag" -EA 0
Write-Host "Scheduled defragmentation re-enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Disable-ScheduledTask -TaskPath "\Microsoft\Windows\Defrag\" -TaskName "ScheduledDefrag" -EA 0
Write-Host "Scheduled defragmentation disabled" -ForegroundColor Green
"#.to_string(),
                }
            ],
        },
    ]
}
