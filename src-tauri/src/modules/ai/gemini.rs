use keyring::Entry;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const SERVICE_NAME: &str = "Tunevex";
const KEY_USERNAME: &str = "gemini_api_key";
const GEMINI_MODEL: &str = "gemini-3.5-flash";
const MAX_RESPONSE_BYTES: usize = 2 * 1024 * 1024;

fn endpoint() -> String {
    format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{GEMINI_MODEL}:generateContent"
    )
}

pub fn validate_api_key_format(key: &str) -> Result<&str, String> {
    let key = key.trim();
    // Google AI Studio now issues authorization keys as well as legacy API
    // keys, so an `AIza` prefix is neither required nor sufficient.
    if !(20..=512).contains(&key.len()) {
        return Err("The Gemini key must be between 20 and 512 characters.".to_string());
    }
    if !key.is_ascii() || key.chars().any(char::is_whitespace) {
        return Err("The Gemini key contains whitespace or unsupported characters.".to_string());
    }
    Ok(key)
}

pub fn save_api_key(key: &str) -> Result<(), String> {
    let key = validate_api_key_format(key)?;
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {e}"))?;
    entry
        .set_password(key)
        .map_err(|e| format!("Failed to save key: {e}"))
}

pub fn get_api_key() -> Result<String, String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {e}"))?;
    let key = entry
        .get_password()
        .map_err(|_| "No Gemini key found. Add an AI Studio key in AI settings.".to_string())?;
    validate_api_key_format(&key)?;
    Ok(key)
}

pub fn delete_api_key() -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| format!("Credential Manager error: {e}"))?;
    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete key: {e}"))
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
    #[serde(default)]
    candidates: Vec<Candidate>,
    #[serde(rename = "promptFeedback")]
    prompt_feedback: Option<PromptFeedback>,
}

#[derive(Deserialize)]
struct PromptFeedback {
    #[serde(rename = "blockReason")]
    block_reason: Option<String>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct CandidateContent {
    #[serde(default)]
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    #[serde(default)]
    text: String,
}

pub async fn test_connection() -> Result<(), String> {
    call_gemini(
        vec![("user".to_string(), "Reply with OK.".to_string())],
        false,
    )
    .await
    .map(|_| ())
}

pub async fn call_gemini(
    messages: Vec<(String, String)>,
    expect_json: bool,
) -> Result<String, String> {
    let api_key = get_api_key()?;
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let contents = messages
        .into_iter()
        .filter(|(role, text)| (role == "user" || role == "model") && !text.is_empty())
        .map(|(role, text)| GeminiContent {
            role,
            parts: vec![Part { text }],
        })
        .collect();

    let body = GeminiRequest {
        contents,
        generation_config: GenerationConfig {
            temperature: if expect_json { 0.2 } else { 0.7 },
            max_output_tokens: 8192,
            response_mime_type: if expect_json {
                "application/json".to_string()
            } else {
                "text/plain".to_string()
            },
        },
    };

    let mut last_error = String::new();
    for attempt in 0..3 {
        let response = client
            .post(endpoint())
            // Header authentication keeps credentials out of URLs, logs, and
            // proxy histories and matches the current Gemini REST guidance.
            .header("x-goog-api-key", &api_key)
            .json(&body)
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(error) if (error.is_connect() || error.is_timeout()) && attempt < 2 => {
                last_error = format!("Network error: {error}");
                tokio::time::sleep(Duration::from_millis(500 * (1 << attempt))).await;
                continue;
            }
            Err(error) => return Err(format!("Network error: {error}")),
        };

        let status = response.status();
        if (status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()) && attempt < 2 {
            last_error = format!("Gemini temporarily returned HTTP {status}");
            tokio::time::sleep(Duration::from_millis(750 * (1 << attempt))).await;
            continue;
        }
        if let Some(length) = response.content_length() {
            if length > MAX_RESPONSE_BYTES as u64 {
                return Err("Gemini returned an unexpectedly large response.".to_string());
            }
        }
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read Gemini response: {e}"))?;
        if bytes.len() > MAX_RESPONSE_BYTES {
            return Err("Gemini returned an unexpectedly large response.".to_string());
        }
        if !status.is_success() {
            let detail = String::from_utf8_lossy(&bytes);
            let detail = detail.chars().take(1_500).collect::<String>();
            return Err(format!("Gemini API error {status}: {detail}"));
        }

        let parsed: GeminiResponse = serde_json::from_slice(&bytes)
            .map_err(|e| format!("Failed to parse Gemini response: {e}"))?;
        let candidate = parsed.candidates.into_iter().next().ok_or_else(|| {
            parsed
                .prompt_feedback
                .and_then(|feedback| feedback.block_reason)
                .map(|reason| format!("Gemini blocked the request: {reason}"))
                .unwrap_or_else(|| "Gemini returned no candidate response.".to_string())
        })?;
        if let Some(reason) = candidate.finish_reason.as_deref() {
            if reason != "STOP" {
                return Err(format!("Gemini response was incomplete ({reason})."));
            }
        }
        let text = candidate
            .content
            .parts
            .into_iter()
            .map(|part| part.text)
            .collect::<String>();
        if text.trim().is_empty() {
            return Err("Gemini returned an empty response.".to_string());
        }
        return Ok(text);
    }

    Err(last_error)
}

#[cfg(test)]
mod tests {
    use super::validate_api_key_format;

    #[test]
    fn accepts_current_authorization_keys_without_legacy_prefix() {
        assert!(validate_api_key_format("authorization-key-without-legacy-prefix").is_ok());
        assert!(validate_api_key_format("short").is_err());
        assert!(validate_api_key_format("authorization key with spaces").is_err());
    }
}
