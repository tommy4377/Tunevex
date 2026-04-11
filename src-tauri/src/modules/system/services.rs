use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "system_disable_windows_search".to_string(),
            category: TweakCategory::System,
            name: "Disable Windows Search Indexer".to_string(),
            description: "Disables file indexing. Reduces high disk usage and CPU load, but slows down file search results.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "WSearch".to_string(), mode: "auto".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::ServiceDisabled { name: "WSearch".to_string() }),
            operations: vec![
                TweakOperation::ServiceDisable { name: "WSearch".to_string() },
            ]
        },
        Tweak {
            id: "system_set_bits_manual".to_string(),
            category: TweakCategory::System,
            name: "Set BITS to Manual".to_string(),
            description: "Sets Background Intelligent Transfer Service to Manual. Stops it from running constantly in the background.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "BITS".to_string(), mode: "auto".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::ServiceMode { name: "BITS".to_string(), mode: "demand".to_string() }),
            operations: vec![
                TweakOperation::ServiceSetMode { name: "BITS".to_string(), mode: "demand".to_string() },
            ]
        },
        Tweak {
            id: "system_disable_superfetch".to_string(),
            category: TweakCategory::System,
            name: "Disable SysMain (Superfetch)".to_string(),
            description: "Disables SysMain/Superfetch service.

⚠️ MODERN ADVICE (2024+):
On modern systems with 8GB+ RAM and NVMe SSDs, SysMain often HELPS performance by:
- Preloading frequently used apps into RAM
- Using compression to fit more in memory
- Intelligently caching based on usage patterns

Consider disabling ONLY if:
- System has less than 8GB RAM
- Experiencing high disk usage from SysMain
- Running on a slow HDD (rare in 2024)

The old 2013 advice to 'always disable on SSDs' is outdated.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "SysMain".to_string(), mode: "auto".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::ServiceDisabled { name: "SysMain".to_string() }),
            operations: vec![
                TweakOperation::ServiceDisable { name: "SysMain".to_string() },
            ]
        },
    ]
}
