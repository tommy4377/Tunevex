<script lang="ts">
    import { fade } from "svelte/transition";
    import { Monitor, Gamepad2, Settings } from "lucide-svelte";
    import { Card, CardGrid, BackButton } from "../ui";
    import MonitorSection from "./MonitorSection.svelte";
    import GpuSection from "./GpuSection.svelte";
    import SystemSection from "./SystemSection.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let currentView: "dashboard" | "monitor" | "gpu" | "system" = "dashboard";

    // Filters - Keyword-based filtering
    $: monitorTweaks = allTweaks.filter(
        (t) =>
            t.category === "DisplayMonitor" &&
            (t.id.includes("refresh") ||
                t.id.includes("dpi") ||
                t.id.includes("color") ||
                t.id.includes("monitor")),
    );
    $: gpuTweaks = allTweaks.filter(
        (t) =>
            t.category === "DisplayMonitor" &&
            (t.id.includes("vrr") ||
                t.id.includes("scaling") ||
                t.id.includes("hdr") ||
                t.id.includes("latency")),
    );
    $: systemTweaks = allTweaks.filter(
        (t) =>
            t.category === "DisplayMonitor" &&
            (t.id.includes("timer") || t.id.includes("resolution")),
    );
</script>

<div class="display-container">
    {#if currentView === "dashboard"}
        <CardGrid columns="repeat(3, 1fr)">
            <Card
                icon={Monitor}
                title="Monitor Configuration"
                description="Refresh rates, colors, and DPI."
                status="{monitorTweaks.length} tweaks"
                onclick={() => (currentView = "monitor")}
            />
            <Card
                icon={Gamepad2}
                title="GPU & Graphics"
                description="VRR, GPU scaling, and latency."
                status="{gpuTweaks.length} tweaks"
                onclick={() => (currentView = "gpu")}
            />
            <Card
                icon={Settings}
                title="System Latency"
                description="Timer resolution and deep tweaks."
                status="{systemTweaks.length} tweaks"
                onclick={() => (currentView = "system")}
            />
        </CardGrid>
    {:else}
        <div class="detail-view" in:fade>
            <BackButton onclick={() => (currentView = "dashboard")} />

            {#if currentView === "monitor"}
                <MonitorSection tweaks={monitorTweaks} />
            {:else if currentView === "gpu"}
                <GpuSection tweaks={gpuTweaks} />
            {:else if currentView === "system"}
                <SystemSection tweaks={systemTweaks} />
            {/if}
        </div>
    {/if}
</div>

<style>
    .display-container {
        height: 100%;
        color: var(--text-color);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        overflow-y: auto;
    }
</style>
