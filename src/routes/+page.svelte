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
  import ProgramsDashboard from "../components/programs/ProgramsDashboard.svelte";
  import UIDashboard from "../components/ui/UIDashboard.svelte";
  import TweakList from "../components/TweakList.svelte";
  import type { Tweak } from "$lib/types";
  import { activeCategory } from "$lib/stores";

  let tweaks: Tweak[] = [];
  let loading = true;
  let error: string | null = null;
  let currentCat: string | null = null;
  let checkedCategories: Set<string> = new Set();
  let unlistenCheckResult: (() => void) | null = null;

  // Map UI category names to Rust TweakCategory debug names
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
  };

  // Queue of categories to check in background
  let checkQueue: string[] = [];
  let isChecking = false;

  activeCategory.subscribe((c) => {
    const prevCat = currentCat;
    currentCat = c;

    // If category changed and not already checked, prioritize it
    if (c && prevCat !== c && !checkedCategories.has(c)) {
      checkCategoryNow(c);
    }
  });

  async function checkCategoryNow(cat: string) {
    const rustCategory = categoryMap[cat];
    // Skip Home as it has no tweaks to check, preventing unnecessary overhead
    if (!rustCategory || checkedCategories.has(cat) || cat === "Home") return;

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
        // Small delay between categories to avoid overwhelming
        await new Promise((r) => setTimeout(r, 100));
      }
    }

    isChecking = false;
  }

  onMount(async () => {
    try {
      // 1. Fast load - instant UI
      tweaks = await invoke("get_tweaks_fast");
      loading = false;

      // 2. Listen for check results
      unlistenCheckResult = await listen<{ id: string; enabled: boolean }>(
        "tweak-check-result",
        (event: any) => {
          const { id, enabled } = event.payload;
          tweaks = tweaks.map((t) => (t.id === id ? { ...t, enabled } : t));
        },
      );

      // 3. Check current category first (Home by default)
      const startCategory = currentCat || "Home";
      await checkCategoryNow(startCategory);

      // 4. Queue other categories for background loading
      const allCategories = Object.keys(categoryMap);
      checkQueue = allCategories.filter((c) => c !== startCategory);
      processBackgroundQueue();
    } catch (e: any) {
      console.error("Failed to load tweaks:", e);
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
        <StartupManager />
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
      {:else if currentCat === "Programs"}
        <ProgramsDashboard />
      {:else if currentCat === "InterfaceUx"}
        <UIDashboard allTweaks={tweaks} />
      {:else if currentCat === "Activation"}
        <ActivationDashboard allTweaks={tweaks} />
      {:else}
        <TweakList bind:tweaks />
      {/if}
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
    height: 100%;
    overflow: hidden;
    display: flex; /* Ensure children fill height */
    flex-direction: column;
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
