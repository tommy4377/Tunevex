//! Timer Resolution and Boot Configuration Tweaks
//!
//! Based on: bcdedit-tweaks.yml, DisablePowerSaving.ps1

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_timer_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "cpu_timer_resolution".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Enable Global Timer Resolution Requests".to_string(),
            description: "Allows apps to request higher timer resolution (0.5ms) for smoother frametimes.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                    key: "GlobalTimerResolutionRequests".to_string(),
                }
            ]),
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Kernel".to_string(),
                key: "GlobalTimerResolutionRequests".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
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
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "tscsyncpolicy".to_string()],
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$bcd = bcdedit /enum "{current}" | Select-String "tscsyncpolicy"
if ($bcd -match "Enhanced") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
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
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "disabledynamictick".to_string()],
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$bcd = bcdedit /enum "{current}" | Select-String "disabledynamictick"
if ($bcd -match "Yes") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "disabledynamictick".to_string(), "yes".to_string()],
                }
            ]
        },
        Tweak {
            id: "cpu_disable_hpet".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "⚡ Disable HPET for Lower Latency".to_string(),
            description: "Disables High Precision Event Timer. Modern TSC is faster. Can improve FPS by 10-20% in games.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$hpet = Get-PnpDevice | Where-Object { $_.FriendlyName -like "*High Precision Event Timer*" }
if (-not $hpet -or $hpet.Status -eq "Error" -or $hpet.Status -eq "Disabled") {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "useplatformclock".to_string(), "true".to_string()],
                },
                TweakOperation::Powershell {
                    script: r#"
Get-PnpDevice | Where-Object { $_.FriendlyName -like "*High Precision Event Timer*" } | 
    Enable-PnpDevice -Confirm:$false -EA 0
Write-Host "HPET re-enabled" -ForegroundColor Green
"#.to_string(),
                },
            ]),
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/deletevalue".to_string(), "useplatformclock".to_string()],
                },
                TweakOperation::Powershell {
                    script: r#"
# Also disable HPET device in Device Manager
$hpet = Get-PnpDevice | Where-Object { $_.FriendlyName -like "*High Precision Event Timer*" }
if ($hpet) {
    Disable-PnpDevice -InstanceId $hpet.InstanceId -Confirm:$false -EA 0
    Write-Host "HPET disabled in Device Manager" -ForegroundColor Green
} else {
    Write-Host "HPET device not found (may already be disabled)" -ForegroundColor Yellow
}

Write-Host "`nIMPORTANT: Also disable HPET in BIOS for full effect!" -ForegroundColor Cyan
Write-Host "Location varies by motherboard - look in CPU or Power settings" -ForegroundColor White
"#.to_string(),
                },
            ],
        },
        Tweak {
            id: "cpu_legacy_boot_menu".to_string(),
            category: TweakCategory::CpuPerformance,
            name: "Use Legacy Boot Menu".to_string(),
            description: "Sets legacy boot menu policy for faster boot times.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle, enabled: false,
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "bootmenupolicy".to_string(), "standard".to_string()],
                }
            ]),
            check: Some(TweakCheck::Powershell {
                script: r#"
$bcd = bcdedit /enum "{current}" | Select-String "bootmenupolicy"
if ($bcd -match "Legacy") { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "bcdedit".to_string(),
                    args: vec!["/set".to_string(), "bootmenupolicy".to_string(), "legacy".to_string()],
                }
            ]
        },
        Tweak {
            id: "cpu_processor_check_interval".to_string(), // Renamed to drop "_timer" suffix for consistency if desired, or keep. Plan says "cpu_processor_check_interval".
            category: TweakCategory::CpuPerformance,
            name: "⚡ Optimize Processor Check Interval".to_string(),
            description: "Sets processor performance check interval to 1 (minimum). Reduces latency by checking CPU state more frequently. Windows default is 15.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$result = powercfg /q scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5
if ($result -match "0x00000001") {
    Write-Output "True"
} else {
    Write-Output "False"
}
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
# Restore Windows default (15, NOT 15ms - it's a counter value)
powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 15
powercfg /setdcvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 15
powercfg /setactive scheme_current
Write-Host "Processor check interval restored to default (15)" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Processor Check Interval:
# - Values are timer tick counts, NOT milliseconds
# - 200 = check every 200 ticks (WRONG - increases latency!)
# - 15 = Windows default
# - 1 = check every tick (OPTIMAL for performance)

powercfg /setacvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 1
powercfg /setdcvalueindex scheme_current 54533251-82be-4824-96c1-47b60b740d00 4d2b0152-7d5c-498b-88e2-34345392a2c5 1
powercfg /setactive scheme_current
Write-Host "Processor check interval set to 1 (minimum latency)" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
