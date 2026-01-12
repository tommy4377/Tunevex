use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

pub fn get_maintenance_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "system_clean_temp".to_string(),
            category: TweakCategory::System,
            name: "🧹 Clean Temporary Files".to_string(),
            description: "Cleans Windows temp, user temp, browser cache, Discord cache, and Windows logs.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: None, 
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Write-Host "Cleaning temporary files..." -ForegroundColor Cyan
                    ipconfig /flushdns | Out-Null
                    # Windows Temp
                    Remove-Item "$env:TEMP\*" -Recurse -Force -EA 0
                    Remove-Item "$env:LOCALAPPDATA\Temp\*" -Recurse -Force -EA 0
                    Remove-Item "C:\Windows\Temp\*" -Recurse -Force -EA 0
                    # Internet Cache
                    Remove-Item "$env:LOCALAPPDATA\Microsoft\Windows\INetCache\*" -Recurse -Force -EA 0
                    Remove-Item "$env:LOCALAPPDATA\Microsoft\Windows\INetCookies\*" -Recurse -Force -EA 0
                    # Discord
                    Remove-Item "$env:APPDATA\Discord\Cache\*" -Recurse -Force -EA 0
                    Remove-Item "$env:APPDATA\Discord\Code Cache\*" -Recurse -Force -EA 0
                    # Leaves
                    Remove-Item "C:\Windows\Logs\*" -Recurse -Force -EA 0
                    Write-Host "Temporary files cleaned!" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
        Tweak {
            id: "system_clean_prefetch".to_string(),
            category: TweakCategory::System,
            name: "📂 Clean Prefetch".to_string(),
            description: "Clears Windows Prefetch folder. May slow first app launches temporarily.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: None, 
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Remove-Item "C:\Windows\Prefetch\*" -Recurse -Force -EA 0
                    Write-Host "Prefetch cleaned" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
        Tweak {
            id: "system_clean_recycle".to_string(),
            category: TweakCategory::System,
            name: "🗑️ Empty Recycle Bin".to_string(),
            description: "Empties the Recycle Bin for all drives.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: None, 
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
                    Clear-RecycleBin -Force -EA 0
                    Write-Host "Recycle Bin emptied" -ForegroundColor Green
                "#.to_string(),
                }
            ]
        },
    ]
}
