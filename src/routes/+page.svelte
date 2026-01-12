<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import CategorySidebar from "../components/CategorySidebar.svelte";
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
  import RestoreDashboard from "../components/system/RestoreDashboard.svelte";
  import UIDashboard from "../components/ui/UIDashboard.svelte";
  import TweakList from "../components/TweakList.svelte";
  import type { Tweak } from "$lib/types";
  import { activeCategory } from "$lib/stores";

  let tweaks: Tweak[] = [];
  let loading = true;
  let error: string | null = null;
  let currentCat: string | null = null;

  activeCategory.subscribe((c) => (currentCat = c));

  onMount(async () => {
    try {
      tweaks = await invoke("get_tweaks");
    } catch (e: any) {
      console.error("Failed to load tweaks:", e);
      error = e.toString();
    } finally {
      loading = false;
    }
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
      {#if currentCat === "Network"}
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
      {:else if currentCat === "Restore"}
        <RestoreDashboard />
      {:else if currentCat === "InterfaceUx"}
        <UIDashboard allTweaks={tweaks} />
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
