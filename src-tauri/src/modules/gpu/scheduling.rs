use crate::modules::types::{TweakType, RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, WarningLevel};

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
                    value: RegistryValue::DWord(5), // Disable MPO
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
                    value: RegistryValue::DWord(3), // Recover on timeout
                },
            ],
        },

        // ============================================
        // NVIDIA Telemetry Disable
        // ============================================
        Tweak {
            id: "gpu_disable_nvidia_telemetry".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Disable NVIDIA Telemetry".to_string(),
            description: "Stops and disables NVIDIA telemetry services (NvTelemetryContainer, NvContainerLocalSystem). Reduces background CPU usage and network traffic.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$serviceDisabled = $true
$svc = Get-Service -Name "NvTelemetryContainer" -EA 0
if ($svc -and $svc.StartType -ne 'Disabled') { $serviceDisabled = $false }

$tasksDisabled = $true
$tasks = Get-ScheduledTask | Where-Object { $_.TaskName -like "*NvTmMon*" }
foreach ($t in $tasks) {
    if ($t.State -ne 'Disabled') { $tasksDisabled = $false; break }
}

if ($serviceDisabled -and $tasksDisabled) { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
# Re-enable NVIDIA telemetry services
$services = @('NvTelemetryContainer', 'NVDisplay.ContainerLocalSystem')
foreach ($svc in $services) {
    $service = Get-Service -Name $svc -EA 0
    if ($service) {
        Set-Service -Name $svc -StartupType Automatic -EA 0
        Start-Service -Name $svc -EA 0
        Write-Host "Re-enabled: $svc" -ForegroundColor Green
    }
}

# Re-enable scheduled tasks
Get-ScheduledTask | Where-Object { $_.TaskPath -like "*NVIDIA*" } | 
    Enable-ScheduledTask -EA 0 | Out-Null
Write-Host "NVIDIA telemetry services restored" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$disabled = 0
$notFound = 0

# Disable NVIDIA telemetry services
$services = @('NvTelemetryContainer', 'NVDisplay.ContainerLocalSystem')
foreach ($svc in $services) {
    $service = Get-Service -Name $svc -EA 0
    if ($service) {
        Stop-Service -Name $svc -Force -EA 0
        Set-Service -Name $svc -StartupType Disabled -EA 0
        $disabled++
        Write-Host "Disabled: $svc" -ForegroundColor Green
    } else {
        $notFound++
    }
}

# Disable NVIDIA scheduled tasks
$tasks = Get-ScheduledTask | Where-Object { 
    $_.TaskPath -like "*NVIDIA*" -and $_.TaskName -like "*Telemetry*" 
}
foreach ($task in $tasks) {
    Disable-ScheduledTask -TaskName $task.TaskName -EA 0 | Out-Null
    Write-Host "Disabled task: $($task.TaskName)" -ForegroundColor Green
}

# Block telemetry via registry
$regPath = "HKLM:\SOFTWARE\NVIDIA Corporation\NvControlPanel2\Client"
if (!(Test-Path $regPath)) { New-Item -Path $regPath -Force | Out-Null }
Set-ItemProperty -Path $regPath -Name "OptInOrOutPreference" -Value 0 -Type DWord -Force -EA 0

if ($disabled -gt 0) {
    Write-Host "`nDisabled $disabled NVIDIA telemetry service(s)" -ForegroundColor Green
} else {
    Write-Host "No NVIDIA telemetry services found (AMD/Intel GPU or services not installed)" -ForegroundColor Yellow
}
"#.to_string(),
                }
            ],
        },

        // ============================================
        // GPU Preemption Disable
        // ============================================
        Tweak {
            id: "gpu_disable_preemption".to_string(),
            category: TweakCategory::GpuOptimization,
            name: "Disable GPU Preemption".to_string(),
            description: "Disables GPU preemption for potentially smoother frame pacing. May reduce micro-stuttering in some games.

Note: Effects vary by GPU and game. Test thoroughly.
May slightly increase input latency in some scenarios.".to_string(),
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
