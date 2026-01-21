use std::path::PathBuf;

pub fn get_app_dir() -> Result<PathBuf, String> {
    let app_data = std::env::var("APPDATA")
        .map_err(|_| "APPDATA environment variable not found".to_string())?;

    let tommy_dir = PathBuf::from(app_data).join("Tunevex");

    if !tommy_dir.exists() {
        std::fs::create_dir_all(&tommy_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    Ok(tommy_dir)
}

pub fn get_state_path() -> Result<PathBuf, String> {
    Ok(get_app_dir()?.join("state.json"))
}

pub fn get_backup_dir() -> Result<PathBuf, String> {
    let backup_dir = get_app_dir()?.join("backups");

    if !backup_dir.exists() {
        std::fs::create_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;
    }

    Ok(backup_dir)
}

pub fn get_logs_dir() -> Result<PathBuf, String> {
    let logs_dir = get_app_dir()?.join("logs");

    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir)
            .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    }

    Ok(logs_dir)
}
