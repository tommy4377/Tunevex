use std::io;
use std::path::Path;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_READ_DATA, FILE_SHARE_READ, FILE_SHARE_WRITE,
    FILE_WRITE_DATA, OPEN_EXISTING,
};
use windows::Win32::System::IO::DeviceIoControl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Xpress4K = 0,
    Xpress8K = 1,
    Xpress16K = 2,
    Lzx = 3,
}

#[repr(C)]
struct WofExternalInfo {
    version: u32,
    provider: u32,
}

#[repr(C)]
struct FileProviderExternalInfo {
    version: u32,
    algorithm: u32,
    flags: u32,
}

const WOF_CURRENT_VERSION: u32 = 1;
const WOF_PROVIDER_FILE: u32 = 2;
const FILE_PROVIDER_CURRENT_VERSION: u32 = 1;

// Algorithms for File Provider
const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0;
const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 1;
const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 2;
const FILE_PROVIDER_COMPRESSION_LZX: u32 = 3;

// Define IOCTLs if missing
const FSCTL_SET_EXTERNAL_BACKING: u32 = 0x9030C;

pub fn compress_file(path: &Path, algo: Algorithm) -> io::Result<bool> {
    unsafe {
        // Correct string conversion for Windows API
        let path_str = HSTRING::from(path.as_os_str());

        let handle = CreateFileW(
            PCWSTR::from_raw(path_str.as_ptr()),
            FILE_READ_DATA.0 | FILE_WRITE_DATA.0, // Extract u32 from FILE_ACCESS_RIGHTS
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            None, // Template file is Option<HANDLE>
        )
        .map_err(|e: windows::core::Error| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        // Guard to close handle automatically
        let _handle_guard = HandleGuard(handle);

        let wof_info = WofExternalInfo {
            version: WOF_CURRENT_VERSION,
            provider: WOF_PROVIDER_FILE,
        };

        let file_info = FileProviderExternalInfo {
            version: FILE_PROVIDER_CURRENT_VERSION,
            algorithm: match algo {
                Algorithm::Xpress4K => FILE_PROVIDER_COMPRESSION_XPRESS4K,
                Algorithm::Xpress8K => FILE_PROVIDER_COMPRESSION_XPRESS8K,
                Algorithm::Xpress16K => FILE_PROVIDER_COMPRESSION_XPRESS16K,
                Algorithm::Lzx => FILE_PROVIDER_COMPRESSION_LZX,
            },
            flags: 0,
        };

        // Pack structs into buffer
        let mut input_buffer = Vec::new();
        let wof_bytes: &[u8] = std::slice::from_raw_parts(
            &wof_info as *const _ as *const u8,
            std::mem::size_of::<WofExternalInfo>(),
        );
        let file_bytes: &[u8] = std::slice::from_raw_parts(
            &file_info as *const _ as *const u8,
            std::mem::size_of::<FileProviderExternalInfo>(),
        );
        input_buffer.extend_from_slice(wof_bytes);
        input_buffer.extend_from_slice(file_bytes);

        let mut bytes_returned: u32 = 0;

        DeviceIoControl(
            handle,
            FSCTL_SET_EXTERNAL_BACKING,
            Some(input_buffer.as_ptr() as *const _),
            input_buffer.len() as u32,
            None,
            0,
            Some(&mut bytes_returned),
            None,
        )
        .map_err(|e: windows::core::Error| {
            io::Error::new(io::ErrorKind::Other, format!("Compression failed: {}", e))
        })?;
        Ok(true)
    }
}

const FSCTL_DELETE_EXTERNAL_BACKING: u32 = 0x90310;

pub fn decompress_file(path: &Path) -> io::Result<bool> {
    unsafe {
        let path_str = HSTRING::from(path.as_os_str());

        let handle = CreateFileW(
            PCWSTR::from_raw(path_str.as_ptr()),
            FILE_READ_DATA.0 | FILE_WRITE_DATA.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            None,
        )
        .map_err(|e: windows::core::Error| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        let _handle_guard = HandleGuard(handle);
        let mut bytes_returned: u32 = 0;

        DeviceIoControl(
            handle,
            FSCTL_DELETE_EXTERNAL_BACKING,
            None,
            0,
            None,
            0,
            Some(&mut bytes_returned),
            None,
        )
        .map_err(|e: windows::core::Error| {
            // Error 3221225485 (0xC000000D) = KEY_DELETED or similar if not compressed?
            // Actually usually ERROR_INVALID_PARAMETER or similar if not compressed
            io::Error::new(io::ErrorKind::Other, format!("Decompression failed: {}", e))
        })?;

        Ok(true)
    }
}

#[allow(unused_variables)]
pub fn is_compressed(path: &Path) -> bool {
    // TODO: Implement FSCTL_GET_EXTERNAL_BACKING check
    false
}

// Helper for handle
struct HandleGuard(HANDLE);
impl Drop for HandleGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}
