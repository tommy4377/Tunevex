use crate::modules::startup::types::{AutostartSource, StartupItem};
use crate::modules::startup::{assess_safety, utils};
use serde_json::Value;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

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

    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            r#"Get-WmiObject Win32_Service | Where-Object { $_.StartMode -in @('Auto','Boot','System') } | Select-Object Name, DisplayName, PathName, Description, StartMode, State | ConvertTo-Json -Compress"#
        ])
        .creation_flags(0x08000000)
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::default(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });

    let json = String::from_utf8_lossy(&output.stdout);

    // Handle array vs single vs empty
    let services: Vec<Value> = if let Ok(arr) = serde_json::from_str::<Vec<Value>>(&json) {
        arr
    } else if let Ok(obj) = serde_json::from_str::<Value>(&json) {
        vec![obj]
    } else {
        Vec::new()
    };

    services
        .into_iter()
        .filter_map(|s| {
            let name = s["Name"].as_str()?.to_string();
            let display_name = s["DisplayName"].as_str().unwrap_or(&name).to_string();
            let path = s["PathName"].as_str().unwrap_or("").to_string();
            let desc = s["Description"].as_str().map(|s| s.to_string());
            let start_mode = s["StartMode"].as_str()?;

            // Verify existence using sanitized path logic
            let clean_path = utils::sanitize_path(&path);
            let exists = std::path::Path::new(&clean_path).exists();

            // Get publisher if possible
            let (publisher, extra_desc) = utils::get_file_info(&path);
            let final_desc = desc.or(extra_desc);

            // Determine safety rating
            let mut rating = assess_safety(&path, publisher.as_deref());

            // Override rating for critical services whitelist
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
                command: path.clone(),
                enabled: start_mode != "Disabled",
                publisher,
                description: final_desc,
                source: AutostartSource::Service,
                safety_rating: rating,
                file_exists: exists,
            })
        })
        .collect()
}

pub fn toggle_service(id: &str, enable: bool) -> Result<(), String> {
    let service_name = id
        .strip_prefix("SVC:")
        .unwrap_or_else(|| id.strip_prefix("SERVICE:").unwrap_or(id));
    let start_mode = if enable { "Automatic" } else { "Disabled" };

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

    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &script])
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}
