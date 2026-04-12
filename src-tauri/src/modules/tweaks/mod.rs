use crate::modules::types::TweakCheck;
use std::os::windows::process::CommandExt;
use std::process::Command;

pub mod helpers;

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
        TweakCheck::Registry {
            root_key,
            path,
            key,
            expected_value,
        } => helpers::checks::check_registry_value(root_key, path, key, expected_value),
        TweakCheck::MultiScheduledTaskDisabled { names } => names.iter().all(|name| {
            let output = Command::new("schtasks")
                .args(["/Query", "/TN", name, "/V", "/FO", "LIST"])
                .creation_flags(0x08000000)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: std::os::windows::process::ExitStatusExt::from_raw(1),
                    stdout: vec![],
                    stderr: vec![],
                });
            let s = String::from_utf8_lossy(&output.stdout).to_string();
            s.contains("Disabled") || s.contains("Disabilitato")
        }),
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
        TweakCheck::ScheduledTaskDisabled { name } => {
            let output = Command::new("schtasks")
                .args(["/query", "/tn", name, "/v", "/fo", "list"])
                .creation_flags(0x08000000)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: std::os::windows::process::ExitStatusExt::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            let out_str = String::from_utf8_lossy(&output.stdout).to_string();
            out_str.contains("Disabled") || out_str.contains("Disabilitato")
        }
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
