use anyhow::Result;
use std::path::PathBuf;

// Hardcoded for MVP or fetch using tauri API if wrapped
pub fn get_backup_dir() -> Result<PathBuf> {
    // Ideally we use tauri's path resolver. For now let's ensure it's in AppData
    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("backups");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}

pub fn get_state_path() -> Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("state.json");
    Ok(path)
}
