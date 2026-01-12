<script lang="ts">
    import { fade } from "svelte/transition";
    import MonitorSection from "./MonitorSection.svelte";
    import GpuSection from "./GpuSection.svelte";
    import SystemSection from "./SystemSection.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Sub-routes
    let currentView: "dashboard" | "monitor" | "gpu" | "system" = "dashboard";

    // Filter tweaks on mount or when allTweaks changes
    $: monitorTweaks = allTweaks.filter((t) =>
        [
            "display_max_refresh_rate",
            "display_dpi_100",
            "display_8bit_color",
        ].includes(t.id),
    );
    $: gpuTweaks = allTweaks.filter((t) =>
        ["display_enable_vrr", "display_no_gpu_scaling"].includes(t.id),
    );
    $: systemTweaks = allTweaks.filter((t) =>
        ["display_timer_resolution"].includes(t.id),
    );
</script>

<div class="display-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- Monitor Card -->
            <div
                class="card monitor-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "monitor")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "monitor")}
            >
                <div class="card-icon">🖥️</div>
                <h3>Monitor Configuration</h3>
                <p>Refresh rates, colors, and DPI.</p>
                <div class="status">{monitorTweaks.length} tweaks</div>
            </div>

            <!-- GPU Card -->
            <div
                class="card gpu-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "gpu")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "gpu")}
            >
                <div class="card-icon">🎮</div>
                <h3>GPU & Graphics</h3>
                <p>VRR, GPU scaling, and latency.</p>
                <div class="status">{gpuTweaks.length} tweaks</div>
            </div>

            <!-- System Card -->
            <div
                class="card system-card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "system")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "system")}
            >
                <div class="card-icon">⚙️</div>
                <h3>System Latency</h3>
                <p>Timer resolution and deep tweaks.</p>
                <div class="status">{systemTweaks.length} tweaks</div>
            </div>
        </div>
    {:else}
        <div class="detail-view" in:fade>
            <button
                class="back-btn"
                on:click={() => (currentView = "dashboard")}
            >
                ← Back to Dashboard
            </button>

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

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 24px;
        margin-top: 20px;
        overflow-y: auto;
        flex: 1;
        padding: 24px;
        padding-top: 4px;
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 24px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        position: relative;
        z-index: 1;
    }

    .card:hover {
        background: rgba(255, 255, 255, 0.06);
        transform: translateY(-2px);
        border-color: var(--accent-color);
        z-index: 10;
        position: relative;
    }

    .card-icon {
        font-size: 32px;
        margin-bottom: 16px;
    }

    h3 {
        margin: 0 0 8px 0;
        font-size: 18px;
        font-weight: 600;
    }

    p {
        margin: 0 0 24px 0;
        color: var(--text-muted);
        font-size: 14px;
        line-height: 1.5;
        flex-grow: 1;
    }

    .status {
        font-size: 12px;
        font-weight: 500;
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.1);
        padding: 6px 12px;
        border-radius: 20px;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        overflow-y: auto;
    }

    .back-btn {
        align-self: flex-start;
        background: none;
        border: none;
        color: var(--text-muted);
        font-size: 14px;
        cursor: pointer;
        padding: 8px 0;
        margin-bottom: 16px;
        transition: color 0.2s;
    }

    .back-btn:hover {
        color: var(--text-color);
    }
</style>
