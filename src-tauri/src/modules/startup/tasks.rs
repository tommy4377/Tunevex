// src-tauri/src/modules/startup/tasks.rs
//
// BUG-M3 FIX: replaced PowerShell Get-ScheduledTask (~500ms) with:
//   1. winreg TaskCache\Tasks  → task list + commands  (~1-5ms)
//   2. C:\Windows\System32\Tasks\<path> XML file read → enabled state (~1ms each)
//   3. schtasks /Change (already whitelisted)         → toggle

use crate::modules::startup::types::{AutostartSource, StartupItem};
use crate::modules::startup::{assess_safety, utils};
use winreg::{enums::*, RegKey};

const TASK_CACHE_TASKS: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tasks";
const TASK_CACHE_TREE: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tree";

// ─── Public API ─────────────────────────────────────────────────────────────

pub fn scan() -> Vec<StartupItem> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // Build GUID → full task path map from Tree (fast recursive walk)
    let path_map = build_path_map(&hklm);

    let tasks_key = match hklm.open_subkey(TASK_CACHE_TASKS) {
        Ok(k) => k,
        Err(_) => return vec![],
    };

    tasks_key
        .enum_keys()
        .filter_map(|r| r.ok())
        .filter_map(|guid| {
            let sub = tasks_key.open_subkey(&guid).ok()?;

            // Full path like "\Microsoft\Windows\TaskName"
            let task_path = path_map
                .get(&guid)
                .cloned()
                .unwrap_or_else(|| format!("\\{}", guid));

            // Extract the executable command from the Actions binary blob
            let command = extract_command(&sub).unwrap_or_default();
            if command.is_empty() {
                return None;
            }

            // Read enabled state from the canonical XML file — most reliable source
            let enabled = is_task_enabled(&task_path);

            let name = task_path
                .split('\\')
                .filter(|s| !s.is_empty())
                .last()
                .unwrap_or(&task_path)
                .to_string();

            let triggers = infer_triggers(&task_path);
            let (publisher, description) = utils::get_file_info(&command);
            let mut rating = assess_safety(&command, publisher.as_deref());

            // Mark Windows system tasks as Critical
            let task_lower = task_path.to_lowercase();
            if task_lower.starts_with(r"\microsoft\windows\") {
                use crate::modules::startup::types::SafetyRating;
                rating = SafetyRating::Critical;
            }

            let first_token = command
                .trim_matches('"')
                .split_whitespace()
                .next()
                .unwrap_or(&command);
            let file_exists = std::path::Path::new(first_token).exists();

            Some(StartupItem {
                id: format!("TASK|{}", task_path),
                name,
                category: "ScheduledTask".to_string(),
                subcategory: triggers,
                location: "Task Scheduler".to_string(),
                command,
                enabled,
                publisher,
                description,
                source: AutostartSource::TaskScheduler,
                safety_rating: rating,
                file_exists,
            })
        })
        .collect()
}

/// Toggle a scheduled task using schtasks.exe (already in ALLOWED_COMMANDS whitelist).
/// ID format: `"TASK|\\Full\\Task\\Path"`
#[cfg(target_os = "windows")]
pub fn toggle_task(id: &str, enable: bool) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    let task_path = id.strip_prefix("TASK|").ok_or("Invalid Task ID")?;
    let flag = if enable { "/ENABLE" } else { "/DISABLE" };

    let output = Command::new("schtasks")
        .args(["/Change", "/TN", task_path, flag])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .map_err(|e| format!("schtasks spawn failed: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(())
}

// ─── Registry helpers ────────────────────────────────────────────────────────

/// Recursively walks TaskCache\Tree and builds a map of GUID → full task path.
fn build_path_map(hklm: &RegKey) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if let Ok(tree) = hklm.open_subkey(TASK_CACHE_TREE) {
        walk_tree(&tree, "\\", &mut map);
    }
    map
}

fn walk_tree(
    key: &RegKey,
    current_path: &str,
    map: &mut std::collections::HashMap<String, String>,
) {
    // Each leaf node has an "Id" value containing the GUID
    if let Ok(id) = key.get_value::<String, _>("Id") {
        map.insert(id, current_path.to_string());
    }
    for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
        if let Ok(sub) = key.open_subkey(&subkey_name) {
            let child_path = if current_path == "\\" {
                format!("\\{}", subkey_name)
            } else {
                format!("{}\\{}", current_path, subkey_name)
            };
            walk_tree(&sub, &child_path, map);
        }
    }
}

// ─── Task XML helpers ────────────────────────────────────────────────────────

/// Checks the canonical task XML file in C:\Windows\System32\Tasks\.
/// Returns `false` only when `<Enabled>false</Enabled>` is explicitly present.
/// Falls back to `true` (enabled) if the file is missing or unreadable.
fn is_task_enabled(task_path: &str) -> bool {
    let clean = task_path.trim_start_matches('\\');
    // task_path uses backslashes, PathBuf::join handles them correctly on Windows
    let xml_path = std::path::PathBuf::from(r"C:\Windows\System32\Tasks").join(clean);
    match std::fs::read_to_string(&xml_path) {
        Ok(content) => {
            // The XML is case-insensitive in practice
            !content.to_lowercase().contains("<enabled>false</enabled>")
        }
        // File missing = task may have been registered only in registry, assume enabled
        Err(_) => true,
    }
}

/// Decodes the Actions binary blob from the TaskCache\Tasks\{GUID} registry key.
///
/// Format: 2-byte header + UTF-16LE encoded XML.
/// We extract the first `<Command>` element which holds the executable path.
fn extract_command(sub: &RegKey) -> Option<String> {
    let raw = sub.get_raw_value("Actions").ok()?;
    let bytes = &raw.bytes;
    if bytes.len() < 4 {
        return None;
    }
    // Skip 2-byte version header, decode remaining bytes as UTF-16LE
    let utf16: Vec<u16> = bytes[2..]
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    let xml = String::from_utf16_lossy(&utf16);

    // Extract <Command>...</Command> (the executable path)
    let tag = "<Command>";
    let end_tag = "</Command>";
    let start = xml.find(tag)? + tag.len();
    let end = xml[start..].find(end_tag)?;
    let cmd = xml[start..start + end].trim().to_string();

    if cmd.is_empty() {
        None
    } else {
        Some(cmd)
    }
}

// ─── Utility ─────────────────────────────────────────────────────────────────

/// Heuristic trigger label derived from the task's full path.
fn infer_triggers(path: &str) -> String {
    let lower = path.to_lowercase();
    if lower.contains("logon") || lower.contains("login") {
        "Logon".to_string()
    } else if lower.contains("startup") || lower.contains("boot") {
        "Boot".to_string()
    } else if lower.contains("idle") {
        "Idle".to_string()
    } else if lower.contains("maintenance") {
        "Maintenance".to_string()
    } else {
        "Scheduled".to_string()
    }
}
