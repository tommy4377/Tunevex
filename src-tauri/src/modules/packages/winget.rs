use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WingetPackage {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String, // e.g., "Browser", "Development", "Utility"
    pub installed: bool,
}

#[command]
pub async fn install_package(id: String) -> Result<String, String> {
    // Run winget install command
    // winget install --id <ID> -e --silent --accept-package-agreements --accept-source-agreements
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "winget install --id {} -e --silent --accept-package-agreements --accept-source-agreements",
                id
            ),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn uninstall_package(id: String) -> Result<String, String> {
    // Run winget uninstall command
    // winget uninstall --id <ID> -e --silent
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("winget uninstall --id {} -e --silent", id),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn check_package_status(id: String) -> bool {
    // Check if package is installed via winget list
    // winget list -e --id <ID>
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("winget list -e --id {}", id),
        ])
        .output();

    match output {
        Ok(o) => o.status.success(), // If existing, status is success (exit code 0)
        Err(_) => false,
    }
}
