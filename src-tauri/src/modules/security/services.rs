use crate::modules::types::{
    Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_service_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "sec_disable_remote_services".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Remote Access Services".to_string(),
            description: "Disables Remote Registry, Remote Access, WinRM. Improves security by reducing attack surface.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            revert_operations: Some(vec![
                TweakOperation::ServiceSetMode { name: "RemoteRegistry".to_string(), mode: "demand".to_string() },
                TweakOperation::ServiceSetMode { name: "RemoteAccess".to_string(), mode: "demand".to_string() },
                TweakOperation::ServiceSetMode { name: "WinRM".to_string(), mode: "demand".to_string() },
                TweakOperation::ServiceSetMode { name: "TermService".to_string(), mode: "demand".to_string() },
                TweakOperation::ServiceSetMode { name: "SessionEnv".to_string(), mode: "demand".to_string() },
            ]),
            tweak_type: TweakType::Toggle, enabled: false,
            check: Some(TweakCheck::MultiServiceDisabled {
                names: vec!["RemoteRegistry".to_string(), "RemoteAccess".to_string(), "WinRM".to_string(), "TermService".to_string(), "SessionEnv".to_string()],
            }),
            operations: vec![
                TweakOperation::ServiceDisable { name: "RemoteRegistry".to_string() },
                TweakOperation::ServiceDisable { name: "RemoteAccess".to_string() },
                TweakOperation::ServiceDisable { name: "WinRM".to_string() },
                TweakOperation::ServiceDisable { name: "TermService".to_string() },
                TweakOperation::ServiceDisable { name: "SessionEnv".to_string() },
            ]
        }
    ]
}
