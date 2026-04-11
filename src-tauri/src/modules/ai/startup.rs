use serde::{Deserialize, Serialize};
use crate::modules::startup::types::{StartupItem, SafetyRating};
use super::{gemini, profiler};
use super::memory::{AiMemoryStore, MemoryKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartupRecommendation {
    pub item_id: String,
    pub item_name: String,
    pub action: String,
    pub priority: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartupScanResult {
    pub summary: String,
    pub recommendations: Vec<StartupRecommendation>,
}

#[derive(Serialize)]
struct StartupItemView<'a> {
    id: &'a str,
    name: &'a str,
    category: &'a str,
    subcategory: &'a str,
    command: &'a str,
    publisher: Option<&'a str>,
    enabled: bool,
    file_exists: bool,
    safety_rating: &'a str,
}

pub async fn scan_startup_with_ai(
    items: Vec<StartupItem>,
) -> Result<StartupScanResult, String> {
    let profile = profiler::scan_system_profile()?;
    let memory = AiMemoryStore::load();
    let memory_ctx = memory.to_prompt_context();

    let views: Vec<StartupItemView> = items.iter().map(|i| StartupItemView {
        id: &i.id,
        name: &i.name,
        category: &i.category,
        subcategory: &i.subcategory,
        command: &i.command,
        publisher: i.publisher.as_deref(),
        enabled: i.enabled,
        file_exists: i.file_exists,
        safety_rating: match i.safety_rating {
            SafetyRating::Safe => "Safe",
            SafetyRating::Careful => "Careful",
            SafetyRating::Dangerous => "Dangerous",
            SafetyRating::Unknown => "Unknown",
            SafetyRating::Critical => "Critical",
        },
    }).collect();

    let items_json = serde_json::to_string(&views)
        .map_err(|e| e.to_string())?;

    if items_json.len() > 200_000 {
        eprintln!(
            "[AI Startup] Warning: items_json is {}KB — unusually large startup list",
            items_json.len() / 1024
        );
    }

    let prompt = build_startup_scan_prompt(&serde_json::to_string_pretty(&profile).unwrap_or_default(), &items_json, &memory_ctx);

    let messages = vec![
        ("user".to_string(),
         format!("You are an expert Windows optimization assistant in TommyTweaker. \
                  Reply ONLY with valid JSON matching the StartupScanResult schema. \
                  SYSTEM PROFILE:\n{}\n\nPAST AI MEMORY (last actions/diagnoses):\n{}",
                  serde_json::to_string_pretty(&profile).unwrap_or_default(), memory_ctx)),
        ("model".to_string(),
         "Understood. I have the system profile and AI memory. Ready to analyze startup.".to_string()),
        ("user".to_string(), prompt),
    ];

    let raw = gemini::call_gemini(messages, true).await?;
    let result: StartupScanResult = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed to parse startup scan: {}\nRaw: {:.300}", e, raw))?;

    let mut mem = AiMemoryStore::load();
    mem.add(MemoryKind::Recommendation {
        scan_summary: result.summary.clone(),
        add: vec![],
        remove: result.recommendations.iter()
            .filter(|r| r.action == "disable" || r.action == "investigate")
            .map(|r| r.item_id.clone())
            .collect(),
        applied: vec![],
    });
    let _ = mem.save();

    Ok(result)
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
      "item_name": "human readable name",
      "action": "disable|keep|investigate",
      "priority": "high|medium|low",
      "reason": "one sentence referencing specific item data"
    }}
  ]
}}"#)
}
