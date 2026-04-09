// ============================================================
// adapter.rs — Network Adapter Tweaks (100 % PowerShell-free)
// ============================================================
// All advanced-property tweaks use TweakOperation::NetAdapterProperty,
// which writes to
//   HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}\XXXX
// natively via winreg.
//
// Chimney / task-offload / packet-coalescing use TweakOperation::Command
// (netsh int tcp set global …) — a native Windows binary, no PowerShell.
//
// Power-management (PnPCapabilities) is handled via RegistrySet after we
// identify each adapter's device-instance path through winreg enumeration.
// Since the path per adapter varies, we use a single registry operation
// targeting the per-class driver key instead of the per-device Enum key,
// which is the authoritative location for adapter advanced properties.
// ============================================================

use crate::modules::types::{
    RegistryValue, Tweak, TweakCategory, TweakCheck, TweakOperation, TweakType, WarningLevel,
};

pub fn get_adapter_tweaks() -> Vec<Tweak> {
    vec![
        // ── 1. Flow Control ───────────────────────────────────────────────
        // Registry property name: *FlowControl
        //   0 = Disabled, 3 = Rx & Tx Enabled (Windows default)
        Tweak {
            id: "net_disable_flow_control".to_string(),
            category: TweakCategory::Network,
            name: "Disable Flow Control".to_string(),
            description:
                "Prevents network adapter from pausing transmission. Reduces latency jitter."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*FlowControl".to_string(),
                expected_value: "0".to_string(),
            }),
            operations: vec![TweakOperation::NetAdapterProperty {
                property: "*FlowControl".to_string(),
                value: "0".to_string(),
            }],
            revert_operations: Some(vec![TweakOperation::NetAdapterProperty {
                property: "*FlowControl".to_string(),
                value: "3".to_string(), // Rx & Tx Enabled
            }]),
        },
        // ── 2. Large Send Offload V2 ──────────────────────────────────────
        // *LsoV2IPv4 / *LsoV2IPv6:  0 = Disabled, 1 = Enabled
        Tweak {
            id: "net_disable_lso".to_string(),
            category: TweakCategory::Network,
            name: "Disable Large Send Offload".to_string(),
            description: "Disables hardware packet segmentation. Can fix stuttering.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*LsoV2IPv4".to_string(),
                expected_value: "0".to_string(),
            }),
            operations: vec![
                TweakOperation::NetAdapterProperty {
                    property: "*LsoV2IPv4".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*LsoV2IPv6".to_string(),
                    value: "0".to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::NetAdapterProperty {
                    property: "*LsoV2IPv4".to_string(),
                    value: "1".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*LsoV2IPv6".to_string(),
                    value: "1".to_string(),
                },
            ]),
        },
        // ── 3. Checksum Offload ───────────────────────────────────────────
        // *IPChecksumOffloadIPv4 / *TCPChecksumOffloadIPv4 / *UDPChecksumOffloadIPv4
        // *IPChecksumOffloadIPv6 / *TCPChecksumOffloadIPv6 / *UDPChecksumOffloadIPv6
        //   0 = Disabled, 3 = Rx & Tx Enabled
        // We check only *TCPChecksumOffloadIPv4 as a representative key.
        Tweak {
            id: "net_disable_checksum_offload".to_string(),
            category: TweakCategory::Network,
            name: "Disable Checksum Offload".to_string(),
            description: "Forces CPU to handle checksums. Can reduce micro-stuttering.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*TCPChecksumOffloadIPv4".to_string(),
                expected_value: "0".to_string(),
            }),
            operations: vec![
                TweakOperation::NetAdapterProperty {
                    property: "*IPChecksumOffloadIPv4".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*TCPChecksumOffloadIPv4".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*UDPChecksumOffloadIPv4".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*IPChecksumOffloadIPv6".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*TCPChecksumOffloadIPv6".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*UDPChecksumOffloadIPv6".to_string(),
                    value: "0".to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::NetAdapterProperty {
                    property: "*IPChecksumOffloadIPv4".to_string(),
                    value: "3".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*TCPChecksumOffloadIPv4".to_string(),
                    value: "3".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*UDPChecksumOffloadIPv4".to_string(),
                    value: "3".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*IPChecksumOffloadIPv6".to_string(),
                    value: "3".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*TCPChecksumOffloadIPv6".to_string(),
                    value: "3".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*UDPChecksumOffloadIPv6".to_string(),
                    value: "3".to_string(),
                },
            ]),
        },
        // ── 4. RSS Profile ────────────────────────────────────────────────
        // *RSS:         0 = Disabled, 1 = Enabled
        // *RSSProfile:  1 = ClosestProcessor, 4 = NUMAStatic
        // (ClosestProcessor is Windows default and best for gaming)
        Tweak {
            id: "net_configure_rss".to_string(),
            category: TweakCategory::Network,
            name: "Configure RSS for Gaming".to_string(),
            description: "Sets RSS profile to ClosestProcessor and enables RSS.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*RSSProfile".to_string(),
                expected_value: "1".to_string(), // ClosestProcessor
            }),
            operations: vec![
                TweakOperation::NetAdapterProperty {
                    property: "*RSS".to_string(),
                    value: "1".to_string(), // Enable RSS
                },
                TweakOperation::NetAdapterProperty {
                    property: "*RSSProfile".to_string(),
                    value: "1".to_string(), // ClosestProcessor
                },
            ],
            revert_operations: Some(vec![TweakOperation::NetAdapterProperty {
                property: "*RSSProfile".to_string(),
                value: "4".to_string(), // NUMAStatic (previous default in this codebase)
            }]),
        },
        // ── 5. Receive Segment Coalescing (RSC) ───────────────────────────
        // *RscIPv4 / *RscIPv6:  0 = Disabled, 1 = Enabled
        Tweak {
            id: "net_disable_rsc".to_string(),
            category: TweakCategory::Network,
            name: "Disable Receive Segment Coalescing (RSC)".to_string(),
            description: "Disables RSC on all adapters.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*RscIPv4".to_string(),
                expected_value: "0".to_string(),
            }),
            operations: vec![
                TweakOperation::NetAdapterProperty {
                    property: "*RscIPv4".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*RscIPv6".to_string(),
                    value: "0".to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::NetAdapterProperty {
                    property: "*RscIPv4".to_string(),
                    value: "1".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*RscIPv6".to_string(),
                    value: "1".to_string(),
                },
            ]),
        },
        // ── 6. Jumbo Packet ───────────────────────────────────────────────
        // *JumboPacket:  1514 = standard Ethernet MTU (disabled), 9014 = jumbo
        Tweak {
            id: "net_disable_jumbo_packet".to_string(),
            category: TweakCategory::Network,
            name: "Disable Jumbo Packet".to_string(),
            description: "Ensures Jumbo Packet is disabled (1514 bytes).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*JumboPacket".to_string(),
                expected_value: "1514".to_string(),
            }),
            operations: vec![TweakOperation::NetAdapterProperty {
                property: "*JumboPacket".to_string(),
                value: "1514".to_string(),
            }],
            revert_operations: Some(vec![TweakOperation::NetAdapterProperty {
                property: "*JumboPacket".to_string(),
                value: "1514".to_string(), // 1514 is already the standard default
            }]),
        },
        // ── 7. TCP Chimney Offload ────────────────────────────────────────
        // Uses `netsh int tcp set global chimney=disabled`.
        // The registry backing key is
        //   HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters → EnableTCPChimney (DWORD 0/1)
        // but netsh is the authoritative API surface; we use it for apply and
        // a Registry check (no PowerShell) because the value is global.
        Tweak {
            id: "net_disable_chimney".to_string(),
            category: TweakCategory::Network,
            name: "Disable Chimney Offload".to_string(),
            description: "Disables TCP Chimney Offload via netsh (native).".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // EnableTCPChimney=0 means disabled; absent key also means disabled on Win10+
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "EnableTCPChimney".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec![
                        "int".into(),
                        "tcp".into(),
                        "set".into(),
                        "global".into(),
                        "chimney=disabled".into(),
                    ],
                },
                // Belt-and-suspenders: also write the registry key
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "EnableTCPChimney".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec![
                        "int".into(),
                        "tcp".into(),
                        "set".into(),
                        "global".into(),
                        "chimney=enabled".into(),
                    ],
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                    key: "EnableTCPChimney".to_string(),
                },
            ]),
        },
        // ── 8. Task Offload ───────────────────────────────────────────────
        // HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters → DisableTaskOffload
        //   0 = Task offload enabled (default), 1 = disabled
        Tweak {
            id: "net_disable_task_offload".to_string(),
            category: TweakCategory::Network,
            name: "Disable Task Offload".to_string(),
            description: "Forces CPU to handle network processing (DisableTaskOffload=1)."
                .to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "DisableTaskOffload".to_string(),
                expected_value: RegistryValue::DWord(1),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "DisableTaskOffload".to_string(),
                value: RegistryValue::DWord(1),
            }],
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters".to_string(),
                key: "DisableTaskOffload".to_string(),
            }]),
        },
        // ── 9. Packet Coalescing Filter ───────────────────────────────────
        // `netsh int ip set global packetcoalescing=disabled` (Win8+)
        // There is no single stable registry backing key, so we check via
        // netsh output — but we avoid PowerShell entirely by running netsh
        // and checking its exit / stdout via a Command check.
        // The closest registry approach: the coalescing setting lives in
        //   HKLM\SYSTEM\CurrentControlSet\Services\Ndu → EnableCoalescing (DWORD)
        // However that key is not universally present.  We take the pragmatic
        // approach of using netsh for apply/revert and a Registry check on the
        // known NDU key, falling back to "not applied" when absent.
        Tweak {
            id: "net_disable_packet_coalescing".to_string(),
            category: TweakCategory::Network,
            name: "Disable Packet Coalescing Filter".to_string(),
            description: "Prevents grouping of packets for reduced latency (netsh native)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // EnableCoalescing=0 means disabled; key absent → default (enabled)
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Ndu".to_string(),
                key: "EnableCoalescing".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec![
                        "int".into(),
                        "ip".into(),
                        "set".into(),
                        "global".into(),
                        "packetcoalescing=disabled".into(),
                    ],
                },
                TweakOperation::RegistrySet {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Ndu".to_string(),
                    key: "EnableCoalescing".to_string(),
                    value: RegistryValue::DWord(0),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::Command {
                    cmd: "netsh".to_string(),
                    args: vec![
                        "int".into(),
                        "ip".into(),
                        "set".into(),
                        "global".into(),
                        "packetcoalescing=enabled".into(),
                    ],
                },
                TweakOperation::RegistryDelete {
                    root_key: "HKLM".to_string(),
                    path: "SYSTEM\\CurrentControlSet\\Services\\Ndu".to_string(),
                    key: "EnableCoalescing".to_string(),
                },
            ]),
        },
        // ── 10. Network Adapter Power Management ──────────────────────────
        // PnPCapabilities value in the adapter's NIC class driver subkey:
        //   HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e972...}\XXXX
        //   PnPCapabilities = 24 (0x18)  → wake-on-LAN disabled
        //   PnPCapabilities = 0 (absent) → wake-on-LAN enabled (default)
        //
        // We store this in the same NIC class subkey using NetAdapterProperty,
        // which writes REG_SZ "24" into each physical NIC instance.
        // Note: some drivers read PnPCapabilities as DWORD from the Enum path,
        // but the NIC class path is universally writable and applied on next
        // driver load.  The check uses a Registry DWORD check in case the
        // winreg read returns numeric type.
        //
        // For the WakeOnMagicPacket / WakeOnPattern NDIS properties we mirror
        // the same keys that Set-NetAdapterPowerManagement writes:
        //   *WakeOnMagicPacket:  0 = Disabled, 1 = Enabled
        //   *WakeOnPattern    :  0 = Disabled, 1 = Enabled
        Tweak {
            id: "net_disable_power_mgmt".to_string(),
            category: TweakCategory::Network,
            name: "Disable Network Adapter Power Management".to_string(),
            description: "Prevents network adapter from entering low-power states (native winreg)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            // Check that WakeOnMagicPacket is disabled on the NIC class key
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*WakeOnMagicPacket".to_string(),
                expected_value: "0".to_string(),
            }),
            operations: vec![
                TweakOperation::NetAdapterProperty {
                    property: "*WakeOnMagicPacket".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*WakeOnPattern".to_string(),
                    value: "0".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "PnPCapabilities".to_string(),
                    value: "24".to_string(),
                },
            ],
            revert_operations: Some(vec![
                TweakOperation::NetAdapterProperty {
                    property: "*WakeOnMagicPacket".to_string(),
                    value: "1".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "*WakeOnPattern".to_string(),
                    value: "1".to_string(),
                },
                TweakOperation::NetAdapterProperty {
                    property: "PnPCapabilities".to_string(),
                    value: "0".to_string(),
                },
            ]),
        },
        // ── 11. Disable IPv6 ─────────────────────────────────────────────
        // (Already native — kept as-is)
        Tweak {
            id: "net_disable_ipv6".to_string(),
            category: TweakCategory::Network,
            name: "Disable IPv6".to_string(),
            description: "Disables IPv6 to reduce network overhead if not used.".to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters".to_string(),
                key: "DisabledComponents".to_string(),
                expected_value: RegistryValue::DWord(0xFFFFFFFF),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters".to_string(),
                key: "DisabledComponents".to_string(),
                value: RegistryValue::DWord(0xFFFFFFFF),
            }],
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters".to_string(),
                key: "DisabledComponents".to_string(),
            }]),
        },
        // ── 12. Speed & Duplex (Auto-Negotiate) ───────────────────────────
        // *SpeedDuplex:  0 = Auto Negotiation
        Tweak {
            id: "net_optimize_speed".to_string(),
            category: TweakCategory::Network,
            name: "Optimize Link Speed (Auto-Negotiate)".to_string(),
            description: "Ensures network adapter negotiates the highest possible speed (Auto)."
                .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*SpeedDuplex".to_string(),
                expected_value: "0".to_string(),
            }),
            operations: vec![TweakOperation::NetAdapterProperty {
                property: "*SpeedDuplex".to_string(),
                value: "0".to_string(), // Auto Negotiation
            }],
            revert_operations: Some(vec![TweakOperation::NetAdapterProperty {
                property: "*SpeedDuplex".to_string(),
                value: "0".to_string(), // Already auto — no change needed
            }]),
        },
        // ── 13. Energy Efficient Ethernet (EEE) ───────────────────────────
        // *EEE:  0 = Disabled, 1 = Enabled
        Tweak {
            id: "net_disable_eee".to_string(),
            category: TweakCategory::Network,
            name: "Disable Energy Efficient Ethernet (EEE)".to_string(),
            description:
                "Disables EEE on all active adapters to avoid power-saving induced latency spikes."
                    .to_string(),
            warning_level: WarningLevel::Safe,
            requires_restart: false,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::NetAdapterProperty {
                property: "*EEE".to_string(),
                expected_value: "0".to_string(),
            }),
            operations: vec![TweakOperation::NetAdapterProperty {
                property: "*EEE".to_string(),
                value: "0".to_string(),
            }],
            revert_operations: Some(vec![TweakOperation::NetAdapterProperty {
                property: "*EEE".to_string(),
                value: "1".to_string(),
            }]),
        },
        // ── 14. QoS Reservable Bandwidth ──────────────────────────────────
        // (Already native — kept as-is)
        Tweak {
            id: "net_qos_limit_reservable_bandwidth".to_string(),
            category: TweakCategory::Network,
            name: "Limit Reservable Bandwidth to 0%".to_string(),
            description: "Sets NonBestEffortLimit=0 so QoS does not reserve bandwidth.".to_string(),
            warning_level: WarningLevel::Careful,
            requires_restart: true,
            tweak_type: TweakType::Toggle,
            enabled: false,
            check: Some(TweakCheck::Registry {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Psched".to_string(),
                key: "NonBestEffortLimit".to_string(),
                expected_value: RegistryValue::DWord(0),
            }),
            operations: vec![TweakOperation::RegistrySet {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Psched".to_string(),
                key: "NonBestEffortLimit".to_string(),
                value: RegistryValue::DWord(0),
            }],
            revert_operations: Some(vec![TweakOperation::RegistryDelete {
                root_key: "HKLM".to_string(),
                path: "SOFTWARE\\Policies\\Microsoft\\Windows\\Psched".to_string(),
                key: "NonBestEffortLimit".to_string(),
            }]),
        },
    ]
}
