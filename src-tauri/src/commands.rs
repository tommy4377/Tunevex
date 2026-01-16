use crate::modules::registry::backup::RegistryBackup;
use crate::modules::registry::operations::apply_registry_tweak;
use crate::modules::types::{RegistryValue, Tweak, TweakCheck, TweakOperation};
use crate::modules::utils::privileges::is_admin;
use crate::modules::utils::state::AppState;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;
use tauri::{Emitter, State};

// Placeholder for the global tweak registry
pub struct TweakContext {
    pub tweaks: Vec<Tweak>,
}

/// Check if a tweak is currently enabled on the system by evaluating its TweakCheck
fn check_tweak_enabled(check: &TweakCheck) -> bool {
    match check {
        TweakCheck::Registry {
            root_key,
            path,
            key,
            expected_value,
        } => check_registry_value(root_key, path, key, expected_value),
        TweakCheck::Powershell {
            script,
            expected_output,
        } => check_powershell_output(script, expected_output),
    }
}

/// Check if a registry value matches the expected value
#[cfg(target_os = "windows")]
fn check_registry_value(
    root_key: &str,
    path: &str,
    key: &str,
    expected_value: &RegistryValue,
) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkey = match root_key.to_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKCU" | "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKCR" | "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKU" | "HKEY_USERS" => HKEY_USERS,
        _ => return false,
    };

    let regkey = match RegKey::predef(hkey).open_subkey(path) {
        Ok(k) => k,
        Err(_) => return false,
    };

    match expected_value {
        RegistryValue::DWord(expected) => {
            regkey.get_value::<u32, _>(key).map(|v| v == *expected).unwrap_or(false)
        }
        RegistryValue::QWord(expected) => {
            regkey.get_value::<u64, _>(key).map(|v| v == *expected).unwrap_or(false)
        }
        RegistryValue::String(expected) => {
            regkey.get_value::<String, _>(key).map(|v| v == *expected).unwrap_or(false)
        }
        RegistryValue::Binary(expected) => {
            // Use get_raw_value to avoid winreg version conflicts
            regkey.get_raw_value(key)
                .map(|v| v.bytes == *expected)
                .unwrap_or(false)
        }
        RegistryValue::MultiString(_) => {
            // MultiString rarely used for tweak checks, skip for now
            false
        }
    }

}

#[cfg(not(target_os = "windows"))]
fn check_registry_value(
    _root_key: &str,
    _path: &str,
    _key: &str,
    _expected_value: &RegistryValue,
) -> bool {
    false // Registry checks not available on non-Windows
}

/// Check if PowerShell script output matches expected value
#[cfg(target_os = "windows")]
fn check_powershell_output(script: &str, expected_output: &str) -> bool {
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", script])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            let expected = expected_output.trim().to_lowercase();
            stdout == expected
        }
        Err(_) => false,
    }
}

#[cfg(not(target_os = "windows"))]
fn check_powershell_output(_script: &str, _expected_output: &str) -> bool {
    false // PowerShell checks not available on non-Windows
}

#[tauri::command]

pub fn check_is_admin() -> bool {
    is_admin()
}

// Startup Manager Commands
#[tauri::command]
pub async fn scan_startup() -> Result<Vec<crate::modules::startup::types::StartupItem>, String> {
    Ok(crate::modules::startup::scan_all_startup_items())
}

#[tauri::command]
pub async fn set_startup_item_enabled(id: String, enable: bool) -> Result<(), String> {
    crate::modules::startup::toggle_item(id, enable)
}

use crate::modules::network::dns_benchmark::{self, DnsBenchmarkResult};

#[tauri::command]
pub async fn benchmark_dns() -> Result<Vec<DnsBenchmarkResult>, String> {
    Ok(dns_benchmark::run_benchmark())
}

#[tauri::command]
pub async fn apply_dns_server(primary: String, secondary: String) -> Result<(), String> {
    dns_benchmark::apply_dns(primary, secondary)
}

#[tauri::command]
pub fn get_tweaks(ctx: State<Mutex<TweakContext>>, state: State<Mutex<AppState>>) -> Vec<Tweak> {
    let context = ctx.lock().unwrap();
    let app_state = state.lock().unwrap();

    context
        .tweaks
        .iter()
        .map(|t| {
            let mut tweak = t.clone();
            
            // First, check actual system state via TweakCheck if available
            if let Some(ref check) = tweak.check {
                tweak.enabled = check_tweak_enabled(check);
            } else {
                // Fallback to app state for tweaks without explicit checks
                tweak.enabled = app_state.applied_tweaks.contains(&tweak.id);
            }
            
            tweak
        })
        .collect()
}

/// Fast version - returns tweaks immediately without running any checks
/// Used for instant UI loading
#[tauri::command]
pub async fn get_tweaks_fast(ctx: State<'_, Mutex<TweakContext>>, state: State<'_, Mutex<AppState>>) -> Result<Vec<Tweak>, String> {
    let context = ctx.lock().map_err(|e| e.to_string())?;
    let app_state = state.lock().map_err(|e| e.to_string())?;
    
    // Return tweaks with state from app_state (no live checks)
    Ok(context
        .tweaks
        .iter()
        .map(|t| {
            let mut tweak = t.clone();
            // Use cached state only, don't run any checks
            tweak.enabled = app_state.applied_tweaks.contains(&tweak.id);
            tweak
        })
        .collect())
}

/// Check tweaks by category - runs checks in background and emits events
/// Called after UI loads to progressively update tweak states
#[tauri::command]
pub async fn check_category(
    category: String,
    ctx: State<'_, Mutex<TweakContext>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Get tweaks for this category
    let tweaks: Vec<Tweak> = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .filter(|t| format!("{:?}", t.category) == category)
            .cloned()
            .collect()
    };

    // Clone app handle for the spawned task
    let app_clone = app.clone();
    let category_clone = category.clone();
    
    // Run checks in background thread
    tokio::task::spawn_blocking(move || {
        for tweak in tweaks {
            if let Some(ref check) = tweak.check {
                let enabled = check_tweak_enabled(check);
                let _ = app_clone.emit("tweak-check-result", serde_json::json!({
                    "id": tweak.id,
                    "enabled": enabled
                }));
            }
        }
        // Emit completion for this category
        let _ = app_clone.emit("category-check-complete", serde_json::json!({
            "category": category_clone
        }));
    });

    Ok(())
}

#[tauri::command]
pub async fn apply_tweak(
    id: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 1. Find the tweak (Keep lock critical section short)
    let tweak = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or("Tweak ID not found")?
    };

    // Emit progress start
    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id.clone(),
        "status": "applying",
        "progress": 0
    }));

    // 2. Execute operations (Async/Blocking wrapper)
    // We use tokio::spawn_blocking for heavy IO/Process operations to avoid blocking the async runtime
    let tweak_clone = tweak.clone();
    let id_closure = id.clone();
    let app_closure = app.clone();

    let _result = tokio::task::spawn_blocking(move || -> Result<(), String> {
        let id = id_closure;
        let app = app_closure;

        println!("[TWEAK APPLY] Applying ID: {}", tweak_clone.id);
        for (i, op) in tweak_clone.operations.iter().enumerate() {
            println!("[TWEAK APPLY] Operation {}/{}: {:?}", i + 1, tweak_clone.operations.len(), op);
            match op {
                TweakOperation::RegistrySet {
                    root_key,
                    path,
                    key,
                    value,
                } => {
                    println!("  -> RegistrySet: {}\\{}\\{} = {:?}", root_key, path, key, value);
                    // Perform BACKUP before applying
                    let backup_path = crate::modules::utils::dirs::get_backup_dir()
                        .unwrap_or_else(|_| std::path::PathBuf::from("backups"));
                    let mut backup_mgr = RegistryBackup::new(backup_path);

                    if let Err(e) = backup_mgr.backup_value(root_key, path, key) {
                        eprintln!("  -> Backup warning for {}: {}", key, e);
                    }

                    if let Err(e) = apply_registry_tweak(op) {
                       eprintln!("  -> Registry Error: {:?}", e);
                       return Err(format!("Registry error: {:?}", e));
                    }
                    println!("  -> Registry Set Success");
                }
                TweakOperation::RegistryDelete {
                    root_key,
                    path,
                    key,
                } => {
                    println!("  -> RegistryDelete: {}\\{}\\{}", root_key, path, key);
                    let backup_path = crate::modules::utils::dirs::get_backup_dir()
                        .unwrap_or_else(|_| std::path::PathBuf::from("backups"));
                    let mut backup_mgr = RegistryBackup::new(backup_path);

                    if let Err(e) = backup_mgr.backup_value(root_key, path, key) {
                        eprintln!("  -> Backup warning for {}: {}", key, e);
                    }

                    if let Err(e) = apply_registry_tweak(op) {
                       eprintln!("  -> Registry Error: {:?}", e);
                       return Err(format!("Registry error: {:?}", e));
                    }
                    println!("  -> Registry Delete Success");
                }
                TweakOperation::Command { cmd, args } => {
                    println!("  -> Command: {} {:?}", cmd, args);
                    let output = Command::new(cmd)
                        .args(args)
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Command exec failed: {}", e))?;

                    if !output.stdout.is_empty() {
                        println!("    [STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                        eprintln!("    [STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        return Err(format!("Command returned non-zero code: {:?}", output.status.code()));
                    }
                }
                TweakOperation::Powershell { script } => {
                    let id_closure_log = id.clone();
                    let app_log = app.clone();
                    // Helper to emit logs
                    let log = move |msg: String| {
                        println!("    [PS STREAM] {}", msg);
                        let _ = app_log.emit("tweak-output", serde_json::json!({
                            "id": id_closure_log,
                            "type": "stdout",
                            "line": msg
                        }));
                    };

                    log(format!("Executing Script..."));

                    use std::io::{BufRead, BufReader};
                    use std::process::Stdio;

                    let app_handle = app.clone();
                    let id_clone = id.clone();

                    let mut child = Command::new("powershell")
                        .args(&[
                            "-NoProfile",
                            "-ExecutionPolicy",
                            "Bypass",
                            "-Command",
                            // Add output flush to ensure streaming works better
                            &format!("$OutputEncoding = [Console]::OutputEncoding = [System.Text.Encoding]::UTF8; {} ; [Console]::Out.Flush()", script),
                        ])
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .map_err(|e| format!("PowerShell spawn failed: {}", e))?;

                    // Register PID
                    let pid = child.id();
                    {
                        use tauri::Manager;
                        if let Some(state) = app_handle.try_state::<Mutex<crate::modules::utils::process_manager::ProcessManager>>() {
                             let mut mgr = state.lock().unwrap();
                             mgr.register(id_clone.clone(), pid);
                        }
                    }

                    // Handle stdout streaming
                    if let Some(stdout) = child.stdout.take() {
                        let app_handle = app_handle.clone();
                        let id_clone = id_clone.clone();
                        // We are already in spawn_blocking, so we can block on reading
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            match line {
                                Ok(l) => {
                                    println!("    [PS STREAM] {}", l);
                                    let _ = app_handle.emit("tweak-output", serde_json::json!({
                                        "id": id_clone,
                                        "type": "stdout",
                                        "line": l
                                    }));
                                }
                                Err(e) => eprintln!("Error reading stdout: {}", e),
                            }
                        }
                    }

                    // Wait for completion
                    let output = child.wait_with_output().map_err(|e| format!("Wait failed: {}", e))?;

                    // Unregister PID
                    {
                        use tauri::Manager;
                        if let Some(state) = app_handle.try_state::<Mutex<crate::modules::utils::process_manager::ProcessManager>>() {
                             let mut mgr = state.lock().unwrap();
                             mgr.unregister(&id_clone);
                        }
                    }

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                         let _ = app_handle.emit("tweak-output", serde_json::json!({
                                    "id": id_clone,
                                    "type": "stderr",
                                    "line": stderr
                                }));
                        return Err(format!("PowerShell script failed with exit code {:?}", output.status.code()));
                    } else {
                        let _ = app_handle.emit("tweak-output", serde_json::json!({
                            "id": id_clone,
                            "type": "stdout",
                            "line": "Process finished successfully."
                        }));
                    }
                }
                TweakOperation::ServiceDisable { name } => {
                    println!("  -> ServiceDisable: {}", name);
                    let output = Command::new("powershell")
                        .args(&["-Command", &format!("Stop-Service -Name '{}' -Force -ErrorAction SilentlyContinue; Set-Service -Name '{}' -StartupType Disabled", name, name)])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to disable service {}: {}", name, e))?;

                    if !output.stdout.is_empty() {
                         println!("    [SVC STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                         eprintln!("    [SVC STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        eprintln!("Warning: Failed to disable service {}", name);
                    }
                }
                TweakOperation::ServiceSetMode { name, mode } => {
                    println!("  -> ServiceSetMode: {} -> {}", name, mode);
                    let output = Command::new("powershell")
                        .args(&[
                            "-Command",
                            &format!("Set-Service -Name '{}' -StartupType {}", name, mode),
                        ])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to set service mode {}: {}", name, e))?;

                    if !output.stdout.is_empty() {
                         println!("    [SVC STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                         eprintln!("    [SVC STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }
                    
                    if !output.status.success() {
                        eprintln!("Warning: Failed to set service mode {}", name);
                    }
                }
                TweakOperation::ScheduledTaskDisable { path: _path, name } => {
                    println!("  -> ScheduledTaskDisable: {}", name);
                    let output = Command::new("powershell")
                        .args(&[
                            "-Command",
                            &format!(
                                "Disable-ScheduledTask -TaskName '{}' -ErrorAction SilentlyContinue",
                                name
                            ),
                        ])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to disable task {}: {}", name, e))?;

                    if !output.stdout.is_empty() {
                         println!("    [TASK STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                         eprintln!("    [TASK STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        eprintln!("Warning: Failed to disable task {}", name);
                    }
                }
                TweakOperation::FileOperation(_) => {
                    eprintln!("Warning: FileOperation skipped (not implemented)");
                }
            }
        }
        Ok(())
    }).await.map_err(|e| e.to_string())??;

    // 3. Update State & Persist
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.applied_tweaks.insert(id.clone());

        let state_path = crate::modules::utils::dirs::get_state_path()
            .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));

        if let Err(e) = app_state.save(state_path) {
            eprintln!("Failed to save app state: {}", e);
        }
    }

    // Emit success
    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id,
        "status": "success",
        "progress": 100
    }));

    Ok(())
}

#[tauri::command]
pub fn kill_tweak_process(
    id: String,
    proc_mgr: State<Mutex<crate::modules::utils::process_manager::ProcessManager>>,
) -> Result<(), String> {
    
    // We need to look up the PID
    let pid = {
        let mgr = proc_mgr.lock().unwrap();
        mgr.get_pid(&id)
    };

    if let Some(pid) = pid {
        println!("[PROCESS KILL] Killing process PID {} for tweak {}", pid, id);
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("taskkill")
                .args(&["/F", "/PID", &pid.to_string(), "/T"]) // /T kills child processes (important for powershell -> script)
                .creation_flags(0x08000000)
                .output();
        }
        #[cfg(not(target_os = "windows"))]
        {
             let _ = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output();
        }
    } else {
        println!("[PROCESS KILL] No active PID found for tweak {}", id);
    }
    
    Ok(())
}

#[tauri::command]
pub fn undo_tweak(
    id: String,
    ctx: State<Mutex<TweakContext>>,
    state: State<Mutex<AppState>>,
) -> Result<(), String> {
    let context = ctx.lock().unwrap();
    // ... existing implementation ...
    // Note: Since I can't fit the entire undo_tweak implementation here due to size limits, 
    // I am only replacing the end of apply_tweak and adding kill_tweak_process.
    // The user should ensure undo_tweak remains if outside the replace block.
    // Wait, the REPLACE block targets EndLine: 605 which IS the end of the file.
    // So I need to include undo_tweak fully.
    
    let tweak = context
        .tweaks
        .iter()
        .find(|t| t.id == id)
        .ok_or("Tweak ID not found")?;

    let backup_path = crate::modules::utils::dirs::get_backup_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("backups"));

    // Try to load backup manager for registry operations
    let backup_mgr = RegistryBackup::load(backup_path.clone()).ok();

    // First, restore registry values from backup
    for op in &tweak.operations {
        match op {
            TweakOperation::RegistrySet {
                root_key,
                path,
                key,
                ..
            }
            | TweakOperation::RegistryDelete {
                root_key,
                path,
                key,
            } => {
                if let Some(ref mgr) = backup_mgr {
                    let _ = mgr.restore_tweak_backup(root_key, path, key);
                }
            }
            _ => {}
        }
    }

    // Then, execute explicit revert operations (for PowerShell, Services, etc.)

    println!("[TWEAK REVERT] Reverting ID: {}", id);
    if let Some(ref revert_ops) = tweak.revert_operations {
        for (i, op) in revert_ops.iter().enumerate() {
            println!("[TWEAK REVERT] Operation {}/{}: {:?}", i + 1, revert_ops.len(), op);
            match op {
                TweakOperation::RegistrySet { .. } | TweakOperation::RegistryDelete { .. } => {
                     println!("  -> Registry Operation (Revert): {:?}", op);
                     apply_registry_tweak(op).map_err(|e| format!("Revert registry error: {:?}", e))?;
                     println!("  -> Revert Registry Success");
                }
                TweakOperation::Powershell { script } => {
                    println!("  -> Revert PowerShell: {}", script);
                    let output = Command::new("powershell")
                        .args(&[
                            "-NoProfile",
                            "-ExecutionPolicy",
                            "Bypass",
                            "-Command",
                            script,
                        ])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Revert PowerShell failed: {}", e))?;

                    if !output.stdout.is_empty() {
                         println!("    [REVERT STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                         eprintln!("    [REVERT STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        eprintln!("Warning: Revert PowerShell script returned non-zero");
                    }
                }
                TweakOperation::ServiceSetMode { name, mode } => {
                    println!("  -> Revert ServiceSetMode: {} -> {}", name, mode);
                    let output = Command::new("powershell")
                        .args(&[
                            "-Command",
                            &format!("Set-Service -Name '{}' -StartupType {}", name, mode),
                        ])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Revert service failed: {}", e))?;
                    
                    if !output.stdout.is_empty() {
                         println!("    [REVERT SVC STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                         eprintln!("    [REVERT SVC STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        eprintln!("Warning: Failed to revert service mode {}", name);
                    }
                }
                _ => {
                    println!("  -> Skipped unknown revert op: {:?}", op);
                    // Other operations not yet supported for revert
                }
            }
        }
    }

    // Update State (Remove from applied)
    {
        let mut app_state = state.lock().unwrap();
        if app_state.applied_tweaks.remove(&id) {
            let state_path = crate::modules::utils::dirs::get_state_path()
                .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));
            let _ = app_state.save(state_path);
        }
    }

    Ok(())
}
