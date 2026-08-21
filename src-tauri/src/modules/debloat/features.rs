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
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-Foundation-InternetPrinting-Client".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:LPDPrintService".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-Foundation-LPRPortMonitor".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-PrintToPDFServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-XPSServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Xps-Foundation-Xps-Viewer".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:WorkFolders-Client".to_string(), "/NoRestart".to_string()] },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "dism".to_string(),
                args: vec!["/Online".to_string(), "/Get-FeatureInfo".to_string(), "/FeatureName:Printing-Foundation-InternetPrinting-Client".to_string()],
                contains: "Disabled".to_string()
            }),
            operations: vec![
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-Foundation-InternetPrinting-Client".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:LPDPrintService".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-Foundation-LPRPortMonitor".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-PrintToPDFServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-XPSServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Xps-Foundation-Xps-Viewer".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:WorkFolders-Client".to_string(), "/NoRestart".to_string()] },
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
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Internet-Explorer-Optional-amd64".to_string(), "/NoRestart".to_string()] }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "dism".to_string(),
                args: vec!["/Online".to_string(), "/Get-FeatureInfo".to_string(), "/FeatureName:Internet-Explorer-Optional-amd64".to_string()],
                contains: "Disabled".to_string()
            }),
            operations: vec![
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Internet-Explorer-Optional-amd64".to_string(), "/NoRestart".to_string()] }
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
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:WindowsMediaPlayer".to_string(), "/NoRestart".to_string()] }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "dism".to_string(),
                args: vec!["/Online".to_string(), "/Get-FeatureInfo".to_string(), "/FeatureName:WindowsMediaPlayer".to_string()],
                contains: "Disabled".to_string()
            }),
            operations: vec![
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:WindowsMediaPlayer".to_string(), "/NoRestart".to_string()] }
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
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Microsoft-Windows-WordPad".to_string(), "/NoRestart".to_string()] }
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::CommandOutputContains {
                cmd: "dism".to_string(),
                args: vec!["/Online".to_string(), "/Get-FeatureInfo".to_string(), "/FeatureName:Microsoft-Windows-WordPad".to_string()],
                contains: "Disabled".to_string()
            }),
            operations: vec![
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Microsoft-Windows-WordPad".to_string(), "/NoRestart".to_string()] }
            ]
        },
    ]
}
