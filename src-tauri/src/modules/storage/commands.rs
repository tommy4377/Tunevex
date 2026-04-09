use crate::modules::storage::compression::{
    compress_file, decompress_file, is_compressed, Algorithm,
};
use crate::modules::storage::scanner::{scan_directory, ScanResult};
use crate::modules::storage::state::CompactorState;
use crate::modules::utils::dirs::get_state_path;
use crate::modules::utils::state::AppState;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};

// File extensions that don't compress well (already compressed formats)
const SKIP_EXTENSIONS: &[&str] = &[
    // Images
    "jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "svg", // Video
    "mp4", "mkv", "avi", "mov", "webm", "wmv", "flv", "m4v", // Audio
    "mp3", "aac", "flac", "ogg", "wma", "m4a", "opus", // Archives (already compressed)
    "zip", "rar", "7z", "gz", "xz", "zst", "bz2", "tar", "cab",
    // Documents (often compressed internally)
    "pdf", "docx", "xlsx", "pptx",
];

// System files that should NEVER be touched (can cause BSOD)
const SKIP_FILENAMES: &[&str] = &[
    "pagefile.sys",
    "hiberfil.sys",
    "swapfile.sys",
    "ntuser.dat",
    "usrclass.dat",
    "desktop.ini",
    "thumbs.db",
];

// Minimum file size worth compressing (4KB)
const MIN_FILE_SIZE: u64 = 4096;

/// Check if a file should be skipped during compression
fn should_skip_file(path: &Path, size: u64) -> bool {
    // Skip files that are too small
    if size < MIN_FILE_SIZE {
        return true;
    }

    // Check filename against skip list
    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        let filename_lower = filename.to_lowercase();
        if SKIP_FILENAMES.iter().any(|&skip| filename_lower == skip) {
            return true;
        }
    }

    // Check extension against skip list
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        if SKIP_EXTENSIONS.iter().any(|&skip| ext_lower == skip) {
            return true;
        }
    }

    // Skip files that are already WOF-compressed
    if is_compressed(path) {
        return true;
    }

    false
}

#[tauri::command]
pub async fn cancel_compactor(state: State<'_, Mutex<CompactorState>>) -> Result<(), String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    state.cancel();
    Ok(())
}

#[tauri::command]
pub async fn scan_storage(
    app: AppHandle,
    path: String,
    compactor_state: State<'_, Mutex<CompactorState>>,
) -> Result<ScanResult, String> {
    // Reset cancel state
    {
        let s = compactor_state.lock().map_err(|e| e.to_string())?;
        s.reset();
    }
    let cancel_token = {
        let s = compactor_state.lock().map_err(|e| e.to_string())?;
        s.cancel_token.clone()
    };

    // Emit start event
    let _ = app.emit("compactor-status", "Scanning...");

    let result = scan_directory(&path, cancel_token, Some(app)).await;

    Ok(result)
}

#[tauri::command]
pub async fn compress_folder(
    app: AppHandle,
    path: String,
    algo_idx: u8,
    state: State<'_, Mutex<AppState>>,
    compactor_state: State<'_, Mutex<CompactorState>>,
) -> Result<String, String> {
    // Reset cancel state
    {
        let s = compactor_state.lock().map_err(|e| e.to_string())?;
        s.reset();
    }
    let cancel_token = {
        let s = compactor_state.lock().map_err(|e| e.to_string())?;
        s.cancel_token.clone()
    };

    // Compress everything in folder
    let algo = match algo_idx {
        0 => Algorithm::Xpress4K,
        1 => Algorithm::Xpress8K,
        2 => Algorithm::Xpress16K,
        3 => Algorithm::Lzx,
        _ => Algorithm::Xpress4K,
    };

    let path_clone = path.clone();
    let app_clone = app.clone();
    let cancel_clone = cancel_token.clone();

    let result = tokio::task::spawn_blocking(move || -> Result<String, String> {
        let mut count = 0;
        let mut skipped = 0;
        let mut bytes_processed: u64 = 0;

        let walker = walkdir::WalkDir::new(&path_clone).into_iter();

        for entry in walker.filter_map(|e| e.ok()) {
            if cancel_clone.load(Ordering::SeqCst) {
                let _ = app_clone.emit("compactor-status", "Compression Cancelled");
                return Ok("Cancelled".to_string());
            }

            if entry.file_type().is_file() {
                let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                if should_skip_file(entry.path(), file_size) {
                    skipped += 1;
                    continue;
                }

                if let Some(path_str) = entry.path().to_str() {
                    let _ = app_clone.emit("compactor-file", path_str.to_string());
                }

                match compress_file(entry.path(), algo) {
                    Ok(_) => {
                        count += 1;
                        bytes_processed += file_size;
                        if count % 5 == 0 {
                            let _ = app_clone.emit("compactor-progress", count);
                            let _ = app_clone.emit("compactor-bytes", bytes_processed);
                        }
                    }
                    Err(_) => {
                        skipped += 1;
                    }
                }
            }
        }

        let _ = app_clone.emit("compactor-progress", count);

        Ok(format!("Compressed {} files ({} skipped)", count, skipped))
    })
    .await
    .map_err(|e| e.to_string())??;

    // Persist state after spawn_blocking completes
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.compressed_folders.insert(path.clone());
        if let Ok(p) = get_state_path() {
            let _ = app_state.save(p);
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn decompress_folder(
    app: AppHandle,
    path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let path_clone = path.clone();
    let app_clone = app.clone();

    let count = tokio::task::spawn_blocking(move || -> usize {
        let mut count = 0;

        let walker = walkdir::WalkDir::new(&path_clone).into_iter();

        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(path_str) = entry.path().to_str() {
                    let _ = app_clone.emit("compactor-file", path_str.to_string());
                }
                if decompress_file(entry.path()).is_ok() {
                    count += 1;
                    if count % 5 == 0 {
                        let _ = app_clone.emit("compactor-progress", count);
                    }
                }
            }
        }

        let _ = app_clone.emit("compactor-progress", count);
        count
    })
    .await
    .map_err(|e| e.to_string())?;

    // Update State
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.compressed_folders.remove(&path);
        if let Ok(p) = get_state_path() {
            let _ = app_state.save(p);
        }
    }

    Ok(format!("Decompressed {} files", count))
}

#[derive(serde::Serialize)]
pub struct FolderStats {
    pub path: String,
    pub total_size: u64,
    pub compressed_size: u64,
    pub file_count: usize,
}

#[tauri::command]
pub async fn get_folder_stats(path: String) -> Result<FolderStats, String> {
    let cancel = Arc::new(AtomicBool::new(false));
    let result = scan_directory(&path, cancel, None).await;
    Ok(FolderStats {
        path,
        total_size: result.total_size,
        compressed_size: result.total_compressed_size,
        file_count: result.file_count,
    })
}

#[tauri::command]
pub async fn get_compressed_folders(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    Ok(app_state.compressed_folders.iter().cloned().collect())
}
