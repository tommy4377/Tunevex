//! System Memory Tweaks
//!
//! Optimizations for memory management, paging, and cache settings.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_memory_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Large System Cache
        // ============================================
        Tweak {
            id: "mem_large_system_cache".to_string(),
            category: TweakCategory::System,
            name: "Enable Large System Cache".to_string(),
            description: "Optimizes file cache for large file operations.

When enabled, Windows uses more RAM for file caching.
Recommended for systems with 8GB+ RAM.
May improve disk-heavy gaming (open world games with streaming)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management"
                    .to_string(),
                key: "LargeSystemCache".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management"
                    .to_string(),
                key: "LargeSystemCache".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management"
                    .to_string(),
                key: "LargeSystemCache".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // ============================================
        // Set Fixed Page File Size (4GB)
        // ============================================
        Tweak {
            id: "mem_fixed_pagefile".to_string(),
            category: TweakCategory::System,
            name: "Set Fixed Page File (4GB)".to_string(),
            description:
                "Sets a fixed 4GB page file to prevent dynamic resizing I/O stalls during gaming.

WARNING: Requires at least 4GB free disk space on system drive."
                    .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "$cs = Get-CimInstance Win32_ComputerSystem -ErrorAction Stop; $p = @(Get-CimInstance Win32_PageFileSetting -ErrorAction Stop); (-not $cs.AutomaticManagedPagefile) -and $p.Count -eq 1 -and $p[0].Name -eq \"$env:SystemDrive\\pagefile.sys\" -and $p[0].InitialSize -eq 4096 -and $p[0].MaximumSize -eq 4096".into(),
                expected_output: "True".into(),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$file = Join-Path $env:ProgramData 'TommyTweaker\backups\pagefile.json'
if (!(Test-Path -LiteralPath $file)) { throw 'No saved pagefile configuration; refusing to guess the previous configuration' }
$saved = Get-Content -Raw -LiteralPath $file | ConvertFrom-Json
$cs = Get-CimInstance Win32_ComputerSystem
$cs | Set-CimInstance -Property @{ AutomaticManagedPagefile = $false }
Get-CimInstance Win32_PageFileSetting | Remove-CimInstance
foreach ($p in $saved.Pages) {
    New-CimInstance -ClassName Win32_PageFileSetting -Property @{ Name = [string]$p.Name; InitialSize = [uint32]$p.InitialSize; MaximumSize = [uint32]$p.MaximumSize } | Out-Null
}
$cs | Set-CimInstance -Property @{ AutomaticManagedPagefile = [bool]$saved.Automatic }
Remove-Item -LiteralPath $file
"#.to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$file = Join-Path $env:ProgramData 'TommyTweaker\backups\pagefile.json'
$cs = Get-CimInstance Win32_ComputerSystem
if (!(Test-Path -LiteralPath $file)) {
    $saved = @{ Automatic = $cs.AutomaticManagedPagefile; Pages = @(Get-CimInstance Win32_PageFileSetting | Select-Object Name, InitialSize, MaximumSize) }
    New-Item -ItemType Directory -Force -Path (Split-Path $file) | Out-Null
    $saved | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $file -ErrorAction Stop
}
$cs | Set-CimInstance -Property @{ AutomaticManagedPagefile = $false }
Get-CimInstance Win32_PageFileSetting | Remove-CimInstance
New-CimInstance -ClassName Win32_PageFileSetting -Property @{ Name = "$env:SystemDrive\pagefile.sys"; InitialSize = [uint32]4096; MaximumSize = [uint32]4096 } | Out-Null
"#.to_string(),
            }],
        },
        // ============================================
        // Disable Memory Compression
        // ============================================
        Tweak {
            id: "mem_disable_compression".to_string(),
            category: TweakCategory::System,
            name: "Disable Memory Compression".to_string(),
            description: "Disables Windows memory compression feature.

Memory compression uses CPU to compress RAM contents.
Disabling frees CPU cycles but may increase disk paging.
Recommended only for systems with 16GB+ RAM."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powershell".to_string(),
                args: vec![
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    "(Get-MMAgent).MemoryCompression".to_string(),
                ],
                contains: "False".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "powershell".to_string(),
                args: vec![
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    "Enable-MMAgent -MemoryCompression".to_string(),
                ],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "powershell".to_string(),
                args: vec![
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    "Disable-MMAgent -MemoryCompression".to_string(),
                ],
            }],
        },
        // ============================================
        // SvcHost Split Threshold
        // ============================================
        Tweak {
            id: "mem_svchost_split".to_string(),
            category: TweakCategory::System,
            name: "Optimize Service Host Splitting".to_string(),
            description: "Configures Windows to group more services into single svchost processes.

Reduces memory overhead from many small svchost.exe processes.
Set to a high value (4GB) to consolidate services."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control".to_string(),
                key: "SvcHostSplitThresholdInKB".to_string(),
                expected_value: RegistryValue::DWord(4194304),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control".to_string(),
                key: "SvcHostSplitThresholdInKB".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control".to_string(),
                key: "SvcHostSplitThresholdInKB".to_string(),
                value: RegistryValue::DWord(4194304), // 4GB in KB
            }],
        },
    ]
}$file = Join-Path $env:ProgramData 'Tunevex\backups\pagefile.json'
if (!(Test-Path -LiteralPath $file)) { $file = Join-Path $env:ProgramData 'TommyTweaker\backups\pagefile.json' }ging, and cache settings.

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_memory_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Large System Cache
        // ============================================
        Tweak {
            id: "mem_large_system_cache".to_string(),
            category: TweakCategory::System,
            name: "Enable Large System Cache".to_string(),
            description: "Optimizes file cache for large file operations.

When enabled, Windows uses more RAM for file caching.
Recommended for systems with 8GB+ RAM.
May improve disk-heavy gaming (open world games with streaming)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management"
                    .to_string(),
                key: "LargeSystemCache".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management"
                    .to_string(),
                key: "LargeSystemCache".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management"
                    .to_string(),
                key: "LargeSystemCache".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // ============================================
        // Set Fixed Page File Size (4GB)
        // ============================================
        Tweak {
            id: "mem_fixed_pagefile".to_string(),
            category: TweakCategory::System,
            name: "Set Fixed Page File (4GB)".to_string(),
            description:
                "Sets a fixed 4GB page file to prevent dynamic resizing I/O stalls during gaming.

WARNING: Requires at least 4GB free disk space on system drive."
                    .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: "$cs = Get-CimInstance Win32_ComputerSystem -ErrorAction Stop; $p = @(Get-CimInstance Win32_PageFileSetting -ErrorAction Stop); (-not $cs.AutomaticManagedPagefile) -and $p.Count -eq 1 -and $p[0].Name -eq \"$env:SystemDrive\\pagefile.sys\" -and $p[0].InitialSize -eq 4096 -and $p[0].MaximumSize -eq 4096".into(),
                expected_output: "True".into(),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$file = Join-Path $env:ProgramData 'TommyTweaker\backups\pagefile.json'
if (!(Test-Path -LiteralPath $file)) { throw 'No saved pagefile configuration; refusing to guess the previous configuration' }
$saved = Get-Content -Raw -LiteralPath $file | ConvertFrom-Json
$cs = Get-CimInstance Win32_ComputerSystem
$cs | Set-CimInstance -Property @{ AutomaticManagedPagefile = $false }
Get-CimInstance Win32_PageFileSetting | Remove-CimInstance
foreach ($p in $saved.Pages) {
    New-CimInstance -ClassName Win32_PageFileSetting -Property @{ Name = [string]$p.Name; InitialSize = [uint32]$p.InitialSize; MaximumSize = [uint32]$p.MaximumSize } | Out-Null
}
$cs | Set-CimInstance -Property @{ AutomaticManagedPagefile = [bool]$saved.Automatic }
Remove-Item -LiteralPath $file
"#.to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$file = Join-Path $env:ProgramData 'TommyTweaker\backups\pagefile.json'
$cs = Get-CimInstance Win32_ComputerSystem
if (!(Test-Path -LiteralPath $file)) {
    $saved = @{ Automatic = $cs.AutomaticManagedPagefile; Pages = @(Get-CimInstance Win32_PageFileSetting | Select-Object Name, InitialSize, MaximumSize) }
    New-Item -ItemType Directory -Force -Path (Split-Path $file) | Out-Null
    $saved | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $file -ErrorAction Stop
}
$cs | Set-CimInstance -Property @{ AutomaticManagedPagefile = $false }
Get-CimInstance Win32_PageFileSetting | Remove-CimInstance
New-CimInstance -ClassName Win32_PageFileSetting -Property @{ Name = "$env:SystemDrive\pagefile.sys"; InitialSize = [uint32]4096; MaximumSize = [uint32]4096 } | Out-Null
"#.to_string(),
            }],
        },
        // ============================================
        // Disable Memory Compression
        // ============================================
        Tweak {
            id: "mem_disable_compression".to_string(),
            category: TweakCategory::System,
            name: "Disable Memory Compression".to_string(),
            description: "Disables Windows memory compression feature.

Memory compression uses CPU to compress RAM contents.
Disabling frees CPU cycles but may increase disk paging.
Recommended only for systems with 16GB+ RAM."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "powershell".to_string(),
                args: vec![
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    "(Get-MMAgent).MemoryCompression".to_string(),
                ],
                contains: "False".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "powershell".to_string(),
                args: vec![
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    "Enable-MMAgent -MemoryCompression".to_string(),
                ],
            }]),
            operations: vec![TweakOperation::Command {
                cmd: "powershell".to_string(),
                args: vec![
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    "Disable-MMAgent -MemoryCompression".to_string(),
                ],
            }],
        },
        // ============================================
        // SvcHost Split Threshold
        // ============================================
        Tweak {
            id: "mem_svchost_split".to_string(),
            category: TweakCategory::System,
            name: "Optimize Service Host Splitting".to_string(),
            description: "Configures Windows to group more services into single svchost processes.

Reduces memory overhead from many small svchost.exe processes.
Set to a high value (4GB) to consolidate services."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control".to_string(),
                key: "SvcHostSplitThresholdInKB".to_string(),
                expected_value: RegistryValue::DWord(4194304),
            }),
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control".to_string(),
                key: "SvcHostSplitThresholdInKB".to_string(),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control".to_string(),
                key: "SvcHostSplitThresholdInKB".to_string(),
                value: RegistryValue::DWord(4194304), // 4GB in KB
            }],
        },
    ]
}
