# Tweak-engine fixes — 2026-09-05

## Fixed

- ECN detection now matches the exact TCP setting, rather than any occurrence of “Enabled.” TCP timestamps use modern netsh apply/revert/detection; revert explicitly disables timestamps.
- Window Scaling heuristics are correctly named and detected. The operations do not change RSS.
- Teredo checks the explicit persistent configuration, distinguishing Disabled from Default even when the effective default is disabled.
- Scheduled-task detection reads the task's Enabled property through Task Scheduler, avoiding unrelated “Disabled” fields in verbose output.
- MSI enumeration uses PCI hardware/instance paths and ClassGUID. GPU, USB, NIC and NVMe checks use their own target classes. Priority uses Affinity Policy\\DevicePriority; Normal is 2. The driver's MessageNumberLimit is preserved. Empty target sets no longer count as enabled.
- Device operations preserve original per-device values and types in per-tweak journals. Revert restores those values, preserves absence, detects conflicting later changes, and refuses to guess without a backup. NIC apply errors are no longer silently ignored.
- Auto-Tuning, Jumbo Packet, Auto-Negotiate, classic context menu, fixed pagefile and reserved storage have stateful detection. Auto-Tuning and pagefile save prior configurations. Pagefile uses CIM, the actual system drive, and the automatic-management flag.
- Primary-display maximum refresh is detected at the current resolution and color depth. It remains an action with detected state because automatic rollback is unavailable.
- Related TCP Fast Open, initial RTO, congestion-provider and initial-congestion-window detectors were corrected while tracing the same patterns.
- PowerShell queries and operations execute as complete script blocks with terminating input. Failed queries are not treated as successful matches. Registry deletion failures propagate; binary/multi-string backup handling was completed, and successful registry reverts consume their snapshots.
- UI state is read back after apply/revert. Initial state is unknown instead of click history. Profile import refreshes state, category navigation rechecks it, and export queries Windows. Unknown/unqueryable states export as skip. Queryable actions can export enable; actions without rollback export skip when inactive.
- Enabled, Run and failed styling now follows detected state consistently. Twenty-nine existing dashboard/section handlers received mechanical replacements of optimistic boolean assignments with returned detection results.

## Verification

- `cargo test --manifest-path src-tauri/Cargo.toml --lib --no-fail-fast`: **24 passed**.
- `cargo build --manifest-path src-tauri/Cargo.toml --bins`: **passed** (debug app binary).
- `npm.cmd run check`: **passed**, zero errors and zero warnings.
- `npm.cmd run build`: **passed**.
- `git diff --check`: **passed**.
- Tests include TCP matching/RSS invariants, MSI class/priority checks, rollback conflict/absence handling, failed PowerShell queries, missing tasks, profile history disagreement, and syntax parsing of touched PowerShell scripts without executing tweaks.
- Read-only Windows checks verified the TCP dump format, Teredo Default versus effective Disabled, ClassGUID availability, and the reserved-storage cmdlet signature. System-changing apply/revert was not exercised on this PC; no live visual browser test or release installer build was performed.

## Limits

- MSI/NIC/pagefile/Auto-Tuning changes made before these backups existed cannot be safely reconstructed; revert reports missing rollback data.
- NVMe targeting is limited to Microsoft's stornvme driver; vendor-specific NVMe drivers are not inferred from the broad storage class.
- Maximum-refresh detection covers the primary display and does not offer rollback.
- NIC registry changes and MSI configuration require a restart to become effective. Detection reports configured values.
- A restored previous value can itself satisfy the active detector. The UI reports that actual state instead of forcing Disabled after revert.
- Elevated/hardware-specific end-to-end verification remains necessary. Unsupported or failed touched queries export skip; older unrelated boolean detectors retain their existing semantics.

## Documentation consulted through Context7

- [Microsoft netsh interface reference](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/netsh-interface)
- [Microsoft MSI registry configuration](https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/enabling-message-signaled-interrupts-in-the-registry)
- [Microsoft Teredo configuration reference](https://learn.microsoft.com/en-us/powershell/module/networktransition/get-netteredoconfiguration)
- [Microsoft reserved-storage state reference](https://learn.microsoft.com/en-us/powershell/module/dism/get-windowsreservedstoragestate)

## Files changed

The existing profile-path changes in profiles.rs were preserved. The pre-existing untracked TWEAKS.md was not edited.

- `src-tauri/src/commands.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/modules/display/monitor.rs`
- `src-tauri/src/modules/gpu/msi.rs`
- `src-tauri/src/modules/input/usb.rs`
- `src-tauri/src/modules/interface/context_menu.rs`
- `src-tauri/src/modules/network/adapter.rs`
- `src-tauri/src/modules/network/msi.rs`
- `src-tauri/src/modules/network/tcp.rs`
- `src-tauri/src/modules/profiles.rs`
- `src-tauri/src/modules/registry/backup.rs`
- `src-tauri/src/modules/registry/operations.rs`
- `src-tauri/src/modules/storage/msi.rs`
- `src-tauri/src/modules/system/maintenance.rs`
- `src-tauri/src/modules/system/memory.rs`
- `src-tauri/src/modules/tweaks/helpers/checks.rs`
- `src-tauri/src/modules/tweaks/helpers/mod.rs`
- `src-tauri/src/modules/tweaks/helpers/msi.rs`
- `src-tauri/src/modules/tweaks/helpers/nic.rs`
- `src-tauri/src/modules/tweaks/mod.rs`
- `src-tauri/src/modules/types.rs`
- `src/components/ProfileManager.svelte`
- `src/components/TweakCard.svelte`
- `src/components/TweakList.svelte`
- `src/components/ai/ChatPanel.svelte`
- `src/components/cpu/CpuDashboard.svelte`
- `src/components/debloat/AppsSection.svelte`
- `src/components/debloat/EdgeSection.svelte`
- `src/components/debloat/FeaturesSection.svelte`
- `src/components/debloat/ServicesSection.svelte`
- `src/components/display/GpuSection.svelte`
- `src/components/display/MonitorSection.svelte`
- `src/components/display/SystemSection.svelte`
- `src/components/gaming/GamingDashboard.svelte`
- `src/components/gpu/GpuDashboard.svelte`
- `src/components/input/KeyboardDashboard.svelte`
- `src/components/input/MouseDashboard.svelte`
- `src/components/input/UsbDashboard.svelte`
- `src/components/network/AdapterSection.svelte`
- `src/components/network/DnsSection.svelte`
- `src/components/network/MsiSection.svelte`
- `src/components/network/SecuritySection.svelte`
- `src/components/network/TcpSection.svelte`
- `src/components/privacy/PrivacyDashboard.svelte`
- `src/components/security/AuthSection.svelte`
- `src/components/security/DefenderSection.svelte`
- `src/components/security/ErrorReportingSection.svelte`
- `src/components/security/FirewallSection.svelte`
- `src/components/security/HardeningSection.svelte`
- `src/components/security/SmartScreenSection.svelte`
- `src/components/storage/StorageDashboard.svelte`
- `src/components/system/SystemDashboard.svelte`
- `src/components/ui/UIDashboard.svelte`
- `src/lib/types.ts`
- `src/routes/+page.svelte`
- `src-tauri/src/modules/tweaks/helpers/device_backup.rs`
- `TWEAK_ENGINE_FIXES.md` (this report)

