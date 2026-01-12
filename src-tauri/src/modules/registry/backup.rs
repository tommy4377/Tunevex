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
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistryBackup {
    pub entries: HashMap<String, BackupEntry>, // Keyed by tweak ID
}

impl RegistryBackup {
    pub fn new(_path: PathBuf) -> Self {
        // In this simplified version we don't hold the path in struct,
        // we just load/save to it. Ideally we'd store it.
        RegistryBackup::default()
    }

    pub fn backup_value(&mut self, root: &str, path: &str, key: &str) -> Result<()> {
        use crate::modules::registry::operations::{get_root_key, open_subkey, read_value};

        let root_key = get_root_key(root);
        // Try to read existing value
        let original = if let Ok(subkey) = open_subkey(&root_key, path, false) {
            read_value(&subkey, key).ok()
        } else {
            None
        };

        let id = format!("{}::{}::{}", root, path, key); // Simple unique ID for now

        // Only backup if not exists? Or overwrite?
        // If we want multiple restore points, we need a better ID (e.g. timestamped)
        // For simple TOGGLE logic, we just want the state before we messed with it.
        // If we already have a backup, maybe don't overwrite it (preserve ORIGINAL original state)
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
                let _ = subkey.delete_value(key);
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
