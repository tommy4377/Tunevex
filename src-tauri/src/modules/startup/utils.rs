// use serde_json::Value;
// use std::process::Command;

// #[cfg(target_os = "windows")]
// use std::os::windows::process::CommandExt;

pub fn get_file_info(path: &str) -> (Option<String>, Option<String>) {
    let clean_path = path.trim_matches('"').split(' ').next().unwrap_or(path);
    if !std::path::Path::new(clean_path).exists() {
        return (None, None);
    }

    // Optimization: PowerShell is too slow (spawn per file) causing timeouts.
    // For now, return None to ensure UI loads.
    // TODO: Add 'pelite' crate for fast native PE parsing.

    (None, None)
}
