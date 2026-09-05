use std::path::PathBuf;

pub fn get_app_dir() -> Result<PathBuf, String> {
    if let Ok(program_data) = std::env::var("PROGRAMDATA") {
        let dir = PathBuf::from(program_data).join("Tunevex");
        if ensure_writable_dir(&dir) {
            return Ok(dir);
        }
    }
    if let Ok(app_data) = std::env::var("APPDATA") {
        let dir = PathBuf::from(app_data).join("Tunevex");
        if ensure_writable_dir(&dir) {
            return Ok(dir);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let dir = parent.join("TunevexData");
            if ensure_writable_dir(&dir) {
                return Ok(dir);
            }
        }
    }
    Err("Cannot find a writable directory for Tunevex data.".to_string())
}

fn ensure_writable_dir(dir: &PathBuf) -> bool {
    if !dir.exists() {
        if std::fs::create_dir_all(dir).is_err() {
            return false;
        }
    }
    let test = dir.join(".writetest");
    let ok = std::fs::write(&test, "ok").is_ok();
    let _ = std::fs::remove_file(&test);
    ok && existing_files_are_writable(dir)
}

fn existing_files_are_writable(dir: &PathBuf) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            return false;
        };
        if file_type.is_symlink() {
            return false;
        }
        if file_type.is_dir() {
            if !existing_files_are_writable(&entry.path()) {
                return false;
            }
        } else if std::fs::OpenOptions::new()
            .write(true)
            .open(entry.path())
            .is_err()
        {
            return false;
        }
    }
    true
}

pub fn get_state_path() -> Result<PathBuf, String> {
    Ok(get_app_dir()?.join("state.json"))
}

pub fn get_backup_dir() -> Result<PathBuf, String> {
    let dir = get_app_dir()?.join("backups");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn get_registry_backup_path() -> Result<PathBuf, String> {
    Ok(get_backup_dir()?.join("registry.json"))
}

pub fn get_logs_dir() -> Result<PathBuf, String> {
    let dir = get_app_dir()?.join("logs");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}
