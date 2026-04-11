# TommyTweaker — Recommended Actions

---

## 1. Tweaks to Remove (true duplicates / conflicts)

### `net_tcp_ctcp` → REMOVE
- **Why:** Both `net_tcp_ctcp` and `net_tcp_bbr` call
  `netsh int tcp set supplemental template=internet congestionprovider=...`
  on the same global setting. The last one applied wins, making the other
  meaningless.
- **Keep:** `net_tcp_bbr` — BBR (Google) is superior to CTCP on Windows 11
  for both gaming and streaming. CTCP is a legacy Microsoft algorithm.

### `mem_disable_paging_executive` → REMOVE
- **Why:** Exact duplicate of `cpu_disable_paging_executive`. Both write
  `HKLM\...\Memory Management → DisablePagingExecutive = 1`.
- **Keep:** `cpu_disable_paging_executive` (correct module: `cpu/memory.rs`).

### `net_system_responsiveness` → REMOVE
- **Why:** Both `net_system_responsiveness` and `sys_responsiveness` write
  `HKLM\Control\PriorityControl → SystemResponsiveness`.
- **Keep:** `sys_responsiveness` — it is the canonical, category-correct tweak.

---

## 2. NOT a Duplicate (AI was wrong)

### `cpu_disable_svchost_split` vs `mem_svchost_split` — KEEP BOTH (or just CPU)
The AI said these write the same registry value. **This is incorrect:**
- `cpu_disable_svchost_split` → uses `TweakOperationSvcHostSplitAll`,
  writes `SvcHostSplitDisable = 1` into **every individual service key**
  under `HKLM\SYSTEM\...\Services\*`
- `mem_svchost_split` → writes a single global threshold:
  `HKLM\...\Memory Management → SvcHostSplitThresholdInKB = 4194304`

They are different mechanisms. Having both active is slightly redundant
but they do **not** overwrite each other. If you want to clean up,
keep `cpu_disable_svchost_split` as it is more thorough.

---

## 3. `priv_all_in_one` — AI was WRONG about redundancy

The AI claimed `priv_all_in_one` makes these tweaks redundant:
`priv_disable_telemetry`, `priv_disable_location`, `priv_disable_advertising_id`,
`priv_disable_activity_history`, `priv_disable_app_tracking`,
`priv_disable_suggestions`, `priv_disable_tailored`,
`priv_disable_sync_notifs`, `priv_disable_background_apps`.

**Reality:** `priv_all_in_one` only covers these specific keys:
| Key | What it does |
|-----|-------------|
| `HKCU\...\Siuf\Rules → NumberOfSIUFInPeriod = 0` | Disable feedback requests |
| `HKLM\...\HandwritingErrorReports → PreventHandwritingErrorReports = 1` | Disable handwriting reports |
| `HKLM\...\WindowsAI → DisableAIDataAnalysis = 1` | Disable Recall / AI analysis |
| `HKCU\...\WindowsCopilot → TurnOffWindowsCopilot = 1` | Disable Copilot |
| `HKLM\...\System → EnableSmartScreen = 0` | Disable SmartScreen |
| `HKCU\...\Explorer → DisableSearchBoxSuggestions = 1` | Disable search suggestions |
| `ContentDeliveryManager → SubscribedContent-* = 0` (8 keys) | Disable Windows suggested content |

**None of these overlap with `priv_disable_telemetry` (AllowTelemetry),
`priv_disable_location` (sensor/location keys), `priv_disable_advertising_id`
(AdvertisingInfo), etc.** Those individual tweaks write to completely
different registry paths and are NOT made redundant.

**Conclusion: keep all individual `priv_disable_*` tweaks active alongside
`priv_all_in_one`. They are complementary, not conflicting.**

---

## 4. Hardware-Specific: Disable HPET

Your system profile shows `hpet_enabled: true`.
On AMD Ryzen (especially 3D V-Cache), HPET causes slightly higher DPC
latency and micro-stuttering. Modern AMD systems use TSC as the
high-resolution clock, making HPET unnecessary.

**How to disable:**
1. Enter BIOS → Advanced → AMD CBS (or similar)
2. Find **HPET** → set to **Disabled**
3. Confirm `hpet_enabled` flips to `false` in your profile

> Note: also verify that your `hpet_enabled` BIOS flag matches the actual
> registry PnP `ConfigFlags` your profiler checks — they should agree after
> the BIOS change.

---

## 5. Summary of Actions

| Action | Tweak | Reason |
|--------|-------|--------|
| ❌ Remove | `net_tcp_ctcp` | Conflicts with `net_tcp_bbr`, same setting |
| ❌ Remove | `mem_disable_paging_executive` | Exact duplicate of `cpu_` version |
| ❌ Remove | `net_system_responsiveness` | Exact duplicate of `sys_responsiveness` |
| ✅ Keep both | `cpu_disable_svchost_split` + `mem_svchost_split` | Different mechanisms |
| ✅ Keep all | `priv_all_in_one` + all `priv_disable_*` | No overlap, different registry keys |
| 🔧 BIOS | Disable HPET | Lower DPC latency on Ryzen 5700X3D |