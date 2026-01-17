#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::command;

#[command]
pub async fn empty_recycle_bin() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // Use Shell.Application COM object - most reliable method
        let script = r#"
try {
    $shell = New-Object -ComObject Shell.Application
    $recycleBin = $shell.NameSpace(0xA)
    $itemCount = $recycleBin.Items().Count
    if ($itemCount -eq 0) {
        Write-Output "EMPTY"
    } else {
        Clear-RecycleBin -Force -Confirm:$false -ErrorAction Stop
        Write-Output "CLEARED:$itemCount"
    }
} catch {
    Write-Output "ERROR:$($_.Exception.Message)"
}
"#;

        let output = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                script,
            ])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if stdout == "EMPTY" {
            Ok("Already empty".to_string())
        } else if stdout.starts_with("CLEARED:") {
            let count = stdout.replace("CLEARED:", "");
            Ok(format!("Emptied ({} items)", count))
        } else if stdout.starts_with("ERROR:") {
            Err(stdout.replace("ERROR:", ""))
        } else {
            Ok("Done".to_string())
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported".to_string())
}

#[command]
pub async fn clear_temp_files() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let script = r#"
$folders = @($env:TEMP, "$env:WINDIR\Temp")
$startSize = 0
$deleted = 0
foreach ($folder in $folders) {
    if (Test-Path $folder) {
        $items = Get-ChildItem -Path $folder -Recurse -Force -ErrorAction SilentlyContinue
        $startSize += ($items | Measure-Object -Property Length -Sum).Sum
        foreach ($item in $items) {
            try {
                Remove-Item $item.FullName -Recurse -Force -ErrorAction Stop
                $deleted++
            } catch {}
        }
    }
}
$cleared = [math]::Round($startSize / 1MB, 1)
Write-Output "$cleared MB"
"#;
        let output = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                script,
            ])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() && stdout != "0 MB" {
            Ok(format!("Freed {}", stdout))
        } else {
            Ok("Already clean".to_string())
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported".to_string())
}

#[command]
pub async fn flush_dns_cache() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // Use ipconfig directly - most reliable method
        let output = Command::new("cmd")
            .args(&["/C", "ipconfig", "/flushdns"])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok("DNS flushed".to_string())
        } else {
            // Fallback to PowerShell
            let _ = Command::new("powershell")
                .args(&["-NoProfile", "-Command", "Clear-DnsClientCache"])
                .creation_flags(0x08000000)
                .output();
            Ok("DNS flushed".to_string())
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported".to_string())
}

#[command]
pub async fn reset_network() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // Run netsh commands directly via cmd for reliability
        let commands = [
            ("netsh", &["winsock", "reset"][..]),
            ("netsh", &["int", "ip", "reset"][..]),
            ("ipconfig", &["/flushdns"][..]),
        ];

        for (cmd, args) in commands {
            let _ = Command::new(cmd)
                .args(args)
                .creation_flags(0x08000000)
                .output();
        }

        Ok("Network reset done".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    Ok("Not supported".to_string())
}
