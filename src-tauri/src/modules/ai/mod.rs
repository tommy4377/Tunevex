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
    pub priority: String,
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
    pub confidence: String,
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}