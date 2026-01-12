//! Timer Resolution and Boot Configuration Tweaks
//!
//! Based on: bcdedit-tweaks.yml, DisablePowerSaving.ps1

use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_timer_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "cpu_timer_resolution".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Enable Global Timer Resolution Requests".to_string(),
            description: "Allows apps to request higher timer resolution (0.5ms) for smoother frametimes.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                    key: "GlobalTimerResolutionRequests".to_string(),
                }
            ]),
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                    key: "GlobalTimerResolutionRequests".to_string(),
                    value: RegistryValue::DWord(1),
                }
            ]
        },
        Tweak {
            id: "cpu_tsc_sync".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Enable Enhanced TSC Synchronization".to_string(),
            description: "Forces enhanced TSC sync across CPU cores for better timing accuracy.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "tscsyncpolicy".to_string()],
                }
            ]),
            check: None,
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "tscsyncpolicy".to_string(), "Enhanced".to_string()],
                }
            ]
        },
        Tweak {
            id: "cpu_disable_dynamic_tick".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Disable Dynamic Tick".to_string(),
            description: "Forces constant timer interrupts for consistent performance.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "disabledynamictick".to_string()],
                }
            ]),
            check: None,
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "disabledynamictick".to_string(), "yes".to_string()],
                }
            ]
        },
        Tweak {
            id: "cpu_use_platform_clock".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Use Platform Clock (HPET)".to_string(),
            description: "Enables HPET via bcdedit. May improve or hurt performance depending on hardware.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "useplatformclock".to_string()],
                }
            ]),
            check: None,
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "useplatformclock".to_string(), "true".to_string()],
                }
            ]
        },
        Tweak {
            id: "cpu_legacy_boot_menu".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Use Legacy Boot Menu".to_string(),
            description: "Sets legacy boot menu policy for faster boot times.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "bootmenupolicy".to_string(), "standard".to_string()],
                }
            ]),
            check: None,
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "bootmenupolicy".to_string(), "legacy".to_string()],
                }
            ]
        },
        Tweak {
            id: "cpu_processor_check_interval_timer".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Optimize Processor Time Check Interval".to_string(),
            description: "Sets processor time check interval to 200ms for reduced DPCs.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            enabled: false,
            revert_operations: Some(vec![
                 TweakOperation::Powershell {
                    script: r#"
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 15
powercfg /setactive scheme_current
"#.to_string(), // Default 15ms?
                }
            ]),
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 200
powercfg /setactive scheme_current
"#.to_string(),
                }
            ]
        },
    ]
}
