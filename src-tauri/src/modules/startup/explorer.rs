use crate::modules::startup::types::{AutostartSource, SafetyRating, StartupItem};
use winreg::enums::*;
use winreg::RegKey;

pub fn scan() -> Vec<StartupItem> {
    let paths = vec![
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
    ];

    let mut items = Vec::new();

    for (hive, path, subcat) in paths {
        let root = RegKey::predef(hive);
        if let Ok(key) = root.open_subkey(path) {
            for clsid_result in key.enum_keys() {
                if let Ok(clsid) = clsid_result {
                    let display_name = clsid.clone();

                    items.push(StartupItem {
                        id: format!("EXP:{}:{}", subcat, clsid),
                        name: display_name,
                        category: "Explorer".to_string(),
                        subcategory: subcat.to_string(),
                        location: format!("{:?}\\{}\\{}", hive, path, clsid),
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
