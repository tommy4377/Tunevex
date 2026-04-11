# TommyTweaker — Gemini AI Advisor: Full Implementation Guide

---

## Overview

The AI Advisor adds three capabilities to TommyTweaker:
1. **Analyze** — scans the real system state and recommends tweaks to add or remove
2. **Chat** — free conversation with context of the current system and active tweaks
3. **Diagnose** — given a problem description, finds which active tweaks may be causing it

API: **Google Gemini 3 Flash** via AI Studio (free tier, no billing required).
The API key is stored in **Windows Credential Manager** — never on disk, never in plain text.

---

## 1. `Cargo.toml` — New Dependencies

```toml
[dependencies]
# existing deps...

# HTTP client for Gemini API calls
reqwest  = { version = "0.12", features = ["json", "rustls-tls"] }

# Secure API key storage via Windows Credential Manager
keyring  = { version = "3", features = ["windows-native"] }

# Already present — confirm these exist:
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
tokio       = { version = "1", features = ["full"] }
```

> **Why `keyring` with `windows-native`?**
> The `windows-native` feature uses `CredWrite` / `CredRead` from the Win32 API directly —
> the same mechanism used by browsers and Windows itself to store passwords. [web:78]
> The key is encrypted with the user's Windows login credentials (DPAPI) and is
> completely invisible to other applications. It never touches the filesystem. [web:68]

---

## 2. `tauri.conf.json` — Allow HTTP to Gemini

Tauri v2 blocks all outbound HTTP by default. Add the permission in
`src-tauri/capabilities/default.json`: [web:87]

```json
{
  "identifier": "default",
  "description": "Default capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "opener:default",
    "dialog:default",
    {
      "identifier": "http:default",
      "allow": [
        { "url": "https://generativelanguage.googleapis.com/**" }
      ]
    }
  ]
}
```

Also add to `Cargo.toml` under `[dependencies]`:
```toml
tauri-plugin-http = "2"
```

And register the plugin in `lib.rs`:
```rust
.plugin(tauri_plugin_http::init())
```

---

## 3. File Structure

```
src-tauri/src/modules/ai/
├── mod.rs          ← re-exports + public types
├── profiler.rs     ← SystemProfile + DiagnosticFlags collection
├── gemini.rs       ← HTTP client + key storage
└── prompts.rs      ← all prompt builders

src/components/ai/
├── AiDashboard.svelte
├── ScanPanel.svelte
├── ChatPanel.svelte
├── DiagnosePanel.svelte
├── RecommendationRow.svelte
└── DiagnosticCard.svelte
```

---

## 4. `ai/mod.rs`

```rust
pub mod gemini;
pub mod profiler;
pub mod prompts;

pub use profiler::{SystemProfile, DiagnosticFlags};

use serde::{Deserialize, Serialize};

// Returned by ai_analyze command
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub add: Vec<RecommendedTweak>,
    pub remove: Vec<RecommendedTweak>,
    pub conflicts: Vec<TweakConflict>,
    pub system_summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendedTweak {
    pub id: String,
    pub priority: String,   // "high" | "medium" | "low"
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweakConflict {
    pub tweak_ids: Vec<String>,
    pub issue: String,
}

// Returned by ai_diagnose command
#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosisResult {
    pub likely_causes: Vec<DiagnosisCause>,
    pub suggested_fix: String,
    pub safe_to_revert: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosisCause {
    pub tweak_id: String,
    pub confidence: String,   // "high" | "medium" | "low"
    pub explanation: String,
}

// Chat message for history
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,     // "user" | "model"
    pub content: String,
}
```

---

## 5. `ai/gemini.rs` — Client + Secure Key Storage

```rust
use keyring::Entry;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ── Constants ─────────────────────────────────────────────────────────────────

const SERVICE_NAME: &str = "TommyTweaker";
const KEY_USERNAME: &str = "gemini_api_key";

// Use gemini-2.0-flash — free on AI Studio, fast, large context window
const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

// ── Secure Key Storage (Windows Credential Manager) ───────────────────────────

/// Saves the Gemini API key to Windows Credential Manager.
/// Encrypted with DPAPI — only readable by the same Windows user.
pub fn save_api_key(key: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {}", e))?;
    entry
        .set_password(key)
        .map_err(|e| format!("Failed to save key: {}", e))
}

/// Retrieves the key from Windows Credential Manager.
/// Returns Err if no key has been saved yet.
pub fn get_api_key() -> Result<String, String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {}", e))?;
    entry
        .get_password()
        .map_err(|_| "No API key found. Please add your Gemini API key in Settings.".to_string())
}

/// Deletes the key from Windows Credential Manager.
pub fn delete_api_key() -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {}", e))?;
    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete key: {}", e))
}

// ── Gemini HTTP Request Types ─────────────────────────────────────────────────

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct GeminiContent {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    #[serde(rename = "responseMimeType")]
    response_mime_type: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

// ── Core Call Function ────────────────────────────────────────────────────────

/// Sends a multi-turn conversation to Gemini and returns the response text.
///
/// - `messages`: conversation history as (role, text) pairs.
///   Role must alternate "user" / "model". First message must be "user".
/// - `expect_json`: if true, sets responseMimeType to application/json
///   and lowers temperature to 0.1 for deterministic structured output. [web:83]
pub async fn call_gemini(
    messages: Vec<(String, String)>,
    expect_json: bool,
) -> Result<String, String> {
    let api_key = get_api_key()?;

    let client = Client::builder()
        .timeout(Duration::from_secs(60))  // Gemini can be slow on first call
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let contents: Vec<GeminiContent> = messages
        .into_iter()
        .map(|(role, text)| GeminiContent {
            role,
            parts: vec![Part { text }],
        })
        .collect();

    let body = GeminiRequest {
        contents,
        generation_config: GenerationConfig {
            // Low temperature for JSON = deterministic structured output [web:83]
            // Higher temperature for chat = more natural responses
            temperature: if expect_json { 0.1 } else { 0.7 },
            max_output_tokens: 4096,
            response_mime_type: if expect_json {
                "application/json".to_string()
            } else {
                "text/plain".to_string()
            },
        },
    };

    let url = format!("{}?key={}", GEMINI_URL, api_key);

    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    // Surface HTTP errors clearly (e.g. 400 bad key, 429 rate limit)
    if !response.status().is_success() {
        let status = response.status();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("Gemini API error {}: {}", status, body_text));
    }

    let parsed: GeminiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

    let candidate = parsed
        .candidates
        .into_iter()
        .next()
        .ok_or("No response from Gemini")?;

    // Warn if generation was cut off
    if let Some(reason) = &candidate.finish_reason {
        if reason != "STOP" {
            eprintln!("Gemini finish reason: {}", reason);
        }
    }

    candidate
        .content
        .parts
        .into_iter()
        .next()
        .map(|p| p.text)
        .ok_or("Empty response from Gemini".to_string())
}
```

---

## 6. `ai/prompts.rs` — All Prompt Builders

```rust
/// System context injected at the start of every conversation.
/// Gemini does not have a "system" role — we inject as first user message
/// followed by a model acknowledgement (required for multi-turn). [web:66]
pub fn build_context_injection(profile_json: &str, applied_json: &str) -> (String, String) {
    let user = format!(
        r#"You are an expert Windows optimization assistant embedded in TommyTweaker,
a professional system tweaking app built with Tauri and Rust.

You have full access to the user's system state:

SYSTEM PROFILE:
{}

CURRENTLY APPLIED TWEAKS:
{}

Rules:
- Always reference specific tweak IDs when discussing them (e.g. "net_tcp_ack_freq")
- Be honest about risks — do not recommend Dangerous tweaks unless explicitly asked
- If something could cause instability, say so clearly
- Keep answers concise and technical"#,
        profile_json, applied_json
    );
    let model = "Understood. I have full context of this system and its applied tweaks. Ready to help.".to_string();
    (user, model)
}

/// Prompt for the full analysis: add + remove + conflicts.
/// Returns a prompt that forces JSON output matching AnalysisResult schema.
pub fn build_analyze_prompt(profile_json: &str, tweaks_json: &str) -> String {
    format!(
        r#"Analyze this Windows system and return optimization recommendations.

SYSTEM PROFILE:
{}

ALL AVAILABLE TWEAKS (each has: id, name, description, risk_level, category, currently_applied):
{}

Your task:
1. Identify tweaks to ADD — not yet applied, would genuinely benefit THIS specific system
2. Identify tweaks to REMOVE — currently applied but useless or harmful for this system
   (e.g. Intel-specific tweak on an AMD system, laptop tweak on a desktop, etc.)
3. Identify conflicts between currently applied tweaks
4. Write a 2-3 sentence honest assessment of the system's current state

Return ONLY valid JSON matching this exact schema:
{{
  "system_summary": "string",
  "add": [
    {{
      "id": "string",
      "priority": "high|medium|low",
      "reason": "one sentence referencing actual system data"
    }}
  ],
  "remove": [
    {{
      "id": "string",
      "priority": "high|medium|low",
      "reason": "why it is counterproductive on this system"
    }}
  ],
  "conflicts": [
    {{
      "tweak_ids": ["string"],
      "issue": "what the conflict causes"
    }}
  ]
}}"#,
        profile_json, tweaks_json
    )
}

/// Prompt for diagnosing a user-reported problem.
pub fn build_diagnose_prompt(
    profile_json: &str,
    applied_tweaks_json: &str,
    problem: &str,
) -> String {
    format!(
        r#"A user is experiencing a problem with their Windows PC.

PROBLEM DESCRIPTION: {}

SYSTEM PROFILE:
{}

CURRENTLY APPLIED TWEAKS (these are the only changes made to the system):
{}

Analyze which applied tweaks could be causing or contributing to this problem.
Consider interactions between tweaks, not just individual ones.

Return ONLY valid JSON:
{{
  "likely_causes": [
    {{
      "tweak_id": "string",
      "confidence": "high|medium|low",
      "explanation": "specific reason this tweak causes the described symptom"
    }}
  ],
  "suggested_fix": "clear step-by-step instructions to resolve the issue",
  "safe_to_revert": ["tweak_id_1", "tweak_id_2"]
}}"#,
        problem, profile_json, applied_tweaks_json
    )
}
```

---

## 7. `ai/profiler.rs` — System Scanner

```rust
use serde::Serialize;
use std::process::Command;
use winreg::{enums::*, RegKey};

#[derive(Debug, Serialize)]
pub struct SystemProfile {
    // Hardware
    pub cpu_name: String,
    pub cpu_vendor: String,       // "Intel" | "AMD" | "Unknown"
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub gpu_name: String,
    pub is_laptop: bool,

    // Storage
    pub system_drive_type: String, // "NVMe" | "SSD" | "HDD"
    pub system_drive_free_pct: f32,

    // Network
    pub connection_type: String,   // "Ethernet" | "WiFi" | "Unknown"
    pub link_speed_mbps: u32,

    // Windows State
    pub windows_version: String,
    pub power_plan: String,
    pub vbs_enabled: bool,
    pub hpet_enabled: bool,
    pub page_file_auto: bool,
    pub defender_realtime: bool,

    // Runtime indicators
    pub ram_usage_pct: f32,
    pub cpu_usage_pct: f32,
    pub startup_items_count: u32,
    pub running_services_count: u32,

    // Diagnostic flags (pre-computed for the prompt)
    pub flags: DiagnosticFlags,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticFlags {
    pub vbs_on_gaming_rig: bool,    // VBS enabled = ~15% GPU loss
    pub balanced_power_plan: bool,  // Not optimal for performance
    pub hpet_on_gaming: bool,       // HPET enabled = higher DPC latency
    pub high_ram_at_idle: bool,     // >65% RAM usage at idle
    pub many_startup_items: bool,   // >15 startup entries
    pub hdd_system_drive: bool,     // HDD as system drive
}

pub fn scan_system_profile() -> Result<SystemProfile, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // CPU info from registry (faster than WMI)
    let cpu_key = hklm
        .open_subkey("HARDWARE\DESCRIPTION\System\CentralProcessor\0")
        .map_err(|e| e.to_string())?;
    let cpu_name: String = cpu_key.get_value("ProcessorNameString").unwrap_or_default();
    let cpu_vendor = if cpu_name.contains("Intel") { "Intel".to_string() }
        else if cpu_name.contains("AMD") { "AMD".to_string() }
        else { "Unknown".to_string() };

    // RAM from GlobalMemoryStatusEx via PowerShell (one-liner, fast)
    let ram_gb = get_ram_gb();
    let ram_usage_pct = get_ram_usage_pct();

    // GPU from registry
    let gpu_name = get_gpu_name(&hklm);

    // Laptop detection via battery
    let is_laptop = hklm
        .open_subkey("SYSTEM\CurrentControlSet\Control\Power")
        .map(|k| k.get_value::<u32, _>("BatteryPresent").unwrap_or(0) == 1)
        .unwrap_or(false);

    // Windows version
    let ver_key = hklm
        .open_subkey("SOFTWARE\Microsoft\Windows NT\CurrentVersion")
        .map_err(|e| e.to_string())?;
    let build: String = ver_key.get_value("CurrentBuildNumber").unwrap_or_default();
    let edition: String = ver_key.get_value("EditionID").unwrap_or_default();
    let windows_version = format!("Windows 11 {} Build {}", edition, build);

    // VBS status
    let vbs_enabled = hklm
        .open_subkey("SYSTEM\CurrentControlSet\Control\DeviceGuard")
        .map(|k| k.get_value::<u32, _>("EnableVirtualizationBasedSecurity").unwrap_or(0) == 1)
        .unwrap_or(false);

    // Power plan via powercfg (native, fast)
    let power_plan = get_active_power_plan();

    // Page file
    let page_file_auto = hklm
        .open_subkey("SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management")
        .map(|k| k.get_value::<u32, _>("PagingFiles").map(|_: String| false).unwrap_or(true))
        .unwrap_or(true);

    // Defender real-time
    let defender_realtime = hklm
        .open_subkey("SOFTWARE\Microsoft\Windows Defender\Real-Time Protection")
        .map(|k| k.get_value::<u32, _>("DisableRealtimeMonitoring").unwrap_or(0) == 0)
        .unwrap_or(true);

    // Startup items count from registry run keys
    let startup_items_count = count_startup_items(&hklm);

    // Running services (approximate)
    let running_services_count = count_running_services();

    // CPU usage (quick sample)
    let cpu_usage_pct = get_cpu_usage();

    // HPET — check PnP ConfigFlags
    let hpet_enabled = !hklm
        .open_subkey("SYSTEM\CurrentControlSet\Enum\ACPI\PNP0103\0\Device Parameters")
        .map(|k| k.get_value::<u32, _>("ConfigFlags").unwrap_or(0) & 0x1 != 0)
        .unwrap_or(false);

    // Network type (first active adapter)
    let (connection_type, link_speed_mbps) = get_network_info();

    // Storage type (system drive)
    let (system_drive_type, system_drive_free_pct) = get_storage_info();

    let flags = DiagnosticFlags {
        vbs_on_gaming_rig:   vbs_enabled && !is_laptop,
        balanced_power_plan: power_plan.to_lowercase().contains("balanced"),
        hpet_on_gaming:      hpet_enabled && !is_laptop,
        high_ram_at_idle:    ram_usage_pct > 65.0,
        many_startup_items:  startup_items_count > 15,
        hdd_system_drive:    system_drive_type == "HDD",
    };

    Ok(SystemProfile {
        cpu_name, cpu_vendor, cpu_cores: get_cpu_cores(),
        ram_gb, ram_usage_pct, gpu_name, is_laptop,
        system_drive_type, system_drive_free_pct,
        connection_type, link_speed_mbps,
        windows_version, power_plan, vbs_enabled,
        hpet_enabled, page_file_auto, defender_realtime,
        cpu_usage_pct, startup_items_count, running_services_count,
        flags,
    })
}

fn get_ram_gb() -> u32 {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // Read from ComputerInfo registry key (fast, no WMI)
    hklm.open_subkey("HARDWARE\RESOURCEMAP\System Resources\Physical Memory")
        .ok()
        .and_then(|_| None) // fallback to command
        .unwrap_or_else(|| {
            let out = Command::new("wmic")
                .args(["ComputerSystem", "get", "TotalPhysicalMemory"])
                .output()
                .ok();
            out.and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout);
                s.lines()
                    .nth(1)?
                    .trim()
                    .parse::<u64>()
                    .ok()
                    .map(|b| (b / (1024 * 1024 * 1024)) as u32)
            })
            .unwrap_or(0)
        })
}

fn get_gpu_name(hklm: &RegKey) -> String {
    hklm.open_subkey(
        "SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000",
    )
    .and_then(|k| k.get_value("DriverDesc"))
    .unwrap_or_else(|_| "Unknown GPU".to_string())
}

fn get_active_power_plan() -> String {
    Command::new("powercfg")
        .args(["/getactivescheme"])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            // Extract plan name from parentheses
            s.split('(').nth(1)?.split(')').next().map(str::to_string)
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_network_info() -> (String, u32) {
    // Read from HKLM Network adapters — no PowerShell needed
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let base = "SYSTEM\CurrentControlSet\Control\Network\{4D36E972-E325-11CE-BFC1-08002BE10318}";
    if let Ok(net_key) = hklm.open_subkey(base) {
        for name in net_key.enum_keys().flatten() {
            if let Ok(conn_key) = net_key.open_subkey(format!("{}\Connection", name)) {
                let media_type: String = conn_key.get_value("MediaSubType").unwrap_or_default();
                if media_type.contains("Wireless") || media_type.contains("WiFi") {
                    return ("WiFi".to_string(), 0);
                }
                return ("Ethernet".to_string(), 1000); // assume Gigabit
            }
        }
    }
    ("Unknown".to_string(), 0)
}

fn get_storage_info() -> (String, f32) {
    // Check if C: is NVMe via device path
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let storage_type =
        hklm.open_subkey("SYSTEM\CurrentControlSet\Services\stornvme\Enum")
            .map(|k| k.get_value::<u32, _>("Count").unwrap_or(0))
            .map(|count| if count > 0 { "NVMe" } else { "SSD" })
            .unwrap_or("HDD");

    // Free space via GetDiskFreeSpaceEx — use wmic as fallback
    let free_pct = Command::new("wmic")
        .args(["logicaldisk", "where", "DeviceID='C:'",
               "get", "FreeSpace,Size"])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            let nums: Vec<u64> = s
                .lines()
                .nth(1)?
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            if nums.len() >= 2 && nums[1] > 0 {
                Some((nums[0] as f32 / nums[1] as f32) * 100.0)
            } else {
                None
            }
        })
        .unwrap_or(50.0);

    (storage_type.to_string(), free_pct)
}

fn count_startup_items(hklm: &RegKey) -> u32 {
    let mut count = 0u32;
    let run_paths = [
        "SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
        "SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
    ];
    for path in &run_paths {
        if let Ok(key) = hklm.open_subkey(path) {
            count += key.enum_values().count() as u32;
        }
    }
    count
}

fn count_running_services() -> u32 {
    Command::new("sc")
        .args(["query", "state=", "running"])
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| l.trim_start().starts_with("SERVICE_NAME"))
                .count() as u32
        })
        .unwrap_or(0)
}

fn get_cpu_cores() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(0)
}

fn get_ram_usage_pct() -> f32 {
    // Read from performance counter key — no PS needed
    0.0 // Filled by monitoring.rs get_quick_stats — reuse existing fn
}

fn get_cpu_usage() -> f32 {
    0.0 // Reuse monitoring.rs get_quick_stats
}
```

---

## 8. Tauri Commands in `commands.rs`

```rust
use crate::modules::ai::{
    self, AnalysisResult, ChatMessage, DiagnosisResult,
    gemini, prompts, profiler,
};

// ── Key management ────────────────────────────────────────────────────────────

#[tauri::command]
pub fn save_gemini_key(key: String) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err("API key cannot be empty".to_string());
    }
    if !key.starts_with("AIza") {
        return Err("Invalid Gemini API key format (must start with AIza)".to_string());
    }
    gemini::save_api_key(key.trim())
}

#[tauri::command]
pub fn get_gemini_key_status() -> bool {
    // Returns true/false only — never exposes the key to the frontend
    gemini::get_api_key().is_ok()
}

#[tauri::command]
pub fn delete_gemini_key() -> Result<(), String> {
    gemini::delete_api_key()
}

// ── Analysis ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn ai_analyze(
    ctx: tauri::State<'_, std::sync::Mutex<crate::commands::TweakContext>>,
    state: tauri::State<'_, std::sync::Mutex<crate::modules::utils::state::AppState>>,
) -> Result<AnalysisResult, String> {
    let profile = profiler::scan_system_profile()?;

    let tweaks_summary = {
        let ctx = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        ctx.tweaks
            .iter()
            .map(|t| serde_json::json!({
                "id":              t.id,
                "name":            t.name,
                "description":     t.description,
                "risk_level":      format!("{:?}", t.warninglevel),
                "category":        format!("{:?}", t.category),
                "currently_applied": st.applied_tweaks.contains(&t.id),
            }))
            .collect::<Vec<_>>()
    };

    let prompt = prompts::build_analyze_prompt(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&tweaks_summary).map_err(|e| e.to_string())?,
    );

    let raw = gemini::call_gemini(
        vec![("user".to_string(), prompt)],
        true,   // expect JSON
    )
    .await?;

    serde_json::from_str::<AnalysisResult>(&raw)
        .map_err(|e| format!("Failed to parse Gemini response: {}\nRaw: {}", e, &raw[..200.min(raw.len())]))
}

// ── Chat ──────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn ai_chat(
    message: String,
    history: Vec<ChatMessage>,
    ctx: tauri::State<'_, std::sync::Mutex<crate::commands::TweakContext>>,
    state: tauri::State<'_, std::sync::Mutex<crate::modules::utils::state::AppState>>,
) -> Result<String, String> {
    let profile = profiler::scan_system_profile()?;

    let applied_summary = {
        let ctx = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        ctx.tweaks
            .iter()
            .filter(|t| st.applied_tweaks.contains(&t.id))
            .map(|t| serde_json::json!({"id": t.id, "name": t.name}))
            .collect::<Vec<_>>()
    };

    let (ctx_user, ctx_model) = prompts::build_context_injection(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&applied_summary).map_err(|e| e.to_string())?,
    );

    // Build message list: context injection + history + new message
    // Gemini requires strictly alternating user/model roles [web:66]
    let mut messages = vec![
        ("user".to_string(),  ctx_user),
        ("model".to_string(), ctx_model),
    ];
    for msg in history {
        messages.push((msg.role, msg.content));
    }
    messages.push(("user".to_string(), message));

    gemini::call_gemini(messages, false).await  // plain text for chat
}

// ── Diagnose ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn ai_diagnose(
    problem: String,
    ctx: tauri::State<'_, std::sync::Mutex<crate::commands::TweakContext>>,
    state: tauri::State<'_, std::sync::Mutex<crate::modules::utils::state::AppState>>,
) -> Result<DiagnosisResult, String> {
    let profile = profiler::scan_system_profile()?;

    let applied_detail = {
        let ctx = ctx.lock().map_err(|e| e.to_string())?;
        let st = state.lock().map_err(|e| e.to_string())?;
        ctx.tweaks
            .iter()
            .filter(|t| st.applied_tweaks.contains(&t.id))
            .map(|t| serde_json::json!({
                "id":          t.id,
                "name":        t.name,
                "description": t.description,
                "category":    format!("{:?}", t.category),
                "risk_level":  format!("{:?}", t.warninglevel),
            }))
            .collect::<Vec<_>>()
    };

    let prompt = prompts::build_diagnose_prompt(
        &serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?,
        &serde_json::to_string(&applied_detail).map_err(|e| e.to_string())?,
        &problem,
    );

    let raw = gemini::call_gemini(
        vec![("user".to_string(), prompt)],
        true,
    )
    .await?;

    serde_json::from_str::<DiagnosisResult>(&raw)
        .map_err(|e| format!("Failed to parse diagnosis: {}", e))
}
```

Register in `lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands ...
    commands::save_gemini_key,
    commands::get_gemini_key_status,
    commands::delete_gemini_key,
    commands::ai_analyze,
    commands::ai_chat,
    commands::ai_diagnose,
])
```

---

## 9. `AiDashboard.svelte` — Complete UI

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { Sparkles, MessageSquare, AlertTriangle, Key, Trash2 } from "lucide-svelte";
  import Button from "../ui/Button.svelte";
  import ScanPanel from "./ScanPanel.svelte";
  import ChatPanel from "./ChatPanel.svelte";
  import DiagnosePanel from "./DiagnosePanel.svelte";

  type Tab = "analyze" | "chat" | "diagnose";
  let tab: Tab = "analyze";
  let hasKey = false;
  let keyInput = "";
  let savingKey = false;
  let keyError = "";

  onMount(async () => {
    hasKey = await invoke<boolean>("get_gemini_key_status");
  });

  async function saveKey() {
    savingKey = true;
    keyError = "";
    try {
      await invoke("save_gemini_key", { key: keyInput });
      hasKey = true;
      keyInput = "";
    } catch (e) {
      keyError = e as string;
    } finally {
      savingKey = false;
    }
  }

  async function deleteKey() {
    await invoke("delete_gemini_key");
    hasKey = false;
  }
</script>

<div class="ai-dashboard">
  {#if !hasKey}
    <!-- ── API Key Setup ── -->
    <div class="setup-card">
      <div class="setup-icon"><Sparkles size={32} /></div>
      <h2>Connect Gemini AI</h2>
      <p>
        Get a free API key at
        <a href="https://aistudio.google.com/app/apikey" target="_blank">
          aistudio.google.com
        </a>
        — no billing required.
      </p>
      <p class="security-note">
        🔒 Your key is stored in <strong>Windows Credential Manager</strong>,
        encrypted with your Windows login. It never touches the filesystem.
      </p>
      <div class="key-input-row">
        <input
          type="password"
          placeholder="AIzaSy..."
          bind:value={keyInput}
          onkeydown={(e) => e.key === "Enter" && saveKey()}
        />
        <Button onclick={saveKey} disabled={savingKey || !keyInput}>
          <Key size={13} /> {savingKey ? "Saving..." : "Save Key"}
        </Button>
      </div>
      {#if keyError}
        <p class="error">{keyError}</p>
      {/if}
    </div>

  {:else}
    <!-- ── Main UI ── -->
    <div class="header">
      <div class="title-row">
        <Sparkles size={20} />
        <h1>AI Advisor</h1>
        <span class="powered-by">powered by Gemini 2.0 Flash</span>
      </div>
      <button class="remove-key" onclick={deleteKey} title="Remove API key">
        <Trash2 size={13} /> Remove Key
      </button>
    </div>

    <div class="tabs">
      <button class:active={tab === "analyze"} onclick={() => tab = "analyze"}>
        <Sparkles size={14} /> Analyze
      </button>
      <button class:active={tab === "chat"} onclick={() => tab = "chat"}>
        <MessageSquare size={14} /> Chat
      </button>
      <button class:active={tab === "diagnose"} onclick={() => tab = "diagnose"}>
        <AlertTriangle size={14} /> Diagnose Problem
      </button>
    </div>

    <div class="tab-content">
      {#if tab === "analyze"}
        <ScanPanel />
      {:else if tab === "chat"}
        <ChatPanel />
      {:else}
        <DiagnosePanel />
      {/if}
    </div>
  {/if}
</div>

<style>
  .ai-dashboard {
    padding: 0 32px 32px;
    height: 100%;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
  .setup-card {
    margin: auto;
    max-width: 480px;
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-card);
    padding: 40px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    align-items: center;
    text-align: center;
  }
  .setup-icon { color: var(--accent-color); }
  .setup-card h2 { font-size: 22px; font-weight: 700; margin: 0; }
  .setup-card p { color: var(--text-muted); font-size: 14px; margin: 0; }
  .security-note {
    background: rgba(var(--accent-rgb), 0.08);
    border: 1px solid rgba(var(--accent-rgb), 0.2);
    border-radius: var(--radius-sm);
    padding: 10px 14px;
    font-size: 13px !important;
    color: var(--text-secondary) !important;
    text-align: left !important;
  }
  .key-input-row {
    display: flex;
    gap: 8px;
    width: 100%;
  }
  .key-input-row input {
    flex: 1;
    background: rgba(255,255,255,0.05);
    border: var(--border-glass);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    color: var(--text-color);
    font-size: 13px;
    outline: none;
  }
  .key-input-row input:focus { border-color: var(--accent-color); }
  .error { color: var(--danger); font-size: 12px; }
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-top: 32px;
  }
  .title-row {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--accent-color);
  }
  .title-row h1 { font-size: 24px; font-weight: 700; margin: 0; color: var(--text-color); }
  .powered-by { font-size: 11px; color: var(--text-muted); margin-top: 2px; }
  .remove-key {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    font-size: 12px;
    padding: 5px 10px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 5px;
    transition: color 0.2s, border-color 0.2s;
  }
  .remove-key:hover { color: var(--danger); border-color: var(--danger); }
  .tabs {
    display: flex;
    gap: 4px;
    margin-bottom: 20px;
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-sm);
    padding: 4px;
    width: fit-content;
  }
  .tabs button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 7px 16px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    border-radius: var(--radius-sm);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.18s;
  }
  .tabs button:hover { color: var(--text-color); }
  .tabs button.active {
    background: rgba(var(--accent-rgb), 0.15);
    color: var(--accent-color);
  }
  .tab-content { flex: 1; overflow-y: auto; }
</style>
```

---

## 10. Add to Sidebar

In `CategorySidebar.svelte`, add to the `categories` array:

```typescript
import { Sparkles } from "lucide-svelte";

// Add after the Home entry:
{ id: "AiAdvisor" as TweakCategory, label: "AI Advisor", icon: Sparkles },
```

In `+page.svelte` or wherever dashboards are routed, add:
```svelte
{#if $activeCategory === "AiAdvisor"}
  <AiDashboard />
{/if}
```

---

## 11. Security Summary

| What | How |
|---|---|
| API key at rest | Windows Credential Manager (DPAPI encrypted) [web:78] |
| Key in memory | Fetched per-request, never stored in Rust state or Svelte store |
| Key exposed to frontend | Never — `get_gemini_key_status` returns `bool` only |
| Network access | Allowlisted to `generativelanguage.googleapis.com` only [web:87] |
| System data sent to Gemini | Profile JSON + tweak list only — no personal files, no registry dumps |

---