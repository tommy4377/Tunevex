//! Microsoft Edge debloat tweaks

use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "debloat_edge_sidebar".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🔧 Disable Edge Sidebar & Discover".to_string(),
            description: "Disables Edge sidebar, Discover button, and related bloat features.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "HubsSidebarEnabled" -EA 0
Remove-ItemProperty -Path $path -Name "WebWidgetAllowed" -EA 0
Remove-ItemProperty -Path $path -Name "EdgeShoppingAssistantEnabled" -EA 0
Write-Host "Edge sidebar features enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "HubsSidebarEnabled" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "WebWidgetAllowed" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "EdgeShoppingAssistantEnabled" -Value 0 -Type DWord -Force
Write-Host "Edge sidebar features disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_edge_startup".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🔧 Disable Edge First Run & Welcome".to_string(),
            description: "Disables Edge first run experience and welcome page.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "HideFirstRunExperience" -EA 0
Remove-ItemProperty -Path $path -Name "RunStartUpSystemCheck" -EA 0
Write-Host "Edge first run enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "HideFirstRunExperience" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path -Name "RunStartUpSystemCheck" -Value 0 -Type DWord -Force
Write-Host "Edge first run disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_edge_sync".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🔧 Disable Edge Sync & Cloud Features".to_string(),
            description: "Disables Edge sync, collections, and cloud features.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "SyncDisabled" -EA 0
Remove-ItemProperty -Path $path -Name "EdgeCollectionsEnabled" -EA 0
Write-Host "Edge sync enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "SyncDisabled" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path -Name "EdgeCollectionsEnabled" -Value 0 -Type DWord -Force
Write-Host "Edge sync disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_edge_telemetry".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🔧 Disable Edge Telemetry".to_string(),
            description: "Disables Edge telemetry, usage data, and personalization.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
Remove-ItemProperty -Path $path -Name "PersonalizationReportingEnabled" -EA 0
Remove-ItemProperty -Path $path -Name "UserFeedbackAllowed" -EA 0
Remove-ItemProperty -Path $path -Name "MetricsReportingEnabled" -EA 0
Write-Host "Edge telemetry enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\Edge"
if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
Set-ItemProperty -Path $path -Name "PersonalizationReportingEnabled" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "UserFeedbackAllowed" -Value 0 -Type DWord -Force
Set-ItemProperty -Path $path -Name "MetricsReportingEnabled" -Value 0 -Type DWord -Force
Write-Host "Edge telemetry disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_edge_autostart".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "🔧 Disable Edge Auto-Start".to_string(),
            description: "Prevents Edge from starting automatically at login.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKLM:\SOFTWARE\Policies\Microsoft\MicrosoftEdge\Main"
Remove-ItemProperty -Path $path -Name "PreventFirstRunPage" -EA 0
reg delete "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run" /v "MicrosoftEdgeAutoLaunch*" /f 2>$null
Write-Host "Edge auto-start settings restored" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
# Disable Edge auto-start via startup approved
$path = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run"
Get-ItemProperty -Path $path -EA 0 | Get-Member -MemberType NoteProperty | 
    Where-Object { $_.Name -like "*Edge*" } | 
    ForEach-Object { Remove-ItemProperty -Path $path -Name $_.Name -EA 0 }

# Remove Edge from Run key
$runPath = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Run"
Get-ItemProperty -Path $runPath -EA 0 | Get-Member -MemberType NoteProperty | 
    Where-Object { $_.Name -like "*Edge*" } | 
    ForEach-Object { Remove-ItemProperty -Path $runPath -Name $_.Name -EA 0 }

Write-Host "Edge auto-start disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
