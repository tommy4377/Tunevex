use crate::modules::utils::dirs::get_app_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn get_memory_path() -> Result<PathBuf, String> {
    Ok(get_app_dir()?.join("ai_memory.json"))
}

fn get_chats_dir() -> Result<PathBuf, String> {
    let dir = get_app_dir()?.join("ai_chats");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub started_at: u64,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMemoryEntry {
    pub timestamp: u64,
    pub kind: MemoryKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MemoryKind {
    TweakAction {
        tweak_id: String,
        action: String,
        reason: String,
        outcome: String,
    },
    Diagnosis {
        problem: String,
        likely_causes: Vec<String>,
        suggested_fix: String,
        resolved: Option<bool>,
        follow_up_notes: Option<String>,
    },
    Recommendation {
        scan_summary: String,
        add: Vec<String>,
        remove: Vec<String>,
        applied: Vec<String>,
    },
    StartupAction {
        item_id: String,
        item_name: String,
        action: String,
        reason: String,
    },
    UserNote {
        note: String,
    },
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AiMemoryStore {
    pub entries: Vec<AiMemoryEntry>,
    pub last_system_summary: Option<String>,
    pub condensed_context: Option<String>,
}

impl AiMemoryStore {
    pub fn load() -> Self {
        get_memory_path()
            .ok()
            .and_then(|p| fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = get_memory_path()?;
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn add(&mut self, kind: MemoryKind) {
        self.entries.push(AiMemoryEntry {
            timestamp: now_unix(),
            kind,
        });
        if self.entries.len() > 500 {
            self.entries.drain(0..self.entries.len() - 500);
        }
    }

    pub fn to_prompt_context(&self) -> String {
        let recent: Vec<_> = self.entries.iter().rev().take(30).collect();
        serde_json::to_string(&recent).unwrap_or_default()
    }
}

pub fn save_chat_session(session: &ChatSession) -> Result<(), String> {
    let dir = get_chats_dir()?;
    let path = dir.join(format!("{}.json", session.id));
    let json = serde_json::to_string_pretty(session).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn list_chat_sessions() -> Vec<ChatSessionMeta> {
    let Ok(dir) = get_chats_dir() else {
        return vec![];
    };
    let mut sessions: Vec<ChatSessionMeta> = fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().extension().map_or(false, |x| x == "json"))
        .filter_map(|e| {
            let content = fs::read_to_string(e.path()).ok()?;
            let s: ChatSession = serde_json::from_str(&content).ok()?;
            Some(ChatSessionMeta {
                id: s.id,
                title: s.title,
                started_at: s.started_at,
                message_count: s.messages.len(),
            })
        })
        .collect();
    sessions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    sessions
}

pub fn load_chat_session(id: &str) -> Result<ChatSession, String> {
    let dir = get_chats_dir()?;
    let path = dir.join(format!("{}.json", id));
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

pub fn delete_chat_session(id: &str) -> Result<(), String> {
    let dir = get_chats_dir()?;
    let path = dir.join(format!("{}.json", id));
    fs::remove_file(path).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSessionMeta {
    pub id: String,
    pub title: String,
    pub started_at: u64,
    pub message_count: usize,
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
