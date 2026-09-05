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
            warning_level: WarningLevel::Careful,
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
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "fsutil".to_string(),
                args: vec!["behavior".to_string(), "query".to_string(), "disablelastaccess".to_string()],
                contains: "DisableLastAccess = 1".to_string(),
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
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["8dot3name".to_string(), "set".to_string(), "2".to_string()], // 2 = Volume Default
            }]),
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "fsutil".to_string(),
                args: vec!["8dot3name".to_string(), "query".to_string()],
                contains: "8dot3 name creation is disabled on all volumes".to_string(),
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
            warning_level: WarningLevel::Careful,
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
$disks = @(Get-CimInstance -Namespace "root/Microsoft/Windows/Storage" -ClassName MSFT_PhysicalDisk -ErrorAction SilentlyContinue)
if ($disks.Count -gt 0 -and @($disks | Where-Object { -not $_.IsWriteCacheEnabled }).Count -eq 0) { "True" } else { "False" }
"#
                .to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$statePath = Join-Path $env:ProgramData "Tunevex\write-cache-state.json"
if (!(Test-Path -LiteralPath $statePath)) { throw "Original disk write-cache state is unavailable: $statePath" }
$saved = @(Get-Content -LiteralPath $statePath -Raw | ConvertFrom-Json)
$disks = @(Get-CimInstance -Namespace "root/Microsoft/Windows/Storage" -ClassName MSFT_PhysicalDisk -ErrorAction Stop)
$failures = @()
foreach ($disk in $disks) {
    $original = $saved | Where-Object { $_.UniqueId -eq $disk.UniqueId } | Select-Object -First 1
    if ($null -eq $original) { continue }
    $result = Invoke-CimMethod -InputObject $disk -MethodName SetWriteCache -Arguments @{ WriteCacheEnabled = [bool]$original.IsWriteCacheEnabled } -ErrorAction Stop
    if ($result.ReturnValue -ne 0) { $failures += "$($disk.FriendlyName): $($result.ReturnValue)" }
}
if ($failures.Count -gt 0) { throw "Could not restore write caching on: $($failures -join ', ')" }
Remove-Item -LiteralPath $statePath -Force
Write-Host "Restored the original write-cache state for supported physical disks." -ForegroundColor Green
"#
                .to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$directory = Join-Path $env:ProgramData "Tunevex"
$statePath = Join-Path $directory "write-cache-state.json"
New-Item -ItemType Directory -Path $directory -Force | Out-Null
$disks = @(Get-CimInstance -Namespace "root/Microsoft/Windows/Storage" -ClassName MSFT_PhysicalDisk -ErrorAction Stop)
if ($disks.Count -eq 0) { throw "Windows did not report any physical disks through MSFT_PhysicalDisk" }
if (!(Test-Path -LiteralPath $statePath)) {
    $disks | Select-Object UniqueId, FriendlyName, IsWriteCacheEnabled | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath $statePath -Encoding UTF8 -Force
}
$failures = @()
foreach ($disk in $disks) {
    $result = Invoke-CimMethod -InputObject $disk -MethodName SetWriteCache -Arguments @{ WriteCacheEnabled = $true } -ErrorAction Stop
    if ($result.ReturnValue -ne 0) { $failures += "$($disk.FriendlyName): $($result.ReturnValue)" }
}
if ($failures.Count -gt 0) { throw "Write caching is unsupported or failed on: $($failures -join ', ')" }
Write-Host "Enabled write caching on all supported physical disks; original state was saved to $statePath." -ForegroundColor Green
"#
                .to_string(),
            }],
        },

        // ============================================
        // C.8: Disable Scheduled Defrag
        // ============================================
        Tweak {
            id: "storage_disable_scheduled_defrag".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable Scheduled Defragmentation".to_string(),
            description: "Disables the Windows scheduled defragmentation task.

Recommended for SSDs where defrag is unnecessary and can cause extra writes.
Windows should auto-detect SSDs, but this ensures it's disabled.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::ScheduledTaskDisabled { name: "\\Microsoft\\Windows\\Defrag\\ScheduledDefrag".to_string() }),
            revert_operations: Some(vec![
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Defrag".to_string(), name: "ScheduledDefrag".to_string() }
            ]),
            operations: vec![
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Defrag".to_string(), name: "ScheduledDefrag".to_string() }
            ],
        },
        // ============================================
        // Memory Usage & Tunneling (Migrated from filesystem)
        // ============================================
        Tweak {
            id: "storage_ntfs_memory_usage".to_string(),
            category: TweakCategory::FileSystem,
            name: "Increase NTFS Memory Cache".to_string(),
            description: "Increases NTFS paged pool memory usage for better metadata caching.".to_string(),
            warning_level: WarningLevel::Careful,
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
        Tweak {
            id: "storage_ntfs_tunneling".to_string(),
            category: TweakCategory::FileSystem,
            name: "Disable NTFS File Tunneling".to_string(),
            description: "Disables metadata preservation (tunneling) on file delete/recreate.".to_string(),
            warning_level: WarningLevel::Careful,
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
