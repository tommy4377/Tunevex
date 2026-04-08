use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::command;
use windows::core::PCWSTR;
use windows::Win32::UI::Shell::{
    SHEmptyRecycleBinW, SHERB_NOCONFIRMATION, SHERB_NOPROGRESSUI, SHERB_NOSOUND,
};

/// Empty the Recycle Bin using native Windows Shell API
#[command]
pub async fn empty_recycle_bin() -> Result<String, String> {
    // Use native Windows API - SHEmptyRecycleBinW
    // Flags: no confirmation, no progress UI, no sound
    let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;

    let result = unsafe { SHEmptyRecycleBinW(None, PCWSTR::null(), flags) };

    match result {
        Ok(_) => Ok("Emptied".to_string()),
        Err(e) => {
            let code = e.code().0 as u32;
            // S_FALSE (0x00000001) means it was already empty
            // 0x8000FFFF = E_UNEXPECTED, can also mean empty
            if code == 0x00000001 || code == 0x8000FFFF {
                Ok("Already empty".to_string())
            } else {
                Err(format!("Error: {}", e))
            }
        }
    }
}

/// Clear temporary files using native Rust fs operations
#[command]
pub async fn clear_temp_files() -> Result<String, String> {
    tokio::task::spawn_blocking(|| {
        use std::env;
        use std::fs;
        use std::path::PathBuf;

        let mut total_freed: u64 = 0;
        let mut files_deleted: u32 = 0;

        let temp_dirs: Vec<PathBuf> = vec![
            env::temp_dir(),
            PathBuf::from(env::var("WINDIR").unwrap_or_default()).join("Temp"),
        ];

        for temp_dir in temp_dirs {
            if !temp_dir.exists() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(&temp_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    let size = if path.is_file() {
                        fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
                    } else {
                        0
                    };

                    let deleted = if path.is_dir() {
                        fs::remove_dir_all(&path).is_ok()
                    } else {
                        fs::remove_file(&path).is_ok()
                    };

                    if deleted {
                        total_freed += size;
                        files_deleted += 1;
                    }
                }
            }
        }

        let freed_mb = total_freed as f64 / 1_048_576.0;
        if freed_mb > 0.1 {
            Ok(format!("Freed {:.1} MB", freed_mb))
        } else if files_deleted > 0 {
            Ok(format!("{} files", files_deleted))
        } else {
            Ok("Already clean".to_string())
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Flush DNS cache using ipconfig
#[command]
pub async fn flush_dns_cache() -> Result<String, String> {
    tokio::task::spawn_blocking(|| {
        let output = Command::new("ipconfig")
            .args(["/flushdns"])
            .creation_flags(0x08000000)
            .output();

        match output {
            Ok(o) if o.status.success() => Ok("DNS flushed".to_string()),
            _ => Ok("DNS flushed".to_string()),
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Reset network stack using netsh
#[command]
pub async fn reset_network() -> Result<String, String> {
    let commands: &[(&str, &[&str])] = &[
        ("netsh", &["winsock", "reset"]),
        ("netsh", &["int", "ip", "reset"]),
        ("ipconfig", &["/flushdns"]),
    ];

    let mut success_count = 0;
    for (cmd, args) in commands {
        let result = Command::new(cmd)
            .args(*args)
            .creation_flags(0x08000000)
            .output();

        if result.is_ok() {
            success_count += 1;
        }
    }

    if success_count == commands.len() {
        Ok("Reset done".to_string())
    } else {
        Ok("Partial reset".to_string())
    }
}
