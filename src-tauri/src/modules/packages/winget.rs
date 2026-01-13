use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WingetPackage {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String, // e.g., "Browser", "Development", "Utility"
    pub installed: bool,
}

#[command]
pub async fn install_package(id: String) -> Result<String, String> {
    // Run winget install command
    // winget install --id <ID> -e --silent --accept-package-agreements --accept-source-agreements
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "winget install --id {} -e --silent --accept-package-agreements --accept-source-agreements",
                id
            ),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkInstallResult {
    pub id: String,
    pub status: String,
    pub code: i32,
}

#[command]
pub async fn install_packages_bulk(ids: Vec<String>) -> Result<Vec<BulkInstallResult>, String> {
    // Install multiple packages sequentially and report status for each
    let ids_str = ids.join("\", \"");
    // PowerShell script to iterate and capture results
    let script = format!(r#"
$ids = @("{}")
$results = @()
foreach ($id in $ids) {{
    Write-Host "Installing $id..."
    $proc = Start-Process -FilePath "winget" -ArgumentList "install --id $id -e --silent --accept-package-agreements --accept-source-agreements" -Wait -PassThru -NoNewWindow
    $code = $proc.ExitCode
    $status = if ($code -eq 0) {{ "Success" }} else {{ "Failed" }}
    $results += [PSCustomObject]@{{
        id = $id
        status = $status
        code = $code
    }}
}}
$results | ConvertTo-Json -Depth 2
"#, ids_str);

    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &script,
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let json_output = String::from_utf8_lossy(&output.stdout);
        // Parse JSON output
        let results: Vec<BulkInstallResult> = serde_json::from_str(&json_output)
            .or_else(|_| {
                // Handle single object case or empty array wrapper issues if convertto-json behaves oddly
                // Sometimes singular object isn't in array
                serde_json::from_str::<BulkInstallResult>(&json_output).map(|r| vec![r])
            })
            .map_err(|e| format!("Failed to parse bulk install results: {} | Output: {}", e, json_output))?;
        
        Ok(results)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn uninstall_package(id: String) -> Result<String, String> {
    // Run winget uninstall command
    // winget uninstall --id <ID> -e --silent
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("winget uninstall --id {} -e --silent", id),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
pub async fn check_package_status(id: String) -> bool {
    // B.11 FIX: Check stdout content, not just exit code
    // winget list returns exit 0 even if package not found
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("winget list -e --id {}", id),
        ])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            // Check if the package ID appears in the output (not just header/footer)
            stdout.contains(&id) && !stdout.contains("No installed package found")
        }
        Err(_) => false,
    }
}

// C.9: Package Search
#[command]
pub async fn search_packages(query: String) -> Result<Vec<WingetPackage>, String> {
    // Search with exact limit
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("winget search --query '{}' --count 20", query),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() < 2 {
        return Ok(packages);
    }

    // Find header line to parse column positions (Name, Id, Version, etc.)
    // Header usually looks like: Name      Id      Version      Match      Source
    let header_line_idx = lines.iter().position(|l| l.contains("Id") && l.contains("Name"));
    
    if let Some(idx) = header_line_idx {
        let header = lines[idx];
        let id_start = header.find("Id").unwrap_or(0);
        let version_start = header.find("Version").unwrap_or(id_start + 10); // fallback

        // Iterate lines after header (and maybe skip separator line "---")
        for line in lines.iter().skip(idx + 1) {
            if line.trim().is_empty() || line.starts_with("---") {
                continue;
            }

            // Slice the line based on column positions
            // Name is from 0 to id_start
            // Id is from id_start to version_start
            let name_end = std::cmp::min(id_start, line.len());
            let name = line[0..name_end].trim().to_string();

            let id_end = std::cmp::min(version_start, line.len());
            if id_start >= line.len() { continue; } // Line too short
            let id = line[id_start..id_end].trim().to_string();

            if !id.is_empty() && !name.is_empty() {
                packages.push(WingetPackage {
                    id,
                    name,
                    description: String::new(),
                    category: "Search Result".to_string(),
                    installed: false, // Would need check
                });
            }
        }
    } else {
        // Fallback or empty result if header strictly not found
        // The original implementation logic was too fragile, returning empty is safer than garbage
    }

    Ok(packages)
}

// Popular packages catalog for quick install
#[command]
pub async fn get_popular_packages() -> Vec<WingetPackage> {
    vec![
        // Browsers
        WingetPackage { id: "Google.Chrome".to_string(), name: "Google Chrome".to_string(), description: "Fast, secure browser".to_string(), category: "Browser".to_string(), installed: false },
        WingetPackage { id: "Mozilla.Firefox".to_string(), name: "Mozilla Firefox".to_string(), description: "Privacy-focused browser".to_string(), category: "Browser".to_string(), installed: false },
        WingetPackage { id: "BraveSoftware.BraveBrowser".to_string(), name: "Brave".to_string(), description: "Privacy browser with ad blocking".to_string(), category: "Browser".to_string(), installed: false },
        
        // Development
        WingetPackage { id: "Microsoft.VisualStudioCode".to_string(), name: "VS Code".to_string(), description: "Popular code editor".to_string(), category: "Development".to_string(), installed: false },
        WingetPackage { id: "Git.Git".to_string(), name: "Git".to_string(), description: "Version control system".to_string(), category: "Development".to_string(), installed: false },
        WingetPackage { id: "OpenJS.NodeJS.LTS".to_string(), name: "Node.js LTS".to_string(), description: "JavaScript runtime".to_string(), category: "Development".to_string(), installed: false },
        WingetPackage { id: "Python.Python.3.12".to_string(), name: "Python 3.12".to_string(), description: "Programming language".to_string(), category: "Development".to_string(), installed: false },
        WingetPackage { id: "Rustlang.Rustup".to_string(), name: "Rust".to_string(), description: "Systems programming language".to_string(), category: "Development".to_string(), installed: false },
        
        // Utilities
        WingetPackage { id: "7zip.7zip".to_string(), name: "7-Zip".to_string(), description: "File archiver".to_string(), category: "Utility".to_string(), installed: false },
        WingetPackage { id: "Notepad++.Notepad++".to_string(), name: "Notepad++".to_string(), description: "Text editor".to_string(), category: "Utility".to_string(), installed: false },
        WingetPackage { id: "VideoLAN.VLC".to_string(), name: "VLC".to_string(), description: "Media player".to_string(), category: "Utility".to_string(), installed: false },
        WingetPackage { id: "voidtools.Everything".to_string(), name: "Everything".to_string(), description: "Fast file search".to_string(), category: "Utility".to_string(), installed: false },
        
        // Communication
        WingetPackage { id: "Discord.Discord".to_string(), name: "Discord".to_string(), description: "Voice and text chat".to_string(), category: "Communication".to_string(), installed: false },
        WingetPackage { id: "Telegram.TelegramDesktop".to_string(), name: "Telegram".to_string(), description: "Messaging app".to_string(), category: "Communication".to_string(), installed: false },
        
        // Gaming
        WingetPackage { id: "Valve.Steam".to_string(), name: "Steam".to_string(), description: "Gaming platform".to_string(), category: "Gaming".to_string(), installed: false },
        WingetPackage { id: "EpicGames.EpicGamesLauncher".to_string(), name: "Epic Games".to_string(), description: "Gaming platform".to_string(), category: "Gaming".to_string(), installed: false },
    ]
}

