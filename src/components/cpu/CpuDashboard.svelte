<script lang="ts">
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/core";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Navigation state
    let currentView: "dashboard" | "scheduling" | "power" | "memory" | "timer" =
        "dashboard";

    // --- Filters ---

    $: schedulingTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("priority") ||
                t.id.includes("fth") ||
                t.id.includes("svchost") ||
                t.id.includes("page_combining") ||
                t.id.includes("sleep_study")),
    );

    $: powerTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("power") ||
                t.id.includes("performance") ||
                t.id.includes("usb") ||
                t.id.includes("pcie") ||
                t.id.includes("core_parking") ||
                t.id.includes("idle") ||
                t.id.includes("acpi") ||
                t.id.includes("throttle")),
    );

    $: memoryTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("ntfs") ||
                t.id.includes("memory") ||
                t.id.includes("cache") ||
                t.id.includes("paging") ||
                t.id.includes("paging") ||
                t.id.includes("prefetch")),
    );

    $: timerTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("timer") ||
                t.id.includes("tsc") ||
                t.id.includes("tick") ||
                t.id.includes("clock") ||
                t.id.includes("boot") ||
                t.id.includes("processor_check_interval")),
    );

    // Helper for "Apply Safe Tweaks"
    async function applySafeTweaks(tweaks: Tweak[]) {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    await invoke("apply_tweak", { id: tweak.id });
                    tweak.enabled = true;
                } catch (e) {
                    console.error(`Failed to apply tweak ${tweak.id}:`, e);
                }
            }
        }
        allTweaks = allTweaks; // Trigger updates
    }
</script>

<div class="cpu-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- Scheduling Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "scheduling")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "scheduling")}
            >
                <div class="card-icon">🎯</div>
                <h3>Scheduling & Priority</h3>
                <p>Optimize CPU thread handling and priorities.</p>
                <div class="status">{schedulingTweaks.length} tweaks</div>
            </div>

            <!-- Power Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "power")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "power")}
            >
                <div class="card-icon">🔌</div>
                <h3>Power Management</h3>
                <p>High performance power plans and throttling.</p>
                <div class="status">{powerTweaks.length} tweaks</div>
            </div>

            <!-- Memory Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "memory")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "memory")}
            >
                <div class="card-icon">💾</div>
                <h3>Memory & Storage</h3>
                <p>RAM management, caching, and paging file.</p>
                <div class="status">{memoryTweaks.length} tweaks</div>
            </div>

            <!-- Timer Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "timer")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "timer")}
            >
                <div class="card-icon">⏱️</div>
                <h3>Timer & Boot</h3>
                <p>System timers, HPET, and boot configuration.</p>
                <div class="status">{timerTweaks.length} tweaks</div>
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

            <div class="section-content">
                {#if currentView === "scheduling"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🎯 Scheduling & Priority</h2>
                            <p>
                                Fine-tune how Windows allocates CPU resources.
                            </p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(schedulingTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList
                            tweaks={schedulingTweaks}
                            showHeader={false}
                        />
                    </div>
                {:else if currentView === "power"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🔌 Power Management</h2>
                            <p>Unlock maximum performance power states.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(powerTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={powerTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "memory"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>💾 Memory & Storage</h2>
                            <p>System cache and memory optimizations.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(memoryTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={memoryTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "timer"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>⏱️ Timer & Boot Config</h2>
                            <p>Low-level timer resolution and boot settings.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(timerTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={timerTweaks} showHeader={false} />
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .cpu-container {
        height: 100%;
        color: var(--text-color);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
        gap: 20px;
        margin-top: 20px;
        overflow-y: auto;
        flex: 1;
        padding: 24px;
        padding-top: 4px;
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
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
        color: var(--text-color);
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
    }

    .section-content {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .section-header {
        margin-bottom: 24px;
        padding-bottom: 16px;
        border-bottom: 1px solid var(--border-color);
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .header-text h2 {
        font-size: 20px;
        margin: 0 0 8px 0;
        color: var(--text-color);
    }
    .header-text p {
        margin: 0 0 16px 0;
        color: var(--text-muted);
        font-size: 14px;
    }

    .optimize-btn {
        border: none;
        padding: 8px 16px;
        border-radius: var(--radius-sm);
        font-weight: 500;
        cursor: pointer;
        color: white;
        white-space: nowrap;
    }
    .optimize-btn.safe {
        background: #10b981;
    }
    .optimize-btn.safe:hover {
        background: #059669;
    }

    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
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
