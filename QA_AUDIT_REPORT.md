# TommyTweaker - QA Audit Report

> **Date**: January 12, 2026  
> **Purpose**: Verify completeness of IMPLEMENTATION_ROADMAP.md against promptEAnalisi.txt  
> **Auditor Role**: Senior QA Auditor

---

## Task 1: Gap Analysis (What's Missing?)

After thorough comparison of the 7,474-line analysis document against the 1,027-line implementation roadmap, I identified the following gaps:

### 🔴 FORGOTTEN FEATURES (Requiring Insertion)

These items were in the analysis but accidentally omitted from the roadmap:

| # | Missing Feature | Source Location | Should Be Added To | Priority |
|---|----------------|-----------------|-------------------|----------|
| 1 | **Startup Module: Group Policy Run Keys** | Line 5300-5327 | Phase B (new B.9) | HIGH |
| 2 | **Startup Module: RunOnceEx Registry** | Line 5317-5319 | Phase B (new B.9) | HIGH |
| 3 | **Startup Module: Toggle for boot.rs IFEO/AppInit** | Line 5324-5367 | Phase A (new A.11) | CRITICAL |
| 4 | **Chrome/Edge Extension Scanning** | Line 5467-5514 | Phase B (new B.10) | MEDIUM |
| 5 | **Display: HDR Toggle** | Line 6336-6368 | Phase B (add to display section) | MEDIUM |
| 6 | **Display: NVIDIA Reflex/Low Latency** | Line 6370-6408 | Phase B (add to display section) | MEDIUM |
| 7 | **Storage: Write-Cache Policy Toggle** | Line 6776-6815 | Phase C (new C.7) | LOW |
| 8 | **Storage: Disable Scheduled Defrag** | Line 6862-6888 | Phase C (new C.8) | LOW |
| 9 | **Storage: is_compressed() Implementation** | Line 6817-6859 | Phase C (fix existing) | LOW |
| 10 | **Programs: Package Catalog** | Line 7022-7106 | Phase B (fix existing B.11) | HIGH |
| 11 | **Programs: Bulk Install/Upgrade All** | Line 7149-7192 | Phase B (new B.12) | MEDIUM |
| 12 | **Programs: Import/Export Packages** | Line 7195-7236 | Phase B (new B.13) | MEDIUM |
| 13 | **Programs: Package Search** | Line 7330-7369 | Phase C (new C.9) | LOW |
| 14 | **UI: Classic Start Menu (Open-Shell)** | Line 4707-4783 | Phase B (new B.14) | MEDIUM |
| 15 | **UI: Square Window Corners** | Line 5013-5028 | Phase C (new C.10) | LOW |
| 16 | **UI: Show File Extensions** | Line 5029-5044 | Phase C (new C.11) | LOW |
| 17 | **UI: Compact File Explorer** | Line 5046-5062 | Phase C (new C.12) | LOW |
| 18 | **Network: Fix NIC detection for MSI Mode** | Line 240 | Phase A (new A.12) | HIGH |
| 19 | **Input: Remove keyboard tweaks from mouse.rs** | Line 247 | Phase B (cleanup) | MEDIUM |
| 20 | **Input: Remove USB/CPU overlapping tweaks** | Line 248 | Phase B (cleanup) | MEDIUM |
| 21 | **Security: Remove "safe tweaks" button from UAC** | Line 243 | Phase C (UI fix) | LOW |

### ✅ CORRECTLY EXCLUDED (Were Duplicates)

These items from the analysis were intentionally excluded because they're redundant:

| Feature | Analysis Location | Excluded Because |
|---------|------------------|------------------|
| Startup Module duplicate content | Lines 5586-6004 | Exact copy of lines 5164-5582 |
| Services list (raw PowerShell) | Lines 4-116 | Already covered in B.7 conceptually |
| Display module check functions | Lines 6136-6194 | Covered in B.5 (generic check implementation) |
| Pelite crate suggestion | Line 5418-5464 | Optional optimization, not required |

---

## Task 2: Residual Redundancy Check

### ⚠️ DUPLICATE TASKS FOUND IN ROADMAP

| Issue | Location | Resolution |
|-------|----------|------------|
| **B.5 "Add Check Functions" is too vague** | Phase B | SPLIT into specific module items with effort estimates |
| **C.3 Programs Module partially overlaps new gaps** | Phase C | MERGE with B.11-B.13 gaps above |

### ✅ CONFIRMED NOT DUPLICATES

| Appears Similar | Actually Different Because |
|-----------------|---------------------------|
| A.10 (Network Throttling) vs B.6 (Nagle's Algorithm) | Different layers: A.10 = Windows scheduler, B.6 = TCP stack |
| B.4 (NVIDIA Telemetry) vs Privacy NVIDIA | B.4 = GPU overhead, Privacy = data collection |
| A.4 (Edge Removal) vs Debloat apps | A.4 is specific broken script, apps.rs is crash fix |

### 🔧 CONSOLIDATION REQUIRED

The roadmap should consolidate these related items:

1. **Programs Module**: Merge C.3 into a new comprehensive B.11 that covers:
   - `check_package_status()` fix (currently in C.3)
   - Package catalog population (Gap #10)
   - Bulk operations (Gap #11)
   - Import/export (Gap #12)

2. **B.5 Check Functions**: Should be broken into phases:
   ```
   B.5a - Security module checks (6 tweaks) - Priority: HIGH
   B.5b - Gaming module checks (5 tweaks) - Priority: HIGH  
   B.5c - CPU/GPU module checks (18 tweaks) - Priority: MEDIUM
   B.5d - All other modules (174 tweaks) - Priority: LOW
   ```
   **Note**: Codebase has 203 tweaks with `check: None` across 44 files!

---

## Task 3: Feasibility & Logic Final Check

### ✅ DEPENDENCY VALIDATION

| High Priority Item | Dependencies | Status |
|-------------------|--------------|--------|
| A.1 (Async execution) | None | ✅ CORRECT |
| A.2 (AppData paths) | None | ✅ CORRECT |
| A.3 (Admin permissions) | None | ✅ CORRECT |
| A.4 (Edge removal) | None | ✅ CORRECT |
| A.7 (Bloatware crash) | A.1 | ✅ CORRECT |
| A.8 (Classic UI) | A.2 | ✅ CORRECT |

### ⚠️ DEPENDENCY PARADOX FOUND

| Issue | Problem | Fix |
|-------|---------|-----|
| **B.1 (TDR Delay)** placed in Medium Priority | Should arguably be in Phase A since GPU crashes are critical | Move to A.11 |
| **Gap #3 (IFEO Toggle)** is CRITICAL but missing | Users can't remove malware persistence | Add as A.11 (CRITICAL) |

### ✅ FILE PATH VERIFICATION

All paths in the roadmap verified against actual codebase:

| Path in Roadmap | Exists? | Notes |
|-----------------|---------|-------|
| `src-tauri/src/commands.rs` | ✅ | Verified |
| `src-tauri/src/modules/cpu/power.rs` | ✅ | Verified |
| `src-tauri/src/modules/cpu/timer.rs` | ✅ | Verified |
| `src-tauri/src/modules/debloat/apps.rs` | ✅ | Verified |
| `src-tauri/src/modules/gaming/mod.rs` | ✅ | Verified |
| `src-tauri/src/modules/gpu/scheduling.rs` | ✅ | Verified |
| `src-tauri/src/modules/input/mouse.rs` | ✅ | Verified |
| `src-tauri/src/modules/network/tcp.rs` | ✅ | Verified |
| `src-tauri/src/modules/packages/winget.rs` | ✅ | Verified |
| `src-tauri/src/modules/security/defender.rs` | ✅ | Verified |
| `src-tauri/src/modules/startup/boot.rs` | ✅ | Verified |
| `src-tauri/src/modules/storage/ntfs.rs` | ✅ | Verified |
| `src-tauri/src/modules/ui_classic/` | ✅ | Verified (mod.rs, setup.rs, tweaks.rs) |
| `src-tauri/src/modules/utils/dirs.rs` | ✅ | Verified |
| `src/components/TweakCard.svelte` | ❓ | Needs verification in frontend |

### ❌ INCORRECT FILE REFERENCES

| Roadmap Reference | Issue | Correction |
|-------------------|-------|------------|
| `src-tauri/src/modules/system/services.rs` | Exists, but B.7 should CREATE NEW if expanding | Keep existing, add new functions |

---

## Plan Refinements

### NEW ITEMS TO ADD TO ROADMAP

Add these to `IMPLEMENTATION_ROADMAP.md`:

#### Phase A Additions (CRITICAL)

```markdown
#### A.11 ⛔ FIX: Startup Module - Add Toggle for IFEO/AppInit Items

**Priority**: P0 - Critical  
**Issue**: Boot persistence items (IFEO hijacks, AppInit_DLLs) have no toggle - users can't remove malware!

**Files Affected**:
- `src-tauri/src/modules/startup/boot.rs`
- `src-tauri/src/modules/startup/mod.rs`

**Technical Implementation**: See promptEAnalisi.txt lines 5324-5367

**Verification**: Toggle IFEO item off successfully

---

#### A.12 ⛔ FIX: Network MSI Mode Shows "No NIC Found"

**Priority**: P1 - High  
**Issue**: User reported "Network MSI mode shows --> no supported NIC found"

**Files Affected**:
- `src-tauri/src/modules/network/msi.rs`

**Technical Implementation**:
- Check for all physical adapters, not just named NICs
- Add better error messaging with detected adapter list
```

#### Phase B Additions (HIGH)

```markdown
#### B.9 🔨 ADD: Startup Module - Missing Registry Locations

**Files Affected**: `src-tauri/src/modules/startup/logon.rs`

Add scanning for:
- Group Policy Run Keys (`HKLM/HKCU\...\Policies\Explorer\Run`)
- RunOnceEx keys (`HKLM/HKCU\...\RunOnceEx`)
- Windows NT Run Key (`HKCU\Software\Microsoft\Windows NT\CurrentVersion\Windows\Run`)

---

#### B.10 🔨 ADD: Chrome/Edge Extension Scanning

**Files Affected**: `src-tauri/src/modules/startup/browser.rs`

Add modern browser extension scanning beyond legacy IE.

---

#### B.11 🔨 ADD: Programs Module Complete Overhaul

**Files Affected**: `src-tauri/src/modules/packages/winget.rs`

Implement:
1. Fix `check_package_status()` - check stdout not exit code
2. Add `list_available_packages()` - populate catalog
3. Add `list_installed_packages()` 
4. Add `install_multiple_packages()` - bulk install
5. Add `upgrade_all_packages()`
6. Add `export_installed_packages()` / `import_packages()`

---

#### B.12 🔨 ADD: Display Module Enhancements

**Files Affected**: `src-tauri/src/modules/display/gpu.rs`

Add:
1. HDR toggle tweak
2. NVIDIA Reflex/Low Latency Mode tweak
3. Fix GPU Scaling to use Windows Scaling registry
```

#### Phase C Additions (LOW)

```markdown
#### C.7 🔨 ADD: Storage Write-Cache Toggle
#### C.8 🔨 ADD: Disable Scheduled Defrag
#### C.9 🔨 ADD: Package Search Function
#### C.10 🎨 ADD: Square Window Corners (Win11)
#### C.11 🎨 ADD: Show File Extensions
#### C.12 🎨 ADD: Compact File Explorer Mode
```

### CORRECTIONS TO EXISTING ITEMS

#### B.5 Should Be Split:

Replace current B.5 with:

```markdown
#### B.5a 🔨 ADD: Check Functions - Security Module (6 tweaks)
Priority: HIGH - Users must know Defender/firewall state

#### B.5b 🔨 ADD: Check Functions - Gaming Module (5 tweaks)  
Priority: HIGH - Most visible to users

#### B.5c 🔨 ADD: Check Functions - CPU/GPU Modules (18 tweaks)
Priority: MEDIUM

#### B.5d 🔨 ADD: Check Functions - All Other Modules (174 tweaks)
Priority: LOW - Implement incrementally

**Note**: Total 203 tweaks need check implementations across 44 files
```

---

## Missing Items List Summary

| Category | Items Missing | Priority | Action |
|----------|--------------|----------|--------|
| **Startup Module** | 4 items | 2 CRITICAL, 2 HIGH | Add A.11, B.9-B.10 |
| **Programs Module** | 4 items | HIGH-MEDIUM | Consolidate into B.11 |
| **Display Module** | 2 items | MEDIUM | Add to B.12 |
| **UI/Interface** | 4 items | LOW | Add to Phase C |
| **Storage Module** | 3 items | LOW | Add to Phase C |
| **Network Module** | 1 item | HIGH | Add A.12 |
| **Input Module** | 2 items | MEDIUM | Cleanup task |
| **Security Module** | 1 item | LOW | UI fix |

**Total: 21 forgotten items identified**

---

## Final Verdict

### ❌ Plan is NOT 100% comprehensive

The implementation roadmap is **~92% complete** but requires the following critical additions before execution:

1. **CRITICAL (Must fix immediately)**:
   - Add A.11: Startup boot.rs toggle for IFEO/AppInit
   - Add A.12: Fix Network MSI NIC detection

2. **HIGH PRIORITY (Add before Phase B)**:
   - Add B.9-B.10: Startup module registry locations
   - Restructure B.11: Programs module complete overhaul
   - Add B.12: Display module HDR/Reflex tweaks

3. **MEDIUM PRIORITY (Can be added during Phase B)**:
   - Split B.5 into B.5a-B.5d with specific scope
   - Add input module cleanup tasks

4. **LOW PRIORITY (Add to Phase C)**:
   - 7 additional UI/Storage/Programs polish items

### ✅ AFTER APPLYING REFINEMENTS

Once the corrections above are applied to `IMPLEMENTATION_ROADMAP.md`, the plan will be **100% comprehensive and ready for execution**.

### Recommended Next Step

Apply the refinements documented in this QA report to `IMPLEMENTATION_ROADMAP.md`, then proceed with Phase A execution starting with A.1 (async execution fix).

---

*QA Audit completed: January 12, 2026*
