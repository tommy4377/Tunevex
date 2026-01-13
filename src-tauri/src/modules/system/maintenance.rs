use crate::modules::types::{TweakType, RegistryValue, Tweak, TweakCheck, TweakCategory, TweakOperation, WarningLevel};

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
            tweak_type: TweakType::Action, enabled: false,
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
            name: "⚠️ Clean Prefetch Cache (NOT RECOMMENDED)".to_string(),
            description: "Clears Windows Prefetch cache. 

⛔ WARNING: THIS HURTS PERFORMANCE! ⛔

Prefetch improves application launch times by 15-30% by pre-loading 
frequently used data. Cleaning it provides NO benefit and forces Windows 
to rebuild the cache from scratch.

Only use if:
- Troubleshooting corrupted prefetch data
- SSD is nearly full (saves ~50MB)
- Testing fresh launch times

Modern SSDs are not harmed by Prefetch writes.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            revert_operations: None, 
            tweak_type: TweakType::Action, enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$confirmation = Read-Host "This will SLOW DOWN your system. Type 'YES' to confirm"
if ($confirmation -ne "YES") {
    Write-Host "Operation cancelled" -ForegroundColor Yellow
    exit 1
}

$prefetchPath = "$env:SystemRoot\Prefetch"
$count = (Get-ChildItem $prefetchPath -EA 0).Count

Remove-Item "$prefetchPath\*" -Force -EA 0

Write-Host "Removed $count prefetch files (~50MB)" -ForegroundColor Yellow
Write-Host "Windows will rebuild prefetch over the next few days" -ForegroundColor Yellow
Write-Host "Expect slower app launches until then" -ForegroundColor Red
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
            tweak_type: TweakType::Action, enabled: false,
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

        // Disable Fast Startup (Hybrid Shutdown)
        Tweak {
            id: "system_disable_fast_startup".to_string(),
            category: TweakCategory::System,
            name: "⚡ Disable Fast Startup".to_string(),
            description: "Disables Windows Fast Startup (hybrid shutdown). Performs full shutdown instead of hibernate-based shutdown.

Benefits:
- Fixes driver/hardware issues that persist across 'shutdowns'
- Prevents hibernation file from consuming disk space
- Ensures clean boot state
- Required for dual-boot systems

Note: Boot time may increase by 2-5 seconds.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power".to_string(),
                key: "HiberbootEnabled".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power".to_string(),
                    key: "HiberbootEnabled".to_string(),
                    value: crate::modules::types::RegistryValue::DWord(1),
                },
            ]),
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power".to_string(),
                    key: "HiberbootEnabled".to_string(),
                    value: crate::modules::types::RegistryValue::DWord(0),
                },
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Fast Startup disabled" -ForegroundColor Green
Write-Host "Windows will now perform full shutdowns" -ForegroundColor Cyan
"#.to_string(),
                }
            ],
        },
    ]
}
