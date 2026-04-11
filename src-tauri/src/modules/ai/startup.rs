use serde::{Deserialize, Serialize};
use crate::modules::startup::types::StartupItem;
use super::{gemini, profiler};
use super::memory::{AiMemoryStore, MemoryKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartupRecommendation {
    pub item_id: String,
    pub action: String,
    pub priority: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartupScanResult {
    pub summary: String,
    pub recommendations: Vec<StartupRecommendation>,
}

pub async fn scan_startup_with_ai(
    items: Vec<StartupItem>,
) -> Result<StartupScanResult, String> {
    let profile = profiler::scan_system_profile()?;
    let profile_json = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;

    let memory = AiMemoryStore::load();
    let memory_ctx = memory.to_prompt_context();

    let items_json = serde_json::to_string_pretty(&items).map_err(|e| e.to_string())?;

    let prompt = build_startup_scan_prompt(&profile_json, &items_json, &memory_ctx);

    let messages = vec![
        ("user".to_string(),
         format!("You are an expert Windows optimization assistant in TommyTweaker. \
                  Reply ONLY with valid JSON matching the StartupScanResult schema. \
                  SYSTEM PROFILE:\n{}\n\nPAST AI MEMORY (last actions/diagnoses):\n{}",
                  profile_json, memory_ctx)),
        ("model".to_string(),
         "Understood. I have the system profile and AI memory. Ready to analyze startup.".to_string()),
        ("user".to_string(), prompt),
    ];

    let raw = gemini::call_gemini(messages, true).await?;
    serde_json::from_str::<StartupScanResult>(&raw)
        .map_err(|e| format!("Failed to parse startup scan: {}\nRaw: {:.300}", e, raw))
}

fn build_startup_scan_prompt(
    profile_json: &str,
    items_json: &str,
    memory_ctx: &str,
) -> String {
    format!(r#"Analyze these Windows startup items and recommend which to disable.

STARTUP ITEMS (full list):
{items_json}

SYSTEM PROFILE:
{profile_json}

PAST AI DECISIONS (memory):
{memory_ctx}

RULES:
1. Evaluate EVERY item. Never skip.
2. "disable" only if clearly non-essential for this specific hardware/use case (gaming PC).
3. "investigate" if unknown publisher, suspicious path (%TEMP%, %APPDATA% + random name), or IFEO hijack.
4. "keep" if Microsoft system critical, GPU driver (NVIDIA/AMD), or gaming-related (Steam, Discord).
5. Consider previous diagnoses in memory — if an item was involved in a past problem, flag it.
6. safety_rating from the item data MUST inform your priority.

Return ONLY valid JSON:
{{
  "summary": "2-3 sentence assessment of the startup state",
  "recommendations": [
    {{
      "item_id": "exact id string",
      "action": "disable|keep|investigate",
      "priority": "high|medium|low",
      "reason": "one sentence referencing specific item data"
    }}
  ]
}}"#)
}