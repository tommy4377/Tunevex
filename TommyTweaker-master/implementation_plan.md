# Implementation Plan - System & Hardware Tweaks Integration

This plan outlines the integration of missing high-value tweaks from the "Platinum+ Optimizer" script into **TommyTweaker**. We will focus on System Priorities, Memory, FileSystem, and Hardware-specific optimizations (Intel/AMD CPU, Nvidia/AMD/Intel GPU).

## User Review Required
> [!IMPORTANT]
> Some tweaks like `Win32PrioritySeparation` and `DisablePagingExecutive` require a restart to take full effect.
> We will filter out "placebo" tweaks or those that are harmful (e.g., clearing all temp files blindly) and implement only the technically sound ones.

## Proposed Changes

### 1. New Module: `system` (Enhancements)
We will split `system` into more granular files.

#### [NEW] [priority.rs](file:///home/tommy/Scrivania/TommyTweaker/TommyTweaker/src-tauri/src/modules/system/priority.rs)
*   **Win32PrioritySeparation**: Control process scheduling (Foreground vs Background).
*   **IRQ Priority**: Set RTC (IRQ8) and Coprocessor (IRQ13) to High priority.
*   **Game Mode Priority**: Registry tweaks for `SystemProfile\Tasks\Games` (GPU Priority, Scheduling Category).

#### [NEW] [memory.rs](file:///home/tommy/Scrivania/TommyTweaker/TommyTweaker/src-tauri/src/modules/system/memory.rs)
*   **LargeSystemCache**: Optimize file cache strategy.
*   **DisablePagingExecutive**: Keep kernel components in RAM (if >4GB RAM).
*   **FeatureSettingsOverride**: Mitigations for Spectre/Meltdown (optional performance boost vs security).

### 2. Module: `filesystem`
Populate the currently empty module.

#### [MODIFY] [mod.rs](file:///home/tommy/Scrivania/TommyTweaker/TommyTweaker/src-tauri/src/modules/filesystem/mod.rs)
*   **NTFS Optimizations**:
    *   `Disable8dot3`: Disable legacy DOS filenames (performance boost).
    *   `DisableLastAccess`: Disable updating file access timestamps.
    *   `NtfsMemoryUsage`: Increase NTFS metadata cache.

### 3. Hardware Specific Tweaks
We will check existing modules and add missing specifics.

### 3. Smart Hardware Specific Tweaks (Auto-Detection)
We will NOT create separate UI sections for "Intel", "AMD", etc. Instead, we will implement smart detection logic so tweaks only appear for the relevant hardware.

#### [MODIFY] [cpu/mod.rs](file:///home/tommy/Scrivania/TommyTweaker/TommyTweaker/src-tauri/src/modules/cpu/mod.rs) & [gpu/mod.rs](file:///home/tommy/Scrivania/TommyTweaker/TommyTweaker/src-tauri/src/modules/gpu/mod.rs)
*   **Strategy**: Use Rust to detect CPU/GPU vendor (via `sysinfo` or similar, or checking registry keys like `HKLM\HARDWARE\DESCRIPTION\System\CentralProcessor\0`).
*   **Implementation**:
    *   **Intel Tweaks**: (e.g., `IntelPPM`) -> Wrapped in `if is_intel()`.
    *   **Nvidia Tweaks**: (e.g., `DisableDynamicPstate`) -> Wrapped in `if is_nvidia()`.
    *   **AMD Tweaks**: Similar logic.
*   **Result**: The user sees a clean list where irrelevant tweaks are completely hidden.

### 4. UI Integration
*   The new tweaks will be exposed via the `get_system_tweaks`, `get_filesystem_tweaks`, etc., functions.
*   The frontend (UI) automatically renders these if they are returned by the backend. We just need to ensure categories are correct.


### Manual Verification
*   **Dry Run**: Use the "Check" functionality in the app to see if it correctly identifies the current state of these registry keys.
*   **Apply**: Apply a tweak and verify the registry key changes using `regedit`.
*   **Revert**: Verify the tweak can be reverted to original state.
