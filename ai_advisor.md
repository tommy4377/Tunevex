# TommyTweaker — AI Advisor: Full Implementation Guide

> **Model:** `gemini-3-flash-preview` (Gemini 3 Flash, free tier on AI Studio)  
> **API endpoint:** `https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent`

---

## 0. Cargo.toml

```toml
[dependencies]
# HTTP — usa native-tls (Schannel su Windows) per non gonfiare il binario
reqwest  = { version = "0.12", features = ["json", "native-tls"] }
keyring  = { version = "3",    features = ["windows-native"] }
serde    = { version = "1",    features = ["derive"] }
serde_json = "1"
tokio    = { version = "1",    features = ["full"] }

[profile.release]
opt-level     = "z"
lto           = true
codegen-units = 1
strip         = true
panic         = "abort"
```

---

## 1. capabilities/default.json — permesso HTTP

```json
{
  "identifier": "default",
  "permissions": [
    "core:default",
    "opener:default",
    "dialog:default",
    {
      "identifier": "http:default",
      "allow_url": ["https://generativelanguage.googleapis.com"]
    }
  ]
}
```

Registra il plugin in `lib.rs`:
```rust
.plugin(tauri_plugin_http::init())
```

---

## 2. src/modules/ai/mod.rs

```rust
pub mod gemini;
pub mod profiler;
pub mod prompts;

pub use profiler::{SystemProfile, DiagnosticFlags};

use serde::{Deserialize, Serialize};

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
    pub priority: String, // "high" | "medium" | "low"
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweakConflict {
    pub tweak_ids: Vec<String>,
    pub issue: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosisResult {
    pub likely_causes: Vec<DiagnosisCause>,
    pub suggested_fix: String,
    pub safe_to_revert: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosisCause {
    pub tweak_id: String,
    pub confidence: String, // "high" | "medium" | "low"
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,  // "user" | "model"
    pub content: String,
}
```

---

## 3. src/modules/ai/gemini.rs

```rust
use keyring::Entry;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const SERVICE_NAME: &str = "TommyTweaker";
const KEY_USERNAME: &str = "gemini_api_key";
// ✅ Gemini 3 Flash — free tier, 1M context, Pro-level intelligence
const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent";

// --- Key storage (Windows Credential Manager / DPAPI) ---

pub fn save_api_key(key: &str) -> Result<(), String> {
    Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {e}"))?
        .set_password(key)
        .map_err(|e| format!("Failed to save key: {e}"))
}

pub fn get_api_key() -> Result<String, String> {
    Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {e}"))?
        .get_password()
        .map_err(|_| "No API key found. Add your Gemini API key in Settings.".to_string())
}

pub fn delete_api_key() -> Result<(), String> {
    Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {e}"))?
        .delete_credential()
        .map_err(|e| format!("Failed to delete key: {e}"))
}

// --- Gemini HTTP types ---

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

// --- Core call ---

pub async fn call_gemini(
    messages: Vec<(String, String)>, // (role, text)
    expect_json: bool,
) -> Result<String, String> {
    let api_key = get_api_key()?;
    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

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
            // Gemini 3: keep temperature at default 1.0 — do NOT lower it
            // (lowering causes looping/degraded perf on Gemini 3 models)
            temperature: if expect_json { 1.0 } else { 1.0 },
            max_output_tokens: 8192,
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
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("Gemini API error {status}: {body_text}"));
    }

    let parsed: GeminiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Gemini response: {e}"))?;

    let candidate = parsed
        .candidates
        .into_iter()
        .next()
        .ok_or("No response from Gemini")?;

    if let Some(reason) = &candidate.finish_reason {
        if reason != "STOP" {
            eprintln!("Gemini finish reason: {reason}");
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

pub fn build_context_injection(profile_json: &str, applied_json: &str) -> (String, String) {
    let user = format!(
        r#"You are an expert Windows optimization assistant embedded in TommyTweaker.
You have full access to the user's system state.

SYSTEM PROFILE:
{profile_json}

CURRENTLY APPLIED TWEAKS:
{applied_json}

Rules:
- Always reference specific tweak IDs (e.g. net-tcp-ack-freq)
- Be honest about risks; don't recommend Dangerous tweaks unless asked
- If something could cause instability, say so clearly
- Keep answers concise and technical"#
    );
    let model = "Understood. I have full context of this system and its applied tweaks. Ready to help.".to_string();
    (user, model)
}
```

---

## 4. src/modules/ai/prompts.rs

```rust
pub fn build_analyze_prompt(profile_json: &str, tweaks_json: &str) -> String {
    format!(
        r#"Analyze this Windows system and return optimization recommendations.

SYSTEM PROFILE:
{profile_json}

ALL AVAILABLE TWEAKS (id, name, description, risk_level, category, currently_applied):
{tweaks_json}

Tasks:
1. Identify tweaks to ADD (not yet applied, would genuinely benefit THIS system)
2. Identify tweaks to REMOVE (applied but useless or harmful for this system)
3. Identify conflicts between currently applied tweaks
4. Write a 2-3 sentence honest assessment of the system's current state

Return ONLY valid JSON:
{{
  "system_summary": "string",
  "add":    [{{ "id": "string", "priority": "high|medium|low", "reason": "one sentence" }}],
  "remove": [{{ "id": "string", "priority": "high|medium|low", "reason": "why it is counterproductive" }}],
  "conflicts": [{{ "tweak_ids": ["string"], "issue": "what the conflict causes" }}]
}}"#
    )
}

pub fn build_diagnose_prompt(
    profile_json: &str,
    applied_tweaks_json: &str,
    problem: &str,
) -> String {
    format!(
        r#"A user is experiencing a problem with their Windows PC.

PROBLEM:
{problem}

SYSTEM PROFILE:
{profile_json}

CURRENTLY APPLIED TWEAKS (the only changes made to the system):
{applied_tweaks_json}

Analyze which applied tweaks could be causing or contributing to this problem.
Consider interactions between tweaks, not just individual ones.

Return ONLY valid JSON:
{{
  "likely_causes": [{{ "tweak_id": "string", "confidence": "high|medium|low", "explanation": "specific reason" }}],
  "suggested_fix": "clear step-by-step instructions",
  "safe_to_revert": ["tweak_id1", "tweak_id2"]
}}"#
    )
}
```

---

## 5. src/modules/ai/profiler.rs

```rust
use serde::Serialize;
use std::process::Command;
use winreg::{enums::*, RegKey};

#[derive(Debug, Serialize)]
pub struct SystemProfile {
    // Hardware
    pub cpu_name: String,
    pub cpu_vendor: String,   // "Intel" | "AMD" | "Unknown"
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub gpu_name: String,
    pub is_laptop: bool,
    // Storage
    pub system_drive_type: String,  // "NVMe" | "SSD" | "HDD"
    pub system_drive_free_pct: f32,
    // Network
    pub connection_type: String,    // "Ethernet" | "WiFi" | "Unknown"
    pub link_speed_mbps: u32,
    // Windows state
    pub windows_version: String,
    pub power_plan: String,
    pub vbs_enabled: bool,
    pub hpet_enabled: bool,
    pub pagefile_auto: bool,
    pub defender_realtime: bool,
    // Runtime
    pub ram_usage_pct: f32,
    pub cpu_usage_pct: f32,
    pub startup_items_count: u32,
    pub running_services_count: u32,
    // Pre-computed flags
    pub flags: DiagnosticFlags,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticFlags {
    pub vbs_on_gaming_rig: bool,   // VBS enabled → ~15% GPU loss
    pub balanced_power_plan: bool,
    pub hpet_on_gaming: bool,
    pub high_ram_at_idle: bool,    // >65% RAM at idle
    pub many_startup_items: bool,  // >15 entries
    pub hdd_system_drive: bool,
}

pub fn scan_system_profile() -> Result<SystemProfile, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // CPU
    let cpu_key = hklm
        .open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0")
        .map_err(|e| e.to_string())?;
    let cpu_name: String = cpu_key.get_value("ProcessorNameString").unwrap_or_default();
    let cpu_vendor = if cpu_name.contains("Intel") { "Intel" }
        else if cpu_name.contains("AMD") { "AMD" }
        else { "Unknown" }.to_string();

    // RAM
    let ram_gb = get_ram_gb();
    let ram_usage_pct = get_ram_usage_pct();

    // GPU
    let gpu_name = get_gpu_name(&hklm);

    // Laptop detection
    let is_laptop = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Power")
        .map(|k| k.get_value::<u32, _>("BatteryPresent").unwrap_or(0) == 1)
        .unwrap_or(false);

    // Windows version
    let ver_key = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .map_err(|e| e.to_string())?;
    let build: String = ver_key.get_value("CurrentBuildNumber").unwrap_or_default();
    let edition: String = ver_key.get_value("EditionID").unwrap_or_default();
    let windows_version = format!("Windows 11 {} (Build {})", edition, build);

    // VBS
    let vbs_enabled = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\DeviceGuard")
        .map(|k| k.get_value::<u32, _>("EnableVirtualizationBasedSecurity").unwrap_or(0) == 1)
        .unwrap_or(false);

    // Power plan
    let power_plan = get_active_power_plan();

    // Page file
    let pagefile_auto = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management")
        .map(|k| k.get_value::<String, _>("PagingFiles").map(|_| false).unwrap_or(true))
        .unwrap_or(true);

    // Defender real-time
    let defender_realtime = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows Defender\\Real-Time Protection")
        .map(|k| k.get_value::<u32, _>("DisableRealtimeMonitoring").unwrap_or(0) == 0)
        .unwrap_or(true);

    let startup_items_count = count_startup_items(&hklm);
    let cpu_usage_pct = get_cpu_usage_pct();

    let flags = DiagnosticFlags {
        vbs_on_gaming_rig: vbs_enabled && !is_laptop,
        balanced_power_plan: power_plan.to_lowercase().contains("balanced"),
        hpet_on_gaming: false, // fill from hpet check
        high_ram_at_idle: ram_usage_pct > 65.0,
        many_startup_items: startup_items_count > 15,
        hdd_system_drive: false, // fill from drive type check
    };

    Ok(SystemProfile {
        cpu_name, cpu_vendor, cpu_cores: 0, ram_gb, gpu_name, is_laptop,
        system_drive_type: "NVMe".to_string(), system_drive_free_pct: 0.0,
        connection_type: "Unknown".to_string(), link_speed_mbps: 0,
        windows_version, power_plan, vbs_enabled, hpet_enabled: false,
        pagefile_auto, defender_realtime,
        ram_usage_pct, cpu_usage_pct, startup_items_count,
        running_services_count: 0, flags,
    })
}

fn get_ram_gb() -> u32 {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB"])
        .output()
        .ok();
    out.and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<f32>().ok())
        .map(|v| v as u32)
        .unwrap_or(0)
}

fn get_ram_usage_pct() -> f32 {
    // Hook into your existing systemStore stats here
    0.0
}

fn get_cpu_usage_pct() -> f32 {
    // Hook into your existing systemStore stats here
    0.0
}

fn get_gpu_name(hklm: &RegKey) -> String {
    hklm.open_subkey("SOFTWARE\\Microsoft\\DirectX")
        .and_then(|k| k.get_value::<String, _>("Description"))
        .unwrap_or_else(|_| "Unknown GPU".to_string())
}

fn get_active_power_plan() -> String {
    Command::new("powercfg")
        .args(["/getactivescheme"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn count_startup_items(hklm: &RegKey) -> u32 {
    let run_keys = [
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run",
    ];
    run_keys.iter()
        .filter_map(|k| hklm.open_subkey(k).ok())
        .map(|k| k.enum_values().count() as u32)
        .sum()
}
```

---

## 6. Tauri commands — commands.rs (aggiungi)

```rust
use crate::modules::ai::{self, gemini, prompts, profiler};
use serde_json;

#[tauri::command]
pub async fn ai_save_key(key: String) -> Result<(), String> {
    gemini::save_api_key(&key)
}

#[tauri::command]
pub async fn ai_delete_key() -> Result<(), String> {
    gemini::delete_api_key()
}

#[tauri::command]
pub async fn ai_has_key() -> bool {
    gemini::get_api_key().is_ok()
}

#[tauri::command]
pub async fn ai_analyze(
    all_tweaks: serde_json::Value,
) -> Result<ai::AnalysisResult, String> {
    let profile = profiler::scan_system_profile()?;
    let profile_json = serde_json::to_string_pretty(&profile)
        .map_err(|e| e.to_string())?;
    let tweaks_json = serde_json::to_string_pretty(&all_tweaks)
        .map_err(|e| e.to_string())?;

    let (ctx_user, ctx_model) = gemini::build_context_injection(&profile_json, &tweaks_json);
    let analyze_prompt = prompts::build_analyze_prompt(&profile_json, &tweaks_json);

    let messages = vec![
        ("user".to_string(),  ctx_user),
        ("model".to_string(), ctx_model),
        ("user".to_string(),  analyze_prompt),
    ];

    let raw = gemini::call_gemini(messages, true).await?;
    serde_json::from_str::<ai::AnalysisResult>(&raw)
        .map_err(|e| format!("Failed to parse AI response: {e}\n\nRaw: {raw}"))
}

#[tauri::command]
pub async fn ai_diagnose(
    problem: String,
    applied_tweaks: serde_json::Value,
) -> Result<ai::DiagnosisResult, String> {
    let profile = profiler::scan_system_profile()?;
    let profile_json = serde_json::to_string_pretty(&profile)
        .map_err(|e| e.to_string())?;
    let applied_json = serde_json::to_string_pretty(&applied_tweaks)
        .map_err(|e| e.to_string())?;

    let (ctx_user, ctx_model) = gemini::build_context_injection(&profile_json, &applied_json);
    let diagnose_prompt = prompts::build_diagnose_prompt(&profile_json, &applied_json, &problem);

    let messages = vec![
        ("user".to_string(),  ctx_user),
        ("model".to_string(), ctx_model),
        ("user".to_string(),  diagnose_prompt),
    ];

    let raw = gemini::call_gemini(messages, true).await?;
    serde_json::from_str::<ai::DiagnosisResult>(&raw)
        .map_err(|e| format!("Failed to parse AI response: {e}\n\nRaw: {raw}"))
}

#[tauri::command]
pub async fn ai_chat(
    message: String,
    history: Vec<ai::ChatMessage>,
    applied_tweaks: serde_json::Value,
) -> Result<String, String> {
    let profile = profiler::scan_system_profile()?;
    let profile_json = serde_json::to_string_pretty(&profile)
        .map_err(|e| e.to_string())?;
    let applied_json = serde_json::to_string_pretty(&applied_tweaks)
        .map_err(|e| e.to_string())?;

    let (ctx_user, ctx_model) = gemini::build_context_injection(&profile_json, &applied_json);

    let mut messages = vec![
        ("user".to_string(),  ctx_user),
        ("model".to_string(), ctx_model),
    ];

    for msg in &history {
        messages.push((msg.role.clone(), msg.content.clone()));
    }
    messages.push(("user".to_string(), message));

    gemini::call_gemini(messages, false).await
}
```

---

## 7. AiDashboard.svelte — setup card (no emoji, no gradients)

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Sparkles } from "lucide-svelte";
  import ScanPanel   from "./ScanPanel.svelte";
  import ChatPanel   from "./ChatPanel.svelte";
  import DiagnosePanel from "./DiagnosePanel.svelte";

  let hasKey = false;
  let keyInput = "";
  let saving = false;
  let saveError = "";
  let activeTab: "scan" | "chat" | "diagnose" = "scan";

  onMount(async () => { hasKey = await invoke("ai_has_key"); });

  async function saveKey() {
    if (!keyInput.trim()) return;
    saving = true; saveError = "";
    try {
      await invoke("ai_save_key", { key: keyInput.trim() });
      hasKey = true;
    } catch (e) { saveError = e as string; }
    finally { saving = false; }
  }

  async function deleteKey() {
    await invoke("ai_delete_key");
    hasKey = false;
  }
</script>

<div class="ai-container">
  {#if !hasKey}
    <div class="setup-card">
      <div class="setup-header">
        <Sparkles size={16} />
        <h2>Connect Gemini AI</h2>
      </div>
      <p class="setup-sub">
        Get a free API key at
        <a href="https://aistudio.google.com/app/apikey" target="_blank" class="subtle-link">
          aistudio.google.com
        </a>
        — no billing required.
      </p>
      <div class="security-note">
        Your key is stored in <strong>Windows Credential Manager</strong>,
        encrypted with your Windows login. It never touches the filesystem.
      </div>
      <div class="key-row">
        <input
          type="password"
          bind:value={keyInput}
          placeholder="AIza..."
          disabled={saving}
          onkeydown={(e) => e.key === "Enter" && saveKey()}
        />
        <button class="save-btn" onclick={saveKey} disabled={saving || !keyInput.trim()}>
          {saving ? "Saving…" : "Save Key"}
        </button>
      </div>
      {#if saveError}<p class="error">{saveError}</p>{/if}
    </div>

  {:else}
    <div class="tabs">
      <button class:active={activeTab === "scan"}    onclick={() => activeTab = "scan"}>Scan</button>
      <button class:active={activeTab === "chat"}    onclick={() => activeTab = "chat"}>Chat</button>
      <button class:active={activeTab === "diagnose"} onclick={() => activeTab = "diagnose"}>Diagnose</button>
      <button class="disconnect-btn" onclick={deleteKey}>Disconnect</button>
    </div>
    <div class="tab-content">
      {#if activeTab === "scan"}    <ScanPanel />    {/if}
      {#if activeTab === "chat"}    <ChatPanel />    {/if}
      {#if activeTab === "diagnose"} <DiagnosePanel /> {/if}
    </div>
  {/if}
</div>

<style>
  .ai-container { height: 100%; display: flex; flex-direction: column; padding: 24px; gap: 20px; }

  /* Setup card — flat, coerente con il resto dell'app */
  .setup-card {
    max-width: 460px;
    background: var(--layer-card);
    border: var(--border-glass);
    border-radius: var(--radius-lg);
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin: auto;
  }
  .setup-header {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--accent-color);
  }
  .setup-header h2 {
    font-size: 16px;
    font-weight: 600;
    margin: 0;
    color: var(--text-color);
  }
  .setup-sub {
    font-size: 13px;
    color: var(--text-muted);
    margin: 0;
  }
  .subtle-link {
    color: var(--accent-color);
    text-decoration: none;
    opacity: 0.85;
    transition: opacity 0.15s;
  }
  .subtle-link:hover { opacity: 1; text-decoration: underline; }

  .security-note {
    background: rgba(255,255,255,0.03);
    border: var(--border-glass);
    border-radius: var(--radius-sm);
    padding: 10px 14px;
    font-size: 13px;
    color: var(--text-muted);
    line-height: 1.5;
  }
  .security-note strong { color: var(--text-secondary); font-weight: 600; }

  .key-row { display: flex; gap: 8px; }
  .key-row input {
    flex: 1;
    background: rgba(0,0,0,0.2);
    border: var(--border-glass);
    border-radius: var(--radius-md);
    padding: 9px 12px;
    color: var(--text-color);
    font-size: 13px;
    outline: none;
  }
  .key-row input:focus { border-color: var(--accent-color); }
  .save-btn {
    background: var(--accent-color);
    border: none;
    border-radius: var(--radius-md);
    padding: 9px 16px;
    color: white;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s;
  }
  .save-btn:hover:not(:disabled) { opacity: 0.85; }
  .save-btn:disabled { opacity: 0.5; cursor: wait; }
  .error { font-size: 12px; color: var(--danger); margin: 0; }

  /* Tabs */
  .tabs {
    display: flex;
    gap: 4px;
    border-bottom: var(--border-glass);
    padding-bottom: 12px;
  }
  .tabs button {
    background: none;
    border: none;
    padding: 6px 14px;
    font-size: 13px;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: all 0.15s;
  }
  .tabs button:hover  { color: var(--text-color); background: var(--layer-hover); }
  .tabs button.active { color: var(--accent-color); background: rgba(129,140,248,0.08); }
  .disconnect-btn { margin-left: auto; font-size: 12px !important; }
  .tab-content { flex: 1; overflow: hidden; }
</style>
```

---

## 8. HomeDashboard.svelte — AI Advisor button (flat, no gradient)

```svelte
<!-- Aggiungi all'inizio dello script -->
import { Sparkles } from "lucide-svelte";
import { activeCategory } from "$lib/stores";

<!-- Aggiungi nel template, dopo la div.header -->
<button class="ai-cta" onclick={() => ($activeCategory = "AiAdvisor")}>
  <Sparkles size={16} />
  <div>
    <span class="ai-cta-title">AI Advisor</span>
    <span class="ai-cta-sub">Scan your system and get personalized recommendations</span>
  </div>
  <span class="ai-cta-arrow">→</span>
</button>
```

```css
/* Aggiungi nella <style> di HomeDashboard.svelte */
.ai-cta {
  display: flex;
  align-items: center;
  gap: 14px;
  width: 100%;
  padding: 14px 18px;
  background: var(--layer-card);
  border: var(--border-glass);
  border-left: 2px solid var(--accent-color);
  border-radius: var(--radius-card);
  color: var(--text-color);
  cursor: pointer;
  transition: background 0.18s, border-color 0.18s;
  text-align: left;
}
.ai-cta:hover {
  background: var(--layer-hover);
  border-left-color: var(--accent-hover, var(--accent-color));
}
.ai-cta > :global(svg) { color: var(--accent-color); flex-shrink: 0; }
.ai-cta-title  { display: block; font-size: 14px; font-weight: 600; color: var(--text-color); }
.ai-cta-sub    { display: block; font-size: 12px; color: var(--text-muted); margin-top: 2px; }
.ai-cta-arrow  { font-size: 14px; color: var(--text-muted); margin-left: auto; }
```

---

