# TommyTweaker — AI Advisor: Full Implementation Guide

> **Model:** `gemini-3-flash-preview` (Gemini 3 Flash, free tier on AI Studio)  
> **API endpoint:** `https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent`

---

## 0. Cargo.toml

**Root cause:** TommyTweaker runs elevated (admin). When elevated,
`%APPDATA%` resolves to the admin profile
(`C:\Windows\system32\config\systemprofile\AppData\Roaming`) which is
read-protected. The existing fallback path via `current_dir()` also fails
when launched from a protected directory.

**Fix:** use `%PROGRAMDATA%` (`C:\ProgramData`) which is always
writable by elevated processes.

```rust
// src/modules/utils/dirs.rs — replace get_app_dir() entirely

pub fn get_app_dir() -> Result<PathBuf, String> {
    // Primary: %PROGRAMDATA%\TommyTweaker — always writable when elevated
    if let Ok(program_data) = std::env::var("PROGRAMDATA") {
        let dir = PathBuf::from(program_data).join("TommyTweaker");
        if ensure_writable_dir(&dir) {
            return Ok(dir);
        }
    }
    // Fallback 1: %APPDATA%\TommyTweaker (works for non-elevated)
    if let Ok(app_data) = std::env::var("APPDATA") {
        let dir = PathBuf::from(app_data).join("TommyTweaker");
        if ensure_writable_dir(&dir) {
            return Ok(dir);
        }
    }
    // Fallback 2: next to the executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let dir = parent.join("TommyTweakerData");
            if ensure_writable_dir(&dir) {
                return Ok(dir);
            }
        }
    }
    Err("Cannot find a writable directory for TommyTweaker data.".to_string())
}

/// Creates the directory if needed, then does a write-test.
/// Returns true only if the directory is confirmed writable.
fn ensure_writable_dir(dir: &PathBuf) -> bool {
    if !dir.exists() {
        if std::fs::create_dir_all(dir).is_err() {
            return false;
        }
    }
    let test = dir.join(".writetest");
    let ok = std::fs::write(&test, "ok").is_ok();
    let _ = std::fs::remove_file(&test);
    ok
}

// get_backup_dir, get_state_path, get_logs_dir stay the same —
// they all call get_app_dir() so the fix propagates automatically.
pub fn get_backup_dir() -> Result<PathBuf, String> {
    let dir = get_app_dir()?.join("backups");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn get_state_path() -> Result<PathBuf, String> {
    Ok(get_app_dir()?.join("state.json"))
}

pub fn get_logs_dir() -> Result<PathBuf, String> {
    let dir = get_app_dir()?.join("logs");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}
```

---

## Fix 2 — MsiRemoveNet: "Failed to open PCI class Net" (os error 2)

The `"Net"` class key does not exist by default on all machines.
Make the function skip silently instead of erroring.

```rust
// In the function apply_msi_remove_class (and apply_msi_set_class) — same fix:

fn apply_msi_remove_class(class: &str) -> Result<(), String> {
    let class_key_path = format!("{}{}", PCI_PATH, class);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // BEFORE: .map_err(|e| format!("Failed to open PCI class {}: {}", class, e))?
    // AFTER: silent skip — class may simply not exist on this machine
    let class_key = match hklm.open_subkey_with_flags(&class_key_path, KEY_READ) {
        Ok(k) => k,
        Err(_) => return Ok(()), // ← not an error, just not present
    };
    for dev_name in class_key.enum_keys().flatten() {
        let msi_path = format!(
            "{}{}\\{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
            PCI_PATH, class, dev_name
        );
        if let Ok(msi_key) = hklm.open_subkey_with_flags(&msi_path, KEY_SET_VALUE) {
            let _ = msi_key.delete_value("MSISupported");
            let _ = msi_key.delete_value("MessageNumberLimit");
            let _ = msi_key.delete_value("Priority");
        }
    }
    Ok(())
}
```

---

## Fix 3 — bcdedit "Element not found" is not a real error

`bcdedit /deletevalue useplatformclock` returns non-zero when the value
was never set — meaning the system is already in the desired state.

```rust
// In apply_tweak and undo_tweak, inside TweakOperation::Command:

TweakOperation::Command { cmd, args } => {
    let output = Command::new(cmd)
        .args(args)
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| format!("Command exec failed: {}", e))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}{}", stdout, stderr).to_lowercase();

        // bcdedit: "element not found" = already in target state, not a failure
        // bcdedit: "the value is protected" = secure boot, warn but don't fail
        if combined.contains("element not found")
            || combined.contains("value is protected")
        {
            eprintln!("[CMD] Non-fatal: {} {:?} → {}", cmd, args, combined.trim());
            // continue — already in the correct state
        } else {
            return Err(format!(
                "Command returned non-zero {:?}: {}",
                output.status.code(),
                combined.trim()
            ));
        }
    }
}
```

---

## Fix 4 — Revert warnings surfaced in the UI

### commands.rs — collect warnings instead of silently printing

```rust
// In undo_tweak, replace the revert loop with a warning-collecting version:

let mut warnings: Vec<String> = Vec::new();

// For every non-blocking revert op, use this pattern:
// (example for MsiRemoveNet, DefenderServiceControl, etc.)
TweakOperation::MsiRemoveNet => {
    println!("  -> Revert MsiRemoveNet");
    if let Err(e) = apply_msi_remove_net() {
        warnings.push(format!("MsiRemoveNet: {}", e));
        // do NOT return Err — it's non-blocking
    }
}
TweakOperation::DefenderServiceControl { services, action } => {
    if let Err(e) = control_defender_services(services, action) {
        warnings.push(format!("DefenderServiceControl({}): {}", action, e));
    }
}
// ... same pattern for all other non-blocking revert ops

// At the END of the blocking closure, emit different events:
if warnings.is_empty() {
    app.emit("tweak-progress", serde_json::json!({
        "id": id, "status": "revert-success", "progress": 100
    })).ok();
} else {
    app.emit("tweak-progress", serde_json::json!({
        "id": id,
        "status": "revert-partial",
        "progress": 100,
        "warnings": warnings
    })).ok();
}
Ok(())
```

### TweakCard.svelte — show warnings inline

```svelte
<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { AlertTriangle } from "lucide-svelte";
  import { onMount } from "svelte";

  export let tweak: any;

  let revertWarnings: string[] = [];
  let showWarnings = false;

  onMount(async () => {
    await listen("tweak-progress", (e: any) => {
      const p = e.payload;
      if (p.id !== tweak.id) return;
      if (p.status === "revert-partial") {
        revertWarnings = p.warnings ?? [];
      } else if (p.status === "revert-success") {
        revertWarnings = [];
      }
    });
  });
</script>

<!-- Add below the tweak toggle/status, inside the tweak row: -->
{#if revertWarnings.length > 0}
  <div class="revert-warn">
    <button class="warn-header" onclick={() => showWarnings = !showWarnings}>
      <AlertTriangle size={11} />
      Reverted with {revertWarnings.length} warning{revertWarnings.length > 1 ? "s" : ""}
      <span class="chevron">{showWarnings ? "▲" : "▼"}</span>
    </button>
    {#if showWarnings}
      <ul class="warn-list">
        {#each revertWarnings as w}<li>{w}</li>{/each}
      </ul>
    {/if}
  </div>
{/if}

<style>
  .revert-warn {
    margin-top: 4px;
    border-radius: var(--radius-sm);
    background: rgba(251,191,36,0.07);
    border: 1px solid rgba(251,191,36,0.15);
    overflow: hidden;
  }
  .warn-header {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 5px 10px;
    background: none;
    border: none;
    color: #fbbf24;
    font-size: 11px;
    cursor: pointer;
    text-align: left;
  }
  .warn-header :global(svg) { flex-shrink: 0; }
  .chevron { margin-left: auto; font-size: 9px; }
  .warn-list {
    list-style: none;
    padding: 0 10px 8px 24px;
    margin: 0;
  }
  .warn-list li {
    font-size: 10px;
    color: var(--text-muted);
    padding: 1px 0;
    font-family: monospace;
  }
  .warn-list li::before { content: "→ "; color: #fbbf24; }
</style>
```

---

## Fix 5 — AI: exhaustive + stable scan

```rust
// commands.rs — replace ai_analyze tweak collection:

#[tauri::command]
pub async fn ai_analyze(
    ctx: tauri::State<'_, std::sync::Mutex<TweakContext>>,
    state: tauri::State<'_, std::sync::Mutex<crate::modules::utils::state::AppState>>,
) -> Result<AnalysisResult, String> {
    let profile = profiler::scan_system_profile()?;
    let profile_json = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;

    let tweaks_summary = {
        let context = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        let mut tweaks: Vec<_> = context.tweaks.iter().map(|t| {
            serde_json::json!({
                "id": t.id,
                "name": t.name,
                "description": t.description,
                "risk_level": format!("{:?}", t.warninglevel),
                "category": format!("{:?}", t.category),
                "currently_applied": st.applied_tweaks.contains(&t.id),
                "requires_restart": t.requiresrestart,
            })
        }).collect();
        // Stable sort: category → id (same order every run = deterministic AI output)
        tweaks.sort_by(|a, b| {
            let ca = a["category"].as_str().unwrap_or("");
            let cb = b["category"].as_str().unwrap_or("");
            ca.cmp(cb).then(
                a["id"].as_str().unwrap_or("").cmp(b["id"].as_str().unwrap_or(""))
            )
        });
        tweaks
    };

    let tweaks_json = serde_json::to_string(&tweaks_summary).map_err(|e| e.to_string())?;
    let prompt = prompts::build_analyze_prompt(&profile_json, &tweaks_json);

    let (ctx_user, ctx_model) = gemini::build_context_injection(&profile_json, &tweaks_json);
    let messages = vec![
        ("user".to_string(),  ctx_user),
        ("model".to_string(), ctx_model),
        ("user".to_string(),  prompt),
    ];

    let raw = gemini::call_gemini(messages, true).await?;
    serde_json::from_str::<AnalysisResult>(&raw)
        .map_err(|e| format!("Failed to parse AI response: {}\n\nRaw: {:.300}", e, raw))
}
```

```rust
// prompts.rs — exhaustive prompt with strict instructions:

pub fn build_analyze_prompt(profile_json: &str, tweaks_json: &str) -> String {
    format!(r#"You are a Windows optimization expert. Analyze this system EXHAUSTIVELY.

SYSTEM PROFILE:
{profile_json}

ALL AVAILABLE TWEAKS (sorted by category then id — evaluate every single one):
{tweaks_json}

INSTRUCTIONS — follow exactly, no exceptions:
1. Go through EVERY tweak in the list above one by one. Do not skip any.
2. For each tweak decide: ADD (not applied, beneficial for this hardware), REMOVE (applied but counterproductive), or ignore.
3. Only ADD tweaks genuinely useful for THIS specific CPU/GPU/RAM/connection.
4. Only REMOVE tweaks that are applied AND actively harmful (e.g. Intel tweak on AMD, laptop tweak on desktop, conflicting pair).
5. Only report CONFLICTS between tweaks that are BOTH currently_applied=true AND directly interfere.
6. Be deterministic: same system data must always produce the same output.
7. system_summary: 2-3 sentences referencing actual values (CPU model, RAM GB, GPU name).
8. If ram_gb is 0 the profiler failed — skip all RAM-specific recommendations.

Return ONLY valid JSON, no markdown fences, no extra text:
{{
  "system_summary": "string",
  "add":       [{{"id":"string","priority":"high|medium|low","reason":"one sentence with specific hardware data"}}],
  "remove":    [{{"id":"string","priority":"high|medium|low","reason":"why counterproductive on this exact hardware"}}],
  "conflicts": [{{"tweak_ids":["string","string"],"issue":"what the conflict causes"}}]
}}"#)
}
```

---

## Fix 6 — RAM reading (profiler.rs)

```rust
// Replace both get_ram_gb() and get_ram_usage_pct() with Win32 direct calls.
// No PowerShell, no wmic, no registry. Same API as Task Manager.

fn get_ram_gb() -> u32 {
    unsafe {
        let mut ms = MemoryStatusEx::zeroed();
        if GlobalMemoryStatusEx(&mut ms) != 0 {
            return (ms.ull_total_phys / 1_073_741_824) as u32;
        }
    }
    // Fallback: wmic
    std::process::Command::new("wmic")
        .args(["ComputerSystem", "get", "TotalPhysicalMemory"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            s.lines().nth(1)?.trim().parse::<u64>().ok()
             .map(|b| (b / 1_073_741_824) as u32)
        })
        .unwrap_or(0)
}

fn get_ram_usage_pct() -> f32 {
    unsafe {
        let mut ms = MemoryStatusEx::zeroed();
        if GlobalMemoryStatusEx(&mut ms) != 0 {
            // dw_memory_load is already 0-100, filled by Windows directly
            return ms.dw_memory_load as f32;
        }
    }
    0.0
}

// Win32 struct — add at the top of profiler.rs
#[repr(C)]
struct MemoryStatusEx {
    dw_length: u32,
    dw_memory_load: u32,
    ull_total_phys: u64,
    ull_avail_phys: u64,
    ull_total_page_file: u64,
    ull_avail_page_file: u64,
    ull_total_virtual: u64,
    ull_avail_virtual: u64,
    ull_avail_extended_virtual: u64,
}

impl MemoryStatusEx {
    fn zeroed() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.dw_length = std::mem::size_of::<Self>() as u32;
        s
    }
}

extern "system" {
    fn GlobalMemoryStatusEx(lp_buffer: *mut MemoryStatusEx) -> i32;
}
```

---

## Fix 7 — Markdown rendered in ChatPanel

```bash
npm install marked
```

```svelte
<!-- ChatPanel.svelte -->
<script lang="ts">
  import { marked } from "marked";
  marked.setOptions({ breaks: true, gfm: true });

  function md(text: string): string {
    return marked.parse(text) as string;
  }
</script>

<!-- Replace plain text bubble with: -->
<div class="msg-content md-body">
  {@html md(msg.content)}
</div>
```

```css
/* Inside ChatPanel <style> — scoped markdown */
:global(.md-body p)           { margin: 0 0 8px 0; }
:global(.md-body p:last-child){ margin-bottom: 0; }
:global(.md-body strong)      { color: var(--text-color); font-weight: 600; }
:global(.md-body em)          { color: var(--text-secondary); font-style: italic; }
:global(.md-body code) {
  background: rgba(255,255,255,0.08);
  border-radius: 4px;
  padding: 1px 5px;
  font-family: "Cascadia Code", "Consolas", monospace;
  font-size: 12px;
  color: var(--accent-hover);
}
:global(.md-body pre) {
  background: rgba(0,0,0,0.25);
  border: var(--border-glass);
  border-radius: var(--radius-md);
  padding: 12px 14px;
  overflow-x: auto;
  margin: 8px 0;
}
:global(.md-body pre code) { background: none; padding: 0; color: var(--text-secondary); }
:global(.md-body ul), :global(.md-body ol) { padding-left: 18px; margin: 4px 0 8px; }
:global(.md-body li)  { margin-bottom: 3px; font-size: 13px; }
:global(.md-body h3)  { font-size: 14px; font-weight: 600; margin: 10px 0 4px; }
:global(.md-body a)   { color: var(--accent-color); text-decoration: none; }
:global(.md-body a:hover) { text-decoration: underline; }
```

---

## Fix 8 — Conflicts section UI (no more yellow block)

```svelte
<!-- ScanPanel.svelte — replace conflicts block -->
{#if analysis.conflicts.length}
  <div class="section-label">
    <AlertTriangle size={11} /> Conflicts detected
  </div>
  {#each analysis.conflicts as c}
    <div class="conflict-row">
      <AlertTriangle size={12} />
      <div class="conflict-body">
        <span>{c.issue}</span>
        <span class="conflict-ids">{c.tweak_ids.join(" + ")}</span>
      </div>
    </div>
  {/each}
{/if}
```

```css
.section-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.6px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding-bottom: 8px;
  border-bottom: var(--border-glass);
  margin-top: 8px;
}
.conflict-row {
  display: grid;
  grid-template-columns: 14px 1fr;
  gap: 10px;
  align-items: start;
  padding: 9px 12px;
  background: var(--layer-card);
  border: var(--border-glass);
  border-left: 2px solid rgba(251,191,36,0.4);
  border-radius: var(--radius-md);
  font-size: 12px;
}
.conflict-row :global(svg) { color: #fbbf24; margin-top: 1px; }
.conflict-body { display: flex; flex-direction: column; gap: 2px; }
.conflict-body span { color: var(--text-secondary); line-height: 1.4; }
.conflict-ids { font-size: 10px; color: var(--text-muted); font-family: monospace; }
```

---

## Fix 9 — Recommendation rows (custom checkboxes, no native input)

```svelte
<!-- ScanPanel.svelte — replace rec-row markup -->
{#each recs as rec}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    class="rec-row"
    class:selected={rec.selected}
    class:danger={section === 'remove'}
    role="checkbox"
    aria-checked={rec.selected}
    tabindex="0"
    onclick={() => rec.selected = !rec.selected}
    onkeydown={(e) => e.key === " " && (rec.selected = !rec.selected)}
  >
    <div class="rec-check" class:checked={rec.selected}>
      {#if rec.selected}<Check size={9} />{/if}
    </div>
    <div class="rec-body">
      <div class="rec-header">
        <code class="rec-id">{rec.id}</code>
        <span class="rec-priority {rec.priority}">{rec.priority}</span>
      </div>
      <p class="rec-reason">{rec.reason}</p>
    </div>
  </div>
{/each}
```

```css
.rec-row {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 10px 14px;
  background: var(--layer-card);
  border: 1px solid rgba(255,255,255,0.06);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: border-color 0.15s, background 0.15s;
  user-select: none;
  outline: none;
}
.rec-row:hover        { background: var(--layer-hover); border-color: rgba(255,255,255,0.1); }
.rec-row:focus-visible{ border-color: var(--accent-color); }
.rec-row.selected     { border-color: rgba(129,140,248,0.28); background: rgba(129,140,248,0.05); }
.rec-row.danger.selected { border-color: rgba(248,113,113,0.22); background: rgba(248,113,113,0.04); }

.rec-check {
  width: 16px; height: 16px;
  border-radius: 4px;
  border: 1px solid rgba(255,255,255,0.18);
  background: rgba(255,255,255,0.04);
  flex-shrink: 0;
  margin-top: 2px;
  display: flex; align-items: center; justify-content: center;
  transition: all 0.12s;
  color: white;
}
.rec-check.checked          { background: var(--accent-color); border-color: var(--accent-color); }
.danger .rec-check.checked  { background: #f87171;             border-color: #f87171; }

.rec-body { flex: 1; min-width: 0; }
.rec-header { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; flex-wrap: wrap; }
.rec-id     { font-size: 12px; font-family: monospace; color: var(--text-secondary); font-weight: 600; }
.rec-reason { margin: 0; font-size: 12px; color: var(--text-muted); line-height: 1.5; }

.rec-priority {
  font-size: 10px; font-weight: 700;
  padding: 1px 6px; border-radius: 999px;
  text-transform: uppercase; letter-spacing: 0.3px;
}
.rec-priority.high   { background: rgba(248,113,113,0.12); color: #f87171; }
.rec-priority.medium { background: rgba(251,191,36,0.12);  color: #fbbf24; }
.rec-priority.low    { background: rgba(52,211,153,0.12);  color: #34d399; }

/* Apply bar */
.action-bar {
  display: flex; justify-content: flex-end;
  padding-top: 12px;
  border-top: var(--border-glass);
}
```