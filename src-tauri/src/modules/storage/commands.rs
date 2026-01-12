use crate::modules::storage::compression::{compress_file, decompress_file, Algorithm};
use crate::modules::storage::scanner::{scan_directory, ScanResult};
use crate::modules::utils::dirs::get_state_path;
use crate::modules::utils::state::AppState;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tauri::State;
// use std::path::PathBuf;

#[tauri::command]
pub async fn scan_storage(path: String) -> Result<ScanResult, String> {
    // Simple scan trigger
    let cancel = Arc::new(AtomicBool::new(false));
    let result = scan_directory(&path, cancel).await;
    Ok(result)
}

#[tauri::command]
pub async fn compress_folder(
    path: String,
    algo_idx: u8,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    // Compress everything in folder
    let algo = match algo_idx {
        0 => Algorithm::Xpress4K,
        1 => Algorithm::Xpress8K,
        2 => Algorithm::Xpress16K,
        3 => Algorithm::Lzx,
        _ => Algorithm::Xpress8K,
    };

    // We should re-scan/walk and compress each file.
    let mut count = 0;

    for entry in walkdir::WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(_) = compress_file(entry.path(), algo) {
                count += 1;
            }
        }
    }

    // Persist state
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.compressed_folders.insert(path.clone());
        if let Ok(path) = get_state_path() {
            let _ = app_state.save(path);
        }
    }

    Ok(format!("Compressed {} files", count))
}

#[tauri::command]
pub async fn decompress_folder(
    path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let mut count = 0;

    for entry in walkdir::WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(_) = decompress_file(entry.path()) {
                count += 1;
            }
        }
    }

    // Update State
    {
        let mut app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.compressed_folders.remove(&path);
        if let Ok(path) = get_state_path() {
            let _ = app_state.save(path);
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
    let result = scan_directory(&path, cancel).await;
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
