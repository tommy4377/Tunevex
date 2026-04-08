# BUGFIX_CHECKLIST.md

| Bug | Status | Summary |
|-----|--------|---------|
| BUG-01 | [x] | Registry backup now persists to disk via backup_mgr.save() |
| BUG-02 | [x] | undo_tweak now async fn with spawn_blocking |
| BUG-03 | [x] | FileOperation now executes (Delete/Copy/Move) instead of silent no-op |
| BUG-04 | [x] | compress_folder/decompress_folder use spawn_blocking |
| BUG-05 | [x] | App removal script exits 1 when all attempts fail |
| BUG-06 | [x] | WMI adapter lookup uses InterfaceIndex matching |
| BUG-07 | [x] | Tamper Protection check outputs False instead of unhandled string |
| BUG-08 | [x] | priv_nvidia_telemetry changed to WarningLevel::Careful |
| BUG-09 | [x] | create_restore_point wraps blocking PowerShell in spawn_blocking |
| BUG-10 | [x] | flush_dns_cache wraps blocking ipconfig in spawn_blocking |
| BUG-11 | [x] | clear_temp_files wraps blocking fs operations in spawn_blocking |
| BUG-12 | [x] | scan_startup wraps synchronous scan_all_startup_items() in spawn_blocking |
| BUG-13 | [x] | benchmark_dns wraps run_benchmark() in spawn_blocking |
| BUG-14 | [x] | undo_tweak now emits progress events (fixed with BUG-02) |
| BUG-15 | [x] | undo_tweak double-revert prevented (fixed with BUG-02) |
| BUG-16 | [x] | check_category spawn_blocking now awaited |
| BUG-17 | [x] | RegistryValue::MultiString check now actually reads and compares value |
| BUG-18 | [x] | ScheduledTaskDisable now uses path field with -TaskPath parameter |
| BUG-19 | [x] | sec_uac_lower check now verifies both ConsentPromptBehaviorAdmin AND PromptOnSecureDesktop |
| BUG-20 | [x] | sec_disable_remote_services check now verifies all 5 services |
| BUG-21 | [x] | sec_disable_windows_update check now verifies all 7 services |
| BUG-22 | [x] | SystemMonitor GPU thread has cancellation channel + Drop cleanup |
| BUG-23 | [x] | activation_remove_office uses exit 1 instead of return |
| BUG-24 | [x] | input_disable_mousetrails revert uses RegistryDelete, check handles absent key |
| BUG-25 | [x] | SystemMonitor mutex poisoning handled gracefully with map_err |
| BUG-26 | [x] | All RegistryDelete in revert_operations changed to RegistrySet with defaults (16 instances) |
| BUG-27 | [x] | sec_disable_remote_assistance revert fixed by BUG-01 (backup now persists) |
| BUG-28 | [x] | net_disable_checksum_offload now includes IPv6 variants in apply and revert |
| BUG-29 | [x] | net_configure_rss revert uses ClosestProcessor instead of NUMAStatic |
| BUG-30 | [x] | net_tcp_autotuning check returns True/False instead of raw Normal value |
| BUG-31 | [x] | net_msi_nic_high revert sets MSISupported=0 instead of deleting key |
| BUG-32 | [x] | cpu_large_system_cache removed (functionally inert, 0 is Windows default) |
| BUG-33 | [x] | cpu_ultimate_performance GUID parsing uses regex instead of fragile split |
| BUG-34 | [x] | activation_ohook uses separate mas_ohook.cmd cache path |
| BUG-35 | [x] | MAS script URL pins specific commit hash for integrity |
| BUG-36 | [x] | priv_disable_telemetry revert deletes policy keys instead of hardcoding AllowTelemetry=3 |
| BUG-37 | [x] | gaming_disable_gamebar now stops/disables Xbox background services |
| BUG-38 | [x] | toggle_item("EXP:...") now toggles explorer shell extensions via registry rename |
| ARCH-00 | [x] | All 20 PowerShell calls verified: either in spawn_blocking, thread::spawn, or sync init |
| ARCH-01 | [x] | set_startup_item_enabled wrapped in spawn_blocking |
| ARCH-02 | [x] | apply_dns_server wrapped in spawn_blocking |
