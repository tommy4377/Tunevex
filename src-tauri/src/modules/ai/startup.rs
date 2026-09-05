use super::memory::{AiMemoryStore, MemoryKind};
use super::{gemini, profiler};
use crate::modules::startup::types::{SafetyRating, StartupItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    app: tauri::AppHandle,
) -> Result<StartupScanResult, String> {
    let profile = profiler::scan_system_profile_from_state(Some(app))?;
    let memory = AiMemoryStore::load();
    let memory_ctx = memory.to_prompt_context();

    let views: Vec<StartupItemView> = items
        .iter()
        .map(|i| StartupItemView {
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
        })
        .collect();

    let items_json = serde_json::to_string(&views).map_err(|e| e.to_string())?;

    if items_json.len() > 200_000 {
        eprintln!(
            "[AI Startup] Warning: items_json is {}KB — unusually large startup list",
            items_json.len() / 1024
        );
    }

    let prompt = build_startup_scan_prompt(
        &serde_json::to_string_pretty(&profile).unwrap_or_default(),
        &items_json,
        &memory_ctx,
    );

    let messages = vec![
        (
            "user".to_string(),
            format!(
                "You are an expert Windows optimization assistant in Tunevex. \
                  Reply ONLY with valid JSON matching the StartupScanResult schema. \
                  SYSTEM PROFILE:\n{}\n\nPAST AI MEMORY (last actions/diagnoses):\n{}",
                serde_json::to_string_pretty(&profile).unwrap_or_default(),
                memory_ctx
            ),
        ),
        (
            "model".to_string(),
            "Understood. I have the system profile and AI memory. Ready to analyze startup."
                .to_string(),
        ),
        ("user".to_string(), prompt),
    ];

    let raw = gemini::call_gemini(messages, true).await?;

    let mut result: StartupScanResult = serde_json::from_str(&raw)
        .or_else(|_: serde_json::Error| {
            #[derive(Deserialize)]
            struct RawRec {
                item_id: String,
                #[serde(default)]
                item_name: Option<String>,
                action: String,
                #[serde(default)]
                priority: Option<String>,
                #[serde(default)]
                reason: Option<String>,
            }
            let recs: Vec<RawRec> =
                serde_json::from_str(&raw).map_err(|e| format!("parse array: {}", e))?;
            let recommendations = recs
                .into_iter()
                .map(|r| StartupRecommendation {
                    item_id: r.item_id,
                    item_name: r.item_name.unwrap_or_default(),
                    action: r.action,
                    priority: r.priority.unwrap_or_else(|| "medium".to_string()),
                    reason: r.reason.unwrap_or_default(),
                })
                .collect();
            Ok(StartupScanResult {
                summary: "AI scan completed — see recommendations below.".to_string(),
                recommendations,
            })
        })
        .map_err(|e: String| format!("Failed to parse startup scan: {}\nRaw: {:.300}", e, raw))?;

    // Treat model output as untrusted: bind every recommendation back to the
    // exact item supplied by the native scanner and enforce critical-item
    // protection in code, not just in the prompt.
    let known: HashMap<&str, &StartupItem> =
        items.iter().map(|item| (item.id.as_str(), item)).collect();
    result.recommendations.retain_mut(|recommendation| {
        let Some(item) = known.get(recommendation.item_id.as_str()) else {
            return false;
        };
        if !matches!(
            recommendation.action.as_str(),
            "disable" | "keep" | "investigate"
        ) {
            return false;
        }
        recommendation.item_name = item.name.clone();
        if matches!(item.safety_rating, SafetyRating::Critical) {
            recommendation.action = "keep".to_string();
            recommendation.priority = "high".to_string();
            recommendation.reason = "Protected critical Windows startup component.".to_string();
        }
        true
    });

    let mut mem = AiMemoryStore::load();
    mem.add(MemoryKind::Recommendation {
        scan_summary: result.summary.clone(),
        add: vec![],
        remove: result
            .recommendations
            .iter()
            .filter(|r| r.action == "disable" || r.action == "investigate")
            .map(|r| r.item_id.clone())
            .collect(),
        applied: vec![],
    });
    let _ = mem.save();

    Ok(result)
}

fn build_startup_scan_prompt(profile_json: &str, items_json: &str, memory_ctx: &str) -> String {
    format!(
        r#"Analyze these Windows startup items and recommend which to disable.

STARTUP ITEMS (full list):
{items_json}

SYSTEM PROFILE:
{profile_json}

PAST AI DECISIONS (memory):
{memory_ctx}

RULES:
1. "Critical" items are Windows system processes — report them as "keep" with a brief explanation of what they do. NEVER recommend disabling them.
2. Evaluate ALL "Unknown", "Careful", and "Dangerous" items. Never skip any.
3. "disable" only if clearly non-essential for this specific hardware/use case (gaming PC).
4. "investigate" if unknown publisher, suspicious path (%TEMP%, %APPDATA% + random name), or IFEO hijack.
5. For EVERY item reviewed, tell the user WHAT it is and whether it should be removed.
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
      "reason": "one sentence explaining what this item is and whether it should be removed"
    }}
  ]
 }}"#
    )
}
