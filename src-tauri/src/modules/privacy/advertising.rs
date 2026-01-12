use crate::modules::types::{RegistryValue, Tweak, TweakCategory, TweakOperation, WarningLevel};

/// Advertising & Tracking
pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        // Advertising ID
        Tweak {
            id: "priv_disable_advertising_id".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Advertising ID".to_string(),
            description: "Prevents apps from using advertising ID for personalized ads.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo".to_string(),
                    key: "DisabledByGroupPolicy".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo".to_string(),
                    key: "Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo".to_string(),
                    key: "DisabledByGroupPolicy".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Disable Sync Provider Notifications (File Explorer Ads)
        Tweak {
            id: "priv_disable_sync_notifs".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Sync Provider Notifications".to_string(),
            description: "Disables notifications/ads from OneDrive and other sync providers in File Explorer.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "ShowSyncProviderNotifications".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced".to_string(),
                    key: "ShowSyncProviderNotifications".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // Tailored Experiences
        Tweak {
            id: "priv_disable_tailored".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Tailored Experiences".to_string(),
            description: "Stops Microsoft from using diagnostic data for personalized tips and ads.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Privacy".to_string(),
                    key: "TailoredExperiencesWithDiagnosticDataEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\CloudContent".to_string(),
                    key: "DisableTailoredExperiencesWithDiagnosticData".to_string(),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Privacy".to_string(),
                    key: "TailoredExperiencesWithDiagnosticDataEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows\\CloudContent".to_string(),
                    key: "DisableTailoredExperiencesWithDiagnosticData".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]
        },
        
        // Suggested Content
        Tweak {
            id: "priv_disable_suggestions".to_string(),
            category: TweakCategory::Privacy,
            name: "Disable Suggested Content".to_string(),
            description: "Removes Microsoft suggestions, tips, and spotlight from Start and lock screen.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338388Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338389Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-310093Enabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SoftLandingEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "RotatingLockScreenEnabled".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338388Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-338389Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SubscribedContent-310093Enabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "SoftLandingEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKCU".to_string(),
                    path: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager".to_string(),
                    key: "RotatingLockScreenEnabled".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ]
        },
        
        // 🔒 Comprehensive Privacy All-in-One
        Tweak {
            id: "priv_all_in_one".to_string(),
            category: TweakCategory::Privacy,
            name: "🔐 Privacy Hardening All-in-One".to_string(),
            description: "Comprehensive privacy settings: disables feedback, handwriting reports, Copilot, Recall, web content, and more.".to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Reverting Privacy Hardening..." -ForegroundColor Yellow

# Feedback frequency (delete to reset)
Remove-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Siuf\Rules" -Name "NumberOfSIUFInPeriod" -Force -EA 0

# Handwriting reports
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports" -Name "PreventHandwritingErrorReports" -Force -EA 0

# Recall
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" -Name "DisableAIDataAnalysis" -Force -EA 0

# Copilot
Remove-ItemProperty -Path "HKCU:\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot" -Name "TurnOffWindowsCopilot" -Force -EA 0

# SmartScreen (Enable = 1)
Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\System" -Name "EnableSmartScreen" -Value 1 -Type DWord -Force -EA 0

# Web search in Start
Remove-ItemProperty -Path "HKCU:\SOFTWARE\Policies\Microsoft\Windows\Explorer" -Name "DisableSearchBoxSuggestions" -Force -EA 0

# Enable content suggestions
$cdmPath = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"
$subs = @("338393","353694","353696","338388","338389","310093","314563")
foreach ($s in $subs) {
    Set-ItemProperty -Path $cdmPath -Name "SubscribedContent-${s}Enabled" -Value 1 -Type DWord -Force -EA 0
}
Set-ItemProperty -Path $cdmPath -Name "SystemPaneSuggestionsEnabled" -Value 1 -Type DWord -Force -EA 0

Write-Host "Privacy hardening reverted" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "=== Comprehensive Privacy Hardening ===" -ForegroundColor Cyan

# Feedback frequency
Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Siuf\Rules" -Name "NumberOfSIUFInPeriod" -Value 0 -Type DWord -Force -EA 0

# Handwriting reports
Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports" -Name "PreventHandwritingErrorReports" -Value 1 -Type DWord -Force -EA 0

# Recall / AI Features
$recallPath = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI"
if (!(Test-Path $recallPath)) { New-Item -Path $recallPath -Force | Out-Null }
Set-ItemProperty -Path $recallPath -Name "DisableAIDataAnalysis" -Value 1 -Type DWord -Force

# Copilot
$copilotPath = "HKCU:\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot"
if (!(Test-Path $copilotPath)) { New-Item -Path $copilotPath -Force | Out-Null }
Set-ItemProperty -Path $copilotPath -Name "TurnOffWindowsCopilot" -Value 1 -Type DWord -Force

# SmartScreen
Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\System" -Name "EnableSmartScreen" -Value 0 -Type DWord -Force -EA 0

# Web search in Start
Set-ItemProperty -Path "HKCU:\SOFTWARE\Policies\Microsoft\Windows\Explorer" -Name "DisableSearchBoxSuggestions" -Value 1 -Type DWord -Force -EA 0

# Disable content suggestions
$cdmPath = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"
$subs = @("338393","353694","353696","338388","338389","310093","314563")
foreach ($s in $subs) {
    Set-ItemProperty -Path $cdmPath -Name "SubscribedContent-${s}Enabled" -Value 0 -Type DWord -Force -EA 0
}
Set-ItemProperty -Path $cdmPath -Name "SystemPaneSuggestionsEnabled" -Value 0 -Type DWord -Force -EA 0

Write-Host "Privacy hardening complete!" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
