use crate::modules::startup::types::{AutostartSource, SafetyRating, StartupItem};
use crate::modules::startup::{assess_safety, utils};
use winreg::enums::*;
use winreg::RegKey;

pub fn scan() -> Vec<StartupItem> {
    let mut items = Vec::new();
    items.extend(scan_boot_execute());
    items.extend(scan_appinit());
    items.extend(scan_winlogon());
    items.extend(scan_ifeo()); // Image File Execution Options
    items
}

fn scan_boot_execute() -> Vec<StartupItem> {
    let mut items = Vec::new();
    // HKLM\SYSTEM\CurrentControlSet\Control\Session Manager -> BootExecute (MultiString)
    if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager")
    {
        if let Ok(val) = key.get_value::<Vec<String>, _>("BootExecute") {
            for v in val {
                items.push(StartupItem {
                    id: format!("BOOT:{}", v),
                    name: v.clone(),
                    category: "Boot".to_string(),
                    subcategory: "BootExecute".to_string(),
                    location: "HKLM\\...\\Session Manager".to_string(),
                    command: v,
                    enabled: true,
                    publisher: Some("Microsoft".to_string()),
                    description: Some("Native Boot Program".to_string()),
                    source: AutostartSource::Registry,
                    safety_rating: SafetyRating::Safe, // Usually autochek
                    file_exists: true,
                });
            }
        }
    }
    items
}

fn scan_appinit() -> Vec<StartupItem> {
    let mut items = Vec::new();
    let paths = vec![
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows",
            "AppInit_DLLs",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Wow6432Node\Microsoft\Windows NT\CurrentVersion\Windows",
            "AppInit_DLLs (32-bit)",
        ),
    ];

    for (hive, path, subcat) in paths {
        let root = RegKey::predef(hive);
        if let Ok(key) = root.open_subkey(path) {
            if let Ok(val) = key.get_value::<String, _>("AppInit_DLLs") {
                if !val.is_empty() {
                    // Can be comma or space separated
                    let dlls: Vec<&str> = val.split([',', ' ']).filter(|s| !s.is_empty()).collect();
                    for dll in dlls {
                        let (publ, desc) = utils::get_file_info(dll);
                        items.push(StartupItem {
                            id: format!("APPINIT:{}:{}", subcat, dll),
                            name: dll.to_string(),
                            category: "AppInit".to_string(),
                            subcategory: subcat.to_string(),
                            location: path.to_string(),
                            command: dll.to_string(),
                            enabled: true,
                            publisher: publ,
                            description: desc,
                            source: AutostartSource::Registry,
                            safety_rating: SafetyRating::Unknown, // High risk location
                            file_exists: true,
                        });
                    }
                }
            }
        }
    }
    items
}

fn scan_winlogon() -> Vec<StartupItem> {
    let mut items = Vec::new();
    // Shell and Userinit
    let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon";
    if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(path) {
        for val_name in ["Shell", "Userinit"] {
            if let Ok(val) = key.get_value::<String, _>(val_name) {
                // Shell is normally "explorer.exe", Userinit is "C:\Windows\system32\userinit.exe,"
                // Anything extra is suspicious
                let parts: Vec<&str> = val
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();
                for part in parts {
                    let (publ, desc) = utils::get_file_info(part);
                    let rating = assess_safety(part, publ.as_deref());

                    items.push(StartupItem {
                        id: format!("WINLOGON:{}:{}", val_name, part),
                        name: part.to_string(),
                        category: "Winlogon".to_string(),
                        subcategory: val_name.to_string(),
                        location: path.to_string(),
                        command: part.to_string(),
                        enabled: true,
                        publisher: publ,
                        description: desc,
                        source: AutostartSource::Registry,
                        safety_rating: rating,
                        file_exists: true,
                    });
                }
            }
        }
    }
    items
}

fn scan_ifeo() -> Vec<StartupItem> {
    let mut items = Vec::new();
    let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options";
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = root.open_subkey(path) {
        for key_name in key.enum_keys().flatten() {
            if let Ok(sub) = key.open_subkey(&key_name) {
                if let Ok(debugger) = sub.get_value::<String, _>("Debugger") {
                    // HIJACK detected
                    let (publ, desc) = utils::get_file_info(&debugger);

                    items.push(StartupItem {
                        id: format!("IFEO:{}", key_name),
                        name: key_name.clone(),
                        category: "ImageHijack".to_string(),
                        subcategory: "Debugger Hijack".to_string(),
                        location: format!("{}\\{}", path, key_name),
                        command: debugger.clone(),
                        enabled: true,
                        publisher: publ,
                        description: desc,
                        source: AutostartSource::Registry,
                        safety_rating: SafetyRating::Dangerous, // Usually malicious if not dev tool
                        file_exists: true,
                    });
                }
            }
        }
    }
    items
}
