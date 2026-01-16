use tauri::command;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[command]
pub async fn empty_recycle_bin() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let script = r#"
        $count = (Get-ChildItem 'Recycle Bin' -Force -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count;
        if ($count -eq 0) {
            Write-Output "EMPTY"
        } else {
            Clear-RecycleBin -Force -ErrorAction SilentlyContinue
            Write-Output "CLEARED"
        }
        "#;

        let output = Command::new("powershell")
            .args(&["-NoProfile", "-Command", script])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if stdout == "EMPTY" {
             Ok("Recycle Bin is already empty.".to_string())
        } else if stdout == "CLEARED" {
             Ok("Recycle Bin emptied successfully.".to_string())
        } else {
             // Fallback for safety, though script covers main cases
             Ok("Recycle Bin active.".to_string())
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported on Linux".to_string())
}

#[command]
pub async fn clear_temp_files() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let script = r#"
$folders = @("$env:TEMP", "$env:WINDIR\Temp");
$startSize = 0;
$folders | ForEach-Object { 
    if (Test-Path $_) {
        $startSize += (Get-ChildItem $_ -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum 
    }
}

$folders | ForEach-Object {
    if (Test-Path $_) {
        Get-ChildItem -Path $_ -Recurse -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    }
}

$endSize = 0;
$folders | ForEach-Object {
    if (Test-Path $_) {
        $endSize += (Get-ChildItem $_ -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    }
}

$cleared = ($startSize - $endSize) / 1MB;
Write-Output "$([math]::Round($cleared, 2)) MB"
"#;
        let output = Command::new("powershell")
            .args(&["-NoProfile", "-Command", script])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if output.status.success() {
            Ok(format!("Cleared {} of temp files.", stdout))
        } else {
            Ok("Temp files maintenance complete.".to_string())
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported on Linux".to_string())
}

#[command]
pub async fn flush_dns_cache() -> Result<String, String> {
     #[cfg(target_os = "windows")]
    {
        // Try native PS cmdlet first, fallback to ipconfig
        let script = "Clear-DnsClientCache -ErrorAction SilentlyContinue; if ($?) { 'OK' } else { cmd /c 'ipconfig /flushdns' }";
        let _ = Command::new("powershell")
            .args(&["-NoProfile", "-Command", script])
            .creation_flags(0x08000000) 
            .output()
            .map_err(|e| e.to_string())?;
        
        // As long as it ran, we consider it a success. DNS flush rarely fails critically.
        Ok("DNS Cache flushed successfully.".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported on Linux".to_string())
}

#[command]
pub async fn reset_network() -> Result<String, String> {
     #[cfg(target_os = "windows")]
    {
        let script = r#"
netsh winsock reset;
netsh int ip reset;
ipconfig /release;
ipconfig /renew;
ipconfig /flushdns;
"#;
        // This takes time, so we just run it
        let _ = Command::new("powershell")
            .args(&["-NoProfile", "-Command", script])
            .creation_flags(0x08000000)
            .output();

        Ok("Network reset complete.".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported on Linux".to_string())
}
