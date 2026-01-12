use crate::modules::startup::types::{AutostartSource, SafetyRating, StartupItem};
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

pub fn scan() -> Vec<StartupItem> {
    let mut items = Vec::new();
    items.extend(scan_ie_bho());
    items.extend(scan_chrome_extensions());
    items.extend(scan_edge_extensions());
    items
}

fn scan_ie_bho() -> Vec<StartupItem> {
    let paths = vec![
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Browser Helper Objects",
            "BHO",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Wow6432Node\Microsoft\Windows\CurrentVersion\Explorer\Browser Helper Objects",
            "BHO (32-bit)",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Internet Explorer\Toolbar",
            "Toolbar",
        ),
    ];

    let mut items = Vec::new();

    for (hive, path, subcat) in paths {
        let root = RegKey::predef(hive);
        if let Ok(key) = root.open_subkey(path) {
            for clsid_result in key.enum_keys() {
                if let Ok(clsid) = clsid_result {
                    items.push(StartupItem {
                        id: format!("BROWSER:{}:{}", subcat, clsid),
                        name: clsid.clone(),
                        category: "Browser".to_string(),
                        subcategory: subcat.to_string(),
                        location: format!("{:?}\\{}", hive, path),
                        command: format!("CLSID: {}", clsid),
                        enabled: true,
                        publisher: None,
                        description: None,
                        source: AutostartSource::ShellExtension,
                        safety_rating: SafetyRating::Unknown,
                        file_exists: true,
                    });
                }
            }
        }
    }

    items
}

fn scan_chrome_extensions() -> Vec<StartupItem> {
    let mut items = Vec::new();

    // Chrome extensions are stored in user profile
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let chrome_ext_path =
            PathBuf::from(&local_app_data).join("Google\\Chrome\\User Data\\Default\\Extensions");

        if chrome_ext_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&chrome_ext_path) {
                for entry in entries.flatten() {
                    let ext_id = entry.file_name().to_string_lossy().to_string();
                    if ext_id.len() == 32 && ext_id.chars().all(|c| c.is_ascii_lowercase()) {
                        // Try to get extension name from manifest
                        let name = get_extension_name(&entry.path()).unwrap_or(ext_id.clone());

                        items.push(StartupItem {
                            id: format!("CHROME_EXT:{}", ext_id),
                            name,
                            category: "Browser".to_string(),
                            subcategory: "Chrome Extension".to_string(),
                            location: chrome_ext_path.to_string_lossy().to_string(),
                            command: format!("Extension ID: {}", ext_id),
                            enabled: true,
                            publisher: None,
                            description: Some("Chrome browser extension".to_string()),
                            source: AutostartSource::Browser,
                            safety_rating: SafetyRating::Unknown,
                            file_exists: true,
                        });
                    }
                }
            }
        }
    }

    items
}

fn scan_edge_extensions() -> Vec<StartupItem> {
    let mut items = Vec::new();

    // Edge extensions are stored in user profile
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let edge_ext_path =
            PathBuf::from(&local_app_data).join("Microsoft\\Edge\\User Data\\Default\\Extensions");

        if edge_ext_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&edge_ext_path) {
                for entry in entries.flatten() {
                    let ext_id = entry.file_name().to_string_lossy().to_string();
                    if ext_id.len() == 32 && ext_id.chars().all(|c| c.is_ascii_lowercase()) {
                        // Try to get extension name from manifest
                        let name = get_extension_name(&entry.path()).unwrap_or(ext_id.clone());

                        items.push(StartupItem {
                            id: format!("EDGE_EXT:{}", ext_id),
                            name,
                            category: "Browser".to_string(),
                            subcategory: "Edge Extension".to_string(),
                            location: edge_ext_path.to_string_lossy().to_string(),
                            command: format!("Extension ID: {}", ext_id),
                            enabled: true,
                            publisher: None,
                            description: Some("Microsoft Edge browser extension".to_string()),
                            source: AutostartSource::Browser,
                            safety_rating: SafetyRating::Unknown,
                            file_exists: true,
                        });
                    }
                }
            }
        }
    }

    items
}

fn get_extension_name(ext_path: &std::path::Path) -> Option<String> {
    // Look for manifest.json in version subdirectory
    if let Ok(versions) = std::fs::read_dir(ext_path) {
        for version_dir in versions.flatten() {
            let manifest_path = version_dir.path().join("manifest.json");
            if manifest_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                    // Simple JSON parsing for "name" field
                    if let Some(name_start) = content.find("\"name\"") {
                        let after_name = &content[name_start..];
                        if let Some(colon_pos) = after_name.find(':') {
                            let after_colon = &after_name[colon_pos + 1..];
                            let trimmed = after_colon.trim();
                            if trimmed.starts_with('"') {
                                if let Some(end_quote) = trimmed[1..].find('"') {
                                    let name = &trimmed[1..end_quote + 1];
                                    // Skip __MSG_ localized names
                                    if !name.starts_with("__MSG_") {
                                        return Some(name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
