use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::os::windows::ffi::OsStrExt;
use windows::core::PCWSTR;
use windows::Win32::Storage::FileSystem::{GetCompressedFileSizeW, INVALID_FILE_SIZE};
use windows::Win32::Foundation::{SetLastError, GetLastError, NO_ERROR};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub compressed_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub files: Vec<FileInfo>,
    pub total_size: u64,
    pub total_compressed_size: u64,
    pub file_count: usize,
}

pub async fn scan_directory(path: &str, cancel_token: Arc<AtomicBool>, app_handle: Option<AppHandle>) -> ScanResult {
    let path = path.to_string();
    let cancel = cancel_token.clone();

    tokio::task::spawn_blocking(move || {
        let mut files = Vec::new();
        let mut total_size = 0;
        let mut total_compressed_size = 0;
        
        for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
            if cancel.load(Ordering::Relaxed) {
                 break;
            }
            
            if entry.file_type().is_file() {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                
                // Get compressed size
                let mut compressed_size = size; // Default to logical size if fail
                
                // Convert path to wide string for Windows API
                let path_buf: Vec<u16> = entry.path().as_os_str().encode_wide().chain(std::iter::once(0)).collect();
                unsafe {
                    let mut high_part: u32 = 0;
                    SetLastError(NO_ERROR);
                    let low_part = GetCompressedFileSizeW(PCWSTR::from_raw(path_buf.as_ptr()), Some(&mut high_part));
                    
                    let error = GetLastError();
                    if low_part != INVALID_FILE_SIZE || error == NO_ERROR {
                         compressed_size = ((high_part as u64) << 32) | (low_part as u64);
                    }
                }
                
                total_size += size;
                total_compressed_size += compressed_size;
                
                files.push(FileInfo {
                    path: entry.path().to_string_lossy().to_string(),
                    size,
                    compressed_size
                });

                if let Some(app) = &app_handle {
                     let count = files.len();
                     if count % 50 == 0 {
                         let _ = app.emit("compactor-progress", count);
                         let _ = app.emit("compactor-bytes", total_size);
                         // Also emit the current file being scanned
                         let _ = app.emit("compactor-file", entry.path().to_string_lossy().to_string());
                     }
                }
            }
        }

        let file_count = files.len();
        
        ScanResult {
            files: files.into_iter().take(500).collect(),
            total_size,
            total_compressed_size,
            file_count,
        }
    }).await.unwrap_or(ScanResult {
        files: Vec::new(),
        total_size: 0,
        total_compressed_size: 0,
        file_count: 0
    })
}
