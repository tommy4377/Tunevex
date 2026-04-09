import re

with open('src-tauri/src/modules/debloat/features.rs', 'r') as f:
    text = f.read()

# Fix Checks
text = re.sub(
    r'TweakCheck::Powershell \{\s*script: r\#\"[\s\S]*?FeatureName "([^"]+)"[\s\S]*?\"\#\.to_string\(\),\s*expected_output: \"True\"\.to_string\(\),\s*\}',
    r'TweakCheck::CommandOutputContains { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Get-FeatureInfo".to_string(), "/FeatureName:\1".to_string()], contains: "Disabled".to_string() }',
    text
)

# Fix Single Reverts
text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?dism /Online /Enable-Feature /FeatureName:"([^"]+)" /NoRestart[\s\S]*?\"\#\.to_string\(\),\s*\}',
    r'TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:\1".to_string(), "/NoRestart".to_string()] }',
    text
)

# Fix Single Operations
text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?dism /Online /Disable-Feature /FeatureName:"([^"]+)" /NoRestart[\s\S]*?\"\#\.to_string\(\),\s*\}',
    r'TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:\1".to_string(), "/NoRestart".to_string()] }',
    text
)

# Replace the giant array manually
text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"\s*Write-Host \"Enabling Printer and XPS features\.\.\.\"[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-Foundation-InternetPrinting-Client".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:LPDPrintService".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-Foundation-LPRPortMonitor".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-PrintToPDFServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Printing-XPSServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:Xps-Foundation-Xps-Viewer".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Enable-Feature".to_string(), "/FeatureName:WorkFolders-Client".to_string(), "/NoRestart".to_string()] }''',
    text
)

text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"\s*Write-Host \"Disabling Printer and XPS features\.\.\.\"[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-Foundation-InternetPrinting-Client".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:LPDPrintService".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-Foundation-LPRPortMonitor".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-PrintToPDFServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Printing-XPSServices-Features".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:Xps-Foundation-Xps-Viewer".to_string(), "/NoRestart".to_string()] },
                TweakOperation::Command { cmd: "dism".to_string(), args: vec!["/Online".to_string(), "/Disable-Feature".to_string(), "/FeatureName:WorkFolders-Client".to_string(), "/NoRestart".to_string()] }''',
    text
)

with open('src-tauri/src/modules/debloat/features.rs', 'w') as f:
    f.write(text)
