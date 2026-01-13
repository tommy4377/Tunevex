// use serde_json::Value;
// use std::process::Command;

// #[cfg(target_os = "windows")]
// use std::os::windows::process::CommandExt;

use std::path::Path;

pub fn sanitize_path(path_str: &str) -> String {
    let raw = path_str.trim().trim_matches('"');
    // If exact match exists, return it
    if Path::new(raw).exists() {
        return raw.to_string();
    }

    // Handle unquoted paths with spaces (e.g. C:\Program Files\Foo\Bar.exe -arg)
    // We split by space and incrementally reconstruct the path to see if it exists
    let parts: Vec<&str> = raw.split(' ').collect();
    let mut current = String::new();
    for part in parts {
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(part);

        // Check if this chunk is a valid file (maybe with .exe appended if missing?)
        // Usually services have .exe.
        if Path::new(&current).is_file() {
            return current;
        }
    }

    // Fallback: return raw if nothing matched (likely won't be found, but strictly returning raw is safer than empty)
    raw.to_string()
}

pub fn get_file_info(path: &str) -> (Option<String>, Option<String>) {
    let clean = sanitize_path(path);
    let p = Path::new(&clean);

    if !p.exists() {
        return (None, None);
    }

    // Use pelite to parse headers
    // We map the file into memory
    use pelite::pe64::{Pe, PeFile};
    use pelite::FileMap;

    match FileMap::open(&clean) {
        Ok(file_map) => {
            if let Ok(pe) = PeFile::from_bytes(file_map.as_ref()) {
                if let Ok(resources) = pe.resources() {
                    if let Ok(version_info) = resources.version_info() {
                        // Extract languages
                        if let Some(lang) = version_info.translation().first() {
                            let company = version_info
                                .value(*lang, "CompanyName")
                                .map(|s| s.to_string());
                            let description = version_info
                                .value(*lang, "FileDescription")
                                .map(|s| s.to_string());
                            return (company, description);
                        }
                    }
                }
            }
        }
        Err(_) => {}
    }

    (None, None)
}
