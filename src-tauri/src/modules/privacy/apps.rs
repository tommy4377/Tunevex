use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

/// Application Telemetry (NVIDIA, Office, VS, Chrome, Firefox, etc.)
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // ============================================
        // NVIDIA Telemetry (from scripts + research)
        // ============================================
        Tweak {
            id: "priv_nvidia_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "🎮 Disable NVIDIA Telemetry".to_string(),
            description: "Disables NVIDIA telemetry: services, scheduled tasks, registry keys. From privacy.sexy scripts.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Re-enabling NVIDIA Telemetry..." -ForegroundColor Yellow

# Registry keys
$nvPaths = @(
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\NvControlPanel2\Client"; Name="OptInOrOutPreference"; Value=1},
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS"; Name="EnableRID44231"; Value=1},
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS"; Name="EnableRID64640"; Value=1},
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS"; Name="EnableRID66610"; Value=1},
    @{Path="HKLM:\SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\Startup"; Name="SendTelemetryData"; Value=1}
)
foreach ($item in $nvPaths) {
    if (Test-Path $item.Path) {
        Set-ItemProperty -Path $item.Path -Name $item.Name -Value $item.Value -Type DWord -Force -EA 0
    }
}

# Enable NvTelemetryContainer service
Set-Service -Name 'NvTelemetryContainer' -StartupType Automatic -EA 0
Start-Service -Name 'NvTelemetryContainer' -EA 0

# Enable scheduled tasks
schtasks /change /TN "NvTmMon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}" /ENABLE 2>$null
schtasks /change /TN "NvTmRep_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}" /ENABLE 2>$null
schtasks /change /TN "NvTmRepOnLogon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}" /ENABLE 2>$null

Write-Host "NVIDIA telemetry re-enabled (files cannot be restored)" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Disabling NVIDIA Telemetry..." -ForegroundColor Yellow

# Registry keys
$nvPaths = @(
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\NvControlPanel2\Client"; Name="OptInOrOutPreference"; Value=0},
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS"; Name="EnableRID44231"; Value=0},
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS"; Name="EnableRID64640"; Value=0},
    @{Path="HKLM:\SOFTWARE\NVIDIA Corporation\Global\FTS"; Name="EnableRID66610"; Value=0},
    @{Path="HKLM:\SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\Startup"; Name="SendTelemetryData"; Value=0}
)
foreach ($item in $nvPaths) {
    if (Test-Path $item.Path) {
        Set-ItemProperty -Path $item.Path -Name $item.Name -Value $item.Value -Type DWord -Force -EA 0
    }
}

# Disable NvTelemetryContainer service
$svc = Get-Service -Name 'NvTelemetryContainer' -EA 0
if ($svc) {
    Stop-Service -Name 'NvTelemetryContainer' -Force -EA 0
    Set-Service -Name 'NvTelemetryContainer' -StartupType Disabled -EA 0
}

# Disable scheduled tasks
schtasks /change /TN "NvTmMon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}" /DISABLE 2>$null
schtasks /change /TN "NvTmRep_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}" /DISABLE 2>$null
schtasks /change /TN "NvTmRepOnLogon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}" /DISABLE 2>$null

# Clear telemetry files
Remove-Item "$env:ProgramData\NVIDIA Corporation\NVTelemetry" -Recurse -Force -EA 0
Remove-Item "$env:ProgramData\NVIDIA Corporation\CrashDumps" -Recurse -Force -EA 0

Write-Host "NVIDIA telemetry disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // Office Telemetry
        Tweak {
            id: "priv_office_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "📄 Disable Microsoft Office Telemetry".to_string(),
            description: "Disables Office telemetry, CEIP, and customer data collection.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "sendcustomerdata".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\common\\clienttelemetry".to_string(),
                    key: "sendtelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "qmenable".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Office\\Common\\ClientTelemetry".to_string(),
                    key: "DisableTelemetry".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "sendcustomerdata".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\common\\clienttelemetry".to_string(),
                    key: "sendtelemetry".to_string(),
                    value: RegistryValue::DWord(3),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\office\\16.0\\common".to_string(),
                    key: "qmenable".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Office\\Common\\ClientTelemetry".to_string(),
                    key: "DisableTelemetry".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Visual Studio Telemetry
        Tweak {
            id: "priv_vs_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "💻 Disable Visual Studio Telemetry".to_string(),
            description: "Disables Visual Studio telemetry, VSCEIP, feedback, and IntelliCode collection.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Re-enabling Visual Studio telemetry..." -ForegroundColor Yellow

# VSCEIP SQM OptIn (enable = 1)
$versions = @("14.0", "15.0", "16.0", "17.0")
foreach ($v in $versions) {
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Wow6432Node\Microsoft\VSCommon\$v\SQM" -Name "OptIn" -Value 1 -Type DWord -Force -EA 0
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\VSCommon\$v\SQM" -Name "OptIn" -Value 1 -Type DWord -Force -EA 0
}

# Policy SQM
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\SQM" -Name "OptIn" -Force -EA 0

# Telemetry TurnOffSwitch
Remove-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\VisualStudio\Telemetry" -Name "TurnOffSwitch" -Force -EA 0

# Feedback
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback" -Name "DisableFeedbackDialog" -Force -EA 0
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback" -Name "DisableEmailInput" -Force -EA 0
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback" -Name "DisableScreenshotCapture" -Force -EA 0

# IntelliCode
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\IntelliCode" -Name "DisableRemoteAnalysis" -Force -EA 0

# Enable VSStandardCollectorService
Set-Service -Name 'VSStandardCollectorService150' -StartupType Manual -EA 0

Write-Host "Visual Studio telemetry re-enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Disabling Visual Studio telemetry..." -ForegroundColor Yellow

# VSCEIP SQM OptIn (both architectures)
$versions = @("14.0", "15.0", "16.0", "17.0")
foreach ($v in $versions) {
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Wow6432Node\Microsoft\VSCommon\$v\SQM" -Name "OptIn" -Value 0 -Type DWord -Force -EA 0
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\VSCommon\$v\SQM" -Name "OptIn" -Value 0 -Type DWord -Force -EA 0
}

# Policy SQM
$sqmPath = "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\SQM"
if (!(Test-Path $sqmPath)) { New-Item -Path $sqmPath -Force | Out-Null }
Set-ItemProperty -Path $sqmPath -Name "OptIn" -Value 0 -Type DWord -Force

# Telemetry TurnOffSwitch
Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\VisualStudio\Telemetry" -Name "TurnOffSwitch" -Value 1 -Type DWord -Force -EA 0

# Feedback
$fbPath = "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback"
if (!(Test-Path $fbPath)) { New-Item -Path $fbPath -Force | Out-Null }
Set-ItemProperty -Path $fbPath -Name "DisableFeedbackDialog" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $fbPath -Name "DisableEmailInput" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $fbPath -Name "DisableScreenshotCapture" -Value 1 -Type DWord -Force

# IntelliCode
$icPath = "HKLM:\SOFTWARE\Policies\Microsoft\VisualStudio\IntelliCode"
if (!(Test-Path $icPath)) { New-Item -Path $icPath -Force | Out-Null }
Set-ItemProperty -Path $icPath -Name "DisableRemoteAnalysis" -Value 1 -Type DWord -Force

# Disable VSStandardCollectorService
Stop-Service -Name 'VSStandardCollectorService150' -Force -EA 0
Set-Service -Name 'VSStandardCollectorService150' -StartupType Disabled -EA 0

Write-Host "Visual Studio telemetry disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // VS Code Telemetry
        Tweak {
            id: "priv_vscode_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "📝 Disable VS Code Telemetry".to_string(),
            description: "Modifies VS Code settings.json to disable telemetry, crash reports, and experiments.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$settingsPath = "$env:APPDATA\Code\User\settings.json"
if (Test-Path $settingsPath) {
    try {
        $json = Get-Content $settingsPath -Raw | ConvertFrom-Json
    } catch {
        $json = [PSCustomObject]@{}
    }
    # Restore defaults
    $json | Add-Member -NotePropertyName "telemetry.telemetryLevel" -NotePropertyValue "all" -Force
    $json | Add-Member -NotePropertyName "telemetry.enableTelemetry" -NotePropertyValue $true -Force
    $json | Add-Member -NotePropertyName "telemetry.enableCrashReporter" -NotePropertyValue $true -Force
    $json | Add-Member -NotePropertyName "workbench.enableExperiments" -NotePropertyValue $true -Force
    $json | Add-Member -NotePropertyName "update.showReleaseNotes" -NotePropertyValue $true -Force
    $json | ConvertTo-Json -Depth 10 | Set-Content $settingsPath
    Write-Host "VS Code telemetry re-enabled" -ForegroundColor Green
}
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$settingsPath = "$env:APPDATA\Code\User\settings.json"
if (Test-Path $settingsPath) {
    try {
        $json = Get-Content $settingsPath -Raw | ConvertFrom-Json
    } catch {
        $json = [PSCustomObject]@{}
    }
    $json | Add-Member -NotePropertyName "telemetry.telemetryLevel" -NotePropertyValue "off" -Force
    $json | Add-Member -NotePropertyName "telemetry.enableTelemetry" -NotePropertyValue $false -Force
    $json | Add-Member -NotePropertyName "telemetry.enableCrashReporter" -NotePropertyValue $false -Force
    $json | Add-Member -NotePropertyName "workbench.enableExperiments" -NotePropertyValue $false -Force
    $json | Add-Member -NotePropertyName "update.showReleaseNotes" -NotePropertyValue $false -Force
    $json | ConvertTo-Json -Depth 10 | Set-Content $settingsPath
    Write-Host "VS Code telemetry disabled" -ForegroundColor Green
} else {
    Write-Host "VS Code settings.json not found (VS Code may not be installed)" -ForegroundColor Yellow
}
"#.to_string(),
                }
            ]
        },
        
        // .NET CLI Telemetry
        Tweak {
            id: "priv_dotnet_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable .NET CLI Telemetry".to_string(),
            description: "Sets DOTNET_CLI_TELEMETRY_OPTOUT environment variable.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
[Environment]::SetEnvironmentVariable("DOTNET_CLI_TELEMETRY_OPTOUT", $null, "Machine")
Write-Host ".NET CLI telemetry opt-out removed" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
[Environment]::SetEnvironmentVariable("DOTNET_CLI_TELEMETRY_OPTOUT", "1", "Machine")
Write-Host ".NET CLI telemetry disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // PowerShell Telemetry
        Tweak {
            id: "priv_powershell_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable PowerShell Telemetry".to_string(),
            description: "Sets POWERSHELL_TELEMETRY_OPTOUT environment variable.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
[Environment]::SetEnvironmentVariable("POWERSHELL_TELEMETRY_OPTOUT", $null, "Machine")
Write-Host "PowerShell telemetry opt-out removed" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
[Environment]::SetEnvironmentVariable("POWERSHELL_TELEMETRY_OPTOUT", "1", "Machine")
Write-Host "PowerShell telemetry disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // Chrome Telemetry
        Tweak {
            id: "priv_chrome_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "🌐 Disable Chrome Telemetry".to_string(),
            description: "Disables Chrome metrics reporting and software reporter tool.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "MetricsReportingEnabled".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupReportingEnabled".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupEnabled".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "MetricsReportingEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupReportingEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Google\\Chrome".to_string(),
                    key: "ChromeCleanupEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Firefox Telemetry (registry for enterprise)
        Tweak {
            id: "priv_firefox_telemetry".to_string(),
            category: TweakCategory::Privacy,
            name: "🦊 Disable Firefox Telemetry (Enterprise)".to_string(),
            description: "Sets Firefox enterprise policies to disable telemetry, studies, and crash reports.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableTelemetry".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableFirefoxStudies".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableTelemetry".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Mozilla\\Firefox".to_string(),
                    key: "DisableFirefoxStudies".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
    ]
}
