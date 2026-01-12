pub mod boot;
pub mod browser;
pub mod explorer;
pub mod logon;
pub mod services;
pub mod tasks;
pub mod types;
pub mod utils;

use crate::modules::startup::types::{SafetyRating, StartupItem};
// use std::collections::HashMap;

/// Scans all startup locations and returns a unified list
pub fn scan_all_startup_items() -> Vec<StartupItem> {
    let mut items = Vec::new();

    items.extend(logon::scan());
    items.extend(tasks::scan());
    items.extend(services::scan());
    items.extend(explorer::scan());
    items.extend(browser::scan());
    items.extend(boot::scan());
    // merged boot scan calls into single boot::scan() inside boot.rs implies cleaner API,
    // but previously I had multiple. I will update boot.rs to have a main scan or call multiple here?
    // I'll call `boot::scan()` if I unify it, or keep calling individual ones.
    // Let's assume boot.rs will expose `scan` or I call specific ones.
    // For now I'll call specific ones but `boot` typically should just expose `scan`.
    // Let's stick to the convention: `boot::scan()` implies all boot stuff.
    // But I previously had `scan_boot_execute` etc.
    // I'll update it to check logic in boot.rs later. For now, let's keep it safe:
    // items.extend(boot::scan_boot_execute());
    // items.extend(boot::scan_appinit());

    // Actually, looking at my plan for boot.rs, I should probably expose a single `scan` or keeping separate is fine.
    // I'll rely on boot module exports.

    items
}

pub fn toggle_item(id: String, enable: bool) -> Result<(), String> {
    if id.starts_with("REG:") {
        return logon::toggle_registry(&id, enable);
    } else if id.starts_with("FILE:") {
        return logon::toggle_file(&id, enable);
    } else if id.starts_with("TASK:") {
        return tasks::toggle_task(&id, enable);
    } else if id.starts_with("SVC:") || id.starts_with("SERVICE:") {
        return services::toggle_service(&id, enable);
    } else if id.starts_with("EXP:") || id.starts_with("EXPLORER:") {
        return Err("Toggling Explorer items not supported yet".to_string());
    }

    Err("Unknown item type".to_string())
}

pub fn assess_safety(path: &str, publisher: Option<&str>) -> SafetyRating {
    let path_lower = path.to_lowercase();
    let pub_lower = publisher.unwrap_or("").to_lowercase();

    // Safe publishers/locations
    if pub_lower.contains("microsoft")
        || pub_lower.contains("google")
        || path_lower.contains("windows\\system32")
    {
        return SafetyRating::Safe;
    }

    // Careful (known apps)
    if path_lower.contains("steam")
        || path_lower.contains("discord")
        || path_lower.contains("spotify")
        || pub_lower.contains("valve")
        || pub_lower.contains("nvidia")
        || pub_lower.contains("intel")
    {
        return SafetyRating::Careful;
    }

    // Dangerous (suspicious)
    if path_lower.contains("temp") || path_lower.contains("appdata\\local\\temp") {
        return SafetyRating::Dangerous;
    }

    // If we have no publisher and it's not system32, treat as Unknown (potential risk, but distinct from explicit Dangerous)
    if publisher.is_none() {
        return SafetyRating::Unknown;
    }

    SafetyRating::Unknown
}
