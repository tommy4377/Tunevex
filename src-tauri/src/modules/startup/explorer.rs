use crate::modules::startup::types::{AutostartSource, SafetyRating, StartupItem};
use winreg::enums::*;
use winreg::{RegKey, HKEY};

fn get_explorer_paths() -> Vec<(HKEY, &'static str, &'static str)> {
    vec![
        (
            HKEY_CLASSES_ROOT,
            r"*\ShellEx\ContextMenuHandlers",
            "ContextMenu",
        ),
        (
            HKEY_CLASSES_ROOT,
            r"AllFileSystemObjects\ShellEx\ContextMenuHandlers",
            "ContextMenu",
        ),
        (
            HKEY_CLASSES_ROOT,
            r"Directory\ShellEx\ContextMenuHandlers",
            "ContextMenu",
        ),
        (
            HKEY_CLASSES_ROOT,
            r"Directory\Background\ShellEx\ContextMenuHandlers",
            "ContextMenu",
        ),
        (
            HKEY_CLASSES_ROOT,
            r"Folder\ShellEx\ContextMenuHandlers",
            "ContextMenu",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers",
            "IconOverlay",
        ),
    ]
}

pub fn scan() -> Vec<StartupItem> {
    let paths = get_explorer_paths();
    let mut items = Vec::new();

    for (hive, path, subcat) in &paths {
        let root = RegKey::predef(*hive);
        if let Ok(key) = root.open_subkey(path) {
            for clsid in key.enum_keys().flatten() {
                let display_name = clsid.clone();
                let enabled = !clsid.starts_with('-');
                let clean_clsid = if enabled {
                    clsid.clone()
                } else {
                    clsid[1..].to_string()
                };

                items.push(StartupItem {
                    id: format!("EXP:{}:{}", subcat, clean_clsid),
                    name: display_name,
                    category: "Explorer".to_string(),
                    subcategory: subcat.to_string(),
                    location: format!("{:?}\\{}\\{}", hive, path, clean_clsid),
                    command: format!("CLSID: {}", clean_clsid),
                    enabled,
                    publisher: None,
                    description: None,
                    source: AutostartSource::ShellExtension,
                    safety_rating: SafetyRating::Unknown,
                    file_exists: true,
                });
            }
        }
    }

    items
}

pub fn toggle_explorer_item(id: &str, enable: bool) -> Result<(), String> {
    let parts: Vec<&str> = id
        .strip_prefix("EXP:")
        .unwrap_or(id)
        .splitn(2, ':')
        .collect();
    if parts.len() < 2 {
        return Err("Invalid explorer item ID".to_string());
    }
    let subcat = parts[0];
    let clsid = parts[1];

    let paths = get_explorer_paths();
    let target = paths.iter().find(|(_, _, s)| *s == subcat);

    let (hive, path) = match target {
        Some((h, p, _)) => (*h, *p),
        None => return Err("Unknown explorer subcategory".to_string()),
    };

    let root = RegKey::predef(hive);
    let key = root
        .open_subkey(path)
        .map_err(|e| format!("Failed to open registry key: {}", e))?;

    if enable {
        let disabled_name = format!("-{}", clsid);
        if key.open_subkey(&disabled_name).is_ok() {
            key.rename_subkey(&disabled_name, clsid)
                .map_err(|e| format!("Failed to enable {}: {}", clsid, e))?;
        }
    } else {
        let disabled_name = format!("-{}", clsid);
        if key.open_subkey(clsid).is_ok() {
            key.rename_subkey(clsid, &disabled_name)
                .map_err(|e| format!("Failed to disable {}: {}", clsid, e))?;
        }
    }

    Ok(())
}
