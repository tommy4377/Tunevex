use crate::modules::types::{Tweak, TweakCategory, TweakOperation, TweakType, WarningLevel};

pub fn get_maintenance_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "net_soft_reset".to_string(),
            category: TweakCategory::Network,
            name: "Run Soft Network Reset".to_string(),
            description: "Executes: Winsock reset, IP reset, Flush DNS. Use this to fix connectivity issues. (One-shot action)".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            revert_operations: None, tweak_type: TweakType::Action, enabled: false,
            check: None,
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["winsock".into(), "reset".into()],
                },
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec!["int".into(), "ip".into(), "reset".into()],
                },
                TweakOperation::Command {
                    cmd: "ipconfig".to_string(),
                    args: vec!["/flushdns".into()],
                },
            ]
        }
    ]
}
