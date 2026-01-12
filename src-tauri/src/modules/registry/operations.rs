use crate::modules::types::{TweakType, RegistryValue, TweakOperation};
use anyhow::{Context, Result};
use winreg::enums::*;
use winreg::RegKey;

pub fn get_root_key(root_name: &str) -> RegKey {
    match root_name.to_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => RegKey::predef(HKEY_LOCAL_MACHINE),
        "HKCU" | "HKEY_CURRENT_USER" => RegKey::predef(HKEY_CURRENT_USER),
        "HKCR" | "HKEY_CLASSES_ROOT" => RegKey::predef(HKEY_CLASSES_ROOT),
        "HKU" | "HKEY_USERS" => RegKey::predef(HKEY_USERS),
        "HKCC" | "HKEY_CURRENT_CONFIG" => RegKey::predef(HKEY_CURRENT_CONFIG),
        _ => RegKey::predef(HKEY_CURRENT_USER), // Default to HKCU if unknown
    }
}

pub fn open_subkey(root: &RegKey, path: &str, write: bool) -> Result<RegKey> {
    let access = if write { KEY_ALL_ACCESS } else { KEY_READ };
    root.open_subkey_with_flags(path, access)
        .context(format!("Failed to open subkey: {}", path))
}

pub fn create_subkey(root: &RegKey, path: &str) -> Result<RegKey> {
    let (key, _) = root
        .create_subkey(path)
        .context(format!("Failed to create subkey: {}", path))?;
    Ok(key)
}

pub fn read_value(key: &RegKey, name: &str) -> Result<RegistryValue> {
    // Try reading as varying types. winreg doesn't strictly enforce types on read,
    // but we need to map to our RegistryValue enum.
    // This is a simplification; robust checking would query type first.

    // Attempt String
    if let Ok(s) = key.get_value::<String, _>(name) {
        return Ok(RegistryValue::String(s));
    }
    // Attempt DWORD
    if let Ok(u) = key.get_value::<u32, _>(name) {
        return Ok(RegistryValue::DWord(u));
    }
    // Attempt QWORD
    if let Ok(u) = key.get_value::<u64, _>(name) {
        return Ok(RegistryValue::QWord(u));
    }

    // Fallback or specific error handling needed
    Err(anyhow::anyhow!(
        "Unsupported or missing registry value type for {}",
        name
    ))
}

pub fn write_value(key: &RegKey, name: &str, value: &RegistryValue) -> Result<()> {
    match value {
        RegistryValue::String(s) => key.set_value(name, s)?,
        RegistryValue::DWord(u) => key.set_value(name, u)?,
        RegistryValue::QWord(u) => key.set_value(name, u)?,
        RegistryValue::Binary(b) => key.set_raw_value(
            name,
            &winreg::RegValue {
                vtype: REG_BINARY,
                bytes: b.clone(),
            },
        )?,
        RegistryValue::MultiString(_s) => {
            // Basic implementation for MultiString if supported by wrapper or manual
            // winreg supports MultiString via set_value if type matches?
            // Actually, winreg traits handle Vec<String> as MultiString usually
            // But we need to check trait impl
            // For now, let's treat as error or simple TODO
            // key.set_value(name, s)?
            return Err(anyhow::anyhow!("MultiString not yet fully implemented"));
        }
    };
    Ok(())
}

pub fn apply_registry_tweak(op: &TweakOperation) -> Result<()> {
    match op {
        TweakOperation::RegistrySet {
            root_key,
            path,
            key,
            value,
        } => {
            let root = get_root_key(root_key);
            // Ensure path exists or create it
            let subkey = create_subkey(&root, path)?;
            write_value(&subkey, key, value)?;
            Ok(())
        }
        TweakOperation::RegistryDelete {
            root_key,
            path,
            key,
        } => {
            let root = get_root_key(root_key);
            if let Ok(subkey) = open_subkey(&root, path, true) {
                let _ = subkey.delete_value(key); // Ignore if already missing
            }
            Ok(())
        }
        _ => Ok(()), // Not a registry operation
    }
}
