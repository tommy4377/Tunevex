<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import CategorySidebar from "../components/CategorySidebar.svelte";
  import HomeDashboard from "../components/home/HomeDashboard.svelte";
  import ActivationDashboard from "../components/activation/ActivationDashboard.svelte";
  import NetworkDashboard from "../components/network/NetworkDashboard.svelte";
  import SecurityDashboard from "../components/security/SecurityDashboard.svelte";
  import PrivacyDashboard from "../components/privacy/PrivacyDashboard.svelte";
  import DebloatDashboard from "../components/debloat/DebloatDashboard.svelte";
  import DisplayDashboard from "../components/display/DisplayDashboard.svelte";
  import StartupManager from "../components/startup/StartupManager.svelte";
  import StorageDashboard from "../components/storage/StorageDashboard.svelte";
  import CpuDashboard from "../components/cpu/CpuDashboard.svelte";
  import GamingDashboard from "../components/gaming/GamingDashboard.svelte";
  import GpuDashboard from "../components/gpu/GpuDashboard.svelte";
  import SystemDashboard from "../components/system/SystemDashboard.svelte";
  import InputDashboard from "../components/input/InputDashboard.svelte";
  import AiDashboard from "../components/ai/AiDashboard.svelte";

  import UIDashboard from "../components/ui/UIDashboard.svelte";
  import TweakList from "../components/TweakList.svelte";
  import ProfileManager from "../components/ProfileManager.svelte";
  import type { Tweak } from "$lib/types";
  import { activeCategory } from "$lib/stores";

  let tweaks: Tweak[] = [];
  let loading = true;
  let error: string | null = null;
  let currentCat: string | null = null;
  let checkedCategories: Set<string> = new Set();
  let unlistenCheckResult: (() => void) | null = null;
  let allSearch = "";
  let allRisk = "All";
  let allCategory = "All";

  const categoryMap: Record<string, string> = {
    Network: "Network",
    SecurityPrivacy: "SecurityPrivacy",
    Privacy: "Privacy",
    DebloatTelemetry: "DebloatTelemetry",
    DisplayMonitor: "DisplayMonitor",
    FileSystem: "FileSystem",
    CpuPerformance: "CpuPerformance",
    Performance: "CpuPerformance",
    GameOptimizations: "GameOptimizations",
    GpuOptimization: "GpuOptimization",
    System: "System",
    MouseInput: "MouseInput",
    InterfaceUx: "InterfaceUx",
    Activation: "Activation",
    Home: "Home",
    AiAdvisor: "AiAdvisor",
    AllTweaks: "AllTweaks",
    StartupServices: "StartupServices",
  };

  const overviewMeta: Record<string, { title: string; description: string; category: string }> = {
    Network: { title: "Network", description: "DNS, adapter, TCP/IP, and protocol controls.", category: "Network" },
    SecurityPrivacy: { title: "Security", description: "Protection, authentication, firewall, and advanced hardening.", category: "SecurityPrivacy" },
    Privacy: { title: "Privacy", description: "Telemetry, diagnostics, permissions, and data collection.", category: "Privacy" },
    DebloatTelemetry: { title: "Debloat", description: "Optional Windows apps, features, services, and browser components.", category: "DebloatTelemetry" },
    CpuPerformance: { title: "CPU", description: "Scheduling, power, memory, timers, and processor-specific controls.", category: "CpuPerformance" },
    GpuOptimization: { title: "GPU", description: "Driver, latency, interrupt, and vendor-specific graphics controls.", category: "GpuOptimization" },
    GameOptimizations: { title: "Gaming", description: "Game Mode, capture, scheduling, and compatibility controls.", category: "GameOptimizations" },
    System: { title: "System", description: "Windows behavior, services, recovery, and maintenance.", category: "System" },
    InterfaceUx: { title: "Interface", description: "Explorer, taskbar, context menu, and visual preferences.", category: "InterfaceUx" },
    MouseInput: { title: "Input", description: "Mouse, keyboard, USB, and controller responsiveness.", category: "MouseInput" },
    DisplayMonitor: { title: "Display", description: "Monitor, graphics, color, and presentation settings.", category: "DisplayMonitor" },
    FileSystem: { title: "Storage", description: "Disk, NTFS, NVMe, cleanup, and compression controls.", category: "FileSystem" },
  };

  $: currentOverview = currentCat ? overviewMeta[currentCat] : undefined;
  $: currentOverviewCount = currentOverview
    ? tweaks.filter((t) => t.category === currentOverview.category).length
    : 0;
  $: allCategories = Array.from(new Set(tweaks.map((t) => t.category))).sort();
  $: filteredAllTweaks = tweaks.filter((t) => {
    const query = allSearch.trim().toLowerCase();
    const matchesText = !query || `${t.name} ${t.description} ${t.id}`.toLowerCase().includes(query);
    const matchesRisk = allRisk === "All" || t.warning_level === allRisk;
    const matchesCategory = allCategory === "All" || t.category === allCategory;
    return matchesText && matchesRisk && matchesCategory;
  });

  let checkQueue: string[] = [];
  let isChecking = false;

  activeCategory.subscribe((c) => {
    const prevCat = currentCat;
    currentCat = c;

    if (!loading && c && prevCat !== c) {
      checkedCategories.delete(c);
      checkCategoryNow(c);
    }
  });

  async function checkCategoryNow(cat: string) {
    const rustCategory = categoryMap[cat];
    if (cat === "AllTweaks") {
      await reloadTweaks();
      return;
    }
    if (!rustCategory || checkedCategories.has(cat) || cat === "Home" || cat === "AiAdvisor") return;

    checkedCategories.add(cat);
    try {
      await invoke("check_category", { category: rustCategory });
    } catch (e) {
      console.error("Failed to check category:", cat, e);
    }
  }

  async function processBackgroundQueue() {
    if (isChecking) return;
    isChecking = true;

    while (checkQueue.length > 0) {
      const cat = checkQueue.shift()!;
      if (!checkedCategories.has(cat)) {
        await checkCategoryNow(cat);
        await new Promise((r) => setTimeout(r, 100));
      }
    }

    isChecking = false;
  }

  async function reloadTweaks() {
    try {
      tweaks = await invoke<Tweak[]>("get_tweaks");
      checkedCategories.clear();
    } catch (e) {
      error = `Failed to refresh tweak states: ${String(e)}`;
    }
  }

onMount(async () => {
  try {
    tweaks = await invoke<Tweak[]>('get_tweaks_fast');
    loading = false;

    unlistenCheckResult = await listen<{id: string, enabled: boolean | null}>('tweak-check-result', (event) => {
      const { id, enabled } = event.payload;
      tweaks = tweaks.map(t => t.id === id ? { ...t, enabled } : t);
    });

    // Check only the active category; do not start a background queue
    const startCategory = currentCat || 'Home';
    await checkCategoryNow(startCategory);

    // Background queue intentionally disabled here
  } catch (e: any) {
    error = e.toString();
    loading = false;
  }
});

  onDestroy(() => {
    if (unlistenCheckResult) unlistenCheckResult();
  });
</script>

<div class="main-layout">
  <CategorySidebar />

  {#if loading}
    <div class="center-msg">
      <div class="spinner"></div>
      <p>Loading optimization modules...</p>
    </div>
  {:else if error}
    <div class="center-msg error">
      <p>Error loading modules:</p>
      <pre>{error}</pre>
    </div>
  {:else}
    <div class="content-area">
      {#if currentOverview}
        <header class="page-heading">
          <div>
            <h1>{currentOverview.title}</h1>
            <p>{currentOverview.description}</p>
          </div>
          <span>{currentOverviewCount} tweaks</span>
        </header>
      {/if}
      <div class="section-body">
      {#if currentCat === "Home"}
        <HomeDashboard />
      {:else if currentCat === "Network"}
        <NetworkDashboard allTweaks={tweaks} />
      {:else if currentCat === "SecurityPrivacy"}
        <SecurityDashboard allTweaks={tweaks} />
      {:else if currentCat === "Privacy"}
        <PrivacyDashboard allTweaks={tweaks} />
      {:else if currentCat === "DebloatTelemetry"}
        <DebloatDashboard allTweaks={tweaks} />
      {:else if currentCat === "DisplayMonitor"}
        <DisplayDashboard allTweaks={tweaks} />
      {:else if currentCat === "StartupServices"}
        <StartupManager allTweaks={tweaks} />
      {:else if currentCat === "FileSystem"}
        <StorageDashboard allTweaks={tweaks} />
      {:else if currentCat === "CpuPerformance" || currentCat === "Performance"}
        <CpuDashboard allTweaks={tweaks} />
      {:else if currentCat === "GameOptimizations"}
        <GamingDashboard allTweaks={tweaks} />
      {:else if currentCat === "GpuOptimization"}
        <GpuDashboard allTweaks={tweaks} />
      {:else if currentCat === "System"}
        <SystemDashboard allTweaks={tweaks} />
      {:else if currentCat === "MouseInput"}
        <InputDashboard allTweaks={tweaks} />
      {:else if currentCat === "InterfaceUx"}
        <UIDashboard allTweaks={tweaks} />
      {:else if currentCat === "Activation"}
        <ActivationDashboard allTweaks={tweaks} />
      {:else if currentCat === "AiAdvisor"}
        <AiDashboard allTweaks={tweaks} />
      {:else if currentCat === "AllTweaks"}
        <section class="all-tweaks-shell">
          <header class="all-tweaks-header">
            <div>
              <h1>All Tweaks</h1>
              <p>{filteredAllTweaks.length} of {tweaks.length} controls shown</p>
            </div>
            <div class="all-tweaks-tools">
              <ProfileManager on:applied={reloadTweaks} />
              <div class="all-tweaks-filters">
                <input bind:value={allSearch} aria-label="Search tweaks" placeholder="Search name, description, or ID" />
                <select bind:value={allRisk} aria-label="Filter by risk">
                  <option>All</option>
                  <option>Safe</option>
                  <option>Careful</option>
                  <option>Dangerous</option>
                </select>
                <select bind:value={allCategory} aria-label="Filter by category">
                  <option>All</option>
                  {#each allCategories as category}<option value={category}>{category}</option>{/each}
                </select>
              </div>
            </div>
          </header>
          <div class="all-tweaks-list">
            <TweakList tweaks={filteredAllTweaks} showHeader={false} />
          </div>
        </section>
      {:else}
        <TweakList bind:tweaks />
      {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .main-layout {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .content-area {
    flex: 1;
    min-width: 0;
    height: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .section-body {
    flex: 1;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }

  .page-heading {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 18px 24px 14px;
    border-bottom: 1px solid var(--border-color);
  }

  .page-heading h1,
  .all-tweaks-header h1 {
    margin: 0 0 4px;
    color: var(--text-color);
    font-size: 22px;
    line-height: 1.2;
  }

  .page-heading p,
  .all-tweaks-header p {
    margin: 0;
    color: var(--text-muted);
    font-size: 12px;
  }

  .page-heading > span {
    flex-shrink: 0;
    padding: 4px 9px;
    border: 1px solid rgba(var(--accent-rgb), 0.25);
    border-radius: var(--radius-pill);
    background: rgba(var(--accent-rgb), 0.1);
    color: var(--accent-hover);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
  }

  .all-tweaks-shell {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .all-tweaks-header {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 18px 24px 14px;
    border-bottom: 1px solid var(--border-color);
  }

  .all-tweaks-filters {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    min-width: 0;
  }

  .all-tweaks-tools {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    min-width: 0;
  }

  .all-tweaks-filters input,
  .all-tweaks-filters select {
    min-width: 0;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    background: rgba(255, 255, 255, 0.04);
    color: var(--text-color);
    padding: 7px 10px;
    font: inherit;
    font-size: 12px;
    outline: none;
  }

  .all-tweaks-filters input { width: min(240px, 30vw); }
  .all-tweaks-filters select { max-width: 150px; }
  .all-tweaks-filters option { background: #202020; }
  .all-tweaks-filters :is(input, select):focus { border-color: var(--accent-color); }

  .all-tweaks-list {
    flex: 1;
    min-height: 0;
    padding: 12px 16px 0;
  }

  @media (max-width: 940px) {
    .page-heading { padding: 14px 16px 12px; }
    .all-tweaks-header { align-items: stretch; flex-direction: column; padding: 14px 16px 12px; }
    .all-tweaks-tools { align-items: stretch; flex-direction: column; }
    .all-tweaks-filters { justify-content: stretch; }
    .all-tweaks-filters input { flex: 1; width: auto; }
    .all-tweaks-filters select { width: 112px; }
  }

  .center-msg {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    color: var(--text-muted);
    gap: 16px;
  }

  .error {
    color: var(--danger-color);
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--border-color);
    border-top-color: var(--accent-color);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
