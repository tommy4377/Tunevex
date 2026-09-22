use std::path::{Path, PathBuf};

pub fn get_app_dir() -> Result<PathBuf, String> {
    let mut targets = Vec::new();
    if let Ok(program_data) = std::env::var("PROGRAMDATA") {
        targets.push(PathBuf::from(program_data).join("Tunevex"));
    }
    if let Ok(app_data) = std::env::var("APPDATA") {
        targets.push(PathBuf::from(app_data).join("Tunevex"));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            targets.push(parent.join("TunevexData"));
        }
    }

    for target in targets {
        if ensure_writable_dir(&target) {
            migrate_legacy_data(&target);
            return Ok(target);
        }
    }

    Err("Cannot find a writable directory for Tunevex data.".to_string())
}

fn legacy_dirs() -> Vec<PathBuf> {
    let mut legacy = Vec::new();

    if let Ok(program_data) = std::env::var("PROGRAMDATA") {
        legacy.push(PathBuf::from(program_data).join("TommyTweaker"));
    }
    if let Ok(app_data) = std::env::var("APPDATA") {
        legacy.push(PathBuf::from(app_data).join("TommyTweaker"));
    }
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        legacy.push(PathBuf::from(local_app_data).join("TommyTweaker"));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            legacy.push(parent.join("TommyTweakerData"));
        }
    }

    legacy
}

fn migrate_legacy_data(target: &Path) {
    for source in legacy_dirs() {
        if source == target || !source.exists() {
            continue;
        }

        match move_or_merge_dir(&source, target) {
            Ok(()) => log::info!(
                "Migrated legacy Tunevex data from {} to {}",
                source.display(),
                target.display()
            ),
            Err(error) => log::warn!(
                "Could not migrate legacy Tunevex data from {}: {}",
                source.display(),
                error
            ),
        }
    }
}

fn move_or_merge_dir(source: &Path, target: &Path) -> Result<(), String> {
    std::fs::create_dir_all(target).map_err(|e| e.to_string())?;

    let entries = std::fs::read_dir(source).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let file_type = entry.file_type().map_err(|e| e.to_string())?;

        // Never follow links while migrating application state.
        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            move_or_merge_dir(&source_path, &target_path)?;
        } else if target_path.exists() {
            // New-name data is authoritative. If the legacy file differs,
            // preserve it next to the canonical file instead of losing it.
            let same = std::fs::read(&source_path).ok() == std::fs::read(&target_path).ok();
            if same {
                let _ = std::fs::remove_file(&source_path);
            } else {
                let backup = unique_legacy_backup_path(&target_path);
                move_file(&source_path, &backup)?;
            }
        } else {
            move_file(&source_path, &target_path)?;
        }
    }

    if source.exists()
        && std::fs::read_dir(source)
            .map_err(|e| e.to_string())?
            .next()
            .is_none()
    {
        let _ = std::fs::remove_dir(source);
    }

    Ok(())
}

fn unique_legacy_backup_path(target: &Path) -> PathBuf {
    let file_name = target
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("legacy-data");

    for index in 0..1000 {
        let suffix = if index == 0 {
            ".legacy-tommytweaker".to_string()
        } else {
            format!(".legacy-tommytweaker-{index}")
        };
        let candidate = target.with_file_name(format!("{file_name}{suffix}"));
        if !candidate.exists() {
            return candidate;
        }
    }

    target.with_file_name(format!("{file_name}.legacy-tommytweaker-backup"))
}

fn move_file(source: &Path, target: &Path) -> Result<(), String> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    if std::fs::rename(source, target).is_ok() {
        return Ok(());
    }

    std::fs::copy(source, target).map_err(|e| e.to_string())?;
    std::fs::remove_file(source).map_err(|e| e.to_string())?;
    Ok(())
}

fn ensure_writable_dir(dir: &Path) -> bool {
    if !dir.exists() && std::fs::create_dir_all(dir).is_err() {
        return false;
    }

    let test = dir.join(".writetest");
    let ok = std::fs::write(&test, "ok").is_ok();
    let _ = std::fs::remove_file(&test);
    ok && existing_files_are_writable(dir)
}

fn existing_files_are_writable(dir: &Path) -> bool {
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
