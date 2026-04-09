use crate::modules::types::{RegistryValue, TweakOperation};
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

pub fn open_subkey(root: &RegKey, path: &str, access: u32) -> Result<RegKey> {
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
            // Request minimal rights: KEY_SET_VALUE to delete values
            if let Ok(subkey) = open_subkey(&root, path, KEY_SET_VALUE | KEY_QUERY_VALUE) {
                let _ = subkey.delete_value(key); // Ignore if already missing
            }
            Ok(())
        }
        _ => Ok(()), // Not a registry operation
    }
}

pub fn apply_network_interface_tweak(op: &TweakOperation) -> Result<bool> {
    match op {
        TweakOperation::NetworkInterfacesSet { key, value } => {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let interfaces_path =
                "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces";

            let interfaces = hklm
                .open_subkey_with_flags(interfaces_path, KEY_READ)
                .context("Failed to open Tcpip\\Parameters\\Interfaces")?;

            for name in interfaces
                .enum_values()
                .filter_map(|r| r.ok())
                .map(|(n, _)| n)
            {
                if let Ok(subkey) = interfaces.open_subkey_with_flags(&name, KEY_SET_VALUE) {
                    let _ = write_value(&subkey, key, value);
                }
            }
            Ok(true)
        }
        TweakOperation::NetworkInterfacesDelete { key } => {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let interfaces_path =
                "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces";

            if let Ok(interfaces) = hklm.open_subkey_with_flags(interfaces_path, KEY_READ) {
                for name in interfaces
                    .enum_values()
                    .filter_map(|r| r.ok())
                    .map(|(n, _)| n)
                {
                    if let Ok(subkey) = interfaces.open_subkey_with_flags(&name, KEY_SET_VALUE) {
                        let _ = subkey.delete_value(key);
                    }
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn check_network_interfaces(key_name: &str, expected: &RegistryValue) -> Result<bool> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let interfaces_path = "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces";

    let interfaces = hklm
        .open_subkey_with_flags(interfaces_path, KEY_READ)
        .context("Failed to open Tcpip\\Parameters\\Interfaces")?;

    let interface_keys: Vec<String> = interfaces
        .enum_values()
        .filter_map(|r| r.ok())
        .map(|(n, _)| n)
        .collect();

    if interface_keys.is_empty() {
        return Ok(false);
    }

    for name in interface_keys {
        if let Ok(subkey) = interfaces.open_subkey_with_flags(&name, KEY_READ) {
            if let Ok(found) = read_value(&subkey, key_name) {
                if found != *expected {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }
    Ok(true)
}
