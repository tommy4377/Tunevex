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
    let raw = key.get_raw_value(name)?;
    Ok(match raw.vtype {
        REG_SZ => RegistryValue::String(key.get_value(name)?),
        REG_DWORD => RegistryValue::DWord(key.get_value(name)?),
        REG_QWORD => RegistryValue::QWord(key.get_value(name)?),
        REG_MULTI_SZ => RegistryValue::MultiString(key.get_value(name)?),
        REG_BINARY => RegistryValue::Binary(raw.bytes),
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported registry type for {name}; cannot safely back up"
            ))
        }
    })
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
        RegistryValue::MultiString(s) => key.set_value(name, s)?,
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
            match root.open_subkey_with_flags(path, KEY_SET_VALUE | KEY_QUERY_VALUE) {
                Ok(subkey) => {
                    if let Err(e) = subkey.delete_value(key) {
                        if e.kind() != std::io::ErrorKind::NotFound {
                            return Err(e.into());
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
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
