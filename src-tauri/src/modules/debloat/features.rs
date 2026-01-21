//! Windows Optional Features debloat tweaks

use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "debloat_disable_printer_features".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Printer & XPS Features".to_string(),
            description: "Disables Internet Printing, LPD, LPR, Print to PDF, XPS Services/Viewer, and Work Folders.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Enabling Printer and XPS features..." -ForegroundColor Cyan
$features = @(
    "Printing-Foundation-InternetPrinting-Client",
    "LPDPrintService",
    "Printing-Foundation-LPRPortMonitor",
    "Printing-PrintToPDFServices-Features",
    "Printing-XPSServices-Features",
    "Xps-Foundation-Xps-Viewer",
    "WorkFolders-Client"
)
foreach ($f in $features) {
    dism /Online /Enable-Feature /FeatureName:"$f" /NoRestart 2>$null
}
Write-Host "Printer and XPS features enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$f = Get-WindowsOptionalFeature -Online -FeatureName "Printing-Foundation-InternetPrinting-Client" -ErrorAction SilentlyContinue
if ($f.State -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
Write-Host "Disabling Printer and XPS features..." -ForegroundColor Cyan
$features = @(
    "Printing-Foundation-InternetPrinting-Client",
    "LPDPrintService",
    "Printing-Foundation-LPRPortMonitor",
    "Printing-PrintToPDFServices-Features",
    "Printing-XPSServices-Features",
    "Xps-Foundation-Xps-Viewer",
    "WorkFolders-Client"
)
foreach ($f in $features) {
    dism /Online /Disable-Feature /FeatureName:"$f" /NoRestart 2>$null
}
Write-Host "Printer and XPS features disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_disable_ie_features".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Internet Explorer".to_string(),
            description: "Disables Internet Explorer mode (legacy feature).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Enable-Feature /FeatureName:"Internet-Explorer-Optional-amd64" /NoRestart 2>$null
Write-Host "Internet Explorer enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$f = Get-WindowsOptionalFeature -Online -FeatureName "Internet-Explorer-Optional-amd64" -ErrorAction SilentlyContinue
if ($f.State -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Disable-Feature /FeatureName:"Internet-Explorer-Optional-amd64" /NoRestart 2>$null
Write-Host "Internet Explorer disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_disable_mediaplayer".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable Windows Media Player".to_string(),
            description: "Disables Windows Media Player legacy feature.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Enable-Feature /FeatureName:"WindowsMediaPlayer" /NoRestart 2>$null
Write-Host "Windows Media Player enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$f = Get-WindowsOptionalFeature -Online -FeatureName "WindowsMediaPlayer" -ErrorAction SilentlyContinue
if ($f.State -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Disable-Feature /FeatureName:"WindowsMediaPlayer" /NoRestart 2>$null
Write-Host "Windows Media Player disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
        
        Tweak {
            id: "debloat_disable_wordpad".to_string(),
            category: TweakCategory::DebloatTelemetry,
            name: "Disable WordPad".to_string(),
            description: "Disables WordPad legacy feature.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Enable-Feature /FeatureName:"Microsoft-Windows-WordPad" /NoRestart 2>$null
Write-Host "WordPad enabled" -ForegroundColor Green
"#.to_string(),
                }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::Powershell {
                script: r#"
$f = Get-WindowsOptionalFeature -Online -FeatureName "Microsoft-Windows-WordPad" -ErrorAction SilentlyContinue
if ($f.State -eq 'Disabled') { "True" } else { "False" }
"#.to_string(),
                expected_output: "True".to_string(),
            }),
            operations: vec![
                TweakOperation::Powershell {
                    script: r#"
dism /Online /Disable-Feature /FeatureName:"Microsoft-Windows-WordPad" /NoRestart 2>$null
Write-Host "WordPad disabled" -ForegroundColor Green
"#.to_string(),
                }
            ]
        },
    ]
}
