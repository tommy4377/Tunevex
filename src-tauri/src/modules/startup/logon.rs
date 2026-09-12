use crate::modules::startup::types::{AutostartSource, SafetyRating, StartupItem};
use crate::modules::startup::{assess_safety, utils};
use std::path::PathBuf;
use winreg::enums::{HKEY_CURRENT_USER as CK, HKEY_LOCAL_MACHINE as LK, KEY_ALL_ACCESS, KEY_READ};
use winreg::RegKey;

const BACKUP_KEY_PATH: &str = "Software\\Tunevex\\StartupDisabled";
const LEGACY_BACKUP_KEY_PATH: &str = "Software\\TommyTweaker\\StartupDisabled";

pub fn scan() -> Vec<StartupItem> {
    let mut items = Vec::new();
    items.extend(scan_registry());
    items.extend(scan_folders());
    items
}

fn scan_registry() -> Vec<StartupItem> {
    let mut items = Vec::new();

    // (Root, Path, DisplayPrefix, HiveName for ID)
    let locations = vec![
        // Standard Run/RunOnce locations
        (
            CK,
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            "HKCU\\Run",
            "HKCU",
        ),
        (
            CK,
            "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce",
            "HKCU\\RunOnce",
            "HKCU",
        ),
        (
            LK,
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
            "HKLM\\Run",
            "HKLM",
        ),
        (
            LK,
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce",
            "HKLM\\RunOnce",
            "HKLM",
        ),
        (
            LK,
            "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run",
            "HKLM\\Wow6432Node\\Run",
            "HKLM",
        ),
        // RunOnceEx - runs during Safe Mode too
        (
            LK,
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnceEx",
            "HKLM\\RunOnceEx",
            "HKLM",
        ),
        (
            CK,
            "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnceEx",
            "HKCU\\RunOnceEx",
            "HKCU",
        ),
        // Group Policy Run/RunOnce - often used by malware
        (
            LK,
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer\\Run",
            "HKLM\\Policies\\Run",
            "HKLM",
        ),
        (
            CK,
            "Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer\\Run",
            "HKCU\\Policies\\Run",
            "HKCU",
        ),
        // Windows NT legacy locations - still active
        (
            LK,
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon",
            "HKLM\\Winlogon",
            "HKLM",
        ),
    ];

    for (root, path, subcategory, hive_name) in locations {
        // 1. Scan Active Keys
        if let Ok(key) = RegKey::predef(root).open_subkey_with_flags(path, KEY_READ) {
            for (name, value) in key.enum_values().flatten() {
                let command = value.to_string();
                let (publisher, description) = utils::get_file_info(&command);
                let rating = assess_safety(&command, publisher.as_deref());
                let exists = check_file_exists(&command);

                items.push(StartupItem {
                    id: format!("REG:{}:{}:{}", hive_name, path, name),
                    name: name.clone(),
                    category: "Logon".to_string(), // Frontend expects String for now based on types.rs? No types.rs has enum?
                    // Wait, types.rs defined StartupCategory enum. But User prompt used String in their Request ("Logon").
                    // I should align with types.rs. Let's use the enum .to_string() or just modify types.rs to be compatible.
                    // The user's types.rs replacement struct used `category: String`. I followed that.
                    // So I should use String "Logon".
                    subcategory: subcategory.to_string(),
                    location: format!("{}\\{}", hive_name, path),
                    command: command.clone(),
                    enabled: true,
                    publisher,
                    description,
                    source: AutostartSource::Registry,
                    safety_rating: rating,
                    file_exists: exists,
                });
            }
        }

        // 2. Scan Disabled Keys (Backup). Prefer Tunevex but keep reading
        // the pre-rename TommyTweaker namespace for rollback compatibility.
        for backup_root in [BACKUP_KEY_PATH, LEGACY_BACKUP_KEY_PATH] {
            let disabled_path = format!("{}\\{}", backup_root, path.replace("\\", "_"));
            let Ok(key) = RegKey::predef(CK).open_subkey_with_flags(&disabled_path, KEY_READ)
            else {
                continue;
            };
            let mut found = false;
            for (name, value) in key.enum_values().flatten() {
                found = true;
                let command = value.to_string();
                items.push(StartupItem {
                    id: format!("REG:{}:{}:{}", hive_name, path, name),
                    name: name.clone(),
                    category: "Logon".to_string(),
                    subcategory: format!("{} (Disabled)", subcategory),
                    location: format!("(Backup) {}", path),
                    command,
                    enabled: false,
                    publisher: None,
                    description: None,
                    source: AutostartSource::Registry,
                    safety_rating: SafetyRating::Unknown,
                    file_exists: false,
                });
            }
            if found {
                break;
            }
        }
    }

    items
}

fn scan_folders() -> Vec<StartupItem> {
    let mut items = Vec::new();

    let mut paths = Vec::new();
    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push((
            "CurrentUser",
            PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup"),
        ));
    }
    if let Ok(programdata) = std::env::var("ProgramData") {
        paths.push((
            "AllUsers",
            PathBuf::from(programdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup"),
        ));
    }

    for (user_scope, folder) in paths {
        if !folder.exists() {
            continue;
        }

        if let Ok(entries) = std::fs::read_dir(&folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let enabled = !file_name.ends_with(".disabled");

                    let clean_name = if !enabled {
                        file_name.trim_end_matches(".disabled").to_string()
                    } else {
                        file_name.clone()
                    };

                    if clean_name.eq_ignore_ascii_case("desktop.ini") {
                        continue;
                    }

                    let full_path = path.to_string_lossy().to_string();
                    let (publisher, description) = utils::get_file_info(&full_path);
                    let rating = assess_safety(&full_path, publisher.as_deref());

                    items.push(StartupItem {
                        id: format!("FILE:{}", full_path),
                        name: clean_name,
                        category: "Logon".to_string(),
                        subcategory: format!("Startup Folder ({})", user_scope),
                        location: folder.to_string_lossy().to_string(),
                        command: full_path,
                        enabled,
                        publisher,
                        description,
                        source: AutostartSource::FileSystem,
                        safety_rating: rating,
                        file_exists: true, // It came from read_dir
                    });
                }
            }
        }
    }

    items
}

fn check_file_exists(path: &str) -> bool {
    let clean = path.trim_matches('"').split(' ').next().unwrap_or(path);
    std::path::Path::new(clean).exists()
}

pub fn toggle_registry(id: &str, enable: bool) -> Result<(), String> {
    // Re-implemented similarly to before but matching new structure if needed.
    // Logic is same.
    let parts: Vec<&str> = id.splitn(4, ':').collect();
    if parts.len() != 4 {
        return Err("Invalid ID format".to_string());
    }

    let hive_str = parts[1];
    let path = parts[2];
    let name = parts[3];

    let root = match hive_str {
        "HKCU" => CK,
        "HKLM" => LK,
        _ => return Err("Unknown hive".to_string()),
    };

    let backup_subpath = path.replace("\\", "_");
    let backup_full_path = format!("{}\\{}", BACKUP_KEY_PATH, backup_subpath);

    if enable {
        let mut restored: Option<(String, String)> = None;
        for backup_root in [BACKUP_KEY_PATH, LEGACY_BACKUP_KEY_PATH] {
            let candidate = format!("{}\\{}", backup_root, backup_subpath);
            let Ok(backup_key) =
                RegKey::predef(CK).open_subkey_with_flags(&candidate, KEY_ALL_ACCESS)
            else {
                continue;
            };
            if let Ok(value) = backup_key.get_value::<String, _>(name) {
                restored = Some((value, candidate));
                break;
            }
        }
        let (value, source_backup_path) =
            restored.ok_or_else(|| "Could not find disabled item".to_string())?;

        let (target_key, _) = RegKey::predef(root)
            .create_subkey(path)
            .map_err(|e| e.to_string())?;
        target_key
            .set_value(name, &value)
            .map_err(|e| e.to_string())?;
        if let Ok(backup_key) =
            RegKey::predef(CK).open_subkey_with_flags(&source_backup_path, KEY_ALL_ACCESS)
        {
            let _ = backup_key.delete_value(name);
        }
    } else {
        let original_key = RegKey::predef(root)
            .open_subkey_with_flags(path, KEY_READ)
            .map_err(|_| "Could not find enabled item".to_string())?;
        let value: String = original_key.get_value(name).map_err(|e| e.to_string())?;

        let (target_key, _) = RegKey::predef(CK)
            .create_subkey(&backup_full_path)
            .map_err(|e| e.to_string())?;
        target_key
            .set_value(name, &value)
            .map_err(|e| e.to_string())?;

        let write_key = RegKey::predef(root)
            .open_subkey_with_flags(path, KEY_ALL_ACCESS)
            .map_err(|e| format!("Permission denied: {}", e))?;
        write_key.delete_value(name).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn toggle_file(id: &str, enable: bool) -> Result<(), String> {
    let path_str = id.strip_prefix("FILE:").ok_or("Invalid File ID")?;
    let path = PathBuf::from(path_str);

    if enable {
        if path.extension().map_or(false, |ext| ext == "disabled") {
            let new_path = path.with_extension("");
            std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        }
    } else {
        let mut new_name = path.file_name().ok_or("Invalid path")?.to_os_string();
        new_name.push(".disabled");
        let new_path = path.with_file_name(new_name);
        std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
