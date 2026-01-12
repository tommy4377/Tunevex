```markdown
# 🎯 PROJECT PROMPT: Ultimate Gaming Optimizer

## 📋 Project Overview
Create a **Windows Gaming Optimizer** desktop application using **Rust + Tauri + Svelte** that consolidates ALL PC gaming tweaks into one unified tool. This replaces: Chris Titus WinUtil, TCP Optimizer, DNS Jumper, MSI Mode Utility, Optimizer, AtlasOS tweaks, and Hone tweaks.

**IMPORTANT**: This is a SEPARATE project from TommyMemoryCleaner. Do NOT integrate memory cleaning features.

---

## 🏗️ Tech Stack
- **Backend**: Rust (with Tauri)
- **Frontend**: Svelte (no UI framework yet - focus on functionality first)
- **Architecture**: Clean, modular, maintainable code
- **No hardcoded values**: Everything must be data-driven and configurable
- **i18n ready**: Use proper internationalization structure (English only for now, but prepared for translations)

---

## 🎨 Initial UI Requirements
- **UGLY IS FINE**: Focus 100% on functionality first, not aesthetics
- No CSS frameworks needed yet (plain HTML/CSS is acceptable)
- Simple checkbox-based interface per category
- Category-based navigation (tabs or accordion)
- Each tweak must have:
  - Checkbox to enable/disable
  - Clear description of what it does
  - Warning level indicator (Safe / Careful / Dangerous)
  - Apply button per category or global "Apply All"
  - Undo/Restore capability

---

## 📁 Project Structure

```
gaming-optimizer/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── modules/
│   │   │   ├── mod.rs
│   │   │   ├── network/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── dns.rs
│   │   │   │   ├── tcp_optimizer.rs
│   │   │   │   ├── adapter.rs
│   │   │   │   └── latency.rs
│   │   │   ├── cpu/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── scheduling.rs
│   │   │   │   └── power.rs
│   │   │   ├── gpu/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── nvidia.rs
│   │   │   │   ├── amd.rs
│   │   │   │   └── msi_mode.rs
│   │   │   ├── input/
│   │   │   │   ├── mod.rs
│   │   │   │   └── mouse.rs
│   │   │   ├── display/
│   │   │   │   ├── mod.rs
│   │   │   │   └── refresh_rate.rs
│   │   │   ├── debloat/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── telemetry.rs
│   │   │   │   ├── apps.rs
│   │   │   │   └── services.rs
│   │   │   ├── startup/
│   │   │   │   ├── mod.rs
│   │   │   │   └── autoruns.rs
│   │   │   ├── filesystem/
│   │   │   │   ├── mod.rs
│   │   │   │   └── ntfs.rs
│   │   │   ├── security/
│   │   │   │   ├── mod.rs
│   │   │   │   └── privacy.rs
│   │   │   ├── interface/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── visual_effects.rs
│   │   │   │   └── context_menu.rs
│   │   │   ├── system/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── boot.rs
│   │   │   │   └── updates.rs
│   │   │   └── gaming/
│   │   │       ├── mod.rs
│   │   │       └── hone_tweaks.rs
│   │   ├── registry/
│   │   │   ├── mod.rs
│   │   │   ├── backup.rs
│   │   │   └── operations.rs
│   │   ├── presets/
│   │   │   ├── mod.rs
│   │   │   └── profiles.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       ├── privileges.rs
│   │       └── validation.rs
│   └── Cargo.toml
├── ui/
│   ├── src/
│   │   ├── App.svelte
│   │   ├── lib/
│   │   │   ├── api.ts
│   │   │   └── types.ts
│   │   ├── components/
│   │   │   ├── CategoryPanel.svelte
│   │   │   ├── TweakItem.svelte
│   │   │   └── PresetSelector.svelte
│   │   └── i18n/
│   │       ├── index.ts
│   │       └── locales/
│   │           └── en.json
│   └── package.json
├── tweaks_ready/          # <- Reference implementations go here
│   ├── tcp_optimizer.rs
│   ├── dns_benchmark.rs
│   ├── msi_mode.rs
│   └── README.md
└── README.md
```

---

## 📦 "tweaks_ready" Folder
This folder contains **pre-implemented, production-ready tweaks** that the AI can copy EXACTLY as-is without modifications. These serve as:
- Reference implementations
- Copy-paste ready code
- Best practices examples
- Time savers for common operations

**AI Instructions**: When a similar tweak is needed, check this folder first and adapt existing code rather than reinventing.

---

## 🎯 Core Requirements

### 1. Data-Driven Architecture
**NO HARDCODING**. All tweaks must be defined in a structured data format:

```rust
// Example structure (adapt as needed)
pub struct Tweak {
    pub id: String,
    pub category: TweakCategory,
    pub name: String,
    pub description: String,
    pub warning_level: WarningLevel,
    pub operation: TweakOperation,
    pub requires_restart: bool,
    pub can_undo: bool,
}

pub enum WarningLevel {
    Safe,       // Green - No risks
    Careful,    // Yellow - Minor compatibility risks
    Dangerous,  // Red - Security/stability risks
}

pub enum TweakOperation {
    RegistrySet { path: String, key: String, value: RegistryValue, original: Option<RegistryValue> },
    RegistryDelete { path: String, key: String },
    ServiceDisable { name: String },
    ScheduledTaskDisable { path: String, name: String },
    FileOperation { operation: FileOp },
    Command { cmd: String, args: Vec<String> },
    // etc.
}
```

### 2. Internationalization (i18n)
- Use `rust-i18n` or similar for backend
- Use proper i18n library for Svelte frontend
- All user-facing strings MUST be translatable
- English only for MVP, but architecture must support adding languages later
- Translation keys format: `category.tweak_id.description`

### 3. Registry Operations Safety
- **ALWAYS** backup original values before modification
- Store backups in a database or JSON file
- Implement undo functionality per tweak
- Check if key exists before modifying
- Handle HKU (HKEY_USERS) properly by mounting user hives
- Use proper error handling and logging

### 4. Privilege Escalation
- Detect if running as Administrator
- Request elevation if needed
- Clear messaging to user about why elevation is required
- Handle UAC properly

### 5. State Management
- Track which tweaks are currently applied
- Persist state across app restarts
- Show current system state vs. desired state
- Detect conflicts between tweaks

### 6. Presets/Profiles
Implement 4 built-in profiles:
- **Normal**: Basic optimizations, high compatibility
- **Balanced**: Moderate tweaks, good middle ground
- **Gaming**: Aggressive performance, some compatibility trade-offs
- **Extreme**: Maximum performance, security/compatibility risks

Users should be able to:
- Apply preset profiles
- Customize and save their own profiles
- Export/import profiles as JSON

---

## 📊 COMPLETE TWEAK LIST BY CATEGORY

### CATEGORY 1: NETWORK OPTIMIZATION

#### DNS Optimization
- [ ] **DNS Benchmark Tool**
  - Description: "Tests response time of multiple DNS providers and selects the fastest"
  - Operation: Ping test to 13 DNS providers (Google, Cloudflare, Quad9, OpenDNS, etc.)
  - Warning: Safe
  - Apply: Set DNS per network adapter or globally

- [ ] **Set DNS: Google (8.8.8.8 / 8.8.4.4)**
  - Description: "Use Google Public DNS servers"
  - Warning: Safe

- [ ] **Set DNS: Cloudflare (1.1.1.1 / 1.0.0.1)**
  - Description: "Use Cloudflare's fast and privacy-focused DNS"
  - Warning: Safe

- [ ] **Set DNS: Cloudflare Malware Blocking (1.1.1.2 / 1.0.0.2)**
  - Description: "Cloudflare DNS with malware protection"
  - Warning: Safe

- [ ] **Set DNS: Cloudflare Malware + Adult Content Blocking (1.1.1.3 / 1.0.0.3)**
  - Description: "Cloudflare DNS with malware and adult content filtering"
  - Warning: Safe

- [ ] **Set DNS: Quad9 (9.9.9.9 / 149.112.112.112)**
  - Description: "Quad9 DNS with threat intelligence"
  - Warning: Safe

- [ ] **Set DNS: OpenDNS (208.67.222.222 / 208.67.220.220)**
  - Description: "Cisco's OpenDNS service"
  - Warning: Safe

- [ ] **Set DNS: AdGuard DNS (94.140.14.14 / 94.140.15.15)**
  - Description: "Blocks ads and trackers at DNS level"
  - Warning: Safe

#### TCP/IP Stack Optimization
- [ ] **Optimize TCP Acknowledgment Frequency (TcpAckFrequency = 1)**
  - Description: "Reduces ACK delay for faster packet processing. Improves responsiveness in online games."
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{GUID}`
  - Warning: Safe

- [ ] **Disable Nagle's Algorithm (TCPNoDelay = 1)**
  - Description: "Disables packet buffering. Reduces latency for small packets (important for gaming)."
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{GUID}`
  - Warning: Safe

- [ ] **Set TCP Window Size (TCPWindowSize = 65535)**
  - Description: "Sets maximum TCP receive window size for better throughput"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Enable TCP Window Scaling (Tcp1323Opts = 3)**
  - Description: "Enables window scaling and timestamps for high-speed connections"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Increase Max User Port (MaxUserPort = 65534)**
  - Description: "Increases available ephemeral port range for more concurrent connections"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Reduce TCP Timed Wait Delay (TcpTimedWaitDelay = 30)**
  - Description: "Reduces TIME_WAIT state duration for faster port reuse"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Set Default TTL (DefaultTTL = 64)**
  - Description: "Sets default Time-To-Live for TCP/IP packets"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Enable Path MTU Discovery (EnablePMTUDiscovery = 1)**
  - Description: "Automatically discovers optimal packet size for network path"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Disable PMTU Black Hole Detection (EnablePMTUBHDetect = 0)**
  - Description: "Disables black hole detection to reduce overhead"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Enable Selective Acknowledgment (SackOpts = 1)**
  - Description: "Improves performance when packet loss occurs"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

- [ ] **Disable WSD (Web Services Discovery)**
  - Description: "Disables network discovery protocol that can add latency"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
  - Warning: Safe

#### Network Adapter Advanced Settings
- [ ] **Enable RSS (Receive Side Scaling)**
  - Description: "Distributes network processing across multiple CPU cores"
  - Operation: Set-NetAdapterRss -Enabled $true
  - Warning: Safe

- [ ] **Configure RSS Profile (ClosestProcessor)**
  - Description: "Sets RSS to use closest processor for lower latency"
  - Options: Closest / ClosestStatic / NUMA / NUMAStatic / Conservative
  - Warning: Safe

- [ ] **Set RSS Base CPU**
  - Description: "Specifies starting CPU core for RSS distribution"
  - Warning: Careful (depends on CPU topology)

- [ ] **Configure Receive/Transmit Buffers**
  - Description: "Adjusts network adapter buffer sizes. Higher = more throughput, more latency"
  - Warning: Careful (adapter-specific)

- [ ] **Disable IPv4 Checksum Offload**
  - Description: "Forces CPU to handle checksums. Can reduce micro-stuttering on some systems"
  - Warning: Careful (increases CPU usage)

- [ ] **Disable IPv6 Checksum Offload**
  - Description: "Same as IPv4 but for IPv6 traffic"
  - Warning: Careful

- [ ] **Disable Large Send Offload (LSO)**
  - Description: "Disables hardware packet segmentation. Can fix stuttering in some games"
  - Warning: Careful (reduces throughput)

- [ ] **Configure Interrupt Moderation**
  - Description: "Controls how often network card generates interrupts. Lower = more responsive, higher CPU usage"
  - Options: Disabled / 200 (Minimal) / 400 (Low) / 950 (Medium) / 2000 (High) / Adaptive
  - Warning: Safe

- [ ] **Disable Flow Control**
  - Description: "Prevents network adapter from pausing transmission. Reduces latency jitter"
  - Warning: Safe

- [ ] **Disable Power Management on Network Adapter**
  - Description: "Prevents adapter from entering low-power states"
  - Warning: Safe

#### Global Network Settings
- [ ] **Enable Receive Side Scaling Globally**
  - Description: "Enables RSS at OS level"
  - Command: Set-NetOffloadGlobalSetting -ReceiveSideScaling Enabled
  - Warning: Safe

- [ ] **Disable Receive Segment Coalescing**
  - Description: "Disables packet coalescing for lower latency"
  - Warning: Careful (slightly higher CPU usage)

- [ ] **Disable Chimney Offload**
  - Description: "Legacy offload feature that can cause issues"
  - Warning: Safe

- [ ] **Disable Task Offload**
  - Description: "Forces CPU to handle network processing. Can reduce stuttering"
  - Warning: Careful (increases CPU usage)

- [ ] **Disable Packet Coalescing Filter**
  - Description: "Prevents grouping of packets for reduced latency"
  - Warning: Safe

#### AFD (Ancillary Function Driver) Settings
- [ ] **Set Default Receive Window (Custom)**
  - Description: "Configures Windows Sockets receive buffer size"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\AFD\Parameters`
  - Warning: Careful

- [ ] **Set Default Send Window (Custom)**
  - Description: "Configures Windows Sockets send buffer size"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\AFD\Parameters`
  - Warning: Careful

- [ ] **Optimize Buffer Sizes (Small/Medium/Large/Huge)**
  - Description: "Fine-tunes socket buffer allocation for different packet sizes"
  - Warning: Careful (advanced users only)

---

### CATEGORY 2: CPU & PERFORMANCE

#### CPU Scheduling & Priority
- [ ] **Optimize CPU Scheduling for Programs (Win32PrioritySeparation = 0x26)**
  - Description: "Prioritizes foreground applications over background tasks. Essential for gaming"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\PriorityControl`
  - Warning: Safe

- [ ] **Reduce System Reserved CPU (SystemResponsiveness = 0)**
  - Description: "Allocates 100% CPU to applications instead of reserving 20% for system. Improves game performance"
  - Registry: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile`
  - Warning: Safe

- [ ] **Disable CPU Core Parking**
  - Description: "Prevents Windows from turning off CPU cores to save power. Improves responsiveness"
  - Registry: Multiple power plan attributes
  - Warning: Safe

- [ ] **Disable CPU Throttling (ProcessorThrottlingEnabled = 0)**
  - Description: "Prevents CPU from downclocking under load"
  - Warning: Careful (higher temps/power consumption)

- [ ] **Enable Aggressive Turbo Boost (PerfBoostMode = 2)**
  - Description: "Forces CPU to maintain maximum turbo frequencies"
  - Warning: Careful (higher temps/power consumption)

#### Power Plans
- [ ] **Create & Enable Ultimate Performance Power Plan**
  - Description: "Adds Windows' hidden Ultimate Performance plan. Disables all power saving"
  - Command: powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61
  - Warning: Safe

- [ ] **Set Processor Min/Max State to 100%**
  - Description: "Forces CPU to run at maximum frequency always"
  - Warning: Careful (high power consumption)

- [ ] **Disable PCI Express Link State Power Management**
  - Description: "Prevents PCIe devices (GPU, NVMe) from entering low-power states"
  - Warning: Safe

- [ ] **Disable USB Selective Suspend**
  - Description: "Prevents USB devices from powering down. Fixes mouse/keyboard lag issues"
  - Warning: Safe

- [ ] **Set Hard Disk Turn Off to Never**
  - Description: "Prevents disk spin-down during idle"
  - Warning: Safe

- [ ] **Disable Display Turn Off**
  - Description: "Prevents monitor from sleeping"
  - Warning: Safe (personal preference)

#### Timer Resolution
- [ ] **Optimize Windows Timer Resolution (0.5ms)**
  - Description: "Sets system timer to 0.5ms for better frame pacing and input response"
  - Operation: Requires kernel driver or SetTimerResolution API
  - Warning: Careful (slightly higher power consumption)

- [ ] **Disable Dynamic Tick (bcdedit /set disabledynamictick yes)**
  - Description: "Forces constant timer interrupts for consistent performance"
  - Warning: Careful (higher idle power consumption)

- [ ] **Enable Platform Clock (bcdedit /set useplatformclock true)**
  - Description: "Uses HPET (High Precision Event Timer) for more accurate timekeeping"
  - Warning: Careful (test for performance impact)

#### Game Mode
- [ ] **Enable Windows Game Mode**
  - Description: "Windows' built-in gaming optimization mode"
  - Registry: `HKCU\Software\Microsoft\GameBar`
  - Warning: Safe

- [ ] **Disable Game DVR (Background Recording)**
  - Description: "Disables Xbox Game Bar background recording feature. Saves CPU/GPU resources"
  - Registry: Multiple locations
  - Warning: Safe

- [ ] **Set Game DVR FSE Behavior to Native Fullscreen**
  - Description: "Forces games to use true fullscreen for better performance"
  - Registry: `HKLM\SOFTWARE\Microsoft\PolicyManager\default\ApplicationManagement`
  - Warning: Safe

---

### CATEGORY 3: GPU OPTIMIZATION

#### NVIDIA Specific
- [ ] **Set Power Management to Maximum Performance**
  - Description: "Forces GPU to maintain high clocks. Critical for consistent FPS"
  - Operation: NVIDIA Control Panel registry or nvidia-smi
  - Warning: Safe

- [ ] **Enable Low Latency Mode (Ultra)**
  - Description: "Reduces render queue for lower input lag"
  - Warning: Safe

- [ ] **Set Texture Filtering to High Performance**
  - Description: "Reduces texture filtering quality for higher FPS"
  - Warning: Safe (minimal visual impact)

- [ ] **Disable Vertical Sync Globally**
  - Description: "Disables V-Sync in driver. Let games control it"
  - Warning: Safe

- [ ] **Enable Shader Cache**
  - Description: "Caches compiled shaders for faster load times"
  - Warning: Safe

- [ ] **Enable Threaded Optimization**
  - Description: "Allows driver to use multiple CPU threads"
  - Warning: Safe

- [ ] **Disable NVIDIA HDCP (If Not Using Protected Content)**
  - Description: "Disables copy protection checks. Can reduce latency"
  - Warning: Safe (unless watching DRM content)

- [ ] **Disable NVIDIA Telemetry Services**
  - Description: "Stops NVIDIA from collecting usage data. Saves resources"
  - Services: NvTelemetryContainer, NvProfileUpdaterService64
  - Warning: Safe

- [ ] **Disable NVIDIA Downclocking (PowerMizer)**
  - Description: "Prevents GPU from downclocking during 2D/desktop usage"
  - Registry: NVIDIA driver keys
  - Warning: Careful (higher idle power/temps)

- [ ] **Remove NVIDIA PowerMizer**
  - Description: "Forces P-State 0 (maximum performance) always"
  - Registry: PowerMizerEnable = 0, PerfLevelSrc = 0x3333
  - Warning: Careful

#### AMD Specific
- [ ] **Enable Radeon Anti-Lag**
  - Description: "AMD's input lag reduction technology"
  - Warning: Safe

- [ ] **Disable Radeon Enhanced Sync**
  - Description: "Disables AMD's V-Sync alternative. Let games handle sync"
  - Warning: Safe

- [ ] **Set Texture Filtering to Performance**
  - Description: "Optimizes texture filtering for FPS"
  - Warning: Safe

- [ ] **Set Tessellation Mode to AMD Optimized**
  - Description: "Limits tessellation to reasonable levels"
  - Warning: Safe

- [ ] **Enable Shader Cache**
  - Description: "Caches compiled shaders"
  - Warning: Safe

- [ ] **Enable FreeSync (If Monitor Supports)**
  - Description: "Enables adaptive sync for tear-free gaming"
  - Warning: Safe

#### Universal GPU Tweaks
- [ ] **Enable Hardware-Accelerated GPU Scheduling**
  - Description: "Windows 10 2004+ feature. Reduces latency by letting GPU manage VRAM"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\HwSchMode = 2`
  - Warning: Safe (requires compatible GPU)

- [ ] **Enable MSI (Message-Signaled Interrupts) for GPU**
  - Description: "Reduces interrupt latency for GPU. Major performance improvement on some systems"
  - Registry: Device-specific PCI registry keys
  - Warning: Careful (verify hardware support first)

- [ ] **Set GPU MSI Priority to High**
  - Description: "Increases GPU interrupt priority"
  - Warning: Careful

- [ ] **Configure GPU Interrupt Affinity**
  - Description: "Pins GPU interrupts to specific CPU cores"
  - Warning: Dangerous (advanced users only)

---

### CATEGORY 4: MOUSE & INPUT

#### Mouse Acceleration
- [ ] **Disable Mouse Acceleration (Enhanced Pointer Precision)**
  - Description: "Removes Windows cursor acceleration for consistent aiming. Critical for FPS games"
  - Registry: `HKCU\Control Panel\Mouse\MouseSpeed = 0`
  - Registry: `HKCU\Control Panel\Mouse\MouseThreshold1 = 0`
  - Registry: `HKCU\Control Panel\Mouse\MouseThreshold2 = 0`
  - Warning: Safe

#### Mouse Additional Tweaks
- [ ] **Set Mouse Sensitivity to Default (10)**
  - Description: "Resets Windows mouse sensitivity. Let games handle DPI scaling"
  - Registry: `HKCU\Control Panel\Mouse\MouseSensitivity = 10`
  - Warning: Safe

- [ ] **Reduce Mouse Hover Time (10ms)**
  - Description: "Reduces delay before hover tooltips appear"
  - Registry: `HKCU\Control Panel\Mouse\MouseHoverTime = 10`
  - Warning: Safe

- [ ] **Disable Mouse Trails**
  - Description: "Removes legacy mouse trail effect"
  - Registry: `HKCU\Control Panel\Mouse\MouseTrails = 0`
  - Warning: Safe

- [ ] **Disable Snap To Default Button**
  - Description: "Prevents cursor from auto-moving to dialog buttons"
  - Registry: `HKCU\Control Panel\Mouse\SnapToDefaultButton = 0`
  - Warning: Safe

- [ ] **Optimize Raw Mouse Input**
  - Description: "Ensures games receive unfiltered mouse data"
  - Registry: Various DirectInput registry keys
  - Warning: Safe

#### Keyboard
- [ ] **Set Keyboard Repeat Rate to Maximum (31)**
  - Description: "Fastest key repeat speed"
  - Registry: `HKCU\Control Panel\Keyboard\KeyboardSpeed = 31`
  - Warning: Safe

- [ ] **Set Keyboard Delay to Minimum (0)**
  - Description: "Shortest delay before key repeat starts"
  - Registry: `HKCU\Control Panel\Keyboard\KeyboardDelay = 0`
  - Warning: Safe

---

### CATEGORY 5: DISPLAY & MONITOR

#### Refresh Rate
- [ ] **Auto-Detect and Set Maximum Refresh Rate**
  - Description: "Automatically sets monitor to highest supported refresh rate"
  - Operation: Query CCD API and apply max refresh
  - Warning: Safe

- [ ] **Per-Monitor Refresh Rate Configuration**
  - Description: "Allows setting different refresh rates for multiple monitors"
  - Warning: Safe

#### Fullscreen Optimizations
- [ ] **Disable Fullscreen Optimizations Globally**
  - Description: "Forces games to use exclusive fullscreen for better performance. Disables Windows' borderless fullscreen wrapper"
  - Registry: `HKCU\System\GameConfigStore\GameDVR_FSEBehaviorMode = 2`
  - Warning: Safe

- [ ] **Enable Legacy Fullscreen**
  - Description: "Ensures compatibility with older games expecting exclusive fullscreen"
  - Registry: `HKCU\System\GameConfigStore\GameDVR_DXGIHonorFSEWindowsCompatible = 1`
  - Warning: Safe

- [ ] **Disable Fullscreen Optimizations Per-EXE**
  - Description: "Adds compatibility flag to specific game executables"
  - Operation: Modify EXE compatibility settings in registry
  - Warning: Safe

#### Visual Effects
- [ ] **Disable Window Animations**
  - Description: "Removes minimize/maximize animations for snappier UI"
  - Registry: `HKCU\Control Panel\Desktop\WindowMetrics\MinAnimate = 0`
  - Warning: Safe

- [ ] **Disable Taskbar Animations**
  - Description: "Disables taskbar icon animations"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarAnimations = 0`
  - Warning: Safe

- [ ] **Set Menu Show Delay to 0**
  - Description: "Menus appear instantly"
  - Registry: `HKCU\Control Panel\Desktop\MenuShowDelay = 0`
  - Warning: Safe

- [ ] **Disable Drag Full Windows**
  - Description: "Shows outline instead of full window when dragging"
  - Registry: `HKCU\Control Panel\Desktop\DragFullWindows = 0`
  - Warning: Safe (visual preference)

- [ ] **Disable Listview Shadows**
  - Description: "Removes shadows under icon text"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\ListviewShadow = 0`
  - Warning: Safe

- [ ] **Disable Aero Peek**
  - Description: "Disables taskbar thumbnail previews"
  - Registry: `HKCU\Software\Microsoft\Windows\DWM\EnableAeroPeek = 0`
  - Warning: Safe

- [ ] **Set Visual Effects to Performance**
  - Description: "Applies Windows' 'Adjust for best performance' preset"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects\VisualFXSetting = 2`
  - Warning: Safe

- [ ] **Custom Visual Effects Bitmask**
  - Description: "Fine-grained control over individual visual effects"
  - Registry: `HKCU\Control Panel\Desktop\UserPreferencesMask = 90 12 01 80 10 00 00 00`
  - Warning: Safe

#### Windows 11 Specific
- [ ] **Disable Widgets**
  - Description: "Removes Windows 11 widgets button from taskbar"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDa = 0`
  - Warning: Safe

- [ ] **Disable News & Interests**
  - Description: "Removes taskbar news feed"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarMn = 0`
  - Warning: Safe

- [ ] **Disable Task View Button**
  - Description: "Hides Task View button from taskbar"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\ShowTaskViewButton = 0`
  - Warning: Safe

- [ ] **Hide Search Box from Taskbar**
  - Description: "Removes search box, saves taskbar space"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Search\SearchboxTaskbarMode = 0`
  - Warning: Safe

---

### CATEGORY 6: DEBLOAT & TELEMETRY

#### Disable Telemetry
- [ ] **Disable Telemetry (AllowTelemetry = 0)**
  - Description: "Stops Windows from sending usage data to Microsoft. Saves network bandwidth and resources"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection\AllowTelemetry = 0`
  - Warning: Safe

- [ ] **Disable DiagTrack Service (Connected User Experiences and Telemetry)**
  - Description: "Stops the main telemetry service"
  - Service: DiagTrack → Disabled
  - Warning: Safe

- [ ] **Disable WAP Push Message Routing Service**
  - Description: "Disables push notification telemetry service"
  - Service: dmwappushservice → Disabled
  - Warning: Safe

- [ ] **Disable Windows Error Reporting**
  - Description: "Stops crash report uploads"
  - Registry: `HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\Disabled = 1`
  - Warning: Safe

- [ ] **Disable Activity History**
  - Description: "Stops Windows from tracking your activity"
  - Registry: Multiple ContentDeliveryManager keys
  - Warning: Safe

- [ ] **Disable Tailored Experiences**
  - Description: "Stops Windows from using diagnostic data for personalized tips"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy\TailoredExperiencesWithDiagnosticDataEnabled = 0`
  - Warning: Safe

- [ ] **Disable Advertising ID**
  - Description: "Prevents apps from using your activity for targeted ads"
  - Registry: `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo\Enabled = 0`
  - Warning: Safe

- [ ] **Disable Consumer Features (No Auto-Install Apps)**
  - Description: "Prevents Windows from auto-installing sponsored apps like Candy Crush"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\CloudContent\DisableWindowsConsumerFeatures = 1`
  - Warning: Safe

- [ ] **Disable Feedback Notifications**
  - Description: "Stops Windows from asking for feedback"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection\DoNotShowFeedbackNotifications = 1`
  - Warning: Safe

#### Scheduled Tasks - Disable Telemetry
- [ ] **Disable: Microsoft Compatibility Appraiser**
  - Description: "Collects program telemetry data"
  - Task: `\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser`
  - Warning: Safe

- [ ] **Disable: Customer Experience Improvement Program Tasks**
  - Description: "Multiple CEIP tasks that collect usage data"
  - Tasks: `\Microsoft\Windows\Customer Experience Improvement Program\*`
  - Warning: Safe

- [ ] **Disable: Windows Error Reporting Tasks**
  - Description: "Automated crash reporting"
  - Tasks: `\Microsoft\Windows\Windows Error Reporting\*`
  - Warning: Safe

- [ ] **Disable: DiskDiagnostic Tasks**
  - Description: "Disk diagnostic telemetry"
  - Tasks: `\Microsoft\Windows\DiskDiagnostic\*`
  - Warning: Safe

#### Xbox & Gaming Services Bloat
- [ ] **Disable Xbox Game Bar Overlay**
  - Description: "Removes Xbox overlay (Win+G). Keeps Game Mode functionality"
  - Registry: `HKCU\Software\Microsoft\GameBar\ShowStartupPanel = 0`
  - Warning: Safe

- [ ] **Disable Game DVR Background Recording**
  - Description: "Stops automatic gameplay recording"
  - Registry: `HKLM\SOFTWARE\Microsoft\PolicyManager\default\ApplicationManagement\AllowGameDVR = 0`
  - Warning: Safe

- [ ] **Disable Xbox Services**
  - Description: "Disables Xbox Live services if not using Game Pass"
  - Services: XblAuthManager, XblGameSave, XboxGipSvc, XboxNetApiSvc → Disabled
  - Warning: Careful (breaks Xbox app and Game Pass)

- [ ] **Remove Xbox Apps (Keep Store)**
  - Description: "Uninstalls Xbox apps but preserves Microsoft Store"
  - AppX: Microsoft.Xbox*, excluding Store dependencies
  - Warning: Careful

#### Windows Apps Debloat
- [ ] **Remove Microsoft Bloatware Apps**
  - Description: "Uninstalls pre-installed Microsoft apps you likely don't use"
  - Apps to Remove:
    - Microsoft.BingNews
    - Microsoft.BingWeather
    - Microsoft.BingFinance
    - Microsoft.BingSports
    - Microsoft.GetHelp
    - Microsoft.Getstarted
    - Microsoft.MicrosoftOfficeHub
    - Microsoft.MicrosoftSolitaireCollection
    - Microsoft.People
    - Microsoft.WindowsFeedbackHub
    - Microsoft.WindowsMaps
    - Microsoft.WindowsSoundRecorder
    - Microsoft.YourPhone / Microsoft.PhoneLink
    - Microsoft.ZuneMusic
    - Microsoft.ZuneVideo
    - Microsoft.MixedReality.Portal
  - Warning: Safe

- [ ] **Remove 3rd Party Bloatware**
  - Description: "Removes OEM-installed junk (Candy Crush, etc.)"
  - Apps: CandyCrush*, BubbleWitch*, Netflix, Spotify, Disney+, etc.
  - Warning: Safe

- [ ] **Remove Cortana (If Not Needed)**
  - Description: "Uninstalls Cortana voice assistant"
  - AppX: Microsoft.549981C3F5F10
  - Warning: Careful (Win10 tightly integrated)

#### OneDrive Removal
- [ ] **Fully Uninstall OneDrive**
  - Description: "Completely removes OneDrive from system. Files remain on disk"
  - Operations:
    - Stop OneDrive process
    - Run uninstaller
    - Remove AppX package
    - Clean registry keys
    - Remove startup entries
    - Remove Explorer sidebar entry
  - Warning: Careful (backup OneDrive files first)

#### Edge Debloat
- [ ] **Disable Microsoft Edge Startup Boost**
  - Description: "Prevents Edge from preloading at boot"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Edge\StartupBoostEnabled = 0`
  - Warning: Safe

- [ ] **Disable Edge Background Extensions**
  - Description: "Stops Edge extensions from running when browser is closed"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Edge\BackgroundModeEnabled = 0`
  - Warning: Safe

- [ ] **Remove Edge Scheduled Tasks**
  - Description: "Disables Edge auto-update and telemetry tasks"
  - Tasks: MicrosoftEdge* tasks
  - Warning: Safe

- [ ] **Remove 'Managed by your organization' in Edge**
  - Description: "Removes enterprise policies banner"
  - Registry: Delete `HKLM\SOFTWARE\Policies\Microsoft\Edge`
  - Warning: Safe

---

### CATEGORY 7: STARTUP & SERVICES

#### Startup Management
- [ ] **Scan and List ALL Startup Items**
  - Description: "AutoRuns-like functionality. Shows all programs/services that run at boot"
  - Sources:
    - Registry Run keys (HKLM, HKCU, WOW6432Node)
    - Startup folders (All Users, Current User)
    - Task Scheduler tasks with boot triggers
    - Services set to Automatic
  - Warning: Safe (read-only scan)

- [ ] **Disable Specific Startup Item (User Selectable)**
  - Description: "Allows disabling individual startup items with descriptions"
  - Warning: Varies per item

- [ ] **Safe-to-Disable Preset (Auto-Detect Common Bloat)**
  - Description: "Automatically suggests safe-to-disable startup items"
  - Common targets:
    - Adobe Update services
    - Java Update Scheduler
    - Apple Software Update
    - NVIDIA GeForce Experience (if not used)
    - Discord/Spotify/Steam auto-start
    - Manufacturer bloatware utilities
  - Warning: Safe

#### Services Optimization
- [ ] **Disable Windows Search (WSearch)**
  - Description: "Stops file indexing service. Frees RAM and disk I/O. Use Everything instead"
  - Service: WSearch → Manual or Disabled
  - Warning: Safe (slower file searches in Explorer)

- [ ] **Set BITS to Manual (Background Intelligent Transfer Service)**
  - Description: "Stops background Windows Update downloads when not needed"
  - Service: BITS → Manual
  - Warning: Safe

- [ ] **Disable Windows Biometric Service (WbioSrvc)**
  - Description: "Disables fingerprint reader service if you don't use it"
  - Service: WbioSrvc → Manual or Disabled
  - Warning: Safe

- [ ] **Disable Tablet Input Service**
  - Description: "Disables touch/pen input service on non-touch PCs"
  - Service: TabletInputService → Disabled
  - Warning: Safe (unless using touch/pen)

- [ ] **Disable Windows Media Player Network Sharing**
  - Description: "Stops WMP from sharing media over network"
  - Service: WMPNetworkSvc → Disabled
  - Warning: Safe

- [ ] **Disable Remote Registry**
  - Description: "Security hardening. Prevents remote registry access"
  - Service: RemoteRegistry → Disabled
  - Warning: Safe

- [ ] **Disable Fax Service**
  - Description: "Who uses fax in 2026?"
  - Service: Fax → Disabled
  - Warning: Safe

- [ ] **Disable Print Spooler (If Not Printing)**
  - Description: "Disables print service. Also closes PrintNightmare vulnerability"
  - Service: Spooler → Disabled
  - Warning: Careful (re-enable for printing)

- [ ] **Disable Phone Service**
  - Description: "Disables phone call integration"
  - Service: PhoneSvc → Disabled
  - Warning: Safe

- [ ] **Disable Maps Broker**
  - Description: "Disables Windows Maps background service"
  - Service: MapsBroker → Disabled
  - Warning: Safe

- [ ] **Disable Geolocation Service**
  - Description: "Stops location tracking"
  - Service: lfsvc → Disabled
  - Warning: Safe

- [ ] **Disable Retail Demo Service**
  - Description: "Only for store display PCs"
  - Service: RetailDemo → Disabled
  - Warning: Safe

- [ ] **Disable Windows Wallet Service**
  - Description: "Disables NFC payment service"
  - Service: WalletService → Disabled
  - Warning: Safe

- [ ] **SysMain (Superfetch) - Manual**
  - Description: "Prefetches commonly used programs. Atlas keeps enabled for responsiveness. You can disable for more free RAM"
  - Service: SysMain → Manual
  - Warning: Careful (improves loading times)

---

### CATEGORY 8: FILE SYSTEM & STORAGE

#### NTFS Optimization
- [ ] **Disable Last Access Time Stamps**
  - Description: "Stops NTFS from updating file access times. Reduces disk writes"
  - Command: `fsutil behavior set disablelastaccess 1`
  - Warning: Safe

- [ ] **Disable 8.3 Filename Creation**
  - Description: "Disables legacy DOS 8.3 short filenames. Slight performance gain"
  - Command: `fsutil behavior set disable8dot3 1`
  - Warning: Safe (unless using very old software)

- [ ] **Increase SMB IRPStackSize**
  - Description: "Fixes 'Not enough server storage' errors on network shares"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters\IRPStackSize = 30`
  - Warning: Safe

#### Paging File
- [ ] **Set ClearPageFileAtShutdown to 0**
  - Description: "Disables page file clearing at shutdown for faster shutdowns"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\ClearPageFileAtShutdown = 0`
  - Warning: Safe (slight security trade-off)

- [ ] **Configure Custom Paging File Size**
  - Description: "Sets page file to custom size instead of system-managed"
  - Options: System Managed / Custom (1.5x RAM) / No Paging File (dangerous)
  - Warning: Careful

#### Memory Management
- [ ] **Disable Large System Cache**
  - Description: "Optimizes RAM allocation for applications instead of file cache. Better for gaming workloads"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\LargeSystemCache = 0`
  - Warning: Safe

- [ ] **Enable DisablePagingExecutive**
  - Description: "Keeps Windows kernel in RAM, never pages to disk. Improves responsiveness"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\DisablePagingExecutive = 1`
  - Warning: Careful (requires sufficient RAM)

- [ ] **Set SecondLevelDataCache (Auto-Detect)**
  - Description: "Configures Windows' L2 cache size. Auto-detect CPU cache"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\SecondLevelDataCache = [KB]`
  - Warning: Safe

#### Temp Files Cleanup
- [ ] **Clean Windows Temp Folder**
  - Description: "Deletes C:\Windows\Temp files"
  - Warning: Safe

- [ ] **Clean User Temp Folder**
  - Description: "Deletes %TEMP% files"
  - Warning: Safe

- [ ] **Clean Prefetch (Optional)**
  - Description: "Clears prefetch cache. System will rebuild it"
  - Warning: Careful (temporary slowdown after)

- [ ] **Clean Thumbnail Cache**
  - Description: "Deletes cached image thumbnails. Frees disk space"
  - Warning: Safe

- [ ] **Clean Font Cache**
  - Description: "Rebuilds corrupted font cache"
  - Warning: Safe

- [ ] **Clean DirectX Shader Cache**
  - Description: "Clears compiled shader cache. Games will recompile"
  - Warning: Careful (one-time performance hit)

---

### CATEGORY 9: SECURITY & PRIVACY

#### Disable UAC (DANGEROUS)
- [ ] **Disable User Account Control**
  - Description: "Disables UAC prompts. MAJOR SECURITY RISK. Only for offline/gaming-only PCs"
  - Registry: `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\EnableLUA = 0`
  - Warning: DANGEROUS

#### Disable Core Isolation / HVCI
- [ ] **Disable Virtualization-Based Security (VBS)**
  - Description: "Disables Hyper-V-based security. ~5-10% performance gain but MAJOR security trade-off"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard\EnableVirtualizationBasedSecurity = 0`
  - Command: `bcdedit /set hypervisorlaunchtype off`
  - Warning: DANGEROUS

- [ ] **Disable Hypervisor-Enforced Code Integrity (HVCI)**
  - Description: "Disables memory integrity protection"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity\Enabled = 0`
  - Warning: DANGEROUS

#### Disable Spectre/Meltdown Mitigations
- [ ] **Disable CPU Vulnerability Mitigations**
  - Description: "Disables Spectre/Meltdown patches for ~5% performance gain. SECURITY VULNERABILITY"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\FeatureSettingsOverride = 3`
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\FeatureSettingsOverrideMask = 3`
  - Warning: DANGEROUS

#### Windows Defender
- [ ] **Disable Windows Defender Real-Time Protection**
  - Description: "Stops real-time antivirus scanning. Only do if using 3rd party AV"
  - Registry: Group Policy + service disable
  - Warning: DANGEROUS (unless using other AV)

#### Privacy Settings
- [ ] **Disable Activity History**
  - Description: "Stops Windows from tracking your activity timeline"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\EnableActivityFeed = 0`
  - Warning: Safe

- [ ] **Disable Clipboard History Cloud Sync**
  - Description: "Keeps clipboard history local only"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\AllowClipboardHistory = 0`
  - Warning: Safe

- [ ] **Disable Timeline**
  - Description: "Removes timeline feature from Task View"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\EnableActivityFeed = 0`
  - Warning: Safe

- [ ] **Disable Location Tracking**
  - Description: "Prevents apps from accessing your location"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors\DisableLocation = 1`
  - Warning: Safe

- [ ] **Disable Camera Access**
  - Description: "Blocks apps from using webcam"
  - Registry: LetApps keys
  - Warning: Careful (re-enable for video calls)

- [ ] **Disable Microphone Access**
  - Description: "Blocks apps from using microphone"
  - Warning: Careful (re-enable for calls)

---

### CATEGORY 10: INTERFACE & UX

#### Context Menu
- [ ] **Restore Legacy Context Menu (Windows 11)**
  - Description: "Restores Windows 10-style right-click menu"
  - Registry: Create InprocServer32 key under CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}
  - Warning: Safe

- [ ] **Add 'Run with Priority' Context Menu**
  - Description: "Adds option to launch programs with specific priority"
  - Registry: Add shell verb
  - Warning: Safe

- [ ] **Add 'Open PowerShell/CMD as Admin' Context Menu**
  - Description: "Quick terminal access in any folder"
  - Registry: Add shell verb
  - Warning: Safe

- [ ] **Remove 'Edit with Paint 3D' Context Menu**
  - Description: "Removes bloat from context menu"
  - Registry: Delete Paint3D keys
  - Warning: Safe

#### Taskbar
- [ ] **Disable News & Interests (Win10)**
  - Description: "Removes weather/news widget from taskbar"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Feeds\ShellFeedsTaskbarViewMode = 2`
  - Warning: Safe

- [ ] **Disable Widgets (Win11)**
  - Description: "Removes widgets button"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDa = 0`
  - Warning: Safe

- [ ] **Disable Meet Now Icon**
  - Description: "Removes Skype Meet Now from taskbar"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer\HideSCAMeetNow = 1`
  - Warning: Safe

- [ ] **Disable People Icon**
  - Description: "Removes People icon from taskbar"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\PeopleBand = 0`
  - Warning: Safe

#### Start Menu
- [ ] **Disable Web Search in Start Menu**
  - Description: "Prevents Start Menu from searching Bing"
  - Registry: `HKCU\Software\Policies\Microsoft\Windows\Explorer\DisableSearchBoxSuggestions = 1`
  - Warning: Safe

- [ ] **Disable Start Menu Recommendations (Win11)**
  - Description: "Removes 'Recommended' section from Start Menu"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\Explorer\HideRecommendedSection = 1`
  - Warning: Safe

- [ ] **Install Open-Shell (Classic Start Menu)**
  - Description: "Replaces modern Start Menu with Windows 7-style menu"
  - Operation: Download and install Open-Shell
  - Warning: Safe (optional)

#### File Explorer
- [ ] **Show Hidden Files**
  - Description: "Makes hidden files visible"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\Hidden = 1`
  - Warning: Safe

- [ ] **Show File Extensions**
  - Description: "Shows .exe, .txt, etc. extensions. IMPORTANT for security"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\HideFileExt = 0`
  - Warning: Safe

- [ ] **Disable Quick Access Tracking**
  - Description: "Stops Explorer from tracking recent files"
  - Registry: Multiple ContentDeliveryManager keys
  - Warning: Safe

- [ ] **Disable Automatic Folder Type Discovery**
  - Description: "Stops Explorer from changing folder templates automatically"
  - Registry: `HKCU\Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\Bags`
  - Warning: Safe

- [ ] **Enable Compact View (Win11)**
  - Description: "Reduces spacing in File Explorer"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\UseCompactMode = 1`
  - Warning: Safe

- [ ] **Remove Gallery from File Explorer (Win11)**
  - Description: "Removes Gallery navigation item"
  - Registry: Delete CLSID entries
  - Warning: Safe

- [ ] **Disable App Icons on Thumbnails**
  - Description: "Removes app icons from image thumbnails"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\ShowAppIconsOnThumbnails = 0`
  - Warning: Safe

- [ ] **Set File Explorer to Open 'This PC' by Default**
  - Description: "Opens to This PC instead of Quick Access"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\LaunchTo = 1`
  - Warning: Safe

#### Lock Screen
- [ ] **Disable Lock Screen**
  - Description: "Boots directly to login screen"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\Personalization\NoLockScreen = 1`
  - Warning: Safe

- [ ] **Disable Windows Spotlight (Rotating Backgrounds)**
  - Description: "Stops downloading lock screen images"
  - Registry: ContentDeliveryManager keys
  - Warning: Safe

- [ ] **Disable Lock Screen Tips and Tricks**
  - Description: "Removes suggestions on lock screen"
  - Registry: ContentDeliveryManager keys
  - Warning: Safe

#### Notifications
- [ ] **Disable Notification Center**
  - Description: "Completely removes notification center"
  - Registry: `HKCU\Software\Policies\Microsoft\Windows\Explorer\DisableNotificationCenter = 1`
  - Warning: Careful (no notifications)

- [ ] **Disable Focus Assist**
  - Description: "Removes focus assist notifications"
  - Registry: QuietHoursProfile keys
  - Warning: Safe

#### Other UI
- [ ] **Disable Sticky Keys**
  - Description: "Prevents accidental Sticky Keys popup (5x Shift)"
  - Registry: `HKCU\Control Panel\Accessibility\StickyKeys\Flags = 506`
  - Warning: Safe

- [ ] **Disable Snap Layouts (Win11)**
  - Description: "Disables hover-to-snap window layouts"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\EnableSnapAssistFlyout = 0`
  - Warning: Safe (preference)

- [ ] **Enable NumLock on Startup**
  - Description: "Ensures NumLock is always on at boot"
  - Registry: `HKU\.DEFAULT\Control Panel\Keyboard\InitialKeyboardIndicators = 2`
  - Warning: Safe

- [ ] **Enable Verbose Logon Messages**
  - Description: "Shows detailed startup messages for troubleshooting"
  - Registry: `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\VerboseStatus = 1`
  - Warning: Safe

- [ ] **Remove Shortcut Arrow from Icons**
  - Description: "Removes arrow overlay from shortcuts"
  - Registry: Shell icon overlay
  - Warning: Safe (can be confusing)

- [ ] **Remove '- Shortcut' Text from Shortcut Names**
  - Description: "Auto-removes ' - Shortcut' suffix"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\link = 00 00 00 00`
  - Warning: Safe

---

### CATEGORY 11: SYSTEM TWEAKS

#### Boot & Shutdown
- [ ] **Set Boot Menu Policy to Legacy**
  - Description: "Enables F8 safe mode menu at boot"
  - Command: `bcdedit /set bootmenupolicy Legacy`
  - Warning: Safe

- [ ] **Disable Boot Logo**
  - Description: "Removes Windows logo during boot"
  - Command: `bcdedit /set quietboot on`
  - Warning: Safe

- [ ] **Disable Fast Startup (Hybrid Shutdown)**
  - Description: "Disables hybrid shutdown. Fixes some boot issues, slows shutdown slightly"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Power\HiberbootEnabled = 0`
  - Warning: Safe

- [ ] **Enable AutoEndTasks (Faster Shutdown)**
  - Description: "Automatically ends tasks at shutdown without prompting"
  - Registry: `HKCU\Control Panel\Desktop\AutoEndTasks = 1`
  - Warning: Safe

- [ ] **Reduce WaitToKillServiceTimeout**
  - Description: "Reduces service shutdown wait time from 20s to 2s"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\WaitToKillServiceTimeout = 2000`
  - Warning: Safe

- [ ] **Reduce HungAppTimeout**
  - Description: "Reduces frozen app detection time"
  - Registry: `HKCU\Control Panel\Desktop\HungAppTimeout = 1000`
  - Warning: Safe

- [ ] **Reduce WaitToKillAppTimeout**
  - Description: "Reduces app shutdown timeout"
  - Registry: `HKCU\Control Panel\Desktop\WaitToKillAppTimeout = 2000`
  - Warning: Safe

#### Windows Update
- [ ] **Set Windows Update to Manual**
  - Description: "Prevents automatic updates. Update on your schedule"
  - Registry: Multiple update policy keys
  - Warning: Careful (remember to update manually)

- [ ] **Disable Automatic Driver Updates**
  - Description: "Prevents Windows Update from installing drivers automatically"
  - Registry: `HKLM\SOFTWARE\Policies\Microsoft\Windows\DriverSearching\SearchOrderConfig = 0`
  - Warning: Careful (install drivers manually)

- [ ] **Defer Feature Updates (Delay New Windows Versions)**
  - Description: "Delays major Windows updates by X days"
  - Registry: DeferFeatureUpdatesPeriodInDays
  - Warning: Safe

- [ ] **Pause Windows Updates**
  - Description: "Pauses all updates for 35 days"
  - Registry: PauseUpdatesExpiryTime
  - Warning: Careful

#### Group svchost.exe Processes
- [ ] **Optimize svchost.exe Splitting**
  - Description: "Splits services into separate processes on high-RAM systems for stability"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\SvcHostSplitThresholdInKB = [RAM_IN_KB]`
  - Warning: Safe (requires 8GB+ RAM)

#### Background Apps
- [ ] **Disable Background Apps Globally**
  - Description: "Prevents Store apps from running in background"
  - Registry: `HKCU\Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications\GlobalUserDisabled = 1`
  - Warning: Careful (breaks some app features)

#### Hibernation
- [ ] **Disable Hibernation (Frees Disk Space)**
  - Description: "Disables hibernation, deletes hiberfil.sys (saves GB of disk space)"
  - Command: `powercfg /h off`
  - Warning: Safe (if you don't use hibernate)

#### System Restore
- [ ] **Create Restore Point Before Tweaking**
  - Description: "CRITICAL: Creates restore point for safety"
  - Command: Checkpoint-Computer
  - Warning: Safe (ALWAYS do this)

- [ ] **Allow Multiple Restore Points Per Day**
  - Description: "Removes 24-hour restriction on restore points"
  - Registry: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore\SystemRestorePointCreationFrequency = 0`
  - Warning: Safe

#### Windows Search Indexing
- [ ] **Disable Windows Search Indexing on C:**
  - Description: "Stops indexing system drive. Reduces disk I/O"
  - Operation: Modify indexing locations
  - Warning: Safe (slower searches)

- [ ] **Set Windows Search Service to Manual**
  - Description: "Stops WSearch from running constantly"
  - Service: WSearch → Manual
  - Warning: Safe

#### Registry Tweaks Misc
- [ ] **Enable Long Paths (>260 chars)**
  - Description: "Removes 260-character path length limit"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\LongPathsEnabled = 1`
  - Warning: Safe

- [ ] **Disable Network Throttling**
  - Description: "Removes 10Mbps throttle on multimedia streams"
  - Registry: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\NetworkThrottlingIndex = 0xFFFFFFFF`
  - Warning: Safe

---

### CATEGORY 12: GAMING SPECIFIC (HONE-INSPIRED)

- [ ] **Apply Full Hone Optimization Suite**
  - Description: "Applies ALL Hone.gg tweaks (combines many above tweaks)"
  - Operations:
    - Ultimate Performance Power Plan
    - 0.5ms Timer Resolution
    - MSI Mode for GPU
    - Disable USB/PCIe power saving
    - Optimize memory management
    - Disable telemetry/Xbox services
    - Network TCP optimization
    - Disable transparency/visual effects
    - And more...
  - Warning: Careful (comprehensive changes)

- [ ] **Optimize Raw Mouse Input**
  - Description: "Ensures DirectInput receives unfiltered mouse data"
  - Registry: DirectInput registry keys
  - Warning: Safe

- [ ] **Disable Browser Hardware Acceleration**
  - Description: "Prevents browser from stealing GPU resources when gaming"
  - Operation: Modify Chrome/Firefox/Edge settings
  - Warning: Safe

- [ ] **Flip Integrity Fix**
  - Description: "Fixes flip model presentation issues in some games"
  - Registry: DWM flip policy
  - Warning: Safe

---

### CATEGORY 13: HARDWARE SPECIFIC

#### USB Optimization
- [ ] **Disable USB Selective Suspend (All Hubs)**
  - Description: "Prevents USB devices from entering power-saving mode. Fixes mouse/keyboard lag"
  - Registry: Per-device DisableSelectiveSuspend
  - Warning: Safe

#### SATA/NVMe
- [ ] **Enable Write Caching**
  - Description: "Improves disk performance. Requires UPS to prevent data loss"
  - Device Manager: Enable write caching
  - Warning: Careful (data loss risk on power failure)

#### Audio
- [ ] **Disable Audio Enhancements**
  - Description: "Removes audio processing for lower latency"
  - Registry: Audio device enhancements
  - Warning: Safe (may reduce audio quality)

- [ ] **Set Audio Sample Rate to Native**
  - Description: "Matches sample rate to hardware for no resampling"
  - Audio Properties: 44.1kHz or 48kHz
  - Warning: Safe

---

### CATEGORY 14: ADVANCED / DANGEROUS TWEAKS

- [ ] **Block Razer Software Installation (NTFS Deny)**
  - Description: "Prevents Razer bloatware from installing. Hardware still works"
  - Operation: Create C:\Razer with DENY permissions
  - Warning: Careful (if you need Razer software)

- [ ] **Disable Intel LMS (vPro)**
  - Description: "Disables Intel Management Engine services"
  - Service: LMS → Disabled
  - Warning: Careful (enterprise feature)

- [ ] **Disable Windows Platform Binary Table (WPBT)**
  - Description: "Prevents OEM from force-running software at boot"
  - Registry: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\kernel\DisableWpbtExecution = 1`
  - Warning: Safe

---

### CATEGORY 15: MONITORING & DIAGNOSTICS

#### Latency Monitoring
- [ ] **Enable DPC/ISR Latency Tracking**
  - Description: "Tracks driver latency for troubleshooting stuttering"
  - Operation: Windows Performance Toolkit integration
  - Warning: Safe (monitoring only)

#### Benchmark Tools
- [ ] **Network Latency Test**
  - Description: "Tests ping to game servers"
  - Warning: Safe

- [ ] **DNS Response Time Test**
  - Description: "Measures DNS query latency"
  - Warning: Safe

- [ ] **Disk Benchmark**
  - Description: "Tests disk read/write speed"
  - Warning: Safe

---

### CATEGORY 16: BACKUP & RESTORE

- [ ] **Enable Automatic Registry Backup Before Tweaks**
  - Description: "ALWAYS enabled. Backs up registry before modifications"
  - Warning: Safe (CRITICAL feature)

- [ ] **Create Full System Restore Point**
  - Description: "Creates restore point via Windows System Restore"
  - Warning: Safe (ALWAYS do before tweaking)

- [ ] **Export Current Configuration as Preset**
  - Description: "Saves your current tweak selections as custom profile"
  - Warning: Safe

- [ ] **Import Preset Configuration**
  - Description: "Loads saved preset profile"
  - Warning: Varies

- [ ] **Undo All Tweaks (Restore Original Values)**
  - Description: "Reverts ALL changes made by this tool"
  - Warning: Safe (uses backed-up values)

---

## 🎯 PRESET PROFILES

### Profile: NORMAL
**Description**: Basic optimizations, maximum compatibility
**Tweaks included**:
- Disable telemetry
- Disable mouse acceleration
- Set max refresh rate
- Disable bloatware apps
- Basic TCP optimization
- Disable unnecessary services (safe list)

### Profile: BALANCED
**Description**: Moderate performance gains, good compatibility
**Includes NORMAL + **:
- Visual effects for performance
- Disable Superfetch
- Network adapter optimization
- GPU basic tweaks
- Startup optimization

### Profile: GAMING
**Description**: Aggressive performance, minor compatibility trade-offs
**Includes BALANCED +**:
- Ultimate Performance power plan
- Timer resolution optimization
- Disable fullscreen optimizations
- MSI Mode for GPU
- Advanced TCP tweaks
- Disable VBS (if safe)

### Profile: EXTREME
**Description**: Maximum performance, security/compatibility risks
**Includes GAMING +**:
- Disable UAC (optional)
- Disable Windows Defender (if using other AV)
- Disable Spectre/Meltdown mitigations
- Aggressive memory management
- Remove ALL bloatware
- Disable core isolation

---

## 🛠️ TECHNICAL IMPLEMENTATION NOTES

### AI Agent Instructions

#### Research & Documentation
- **DO NOT HESITATE** to search online for:
  - Registry key documentation
  - Windows API documentation
  - PowerShell cmdlets
  - Rust Windows crates (windows-rs, winreg, etc.)
  - Tauri command patterns
- Check Microsoft Docs, Stack Overflow, Reddit (r/Windows10, r/WindowsTweaks)
- Look for existing implementations on GitHub

#### Code Quality Standards
- **NO HARDCODING**: All tweaks MUST be data-driven
  - Use structs/enums for tweak definitions
  - Store descriptions/values in constants or config files
  - Make it easy to add new tweaks without touching core logic

- **Error Handling**: EVERY operation must handle errors gracefully
  - Don't panic on registry errors
  - Log errors clearly for debugging
  - Show user-friendly error messages

- **Undo Capability**: ALWAYS store original values before modification
  - Use a SQLite database or JSON file to track changes
  - Format: `{ tweak_id: "disable_telemetry", original_value: 1, new_value: 0, timestamp: ... }`

- **Privilege Handling**:
  - Detect if running as admin
  - Request elevation via Tauri if needed
  - Show clear error if user denies elevation

- **i18n Architecture**:
  - Use `rust-i18n` or similar for backend
  - Use proper Svelte i18n library (e.g., `svelte-i18n`)
  - Structure: `category.tweak_id.name`, `category.tweak_id.description`
  - English only for MVP, but code must support adding languages later

#### Registry Operations Best Practices
```rust
// Example pattern (adapt as needed)
fn apply_registry_tweak(tweak: &RegistryTweak) -> Result<(), Error> {
    // 1. Open registry key
    let hkey = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(&tweak.path, KEY_ALL_ACCESS)?;
    
    // 2. Backup original value
    let original = hkey.get_value(&tweak.key).ok();
    store_backup(&tweak.id, original)?;
    
    // 3. Apply new value
    match &tweak.value {
        RegistryValue::DWord(val) => hkey.set_value(&tweak.key, val)?,
        RegistryValue::String(val) => hkey.set_value(&tweak.key, val)?,
        // ...
    }
    
    Ok(())
}
```

#### HKU (HKEY_USERS) Handling
- Mount user hives properly: `reg load HKU\TempUser C:\Users\Username\NTUSER.DAT`
- Apply changes to currently logged-in user: `HKU\{SID}`
- Unload hives after: `reg unload HKU\TempUser`

#### Testing Strategy
- Test on Windows 10 AND Windows 11
- Test with admin AND non-admin privileges
- Test undo functionality for every

