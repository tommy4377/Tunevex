use crate::modules::storage::scanner::{scan_directory, ScanResult};
use crate::modules::storage::compression::{compress_file, decompress_file, Algorithm};
use crate::modules::utils::state::AppState;
use crate::modules::utils::dirs::get_state_path;
use crate::modules::storage::state::CompactorState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{State, Emitter, AppHandle, Manager};
// use std::path::PathBuf;

#[tauri::command]
pub async fn cancel_compactor(state: State<'_, Mutex<CompactorState>>) -> Result<(), String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    state.cancel();
    Ok(())
}

#[tauri::command]
pub async fn scan_storage(app: AppHandle, path: String, compactor_state: State<'_, Mutex<CompactorState>>) -> Result<ScanResult, String> {
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
    
    let result = scan_directory(&path, cancel_token).await;
    
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
        _ => Algorithm::Xpress8K,
    };
    
    // We should re-scan/walk and compress each file.
    let mut count = 0;
    let mut bytes_processed: u64 = 0;
    // Estimate total? Hard to know without double scan. 
    // We will just emit processed count.
    
    let walker = walkdir::WalkDir::new(&path).into_iter();
    
    for (i, entry) in walker.filter_map(|e| e.ok()).enumerate() {
        // Check cancellation
        if cancel_token.load(Ordering::SeqCst) {
             let _ = app.emit("compactor-status", "Compression Cancelled");
             return Ok("Cancelled".to_string());
        }

        if entry.file_type().is_file() {
            // Emit current file being processed
            if let Some(path_str) = entry.path().to_str() {
                let _ = app.emit("compactor-file", path_str.to_string());
            }
            // Get file size before compression
            let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            
            if let Ok(_) = compress_file(entry.path(), algo) {
                count += 1;
                bytes_processed += file_size;
                // Emit progress every 5 files to avoid flooding
                if count % 5 == 0 {
                     let _ = app.emit("compactor-progress", count);
                     let _ = app.emit("compactor-bytes", bytes_processed);
                }
            }
        }
    }
    // Final emit
    let _ = app.emit("compactor-progress", count);
    
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
    app: AppHandle,
    path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let mut count = 0;
    
    let walker = walkdir::WalkDir::new(&path).into_iter();

    for (i, entry) in walker.filter_map(|e| e.ok()).enumerate() {
        if entry.file_type().is_file() {
            // Emit current file being processed
            if let Some(path_str) = entry.path().to_str() {
                let _ = app.emit("compactor-file", path_str.to_string());
            }
            if let Ok(_) = decompress_file(entry.path()) {
                count += 1;
                 if count % 5 == 0 {
                     let _ = app.emit("compactor-progress", count);
                }
            }
        }
    }
    let _ = app.emit("compactor-progress", count);

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
pub async fn get_compressed_folders(state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    Ok(app_state.compressed_folders.iter().cloned().collect())
}
