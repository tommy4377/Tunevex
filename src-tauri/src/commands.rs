use crate::modules::registry::backup::RegistryBackup;
use crate::modules::registry::operations::apply_registry_tweak;
use crate::modules::types::{Tweak, TweakOperation};
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
            if app_state.applied_tweaks.contains(&tweak.id) {
                tweak.enabled = true;
            }
            tweak
        })
        .collect()
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
    let _result = tokio::task::spawn_blocking(move || -> Result<(), String> {
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
                    println!("  -> PowerShell: {}", script);
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
                        .map_err(|e| format!("PowerShell exec failed: {}", e))?;

                    if !output.stdout.is_empty() {
                        println!("    [PS STDOUT] {}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                        eprintln!("    [PS STDERR] {}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        return Err(format!("PowerShell script failed with exit code {:?}", output.status.code()));
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
pub fn undo_tweak(
    id: String,
    ctx: State<Mutex<TweakContext>>,
    state: State<Mutex<AppState>>,
) -> Result<(), String> {
    let context = ctx.lock().unwrap();

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
