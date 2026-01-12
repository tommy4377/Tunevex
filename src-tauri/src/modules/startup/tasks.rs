use crate::modules::startup::types::{AutostartSource, StartupItem};
use crate::modules::startup::{assess_safety, utils};
use serde_json::Value;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn scan() -> Vec<StartupItem> {
    let script = r#"
        $tasks = Get-ScheduledTask | Where-Object { 
            $_.State -ne 'Disabled' -and 
            ($_.Triggers | Where-Object { 
                $_.StartBoundary -or 
                $_.ExecutionTimeLimit -or 
                $_.Repetition -or 
                $_.CimClass.CimClassName -like '*Logon*' -or
                $_.CimClass.CimClassName -like '*Startup*' -or
                $_.CimClass.CimClassName -like '*Boot*' -or
                $_.CimClass.CimClassName -like '*Idle*'
            })
        }
        
        $results = foreach ($task in $tasks) {
            $action = if ($task.Actions.Count -gt 0) { 
                $task.Actions[0].Execute + ' ' + $task.Actions[0].Arguments 
            } else { '' }
            
            [PSCustomObject]@{
                Id = $task.TaskPath.TrimEnd('\') + '\' + $task.TaskName
                Name = $task.TaskName
                Path = $task.TaskPath
                Command = $action.Trim()
                State = $task.State.ToString()
                Triggers = ($task.Triggers | ForEach-Object { $_.CimClass.CimClassName }).Join(', ')
            }
        }
        
        $results | ConvertTo-Json -Compress
    "#;

    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", script])
        .creation_flags(0x08000000)
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::default(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });

    let json = String::from_utf8_lossy(&output.stdout);
    let tasks: Vec<Value> = if let Ok(arr) = serde_json::from_str::<Vec<Value>>(&json) {
        arr
    } else if let Ok(obj) = serde_json::from_str::<Value>(&json) {
        vec![obj]
    } else {
        Vec::new()
    };

    tasks
        .into_iter()
        .filter_map(|t| {
            let name = t["Name"].as_str()?.to_string();
            let id = t["Id"].as_str().unwrap_or(&name).to_string();
            let command = t["Command"].as_str().unwrap_or_default().to_string();
            let state = t["State"].as_str().unwrap_or("Ready");
            let triggers = t["Triggers"].as_str().unwrap_or_default().to_string();

            if command.is_empty() {
                return None;
            }

            let (publisher, desc) = utils::get_file_info(&command);
            let rating = assess_safety(&command, publisher.as_deref());

            Some(StartupItem {
                id: format!("TASK:{}", id),
                name,
                category: "ScheduledTask".to_string(),
                subcategory: triggers,
                location: "Task Scheduler".to_string(),
                command: command.clone(),
                enabled: state != "Disabled",
                publisher,
                description: desc,
                source: AutostartSource::TaskScheduler,
                safety_rating: rating,
                file_exists: std::path::Path::new(
                    command
                        .trim_matches('"')
                        .split(' ')
                        .next()
                        .unwrap_or(&command),
                )
                .exists(),
            })
        })
        .collect()
}

pub fn toggle_task(id: &str, enable: bool) -> Result<(), String> {
    let task_path = id.strip_prefix("TASK:").ok_or("Invalid Task ID")?;
    // We expect ID like "TASK:\Microsoft\Windows\OneDrive\OneDrive Standalone Update Task"
    // PowerShell Disable-ScheduledTask expects -TaskName (if unique) or -TaskPath + -TaskName.
    // Simpler: Just use proper path/name if possible.
    // The ID logic above constructs `TrimEnd('\') + '\' + $task.TaskName` which is basically full path.
    // Let's try passing the full path as TaskName, often works or needs splitting.
    // Robust way: Split last part as Name, rest as Path.

    let path_obj = std::path::Path::new(task_path);
    let name = path_obj
        .file_name()
        .ok_or("Invalid task name")?
        .to_string_lossy();
    let parent = path_obj
        .parent()
        .map(|p| p.to_string_lossy())
        .unwrap_or_default();

    // Convert parent "\" to "\" if root, or "\Folder"
    let folder = if parent == "\\" || parent.is_empty() {
        "\\".to_string()
    } else {
        parent.to_string()
    };

    let cmd = if enable {
        "Enable-ScheduledTask"
    } else {
        "Disable-ScheduledTask"
    };

    let script = format!(
        "{} -TaskName '{}' -TaskPath '{}' -ErrorAction Stop",
        cmd, name, folder
    );

    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &script])
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}
