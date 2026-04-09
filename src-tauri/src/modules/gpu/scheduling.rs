use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_scheduling_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "gpu_enable_hwgpu_scheduling".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Enable Hardware GPU Scheduling".to_string(),
            description: "Enables HAGS for lower input latency. Requires compatible GPU (NVIDIA 10-series+, AMD RX 5000+).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                key: "HwSchMode".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "HwSchMode".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                    key: "HwSchMode".to_string(),
                    value: RegistryValue::DWord(2),
                }
            ]
        },
        Tweak {
            id: "gpu_disable_mpo".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Disable Multi-Plane Overlay (MPO)".to_string(),
            description: "Disables MPO to fix stuttering issues on some systems. May increase GPU usage slightly.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Microsoft\\Windows\\Dwm".to_string(),
                key: "OverlayTestMode".to_string(),
                expected_value: RegistryValue::DWord(5),
            }),
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
                    value: RegistryValue::DWord(5),
                }
            ]
        },
        Tweak {
            id: "gpu_increase_tdr_delay".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Increase GPU Timeout Delay".to_string(),
            description: "Increases GPU timeout from 2s to 8s. Prevents 'Display driver stopped responding' during heavy loads.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers".to_string(),
                key: "TdrDelay".to_string(),
                expected_value: RegistryValue::DWord(8),
            }),
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
                    value: RegistryValue::DWord(3),
                },
            ],
        },
        Tweak {
            id: "gpu_disable_nvidia_telemetry".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Disable NVIDIA Telemetry".to_string(),
            description: "Stops and disables NVIDIA telemetry services. Reduces background CPU usage and network traffic.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MultiServiceDisabled {
                names: vec!["NvTelemetryContainer".to_string(), "NVDisplay.ContainerLocalSystem".to_string()],
            }),
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "NvTelemetryContainer".to_string(), mode: "auto".to_string() },
                TweakOperation::ServiceSetMode { name: "NVDisplay.ContainerLocalSystem".to_string(), mode: "auto".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\NVIDIA".to_string(), name: "NvTmMon".to_string() },
            ]),
            operations: vec![
                TweakOperation::ServiceDisable { name: "NvTelemetryContainer".to_string() },
                TweakOperation::ServiceDisable { name: "NVDisplay.ContainerLocalSystem".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\NVIDIA".to_string(), name: "NvTmMon".to_string() },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\NVIDIA Corporation\\NvControlPanel2\\Client".to_string(),
                    key: "OptInOrOutPreference".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },
        Tweak {
            id: "gpu_disable_preemption".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Disable GPU Preemption".to_string(),
            description: "Disables GPU preemption for potentially smoother frame pacing. May reduce micro-stuttering in some games.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers\\Scheduler".to_string(),
                key: "EnablePreemption".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers\\Scheduler".to_string(),
                    key: "EnablePreemption".to_string(),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers\\Scheduler".to_string(),
                    key: "EnablePreemption".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },
    ]
}
