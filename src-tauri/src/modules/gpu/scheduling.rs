use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_scheduling_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Hardware GPU Scheduling (HAGS)
        // ============================================
        Tweak {
            id: "gpu_enable_hwgpu_scheduling".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Enable Hardware GPU Scheduling".to_string(),
            description: "Enables HAGS for lower input latency. Requires compatible GPU (NVIDIA 10-series+, AMD RX 5000+).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            enabled: false,
            check: None,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "HwSchMode".to_string(),
                    value: RegistryValue::DWord(1), // Default off
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "HwSchMode".to_string(),
                    value: RegistryValue::DWord(2), // Enabled
                }
            ]
        },
        
        // ============================================
        // Disable MPO (Multi-Plane Overlay)
        // ============================================
        Tweak {
            id: "gpu_disable_mpo".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Disable Multi-Plane Overlay (MPO)".to_string(),
            description: "Disables MPO to fix stuttering issues on some systems. May increase GPU usage slightly.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            enabled: false,
            check: None,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Dwm".to_string(),
                    key: "OverlayTestMode".to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Dwm".to_string(),
                    key: "OverlayTestMode".to_string(),
                    value: RegistryValue::DWord(5), // Disable MPO
                }
            ]
        },
        Tweak {
            id: "gpu_increase_tdr_delay".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "⏱️ Increase GPU Timeout Delay".to_string(),
            description: "Increases GPU timeout from 2s to 8s. Prevents 'Display driver stopped responding' during heavy loads.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            enabled: false,
            check: None,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "TdrDelay".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "TdrLevel".to_string(),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "TdrDelay".to_string(),
                    value: RegistryValue::DWord(8),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "TdrLevel".to_string(),
                    value: RegistryValue::DWord(3), // Recover on timeout
                },
            ],
        },
    ]
}
