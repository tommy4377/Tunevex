use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

/// Windows Core Telemetry
/// From: disallow-data-collection.yml, disable-diagnostic-tracing.yml, disable-ceip.yml
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // Comprehensive Windows Telemetry Disable
        // From: disallow-data-collection.yml
        // ============================================
        Tweak {
            id: "priv_disable_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Windows Telemetry".to_string(),
            description: "Sets AllowTelemetry=0 in all locations, disables DiagTrack service, clears telemetry logs.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "MaxTelemetryAllowed".to_string(),
                },
                // Delete Policy Keys to restore default behavior
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowDeviceNameInTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                },
                // Re-enable EventTranscript
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Diagnostics\\DiagTrack\\EventTranscriptKey".to_string(),
                    key: "EnableEventTranscript".to_string(),
                    value: RegistryValue::DWord(1),
                },
                // Re-enable DiagTrack Autologger
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\WMI\\Autologger\\Diagtrack-Listener".to_string(),
                    key: "Start".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                key: "AllowTelemetry".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                // AllowTelemetry in DataCollection (multiple locations)
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "MaxTelemetryAllowed".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    key: "AllowDeviceNameInTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Policies\\DataCollection".to_string(),
                    key: "AllowTelemetry".to_string(),
                    value: RegistryValue::DWord(0),
                },
                // Disable EventTranscript
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Diagnostics\\DiagTrack\\EventTranscriptKey".to_string(),
                    key: "EnableEventTranscript".to_string(),
                    value: RegistryValue::DWord(0),
                },
                // Disable DiagTrack Autologger
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\WMI\\Autologger\\Diagtrack-Listener".to_string(),
                    key: "Start".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
        },
        
        // CEIP - kept in privacy (registry-based)
        Tweak {
            id: "priv_disable_ceip".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Customer Experience Improvement Program".to_string(),
            description: "Disables Windows CEIP data collection via registry.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\SQMClient\\Windows".to_string(),
                key: "CEIPEnable".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\SQMClient\\Windows".to_string(),
                    key: "CEIPEnable".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Disable Windows Error Reporting
        Tweak {
            id: "priv_disable_wer".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Windows Error Reporting".to_string(),
            description: "Prevents Windows from sending error reports to Microsoft.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: r#"
Set-Service -Name 'WerSvc' -StartupType Manual -EA 0
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                key: "Disabled".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting".to_string(),
                    key: "Disabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::Powershell {
                    script: r#"
Stop-Service -Name 'WerSvc' -Force -EA 0
Set-Service -Name 'WerSvc' -StartupType Disabled -EA 0
"#.to_string(),
                }
            ]
        },
        
        // Disable Input Telemetry
        Tweak {
            id: "priv_disable_input_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Input Telemetry".to_string(),
            description: "Disables inking and typing data collection for personalization.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitInkCollection".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitTextCollection".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization\\TrainedDataStore".to_string(),
                    key: "HarvestContacts".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Personalization\\Settings".to_string(),
                    key: "AcceptedPrivacyPolicy".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKCU".to_string(),
                path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                key: "RestrictImplicitInkCollection".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitInkCollection".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization".to_string(),
                    key: "RestrictImplicitTextCollection".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\InputPersonalization\\TrainedDataStore".to_string(),
                    key: "HarvestContacts".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Personalization\\Settings".to_string(),
                    key: "AcceptedPrivacyPolicy".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // Disable Scheduled Telemetry Tasks
        Tweak {
            id: "priv_disable_telemetry_tasks".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Telemetry Scheduled Tasks".to_string(),
            description: "Disables Microsoft Compatibility Appraiser, ProgramDataUpdater, and other telemetry tasks.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$tasks = @(
    '\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser',
    '\Microsoft\Windows\Application Experience\ProgramDataUpdater',
    '\Microsoft\Windows\Autochk\Proxy',
    '\Microsoft\Windows\Customer Experience Improvement Program\Consolidator',
    '\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip',
    '\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector',
    '\Microsoft\Windows\Feedback\Siuf\DmClient',
    '\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload',
    '\Microsoft\Windows\PI\Sqm-Tasks'
)
foreach ($task in $tasks) {
    schtasks /Change /TN $task /Enable 2>$null
}
Write-Host "Telemetry scheduled tasks enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$t = Get-ScheduledTask -TaskName "Microsoft Compatibility Appraiser" -ErrorAction SilentlyContinue
if ($t.State -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$tasks = @(
    '\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser',
    '\Microsoft\Windows\Application Experience\ProgramDataUpdater',
    '\Microsoft\Windows\Autochk\Proxy',
    '\Microsoft\Windows\Customer Experience Improvement Program\Consolidator',
    '\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip',
    '\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector',
    '\Microsoft\Windows\Feedback\Siuf\DmClient',
    '\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload',
    '\Microsoft\Windows\PI\Sqm-Tasks'
)
foreach ($task in $tasks) {
    schtasks /Change /TN $task /Disable 2>$null
}
Write-Host "Telemetry scheduled tasks disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        // Disable NVIDIA Telemetry
        Tweak {
            id: "priv_disable_nvidia_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable NVIDIA Telemetry".to_string(),
            description: "Disables NVIDIA Telemetry Container service and scheduled tasks (NvTmMon, NvTmRep).".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Re-enabling NVIDIA Telemetry..." -ForegroundColor Yellow

# 1. Enable Service
$serviceName = 'NvTelemetryContainer'
if (Get-Service -Name $serviceName -ErrorAction SilentlyContinue) {
    Set-Service -Name $serviceName -StartupType Automatic -ErrorAction SilentlyContinue
    Start-Service -Name $serviceName -ErrorAction SilentlyContinue
    Write-Host "Service $serviceName enabled." -ForegroundColor Green
} else {
    Write-Host "Service $serviceName not found." -ForegroundColor Gray
}

# 2. Enable Tasks
$tasks = Get-ScheduledTask | Where-Object { $_.TaskName -like 'NvTm*' }
foreach ($task in $tasks) {
    Enable-ScheduledTask -TaskName $task.TaskName -ErrorAction SilentlyContinue
    Write-Host "Task $($task.TaskName) enabled." -ForegroundColor Green
}

# 3. Registry (Optional - revert opting out)
# We won't forcefully set 'OptIn' because that imposes a choice, but we can delete the 'OptOut' force keys if we want.
# For now, enabling services/tasks is the main revert action.
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(crate::modules::types::TweakCheck::Powershell {
                script: r#"
$s = Get-Service -Name 'NvTelemetryContainer' -ErrorAction SilentlyContinue
$t = Get-ScheduledTask | Where-Object { $_.TaskName -like 'NvTm*' -and $_.State -eq 'Ready' }
if (($s -and $s.StartType -ne 'Disabled') -or $t) { return 'False' }
return 'True'
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Disabling NVIDIA Telemetry..." -ForegroundColor Yellow

# 1. Disable Service
$serviceName = 'NvTelemetryContainer'
if (Get-Service -Name $serviceName -ErrorAction SilentlyContinue) {
    Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue
    Set-Service -Name $serviceName -StartupType Disabled -ErrorAction SilentlyContinue
    Write-Host "Service $serviceName disabled." -ForegroundColor Green
}

# 2. Disable Tasks
$tasks = Get-ScheduledTask | Where-Object { $_.TaskName -like 'NvTm*' }
foreach ($task in $tasks) {
    Disable-ScheduledTask -TaskName $task.TaskName -ErrorAction SilentlyContinue
    Write-Host "Task $($task.TaskName) disabled." -ForegroundColor Green
}

# 3. Registry
$regKeys = @(
    "HKLM:\SOFTWARE\NVIDIA Corporation\NvControlPanel2\Client",
    "HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS"
)
foreach ($key in $regKeys) {
    if (-not (Test-Path $key)) { New-Item -Path $key -Force | Out-Null }
}
Set-ItemProperty -Path "HKLM:\SOFTWARE\NVIDIA Corporation\NvControlPanel2\Client" -Name "OptInOrOutPreference" -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
Set-ItemProperty -Path "HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS" -Name "EnableRID44231" -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
Set-ItemProperty -Path "HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS" -Name "EnableRID64640" -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
Set-ItemProperty -Path "HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS" -Name "EnableRID66610" -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
"#.to_string(),
                }
            ]
        },
    ]
}
