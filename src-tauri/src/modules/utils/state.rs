use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppState {
    pub applied_tweaks: HashSet<String>, // Set of Tweak IDs
    pub active_profile: Option<String>,
    pub compressed_folders: HashSet<String>,
}

impl AppState {
    pub fn load(path: PathBuf) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let state: AppState = serde_json::from_str(&content)?;
            Ok(state)
        } else {
            Ok(AppState::default())
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
