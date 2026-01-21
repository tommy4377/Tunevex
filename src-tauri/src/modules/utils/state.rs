use super::dirs::get_state_path;
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
    pub fn load(path: PathBuf) -> Result<Self, String> {
        if path.exists() {
            let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let state: AppState = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            Ok(state)
        } else {
            Ok(AppState::default())
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub fn load_state() -> Result<AppState, String> {
    let path = get_state_path()?;
    AppState::load(path)
}

pub fn save_state(state: &AppState) -> Result<(), String> {
    let path = get_state_path()?;
    state.save(path)
}
