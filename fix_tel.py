import re

with open('src-tauri/src/modules/privacy/telemetry.rs', 'r') as f:
    text = f.read()

# Replace Check
text = re.sub(
    r'TweakCheck::Powershell \{\s*script: r\#\"[\s\S]*?Microsoft Compatibility Appraiser[\s\S]*?\"\#\.to_string\(\),\s*expected_output: \"True\"\.to_string\(\),\s*\}',
    lambda m: r'TweakCheck::ScheduledTaskDisabled { name: "\\Microsoft\\Windows\\Application Experience\\Microsoft Compatibility Appraiser".to_string() }',
    text
)

# Replace Revert
text = re.sub(
    r'revert_operations: Some\(vec\!\[\s*TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?schtasks /Change /TN \ /Enable[\s\S]*?\"\#\.to_string\(\),\s*\}\s*\]\),',
    lambda m: r'''revert_operations: Some(vec![
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Application Experience".to_string(), name: "Microsoft Compatibility Appraiser".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Application Experience".to_string(), name: "ProgramDataUpdater".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Autochk".to_string(), name: "Proxy".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Customer Experience Improvement Program".to_string(), name: "Consolidator".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Customer Experience Improvement Program".to_string(), name: "UsbCeip".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\DiskDiagnostic".to_string(), name: "Microsoft-Windows-DiskDiagnosticDataCollector".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Feedback\\Siuf".to_string(), name: "DmClient".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\Feedback\\Siuf".to_string(), name: "DmClientOnScenarioDownload".to_string() },
                TweakOperation::ScheduledTaskEnable { path: "\\Microsoft\\Windows\\PI".to_string(), name: "Sqm-Tasks".to_string() }
            ]),''',
    text
)

# Replace Apply
text = re.sub(
    r'operations: vec\!\[\s*TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?schtasks /Change /TN \ /Disable[\s\S]*?\"\#\.to_string\(\),\s*\}\s*\]\s*\},',
    lambda m: r'''operations: vec![
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Application Experience".to_string(), name: "Microsoft Compatibility Appraiser".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Application Experience".to_string(), name: "ProgramDataUpdater".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Autochk".to_string(), name: "Proxy".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Customer Experience Improvement Program".to_string(), name: "Consolidator".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Customer Experience Improvement Program".to_string(), name: "UsbCeip".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\DiskDiagnostic".to_string(), name: "Microsoft-Windows-DiskDiagnosticDataCollector".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Feedback\\Siuf".to_string(), name: "DmClient".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\Feedback\\Siuf".to_string(), name: "DmClientOnScenarioDownload".to_string() },
                TweakOperation::ScheduledTaskDisable { path: "\\Microsoft\\Windows\\PI".to_string(), name: "Sqm-Tasks".to_string() }
            ]
        },''',
    text
)

with open('src-tauri/src/modules/privacy/telemetry.rs', 'w') as f:
    f.write(text)
