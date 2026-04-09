import re

with open('src-tauri/src/modules/debloat/services.rs', 'r') as f:
    text = f.read()

# 1. Misc services
text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?\ = \@\("WMPNetworkSvc", "MapsBroker", "Fax", "RetailDemo", "WalletService", "PhoneSvc", "TapiSrv"\)[\s\S]*?Set-Service[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::ServiceSetMode { name: "WMPNetworkSvc".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "MapsBroker".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "Fax".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "RetailDemo".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "WalletService".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "PhoneSvc".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "TapiSrv".to_string(), mode: "Manual".to_string() }''',
    text
)

text = re.sub(
    r'TweakCheck::Powershell \{\s*script: r\#\"[\s\S]*?\ = Get-Service[\s\S]*?\"\#\.to_string\(\),\s*expected_output: \"True\"\.to_string\(\),\s*\}',
    'TweakCheck::MultiServiceDisabled { names: vec!["WMPNetworkSvc".to_string(), "MapsBroker".to_string(), "Fax".to_string(), "RetailDemo".to_string(), "WalletService".to_string(), "PhoneSvc".to_string(), "TapiSrv".to_string()] }',
    text
)

text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?\ = \@\("WMPNetworkSvc", "MapsBroker", "Fax", "RetailDemo", "WalletService", "PhoneSvc", "TapiSrv"\)[\s\S]*?Stop-Service[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::ServiceDisable { name: "WMPNetworkSvc".to_string() },
                TweakOperation::ServiceDisable { name: "MapsBroker".to_string() },
                TweakOperation::ServiceDisable { name: "Fax".to_string() },
                TweakOperation::ServiceDisable { name: "RetailDemo".to_string() },
                TweakOperation::ServiceDisable { name: "WalletService".to_string() },
                TweakOperation::ServiceDisable { name: "PhoneSvc".to_string() },
                TweakOperation::ServiceDisable { name: "TapiSrv".to_string() }''',
    text
)

# 2. Edge services
text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?Set-Service -Name "MicrosoftEdgeElevationService"[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::ServiceSetMode { name: "MicrosoftEdgeElevationService".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "edgeupdate".to_string(), mode: "Auto".to_string() },
                TweakOperation::ServiceSetMode { name: "edgeupdatem".to_string(), mode: "Manual".to_string() }''',
    text
)

text = re.sub(
    r'TweakCheck::Powershell \{\s*script: r\#\"[\s\S]*?Get-Service edgeupdate[\s\S]*?\"\#\.to_string\(\),\s*expected_output: \"True\"\.to_string\(\),\s*\}',
    'TweakCheck::MultiServiceDisabled { names: vec!["MicrosoftEdgeElevationService".to_string(), "edgeupdate".to_string(), "edgeupdatem".to_string()] }',
    text
)

text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?\ = \@\("MicrosoftEdgeElevationService", "edgeupdate", "edgeupdatem"\)[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::ServiceDisable { name: "MicrosoftEdgeElevationService".to_string() },
                TweakOperation::ServiceDisable { name: "edgeupdate".to_string() },
                TweakOperation::ServiceDisable { name: "edgeupdatem".to_string() }''',
    text
)

# 3. Printer services
text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?Set-Service -Name "Spooler"[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::ServiceSetMode { name: "Spooler".to_string(), mode: "Auto".to_string() },
                TweakOperation::Command { cmd: "sc".to_string(), args: vec!["start".to_string(), "Spooler".to_string()] }''',
    text
)

text = re.sub(
    r'TweakCheck::Powershell \{\s*script: r\#\"[\s\S]*?Get-Service Spooler[\s\S]*?\"\#\.to_string\(\),\s*expected_output: \"True\"\.to_string\(\),\s*\}',
    'TweakCheck::ServiceDisabled { name: "Spooler".to_string() }',
    text
)

text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?Stop-Service -Name "Spooler"[\s\S]*?\"\#\.to_string\(\),\s*\}',
    'TweakOperation::ServiceDisable { name: "Spooler".to_string() }',
    text
)

# 4. Bluetooth services
text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?\ = \@\("BTAGService", "bthserv"\)[\s\S]*?Set-Service[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::ServiceSetMode { name: "BTAGService".to_string(), mode: "Manual".to_string() },
                TweakOperation::ServiceSetMode { name: "bthserv".to_string(), mode: "Manual".to_string() }''',
    text
)

text = re.sub(
    r'TweakCheck::Powershell \{\s*script: r\#\"[\s\S]*?Get-Service bthserv[\s\S]*?\"\#\.to_string\(\),\s*expected_output: \"True\"\.to_string\(\),\s*\}',
    'TweakCheck::MultiServiceDisabled { names: vec!["BTAGService".to_string(), "bthserv".to_string()] }',
    text
)

text = re.sub(
    r'TweakOperation::Powershell \{\s*script: r\#\"[\s\S]*?\ = \@\("BTAGService", "bthserv"\)[\s\S]*?Stop-Service[\s\S]*?\"\#\.to_string\(\),\s*\}',
    '''TweakOperation::ServiceDisable { name: "BTAGService".to_string() },
                TweakOperation::ServiceDisable { name: "bthserv".to_string() }''',
    text
)

with open('src-tauri/src/modules/debloat/services.rs', 'w') as f:
    f.write(text)
