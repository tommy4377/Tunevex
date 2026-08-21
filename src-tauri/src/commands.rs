use crate::modules::ai::memory::{
    delete_chat_session, list_chat_sessions, load_chat_session, save_chat_session,
    validate_session_id, AiMemoryStore, ChatMessage as AiChatMessage, ChatSession, ChatSessionMeta,
    MemoryKind,
};
use crate::modules::ai::startup::{scan_startup_with_ai, StartupRecommendation};
use crate::modules::registry::backup::RegistryBackup;
use crate::modules::registry::operations::apply_registry_tweak;
use crate::modules::startup::toggle_item;
use crate::modules::startup::types::StartupItem;
use crate::modules::tweaks::{
    apply_msi_remove, apply_msi_set, apply_svc_host_split_all, check_tweak_enabled,
    control_defender_services, get_current_windows_build, reset_dns_servers,
    set_defender_exclusions, set_dns_servers, set_nic_property, win11_only_tweaks, TweakContext,
};
use crate::modules::types::{Tweak, TweakOperation, WarningLevel};
use crate::modules::utils::privileges::is_admin;
use crate::modules::utils::security::{safe_path_existing, safe_path_new, validate_command};
use crate::modules::utils::state::AppState;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;
use strip_ansi_escapes::strip_str;
use tauri::{Emitter, State};
#[tauri::command]

pub fn check_is_admin() -> bool {
    is_admin()
}

// Startup Manager Commands
#[tauri::command]
pub async fn scan_startup() -> Result<Vec<crate::modules::startup::types::StartupItem>, String> {
    tokio::task::spawn_blocking(crate::modules::startup::scan_all_startup_items)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_startup_item_enabled(id: String, enable: bool) -> Result<(), String> {
    let id_clone = id.clone();
    tokio::task::spawn_blocking(move || crate::modules::startup::toggle_item(id_clone, enable))
        .await
        .map_err(|e| e.to_string())?
}

use crate::modules::network::dns_benchmark::{self, DnsBenchmarkResult};

#[tauri::command]
pub async fn benchmark_dns() -> Result<Vec<DnsBenchmarkResult>, String> {
    tokio::task::spawn_blocking(dns_benchmark::run_benchmark)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn apply_dns_server(primary: String, secondary: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || dns_benchmark::apply_dns(primary, secondary))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_tweaks(
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Tweak>, String> {
    // Extract data and immediately drop the locks
    let tweaks = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context.tweaks.clone()
    };
    let applied = {
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.applied_tweaks.clone()
    };

    let build = get_current_windows_build();
    let min_builds = win11_only_tweaks();

    // Move heavy I/O (registry, sc, schtasks) to blocking thread pool
    tokio::task::spawn_blocking(move || {
        tweaks
            .iter()
            .filter(|t| {
                min_builds
                    .get(t.id.as_str())
                    .map_or(true, |&min| build >= min)
            })
            .map(|t| {
                let mut tweak = t.clone();
                // First, check actual system state via TweakCheck if available
                if let Some(ref check) = tweak.check {
                    tweak.enabled = check_tweak_enabled(check);
                } else {
                    // Fallback to app state for tweaks without explicit checks
                    tweak.enabled = applied.contains(&tweak.id);
                }
                tweak
            })
            .collect::<Vec<_>>()
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))
}

/// Fast version - returns tweaks immediately without running any checks
/// Used for instant UI loading
#[tauri::command]
pub async fn get_tweaks_fast(
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Tweak>, String> {
    // BUG-M1 fix: acquire, clone, drop each lock before acquiring the next
    let tweaks = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context.tweaks.clone()
    }; // ctx guard dropped here

    let applied = {
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.applied_tweaks.clone()
    }; // state guard dropped here

    let build = get_current_windows_build();
    let min_builds = win11_only_tweaks();

    Ok(tweaks
        .into_iter()
        .filter(|t| {
            min_builds
                .get(t.id.as_str())
                .map_or(true, |&min| build >= min)
        })
        .map(|mut t| {
            t.enabled = applied.contains(&t.id);
            t
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

    // Run checks in background thread - SEQUENTIAL (no rayon)
    tokio::task::spawn_blocking(move || {
        // Sequential execution - avoids thread pool saturation
        let results: Vec<(String, bool)> = tweaks
            .iter()
            .filter_map(|tweak| {
                tweak
                    .check
                    .as_ref()
                    .map(|check| (tweak.id.clone(), check_tweak_enabled(check)))
            })
            .collect();

        // Emit results sequentially (order doesn't matter for UI)
        for (id, enabled) in results {
            let _ = app_clone.emit(
                "tweak-check-result",
                serde_json::json!({
                    "id": id,
                    "enabled": enabled
                }),
            );
        }
        // Emit completion for this category
        let _ = app_clone.emit(
            "category-check-complete",
            serde_json::json!({
                "category": category_clone
            }),
        );
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn apply_tweak(
    id: String,
    dangerous_acknowledgement: Option<String>,
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

    validate_dangerous_acknowledgement(
        &tweak.id,
        &tweak.warning_level,
        dangerous_acknowledgement.as_deref(),
    )?;

    // Refuse to mutate the PC unless the applied-state journal can be
    // persisted. Without this preflight, a restricted process could change
    // the registry and only then discover that its elevated state file is not
    // writable, leaving the UI out of sync with Windows.
    {
        let state_path = crate::modules::utils::dirs::get_state_path()
            .map_err(|e| format!("Cannot determine state path: {e}"))?;
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state
            .save(state_path)
            .map_err(|e| format!("Cannot safely apply tweak because state is not writable: {e}"))?;
    }

    // Emit progress start
    let _ = app.emit(
        "tweak-progress",
        serde_json::json!({
            "id": id.clone(),
            "status": "applying",
            "progress": 0
        }),
    );

    // 2. Execute operations (Async/Blocking wrapper)
    // We use tokio::spawn_blocking for heavy IO/Process operations to avoid blocking the async runtime
    let tweak_clone = tweak.clone();
    let id_closure = id.clone();
    let app_closure = app.clone();

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let id = id_closure;
        let app = app_closure;

        // Create a single backup manager for all registry operations
        let backup_path = crate::modules::utils::dirs::get_registry_backup_path()
            .map_err(|e| format!("Cannot determine registry backup path: {}", e))?;
        let mut backup_mgr = RegistryBackup::new(backup_path.clone());

        for op in &tweak_clone.operations {
            if let TweakOperation::Command { cmd, .. } = op {
                validate_command(cmd)?;
            }
        }

        // Capture and persist every registry value before the first mutation.
        // This keeps rollback data intact even if a later operation fails or
        // the application is interrupted mid-tweak.
        for op in &tweak_clone.operations {
            match op {
                TweakOperation::RegistrySet { root_key, path, key, .. }
                | TweakOperation::RegistryDelete { root_key, path, key } => backup_mgr
                    .backup_value(root_key, path, key)
                    .map_err(|e| format!("Failed to capture rollback value for {key}: {e}"))?,
                _ => {}
            }
        }
        backup_mgr
            .save(backup_path.clone())
            .map_err(|e| format!("Failed to persist rollback backup: {e}"))?;

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
                    if let Err(e) = apply_registry_tweak(op) {
                       eprintln!("  -> Registry Error: {:?}", e);
                       return Err(format!("Registry error: {:?}", e));
                    }
                    println!("  -> Registry Delete Success");
                }
                TweakOperation::Command { cmd, args } => {
                    println!("  -> Command: {} {:?}", cmd, args);
                    validate_command(&cmd)?;
                    let output = Command::new(cmd)
                        .args(args)
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Command exec failed: {}", e))?;

                    if !output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let combined = format!("{}{}", stdout, stderr).to_lowercase();

                        if combined.contains("element not found") || combined.contains("value is protected") {
                            eprintln!("  -> Non-fatal: {} {:?} → {}", cmd, args, combined.trim());
                        } else {
                            return Err(format!(
                                "Command returned non-zero {:?}: {}",
                                output.status.code(),
                                combined.trim()
                            ));
                        }
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

                    log("Executing Script...".to_string());

                    use std::io::{BufRead, BufReader, Write};
                    use std::process::Stdio;

                    let app_handle = app.clone();
                    let id_clone = id.clone();

                    // SECURITY FIX: Use RemoteSigned + pass script via stdin
                    let mut child = Command::new("powershell")
                        .args([
                            "-NoProfile",
                            "-NonInteractive",
                            "-ExecutionPolicy", "RemoteSigned", // not Bypass
                            "-Command", "-",                    // read from stdin
                        ])
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW
                        .spawn()
                        .map_err(|e| format!("PowerShell spawn failed: {}", e))?;

                    // Write script to stdin (not CLI arg)
                    if let Some(mut stdin) = child.stdin.take() {
                        writeln!(stdin, "{}", script)
                            .map_err(|e| format!("Failed to write script to stdin: {}", e))?;
                    }

                    // Register PID
                    let pid = child.id();
                    {
                        use tauri::Manager;
                        if let Some(state) = app_handle.try_state::<Mutex<crate::modules::utils::process_manager::ProcessManager>>() {
                             if let Ok(mut mgr) = state.lock() {
                                 mgr.register(id_clone.clone(), pid);
                             }
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
                                    let clean = strip_str(&l);
                                    println!("    [PS STREAM] {}", clean);
                                    let _ = app_handle.emit("tweak-output", serde_json::json!({
                                        "id": id_clone,
                                        "type": "stdout",
                                        "line": clean
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
                             if let Ok(mut mgr) = state.lock() {
                                 mgr.unregister(&id_clone);
                             }
                        }
                    }

                    if !output.status.success() {
                        let stderr = strip_str(&String::from_utf8_lossy(&output.stderr));
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
                    let output = Command::new("sc")
                        .args(&["stop", name])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to stop service {}: {}", name, e))?;

                    if !output.status.success() {
                        // ignore error, service might already be stopped
                    }

                    let output_config = Command::new("sc")
                        .args(&["config", name, "start=", "disabled"])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to disable service {}: {}", name, e))?;

                    if !output_config.status.success() {
                        return Err(format!(
                            "Failed to disable service {}: {}",
                            name,
                            String::from_utf8_lossy(&output_config.stderr).trim()
                        ));
                    }
                }
                TweakOperation::ServiceSetMode { name, mode } => {
                    println!("  -> ServiceSetMode: {} -> {}", name, mode);

                    let sc_mode = match mode.to_lowercase().as_str() {
                        "automatic" | "auto" => "auto",
                        "manual" | "demand" => "demand",
                        "disabled" => "disabled",
                        "delayed-auto" | "delayedauto" => "delayed-auto",
                        _ => "demand", // default fallback
                    };

                    let output = Command::new("sc")
                        .args(&["config", name, "start=", sc_mode])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to set service mode {}: {}", name, e))?;

                    if !output.status.success() {
                        return Err(format!(
                            "Failed to set service mode for {}: {}",
                            name,
                            String::from_utf8_lossy(&output.stderr).trim()
                        ));
                    }
                }
                TweakOperation::ScheduledTaskDisable { path, name } => {
                    println!("  -> ScheduledTaskDisable: {}\\{}", path, name);
                    let full_path = if path == "\\" || path.is_empty() {
                        format!("\\{}", name)
                    } else {
                        format!("{}\\{}", path, name)
                    };

                    let output = Command::new("schtasks")
                        .args(&["/Change", "/TN", &full_path, "/Disable"])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to disable task {}: {}", name, e))?;

                    if !output.status.success() {
                        return Err(format!(
                            "Failed to disable scheduled task {}: {}",
                            full_path,
                            String::from_utf8_lossy(&output.stderr).trim()
                        ));
                    }
                }
                TweakOperation::ScheduledTaskEnable { path, name } => {
                    println!("  -> ScheduledTaskEnable: {}\\{}", path, name);
                    let full_path = if path == "\\" || path.is_empty() {
                        format!("\\{}", name)
                    } else {
                        format!("{}\\{}", path, name)
                    };

                    let output = Command::new("schtasks")
                        .args(&["/Change", "/TN", &full_path, "/Enable"])
                        .creation_flags(0x08000000)
                        .output()
                        .map_err(|e| format!("Failed to enable task {}: {}", name, e))?;

                    if !output.status.success() {
                        return Err(format!(
                            "Failed to enable scheduled task {}: {}",
                            full_path,
                            String::from_utf8_lossy(&output.stderr).trim()
                        ));
                    }
                }
                TweakOperation::FileOperation(file_op) => {
                    use crate::modules::types::FileOp;
                    use std::fs;

                    match file_op {
                        FileOp::Delete { path } => {
                            println!("  -> FileOp Delete: {}", path);
                            let safe = safe_path_existing(&path)?;
                            if safe.is_dir() {
                                fs::remove_dir_all(&safe)
                                    .map_err(|e| format!("Failed to delete directory {}: {}", path, e))?;
                            } else {
                                fs::remove_file(&safe)
                                    .map_err(|e| format!("Failed to delete file {}: {}", path, e))?;
                            }
                        }
                        FileOp::Copy { src, dest } => {
                            println!("  -> FileOp Copy: {} -> {}", src, dest);
                            let safe_src = safe_path_existing(&src)?;
                            let safe_dest = safe_path_new(&dest)?;
                            if let Some(parent) = safe_dest.parent() {
                                fs::create_dir_all(parent)
                                    .map_err(|e| format!("Failed to create dest dir: {}", e))?;
                            }
                            fs::copy(&safe_src, &safe_dest)
                                .map_err(|e| format!("Failed to copy {} to {}: {}", src, dest, e))?;
                        }
                        FileOp::Move { src, dest } => {
                            println!("  -> FileOp Move: {} -> {}", src, dest);
                            let safe_src = safe_path_existing(&src)?;
                            let safe_dest = safe_path_new(&dest)?;
                            if let Some(parent) = safe_dest.parent() {
                                fs::create_dir_all(parent)
                                    .map_err(|e| format!("Failed to create dest dir: {}", e))?;
                            }
                            fs::rename(&safe_src, &safe_dest)
                                .map_err(|e| format!("Failed to move {} to {}: {}", src, dest, e))?;
                        }
                        FileOp::Write { path, content } => {
                            println!("  -> FileOp Write: {}", path);
                            let safe = safe_path_new(&path)?;
                            if let Some(parent) = safe.parent() {
                                fs::create_dir_all(parent)
                                    .map_err(|e| format!("Failed to create parent dir: {}", e))?;
                            }
                            fs::write(&safe, &content)
                                .map_err(|e| format!("Failed to write file {}: {}", path, e))?;
                        }
                    }
                }
                TweakOperation::NetAdapterProperty { property, value } => {
                    println!("  -> NetAdapterProperty: {} = {}", property, value);
                    set_nic_property(property, value)
                        .map_err(|e| format!("NetAdapterProperty error: {}", e))?;
                    println!("  -> NetAdapterProperty Success");
                }
                TweakOperation::SetDnsServers { primary, secondary } => {
                    println!("  -> SetDnsServers: {}, {}", primary, secondary);
                    set_dns_servers(primary, secondary)
                        .map_err(|e| format!("SetDnsServers error: {}", e))?;
                    println!("  -> SetDnsServers Success");
                }
                TweakOperation::ResetDnsServers => {
                    println!("  -> ResetDnsServers");
                    reset_dns_servers()
                        .map_err(|e| format!("ResetDnsServers error: {}", e))?;
                    println!("  -> ResetDnsServers Success");
                }
                TweakOperation::DefenderServiceControl { services, action } => {
                    println!("  -> DefenderServiceControl: {:?} action={}", services, action);
                    control_defender_services(services, action)
                        .map_err(|e| format!("DefenderServiceControl error: {}", e))?;
                    println!("  -> DefenderServiceControl Success");
                }
                TweakOperation::DefenderExclusion { paths, action } => {
                    println!("  -> DefenderExclusion: {:?} action={}", paths, action);
                    set_defender_exclusions(paths, action)
                        .map_err(|e| format!("DefenderExclusion error: {}", e))?;
                    println!("  -> DefenderExclusion Success");
                }
                TweakOperation::SvcHostSplitAll { enable_split } => {
                    println!("  -> SvcHostSplitAll: enable_split={}", enable_split);
                    apply_svc_host_split_all(*enable_split)
                        .map_err(|e| format!("SvcHostSplitAll error: {}", e))?;
                    println!("  -> SvcHostSplitAll Success");
                }
                TweakOperation::MsiSet { class, priority } => {
                    println!("  -> MsiSet: class={}, priority={}", class, priority);
                    apply_msi_set(&class, *priority)
                        .map_err(|e| format!("MsiSet error: {}", e))?;
                    println!("  -> MsiSet Success");
                }
                TweakOperation::MsiRemove { class } => {
                    println!("  -> MsiRemove: class={}", class);
                    apply_msi_remove(&class)
                        .map_err(|e| format!("MsiRemove error: {}", e))?;
                    println!("  -> MsiRemove Success");
                }
                TweakOperation::MsiSetNet { priority } => {
                    println!("  -> MsiSetNet: priority={}", priority);
                    apply_msi_set("Net", *priority)
                        .map_err(|e| format!("MsiSetNet error: {}", e))?;
                    println!("  -> MsiSetNet Success");
                }
                TweakOperation::MsiRemoveNet => {
                    println!("  -> MsiRemoveNet");
                    apply_msi_remove("Net")
                        .map_err(|e| format!("MsiRemoveNet error: {}", e))?;
                    println!("  -> MsiRemoveNet Success");
                }
                    TweakOperation::NetworkInterfacesSet { key, value: _ } => {
                    println!("  -> NetworkInterfacesSet: key={}", key);
                    use crate::modules::registry::operations::apply_network_interface_tweak;
                    apply_network_interface_tweak(op)
                        .map_err(|e| format!("NetworkInterfacesSet error: {}", e))?;
                    println!("  -> NetworkInterfacesSet Success");
                }
                TweakOperation::NetworkInterfacesDelete { key } => {
                    println!("  -> NetworkInterfacesDelete: key={}", key);
                    use crate::modules::registry::operations::apply_network_interface_tweak;
                    apply_network_interface_tweak(op)
                        .map_err(|e| format!("NetworkInterfacesDelete error: {}", e))?;
                    println!("  -> NetworkInterfacesDelete Success");
                }
            }
        }

        // Persist all registry backups to disk
        backup_mgr
            .save(backup_path)
            .map_err(|e| format!("Failed to persist rollback backup: {e}"))?;

        Ok(())
    }).await.map_err(|e| e.to_string())??;

    // 3. Update State & Persist
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        if matches!(tweak.tweak_type, crate::modules::types::TweakType::Toggle) {
            app_state.applied_tweaks.insert(id.clone());
        }

        let state_path = crate::modules::utils::dirs::get_state_path()
            .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));

        app_state
            .save(state_path)
            .map_err(|e| format!("Failed to save app state: {e}"))?;
    }

    // Emit success
    let _ = app.emit(
        "tweak-progress",
        serde_json::json!({
            "id": id,
            "status": "success",
            "progress": 100
        }),
    );

    Ok(())
}

fn validate_dangerous_acknowledgement(
    id: &str,
    warning_level: &WarningLevel,
    acknowledgement: Option<&str>,
) -> Result<(), String> {
    if !matches!(warning_level, WarningLevel::Dangerous) {
        return Ok(());
    }

    let expected = format!("APPLY {id}");
    if acknowledgement != Some(expected.as_str()) {
        return Err(format!(
            "Dangerous tweak acknowledgement required. Review the risk and confirm with: {expected}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod dangerous_acknowledgement_tests {
    use super::*;

    #[test]
    fn dangerous_controls_require_the_exact_id_scoped_phrase() {
        assert!(validate_dangerous_acknowledgement(
            "sec_disable_firewall",
            &WarningLevel::Dangerous,
            None,
        )
        .is_err());
        assert!(validate_dangerous_acknowledgement(
            "sec_disable_firewall",
            &WarningLevel::Dangerous,
            Some("APPLY another_tweak"),
        )
        .is_err());
        assert!(validate_dangerous_acknowledgement(
            "sec_disable_firewall",
            &WarningLevel::Dangerous,
            Some("APPLY sec_disable_firewall"),
        )
        .is_ok());
    }

    #[test]
    fn safe_controls_do_not_need_an_acknowledgement() {
        assert!(
            validate_dangerous_acknowledgement("safe_control", &WarningLevel::Safe, None,).is_ok()
        );
    }
}

#[tauri::command]
pub fn kill_tweak_process(
    id: String,
    proc_mgr: State<Mutex<crate::modules::utils::process_manager::ProcessManager>>,
) -> Result<(), String> {
    let pid = {
        let mgr = proc_mgr.lock().map_err(|e| e.to_string())?;
        mgr.get_pid(&id)
    };
    if let Some(pid) = pid {
        println!("[PROCESS KILL] Killing PID {} for tweak: {}", pid, id);
        // BUG-M2 fix: propagate taskkill errors instead of silently ignoring
        let output = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string(), "/T"])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| format!("taskkill spawn failed: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("taskkill failed (PID {}): {}", pid, stderr.trim()));
        }
    } else {
        println!("[PROCESS KILL] No active PID found for tweak: {}", id);
    }
    Ok(())
}

#[tauri::command]
pub async fn undo_tweak(
    id: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let state_path = crate::modules::utils::dirs::get_state_path()
            .map_err(|e| format!("Cannot determine state path: {e}"))?;
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state
            .save(state_path)
            .map_err(|e| format!("Cannot safely undo tweak because state is not writable: {e}"))?;
    }

    let _ = app.emit(
        "tweak-progress",
        serde_json::json!({
            "id": id.clone(),
            "status": "reverting",
            "progress": 0
        }),
    );

    let tweak = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or("Tweak ID not found")?
    };

    let backup_path = crate::modules::utils::dirs::get_registry_backup_path()
        .map_err(|e| format!("Cannot determine registry backup path: {}", e))?;

    let id_clone = id.clone();
    let app_clone = app.clone();

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let id = id_clone;
        let _app = app_clone;

        // Try to load backup manager for registry operations
        let backup_mgr = RegistryBackup::load(backup_path.clone()).ok();

        // Track which registry keys were restored from backup (to avoid double-revert)
        let mut restored_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

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
                        if mgr
                            .restore_tweak_backup(root_key, path, key)
                            .map_err(|e| format!("Failed to restore registry backup: {e}"))?
                        {
                            restored_keys.insert(format!("{}::{}::{}", root_key, path, key));
                        }
                    }
                }
                _ => {}
            }
        }

        // Then, execute explicit revert operations (skip keys already restored from backup)
        println!("[TWEAK REVERT] Reverting ID: {}", id);
        if let Some(ref revert_ops) = tweak.revert_operations {
            for (i, op) in revert_ops.iter().enumerate() {
                println!(
                    "[TWEAK REVERT] Operation {}/{}: {:?}",
                    i + 1,
                    revert_ops.len(),
                    op
                );

                // Skip registry operations that were already restored from backup
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
                        let key_id = format!("{}::{}::{}", root_key, path, key);
                        if restored_keys.contains(&key_id) {
                            println!("  -> Skipped (already restored from backup): {}", key_id);
                            continue;
                        }
                    }
                    _ => {}
                }

                match op {
                    TweakOperation::RegistrySet { .. } | TweakOperation::RegistryDelete { .. } => {
                        println!("  -> Registry Operation (Revert): {:?}", op);
                        apply_registry_tweak(op)
                            .map_err(|e| format!("Revert registry error: {:?}", e))?;
                        println!("  -> Revert Registry Success");
                    }
                    TweakOperation::Powershell { script } => {
                        println!("  -> Revert PowerShell: {}", script);
                        use std::io::Write;
                        use std::process::Stdio;

                        // SECURITY FIX: Use RemoteSigned + pass script via stdin
                        let mut child = Command::new("powershell")
                            .args([
                                "-NoProfile",
                                "-NonInteractive",
                                "-ExecutionPolicy",
                                "RemoteSigned",
                                "-Command",
                                "-",
                            ])
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .creation_flags(0x08000000)
                            .spawn()
                            .map_err(|e| format!("Revert PowerShell spawn failed: {}", e))?;

                        // Write script to stdin (not CLI arg)
                        if let Some(mut stdin) = child.stdin.take() {
                            writeln!(stdin, "{}", script)
                                .map_err(|e| format!("Failed to write script to stdin: {}", e))?;
                        }

                        let output = child
                            .wait_with_output()
                            .map_err(|e| format!("Revert PowerShell failed: {}", e))?;

                        if !output.stdout.is_empty() {
                            println!(
                                "    [REVERT STDOUT] {}",
                                String::from_utf8_lossy(&output.stdout)
                            );
                        }
                        if !output.stderr.is_empty() {
                            eprintln!(
                                "    [REVERT STDERR] {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                        }

                        if !output.status.success() {
                            return Err(format!(
                                "Revert PowerShell script failed: {}",
                                String::from_utf8_lossy(&output.stderr).trim()
                            ));
                        }
                    }
                    TweakOperation::ServiceSetMode { name, mode } => {
                        println!("  -> Revert ServiceSetMode: {} -> {}", name, mode);
                        let sc_mode = match mode.to_lowercase().as_str() {
                            "automatic" | "auto" => "auto",
                            "manual" | "demand" => "demand",
                            "disabled" => "disabled",
                            "delayed-auto" | "delayedauto" => "delayed-auto",
                            _ => "demand", // default fallback
                        };

                        let output = Command::new("sc")
                            .args(&["config", name, "start=", sc_mode])
                            .creation_flags(0x08000000)
                            .output()
                            .map_err(|e| format!("Revert service failed: {}", e))?;

                        if !output.status.success() {
                            return Err(format!(
                                "Failed to restore service mode for {}: {}",
                                name,
                                String::from_utf8_lossy(&output.stderr).trim()
                            ));
                        }
                    }
                    TweakOperation::ScheduledTaskDisable { path, name } => {
                        let full_path = if path == "\\" || path.is_empty() {
                            format!("\\{}", name)
                        } else {
                            format!("{}\\{}", path, name)
                        };
                        let output = Command::new("schtasks")
                            .args(&["/Change", "/TN", &full_path, "/Disable"])
                            .creation_flags(0x08000000)
                            .output()
                            .map_err(|e| format!("Failed to start schtasks: {e}"))?;
                        if !output.status.success() {
                            return Err(format!(
                                "Failed to disable scheduled task {}: {}",
                                full_path,
                                String::from_utf8_lossy(&output.stderr).trim()
                            ));
                        }
                    }
                    TweakOperation::ScheduledTaskEnable { path, name } => {
                        let full_path = if path == "\\" || path.is_empty() {
                            format!("\\{}", name)
                        } else {
                            format!("{}\\{}", path, name)
                        };
                        let output = Command::new("schtasks")
                            .args(&["/Change", "/TN", &full_path, "/Enable"])
                            .creation_flags(0x08000000)
                            .output()
                            .map_err(|e| format!("Failed to start schtasks: {e}"))?;
                        if !output.status.success() {
                            return Err(format!(
                                "Failed to enable scheduled task {}: {}",
                                full_path,
                                String::from_utf8_lossy(&output.stderr).trim()
                            ));
                        }
                    }
                    TweakOperation::ServiceDisable { name } => {
                        let _ = Command::new("sc")
                            .args(&["stop", name])
                            .creation_flags(0x08000000)
                            .output();
                        let output = Command::new("sc")
                            .args(&["config", name, "start=", "disabled"])
                            .creation_flags(0x08000000)
                            .output()
                            .map_err(|e| format!("Failed to start sc.exe: {e}"))?;
                        if !output.status.success() {
                            return Err(format!(
                                "Failed to disable service {}: {}",
                                name,
                                String::from_utf8_lossy(&output.stderr).trim()
                            ));
                        }
                    }
                    TweakOperation::Command { cmd, args } => {
                        println!("  -> Revert Command: {} {:?}", cmd, args);
                        validate_command(&cmd)?;
                        let output = Command::new(cmd)
                            .args(args)
                            .creation_flags(0x08000000)
                            .output()
                            .map_err(|e| format!("Revert command exec failed: {}", e))?;
                        if !output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            let combined = format!("{}{}", stdout, stderr).to_lowercase();

                            if combined.contains("element not found")
                                || combined.contains("value is protected")
                            {
                                eprintln!(
                                    "  -> Non-fatal: {} {:?} -> {}",
                                    cmd,
                                    args,
                                    combined.trim()
                                );
                            } else {
                                return Err(format!(
                                    "Revert command failed with {:?}: {}",
                                    output.status.code(),
                                    combined.trim()
                                ));
                            }
                        }
                    }
                    TweakOperation::NetAdapterProperty { property, value } => {
                        println!("  -> Revert NetAdapterProperty: {} = {}", property, value);
                        if let Err(e) = set_nic_property(property, value) {
                            eprintln!("Warning: Revert NetAdapterProperty failed: {}", e);
                        }
                    }
                    TweakOperation::SetDnsServers { primary, secondary } => {
                        println!("  -> Revert SetDnsServers: {}, {}", primary, secondary);
                        if let Err(e) = set_dns_servers(primary, secondary) {
                            eprintln!("Warning: Revert SetDnsServers failed: {}", e);
                        }
                    }
                    TweakOperation::ResetDnsServers => {
                        println!("  -> Revert ResetDnsServers");
                        if let Err(e) = reset_dns_servers() {
                            eprintln!("Warning: Revert ResetDnsServers failed: {}", e);
                        }
                    }
                    TweakOperation::DefenderServiceControl { services, action } => {
                        println!(
                            "  -> Revert DefenderServiceControl: {:?} action={}",
                            services, action
                        );
                        control_defender_services(services, action)
                            .map_err(|e| format!("Revert DefenderServiceControl failed: {e}"))?;
                    }
                    TweakOperation::DefenderExclusion { paths, action } => {
                        println!(
                            "  -> Revert DefenderExclusion: {:?} action={}",
                            paths, action
                        );
                        set_defender_exclusions(paths, action)
                            .map_err(|e| format!("Revert DefenderExclusion failed: {e}"))?;
                    }
                    TweakOperation::SvcHostSplitAll { enable_split } => {
                        println!("  -> Revert SvcHostSplitAll: enable_split={}", enable_split);
                        apply_svc_host_split_all(*enable_split)
                            .map_err(|e| format!("Revert SvcHostSplitAll failed: {e}"))?;
                    }
                    TweakOperation::MsiSet { class, priority } => {
                        println!("  -> Revert MsiSet: class={}, priority={}", class, priority);
                        apply_msi_set(class, *priority)
                            .map_err(|e| format!("Revert MsiSet failed: {e}"))?;
                    }
                    TweakOperation::MsiRemove { class } => {
                        println!("  -> Revert MsiRemove: class={}", class);
                        apply_msi_remove(class)
                            .map_err(|e| format!("Revert MsiRemove failed: {e}"))?;
                    }
                    TweakOperation::MsiSetNet { priority } => {
                        println!("  -> Revert MsiSetNet: priority={}", priority);
                        apply_msi_set("Net", *priority)
                            .map_err(|e| format!("Revert MsiSetNet failed: {e}"))?;
                    }
                    TweakOperation::MsiRemoveNet => {
                        println!("  -> Revert MsiRemoveNet");
                        apply_msi_remove("Net")
                            .map_err(|e| format!("Revert MsiRemoveNet failed: {e}"))?;
                    }
                    TweakOperation::NetworkInterfacesSet { key, value: _ } => {
                        println!("  -> Revert NetworkInterfacesSet: key={}", key);
                        // BUG-H4 fix: construct enum variant directly — no JSON string injection
                        crate::modules::registry::operations::apply_network_interface_tweak(op)
                            .map_err(|e| format!("Revert network interface value failed: {e}"))?;
                    }
                    TweakOperation::NetworkInterfacesDelete { key } => {
                        println!("  -> Revert NetworkInterfacesDelete: key={}", key);
                        crate::modules::registry::operations::apply_network_interface_tweak(op)
                            .map_err(|e| format!("Revert network interface value failed: {e}"))?;
                    }
                    _ => {
                        println!("  -> Skipped unknown revert op: {:?}", op);
                    }
                }
            }
        }

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())??;

    // Update State (Remove from applied)
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        if app_state.applied_tweaks.remove(&id) {
            let state_path = crate::modules::utils::dirs::get_state_path()
                .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));
            app_state
                .save(state_path)
                .map_err(|e| format!("Failed to save app state: {e}"))?;
        }
    }

    let _ = app.emit(
        "tweak-progress",
        serde_json::json!({
            "id": id,
            "status": "revert-success",
            "progress": 100
        }),
    );

    Ok(())
}

// ─── AI Commands ───────────────────────────────────────────────────────────────

use crate::modules::ai::{
    gemini, profiler, prompts, AnalysisResult, ChatMessage, ChatReply, ChatTweakAction,
    DiagnosisResult,
};

#[tauri::command]
pub fn save_gemini_key(key: String) -> Result<(), String> {
    gemini::save_api_key(&key)
}

#[tauri::command]
pub fn get_gemini_key_status() -> bool {
    gemini::get_api_key().is_ok()
}

#[tauri::command]
pub fn delete_gemini_key() -> Result<(), String> {
    gemini::delete_api_key()
}

#[tauri::command]
pub async fn test_gemini_connection() -> Result<(), String> {
    gemini::test_connection().await
}

#[tauri::command]
pub async fn ai_analyze(
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<AnalysisResult, String> {
    let profile = profiler::scan_system_profile_from_state(Some(app))?;
    let memory = AiMemoryStore::load();
    let memory_ctx = memory.to_prompt_context();

    let startup_items =
        tokio::task::spawn_blocking(crate::modules::startup::scan_all_startup_items)
            .await
            .unwrap_or_default();
    let startup_json = serde_json::to_string(&startup_items).unwrap_or_default();

    let tweaks_summary = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        let mut tweaks: Vec<_> = context
            .tweaks
            .iter()
            .map(|t| {
                serde_json::json!({
                    "id":              t.id,
                    "name":            t.name,
                    "description":     t.description,
                    "risk_level":      format!("{:?}", t.warning_level),
                    "category":        format!("{:?}", t.category),
                    "currently_applied": st.applied_tweaks.contains(&t.id),
                    "requires_restart": t.requires_restart,
                })
            })
            .collect();
        tweaks.sort_by(|a, b| {
            let ca = a["category"].as_str().unwrap_or("");
            let cb = b["category"].as_str().unwrap_or("");
            ca.cmp(cb).then(
                a["id"]
                    .as_str()
                    .unwrap_or("")
                    .cmp(b["id"].as_str().unwrap_or("")),
            )
        });
        tweaks
    };

    let prompt = prompts::build_analyze_prompt(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&tweaks_summary).map_err(|e| e.to_string())?,
    );

    let prompt = format!(
        "{}\n\nSTARTUP ITEMS (for context — suspicious ones may affect performance):\n{}\n\nAI MEMORY:\n{}",
        prompt,
        &startup_json[..startup_json.len().min(3000)],
        memory_ctx
    );

    let (ctx_user, ctx_model) = prompts::build_context_injection(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&tweaks_summary).map_err(|e| e.to_string())?,
        &memory_ctx,
    );

    let raw = gemini::call_gemini(
        vec![
            ("user".to_string(), ctx_user),
            ("model".to_string(), ctx_model),
            ("user".to_string(), prompt),
        ],
        true,
    )
    .await?;

    let raw_trimmed = raw.trim();
    let mut result: AnalysisResult = if raw_trimmed.starts_with('[') {
        serde_json::from_str::<Vec<AnalysisResult>>(raw_trimmed)
            .map_err(|e| {
                format!(
                    "Failed to parse Gemini response: {}\nRaw: {}",
                    e,
                    &raw[..400.min(raw.len())]
                )
            })?
            .into_iter()
            .next()
            .ok_or_else(|| {
                format!(
                    "Empty array in Gemini response\nRaw: {}",
                    &raw[..400.min(raw.len())]
                )
            })?
    } else {
        serde_json::from_str(&raw).map_err(|e| {
            format!(
                "Failed to parse Gemini response: {}\nRaw: {}",
                e,
                &raw[..400.min(raw.len())]
            )
        })?
    };

    // Model output is advisory and untrusted. Only retain ids that exist in
    // the current catalog, and never offer a Dangerous tweak for application.
    let (known, applied, dangerous) = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let state = state.lock().map_err(|e| e.to_string())?;
        (
            context
                .tweaks
                .iter()
                .map(|t| t.id.clone())
                .collect::<std::collections::HashSet<_>>(),
            state.applied_tweaks.clone(),
            context
                .tweaks
                .iter()
                .filter(|t| matches!(t.warning_level, WarningLevel::Dangerous))
                .map(|t| t.id.clone())
                .collect::<std::collections::HashSet<_>>(),
        )
    };
    result.add.retain(|r| {
        known.contains(&r.id) && !applied.contains(&r.id) && !dangerous.contains(&r.id)
    });
    result
        .remove
        .retain(|r| known.contains(&r.id) && applied.contains(&r.id));
    for conflict in &mut result.conflicts {
        conflict.tweak_ids.retain(|id| known.contains(id));
    }
    result
        .conflicts
        .retain(|conflict| conflict.tweak_ids.len() > 1);

    let mut mem = AiMemoryStore::load();
    mem.add(MemoryKind::Recommendation {
        scan_summary: result.system_summary.clone(),
        add: result.add.iter().map(|r| r.id.clone()).collect(),
        remove: result.remove.iter().map(|r| r.id.clone()).collect(),
        applied: vec![],
    });
    mem.last_system_summary = Some(result.system_summary.clone());
    mem.save()?;

    Ok(result)
}

#[tauri::command]
pub async fn ai_chat(
    message: String,
    history: Vec<ChatMessage>,
    session_id: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<ChatReply, String> {
    validate_session_id(&session_id)?;
    let message = message.trim().to_string();
    if message.is_empty() || message.len() > 8_000 {
        return Err("Message must be between 1 and 8,000 characters.".to_string());
    }
    if history.len() > 200
        || history
            .iter()
            .any(|m| !matches!(m.role.as_str(), "user" | "model") || m.content.len() > 32_000)
    {
        return Err("Chat history is invalid or too large.".to_string());
    }
    let profile = profiler::scan_system_profile_from_state(Some(app))?;
    let memory = AiMemoryStore::load();
    let memory_ctx = memory.to_prompt_context();

    let catalog_summary = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .map(|t| {
                serde_json::json!({
                    "id": t.id,
                    "name": t.name,
                    "description": t.description,
                    "warning_level": format!("{:?}", t.warning_level),
                    "applied": st.applied_tweaks.contains(&t.id),
                })
            })
            .collect::<Vec<_>>()
    };

    let (ctx_user, ctx_model) = prompts::build_chat_context_injection(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&catalog_summary).map_err(|e| e.to_string())?,
        &memory_ctx,
    );

    let mut messages = vec![
        ("user".to_string(), ctx_user),
        ("model".to_string(), ctx_model),
    ];
    let history_clone: Vec<_> = history
        .iter()
        .rev()
        .take(30)
        .rev()
        .map(|m| (m.role.clone(), m.content.clone()))
        .collect();
    for msg in history_clone {
        messages.push(msg);
    }
    messages.push(("user".to_string(), message.clone()));

    let response = gemini::call_gemini(messages, false).await?;

    // Model text is untrusted. Re-bind every surfaced action to the live
    // catalog and current applied state; invented IDs can never become buttons.
    let tweak_actions = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .filter(|t| response.contains(&t.id))
            .take(12)
            .map(|t| ChatTweakAction {
                id: t.id.clone(),
                name: t.name.clone(),
                operation: if st.applied_tweaks.contains(&t.id) {
                    "undo".to_string()
                } else {
                    "apply".to_string()
                },
                warning_level: format!("{:?}", t.warning_level),
            })
            .collect::<Vec<_>>()
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let title = message.chars().take(60).collect::<String>();
    let mut all_messages: Vec<AiChatMessage> = history
        .into_iter()
        .map(|m| AiChatMessage {
            role: m.role,
            content: m.content,
            timestamp: now,
            tweak_actions: m.tweak_actions,
        })
        .collect();
    all_messages.push(AiChatMessage {
        role: "user".to_string(),
        content: message,
        timestamp: now,
        tweak_actions: vec![],
    });
    all_messages.push(AiChatMessage {
        role: "model".to_string(),
        content: response.clone(),
        timestamp: now,
        tweak_actions: tweak_actions.clone(),
    });

    let started_at = load_chat_session(&session_id)
        .map(|session| session.started_at)
        .unwrap_or(now);
    let session = ChatSession {
        id: session_id,
        title,
        started_at,
        messages: all_messages,
    };
    save_chat_session(&session)?;

    Ok(ChatReply {
        content: response,
        tweak_actions,
    })
}

#[tauri::command]
pub async fn ai_diagnose(
    problem: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<DiagnosisResult, String> {
    let profile = profiler::scan_system_profile_from_state(Some(app))?;
    let memory = AiMemoryStore::load();
    let memory_ctx = memory.to_prompt_context();

    let applied_detail = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        context
            .tweaks
            .iter()
            .filter(|t| st.applied_tweaks.contains(&t.id))
            .map(|t| {
                serde_json::json!({
                    "id":          t.id,
                    "name":        t.name,
                    "description": t.description,
                    "category":    format!("{:?}", t.category),
                    "risk_level":  format!("{:?}", t.warning_level),
                })
            })
            .collect::<Vec<_>>()
    };

    let (ctx_user, ctx_model) = prompts::build_context_injection(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&applied_detail).map_err(|e| e.to_string())?,
        &memory_ctx,
    );

    let prompt = prompts::build_diagnose_prompt(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&applied_detail).map_err(|e| e.to_string())?,
        &problem,
    );

    let raw = gemini::call_gemini(
        vec![
            ("user".to_string(), ctx_user),
            ("model".to_string(), ctx_model),
            ("user".to_string(), prompt),
        ],
        true,
    )
    .await?;

    let raw_trimmed = raw.trim();
    let mut result: DiagnosisResult = if raw_trimmed.starts_with('[') {
        serde_json::from_str::<Vec<DiagnosisResult>>(raw_trimmed)
            .map_err(|e| {
                format!(
                    "Failed to parse diagnosis: {}\nRaw: {}",
                    e,
                    &raw[..400.min(raw.len())]
                )
            })?
            .into_iter()
            .next()
            .ok_or_else(|| {
                format!(
                    "Empty array in diagnosis response\nRaw: {}",
                    &raw[..400.min(raw.len())]
                )
            })?
    } else {
        serde_json::from_str(&raw).map_err(|e| {
            format!(
                "Failed to parse diagnosis: {}\nRaw: {}",
                e,
                &raw[..400.min(raw.len())]
            )
        })?
    };

    let (known, applied) = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let state = state.lock().map_err(|e| e.to_string())?;
        (
            context
                .tweaks
                .iter()
                .map(|t| t.id.clone())
                .collect::<std::collections::HashSet<_>>(),
            state.applied_tweaks.clone(),
        )
    };
    result
        .likely_causes
        .retain(|cause| known.contains(&cause.tweak_id));
    result
        .safe_to_revert
        .retain(|id| known.contains(id) && applied.contains(id));

    let mut mem = AiMemoryStore::load();
    mem.add(MemoryKind::Diagnosis {
        problem: problem.clone(),
        likely_causes: result
            .likely_causes
            .iter()
            .map(|c| c.tweak_id.clone())
            .collect(),
        suggested_fix: result.suggested_fix.clone(),
        resolved: None,
        follow_up_notes: None,
    });
    let _ = mem.save();

    Ok(result)
}

// ── MEMORY ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_ai_memory_context() -> String {
    AiMemoryStore::load().to_prompt_context()
}

#[tauri::command]
pub fn record_ai_memory(kind_json: String) -> Result<(), String> {
    let kind: MemoryKind =
        serde_json::from_str(&kind_json).map_err(|e| format!("Invalid memory kind JSON: {}", e))?;
    let mut store = AiMemoryStore::load();
    store.add(kind);
    store.save()
}

#[tauri::command]
pub fn get_full_ai_memory() -> Result<String, String> {
    let store = AiMemoryStore::load();
    serde_json::to_string_pretty(&store).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_ai_memory() -> Result<(), String> {
    let store = AiMemoryStore::default();
    store.save()
}

// ── CHAT PERSISTENCE ───────────────────────────────────────────────────────

#[tauri::command]
pub fn save_chat(session_json: String) -> Result<(), String> {
    let session: ChatSession =
        serde_json::from_str(&session_json).map_err(|e| format!("Invalid session JSON: {}", e))?;
    save_chat_session(&session)
}

#[tauri::command]
pub fn list_chats() -> Vec<ChatSessionMeta> {
    list_chat_sessions()
}

#[tauri::command]
pub fn load_chat(id: String) -> Result<ChatSession, String> {
    load_chat_session(&id)
}

#[tauri::command]
pub fn delete_chat(id: String) -> Result<(), String> {
    delete_chat_session(&id)
}

// ── STARTUP AI ───────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn ai_scan_startup(
    items: Vec<StartupItem>,
    app: tauri::AppHandle,
) -> Result<crate::modules::ai::startup::StartupScanResult, String> {
    scan_startup_with_ai(items, app).await
}

#[tauri::command]
pub async fn ai_apply_startup_recommendations(
    recommendations_json: String,
) -> Result<Vec<String>, String> {
    let recs: Vec<StartupRecommendation> = serde_json::from_str(&recommendations_json)
        .map_err(|e| format!("Invalid recs JSON: {}", e))?;

    let current_items =
        tokio::task::spawn_blocking(crate::modules::startup::scan_all_startup_items)
            .await
            .map_err(|e| e.to_string())?;
    let current = current_items
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect::<std::collections::HashMap<_, _>>();

    let mut results: Vec<String> = Vec::new();
    let mut memory = AiMemoryStore::load();

    for rec in &recs {
        if rec.action == "disable" {
            let item = current
                .get(&rec.item_id)
                .ok_or_else(|| format!("Startup item no longer exists: {}", rec.item_id))?;
            if matches!(
                item.safety_rating,
                crate::modules::startup::types::SafetyRating::Critical
            ) {
                return Err(format!(
                    "Refusing to disable critical startup item: {}",
                    item.name
                ));
            }
            let result = tokio::task::spawn_blocking({
                let id = rec.item_id.clone();
                move || toggle_item(id, false)
            })
            .await
            .map_err(|e| e.to_string())?;

            let outcome = if result.is_ok() { "success" } else { "failed" };
            results.push(format!("{}: {}", rec.item_id, outcome));

            memory.add(MemoryKind::StartupAction {
                item_id: rec.item_id.clone(),
                item_name: rec.item_name.clone(),
                action: "disabled".to_string(),
                reason: rec.reason.clone(),
            });
        }
    }

    memory.save()?;
    Ok(results)
}
