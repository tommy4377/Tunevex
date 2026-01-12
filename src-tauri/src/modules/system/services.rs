use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "system_disable_windows_search".to_string(),
            category: TweakCategory::System,
            name: "🔍 Disable Windows Search Indexer".to_string(),
            description: "Disables file indexing. Reduces high disk usage and CPU load, but slows down file search results.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    Set-Service -Name "WSearch" -StartupType Automatic -EA 0
                    Start-Service -Name "WSearch" -EA 0
                    Write-Host "Windows Search enabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Stop-Service -Name "WSearch" -Force -EA 0
                    Set-Service -Name "WSearch" -StartupType Disabled -EA 0
                    Write-Host "Windows Search disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
        Tweak {
            id: "system_set_bits_manual".to_string(),
            category: TweakCategory::System,
            name: "📉 Set BITS to Manual".to_string(),
            description: "Sets Background Intelligent Transfer Service to Manual. Stops it from running constantly in the background.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    Set-Service -Name "BITS" -StartupType Automatic -EA 0
                    Write-Host "BITS set to Automatic" -ForegroundColor Green
                "#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Set-Service -Name "BITS" -StartupType Manual -EA 0
                    Write-Host "BITS set to Manual" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
        Tweak {
            id: "system_disable_superfetch".to_string(),
            category: TweakCategory::System,
            name: "⚡ Disable SysMain (Superfetch)".to_string(),
            description: "Disables SysMain/Superfetch prefetching. Recommended for SSDs to reduce write cycles.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
                    Set-Service -Name "SysMain" -StartupType Automatic -EA 0
                    Start-Service -Name "SysMain" -EA 0
                    Write-Host "SysMain (Superfetch) enabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Stop-Service -Name "SysMain" -Force -EA 0
                    Set-Service -Name "SysMain" -StartupType Disabled -EA 0
                    Write-Host "SysMain (Superfetch) disabled" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
    ]
}
