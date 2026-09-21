use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::modules::types::RegistryValue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupEntry {
    pub root: String,
    pub path: String,
    pub key: String,
    pub original_value: Option<RegistryValue>,
    pub timestamp: u64,
    pub os_build: String, // New: OS Build version
}

fn get_current_build() -> String {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) =
        hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", KEY_READ)
    {
        return key
            .get_value("CurrentBuild")
            .unwrap_or_else(|_| "Unknown".to_string());
    }
    "Unknown".to_string()
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistryBackup {
    pub entries: HashMap<String, BackupEntry>, // Keyed by tweak ID
}

impl RegistryBackup {
    pub fn new(path: PathBuf) -> Self {
        // Preserve previously captured values. Starting from an empty manager
        // here used to overwrite every other tweak's rollback data.
        Self::load(path).unwrap_or_default()
    }

    pub fn backup_value(&mut self, root: &str, path: &str, key: &str) -> Result<()> {
        use crate::modules::registry::operations::{get_root_key, read_value};
        use winreg::enums::KEY_READ;

        let root_key = get_root_key(root);
        // Try to read existing value
        let original = match root_key.open_subkey_with_flags(path, KEY_READ) {
            Ok(subkey) => match subkey.get_raw_value(key) {
                Ok(_) => Some(read_value(&subkey, key)?),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                Err(e) => return Err(e.into()),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e.into()),
        };

        let id = format!("{}::{}::{}", root, path, key);
        let current_build = get_current_build();

        // Keep the first snapshot until a successful revert consumes it.
        if !self.entries.contains_key(&id) {
            self.entries.insert(
                id.clone(),
                BackupEntry {
                    root: root.to_string(),
                    path: path.to_string(),
                    key: key.to_string(),
                    original_value: original,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs(),
                    os_build: current_build,
                },
            );
        }

        Ok(())
    }

    pub fn restore_tweak_backup(&self, root: &str, path: &str, key: &str) -> Result<bool> {
        use crate::modules::registry::operations::{create_subkey, get_root_key, write_value};

        let id = format!("{}::{}::{}", root, path, key);
        if let Some(entry) = self.entries.get(&id) {
            let root_key = get_root_key(root);
            // Ensure key exists
            let subkey = create_subkey(&root_key, path)?;

            if let Some(val) = &entry.original_value {
                write_value(&subkey, key, val)?;
            } else {
                if let Err(e) = subkey.delete_value(key) {
                    if e.kind() != std::io::ErrorKind::NotFound {
                        return Err(e.into());
                    }
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn load(path: PathBuf) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let backup: RegistryBackup = serde_json::from_str(&content)?;
            Ok(backup)
        } else {
            Ok(RegistryBackup::default())
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }
}
