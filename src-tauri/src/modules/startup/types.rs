use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupItem {
    pub id: String,
    pub name: String,
    pub category: String,    // "Logon", "ScheduledTask", "Service", etc.
    pub subcategory: String, // "HKCU\\Run", "StartupFolder", etc.
    pub location: String,
    pub command: String,
    pub enabled: bool,
    pub publisher: Option<String>,
    pub description: Option<String>,
    pub source: AutostartSource,
    pub safety_rating: SafetyRating, // NEW: Safety assessment
    pub file_exists: bool,           // NEW: File path validation
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StartupCategory {
    Logon,
    ScheduledTask,
    Service,
    Explorer,
    Browser,
    Boot,
    ImageHijack,
    AppInit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutostartSource {
    Registry,
    FileSystem,
    TaskScheduler,
    Service,
    ShellExtension, // Added for Explorer/Browser BHOs
    Browser,        // Chrome/Edge extensions
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SafetyRating {
    Safe,      // Microsoft, known good publishers
    Careful,   // Third-party apps (Steam, Discord)
    Dangerous, // Unknown publisher, suspicious path
    Critical,  // Essential system service, never disable
    Unknown,   // No publisher info
}
