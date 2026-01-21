use std::process::Command;
use tauri::command;

#[command]
pub async fn create_restore_point(description: String) -> Result<String, String> {
    // Checkpoint-Computer -Description <DESC> -RestorePointType "MODIFY_SETTINGS"
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS'",
                description.replace("'", "''")
            ),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Restore point created successfully.".to_string())
    } else {
        // If it fails (e.g. nested calls, disabled protection), user gets error
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn open_restore_ui() -> Result<(), String> {
    Command::new("rstrui.exe")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
