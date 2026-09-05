//! Per-tweak device rollback, preserving raw values and absence.
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::ErrorKind};
use winreg::{enums::*, RegKey, RegValue};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct RawValue { bytes: Vec<u8>, kind: u32 }

#[derive(Serialize, Deserialize)]
struct SavedValue {
    path: String,
    name: String,
    original: Option<RawValue>,
    applied: RawValue,
}

fn read(key: &RegKey, name: &str) -> Result<Option<RawValue>, String> {
    match key.get_raw_value(name) {
        Ok(v) => Ok(Some(RawValue { bytes: v.bytes, kind: v.vtype as u32 })),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

fn journal_path(owner: &str) -> Result<std::path::PathBuf, String> {
    if !owner.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("Invalid device backup owner".into());
    }
    Ok(crate::modules::utils::dirs::get_backup_dir()?.join(format!("device-{owner}.json")))
}

pub fn set_values(owner: &str, values: Vec<(String, String, RegValue)>) -> Result<(), String> {
    if values.is_empty() { return Err("No supported devices expose this setting".into()); }
    let file = journal_path(owner)?;
    let mut saved: BTreeMap<String, SavedValue> = if file.exists() {
        serde_json::from_slice(&std::fs::read(&file).map_err(|e| e.to_string())?).map_err(|e| format!("Invalid device backup: {e}"))?
    } else { BTreeMap::new() };
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    for (path, name, value) in &values {
        let original = match root.open_subkey(path) {
            Ok(key) => read(&key, name)?,
            Err(e) if e.kind() == ErrorKind::NotFound => None,
            Err(e) => return Err(e.to_string()),
        };
        saved.entry(format!("{path}::{name}")).or_insert(SavedValue {
            path: path.clone(), name: name.clone(), original,
            applied: RawValue { bytes: value.bytes.clone(), kind: value.vtype.clone() as u32 },
        });
    }
    // Persist the entire snapshot before touching any device.
    let temporary = file.with_extension("json.tmp");
    std::fs::write(&temporary, serde_json::to_vec_pretty(&saved).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    std::fs::rename(&temporary, &file).map_err(|e| e.to_string())?;
    for (path, name, value) in values {
        root.create_subkey(&path).map_err(|e| format!("{path}: {e}"))?.0
            .set_raw_value(&name, &value).map_err(|e| format!("{path}\\{name}: {e}"))?;
    }
    Ok(())
}

fn can_restore(current: &Option<RawValue>, saved: &SavedValue) -> bool {
    current.as_ref() == Some(&saved.applied) || current == &saved.original
}

pub fn restore(owner: &str) -> Result<(), String> {
    let file = journal_path(owner)?;
    let saved: BTreeMap<String, SavedValue> = serde_json::from_slice(
        &std::fs::read(&file).map_err(|_| "No saved device values for this tweak; refusing to guess a driver default".to_string())?
    ).map_err(|e| format!("Invalid device backup: {e}"))?;
    if saved.is_empty() { return Err("No saved device values to restore".into()); }
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    // Check every value before restoring: another tweak or driver may own it now.
    for entry in saved.values() {
        let current = match root.open_subkey(&entry.path) {
            Ok(key) => read(&key, &entry.name)?,
            Err(e) if e.kind() == ErrorKind::NotFound && entry.original.is_none() => None,
            Err(e) => return Err(format!("Device unavailable: {}: {e}", entry.path)),
        };
        if !can_restore(&current, entry) {
            return Err(format!("{}\\{} changed since apply; revert the overlapping tweak first", entry.path, entry.name));
        }
    }
    for entry in saved.values() {
        let key = match root.open_subkey_with_flags(&entry.path, KEY_SET_VALUE) {
            Ok(key) => key,
            Err(e) if e.kind() == ErrorKind::NotFound && entry.original.is_none() => continue,
            Err(e) => return Err(e.to_string()),
        };
        if let Some(value) = &entry.original {
            let kind = match value.kind {
                1 => REG_SZ, 2 => REG_EXPAND_SZ, 3 => REG_BINARY, 4 => REG_DWORD,
                7 => REG_MULTI_SZ, 11 => REG_QWORD,
                _ => return Err("Unsupported saved registry type".into()),
            };
            key.set_raw_value(&entry.name, &RegValue { bytes: value.bytes.clone(), vtype: kind }).map_err(|e| e.to_string())?;
        } else if let Err(e) = key.delete_value(&entry.name) {
            if e.kind() != ErrorKind::NotFound { return Err(e.to_string()); }
        }
    }
    std::fs::remove_file(file).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restoration_preserves_absence_and_rejects_overlapping_changes() {
        let applied = RawValue { bytes: vec![1, 0, 0, 0], kind: 4 };
        let saved = SavedValue { path: "".into(), name: "".into(), original: None, applied: applied.clone() };
        assert!(can_restore(&Some(applied), &saved));
        assert!(can_restore(&None, &saved));
        assert!(!can_restore(&Some(RawValue { bytes: vec![3, 0, 0, 0], kind: 4 }), &saved));
        let json = serde_json::to_string(&saved).unwrap();
        assert!(serde_json::from_str::<SavedValue>(&json).unwrap().original.is_none());
    }
}
