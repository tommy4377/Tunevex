use crate::modules::types::{Tweak, TweakCategory, TweakOperation, WarningLevel};

/// Returns all NTFS filesystem tweaks
pub fn get_ntfs_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "storage_ntfs_lastaccess".to_string(), // Renamed from cpu_
            category: TweakCategory::FileSystem,       // Changed category to match logical module
            name: "Disable NTFS Last Access Time".to_string(),
            description:
                "Disables last access time updates on files for improved performance and privacy."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            enabled: false,
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec![
                    "behavior".to_string(),
                    "set".to_string(),
                    "disablelastaccess".to_string(),
                    "2".to_string(),
                ], // 2 = System Managed (Default)
            }]),
            check: None,
            operations: vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec![
                    "behavior".to_string(),
                    "set".to_string(),
                    "disablelastaccess".to_string(),
                    "1".to_string(),
                ],
            }],
        },
        Tweak {
            id: "storage_ntfs_8dot3".to_string(), // Renamed from cpu_
            category: TweakCategory::FileSystem,
            name: "Disable NTFS 8.3 Name Creation".to_string(),
            description: "Disables legacy 8.3 short filename creation for improved performance."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            enabled: false,
            revert_operations: Some(vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["8dot3name".to_string(), "set".to_string(), "2".to_string()], // 2 = Volume Default
            }]),
            check: None,
            operations: vec![TweakOperation::Command {
                cmd: "fsutil".to_string(),
                args: vec!["8dot3name".to_string(), "set".to_string(), "1".to_string()],
            }],
        },
    ]
}
