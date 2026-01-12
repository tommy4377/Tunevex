use serde::{Deserialize, Serialize};
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use walkdir::WalkDir;
use windows::core::PCWSTR;
use windows::Win32::Storage::FileSystem::GetCompressedFileSizeW;

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

pub async fn scan_directory(path: &str, cancel_token: Arc<AtomicBool>) -> ScanResult {
    let mut files = Vec::new();
    let mut total_size = 0;
    let mut total_compressed_size = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if cancel_token.load(Ordering::Relaxed) {
            break;
        }

        if entry.file_type().is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

            // Get compressed size
            let mut compressed_size = size; // Default to logical size if fail

            // Convert path to wide string for Windows API
            let path_buf: Vec<u16> = entry
                .path()
                .as_os_str()
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            unsafe {
                let mut high_part: u32 = 0;
                let low_part = GetCompressedFileSizeW(
                    PCWSTR::from_raw(path_buf.as_ptr()),
                    Some(&mut high_part),
                );
                if low_part != 0xFFFFFFFF || windows::Win32::Foundation::GetLastError().is_ok() {
                    compressed_size = ((high_part as u64) << 32) | (low_part as u64);
                }
            }

            total_size += size;
            total_compressed_size += compressed_size;

            files.push(FileInfo {
                path: entry.path().to_string_lossy().to_string(),
                size,
                compressed_size,
            });
        }
    }

    let file_count = files.len();

    ScanResult {
        files: files.into_iter().take(500).collect(),
        total_size,
        total_compressed_size,
        file_count,
    }
}
