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

pub fn toggle_boot_item(id: &str, enable: bool) -> Result<(), String> {
    if id.starts_with("IFEO:") {
        toggle_ifeo_item(id, enable)
    } else if id.starts_with("APPINIT:") {
        toggle_appinit_item(id, enable)
    } else if id.starts_with("BOOT:") {
        toggle_bootexec_item(id, enable)
    } else {
        Err(format!("Unknown boot item type: {}", id))
    }
}

fn toggle_ifeo_item(id: &str, enable: bool) -> Result<(), String> {
    let exe_name = id.strip_prefix("IFEO:").ok_or("Invalid IFEO ID format")?;

    let path = format!(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{}",
        exe_name
    );

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm
        .open_subkey_with_flags(&path, KEY_ALL_ACCESS)
        .map_err(|e| format!("Cannot open IFEO key: {}", e))?;

    if enable {
        // Re-enabling IFEO is dangerous - refuse
        return Err("Cannot re-enable IFEO debugger hijacks - too dangerous. \
                    If this was legitimate software, reinstall it."
            .to_string());
    }

    // Backup before deletion
    if let Ok(debugger) = key.get_value::<String, _>("Debugger") {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let backup_path = format!(r"Software\TommyTweaker\Backups\IFEO\{}", exe_name);
        if let Ok((backup_key, _)) = hkcu.create_subkey(&backup_path) {
            let _ = backup_key.set_value("Debugger", &debugger);
        }
    }

    // Delete the debugger value
    key.delete_value("Debugger")
        .map_err(|e| format!("Cannot remove debugger: {}", e))?;

    Ok(())
}

fn toggle_appinit_item(id: &str, enable: bool) -> Result<(), String> {
    let is_32bit = id.contains("(32-bit)");
    let path = if is_32bit {
        r"SOFTWARE\Wow6432Node\Microsoft\Windows NT\CurrentVersion\Windows"
    } else {
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows"
    };

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey_with_flags(path, KEY_ALL_ACCESS) {
        if enable {
            return Err("Cannot re-enable AppInit_DLLs - too dangerous".to_string());
        }

        // Backup
        if let Ok(current) = key.get_value::<String, _>("AppInit_DLLs") {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let backup_path = if is_32bit {
                r"Software\TommyTweaker\Backups\AppInit32"
            } else {
                r"Software\TommyTweaker\Backups\AppInit64"
            };
            if let Ok((backup_key, _)) = hkcu.create_subkey(backup_path) {
                let _ = backup_key.set_value("AppInit_DLLs", &current);
            }
        }

        // Clear
        key.set_value("AppInit_DLLs", &"")
            .map_err(|e| e.to_string())?;
        let _ = key.set_value("LoadAppInit_DLLs", &0u32); // Disable loading
        Ok(())
    } else {
        Err("Could not open AppInit registry key".to_string())
    }
}

fn toggle_bootexec_item(_id: &str, _enable: bool) -> Result<(), String> {
    Err("BootExecute items cannot be toggled safely. \
         Use 'msconfig' or 'autoruns' for manual editing."
        .to_string())
}
