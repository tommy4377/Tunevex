use crate::modules::startup::types::{AutostartSource, SafetyRating, StartupItem};
use winreg::enums::*;
use winreg::RegKey;

pub fn scan() -> Vec<StartupItem> {
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
