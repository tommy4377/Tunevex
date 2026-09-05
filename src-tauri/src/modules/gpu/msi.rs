use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_gpu_msi_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "gpu_msi_high".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Enable MSI Mode on GPU (High Priority)".to_string(),
            description: "Enables Message Signaled Interrupts (MSI) mode with High Priority for GPU. Reduces latency and improves performance.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MsiEnabledForClass { class: "Display".into(), priority: 3 }),
            revert_operations: Some(vec![
                TweakOperation::MsiRemove { class: "Display".to_string() },
            ]),
            operations: vec![
                TweakOperation::MsiSet { class: "Display".to_string(), priority: 3 },
            ]
        },
    ]
}
