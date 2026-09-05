use crate::modules::types::TweakCheck;
use std::os::windows::process::CommandExt;
use std::process::Command;

pub mod helpers;
pub static TWEAK_ENGINE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

// Re-export helpers for convenience
pub use helpers::checks::*;
pub use helpers::defender::*;
pub use helpers::dns::*;
pub use helpers::msi::*;
pub use helpers::nic::*;
pub use helpers::svchost::*;

// ─── TweakContext ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TweakContext {
    pub tweaks: Vec<crate::modules::types::Tweak>,
}

impl TweakContext {
    pub fn new(tweaks: Vec<crate::modules::types::Tweak>) -> Self {
        Self { tweaks }
    }
}

// ─── Windows Build helpers ────────────────────────────────────────────────────

pub fn get_current_windows_build() -> u32 {
    use winreg::{enums::*, RegKey};
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .and_then(|k| k.get_value::<String, _>("CurrentBuildNumber"))
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

pub fn win11_only_tweaks() -> std::collections::HashMap<&'static str, u32> {
    [
        ("interface_taskbar_end_task", 22621),
        ("taskbar_align_left", 22000),
        ("interface_remove_home_namespace", 22000),
        ("interface_disable_dynamic_lighting", 22621),
        ("privacy_disable_recall", 26100),
        ("privacy_disable_cross_device_resume", 26100),
        ("boot_highest_mode", 22000),
    ]
    .into_iter()
    .collect()
}

// ─── Main check dispatcher ────────────────────────────────────────────────────

pub fn check_tweak_enabled(check: &TweakCheck) -> bool {
    match check {
        TweakCheck::TcpGlobal { settings } => check_tcp_global(settings),
        TweakCheck::MsiEnabledForClass { class, priority } => check_msi_enabled_for_class(class, *priority),
        TweakCheck::Registry {
            root_key,
            path,
            key,
            expected_value,
        } => helpers::checks::check_registry_value(root_key, path, key, expected_value),
        TweakCheck::MultiScheduledTaskDisabled { names } => !names.is_empty() && names.iter().all(|name| check_scheduled_task_disabled(name)),
        TweakCheck::Powershell {
            script,
            expected_output,
        } => helpers::checks::check_powershell_output(script, expected_output),
        TweakCheck::NetAdapterProperty {
            property,
            expected_value,
        } => helpers::nic::check_nic_property(property, expected_value),
        TweakCheck::DnsServersContain { ip } => helpers::dns::check_dns_servers_contain(ip),
        TweakCheck::MultiRegistry { checks } => checks.iter().all(|c| {
            helpers::checks::check_registry_value(&c.root_key, &c.path, &c.key, &c.expected_value)
        }),
        TweakCheck::MpComputerStatus { check } => {
            helpers::defender::check_mp_computer_status(check)
        }
        TweakCheck::DefenderExclusionPath { paths } => {
            helpers::defender::check_defender_exclusion_paths(paths)
        }
        TweakCheck::CommandOutputContains {
            cmd,
            args,
            contains,
        } => helpers::checks::check_command_output_contains(cmd, args, contains),
        TweakCheck::RegistryKeyAbsent { root_key, path } => {
            helpers::checks::check_registry_key_absent(root_key, path)
        }
        TweakCheck::ScheduledTaskDisabled { name } => check_scheduled_task_disabled(name),
        TweakCheck::ServiceDisabled { name } => {
            let output = Command::new("sc")
                .args(["qc", name])
                .creation_flags(0x08000000)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: std::os::windows::process::ExitStatusExt::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            let out_str = String::from_utf8_lossy(&output.stdout).to_string();
            out_str.contains("DISABLED") || out_str.contains("4  DISABLED")
        }
        TweakCheck::ServiceMode { name, mode } => {
            let output = Command::new("sc")
                .args(["qc", name])
                .creation_flags(0x08000000)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: std::os::windows::process::ExitStatusExt::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            let out_str = String::from_utf8_lossy(&output.stdout).to_string();
            let mode_upper = mode.to_uppercase();
            out_str.contains(&mode_upper)
                || out_str.contains(&format!("START_TYPE         : {}", mode_upper))
        }
        TweakCheck::MultiServiceDisabled { names } => names.iter().all(|name| {
            let output = Command::new("sc")
                .args(["qc", name])
                .creation_flags(0x08000000)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: std::os::windows::process::ExitStatusExt::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            let out_str = String::from_utf8_lossy(&output.stdout).to_string();
            out_str.contains("DISABLED") || out_str.contains("4  DISABLED")
        }),
        TweakCheck::MsiEnabledGlobally { priority } => {
            helpers::msi::check_msi_enabled_globally(*priority)
        }
        TweakCheck::MsiEnabledOnNet { priority } => {
            helpers::msi::check_msi_enabled_on_net(*priority)
        }
        TweakCheck::NetworkInterfacesCheck {
            key,
            expected_value,
        } => crate::modules::registry::operations::check_network_interfaces(key, expected_value)
            .unwrap_or(false),
    }
}

/// Unknown query results must not be exported as an instruction to revert.
pub fn check_tweak_state(check: &TweakCheck) -> Option<bool> {
    match check {
        TweakCheck::Registry { root_key, path, key, expected_value } => query_registry_value(root_key, path, key, expected_value),
        TweakCheck::MultiRegistry { checks } => {
            if checks.is_empty() { return None; }
            let values: Option<Vec<bool>> = checks.iter().map(|c| query_registry_value(&c.root_key, &c.path, &c.key, &c.expected_value)).collect();
            values.map(|v| v.into_iter().all(|value| value))
        }
        TweakCheck::CommandOutputContains { cmd, args, contains } => {
            if crate::modules::utils::security::validate_command(cmd).is_err() { return None; }
            let out = Command::new(cmd).args(args).creation_flags(0x08000000).output().ok()?;
            if !out.status.success() || out.stdout.is_empty() { return None; }
            Some(String::from_utf8_lossy(&out.stdout).to_lowercase().contains(&contains.to_lowercase()))
        }
        TweakCheck::NetAdapterProperty { property, expected_value } => query_nic_property(property, expected_value),
        TweakCheck::MsiEnabledForClass { class, priority } => query_msi_state(class, *priority),
        TweakCheck::MsiEnabledOnNet { priority } => query_msi_state("Net", *priority),
        TweakCheck::MsiEnabledGlobally { priority } => {
            let states: Vec<bool> = ["Display", "SCSIAdapter", "Net", "USB", "HDC"].iter()
                .filter_map(|c| query_msi_state(c, *priority)).collect();
            if states.is_empty() { None } else { Some(states.into_iter().all(|v| v)) }
        }
        TweakCheck::Powershell { script, expected_output } => query_powershell(script)
            .filter(|out| !out.is_empty())
            .map(|out| out.eq_ignore_ascii_case(expected_output.trim())),
        TweakCheck::ScheduledTaskDisabled { name } => query_scheduled_task_disabled(name),
        TweakCheck::MultiScheduledTaskDisabled { names } => {
            if names.is_empty() { return None; }
            let states: Option<Vec<bool>> = names.iter().map(|name| query_scheduled_task_disabled(name)).collect();
            states.map(|s| s.into_iter().all(|v| v))
        }
        TweakCheck::TcpGlobal { settings } => {
            let out = Command::new("netsh").args(["interface", "tcp", "dump"])
                .creation_flags(0x08000000).output().ok()?;
            if !out.status.success() { return None; }
            let text = String::from_utf8_lossy(&out.stdout);
            if !settings.iter().all(|(key, _)| text.split_whitespace().any(|token| token.starts_with(&format!("{key}=")))) { return None; }
            Some(tcp_global_matches(&text, settings))
        }
        _ => Some(check_tweak_enabled(check)),
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    use crate::modules::types::{TweakOperation, TweakType};

    #[test]
    fn timestamps_use_netsh_and_heuristics_never_disable_rss() {
        let catalog = crate::build_tweak_catalog();
        let timestamps = catalog.iter().find(|t| t.id == "net_tcp_1323_opts").unwrap();
        assert!(matches!(&timestamps.check, Some(TweakCheck::TcpGlobal { settings })
            if settings == &vec![("timestamps".into(), "enabled".into())]));
        for (ops, expected) in [(&timestamps.operations, "timestamps=enabled"),
            (timestamps.revert_operations.as_ref().unwrap(), "timestamps=disabled")] {
            assert!(ops.iter().any(|op| matches!(op, TweakOperation::Command { cmd, args }
                if cmd == "netsh" && args.iter().any(|arg| arg == expected))));
        }
        let heuristics = catalog.iter().find(|t| t.id == "net_tcp_heuristics_disable").unwrap();
        assert!(heuristics.name.contains("Window Scaling"));
        let operations = serde_json::to_string(&heuristics.operations).unwrap();
        assert!(!operations.contains("rss="));
        assert!(!serde_json::to_string(&heuristics.revert_operations).unwrap().contains("rss="));
    }

    #[test]
    fn queryable_examples_are_detectable_and_reversible_where_supported() {
        let catalog = crate::build_tweak_catalog();
        for id in ["net_tcp_autotuning", "net_disable_jumbo_packet", "net_optimize_speed",
            "interface_classic_context_menu", "mem_fixed_pagefile", "sys_reserved_storage_disable"] {
            let tweak = catalog.iter().find(|t| t.id == id).unwrap();
            assert_eq!(tweak.tweak_type, TweakType::Toggle, "{id}");
            assert!(tweak.check.is_some() && tweak.revert_operations.is_some(), "{id}");
        }
        let display = catalog.iter().find(|t| t.id == "display_max_refresh_rate").unwrap();
        assert!(display.check.is_some());
        assert_eq!(display.tweak_type, TweakType::Action);
    }

    #[test]
    fn touched_powershell_scripts_parse_without_executing_tweaks() {
        let catalog = crate::build_tweak_catalog();
        for id in ["net_tcp_autotuning", "mem_fixed_pagefile", "display_max_refresh_rate",
            "net_disable_teredo", "net_tcp_heuristics_disable", "sys_reserved_storage_disable"] {
            let tweak = catalog.iter().find(|t| t.id == id).unwrap();
            let mut scripts = Vec::new();
            if let Some(TweakCheck::Powershell { script, .. }) = &tweak.check { scripts.push(script); }
            for op in tweak.operations.iter().chain(tweak.revert_operations.iter().flatten()) {
                if let TweakOperation::Powershell { script } = op { scripts.push(script); }
            }
            for script in scripts {
                let parse = format!("$tokens = $null; $errors = $null; [System.Management.Automation.Language.Parser]::ParseInput('{}', [ref]$tokens, [ref]$errors) | Out-Null; $errors.Count -eq 0", script.replace('\'', "''"));
                assert!(check_powershell_output(&parse, "True"), "{id} contains invalid PowerShell");
            }
        }
    }
}
