use std::path::PathBuf;

pub const ALLOWED_COMMANDS: &[&str] = &[
    "sc",
    "schtasks",
    "bcdedit",
    "powercfg",
    "fsutil",
    "powershell",
    "taskkill",
    "netsh",
    "reg",
    "cmd",
    "pnputil",
    "del",
    "rmdir",
    "dism",
    "ipconfig",
    "lodctr",
    "net",
    "rundll32",
    "w32tm",
];

pub const ALLOWED_WRITE_ROOTS: &[&str] = &[
    r"C:\Users",
    r"C:\ProgramData",
    r"C:\Program Files",
    r"C:\Program Files (x86)",
];

pub fn validate_command(cmd: &str) -> Result<(), String> {
    let cmd_path = PathBuf::from(cmd);
    if cmd_path.components().count() != 1 {
        return Err("Commands must use an allow-listed executable name, not a path.".to_string());
    }
    let exe_name = cmd_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(cmd)
        .to_lowercase();
    if ALLOWED_COMMANDS.contains(&exe_name.as_str()) {
        Ok(())
    } else {
        Err(format!("Command '{}' is not in the allowed list", cmd))
    }
}

pub fn expand_env_vars(raw: &str) -> String {
    raw.replace("%APPDATA%", &std::env::var("APPDATA").unwrap_or_default())
        .replace(
            "%LOCALAPPDATA%",
            &std::env::var("LOCALAPPDATA").unwrap_or_default(),
        )
        .replace(
            "%ProgramData%",
            &std::env::var("ProgramData").unwrap_or_default(),
        )
        .replace(
            "%ProgramFiles%",
            &std::env::var("ProgramFiles").unwrap_or_default(),
        )
        .replace(
            "%UserProfile%",
            &std::env::var("USERPROFILE").unwrap_or_default(),
        )
        .replace("%Home%", &std::env::var("USERPROFILE").unwrap_or_default())
}

fn is_under_allowed_root(p: &std::path::Path) -> bool {
    let s = p.to_string_lossy().to_lowercase();
    ALLOWED_WRITE_ROOTS
        .iter()
        .map(|root| root.trim_end_matches(['\\', '/']).to_lowercase())
        .any(|root| s == root || s.starts_with(&format!("{root}\\")))
}

pub fn safe_path_existing(raw: &str) -> Result<PathBuf, String> {
    let expanded = expand_env_vars(raw);
    let canonical = std::path::Path::new(&expanded)
        .canonicalize()
        .map_err(|e| format!("Path not found '{}': {}", raw, e))?;
    if !is_under_allowed_root(&canonical) {
        return Err(format!("Path '{}' is outside allowed directories.", raw));
    }
    Ok(canonical)
}

pub fn safe_path_new(raw: &str) -> Result<PathBuf, String> {
    let expanded = expand_env_vars(raw);
    let p = PathBuf::from(&expanded);
    let mut existing = p.clone();
    let mut suffix = PathBuf::new();
    loop {
        if existing.exists() {
            break;
        }
        match existing.parent() {
            Some(parent) => {
                if let Some(component) = existing.file_name() {
                    suffix = PathBuf::from(component).join(&suffix);
                }
                existing = parent.to_path_buf();
            }
            None => break,
        }
    }
    let base = if existing.exists() {
        existing
            .canonicalize()
            .map_err(|e| format!("Cannot resolve base path for '{}': {}", raw, e))?
    } else {
        existing
    };
    let full = base.join(suffix);
    if !is_under_allowed_root(&full) {
        return Err(format!(
            "Destination '{}' is outside allowed directories.",
            raw
        ));
    }
    Ok(full)
}

#[cfg(test)]
mod tests {
    use super::{is_under_allowed_root, validate_command};
    use std::path::Path;

    #[test]
    fn command_paths_and_prefix_confusion_are_rejected() {
        assert!(validate_command("powershell.exe").is_ok());
        assert!(validate_command(r"C:\Temp\powershell.exe").is_err());
        assert!(!is_under_allowed_root(Path::new(r"C:\UsersEvil\payload")));
        assert!(is_under_allowed_root(Path::new(r"C:\Users\Public\payload")));
    }
}
