use keyring::Entry;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const SERVICE_NAME: &str = "TommyTweaker";
const KEY_USERNAME: &str = "gemini_api_key";

const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

pub fn save_api_key(key: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {}", e))?;
    entry
        .set_password(key)
        .map_err(|e| format!("Failed to save key: {}", e))
}

pub fn get_api_key() -> Result<String, String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {}", e))?;
    entry
        .get_password()
        .map_err(|_| "No API key found. Please add your Gemini API key in Settings.".to_string())
}

pub fn delete_api_key() -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {}", e))?;
    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete key: {}", e))
}

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

pub async fn call_gemini(
    messages: Vec<(String, String)>,
    expect_json: bool,
) -> Result<String, String> {
    let api_key = get_api_key()?;

    let client = Client::builder()
        .timeout(Duration::from_secs(60))
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