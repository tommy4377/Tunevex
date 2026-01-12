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

#[command]
pub async fn install_packages_bulk(ids: Vec<String>) -> Result<String, String> {
    // Install multiple packages sequentially
    let ids_str = ids.join("\", \"");
    let script = format!(r#"
$ids = @("{}")
foreach ($id in $ids) {{
    Write-Host "Installing $id..."
    winget install --id $id -e --silent --accept-package-agreements --accept-source-agreements
}}
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
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
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

    // Parse winget output (skip header lines)
    for line in stdout.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0..parts.len()-1].join(" ");
            let id = parts.last().unwrap_or(&"").to_string();
            
            if !id.is_empty() && id.contains('.') {
                packages.push(WingetPackage {
                    id: id.clone(),
                    name,
                    description: String::new(),
                    category: "Search Result".to_string(),
                    installed: false, // Would need to check each one
                });
            }
        }
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

