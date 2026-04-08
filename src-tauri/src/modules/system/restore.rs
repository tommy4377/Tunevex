use tauri::command;
use std::process::Command;

#[command]
pub async fn create_restore_point(description: String) -> Result<String, String> {
    let desc = description.replace("'", "''");
    
    tokio::task::spawn_blocking(move || {
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS'",
                    desc
                ),
            ])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok("Restore point created successfully.".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn open_restore_ui() -> Result<(), String> {
    Command::new("rstrui.exe")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
