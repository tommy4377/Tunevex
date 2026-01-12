use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

/// Privacy Settings (activity, tracking, sync, permissions)
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // Activity History
        Tweak {
            id: "priv_disable_activity_history".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Activity History".to_string(),
            description: "Prevents Windows from collecting activity history and sending to Microsoft.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "EnableActivityFeed".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "PublishUserActivities".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "UploadUserActivities".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "EnableActivityFeed".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "PublishUserActivities".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\System".to_string(),
                    key: "UploadUserActivities".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // App Launch Tracking & Instrumentation
        Tweak {
            id: "priv_disable_app_tracking".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable App Launch Tracking".to_string(),
            description: "Stops Windows from tracking app launches and using instrumentation data.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "Start_TrackProgs".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer".to_string(),
                    key: "NoInstrumentation".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "Start_TrackProgs".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer".to_string(),
                    key: "NoInstrumentation".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Location Services
        Tweak {
            id: "priv_disable_location".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Location Tracking".to_string(),
            description: "Disables location services and sensor permissions.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableLocation".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableWindowsLocationProvider".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableLocation".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors".to_string(),
                    key: "DisableWindowsLocationProvider".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Disable Cloud Sync
        Tweak {
            id: "priv_disable_cloud_sync".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Settings Sync".to_string(),
            description: "Prevents Windows from syncing your settings to Microsoft cloud.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSync".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSyncUserOverride".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSync".to_string(),
                    value: RegistryValue::DWord(2),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\SettingSync".to_string(),
                    key: "DisableSettingSyncUserOverride".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Online Speech Recognition
        Tweak {
            id: "priv_disable_speech".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Online Speech Recognition".to_string(),
            description: "Prevents sending voice data to Microsoft for cloud speech recognition.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Speech_OneCore\\Settings\\OnlineSpeechPrivacy".to_string(),
                    key: "HasAccepted".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Speech_OneCore\\Settings\\OnlineSpeechPrivacy".to_string(),
                    key: "HasAccepted".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },

        // App Permissions
        Tweak {
            id: "priv_app_permissions".to_string(),
            category: TweakCategory::Privacy,
            name: "Restrict App Permissions".to_string(),
            description: "Denies app access to diagnostics, location, and user account information globally.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\appDiagnostics".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Allow".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\location".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Allow".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\userAccountInformation".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Allow".to_string()),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\appDiagnostics".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Deny".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\location".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Deny".to_string()),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\userAccountInformation".to_string(),
                    key: "Value".to_string(),
                    value: RegistryValue::String("Deny".to_string()),
                },
            ]
        },
        
        // Disable Website Language Access
        Tweak {
            id: "priv_disable_lang_list".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Website Language Access".to_string(),
            description: "Prevents websites from accessing your language list to reduce fingerprinting.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\International\\User Profile".to_string(),
                    key: "HttpAcceptLanguageOptOut".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "Control Panel\\International\\User Profile".to_string(),
                    key: "HttpAcceptLanguageOptOut".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // Disable Lockscreen Camera
        Tweak {
            id: "priv_lockscreen_camera".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Lockscreen Camera".to_string(),
            description: "Disables camera access on the lockscreen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization".to_string(),
                    key: "NoLockScreenCamera".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization".to_string(),
                    key: "NoLockScreenCamera".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },

        // Configure Windows Media Player
        Tweak {
            id: "priv_config_wmp".to_string(),
            category: TweakCategory::Privacy,
            name: "Configure WMP Privacy".to_string(),
            description: "Disables Windows Media Player DRM internet access, usage tracking, and first-run wizard.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\WMDRM".to_string(),
                    key: "DisableOnline".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "AcceptedPrivacyStatement".to_string(),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "UsageTracking".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\WMDRM".to_string(),
                    key: "DisableOnline".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "AcceptedPrivacyStatement".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\MediaPlayer\\Preferences".to_string(),
                    key: "UsageTracking".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Disable Performance Track
        Tweak {
            id: "priv_disable_perftrack".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Performance Tracking".to_string(),
            description: "Disables tracking of responsiveness events for privacy.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WDI\\{9c5a40da-b965-4fc3-8781-88dd50a6299d}".to_string(),
                    key: "ScenarioExecutionEnabled".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\WDI\\{9c5a40da-b965-4fc3-8781-88dd50a6299d}".to_string(),
                    key: "ScenarioExecutionEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Disable Bing Web Search
        Tweak {
            id: "priv_disable_bing_search".to_string(),
            category: TweakCategory::Privacy,
            name: "🔍 Disable Bing Web Search".to_string(),
            description: "Disables Bing web search and Cortana in Windows Search.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Search"
Set-ItemProperty -Path $path -Name "BingSearchEnabled" -Value 1 -Type DWord -Force -EA 0
Set-ItemProperty -Path $path -Name "CortanaConsent" -Value 1 -Type DWord -Force -EA 0
$path2 = "HKCU:\SOFTWARE\Policies\Microsoft\Windows\Explorer"
Remove-ItemProperty -Path $path2 -Name "DisableSearchBoxSuggestions" -EA 0
$path3 = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search"
Remove-ItemProperty -Path $path3 -Name "DisableWebSearch" -EA 0
Remove-ItemProperty -Path $path3 -Name "ConnectedSearchUseWeb" -EA 0
Write-Host "Bing search enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
$path = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Search"
Set-ItemProperty -Path $path -Name "BingSearchEnabled" -Value 0 -Type DWord -Force -EA 0
Set-ItemProperty -Path $path -Name "CortanaConsent" -Value 0 -Type DWord -Force -EA 0
$path2 = "HKCU:\SOFTWARE\Policies\Microsoft\Windows\Explorer"
if (!(Test-Path $path2)) { New-Item -Path $path2 -Force | Out-Null }
Set-ItemProperty -Path $path2 -Name "DisableSearchBoxSuggestions" -Value 1 -Type DWord -Force
$path3 = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search"
if (!(Test-Path $path3)) { New-Item -Path $path3 -Force | Out-Null }
Set-ItemProperty -Path $path3 -Name "DisableWebSearch" -Value 1 -Type DWord -Force
Set-ItemProperty -Path $path3 -Name "ConnectedSearchUseWeb" -Value 0 -Type DWord -Force
Write-Host "Bing search disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        // Clear MRU History
        Tweak {
            id: "priv_clear_mru".to_string(),
            category: TweakCategory::Privacy,
            name: "🧹 Clear Recent Files & MRU History".to_string(),
            description: "Clears all Most Recently Used (MRU) lists for privacy.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: None,
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Clearing MRU and recent files..." -ForegroundColor Cyan
Remove-Item "$env:APPDATA\Microsoft\Windows\Recent\AutomaticDestinations\*" -Force -EA 0
Remove-Item "$env:APPDATA\Microsoft\Windows\Recent\CustomDestinations\*" -Force -EA 0
Remove-Item "$env:APPDATA\Microsoft\Windows\Recent\*" -Force -EA 0
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Applets\Regedit" /va /f 2>$null
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\ComDlg32\LastVisitedPidlMRU" /va /f 2>$null
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\ComDlg32\OpenSaveMRU" /va /f 2>$null
reg delete "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\RecentDocs" /va /f 2>$null
Write-Host "MRU and recent files cleared" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
