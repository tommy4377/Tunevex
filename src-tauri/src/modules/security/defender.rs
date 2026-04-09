//! Windows Defender Tweaks
//!
//! Controls for Windows Defender real-time protection, cloud features,
//! sample submission, and exclusions.
//!
//! All checks and operations use native Rust / winreg / sc.exe —
//! zero PowerShell or TweakCheck::Powershell / TweakOperation::Powershell.
//!
//! Registry reference:
//!   Tamper Protection flag:
//!     HKLM\SOFTWARE\Microsoft\Windows Defender\Features\TamperProtection
//!     5 = ON (enforced), 4 = OFF, 0/2 = intermediate/unmanaged
//!
//!   Policy keys (written by this app to disable Defender features):
//!     HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\...
//!
//!   Exclusion paths:
//!     HKLM\SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths
//!     (each path is stored as a value-name with REG_DWORD data = 0)

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_defender_tweaks() -> Vec<Tweak> {
    vec![
        // ── Disable Real-time Protection ──────────────────────────────────────
        Tweak {
            id: "sec_disable_realtime".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Real-time Protection".to_string(),
            description: "Disables Windows Defender real-time scanning. \
                WARNING: Leaves system vulnerable to malware."
                .to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,

            // ── CHECK ──────────────────────────────────────────────────────────
            // Native registry check:
            //   1. Tamper Protection must be OFF (TamperProtection != 5)
            //   2. DisableRealtimeMonitoring policy key must be 1
            check: Some(TweakCheck::MpComputerStatus {
                check: "realtime_disabled".to_string(),
            }),

            // ── APPLY ─────────────────────────────────────────────────────────
            // Prerequisite guard: read TamperProtection from registry natively.
            // If it is ON (== 5) we refuse to write and return an error so the
            // UI shows a meaningful message.  The RegistrySet ops below are
            // only reached when Tamper Protection is already off.
            operations: vec![
                // Tamper-Protection guard: check the registry value synchronously
                // before touching anything.  We encode this as a Command that
                // runs reg.exe to query the key and exits 1 if TP is active.
                // This keeps the "check before apply" contract without PowerShell.
                TweakOperation::Command {
                    cmd: "cmd".to_string(),
                    args: vec![
                        "/C".to_string(),
                        // reg query exits 1 if the key/value is missing,
                        // so we combine with findstr: if "0x5" is found → TP is ON → error
                        concat!(
                            r#"for /f "tokens=3" %v in "# ,
                            r#"('reg query "HKLM\SOFTWARE\Microsoft\Windows Defender\Features" "# ,
                            r#"/v TamperProtection 2^>nul') do "# ,
                            r#"if "%v"=="0x5" (echo Tamper Protection is ON. Disable it in Windows Security first. >&2 & exit /b 1)"#
                        ).to_string(),
                    ],
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableRealtimeMonitoring".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableBehaviorMonitoring".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableOnAccessProtection".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableScanOnRealtimeEnable".to_string(),
                    value: RegistryValue::DWord(1),
                },
            ],

            // ── REVERT ────────────────────────────────────────────────────────
            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableRealtimeMonitoring".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableBehaviorMonitoring".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableOnAccessProtection".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Real-Time Protection"
                        .to_string(),
                    key: "DisableScanOnRealtimeEnable".to_string(),
                },
            ]),
        },

        // ── Disable Cloud-Delivered Protection ───────────────────────────────
        Tweak {
            id: "sec_disable_cloud".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Cloud-Delivered Protection".to_string(),
            description: "Disables cloud-based threat detection. \
                Reduces network usage but may miss new threats."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,

            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                key: "SpynetReporting".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),

            operations: vec![
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                    key: "SpynetReporting".to_string(),
                    value: RegistryValue::DWord(0),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\MpEngine".to_string(),
                    key: "MpCloudBlockLevel".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],

            revert_operations: Some(vec![
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                    key: "SpynetReporting".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\MpEngine".to_string(),
                    key: "MpCloudBlockLevel".to_string(),
                },
            ]),
        },

        // ── Disable Sample Submission ─────────────────────────────────────────
        Tweak {
            id: "sec_disable_samples".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Sample Submission".to_string(),
            description:
                "Stops Defender from sending file samples to Microsoft for analysis.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,

            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                key: "SubmitSamplesConsent".to_string(),
                expected_value: RegistryValue::DWord(2),
            }),

            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                key: "SubmitSamplesConsent".to_string(),
                value: RegistryValue::DWord(2), // 2 = Never send
            }],

            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender\\Spynet".to_string(),
                key: "SubmitSamplesConsent".to_string(),
            }]),
        },

        // ── Add Developer Folder Exclusions ───────────────────────────────────
        //
        // Exclusion paths are stored in the registry at:
        //   HKLM\SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths
        // Each path is a REG_DWORD value-name with data = 0.
        //
        // NOTE: Tamper Protection blocks writes to this key when active.
        //       The user must turn off TP first in Windows Security settings.
        Tweak {
            id: "sec_dev_exclusions".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Add Developer Folder Exclusions".to_string(),
            description: "Excludes common developer folders from Defender scanning \
                (improves build times). Requires Tamper Protection to be OFF."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,

            // ── CHECK ──────────────────────────────────────────────────────────
            // True when all exclusion paths exist as value names in the registry key.
            check: Some(TweakCheck::DefenderExclusionPath {
                paths: defender_dev_exclusion_paths(),
            }),

            // ── APPLY ─────────────────────────────────────────────────────────
            operations: vec![TweakOperation::DefenderExclusion {
                paths: defender_dev_exclusion_paths(),
                action: "add".to_string(),
            }],

            // ── REVERT ────────────────────────────────────────────────────────
            revert_operations: Some(vec![TweakOperation::DefenderExclusion {
                paths: defender_dev_exclusion_paths(),
                action: "remove".to_string(),
            }]),
        },

        // ── Disable PUA Protection ────────────────────────────────────────────
        Tweak {
            id: "sec_disable_pua".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable PUA Protection".to_string(),
            description: "Disables detection of Potentially Unwanted Applications. \
                Useful for tools like cracks, keygens."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,

            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                key: "PUAProtection".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),

            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                key: "PUAProtection".to_string(),
                value: RegistryValue::DWord(0),
            }],

            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                key: "PUAProtection".to_string(),
            }]),
        },

        // ── Disable Windows Defender Completely ───────────────────────────────
        //
        // Check: reads Tamper Protection flag + both policy DWORD keys natively.
        // Apply: sets policy registry keys + stops/disables services via sc.exe.
        // Revert: deletes policy keys + re-enables services via sc.exe.
        //
        // The user MUST disable Tamper Protection first (Windows Security UI).
        // We guard against TP being active by checking the registry value before
        // writing; the TweakCheck::MpComputerStatus check also returns false when
        // TP is on, keeping the UI toggle in the correct OFF state.
        Tweak {
            id: "sec_disable_defender".to_string(),
            category: TweakCategory::SecurityPrivacy,
            name: "Disable Windows Defender Completely".to_string(),
            description: "Completely disables Windows Defender antivirus. EXTREME RISK. \
                NOTE: You MUST disable 'Tamper Protection' manually in \
                Windows Security settings first."
                .to_string(),
            warning_level: WarningLevel::Dangerous,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,

            // ── CHECK ──────────────────────────────────────────────────────────
            // Native registry check: TP off + DisableAntiSpyware==1 + DisableAntiVirus==1
            check: Some(TweakCheck::MpComputerStatus {
                check: "av_disabled".to_string(),
            }),

            // ── APPLY ─────────────────────────────────────────────────────────
            operations: vec![
                // Guard: refuse if Tamper Protection is ON (same cmd-guard as realtime)
                TweakOperation::Command {
                    cmd: "cmd".to_string(),
                    args: vec![
                        "/C".to_string(),
                        concat!(
                            r#"for /f "tokens=3" %v in "# ,
                            r#"('reg query "HKLM\SOFTWARE\Microsoft\Windows Defender\Features" "# ,
                            r#"/v TamperProtection 2^>nul') do "# ,
                            r#"if "%v"=="0x5" (echo Tamper Protection is ON. Disable it in Windows Security first. >&2 & exit /b 1)"#
                        ).to_string(),
                    ],
                },
                // Write policy keys
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "DisableAntiSpyware".to_string(),
                    value: RegistryValue::DWord(1),
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "DisableAntiVirus".to_string(),
                    value: RegistryValue::DWord(1),
                },
                // Disable Defender services natively via sc.exe
                TweakOperation::DefenderServiceControl {
                    services: vec![
                        "WinDefend".to_string(),
                        "WdNisSvc".to_string(),
                        "Sense".to_string(),
                    ],
                    action: "disable".to_string(),
                },
            ],

            // ── REVERT ────────────────────────────────────────────────────────
            revert_operations: Some(vec![
                // Remove policy keys (lets Defender re-read its own internal state)
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "DisableAntiSpyware".to_string(),
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Microsoft\\Windows Defender".to_string(),
                    key: "DisableAntiVirus".to_string(),
                },
                // Re-enable Defender services natively via sc.exe
                TweakOperation::DefenderServiceControl {
                    services: vec![
                        "WinDefend".to_string(),
                        "WdNisSvc".to_string(),
                        "Sense".to_string(),
                    ],
                    action: "enable".to_string(),
                },
            ]),
        },
    ]
}

// ── Developer exclusion path list ─────────────────────────────────────────────
//
// These paths are stored as REG_DWORD value names (data = 0) under
//   HKLM\SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths
//
// We expand %USERPROFILE% and %APPDATA% at runtime inside the helper functions
// in commands.rs.  Here we record them with environment-variable syntax because
// the registry key itself stores expanded paths on real systems, but Windows
// Defender also accepts unexpanded paths for user-profile folders when they are
// added via the registry directly.
//
// To keep things simple and reliable we store concrete expanded paths.
// The helper `defender_dev_exclusion_paths()` expands them at call-time.
fn defender_dev_exclusion_paths() -> Vec<String> {
    let profile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    let appdata =
        std::env::var("APPDATA").unwrap_or_else(|_| format!("{}\\AppData\\Roaming", profile));

    vec![
        format!("{}\\source", profile),
        format!("{}\\repos", profile),
        format!("{}\\.cargo", profile),
        format!("{}\\.rustup", profile),
        format!("{}\\node_modules", profile),
        format!("{}\\.npm", profile),
        format!("{}\\go", profile),
        format!("{}\\npm", appdata),
    ]
}
