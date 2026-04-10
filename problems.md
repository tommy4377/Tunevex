
````markdown
# TommyTweaker — Full Bug Audit (repo-5 state)

## Project Context
- **Stack:** Rust (Tauri v2.9.1) + Svelte frontend
- **Key files:**
  - `src-tauri/src/commands.rs` — main Tauri command handlers + helpers
  - `src-tauri/src/modules/system/monitoring.rs` — system stats + GPU monitoring
  - `src-tauri/src/modules/startup/tasks.rs` — scheduled task scanner
  - `src-tauri/Cargo.toml` — dependency + features config

---

## 🔴 BUG-C1 — [COMPILE ERROR] Wrong PDH API module path in `monitoring.rs` (13 compiler errors)

**File:** `src-tauri/src/modules/system/monitoring.rs`
**Severity:** BLOCKING (will not compile)

The module path `windows::Win32::Performance::DataHelper` / `windows::Win32::Performance::Pdh` does not exist in `windows` crate `0.61.x`.
The correct path is `windows::Win32::System::Performance`.
Additionally, the helper functions `initialize_pdh_query` and `create_gpu_counter` are called but never defined, and all PDH calls are missing `unsafe {}` blocks.

### Step 1 — Add missing feature to `Cargo.toml`

```toml
# Cargo.toml — add Win32_System_Performance to the windows features list
[dependencies.windows]
version = "0.61.3"
features = [
    "Win32_Foundation",
    "Win32_Graphics_Dwm",
    "Win32_UI_Controls",
    "Win32_UI_Shell",
    "Win32_Storage_FileSystem",
    "Win32_System_IO",
    "Win32_System_Ioctl",
    "Win32_System_Console",
    "Win32_Security",
    "Win32_NetworkManagement_IpHelper",
    "Win32_Devices_Display",
    "Win32_System_SystemInformation",
    "Win32_System_Threading",
    "Win32_System_Performance",   # ← ADD THIS
]
```

### Step 2 — Fix `monitoring.rs` entirely

```rust
// ❌ CURRENT — wrong module path, missing functions, missing unsafe
use windows::Win32::Foundation;
use windows::Win32::Performance::DataHelper;  // does not exist
use windows::Win32::Performance::Pdh;         // does not exist

pub struct SystemMonitor {
    pdhquery:   Option<PDH_HQUERY>,    // PDH_HQUERY not in scope
    pdhcounter: Option<PDH_HCOUNTER>,  // PDH_HCOUNTER not in scope
}

// In new():
let pdh_query_result = initialize_pdh_query();  // function not defined
let pdh_counter = if let Ok(query) = pdh_query_result {
    create_gpu_counter(query).ok()              // function not defined
} else { None };

// In thread loop:
unsafe PdhCollectQueryData(query);  // not in unsafe block
let mut value: PDH_FMT_COUNTERVALUE = std::mem::zeroed();   // type not in scope
let status = PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, ...); // not in unsafe block

// In Drop:
PdhCloseQuery(query);  // not in unsafe block, not in scope

// ✅ FIX — correct imports, define helpers, add unsafe blocks
use std::ptr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};
use tauri::command;
use windows::Win32::System::Performance::{    // ← correct path
    PdhAddCounterW, PdhCloseQuery, PdhCollectQueryData,
    PdhGetFormattedCounterValue, PdhOpenQueryW,
    PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE, PDH_HCOUNTER, PDH_HQUERY,
};
use windows::Win32::Foundation::ERROR_SUCCESS;

const CREATE_NO_WINDOW: u32 = 0x08000000;

// ─── Helper: open a PDH query handle ────────────────────────────────────────
unsafe fn initialize_pdh_query() -> Result<PDH_HQUERY, String> {
    let mut query = PDH_HQUERY::default();
    let status = PdhOpenQueryW(None, 0, &mut query);
    if status != 0 {
        return Err(format!("PdhOpenQueryW failed: {:#x}", status));
    }
    Ok(query)
}

// ─── Helper: add GPU utilization counter to an open query ───────────────────
unsafe fn create_gpu_counter(query: PDH_HQUERY) -> Result<PDH_HCOUNTER, String> {
    use windows::core::w;
    let mut counter = PDH_HCOUNTER::default();
    let path = w!(r"\GPU Engine(*engtype_3D)\Utilization Percentage");
    let status = PdhAddCounterW(query, path, 0, &mut counter);
    if status != 0 {
        return Err(format!("PdhAddCounterW failed: {:#x}", status));
    }
    // First collection is always 0 — needed to seed the counter
    PdhCollectQueryData(query);
    Ok(counter)
}

pub struct SystemMonitor {
    sys:             System,
    disks:           Disks,
    gpu_usage:       Arc<Mutex<f32>>,
    gpu_name:        String,
    gpu_thread_handle: Option<thread::JoinHandle<()>>,
    gpu_cancel:      Arc<AtomicBool>,
    pdh_query:       Option<PDH_HQUERY>,
    pdh_counter:     Option<PDH_HCOUNTER>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let gpu_usage   = Arc::new(Mutex::new(0.0f32));
        let gpu_cancel  = Arc::new(AtomicBool::new(false));
        let gpu_usage_clone  = gpu_usage.clone();
        let gpu_cancel_clone = gpu_cancel.clone();

        // Open PDH handles once before spawning the thread
        let (pdh_query, pdh_counter) = unsafe {
            match initialize_pdh_query() {
                Ok(q) => match create_gpu_counter(q) {
                    Ok(c)  => (Some(q), Some(c)),
                    Err(e) => { eprintln!("PDH counter error: {}", e); (Some(q), None) }
                },
                Err(e) => { eprintln!("PDH query error: {}", e); (None, None) }
            }
        };

        let gpu_thread = thread::spawn(move || {
            loop {
                if gpu_cancel_clone.load(Ordering::SeqCst) { break; }

                if let (Some(query), Some(counter)) = (pdh_query, pdh_counter) {
                    // SAFETY: PDH handles are valid until Drop
                    unsafe { PdhCollectQueryData(query) };

                    let mut fmt_value = unsafe { std::mem::zeroed::<PDH_FMT_COUNTERVALUE>() };
                    let status = unsafe {
                        PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut fmt_value)
                    };

                    if status == 0 {
                        // SAFETY: PDH_FMT_DOUBLE guarantees doubleValue is valid
                        let usage = unsafe { fmt_value.Anonymous.doubleValue } as f32;
                        let clamped = usage.clamp(0.0, 100.0);
                        if let Ok(mut g) = gpu_usage_clone.lock() {
                            *g = clamped;
                        }
                    }
                }

                // Sleep in 100ms increments so cancel is responsive
                for _ in 0..20 {
                    if gpu_cancel_clone.load(Ordering::SeqCst) { return; }
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        let gpu_name = get_gpu_name().unwrap_or_else(|| "Unknown GPU".to_string());

        Self {
            sys: System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            disks: Disks::new_with_refreshed_list(),
            gpu_usage,
            gpu_name,
            gpu_thread_handle: Some(gpu_thread),
            gpu_cancel,
            pdh_query,
            pdh_counter,
        }
    }
}

impl Drop for SystemMonitor {
    fn drop(&mut self) {
        self.gpu_cancel.store(true, Ordering::SeqCst);
        if let Some(handle) = self.gpu_thread_handle.take() {
            let _ = handle.join();
        }
        // SAFETY: handles are valid (Some) only if successfully opened
        if let Some(query) = self.pdh_query.take() {
            unsafe { PdhCloseQuery(query) };
        }
        // pdh_counter is automatically invalidated when the query is closed
        self.pdh_counter = None;
    }
}
```

---

## 🔴 BUG-H1 — `safe_path()` calls `canonicalize()` on non-existent destination paths

**File:** `src-tauri/src/commands.rs` — `safe_path()` helper, used in `FileOp::Write`, `FileOp::Copy` (dest), `FileOp::Move` (dest)
**Severity:** HIGH (all file write/copy/move operations silently fail at runtime)

`std::path::Path::canonicalize()` returns an error if the path does not already exist on disk.
`FileOp::Write` paths are always new files; `FileOp::Copy`/`FileOp::Move` destination paths rarely pre-exist.
The current code calls `safe_path()` on the destination, gets `Err("Invalid or non-existent path")`, propagates it with `?`, and the entire operation is aborted — silently, from the user's perspective.

```rust
// ❌ CURRENT — canonicalize() always fails for new files
fn safe_path(raw: &str) -> Result<std::path::PathBuf, String> {
    let p = std::path::PathBuf::from(raw);
    let canonical = p.canonicalize()  // ERROR if path doesn't exist yet
        .map_err(|e| format!("Invalid or non-existent path '{}': {}", raw, e))?;
    // ... allowed roots check
    Ok(canonical)
}

// ✅ FIX — separate validation for existing (source) vs new (destination) paths
const ALLOWED_ROOTS: &[&str] = &[
    r"C:\Users",
    r"C:\ProgramData",
    r"C:\Program Files",
    r"C:\Program Files (x86)",
    r"C:\Windows\Temp",
    r"C:\Temp",
];

fn is_under_allowed_root(p: &std::path::Path) -> bool {
    let s = p.to_string_lossy().to_lowercase();
    ALLOWED_ROOTS.iter().any(|r| s.starts_with(&r.to_lowercase()))
}

/// For SOURCE paths (must exist): canonicalize to resolve symlinks + ".."
fn safe_path_existing(raw: &str) -> Result<std::path::PathBuf, String> {
    let canonical = std::path::Path::new(raw)
        .canonicalize()
        .map_err(|e| format!("Path not found '{}': {}", raw, e))?;
    if !is_under_allowed_root(&canonical) {
        return Err(format!("Path '{}' is outside allowed roots", raw));
    }
    Ok(canonical)
}

/// For DESTINATION paths (may not exist yet): normalize without canonicalize
fn safe_path_new(raw: &str) -> Result<std::path::PathBuf, String> {
    // Resolve env vars manually if needed, then normalize without requiring existence
    let p = std::path::PathBuf::from(raw);
    // Walk ancestors to find the highest-existing prefix and canonicalize only that
    let mut existing = p.clone();
    let mut tail = std::path::PathBuf::new();
    loop {
        if existing.exists() { break; }
        if let Some(parent) = existing.parent() {
            tail = existing.file_name()
                .map(|f| std::path::PathBuf::from(f).join(&tail))
                .unwrap_or(tail);
            existing = parent.to_path_buf();
        } else {
            break;
        }
    }
    let base = if existing.exists() {
        existing.canonicalize()
            .map_err(|e| format!("Cannot resolve base path: {}", e))?
    } else {
        existing
    };
    let full = base.join(tail);
    if !is_under_allowed_root(&full) {
        return Err(format!("Destination '{}' is outside allowed roots", raw));
    }
    Ok(full)
}

// Usage in apply_tweak:
FileOp::Delete { path } => {
    let safe = safe_path_existing(&path)?;                // must exist
    // ...
}
FileOp::Copy { src, dest } => {
    let safe_src  = safe_path_existing(&src)?;            // must exist
    let safe_dest = safe_path_new(&dest)?;                // may not exist
    // ...
}
FileOp::Move { src, dest } => {
    let safe_src  = safe_path_existing(&src)?;
    let safe_dest = safe_path_new(&dest)?;
    // ...
}
FileOp::Write { path, content } => {
    let safe = safe_path_new(&path)?;                     // almost never pre-exists
    // ...
}
```

---

## 🔴 BUG-H2 — `check_tweak_enabled()` check path bypasses `validate_command` whitelist

**File:** `src-tauri/src/commands.rs` — `check_command_output_contains()`
**Severity:** HIGH (security: whitelist only applied on apply/undo paths, not on check path)

`validate_command()` was added to `apply_tweak` and `undo_tweak` for `TweakOperation::Command`.
But `check_tweak_enabled()` calls `check_command_output_contains(cmd, args, contains)` for `TweakCheckCommandOutputContains` variants directly, passing `cmd` to `Command::new()` **without any validation**.
This function is called from `get_tweaks` (spawn_blocking), `check_category` (spawn_blocking), and `get_tweaks_fast`.

```rust
// ❌ CURRENT — unvalidated command execution in check path
fn check_command_output_contains(cmd: &str, args: &[String], contains: &str) -> bool {
    let output = Command::new(cmd)   // ← no validate_command() call
        .args(args)
        .creation_flags(0x08000000)
        .output();
    // ...
}

// ✅ FIX — apply the same whitelist check before spawning
fn check_command_output_contains(cmd: &str, args: &[String], contains: &str) -> bool {
    if validate_command(cmd).is_err() {
        eprintln!("check_command_output_contains: blocked non-whitelisted cmd '{}'", cmd);
        return false;  // treat as "not enabled" — fail safe
    }
    let output = Command::new(cmd)
        .args(args)
        .creation_flags(0x08000000)
        .output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_lowercase();
            let stderr = String::from_utf8_lossy(&out.stderr).to_lowercase();
            let needle = contains.to_lowercase();
            stdout.contains(&needle) || stderr.contains(&needle)
        }
        Err(_) => false,
    }
}
```

---

## 🔴 BUG-H3 — `check_powershell_output()` still uses `-ExecutionPolicy Bypass` on check path

**File:** `src-tauri/src/commands.rs` — `check_powershell_output()`
**Severity:** HIGH (security: the fix for SEC-03 was applied only to apply/undo — the check path was missed)

`check_powershell_output()` is called for every `TweakCheckPowershell` variant during `get_tweaks`, `check_category`, and `get_tweaks_fast`. It still spawns PowerShell with `-ExecutionPolicy Bypass` and passes the script as a CLI argument, meaning the fix applied to `TweakOperation::Powershell` was **not** applied here.

```rust
// ❌ CURRENT — Bypass + script as CLI arg (unchanged from repo-4)
fn check_powershell_output(script: &str, expected_output: &str) -> bool {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", script])
        .creation_flags(0x08000000)
        .output();
    // ...
}

// ✅ FIX — RemoteSigned + script via stdin (same pattern as apply/undo)
fn check_powershell_output(script: &str, expected_output: &str) -> bool {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = match Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy", "RemoteSigned",
            "-Command", "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .spawn()
    {
        Ok(c)  => c,
        Err(_) => return false,
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", script);
    }

    match child.wait_with_output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            stdout == expected_output.trim().to_lowercase()
        }
        Err(_) => false,
    }
}
```

---

## 🔴 BUG-H4 — `NetworkInterfacesSet` revert constructs `TweakOperation` via raw JSON string injection

**File:** `src-tauri/src/commands.rs` — `undo_tweak`, `TweakOperation::NetworkInterfacesSet` revert arm
**Severity:** HIGH (silent failure + JSON injection risk)

The revert for `NetworkInterfacesSet` builds a `TweakOperation` by formatting a raw JSON string and calling `serde_json::from_str()` on it:

```rust
// ❌ CURRENT — JSON injection: if `key` contains `"` or `\`, parse fails silently
TweakOperation::NetworkInterfacesSet { key, value } => {
    if let Ok(op) = serde_json::from_str::<TweakOperation>(
        &format!(r#"{{"NetworkInterfacesDelete":{{"key":"{}"}}}}"#, key)
    ) {
        apply_network_interface_tweak(&op);
    }
    // If serde_json::from_str fails (bad key chars) → revert is silently skipped
}

// ✅ FIX — construct the enum variant directly, no string formatting needed
TweakOperation::NetworkInterfacesSet { key, value: _ } => {
    println!("  -> Revert NetworkInterfacesSet: key={}", key);
    let revert_op = TweakOperation::NetworkInterfacesDelete { key: key.clone() };
    if let Err(e) = apply_network_interface_tweak(&revert_op) {
        eprintln!("Warning: Revert NetworkInterfacesSet failed: {}", e);
    }
}
```

---

## 🔴 BUG-H5 — `undo_tweak`: `context` MutexGuard is NOT dropped before `spawn_blocking(...).await`

**File:** `src-tauri/src/commands.rs` — `undo_tweak`
**Severity:** HIGH (deadlock risk)

In `apply_tweak`, the `context` lock was fixed with an explicit `{ }` scope (comment: *"Keep lock critical section short"*).
In `undo_tweak`, the `context` binding is declared at the same scope level as `backup_path`, `id_clone`, and the subsequent `spawn_blocking(...).await`. The `std::sync::MutexGuard` is held across the `await` point, which Tokio explicitly forbids and which will deadlock when another task (e.g., `check_category`) tries to acquire `ctx`.

```rust
// ❌ CURRENT — guard held across await in undo_tweak
pub async fn undo_tweak(...) -> Result<(), String> {
    let _ = app.emit("tweak-progress", ...);
    let tweak = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context.tweaks.iter().find(|t| t.id == id).cloned()
            .ok_or("Tweak ID not found")?
        // ← Is this brace actually here in the source? Verify and make it explicit.
    };
    let backup_path = ...; // if the above brace is missing, `context` is still alive here
    let id_clone = id.clone();
    tokio::task::spawn_blocking(move || { ... }).await??;  // ← deadlock if guard alive
    // ...
}

// ✅ FIX — guarantee the guard is released with an explicit labeled block
pub async fn undo_tweak(
    id: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id.clone(), "status": "reverting", "progress": 0
    }));

    // Lock acquired AND dropped inside this block
    let tweak = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context.tweaks.iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or("Tweak ID not found")?
    }; // ← MutexGuard dropped here, BEFORE any .await

    let backup_path = get_backup_dir()
        .map_err(|e| format!("Cannot determine backup directory: {}", e))?;
    let id_clone = id.clone();
    let app_clone = app.clone();

    tokio::task::spawn_blocking(move || {
        // ... revert logic
        Ok::<(), String>(())
    }).await.map_err(|e| e.to_string())??;

    // Second short lock for state update
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        if app_state.applied_tweaks.remove(&id) {
            let state_path = get_state_path()
                .unwrap_or_else(|_| std::path::PathBuf::from("state.json"));
            let _ = app_state.save(&state_path);
        }
    }

    let _ = app.emit("tweak-progress", serde_json::json!({
        "id": id, "status": "revert-success", "progress": 100
    }));
    Ok(())
}
```

---

## 🟡 BUG-M1 — `get_tweaks_fast` holds two `std::sync::Mutex` guards simultaneously

**File:** `src-tauri/src/commands.rs` — `get_tweaks_fast`
**Severity:** MEDIUM (potential deadlock if lock acquisition order ever diverges)

`get_tweaks_fast` acquires `ctx` lock and `state` lock simultaneously and holds both for the entire duration of `.iter().map().collect()`. If any other code path acquires these two locks in reverse order (state first, then ctx), the classic two-lock deadlock occurs.

```rust
// ❌ CURRENT — two guards held simultaneously
pub async fn get_tweaks_fast(
    ctx:   State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Tweak>, String> {
    let context   = ctx.lock().map_err(|e| e.to_string())?;    // guard 1 alive
    let app_state = state.lock().map_err(|e| e.to_string())?;  // guard 2 alive
    Ok(context.tweaks.iter().map(|t| {                         // both alive here
        let mut tweak = t.clone();
        tweak.enabled = app_state.applied_tweaks.contains(&tweak.id);
        tweak
    }).collect())
}  // both guards dropped here

// ✅ FIX — acquire, clone, drop, then process
pub async fn get_tweaks_fast(
    ctx:   State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Tweak>, String> {
    let tweaks = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        context.tweaks.clone()
    }; // guard 1 dropped

    let applied = {
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.applied_tweaks.clone()
    }; // guard 2 dropped

    Ok(tweaks.into_iter().map(|mut t| {
        t.enabled = applied.contains(&t.id);
        t
    }).collect())
}
```

---

## 🟡 BUG-M2 — `kill_tweak_process` silently ignores `taskkill` failure, always returns `Ok(())`

**File:** `src-tauri/src/commands.rs` — `kill_tweak_process`
**Severity:** MEDIUM

The result of `Command::new("taskkill").output()` is bound to `let _`, meaning permission errors, already-terminated processes, and invalid PIDs all return `Ok(())` to the caller. The UI shows success even when the process was not killed.

```rust
// ❌ CURRENT — result of taskkill is discarded
pub fn kill_tweak_process(
    id: String,
    proc_mgr: State<Mutex<ProcessManager>>,
) -> Result<(), String> {
    let pid = { let mgr = proc_mgr.lock()...; mgr.get_pid(&id) };
    if let Some(pid) = pid {
        let _ = Command::new("taskkill")   // ← result thrown away
            .args(["/F", "/PID", &pid.to_string(), "/T"])
            .creation_flags(0x08000000)
            .output();
    }
    Ok(())
}

// ✅ FIX — propagate taskkill errors
pub fn kill_tweak_process(
    id: String,
    proc_mgr: State<Mutex<ProcessManager>>,
) -> Result<(), String> {
    let pid = {
        let mgr = proc_mgr.lock().map_err(|e| e.to_string())?;
        mgr.get_pid(&id)
    };
    if let Some(pid) = pid {
        println!("[PROCESS KILL] Killing PID {} for tweak: {}", pid, id);
        let output = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string(), "/T"])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| format!("taskkill spawn failed: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "taskkill failed (PID {}): {}",
                pid, stderr.trim()
            ));
        }
    } else {
        println!("[PROCESS KILL] No active PID found for tweak: {}", id);
    }
    Ok(())
}
```

---

## 🟡 BUG-M3 — `startup/tasks.rs` still uses PowerShell for scheduled task enumeration

**File:** `src-tauri/src/modules/startup/tasks.rs` — `scan()`
**Severity:** MEDIUM (~500ms overhead on every startup scan; same class as the already-fixed `services.rs`)

`services.rs` was migrated from WMI/PowerShell to `winreg` (50-500x faster).
`tasks.rs` still spawns `powershell.exe -Command "Get-ScheduledTask..."` on every call to `scan_startup`.

```rust
// ❌ CURRENT — PowerShell for task enumeration (~500ms per call)
pub fn scan() -> Vec<StartupItem> {
    let script = r#"
        $tasks = Get-ScheduledTask | Where-Object { $_.State -ne 'Disabled' ...}
        $tasks | ConvertTo-Json -Compress
    "#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .creation_flags(0x08000000)
        .output()
        .unwrap_or_default();
    // parse JSON ...
}

// ✅ FIX — read from registry directly (~1-5ms)
// Scheduled task data is cached in:
//   HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tasks   (GUID→metadata)
//   HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tree    (path hierarchy)
use winreg::{enums::*, RegKey};

pub fn scan() -> Vec<StartupItem> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let tasks_root = match hklm.open_subkey(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tasks"
    ) {
        Ok(k) => k,
        Err(_) => return vec![],
    };

    tasks_root.enum_keys().flatten().filter_map(|guid| {
        let sub = tasks_root.open_subkey(&guid).ok()?;
        let path: String = sub.get_value("Path").ok()?;

        // DynamicInfo is a binary blob; byte 8 is the enabled flag (0 = disabled, 1 = enabled)
        let enabled = sub.get_raw_value("DynamicInfo")
            .map(|v| v.bytes.get(8).copied().unwrap_or(1) != 0)
            .unwrap_or(true);

        let name = path.split('\\').last().unwrap_or(&path).to_string();
        let command: String = sub.get_value("Actions").unwrap_or_default(); // simplified

        let (publisher, description) = utils::get_file_info(&command);
        let rating = assess_safety(&command, publisher.as_deref());

        Some(StartupItem {
            id:            format!("TASK:{}", path),
            name,
            category:      "Scheduled Task".to_string(),
            subcategory:   String::new(),
            location:      "Task Scheduler".to_string(),
            command,
            enabled,
            publisher,
            description,
            source:        AutostartSource::TaskScheduler,
            safety_rating: rating,
            file_exists:   true,
        })
    }).collect()
}
```

---

## 🟡 BUG-M4 — `TweakCheckScheduledTaskDisabled` with empty `name` always returns wrong result

**File:** `src-tauri/src/modules/debloat/tasks.rs` — `get_task_tweaks()`
**Severity:** MEDIUM (wrong UI state for the "Disable Misc Scheduled Tasks" toggle)

The `check` field for the `debloat-disable-misc-tasks` tweak uses `TweakCheckScheduledTaskDisabled { name: "".to_string() }`. The `check_tweak_enabled` match arm runs `schtasks /query /tn "" /v /fo list` which always fails (empty task name is invalid), so `outstr.contains("Disabled")` is always `false`. The toggle will never show as "applied" even after all tasks are disabled.

```rust
// ❌ CURRENT — empty name, schtasks always fails, toggle always shows as OFF
Tweak {
    id: "debloat-disable-misc-tasks".to_string(),
    check: Some(TweakCheckScheduledTaskDisabled {
        name: "".to_string(),  // ← schtasks /query /tn "" → error
    }),
    operations: vec![
        TweakOperation::ScheduledTaskDisable { path: "".to_string(), name: "MapsToastTask".to_string() },
        TweakOperation::ScheduledTaskDisable { path: "".to_string(), name: "MapsUpdateTask".to_string() },
        // ...8 tasks total
    ],
    // ...
}

// ✅ FIX — check ALL tasks with TweakCheckMultiScheduledTaskDisabled
// Option A: check just the first task (proxy for the group)
check: Some(TweakCheckScheduledTaskDisabled {
    name: "MapsToastTask".to_string(),  // representative task
}),

// Option B (correct): add TweakCheckMultiScheduledTaskDisabled variant to types.rs
// In types.rs:
// MultiScheduledTaskDisabled { names: Vec<String> },
//
// In check_tweak_enabled():
// TweakCheck::MultiScheduledTaskDisabled { names } => {
//     names.iter().all(|name| {
//         let output = Command::new("schtasks")
//             .args(["/query", "/tn", name, "/v", "/fo", "list"])
//             .output().unwrap_or_default();
//         let s = String::from_utf8_lossy(&output.stdout);
//         s.contains("Disabled") || s.contains("Disabilitato")
//     })
// }
//
// check: Some(TweakCheckMultiScheduledTaskDisabled {
//     names: vec![
//         "MapsToastTask".to_string(), "MapsUpdateTask".to_string(),
//         "SpeechModelDownloadTask".to_string(), /* ... */
//     ],
// }),
```

---

## 🟡 BUG-M5 — `println!` / `eprintln!` throughout production build

**File:** `src-tauri/src/commands.rs` — entire file
**Severity:** MEDIUM (sensitive data in stdout for release builds; `log` crate in `Cargo.toml` but unused)

No change from repo-4. Hundreds of `println!`/`eprintln!` calls expose registry paths, service names, DNS IPs, and PowerShell output in any terminal or log capture, even in release builds.

```rust
// ❌ CURRENT — debug output always active
println!("[TWEAK APPLY] Applying ID: {}", tweak_clone.id);
println!("  -> RegistrySet: {}\\{}\\{} = {:?}", root_key, path, key, value);
eprintln!("  -> Registry Error: {:?}", e);

// ✅ FIX — use log crate (already in Cargo.toml: log = "0.4", env_logger = "0.10")
use log::{debug, warn, error};

debug!("[TWEAK APPLY] Applying: {}", id);
debug!("  -> RegistrySet: {}\\{}\\{}", root_key, path, key);
// Do NOT log the value — may contain sensitive registry data
warn!("  -> Registry write warning: {}", e);
error!("  -> Registry Error: {}", e);

// In lib.rs run() — initialize logger only in debug builds:
#[cfg(debug_assertions)]
env_logger::Builder::from_env(
    env_logger::Env::default().default_filter_or("debug")
).init();
```

---

## 🟢 BUG-L1 — `commands.rs` is still a God Object (~1000+ lines)

**File:** `src-tauri/src/commands.rs`
**Severity:** LOW (maintainability / SRP violation; no behavior impact)

The file structure is unchanged from repo-4. All DNS helpers, NIC helpers, MSI helpers, Defender helpers, SvcHost helpers, registry check functions, and Tauri command handlers remain in a single file.

```
// ✅ FIX — proposed split (same as previous audit):
src-tauri/src/
├── commands/
│   ├── mod.rs          // pub re-exports of all #[tauri::command] fns
│   ├── tweaks.rs       // get_tweaks, get_tweaks_fast, check_category, apply_tweak, undo_tweak
│   ├── startup.rs      // scan_startup, set_startup_item_enabled
│   ├── dns.rs          // benchmark_dns, apply_dns_server
│   └── process.rs      // kill_tweak_process
├── helpers/
│   ├── registry.rs     // check_registry_value, check_registry_key_absent
│   ├── nic.rs          // nic_subkey_paths, set_nic_property, check_nic_property
│   ├── msi.rs          // check_msi_enabled_globally, apply_msi_set, apply_msi_remove
│   └── defender.rs     // control_defender_services, set_defender_exclusions
```

---

## Priority Fix Order

| Priority | Bug | Severity | Effort |
|----------|-----|----------|--------|
| **1** | BUG-C1 — monitoring.rs compile errors | 🔴 BLOCKING | Low |
| **2** | BUG-H5 — undo_tweak MutexGuard across await | 🔴 HIGH | Low |
| **3** | BUG-H1 — safe_path fails on new files | 🔴 HIGH | Medium |
| **4** | BUG-H3 — check_powershell_output uses Bypass | 🔴 HIGH | Low |
| **5** | BUG-H2 — check path bypasses validate_command | 🔴 HIGH | Low |
| **6** | BUG-H4 — NetworkInterfacesSet JSON injection | 🔴 HIGH | Low |
| **7** | BUG-M1 — get_tweaks_fast dual-lock | 🟡 MEDIUM | Low |
| **8** | BUG-M2 — kill_tweak_process ignores errors | 🟡 MEDIUM | Low |
| **9** | BUG-M4 — empty name in TaskDisabled check | 🟡 MEDIUM | Low |
| **10** | BUG-M3 — tasks.rs still uses PowerShell | 🟡 MEDIUM | Medium |
| **11** | BUG-M5 — println!/eprintln! in production | 🟡 MEDIUM | Medium |
| **12** | BUG-L1 — God Object | 🟢 LOW | High |
````

Totale: **12 bug** (1 blocking compilation, 5 high, 5 medium, 1 low).