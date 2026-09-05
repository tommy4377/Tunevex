use crate::modules::tweaks::{
    check_tweak_enabled, get_current_windows_build, win11_only_tweaks, TweakContext,
};
use crate::modules::types::{Tweak, TweakType};
use crate::modules::utils::security::{safe_path_existing, safe_path_new};
use crate::modules::utils::state::AppState;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

const PROFILE_FORMAT: &str = "tommytweaker-profile";
const PROFILE_VERSION: u32 = 1;
const MAX_PROFILE_SIZE: u64 = 1024 * 1024;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProfileOperation {
    Enable,
    Disable,
    Run,
    Skip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProfileEntry {
    id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    operation: ProfileOperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TweakProfile {
    format: String,
    version: u32,
    name: String,
    generated_at_unix: u64,
    scope: String,
    tweaks: Vec<ProfileEntry>,
}

#[derive(Debug, Serialize)]
pub struct ProfileExportResult {
    path: String,
    entries: usize,
}

#[derive(Debug, Serialize)]
pub struct ProfilePreviewEntry {
    id: String,
    name: String,
    category: String,
    tweak_type: String,
    operation: ProfileOperation,
    current_enabled: bool,
    warning_level: String,
    requires_restart: bool,
}

#[derive(Debug, Serialize)]
pub struct ProfileIssue {
    id: Option<String>,
    reason: String,
}

#[derive(Debug, Serialize)]
pub struct ProfileImportPreview {
    name: String,
    entries: Vec<ProfilePreviewEntry>,
    issues: Vec<ProfileIssue>,
}

fn profile_path(raw: &str, existing: bool) -> Result<PathBuf, String> {
    if !raw.to_ascii_lowercase().ends_with(".json") {
        return Err("Tweak profiles must use the .json extension.".to_string());
    }
    if existing || Path::new(raw).exists() {
        safe_path_existing(raw)
    } else {
        safe_path_new(raw)
    }
}

fn current_enabled(tweak: &Tweak, applied: &HashSet<String>) -> bool {
    match &tweak.check {
        Some(check) => check_tweak_enabled(check),
        None => applied.contains(&tweak.id),
    }
}

fn available_catalog(tweaks: Vec<Tweak>) -> Vec<Tweak> {
    let build = get_current_windows_build();
    let min_builds = win11_only_tweaks();
    tweaks
        .into_iter()
        .filter(|tweak| {
            min_builds
                .get(tweak.id.as_str())
                .is_none_or(|minimum| build >= *minimum)
        })
        .collect()
}

fn build_profile(
    tweaks: &[Tweak],
    applied: &HashSet<String>,
    mode: &str,
) -> Result<TweakProfile, String> {
    if mode != "active" && mode != "template" {
        return Err("Unknown export mode. Use 'active' or 'template'.".to_string());
    }

    let mut entries = Vec::new();
    for tweak in tweaks {
        let enabled =
            matches!(tweak.tweak_type, TweakType::Toggle) && current_enabled(tweak, applied);
        let operation = match tweak.tweak_type {
            TweakType::Toggle if enabled => ProfileOperation::Enable,
            TweakType::Toggle => ProfileOperation::Disable,
            TweakType::Action => ProfileOperation::Skip,
        };
        if mode == "active" && operation != ProfileOperation::Enable {
            continue;
        }
        entries.push(ProfileEntry {
            id: tweak.id.clone(),
            name: Some(tweak.name.clone()),
            operation,
        });
    }

    Ok(TweakProfile {
        format: PROFILE_FORMAT.to_string(),
        version: PROFILE_VERSION,
        name: if mode == "active" {
            "My active tweaks".to_string()
        } else {
            "Editable tweak template".to_string()
        },
        generated_at_unix: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        scope: mode.to_string(),
        tweaks: entries,
    })
}

fn parse_profile(
    contents: &str,
    catalog: &[Tweak],
    applied: &HashSet<String>,
) -> Result<ProfileImportPreview, String> {
    let profile: TweakProfile = serde_json::from_str(contents)
        .map_err(|error| format!("Invalid tweak profile JSON: {error}"))?;
    if profile.format != PROFILE_FORMAT {
        return Err(format!("Unsupported profile format '{}'.", profile.format));
    }
    if profile.version != PROFILE_VERSION {
        return Err(format!(
            "Unsupported profile version {}. This app supports version {}.",
            profile.version, PROFILE_VERSION
        ));
    }

    let mut entries = Vec::new();
    let mut issues = Vec::new();
    let mut seen = HashSet::new();
    for requested in profile.tweaks {
        if !seen.insert(requested.id.clone()) {
            issues.push(ProfileIssue {
                id: Some(requested.id),
                reason: "Duplicate entry ignored.".to_string(),
            });
            continue;
        }
        let Some(tweak) = catalog.iter().find(|tweak| tweak.id == requested.id) else {
            issues.push(ProfileIssue {
                id: Some(requested.id),
                reason: "Tweak is not available in this version or on this Windows build."
                    .to_string(),
            });
            continue;
        };
        let operation_is_valid = match tweak.tweak_type {
            TweakType::Toggle => matches!(
                requested.operation,
                ProfileOperation::Enable | ProfileOperation::Disable | ProfileOperation::Skip
            ),
            TweakType::Action => matches!(
                requested.operation,
                ProfileOperation::Run | ProfileOperation::Skip
            ),
        };
        if !operation_is_valid {
            issues.push(ProfileIssue {
                id: Some(requested.id),
                reason: format!(
                    "Operation '{:?}' is not valid for a {:?} tweak.",
                    requested.operation, tweak.tweak_type
                ),
            });
            continue;
        }

        entries.push(ProfilePreviewEntry {
            id: tweak.id.clone(),
            name: tweak.name.clone(),
            category: format!("{:?}", tweak.category),
            tweak_type: format!("{:?}", tweak.tweak_type),
            operation: requested.operation,
            current_enabled: current_enabled(tweak, applied),
            warning_level: format!("{:?}", tweak.warning_level),
            requires_restart: tweak.requires_restart,
        });
    }

    Ok(ProfileImportPreview {
        name: profile.name,
        entries,
        issues,
    })
}

#[tauri::command]
pub async fn export_tweak_profile(
    path: String,
    mode: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<ProfileExportResult, String> {
    let tweaks = available_catalog(
        ctx.lock()
            .map_err(|error| error.to_string())?
            .tweaks
            .clone(),
    );
    let applied = state
        .lock()
        .map_err(|error| error.to_string())?
        .applied_tweaks
        .clone();
    let safe_path = profile_path(&path, false)?;

    tokio::task::spawn_blocking(move || {
        let profile = build_profile(&tweaks, &applied, &mode)?;
        let entries = profile.tweaks.len();
        let json = serde_json::to_string_pretty(&profile)
            .map_err(|error| format!("Could not serialize tweak profile: {error}"))?;
        fs::write(&safe_path, json)
            .map_err(|error| format!("Could not save tweak profile: {error}"))?;
        Ok(ProfileExportResult {
            path: safe_path.to_string_lossy().into_owned(),
            entries,
        })
    })
    .await
    .map_err(|error| format!("Profile export task failed: {error}"))?
}

#[tauri::command]
pub async fn preview_tweak_profile(
    path: String,
    ctx: State<'_, Mutex<TweakContext>>,
    state: State<'_, Mutex<AppState>>,
) -> Result<ProfileImportPreview, String> {
    let safe_path = profile_path(&path, true)?;
    let metadata = fs::metadata(&safe_path)
        .map_err(|error| format!("Could not inspect tweak profile: {error}"))?;
    if metadata.len() > MAX_PROFILE_SIZE {
        return Err("Tweak profile is too large (maximum 1 MB).".to_string());
    }
    let tweaks = available_catalog(
        ctx.lock()
            .map_err(|error| error.to_string())?
            .tweaks
            .clone(),
    );
    let applied = state
        .lock()
        .map_err(|error| error.to_string())?
        .applied_tweaks
        .clone();

    tokio::task::spawn_blocking(move || {
        let contents = fs::read_to_string(&safe_path)
            .map_err(|error| format!("Could not read tweak profile: {error}"))?;
        parse_profile(&contents, &tweaks, &applied)
    })
    .await
    .map_err(|error| format!("Profile import task failed: {error}"))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::types::{TweakCategory, WarningLevel};

    fn toggle(id: &str, enabled: bool) -> Tweak {
        let mut tweak = Tweak::new(
            id,
            TweakCategory::System,
            id,
            "test",
            WarningLevel::Safe,
            false,
            None,
            vec![],
        );
        tweak.enabled = enabled;
        tweak
    }

    fn action(id: &str) -> Tweak {
        let mut tweak = toggle(id, false);
        tweak.tweak_type = TweakType::Action;
        tweak
    }

    #[test]
    fn active_export_contains_only_enabled_toggles() {
        let catalog = vec![toggle("on", false), toggle("off", false), action("cleanup")];
        let applied = HashSet::from(["on".to_string()]);
        let profile = build_profile(&catalog, &applied, "active").unwrap();
        assert_eq!(profile.tweaks.len(), 1);
        assert_eq!(profile.tweaks[0].id, "on");
        assert_eq!(profile.tweaks[0].operation, ProfileOperation::Enable);
    }

    #[test]
    fn template_skips_one_shot_actions() {
        let catalog = vec![toggle("off", false), action("cleanup")];
        let profile = build_profile(&catalog, &HashSet::new(), "template").unwrap();
        assert_eq!(profile.tweaks[0].operation, ProfileOperation::Disable);
        assert_eq!(profile.tweaks[1].operation, ProfileOperation::Skip);
    }

    #[test]
    fn import_rejects_operations_that_do_not_match_the_tweak_type() {
        let catalog = vec![toggle("toggle", false), action("action")];
        let json = r#"{
            "format":"tommytweaker-profile","version":1,"name":"test",
            "generated_at_unix":0,"scope":"custom","tweaks":[
                {"id":"toggle","operation":"run"},
                {"id":"action","operation":"enable"}
            ]
        }"#;
        let preview = parse_profile(json, &catalog, &HashSet::new()).unwrap();
        assert!(preview.entries.is_empty());
        assert_eq!(preview.issues.len(), 2);
    }
}
