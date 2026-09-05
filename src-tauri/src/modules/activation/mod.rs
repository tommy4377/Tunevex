//! License status and official activation helpers.
//!
//! This module intentionally uses only Windows/Office's installed licensing
//! tools. It never downloads or executes third-party activation scripts.

use crate::modules::types::{Tweak, TweakCategory, TweakOperation, TweakType, WarningLevel};

pub fn get_activation_tweaks() -> Vec<Tweak> {
    vec![
        Tweak {
            id: "activation_check_windows".to_string(),
            category: TweakCategory::Activation,
            name: "Check Windows License".to_string(),
            description: "Shows the license status reported by Windows.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
Write-Host '=== WINDOWS LICENSE STATUS ===' -ForegroundColor Cyan
$licenses = Get-CimInstance SoftwareLicensingProduct -Filter "PartialProductKey IS NOT NULL" |
    Where-Object { $_.Name -like '*Windows*' }
if (-not $licenses) { Write-Host 'No installed Windows product key was found.' -ForegroundColor Yellow; exit 0 }
$licenses | Select-Object Name, Description, PartialProductKey, LicenseStatus | Format-List
"#.to_string(),
            }],
        },
        Tweak {
            id: "activation_windows_online".to_string(),
            category: TweakCategory::Activation,
            name: "Activate Windows Online".to_string(),
            description: "Asks Microsoft's built-in licensing service to activate the currently installed genuine product key.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$slmgr = Join-Path $env:SystemRoot 'System32\slmgr.vbs'
$process = Start-Process -FilePath 'cscript.exe' -ArgumentList @('//nologo', $slmgr, '/ato') -Wait -PassThru -NoNewWindow
if ($process.ExitCode -ne 0) { throw "Windows activation failed with exit code $($process.ExitCode)." }
Write-Host 'Windows activation request completed.' -ForegroundColor Green
"#.to_string(),
            }],
        },
        Tweak {
            id: "activation_check_office".to_string(),
            category: TweakCategory::Activation,
            name: "Check Office License".to_string(),
            description: "Shows the license status reported by the installed Office licensing tool.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$paths = @(
  "$env:ProgramFiles\Microsoft Office\Office16\ospp.vbs",
  "${env:ProgramFiles(x86)}\Microsoft Office\Office16\ospp.vbs",
  "$env:ProgramFiles\Microsoft Office\root\Office16\ospp.vbs",
  "${env:ProgramFiles(x86)}\Microsoft Office\root\Office16\ospp.vbs"
)
$tool = $paths | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $tool) { throw 'The Office licensing tool was not found. Office may not be installed.' }
& cscript.exe //nologo $tool /dstatus
if ($LASTEXITCODE -ne 0) { throw "Office license check failed with exit code $LASTEXITCODE." }
"#.to_string(),
            }],
        },
        Tweak {
            id: "activation_office_online".to_string(),
            category: TweakCategory::Activation,
            name: "Activate Office Online".to_string(),
            description: "Asks Office's built-in licensing tool to activate the currently installed genuine product key.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Action,
            enabled: false,
            check: None,
            revert_operations: None,
            operations: vec![TweakOperation::Powershell {
                script: r#"
$ErrorActionPreference = 'Stop'
$paths = @(
  "$env:ProgramFiles\Microsoft Office\Office16\ospp.vbs",
  "${env:ProgramFiles(x86)}\Microsoft Office\Office16\ospp.vbs",
  "$env:ProgramFiles\Microsoft Office\root\Office16\ospp.vbs",
  "${env:ProgramFiles(x86)}\Microsoft Office\root\Office16\ospp.vbs"
)
$tool = $paths | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $tool) { throw 'The Office licensing tool was not found. Office may not be installed.' }
& cscript.exe //nologo $tool /act
if ($LASTEXITCODE -ne 0) { throw "Office activation failed with exit code $LASTEXITCODE." }
"#.to_string(),
            }],
        },
    ]
}
