use std::path::PathBuf;

pub fn get_app_dir() -> Result<PathBuf, String> {
    let app_data = std::env::var("APPDATA")
        .map_err(|_| "APPDATA environment variable not found".to_string())?;

    let tommy_dir = PathBuf::from(app_data).join("TommyTweaker");

    // Try to create directory - if it exists with wrong permissions, this will fail
    // Use a more permissive approach: create with full access, ignore if exists
    if !tommy_dir.exists() {
        match std::fs::create_dir_all(&tommy_dir) {
            Ok(_) => {}
            Err(e) if tommy_dir.exists() => {
                // Directory exists but we can't write - try to fix permissions
                // On Windows, this typically means admin created it
                // We'll fall back to a local directory
                let local_dir = std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join("tommy_tweaker_data");
                std::fs::create_dir_all(&local_dir)
                    .map_err(|e| format!("Failed to create local directory: {}", e))?;
                return Ok(local_dir);
            }
            Err(e) => return Err(format!("Failed to create config directory: {}", e)),
        }
    }

    // Test write access
    let test_file = tommy_dir.join(".write_test");
    match std::fs::write(&test_file, "test") {
        Ok(_) => {
            let _ = std::fs::remove_file(&test_file);
            Ok(tommy_dir)
        }
        Err(_) => {
            // No write access, use local fallback
            let local_dir = std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("tommy_tweaker_data");
            std::fs::create_dir_all(&local_dir)
                .map_err(|e| format!("Failed to create local directory: {}", e))?;
            Ok(local_dir)
        }
    }
}

pub fn get_state_path() -> Result<PathBuf, String> {
    Ok(get_app_dir()?.join("state.json"))
}

pub fn get_backup_dir() -> Result<PathBuf, String> {
    let app_dir = get_app_dir()?;
    let backup_dir = app_dir.join("backups");

    if !backup_dir.exists() {
        match std::fs::create_dir_all(&backup_dir) {
            Ok(_) => {}
            Err(e) if backup_dir.exists() => {
                // Admin created it, no write access - use local fallback
                let local_dir = std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join("tommy_tweaker_data")
                    .join("backups");
                std::fs::create_dir_all(&local_dir)
                    .map_err(|e| format!("Failed to create local backup directory: {}", e))?;
                return Ok(local_dir);
            }
            Err(e) => return Err(format!("Failed to create backup directory: {}", e)),
        }
    }

    // Test write access
    let test_file = backup_dir.join(".write_test");
    match std::fs::write(&test_file, "test") {
        Ok(_) => {
            let _ = std::fs::remove_file(&test_file);
            Ok(backup_dir)
        }
        Err(_) => {
            // No write access, use local fallback
            let local_dir = std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("tommy_tweaker_data")
                .join("backups");
            std::fs::create_dir_all(&local_dir)
                .map_err(|e| format!("Failed to create local backup directory: {}", e))?;
            Ok(local_dir)
        }
    }
}

pub fn get_logs_dir() -> Result<PathBuf, String> {
    let app_dir = get_app_dir()?;
    let logs_dir = app_dir.join("logs");

    if !logs_dir.exists() {
        match std::fs::create_dir_all(&logs_dir) {
            Ok(_) => {}
            Err(e) if logs_dir.exists() => {
                let local_dir = std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join("tommy_tweaker_data")
                    .join("logs");
                std::fs::create_dir_all(&local_dir)
                    .map_err(|e| format!("Failed to create local logs directory: {}", e))?;
                return Ok(local_dir);
            }
            Err(e) => return Err(format!("Failed to create logs directory: {}", e)),
        }
    }

    let test_file = logs_dir.join(".write_test");
    match std::fs::write(&test_file, "test") {
        Ok(_) => {
            let _ = std::fs::remove_file(&test_file);
            Ok(logs_dir)
        }
        Err(_) => {
            let local_dir = std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("tommy_tweaker_data")
                .join("logs");
            std::fs::create_dir_all(&local_dir)
                .map_err(|e| format!("Failed to create local logs directory: {}", e))?;
            Ok(local_dir)
        }
    }
}
