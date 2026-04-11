use crate::modules::registry::backup::RegistryBackup;
use crate::modules::registry::operations::apply_registry_tweak;
use crate::modules::types::{RegistryValue, Tweak, TweakCheck, TweakOperation};
use crate::modules::utils::privileges::is_admin;
use crate::modules::utils::state::AppState;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;
use strip_ansi_escapes::strip_str;
use tauri::{Emitter, State};

// ─── Allowed Commands Whitelist (Security) ───────────────────────────────
const ALLOWED_COMMANDS: &[&str] = &[
    "sc", "schtasks", "bcdedit", "powercfg",
    "fsutil", "powershell", "taskkill", "netsh",
    "reg", "cmd", "pnputil", "del", "rmdir", "dism"
];

fn validate_command(cmd: &str) -> Result<(), String> {
    let cmd_path = std::path::Path::new(cmd);
    let exe_name = cmd_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(cmd)
        .to_lowercase();
    if ALLOWED_COMMANDS.contains(&exe_name.as_str()) {
        Ok(())
    } else {
        Err(format!("Command '{}' is not in the allowed list", cmd))
    }
}

// ─── Allowed Write Roots (Security - Path Traversal Prevention) ───────────────
const ALLOWED_WRITE_ROOTS: &[&str] = &[
    r"C:\Users",
    r"C:\ProgramData",
    r"C:\Program Files",
    r"C:\Program Files (x86)",
];

fn expand_env_vars(raw: &str) -> String {
    raw.replace("%APPDATA%", &std::env::var("APPDATA").unwrap_or_default())
        .replace("%LOCALAPPDATA%", &std::env::var("LOCALAPPDATA").unwrap_or_default())
        .replace("%ProgramData%", &std::env::var("ProgramData").unwrap_or_default())
        .replace("%ProgramFiles%", &std::env::var("ProgramFiles").unwrap_or_default())
        .replace("%UserProfile%", &std::env::var("USERPROFILE").unwrap_or_default())
        .replace("%Home%", &std::env::var("USERPROFILE").unwrap_or_default())
}

fn is_under_allowed_root(p: &std::path::Path) -> bool {
    let s = p.to_string_lossy().to_lowercase();
    ALLOWED_WRITE_ROOTS.iter().any(|r| s.starts_with(&r.to_lowercase()))
}

// For SOURCE paths that MUST already exist (Delete, Copy/Move src)
fn safe_path_existing(raw: &str) -> Result<std::path::PathBuf, String> {
    let expanded = expand_env_vars(raw);
    let canonical = std::path::Path::new(&expanded)
        .canonicalize()
        .map_err(|e| format!("Path not found '{}': {}", raw, e))?;
    if !is_under_allowed_root(&canonical) {
        return Err(format!("Path '{}' is outside allowed directories.", raw));
    }
    Ok(canonical)
}

// For DESTINATION paths that may NOT exist yet (Write, Copy/Move dest)
fn safe_path_new(raw: &str) -> Result<std::path::PathBuf, String> {
    let expanded = expand_env_vars(raw);
    let p = std::path::PathBuf::from(&expanded);
    // Walk up until we find an existing ancestor, canonicalize only that
    let mut existing = p.clone();
    let mut suffix = std::path::PathBuf::new();
    loop {
        if existing.exists() {
            break;
        }
        match existing.parent() {
            Some(parent) => {
                if let Some(component) = existing.file_name() {
                    suffix = std::path::PathBuf::from(component).join(&suffix);
                }
                existing = parent.to_path_buf();
            }
            None => break,
        }
    }
    let base = if existing.exists() {
        existing
            .canonicalize()
            .map_err(|e| format!("Cannot resolve base path for '{}': {}", raw, e))?
    } else {
        existing
    };
    let full = base.join(suffix);
    if !is_under_allowed_root(&full) {
        return Err(format!("Destination '{}' is outside allowed directories.", raw));
    }
    Ok(full)
}

// ─── Tcpip interface registry path ─────────────────────────────────────────
const TCPIP_INTERFACES_PATH: &str =
    "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces";

// ─── NIC class GUID ────────────────────────────────────────────────────────
const NIC_CLASS_PATH: &str =
    "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}";

/// Enumerate every physical NIC subkey (those that have a "DriverDesc" value).
/// Returns a list of registry paths like
///   "SYSTEM\...\{4d36e972...}\0000"
fn nic_subkey_paths() -> Vec<String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = match hklm.open_subkey(NIC_CLASS_PATH) {
        Ok(k) => k,
        Err(_) => return Vec::new(),
    };
    let mut paths = Vec::new();
    for sub in class_key.enum_keys().flatten() {
        // Only 4-digit numeric subkeys are adapter instances
        if sub.len() != 4 || !sub.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let sub_path = format!("{}\\{}", NIC_CLASS_PATH, sub);
        // Verify it is a real adapter (has DriverDesc)
        if let Ok(k) = hklm.open_subkey(&sub_path) {
            if k.get_value::<String, _>("DriverDesc").is_ok() {
                paths.push(sub_path);
            }
        }
    }
    paths
}

/// Write `value` as REG_SZ into `property` for every physical NIC subkey.
/// Silently skips adapters that don't have the property at all.
fn set_nic_property(property: &str, value: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for path in nic_subkey_paths() {
        match hklm.open_subkey_with_flags(&path, KEY_SET_VALUE | KEY_QUERY_VALUE) {
            Ok(k) => {
                // Only set if the property already exists on this adapter
                // (avoids injecting foreign keys into adapters that don't support it)
                let already: Result<String, _> = k.get_value(property);
                if already.is_ok() {
                    if let Err(e) = k.set_value(property, &value.to_string()) {
                        eprintln!("[NicProp] Warning: set {} on {} failed: {}", property, path, e);
                    }
                }
            }
            Err(e) => eprintln!("[NicProp] Warning: open {} failed: {}", path, e),
        }
    }
    Ok(())
}

/// Check that every physical NIC subkey that *has* `property` reports the
/// expected value.  Returns true if no adapter disagrees.
fn check_nic_property(property: &str, expected: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut found_any = false;
    for path in nic_subkey_paths() {
        if let Ok(k) = hklm.open_subkey_with_flags(&path, KEY_READ) {
            if let Ok(val) = k.get_value::<String, _>(property) {
                found_any = true;
                if val.to_lowercase() != expected.to_lowercase() {
                    return false;
                }
            }
        }
    }
    // If no adapter has the property, treat as "not applied"
    found_any
}

// ─── DNS helpers (pure winreg, no PowerShell) ───────────────────────────────

/// Enumerate all {GUID} subkeys under Tcpip\Parameters\Interfaces.
fn tcpip_interface_subkeys() -> Vec<String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let parent = match hklm.open_subkey(TCPIP_INTERFACES_PATH) {
        Ok(k) => k,
        Err(_) => return Vec::new(),
    };
    parent
        .enum_keys()
        .flatten()
        .map(|guid| format!("{}\\{}", TCPIP_INTERFACES_PATH, guid))
        .collect()
}

/// Write static DNS server addresses to every Tcpip interface subkey.
/// Format: "primary,secondary"  (Windows comma-separated REG_SZ).
fn set_dns_servers(primary: &str, secondary: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let value = format!("{},{}", primary, secondary);
    for path in tcpip_interface_subkeys() {
        match hklm.open_subkey_with_flags(&path, KEY_SET_VALUE) {
            Ok(k) => {
                if let Err(e) = k.set_value("NameServer", &value) {
                    eprintln!("[DNS] Warning: set NameServer on {} failed: {}", path, e);
                }
            }
            Err(e) => eprintln!("[DNS] Warning: open {} failed: {}", path, e),
        }
    }
    Ok(())
}

/// Reset DNS to DHCP by clearing NameServer on every Tcpip interface subkey.
fn reset_dns_servers() -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let empty = String::new();
    for path in tcpip_interface_subkeys() {
        match hklm.open_subkey_with_flags(&path, KEY_SET_VALUE) {
            Ok(k) => {
                if let Err(e) = k.set_value("NameServer", &empty) {
                    eprintln!("[DNS] Warning: clear NameServer on {} failed: {}", path, e);
                }
            }
            Err(e) => eprintln!("[DNS] Warning: open {} failed: {}", path, e),
        }
    }
    Ok(())
}

// ─── Windows Defender native helpers (no PowerShell) ───────────────────────

/// Registry path for Tamper Protection feature flag.
const DEFENDER_FEATURES_PATH: &str = "SOFTWARE\\Microsoft\\Windows Defender\\Features";
/// Registry path for Defender policy (set by Group Policy / tweaks).
const DEFENDER_POLICY_PATH: &str = "SOFTWARE\\Policies\\Microsoft\\Windows Defender";
/// Registry path for Defender Exclusions\Paths.
const DEFENDER_EXCLUSIONS_PATH: &str =
    "SOFTWARE\\Microsoft\\Windows Defender\\Exclusions\\Paths";

/// Returns true when Tamper Protection is currently ACTIVE (value == 5).
/// Source: HKLM\SOFTWARE\Microsoft\Windows Defender\Features\TamperProtection
fn is_tamper_protection_on() -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey(DEFENDER_FEATURES_PATH) {
        // TamperProtection == 5 means fully enabled; 4 = disabled; 0/2 = off/intermediate
        if let Ok(val) = key.get_value::<u32, _>("TamperProtection") {
            return val == 5;
        }
    }
    false
}

/// Check native Defender status via pure registry reads.
/// variant "realtime_disabled" → RTP policy key is 1 AND tamper protection is OFF
/// variant "av_disabled"       → both AV policy keys are 1 AND tamper protection is OFF
fn check_mp_computer_status(check: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    // If Tamper Protection is on, any policy tweak is silently ignored by Windows —
    // from the user's perspective the tweak is NOT applied.
    if is_tamper_protection_on() {
        return false;
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    match check {
        "realtime_disabled" => {
            // Policy key: HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection
            let rtp_path = format!("{}\\Real-Time Protection", DEFENDER_POLICY_PATH);
            hklm.open_subkey(&rtp_path)
                .and_then(|k| k.get_value::<u32, _>("DisableRealtimeMonitoring"))
                .map(|v| v == 1)
                .unwrap_or(false)
        }
        "av_disabled" => {
            // Both DisableAntiSpyware and DisableAntiVirus must be 1
            let spyware_off = hklm
                .open_subkey(DEFENDER_POLICY_PATH)
                .and_then(|k| k.get_value::<u32, _>("DisableAntiSpyware"))
                .map(|v| v == 1)
                .unwrap_or(false);
            let antivirus_off = hklm
                .open_subkey(DEFENDER_POLICY_PATH)
                .and_then(|k| k.get_value::<u32, _>("DisableAntiVirus"))
                .map(|v| v == 1)
                .unwrap_or(false);
            spyware_off && antivirus_off
        }
        _ => false,
    }
}

/// Returns true when ALL the given paths appear as value names under
/// HKLM\SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths.
/// The Exclusions\Paths key stores each exclusion path as a REG_DWORD value name
/// (the value data is always 0).
fn check_defender_exclusion_paths(paths: &[String]) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = match hklm.open_subkey(DEFENDER_EXCLUSIONS_PATH) {
        Ok(k) => k,
        Err(_) => return false,
    };
    // Collect all existing value names (case-insensitive)
    let existing: Vec<String> = key
        .enum_values()
        .flatten()
        .map(|(name, _)| name.to_lowercase())
        .collect();
    paths
        .iter()
        .all(|p| existing.contains(&p.to_lowercase()))
}

/// Add or remove exclusion paths from
/// HKLM\SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths.
fn set_defender_exclusions(paths: &[String], action: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    match action {
        "add" => {
            // Open or create the Exclusions\Paths key
            let (key, _) = hklm
                .create_subkey(DEFENDER_EXCLUSIONS_PATH)
                .map_err(|e| format!("Failed to open/create Exclusions\\Paths key: {}", e))?;
            for path in paths {
                // Each path is stored as a value name with DWORD data = 0
                key.set_value(path, &0u32)
                    .map_err(|e| format!("Failed to add exclusion '{}': {}", path, e))?;
            }
        }
        "remove" => {
            if let Ok(key) =
                hklm.open_subkey_with_flags(DEFENDER_EXCLUSIONS_PATH, KEY_SET_VALUE)
            {
                for path in paths {
                    // Ignore "not found" errors — exclusion may already be gone
                    let _ = key.delete_value(path);
                }
            }
            // If the key doesn't exist there is nothing to remove — that is fine.
        }
        _ => return Err(format!("Unknown DefenderExclusion action: {}", action)),
    }
    Ok(())
}

/// Control Defender services (WinDefend, WdNisSvc, Sense) via sc.exe.
/// action "disable" → sc stop + sc config start= disabled
/// action "enable"  → sc config start= auto + sc start
fn control_defender_services(services: &[String], action: &str) -> Result<(), String> {
    for svc in services {
        match action {
            "disable" => {
                // Stop service first (ignore error — it may already be stopped)
                let _ = Command::new("sc")
                    .args(&["stop", svc])
                    .creation_flags(0x08000000)
                    .output();
                // Disable startup type
                let out = Command::new("sc")
                    .args(&["config", svc, "start=", "disabled"])
                    .creation_flags(0x08000000)
                    .output()
                    .map_err(|e| format!("sc config disable {} failed: {}", svc, e))?;
                if !out.status.success() {
                    eprintln!(
                        "[Defender] Warning: sc config start= disabled for {} returned non-zero",
                        svc
                    );
                }
            }
            "enable" => {
                // Set to auto-start
                let out = Command::new("sc")
                    .args(&["config", svc, "start=", "auto"])
                    .creation_flags(0x08000000)
                    .output()
                    .map_err(|e| format!("sc config enable {} failed: {}", svc, e))?;
                if !out.status.success() {
                    eprintln!(
                        "[Defender] Warning: sc config start= auto for {} returned non-zero",
                        svc
                    );
                }
                // Start service (ignore error — may need reboot or tamper protection blocks it)
                let _ = Command::new("sc")
                    .args(&["start", svc])
                    .creation_flags(0x08000000)
                    .output();
            }
            _ => return Err(format!("Unknown DefenderServiceControl action: {}", action)),
        }
    }
    Ok(())
}

/// Run a binary and return true when its stdout+stderr output (case-insensitive)
/// contains the given substring.  Used for bcdedit /enum and powercfg /q checks
/// without spawning PowerShell.
fn check_command_output_contains(cmd: &str, args: &[String], contains: &str) -> bool {
    // BUG-H2 fix: apply the same whitelist used in apply_tweak/undo_tweak
    if validate_command(cmd).is_err() {
        eprintln!("[check] Blocked non-whitelisted command: '{}'", cmd);
        return false; // fail-safe: treat as "not applied"
    }
    let output = Command::new(cmd)
        .args(args)
        .creation_flags(0x08000000)
        .output();
    match output {
        Ok(out) => {
            let combined = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            )
            .to_lowercase();
            combined.contains(&contains.to_lowercase())
        }
        Err(_) => false,
    }
}

/// Returns true when the given registry key path does NOT exist.
fn check_registry_key_absent(root_key: &str, path: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hkey = match root_key.to_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKCU" | "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKCR" | "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKU" | "HKEY_USERS" => HKEY_USERS,
        _ => return false,
    };
    RegKey::predef(hkey).open_subkey(path).is_err()
}

/// Set or remove SvcHostSplitDisable on every non-Xbox service subkey under
/// HKLM\SYSTEM\CurrentControlSet\Services.
/// enable_split = false  → sets SvcHostSplitDisable = DWORD 1 (disable splitting)
/// enable_split = true   → deletes SvcHostSplitDisable (restore default splitting)
/// Only touches subkeys that have a "Start" value (real services).
fn apply_svc_host_split_all(enable_split: bool) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    const SERVICES_PATH: &str = "SYSTEM\\CurrentControlSet\\Services";
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let services_key = hklm
        .open_subkey_with_flags(SERVICES_PATH, KEY_READ)
        .map_err(|e| format!("Failed to open Services key: {}", e))?;

    for sub_name in services_key.enum_keys().flatten() {
        // Skip Xbox services
        let lower = sub_name.to_lowercase();
        if lower.contains("xbl") || lower.contains("xbox") {
            continue;
        }

        let sub_path = format!("{}\\{}", SERVICES_PATH, sub_name);

        // Only act on real services (those that have a "Start" value)
        let sub_key = match hklm.open_subkey_with_flags(
            &sub_path,
            KEY_READ | KEY_SET_VALUE,
        ) {
            Ok(k) => k,
            Err(_) => continue,
        };

        if sub_key.get_value::<u32, _>("Start").is_err() {
            continue; // not a real service
        }

        if enable_split {
            // Restore default: remove SvcHostSplitDisable
            let _ = sub_key.delete_value("SvcHostSplitDisable");
        } else {
            // Disable splitting: set to 1
            if let Err(e) = sub_key.set_value("SvcHostSplitDisable", &1u32) {
                eprintln!("[SvcHostSplit] Warning: set on {} failed: {}", sub_name, e);
            }
        }
    }
    Ok(())
}

/// Check if MSI is enabled globally for all PCI device classes at the specified priority.
/// Returns true only if ALL devices in all classes have MSISupported=1, MessageNumberLimit=1, Priority=priority.
fn check_msi_enabled_globally(priority: u32) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI";
    const CLASSES: &[&str] = &["Display", "SCSIAdapter", "Net", "USB", "HDC"];

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let _pci_key = match hklm.open_subkey_with_flags(PCI_PATH, KEY_READ) {
        Ok(k) => k,
        Err(_) => return false,
    };

    for class in CLASSES {
        let class_key_path = format!("{}\\{}", PCI_PATH, class);
        let class_key = match hklm.open_subkey_with_flags(&class_key_path, KEY_READ) {
            Ok(k) => k,
            Err(_) => continue,
        };

        for dev_name in class_key.enum_keys().flatten() {
            let dev_path = format!("{}\\{}\\{}", PCI_PATH, class, dev_name);
            let _dev_key = match hklm.open_subkey_with_flags(&dev_path, KEY_READ) {
                Ok(k) => k,
                Err(_) => continue,
            };

            let msi_path = format!("{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties", dev_path);
            let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_READ) {
                Ok(k) => k,
                Err(_) => return false,
            };

            let msi_supported: u32 = match msi_key.get_value("MSISupported") {
                Ok(v) => v,
                Err(_) => return false,
            };
            let msg_limit: u32 = match msi_key.get_value("MessageNumberLimit") {
                Ok(v) => v,
                Err(_) => return false,
            };
            let prio: u32 = match msi_key.get_value("Priority") {
                Ok(v) => v,
                Err(_) => return false,
            };

            if msi_supported != 1 || msg_limit != 1 || prio != priority {
                return false;
            }
        }
    }
    true
}

/// Apply MSI settings to all devices of a given PCI class.
fn apply_msi_set(class: &str, priority: u32) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI";
    let class_key_path = format!("{}\\{}", PCI_PATH, class);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = hklm
        .open_subkey_with_flags(&class_key_path, KEY_READ)
        .map_err(|e| format!("Failed to open PCI class {}: {}", class, e))?;

    for dev_name in class_key.enum_keys().flatten() {
        let dev_path = format!("{}\\{}\\{}", PCI_PATH, class, dev_name);
        let base_path = format!("{}\\Device Parameters\\Interrupt Management", dev_path);
        let msi_path = format!("{}\\{}", base_path, "MessageSignaledInterruptProperties");

        // Create keys if they don't exist
        if let Err(_) = hklm.create_subkey(&base_path) {
            continue;
        }
        if let Err(_) = hklm.create_subkey(&msi_path) {
            continue;
        }

        let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_SET_VALUE) {
            Ok(k) => k,
            Err(_) => continue,
        };

        let _ = msi_key.set_value("MSISupported", &1u32);
        let _ = msi_key.set_value("MessageNumberLimit", &1u32);
        let _ = msi_key.set_value("Priority", &priority);
    }
    Ok(())
}

/// Remove MSI settings from all devices of a given PCI class.
fn apply_msi_remove(class: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI";
    let class_key_path = format!("{}\\{}", PCI_PATH, class);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = match hklm.open_subkey_with_flags(&class_key_path, KEY_READ) {
        Ok(k) => k,
        Err(_) => return Ok(()),
    };

    for dev_name in class_key.enum_keys().flatten() {
        let dev_path = format!("{}\\{}\\{}", PCI_PATH, class, dev_name);
        let msi_path = format!("{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties", dev_path);

        let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_SET_VALUE) {
            Ok(k) => k,
            Err(_) => continue,
        };

        let _ = msi_key.delete_value("MSISupported");
        let _ = msi_key.delete_value("MessageNumberLimit");
        let _ = msi_key.delete_value("Priority");
    }
    Ok(())
}

/// Check if MSI is enabled on all network adapters at the specified priority.
fn check_msi_enabled_on_net(priority: u32) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    const PCI_PATH: &str = "SYSTEM\\CurrentControlSet\\Enum\\PCI\\Net";

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let net_key = match hklm.open_subkey_with_flags(PCI_PATH, KEY_READ) {
        Ok(k) => k,
        Err(_) => return false,
    };

    for dev_name in net_key.enum_keys().flatten() {
        let dev_path = format!("{}\\{}", PCI_PATH, dev_name);
        let msi_path = format!("{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties", dev_path);

        let msi_key = match hklm.open_subkey_with_flags(&msi_path, KEY_READ) {
            Ok(k) => k,
            Err(_) => return false,
        };

        let msi_supported: u32 = match msi_key.get_value("MSISupported") {
            Ok(v) => v,
            Err(_) => return false,
        };
        let msg_limit: u32 = match msi_key.get_value("MessageNumberLimit") {
            Ok(v) => v,
            Err(_) => return false,
        };
        let prio: u32 = match msi_key.get_value("Priority") {
            Ok(v) => v,
            Err(_) => return false,
        };

        if msi_supported != 1 || msg_limit != 1 || prio != priority {
            return false;
        }
    }
    true
}

/// Return true when at least one interface has a NameServer value that contains `ip`.
fn check_dns_servers_contain(ip: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for path in tcpip_interface_subkeys() {
        if let Ok(k) = hklm.open_subkey_with_flags(&path, KEY_READ) {
            if let Ok(val) = k.get_value::<String, _>("NameServer") {
                // NameServer is comma-separated, e.g. "8.8.8.8,8.8.4.4"
                if val.split(',').any(|s| s.trim() == ip) {
                    return true;
                }
            }
        }
    }
    false
}

// Placeholder for the global tweak registry
pub struct TweakContext {
    pub tweaks: Vec<Tweak>,
}

/// Check if a tweak is currently enabled on the system by evaluating its TweakCheck
fn check_tweak_enabled(check: &TweakCheck) -> bool {
    match check {
        TweakCheck::Registry {
            root_key,
            path,
            key,
            expected_value,
        } => check_registry_value(root_key, path, key, expected_value),
        TweakCheck::MultiScheduledTaskDisabled { names } => {
    names.iter().all(|name| {
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
    })
},
        TweakCheck::Powershell {
            script,
            expected_output,
        } => check_powershell_output(script, expected_output),
        TweakCheck::NetAdapterProperty {
            property,
            expected_value,
        } => check_nic_property(property, expected_value),
        TweakCheck::DnsServersContain { ip } => check_dns_servers_contain(ip),
        TweakCheck::MultiRegistry { checks } => checks.iter().all(|c| {
            check_registry_value(&c.root_key, &c.path, &c.key, &c.expected_value)
        }),
        TweakCheck::MpComputerStatus { check } => check_mp_computer_status(check),
        TweakCheck::DefenderExclusionPath { paths } => check_defender_exclusion_paths(paths),
        TweakCheck::CommandOutputContains { cmd, args, contains } => {
            check_command_output_contains(cmd, args, contains)
        }
        TweakCheck::RegistryKeyAbsent { root_key, path } => {
            check_registry_key_absent(root_key, path)
        }
        TweakCheck::ScheduledTaskDisabled { name } => {
            let output = Command::new("schtasks")
                .args(&["/query", "/tn", name, "/v", "/fo", "list"])
                .creation_flags(0x08000000)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: std::os::windows::process::ExitStatusExt::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            let out_str = String::from_utf8_lossy(&output.stdout).to_string();
            // In English Windows, "Status: Disabled"
            out_str.contains("Disabled") || out_str.contains("Disabilitato")
        }
        TweakCheck::ServiceDisabled { name } => {
            let output = Command::new("sc")
                .args(&["qc", name])
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
                .args(&["qc", name])
                .creation_flags(0x08000000)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: std::os::windows::process::ExitStatusExt::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            let out_str = String::from_utf8_lossy(&output.stdout).to_string();
            let mode_upper = mode.to_uppercase();
            out_str.contains(&mode_upper) || out_str.contains(&format!("START_TYPE         : {}", mode_upper))
        }
        TweakCheck::MultiServiceDisabled { names } => {
            names.iter().all(|name| {
                let output = Command::new("sc")
                    .args(&["qc", name])
                    .creation_flags(0x08000000)
                    .output()
                    .unwrap_or_else(|_| std::process::Output {
                        status: std::os::windows::process::ExitStatusExt::from_raw(1),
                        stdout: Vec::new(),
                        stderr: Vec::new(),
                    });
                let out_str = String::from_utf8_lossy(&output.stdout).to_string();
                out_str.contains("DISABLED") || out_str.contains("4  DISABLED")
            })
        }
        TweakCheck::MsiEnabledGlobally { priority } => {
            check_msi_enabled_globally(*priority)
        }
        TweakCheck::MsiEnabledOnNet { priority } => {
            check_msi_enabled_on_net(*priority)
        }
        TweakCheck::NetworkInterfacesCheck { key, expected_value } => {
            crate::modules::registry::operations::check_network_interfaces(key, expected_value)
                .unwrap_or(false)
        }
    }
}

/// Check if a registry value matches the expected value
fn check_registry_value(
    root_key: &str,
    path: &str,
    key: &str,
    expected_value: &RegistryValue,
) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkey = match root_key.to_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKCU" | "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKCR" | "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKU" | "HKEY_USERS" => HKEY_USERS,
        _ => return false,
    };

    let regkey = match RegKey::predef(hkey).open_subkey(path) {
        Ok(k) => k,
        Err(_) => return false,
    };

    match expected_value {
        RegistryValue::DWord(expected) => {
            regkey.get_value::<u32, _>(key).map(|v| v == *expected).unwrap_or(false)
        }
        RegistryValue::QWord(expected) => {
            regkey.get_value::<u64, _>(key).map(|v| v == *expected).unwrap_or(false)
        }
        RegistryValue::String(expected) => {
            regkey.get_value::<String, _>(key).map(|v| v == *expected).unwrap_or(false)
        }
        RegistryValue::Binary(expected) => {
            // Use get_raw_value to avoid winreg version conflicts
            regkey.get_raw_value(key)
                .map(|v| v.bytes == *expected)
                .unwrap_or(false)
        }
        RegistryValue::MultiString(expected) => {
            regkey.get_value::<Vec<String>, _>(key)
                .map(|v| v == *expected)
                .unwrap_or(false)
        }
    }

}

/// Check if PowerShell script output matches expected value
fn check_powershell_output(script: &str, expected_output: &str) -> bool {
    use std::io::Write;
    use std::process::Stdio;
    // BUG-H3 fix: RemoteSigned + script via stdin (same as apply/undo paths)
    let mut child = match Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "RemoteSigned",
            "-Command",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", script);
    }
    match child.wait_with_output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            stdout == expected_output.trim().to_lowercase()
        }
        Err(_) => false,
    }
}

#[tauri::command]

pub fn check_is_admin() -> bool {
    is_admin()
}

// Startup Manager Commands
#[tauri::command]
pub async fn scan_startup() -> Result<Vec<crate::modules::startup::types::StartupItem>, String> {
    tokio::task::spawn_blocking(crate::modules::startup::scan_all_startup_items)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_startup_item_enabled(id: String, enable: bool) -> Result<(), String> {
    let id_clone = id.clone();
    tokio::task::spawn_blocking(move || {
        crate::modules::startup::toggle_item(id_clone, enable)
    })
    .await
    .map_err(|e| e.to_string())?
}

use crate::modules::network::dns_benchmark::{self, DnsBenchmarkResult};

fn get_current_windows_build() -> u32 {
    use winreg::{enums::*, RegKey};
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .and_then(|k| k.get_value::<String, _>("CurrentBuildNumber"))
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn win11_only_tweaks() -> std::collections::HashMap<&'static str, u32> {
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

#[tauri::command]
pub async fn benchmark_dns() -> Result<Vec<DnsBenchmarkResult>, String> {
    tokio::task::spawn_blocking(dns_benchmark::run_benchmark)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn apply_dns_server(primary: String, secondary: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        dns_benchmark::apply_dns(primary, secondary)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_tweaks(ctx: State<'_, Mutex<TweakContext>>, state: State<'_, Mutex<AppState>>) -> Result<Vec<Tweak>, String> {
    // Extract data and immediately drop the locks
    let tweaks = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context.tweaks.clone()
    };
    let applied = {
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.applied_tweaks.clone()
    };

    let build = get_current_windows_build();
    let min_builds = win11_only_tweaks();

    // Move heavy I/O (registry, sc, schtasks) to blocking thread pool
    tokio::task::spawn_blocking(move||
        tweaks
            .iter()
            .filter(|t| min_builds.get(t.id.as_str()).map_or(true, |&min| build >= min))
            .map(|t| {
                let mut tweak = t.clone();
                // First, check actual system state via TweakCheck if available
                if let Some(ref check) = tweak.check {
                    tweak.enabled = check_tweak_enabled(check);
                } else {
                    // Fallback to app state for tweaks without explicit checks
                    tweak.enabled = applied.contains(&tweak.id);
                }
                tweak
            })
            .collect::<Vec<_>>()
    )
    .await
    .map_err(|e| format!("Task join error: {}", e))
}

/// Fast version - returns tweaks immediately without running any checks
/// Used for instant UI loading
#[tauri::command]
pub async fn get_tweaks_fast(
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Tweak>, String> {
    // BUG-M1 fix: acquire, clone, drop each lock before acquiring the next
    let tweaks = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context.tweaks.clone()
    }; // ctx guard dropped here

    let applied = {
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.applied_tweaks.clone()
    }; // state guard dropped here

    let build = get_current_windows_build();
    let min_builds = win11_only_tweaks();

    Ok(tweaks
        .into_iter()
        .filter(|t| min_builds.get(t.id.as_str()).map_or(true, |&min| build >= min))
        .map(|mut t| {
            t.enabled = applied.contains(&t.id);
            t
        })
        .collect())
}

/// Check tweaks by category - runs checks in background and emits events
/// Called after UI loads to progressively update tweak states
#[tauri::command]
pub async fn check_category(
    category: String,
    ctx: State<'_, Mutex<TweakContext>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Get tweaks for this category
    let tweaks: Vec<Tweak> = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .filter(|t| format!("{:?}", t.category) == category)
            .cloned()
            .collect()
    };

    // Clone app handle for the spawned task
    let app_clone = app.clone();
    let category_clone = category.clone();
    
    // Run checks in background thread - SEQUENTIAL (no rayon)
    tokio::task::spawn_blocking(move || {
        // Sequential execution - avoids thread pool saturation
        let results: Vec<(String, bool)> = tweaks
            .iter()
            .filter_map(|tweak| {
                tweak.check.as_ref().map(|check| {
                    (tweak.id.clone(), check_tweak_enabled(check))
                })
            })
            .collect();
        
        // Emit results sequentially (order doesn't matter for UI)
        for (id, enabled) in results {
            let _ = app_clone.emit("tweak-check-result", serde_json::json!({
                "id": id,
                "enabled": enabled
            }));
        }
        // Emit completion for this category
        let _ = app_clone.emit("category-check-complete", serde_json::json!({
            "category": category_clone
        }));
    }).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn apply_tweak(
    id: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 1. Find the tweak (Keep lock critical section short)
    let tweak = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or("Tweak ID not found")?
    };

    // Emit progress start
    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id.clone(),
        "status": "applying",
        "progress": 0
    }));

    // 2. Execute operations (Async/Blocking wrapper)
    // We use tokio::spawn_blocking for heavy IO/Process operations to avoid blocking the async runtime
    let tweak_clone = tweak.clone();
    let id_closure = id.clone();
    let app_closure = app.clone();

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let id = id_closure;
        let app = app_closure;

        // Create a single backup manager for all registry operations
        let backup_path = crate::modules::utils::dirs::get_backup_dir()
            .map_err(|e| format!("Cannot determine backup directory: {}", e))?;
        let mut backup_mgr = RegistryBackup::new(backup_path.clone());

        println!("[TWEAK APPLY] Applying ID: {}", tweak_clone.id);
        for (i, op) in tweak_clone.operations.iter().enumerate() {
            println!("[TWEAK APPLY] Operation {}/{}: {:?}", i + 1, tweak_clone.operations.len(), op);
            match op {
                TweakOperation::RegistrySet {
                    root_key,
                    path,
                    key,
                    value,
                } => {
                    println!("  -> RegistrySet: {}\\{}\\{} = {:?}", root_key, path, key, value);
                    if let Err(e) = backup_mgr.backup_value(root_key, path, key) {
                        eprintln!("  -> Backup warning for {}: {}", key, e);
                    }

                    if let Err(e) = apply_registry_tweak(op) {
                       eprintln!("  -> Registry Error: {:?}", e);
                       return Err(format!("Registry error: {:?}", e));
                    }
                    println!("  -> Registry Set Success");
                }
                TweakOperation::RegistryDelete {
                    root_key,
                    path,
                    key,
                } => {
                    println!("  -> RegistryDelete: {}\\{}\\{}", root_key, path, key);
                    if let Err(e) = backup_mgr.backup_value(root_key, path, key) {
                        eprintln!("  -> Backup warning for {}: {}", key, e);
                    }

                    if let Err(e) = apply_registry_tweak(op) {
                       eprintln!("  -> Registry Error: {:?}", e);
                       return Err(format!("Registry error: {:?}", e));
                    }
                    println!("  -> Registry Delete Success");
                }
                TweakOperation::Command { cmd, args } => {
                    println!("  -> Command: {} {:?}", cmd, args);
                    validate_command(&cmd)?;
                    let output = Command::new(cmd)
                        .args(args)
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Command exec failed: {}", e))?;

                    if !output.stdout.is_empty() {
                        println!("    [STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                        eprintln!("    [STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        return Err(format!("Command returned non-zero code: {:?}", output.status.code()));
                    }
                }
                TweakOperation::Powershell { script } => {
                    let id_closure_log = id.clone();
                    let app_log = app.clone();
                    // Helper to emit logs
                    let log = move |msg: String| {
                        println!("    [PS STREAM] {}", msg);
                        let _ = app_log.emit("tweak-output", serde_json::json!({
                            "id": id_closure_log,
                            "type": "stdout",
                            "line": msg
                        }));
                    };

                    log("Executing Script...".to_string());

                    use std::io::{BufRead, BufReader, Write};
                    use std::process::Stdio;

                    let app_handle = app.clone();
                    let id_clone = id.clone();

                    // SECURITY FIX: Use RemoteSigned + pass script via stdin
                    let mut child = Command::new("powershell")
                        .args([
                            "-NoProfile",
                            "-NonInteractive",
                            "-ExecutionPolicy", "RemoteSigned", // not Bypass
                            "-Command", "-",                    // read from stdin
                        ])
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW
                        .spawn()
                        .map_err(|e| format!("PowerShell spawn failed: {}", e))?;

                    // Write script to stdin (not CLI arg)
                    if let Some(mut stdin) = child.stdin.take() {
                        writeln!(stdin, "{}", script)
                            .map_err(|e| format!("Failed to write script to stdin: {}", e))?;
                    }

                    // Register PID
                    let pid = child.id();
                    {
                        use tauri::Manager;
                        if let Some(state) = app_handle.try_state::<Mutex<crate::modules::utils::process_manager::ProcessManager>>() {
                             if let Ok(mut mgr) = state.lock() {
                                 mgr.register(id_clone.clone(), pid);
                             }
                        }
                    }

                    // Handle stdout streaming
                    if let Some(stdout) = child.stdout.take() {
                        let app_handle = app_handle.clone();
                        let id_clone = id_clone.clone();
                        // We are already in spawn_blocking, so we can block on reading
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            match line {
                                Ok(l) => {
                                    let clean = strip_str(&l);
                                    println!("    [PS STREAM] {}", clean);
                                    let _ = app_handle.emit("tweak-output", serde_json::json!({
                                        "id": id_clone,
                                        "type": "stdout",
                                        "line": clean
                                    }));
                                }
                                Err(e) => eprintln!("Error reading stdout: {}", e),
                            }
                        }
                    }

                    // Wait for completion
                    let output = child.wait_with_output().map_err(|e| format!("Wait failed: {}", e))?;

                    // Unregister PID
                    {
                        use tauri::Manager;
                        if let Some(state) = app_handle.try_state::<Mutex<crate::modules::utils::process_manager::ProcessManager>>() {
                             if let Ok(mut mgr) = state.lock() {
                                 mgr.unregister(&id_clone);
                             }
                        }
                    }

                    if !output.status.success() {
                        let stderr = strip_str(&String::from_utf8_lossy(&output.stderr));
                         let _ = app_handle.emit("tweak-output", serde_json::json!({
                                    "id": id_clone,
                                    "type": "stderr",
                                    "line": stderr
                                }));
                        return Err(format!("PowerShell script failed with exit code {:?}", output.status.code()));
                    } else {
                        let _ = app_handle.emit("tweak-output", serde_json::json!({
                            "id": id_clone,
                            "type": "stdout",
                            "line": "Process finished successfully."
                        }));
                    }
                }
                TweakOperation::ServiceDisable { name } => {
                    println!("  -> ServiceDisable: {}", name);
                    let output = Command::new("sc")
                        .args(&["stop", name])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to stop service {}: {}", name, e))?;

                    if !output.status.success() {
                        // ignore error, service might already be stopped
                    }

                    let output_config = Command::new("sc")
                        .args(&["config", name, "start=", "disabled"])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to disable service {}: {}", name, e))?;

                    if !output_config.status.success() {
                        eprintln!("Warning: Failed to disable service {}", name);
                    }
                }
                TweakOperation::ServiceSetMode { name, mode } => {
                    println!("  -> ServiceSetMode: {} -> {}", name, mode);
                    
                    let sc_mode = match mode.to_lowercase().as_str() {
                        "automatic" | "auto" => "auto",
                        "manual" | "demand" => "demand",
                        "disabled" => "disabled",
                        "delayed-auto" | "delayedauto" => "delayed-auto",
                        _ => "demand", // default fallback
                    };

                    let output = Command::new("sc")
                        .args(&["config", name, "start=", sc_mode])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to set service mode {}: {}", name, e))?;

                    if !output.status.success() {
                        eprintln!("Warning: Failed to set service mode {}", name);
                    }
                }
                TweakOperation::ScheduledTaskDisable { path, name } => {
                    println!("  -> ScheduledTaskDisable: {}\\{}", path, name);
                    let full_path = if path == "\\" || path.is_empty() {
                        format!("\\{}", name)
                    } else {
                        format!("{}\\{}", path, name)
                    };
                    
                    let output = Command::new("schtasks")
                        .args(&["/Change", "/TN", &full_path, "/Disable"])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to disable task {}: {}", name, e))?;

                    if !output.status.success() {
                        eprintln!("Warning: Failed to disable task {}", name);
                    }
                }
                TweakOperation::ScheduledTaskEnable { path, name } => {
                    println!("  -> ScheduledTaskEnable: {}\\{}", path, name);
                    let full_path = if path == "\\" || path.is_empty() {
                        format!("\\{}", name)
                    } else {
                        format!("{}\\{}", path, name)
                    };
                    
                    let output = Command::new("schtasks")
                        .args(&["/Change", "/TN", &full_path, "/Enable"])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to enable task {}: {}", name, e))?;

                    if !output.status.success() {
                        eprintln!("Warning: Failed to enable task {}", name);
                    }
                }
                TweakOperation::FileOperation(file_op) => {
                    use crate::modules::types::FileOp;
                    use std::fs;

                    match file_op {
                        FileOp::Delete { path } => {
                            println!("  -> FileOp Delete: {}", path);
                            let safe = safe_path_existing(&path)?;
                            if safe.is_dir() {
                                fs::remove_dir_all(&safe)
                                    .map_err(|e| format!("Failed to delete directory {}: {}", path, e))?;
                            } else {
                                fs::remove_file(&safe)
                                    .map_err(|e| format!("Failed to delete file {}: {}", path, e))?;
                            }
                        }
                        FileOp::Copy { src, dest } => {
                            println!("  -> FileOp Copy: {} -> {}", src, dest);
                            let safe_src = safe_path_existing(&src)?;
                            let safe_dest = safe_path_new(&dest)?;
                            if let Some(parent) = safe_dest.parent() {
                                fs::create_dir_all(parent)
                                    .map_err(|e| format!("Failed to create dest dir: {}", e))?;
                            }
                            fs::copy(&safe_src, &safe_dest)
                                .map_err(|e| format!("Failed to copy {} to {}: {}", src, dest, e))?;
                        }
                        FileOp::Move { src, dest } => {
                            println!("  -> FileOp Move: {} -> {}", src, dest);
                            let safe_src = safe_path_existing(&src)?;
                            let safe_dest = safe_path_new(&dest)?;
                            if let Some(parent) = safe_dest.parent() {
                                fs::create_dir_all(parent)
                                    .map_err(|e| format!("Failed to create dest dir: {}", e))?;
                            }
                            fs::rename(&safe_src, &safe_dest)
                                .map_err(|e| format!("Failed to move {} to {}: {}", src, dest, e))?;
                        }
                        FileOp::Write { path, content } => {
                            println!("  -> FileOp Write: {}", path);
                            let safe = safe_path_new(&path)?;
                            if let Some(parent) = safe.parent() {
                                fs::create_dir_all(parent)
                                    .map_err(|e| format!("Failed to create parent dir: {}", e))?;
                            }
                            fs::write(&safe, &content)
                                .map_err(|e| format!("Failed to write file {}: {}", path, e))?;
                        }
                    }
                }
                TweakOperation::NetAdapterProperty { property, value } => {
                    println!("  -> NetAdapterProperty: {} = {}", property, value);
                    set_nic_property(property, value)
                        .map_err(|e| format!("NetAdapterProperty error: {}", e))?;
                    println!("  -> NetAdapterProperty Success");
                }
                TweakOperation::SetDnsServers { primary, secondary } => {
                    println!("  -> SetDnsServers: {}, {}", primary, secondary);
                    set_dns_servers(primary, secondary)
                        .map_err(|e| format!("SetDnsServers error: {}", e))?;
                    println!("  -> SetDnsServers Success");
                }
                TweakOperation::ResetDnsServers => {
                    println!("  -> ResetDnsServers");
                    reset_dns_servers()
                        .map_err(|e| format!("ResetDnsServers error: {}", e))?;
                    println!("  -> ResetDnsServers Success");
                }
                TweakOperation::DefenderServiceControl { services, action } => {
                    println!("  -> DefenderServiceControl: {:?} action={}", services, action);
                    control_defender_services(services, action)
                        .map_err(|e| format!("DefenderServiceControl error: {}", e))?;
                    println!("  -> DefenderServiceControl Success");
                }
                TweakOperation::DefenderExclusion { paths, action } => {
                    println!("  -> DefenderExclusion: {:?} action={}", paths, action);
                    set_defender_exclusions(paths, action)
                        .map_err(|e| format!("DefenderExclusion error: {}", e))?;
                    println!("  -> DefenderExclusion Success");
                }
                TweakOperation::SvcHostSplitAll { enable_split } => {
                    println!("  -> SvcHostSplitAll: enable_split={}", enable_split);
                    apply_svc_host_split_all(*enable_split)
                        .map_err(|e| format!("SvcHostSplitAll error: {}", e))?;
                    println!("  -> SvcHostSplitAll Success");
                }
                TweakOperation::MsiSet { class, priority } => {
                    println!("  -> MsiSet: class={}, priority={}", class, priority);
                    apply_msi_set(&class, *priority)
                        .map_err(|e| format!("MsiSet error: {}", e))?;
                    println!("  -> MsiSet Success");
                }
                TweakOperation::MsiRemove { class } => {
                    println!("  -> MsiRemove: class={}", class);
                    apply_msi_remove(&class)
                        .map_err(|e| format!("MsiRemove error: {}", e))?;
                    println!("  -> MsiRemove Success");
                }
                TweakOperation::MsiSetNet { priority } => {
                    println!("  -> MsiSetNet: priority={}", priority);
                    apply_msi_set("Net", *priority)
                        .map_err(|e| format!("MsiSetNet error: {}", e))?;
                    println!("  -> MsiSetNet Success");
                }
                TweakOperation::MsiRemoveNet => {
                    println!("  -> MsiRemoveNet");
                    apply_msi_remove("Net")
                        .map_err(|e| format!("MsiRemoveNet error: {}", e))?;
                    println!("  -> MsiRemoveNet Success");
                }
                    TweakOperation::NetworkInterfacesSet { key, value: _ } => {
                    println!("  -> NetworkInterfacesSet: key={}", key);
                    use crate::modules::registry::operations::apply_network_interface_tweak;
                    apply_network_interface_tweak(op)
                        .map_err(|e| format!("NetworkInterfacesSet error: {}", e))?;
                    println!("  -> NetworkInterfacesSet Success");
                }
                TweakOperation::NetworkInterfacesDelete { key } => {
                    println!("  -> NetworkInterfacesDelete: key={}", key);
                    use crate::modules::registry::operations::apply_network_interface_tweak;
                    apply_network_interface_tweak(op)
                        .map_err(|e| format!("NetworkInterfacesDelete error: {}", e))?;
                    println!("  -> NetworkInterfacesDelete Success");
                }
            }
        }

        // Persist all registry backups to disk
        if let Err(e) = backup_mgr.save(backup_path) {
            eprintln!("Warning: Failed to persist backup: {}", e);
        }

        Ok(())
    }).await.map_err(|e| e.to_string())??;

    // 3. Update State & Persist
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.applied_tweaks.insert(id.clone());

        let state_path = crate::modules::utils::dirs::get_state_path()
            .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));

        if let Err(e) = app_state.save(state_path) {
            eprintln!("Failed to save app state: {}", e);
        }
    }

    // Emit success
    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id,
        "status": "success",
        "progress": 100
    }));

    Ok(())
}

#[tauri::command]
pub fn kill_tweak_process(
    id: String,
    proc_mgr: State<Mutex<crate::modules::utils::process_manager::ProcessManager>>,
) -> Result<(), String> {
    let pid = {
        let mgr = proc_mgr.lock().map_err(|e| e.to_string())?;
        mgr.get_pid(&id)
    };
    if let Some(pid) = pid {
        println!("[PROCESS KILL] Killing PID {} for tweak: {}", pid, id);
        // BUG-M2 fix: propagate taskkill errors instead of silently ignoring
        let output = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string(), "/T"])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| format!("taskkill spawn failed: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("taskkill failed (PID {}): {}", pid, stderr.trim()));
        }
    } else {
        println!("[PROCESS KILL] No active PID found for tweak: {}", id);
    }
    Ok(())
}

#[tauri::command]
pub async fn undo_tweak(
    id: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id.clone(),
        "status": "reverting",
        "progress": 0
    }));

    let tweak = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or("Tweak ID not found")?
    };

    let backup_path = crate::modules::utils::dirs::get_backup_dir()
        .map_err(|e| format!("Cannot determine backup directory: {}", e))?;

    let id_clone = id.clone();
    let app_clone = app.clone();

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let id = id_clone;
        let _app = app_clone;

        // Try to load backup manager for registry operations
        let backup_mgr = RegistryBackup::load(backup_path.clone()).ok();

        // Track which registry keys were restored from backup (to avoid double-revert)
        let mut restored_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

        // First, restore registry values from backup
        for op in &tweak.operations {
            match op {
                TweakOperation::RegistrySet {
                    root_key,
                    path,
                    key,
                    ..
                }
                | TweakOperation::RegistryDelete {
                    root_key,
                    path,
                    key,
                } => {
                    if let Some(ref mgr) = backup_mgr {
                        if mgr.restore_tweak_backup(root_key, path, key).unwrap_or(false) {
                            restored_keys.insert(format!("{}::{}::{}", root_key, path, key));
                        }
                    }
                }
                _ => {}
            }
        }

        // Then, execute explicit revert operations (skip keys already restored from backup)
        println!("[TWEAK REVERT] Reverting ID: {}", id);
        if let Some(ref revert_ops) = tweak.revert_operations {
            for (i, op) in revert_ops.iter().enumerate() {
                println!("[TWEAK REVERT] Operation {}/{}: {:?}", i + 1, revert_ops.len(), op);

                // Skip registry operations that were already restored from backup
                match op {
                    TweakOperation::RegistrySet {
                        root_key,
                        path,
                        key,
                        ..
                    }
                    | TweakOperation::RegistryDelete {
                        root_key,
                        path,
                        key,
                    } => {
                        let key_id = format!("{}::{}::{}", root_key, path, key);
                        if restored_keys.contains(&key_id) {
                            println!("  -> Skipped (already restored from backup): {}", key_id);
                            continue;
                        }
                    }
                    _ => {}
                }

                match op {
                    TweakOperation::RegistrySet { .. } | TweakOperation::RegistryDelete { .. } => {
                         println!("  -> Registry Operation (Revert): {:?}", op);
                         apply_registry_tweak(op).map_err(|e| format!("Revert registry error: {:?}", e))?;
                         println!("  -> Revert Registry Success");
                    }
                    TweakOperation::Powershell { script } => {
                        println!("  -> Revert PowerShell: {}", script);
                        use std::process::Stdio;
                        use std::io::Write;
                        
                        // SECURITY FIX: Use RemoteSigned + pass script via stdin
                        let mut child = Command::new("powershell")
                            .args([
                                "-NoProfile",
                                "-NonInteractive",
                                "-ExecutionPolicy", "RemoteSigned",
                                "-Command", "-",
                            ])
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .creation_flags(0x08000000)
                            .spawn()
                            .map_err(|e| format!("Revert PowerShell spawn failed: {}", e))?;

                        // Write script to stdin (not CLI arg)
                        if let Some(mut stdin) = child.stdin.take() {
                            writeln!(stdin, "{}", script)
                                .map_err(|e| format!("Failed to write script to stdin: {}", e))?;
                        }

                        let output = child.wait_with_output()
                            .map_err(|e| format!("Revert PowerShell failed: {}", e))?;

                        if !output.stdout.is_empty() {
                             println!("    [REVERT STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                             eprintln!("    [REVERT STDERR] {}", String::from_utf8_lossy(&output.stderr));
                        }

                        if !output.status.success() {
                            eprintln!("Warning: Revert PowerShell script returned non-zero");
                        }
                    }
                    TweakOperation::ServiceSetMode { name, mode } => {
                        println!("  -> Revert ServiceSetMode: {} -> {}", name, mode);
                        let sc_mode = match mode.to_lowercase().as_str() {
                            "automatic" | "auto" => "auto",
                            "manual" | "demand" => "demand",
                            "disabled" => "disabled",
                            "delayed-auto" | "delayedauto" => "delayed-auto",
                            _ => "demand", // default fallback
                        };

                        let output = Command::new("sc")
                            .args(&["config", name, "start=", sc_mode])
                            .creation_flags(0x08000000)
                            .output()
                            .map_err(|e| format!("Revert service failed: {}", e))?;

                        if !output.status.success() {
                            eprintln!("Warning: Failed to revert service mode {}", name);
                        }
                    }
                    TweakOperation::ScheduledTaskDisable { path, name } => {
                        let full_path = if path == "\\" || path.is_empty() {
                            format!("\\{}", name)
                        } else {
                            format!("{}\\{}", path, name)
                        };
                        let _ = Command::new("schtasks")
                            .args(&["/Change", "/TN", &full_path, "/Disable"])
                            .creation_flags(0x08000000)
                            .output();
                    }
                    TweakOperation::ScheduledTaskEnable { path, name } => {
                        let full_path = if path == "\\" || path.is_empty() {
                            format!("\\{}", name)
                        } else {
                            format!("{}\\{}", path, name)
                        };
                        let _ = Command::new("schtasks")
                            .args(&["/Change", "/TN", &full_path, "/Enable"])
                            .creation_flags(0x08000000)
                            .output();
                    }
                    TweakOperation::ServiceDisable { name } => {
                        let _ = Command::new("sc")
                            .args(&["stop", name])
                            .creation_flags(0x08000000)
                            .output();
                        let _ = Command::new("sc")
                            .args(&["config", name, "start=", "disabled"])
                            .creation_flags(0x08000000)
                            .output();
                    }
                    TweakOperation::Command { cmd, args } => {
                        println!("  -> Revert Command: {} {:?}", cmd, args);
                        validate_command(&cmd)?;
                        let output = Command::new(cmd)
                            .args(args)
                            .creation_flags(0x08000000)
                            .output()
                            .map_err(|e| format!("Revert command exec failed: {}", e))?;
                        if !output.status.success() {
                            eprintln!(
                                "Warning: Revert command returned non-zero: {:?}",
                                output.status.code()
                            );
                        }
                    }
                    TweakOperation::NetAdapterProperty { property, value } => {
                        println!("  -> Revert NetAdapterProperty: {} = {}", property, value);
                        if let Err(e) = set_nic_property(property, value) {
                            eprintln!("Warning: Revert NetAdapterProperty failed: {}", e);
                        }
                    }
                    TweakOperation::SetDnsServers { primary, secondary } => {
                        println!("  -> Revert SetDnsServers: {}, {}", primary, secondary);
                        if let Err(e) = set_dns_servers(primary, secondary) {
                            eprintln!("Warning: Revert SetDnsServers failed: {}", e);
                        }
                    }
                    TweakOperation::ResetDnsServers => {
                        println!("  -> Revert ResetDnsServers");
                        if let Err(e) = reset_dns_servers() {
                            eprintln!("Warning: Revert ResetDnsServers failed: {}", e);
                        }
                    }
                    TweakOperation::DefenderServiceControl { services, action } => {
                        println!("  -> Revert DefenderServiceControl: {:?} action={}", services, action);
                        if let Err(e) = control_defender_services(services, action) {
                            eprintln!("Warning: Revert DefenderServiceControl failed: {}", e);
                        }
                    }
                    TweakOperation::DefenderExclusion { paths, action } => {
                        println!("  -> Revert DefenderExclusion: {:?} action={}", paths, action);
                        if let Err(e) = set_defender_exclusions(paths, action) {
                            eprintln!("Warning: Revert DefenderExclusion failed: {}", e);
                        }
                    }
                    TweakOperation::SvcHostSplitAll { enable_split } => {
                        println!("  -> Revert SvcHostSplitAll: enable_split={}", enable_split);
                        if let Err(e) = apply_svc_host_split_all(*enable_split) {
                            eprintln!("Warning: Revert SvcHostSplitAll failed: {}", e);
                        }
                    }
                    TweakOperation::MsiSet { class, priority } => {
                        println!("  -> Revert MsiSet: class={}, priority={}", class, priority);
                        if let Err(e) = apply_msi_remove(&class) {
                            eprintln!("Warning: Revert MsiSet failed: {}", e);
                        }
                    }
                    TweakOperation::MsiRemove { class } => {
                        println!("  -> Revert MsiRemove: class={}", class);
                        if let Err(e) = apply_msi_set(&class, 0) {
                            eprintln!("Warning: Revert MsiRemove failed: {}", e);
                        }
                    }
                    TweakOperation::MsiSetNet { priority } => {
                        println!("  -> Revert MsiSetNet: priority={}", priority);
                        if let Err(e) = apply_msi_remove("Net") {
                            eprintln!("Warning: Revert MsiSetNet failed: {}", e);
                        }
                    }
                    TweakOperation::MsiRemoveNet => {
                        println!("  -> Revert MsiRemoveNet");
                        if let Err(e) = apply_msi_set("Net", 0) {
                            eprintln!("Warning: Revert MsiRemoveNet failed: {}", e);
                        }
                    }
                    TweakOperation::NetworkInterfacesSet { key, value: _ } => {
                         println!("  -> Revert NetworkInterfacesSet: key={}", key);
                         // BUG-H4 fix: construct enum variant directly — no JSON string injection
                         let revert_op = TweakOperation::NetworkInterfacesDelete { key: key.clone() };
                         if let Err(e) = crate::modules::registry::operations::apply_network_interface_tweak(&revert_op) {
                             eprintln!("Warning: Revert NetworkInterfacesSet failed: {}", e);
                         }
                     }
                    TweakOperation::NetworkInterfacesDelete { key: _ } => {
                        println!("  -> Revert NetworkInterfacesDelete: cannot restore, skipping");
                    }
                    _ => {
                        println!("  -> Skipped unknown revert op: {:?}", op);
                    }
                }
            }
        }

        Ok(())
    }).await.map_err(|e| e.to_string())??;

    // Update State (Remove from applied)
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        if app_state.applied_tweaks.remove(&id) {
            let state_path = crate::modules::utils::dirs::get_state_path()
                .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));
            let _ = app_state.save(state_path);
        }
    }

    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id,
        "status": "revert-success",
        "progress": 100
    }));

    Ok(())
}

// ─── AI Commands ───────────────────────────────────────────────────────────────

use crate::modules::ai::{
    AnalysisResult, ChatMessage, DiagnosisResult,
    gemini, prompts, profiler,
};

#[tauri::command]
pub fn save_gemini_key(key: String) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err("API key cannot be empty".to_string());
    }
    if !key.starts_with("AIza") {
        return Err("Invalid Gemini API key format (must start with AIza)".to_string());
    }
    gemini::save_api_key(key.trim())
}

#[tauri::command]
pub fn get_gemini_key_status() -> bool {
    gemini::get_api_key().is_ok()
}

#[tauri::command]
pub fn delete_gemini_key() -> Result<(), String> {
    gemini::delete_api_key()
}

#[tauri::command]
pub async fn ai_analyze(
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<AnalysisResult, String> {
    let profile = profiler::scan_system_profile()?;

    let tweaks_summary = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        context.tweaks
            .iter()
            .map(|t| serde_json::json!({
                "id":              t.id,
                "name":            t.name,
                "description":     t.description,
                "risk_level":      format!("{:?}", t.warning_level),
                "category":        format!("{:?}", t.category),
                "currently_applied": st.applied_tweaks.contains(&t.id),
            }))
            .collect::<Vec<_>>()
    };

    let prompt = prompts::build_analyze_prompt(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&tweaks_summary).map_err(|e| e.to_string())?,
    );

    let raw = gemini::call_gemini(
        vec![("user".to_string(), prompt)],
        true,
    )
    .await?;

    serde_json::from_str::<AnalysisResult>(&raw)
        .map_err(|e| format!("Failed to parse Gemini response: {}\nRaw: {}", e, &raw[..200.min(raw.len())]))
}

#[tauri::command]
pub async fn ai_chat(
    message: String,
    history: Vec<ChatMessage>,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let profile = profiler::scan_system_profile()?;

    let applied_summary = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        context.tweaks
            .iter()
            .filter(|t| st.applied_tweaks.contains(&t.id))
            .map(|t| serde_json::json!({"id": t.id, "name": t.name}))
            .collect::<Vec<_>>()
    };

    let (ctx_user, ctx_model) = prompts::build_context_injection(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&applied_summary).map_err(|e| e.to_string())?,
    );

    let mut messages = vec![
        ("user".to_string(),  ctx_user),
        ("model".to_string(), ctx_model),
    ];
    for msg in history {
        messages.push((msg.role, msg.content));
    }
    messages.push(("user".to_string(), message));

    gemini::call_gemini(messages, false).await
}

#[tauri::command]
pub async fn ai_diagnose(
    problem: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<DiagnosisResult, String> {
    let profile = profiler::scan_system_profile()?;

    let applied_detail = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        context.tweaks
            .iter()
            .filter(|t| st.applied_tweaks.contains(&t.id))
            .map(|t| serde_json::json!({
                "id":          t.id,
                "name":        t.name,
                "description": t.description,
                "category":    format!("{:?}", t.category),
                "risk_level":  format!("{:?}", t.warning_level),
            }))
            .collect::<Vec<_>>()
    };

    let prompt = prompts::build_diagnose_prompt(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&applied_detail).map_err(|e| e.to_string())?,
        &problem,
    );

    let raw = gemini::call_gemini(
        vec![("user".to_string(), prompt)],
        true,
    )
    .await?;

    serde_json::from_str::<DiagnosisResult>(&raw)
        .map_err(|e| format!("Failed to parse diagnosis: {}", e))
}
