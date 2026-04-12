use std::os::windows::process::CommandExt;
use std::process::Command;

pub const DEFENDER_FEATURES_PATH: &str = "SOFTWARE\\Microsoft\\Windows Defender\\Features";
pub const DEFENDER_POLICY_PATH: &str = "SOFTWARE\\Policies\\Microsoft\\Windows Defender";
pub const DEFENDER_EXCLUSIONS_PATH: &str =
    "SOFTWARE\\Microsoft\\Windows Defender\\Exclusions\\Paths";

pub fn is_tamper_protection_on() -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey(DEFENDER_FEATURES_PATH) {
        if let Ok(val) = key.get_value::<u32, _>("TamperProtection") {
            return val == 5;
        }
    }
    false
}

pub fn check_mp_computer_status(check: &str) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    if is_tamper_protection_on() {
        return false;
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    match check {
        "realtime_disabled" => {
            let rtp_path = format!("{}\\Real-Time Protection", DEFENDER_POLICY_PATH);
            hklm.open_subkey(&rtp_path)
                .and_then(|k| k.get_value::<u32, _>("DisableRealtimeMonitoring"))
                .map(|v| v == 1)
                .unwrap_or(false)
        }
        "av_disabled" => {
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

pub fn check_defender_exclusion_paths(paths: &[String]) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = match hklm.open_subkey(DEFENDER_EXCLUSIONS_PATH) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let existing: Vec<String> = key
        .enum_values()
        .flatten()
        .map(|(name, _)| name.to_lowercase())
        .collect();
    paths.iter().all(|p| existing.contains(&p.to_lowercase()))
}

pub fn set_defender_exclusions(paths: &[String], action: &str) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    match action {
        "add" => {
            let (key, _) = hklm
                .create_subkey(DEFENDER_EXCLUSIONS_PATH)
                .map_err(|e| format!("Failed to open/create Exclusions\\Paths key: {}", e))?;
            for path in paths {
                key.set_value(path, &0u32)
                    .map_err(|e| format!("Failed to add exclusion '{}': {}", path, e))?;
            }
        }
        "remove" => {
            if let Ok(key) = hklm.open_subkey_with_flags(DEFENDER_EXCLUSIONS_PATH, KEY_SET_VALUE) {
                for path in paths {
                    let _ = key.delete_value(path);
                }
            }
        }
        _ => return Err(format!("Unknown DefenderExclusion action: {}", action)),
    }
    Ok(())
}

pub fn control_defender_services(services: &[String], action: &str) -> Result<(), String> {
    for svc in services {
        match action {
            "disable" => {
                let _ = Command::new("sc")
                    .args(["stop", svc])
                    .creation_flags(0x08000000)
                    .output();
                let out = Command::new("sc")
                    .args(["config", svc, "start=", "disabled"])
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
                let out = Command::new("sc")
                    .args(["config", svc, "start=", "auto"])
                    .creation_flags(0x08000000)
                    .output()
                    .map_err(|e| format!("sc config enable {} failed: {}", svc, e))?;
                if !out.status.success() {
                    eprintln!(
                        "[Defender] Warning: sc config start= auto for {} returned non-zero",
                        svc
                    );
                }
                let _ = Command::new("sc")
                    .args(["start", svc])
                    .creation_flags(0x08000000)
                    .output();
            }
            _ => return Err(format!("Unknown DefenderServiceControl action: {}", action)),
        }
    }
    Ok(())
}
