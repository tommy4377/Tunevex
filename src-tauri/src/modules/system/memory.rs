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
May improve disk-heavy gaming (open world games with streaming).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "LargeSystemCache".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "LargeSystemCache".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "LargeSystemCache".to_string(),
                value: RegistryValue::DWord(1),
            }],
        },
        // ============================================
        // Disable Paging Executive
        // ============================================
        Tweak {
            id: "mem_disable_paging_executive".to_string(),
            category: TweakCategory::System,
            name: "Keep Kernel in RAM".to_string(),
            description: "Prevents Windows kernel and drivers from being paged to disk.

Keeps critical system code in RAM for faster access.
Requires 4GB+ RAM. Reduces disk I/O during gaming.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "DisablePagingExecutive".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            revert_operations: Some(vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "DisablePagingExecutive".to_string(),
                value: RegistryValue::DWord(0),
            }]),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "DisablePagingExecutive".to_string(),
                value: RegistryValue::DWord(1),
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
Recommended only for systems with 16GB+ RAM.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$state = Get-MMAgent | Select-Object -ExpandProperty MemoryCompression
if ($state -eq $false) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![TweakOperation::Powershell {
                script: r#"
Enable-MMAgent -MemoryCompression
Write-Host "Memory compression re-enabled" -ForegroundColor Green
"#.to_string(),
            }]),
            operations: vec![TweakOperation::Powershell {
                script: r#"
Disable-MMAgent -MemoryCompression
Write-Host "Memory compression disabled" -ForegroundColor Green
"#.to_string(),
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
Set to a high value (4GB) to consolidate services.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$val = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control" -Name "SvcHostSplitThresholdInKB" -EA 0
if ($val.SvcHostSplitThresholdInKB -ge 4194304) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
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
