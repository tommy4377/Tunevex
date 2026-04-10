use crate::modules::startup::types::{AutostartSource, StartupItem};
use crate::modules::startup::{assess_safety, utils};

pub fn scan() -> Vec<StartupItem> {
    // Critical services that should NEVER be disabled
    const CRITICAL_SERVICES: &[&str] = &[
        "RpcSs",
        "WinDefend",
        "Dnscache",
        "Dhcp",
        "TermService",
        "Spooler",
        "AudioSrv",
        "lanmanworkstation",
        "lanmanserver",
        "EventLog",
        "Schedule",
        "SENS",
        "ProfSvc",
        "Themes",
    ];

    // PERFORMANCE FIX: Use winreg directly instead of WMI via PowerShell
    // Registry read is ~1-5ms vs WMI's 500ms-2s
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let services_key = match hklm.open_subkey(r"SYSTEM\CurrentControlSet\Services") {
            Ok(k) => k,
            Err(_) => return vec![],
        };

        // Start values: 0=Boot, 1=System, 2=Auto, 3=Demand, 4=Disabled
        let results: Vec<StartupItem> = services_key
            .enum_keys()
            .flatten()
            .filter_map(|name| {
                let sub = services_key.open_subkey(&name).ok()?;
                let start: u32 = sub.get_value("Start").ok()?;
                if start == 4 {
                    return None;
                } // skip already-disabled

                let type_val: u32 = sub.get_value("Type").unwrap_or(0);
                if type_val == 0 {
                    return None;
                } // skip non-service entries

                let display_name: String = sub
                    .get_value("DisplayName")
                    .unwrap_or_else(|_| name.clone());
                let image_path: String = sub.get_value("ImagePath").unwrap_or_default();
                let description: Option<String> = sub.get_value("Description").ok();

                let start_mode = match start {
                    0 => "Boot",
                    1 => "System",
                    2 => "Auto",
                    3 => "Demand",
                    _ => "Unknown",
                };

                // Verify existence
                let clean_path = utils::sanitize_path(&image_path);
                let exists = std::path::Path::new(&clean_path).exists();

                // Get publisher if possible
                let (publisher, extra_desc) = utils::get_file_info(&image_path);
                let final_desc = description.or(extra_desc);

                // Determine safety rating
                let mut rating = assess_safety(&image_path, publisher.as_deref());

                // Override rating for critical services
                if CRITICAL_SERVICES.contains(&name.as_str()) {
                    use crate::modules::startup::types::SafetyRating;
                    rating = SafetyRating::Critical;
                }

                Some(StartupItem {
                    id: format!("SVC:{}", name),
                    name: display_name,
                    category: "Service".to_string(),
                    subcategory: format!("{} ({})", start_mode, name),
                    location: "HKLM\\SYSTEM\\CurrentControlSet\\Services".to_string(),
                    command: image_path.clone(),
                    enabled: true,
                    publisher,
                    description: final_desc,
                    source: AutostartSource::Service,
                    safety_rating: rating,
                    file_exists: exists,
                })
            })
            .collect();

        return results;
    }

    // Fallback for non-Windows (shouldn't happen for this desktop app)
    #[cfg(not(target_os = "windows"))]
    {
        Vec::new()
    }
}

pub fn toggle_service(id: &str, enable: bool) -> Result<(), String> {
    let service_name = id
        .strip_prefix("SVC:")
        .unwrap_or_else(|| id.strip_prefix("SERVICE:").unwrap_or(id));
    let start_mode = if enable { "Automatic" } else { "Disabled" };

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;

        let stop_cmd = if !enable {
            format!(
                "Stop-Service -Name '{}' -Force -ErrorAction SilentlyContinue; ",
                service_name
            )
        } else {
            String::new()
        };

        let script = format!(
            "{}Set-Service -Name '{}' -StartupType {}",
            stop_cmd, service_name, start_mode
        );

        let output = std::process::Command::new("powershell")
            .args(&["-NoProfile", "-Command", &script])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
    }

    Ok(())
}
