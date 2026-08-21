//! Memory and NTFS Tweaks
//!
//! Based on:
//! - optimize-ntfs.yml
//! - disable-paging.yml
//! - prompt.md memory optimizations

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Returns all memory and NTFS related tweaks
pub fn get_memory_tweaks() -> Vec<Tweak> {
    vec![
        // NOTE: cpu_large_system_cache removed — LargeSystemCache=0 is already the Windows default
        // (key absent is equivalent to 0). Setting it explicitly has no behavioral effect.
        Tweak {
            id: "cpu_disable_paging_executive".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Keep Kernel in RAM".to_string(),
            description: "Prevents kernel and drivers from being paged to disk. Requires sufficient RAM.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                    key: "DisablePagingExecutive".to_string(),
                    value: RegistryValue::DWord(0), // Default
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                key: "DisablePagingExecutive".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management".to_string(),
                    key: "DisablePagingExecutive".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]
        },

        // ============================================
        // NEW: Disable Prefetch
        // ============================================
        Tweak {
            id: "cpu_disable_prefetch".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Prefetch".to_string(),
            description: "Disables Windows Prefetch. Recommended for SSD systems where prefetch provides minimal benefit.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management\\PrefetchParameters".to_string(),
                    key: "EnablePrefetcher".to_string(),
                    value: RegistryValue::DWord(3), // Default (Boot + App)
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management\\PrefetchParameters".to_string(),
                key: "EnablePrefetcher".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management\\PrefetchParameters".to_string(),
                    key: "EnablePrefetcher".to_string(),
                    value: RegistryValue::DWord(0),
                }
            ]
        },
    ]
}
