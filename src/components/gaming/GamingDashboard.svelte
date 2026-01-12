<script lang="ts">
    import { fade } from "svelte/transition";
    import TweakList from "../TweakList.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Navigation state
    let currentView: "dashboard" | "general" | "visuals" | "priority" =
        "dashboard";

    // Visuals / Graphics (DVR, GameBar, FSO)
    $: visualTweaks = allTweaks.filter(
        (t) =>
            t.category === "GameOptimizations" &&
            (t.id.includes("dvr") ||
                t.id.includes("gamebar") ||
                t.id.includes("fso") ||
                t.id.includes("transparency") ||
                t.id.includes("visual")),
    );

    // Process / Priority (Multimedia Class Scheduling, Priority)
    $: priorityTweaks = allTweaks.filter(
        (t) =>
            t.category === "GameOptimizations" &&
            (t.id.includes("mmcss") ||
                t.id.includes("priority") ||
                t.id.includes("affinity") ||
                t.id.includes("throttling")),
    );

    // General Gaming (everything else)
    $: generalTweaks = allTweaks.filter(
        (t) =>
            t.category === "GameOptimizations" &&
            !visualTweaks.includes(t) &&
            !priorityTweaks.includes(t),
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

<div class="gaming-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- General Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "general")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "general")}
            >
                <div class="card-icon">🎮</div>
                <h3>General Gaming</h3>
                <p>Core Windows gaming settings and Game Mode.</p>
                <div class="status">{generalTweaks.length} tweaks</div>
            </div>

            <!-- Visuals Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "visuals")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "visuals")}
            >
                <div class="card-icon">🎨</div>
                <h3>Visual Optimizations</h3>
                <p>Disable overlays, DVR, and full-screen optimizations.</p>
                <div class="status">{visualTweaks.length} tweaks</div>
            </div>

            <!-- Priority Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "priority")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "priority")}
            >
                <div class="card-icon">⚡</div>
                <h3>Process & Priority</h3>
                <p>MMCSS scheduling and CPU priority boosting.</p>
                <div class="status">{priorityTweaks.length} tweaks</div>
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
                {#if currentView === "general"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🎮 General Gaming</h2>
                            <p>
                                Essential tweaks for a better gaming experience.
                            </p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(generalTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={generalTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "visuals"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🎨 Visual Optimizations</h2>
                            <p>Remove visual clutter and overlays.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(visualTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={visualTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "priority"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>⚡ Process & Priority</h2>
                            <p>Ensure games get maximum CPU attention.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(priorityTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={priorityTweaks} showHeader={false} />
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .gaming-container {
        height: 100%;
        color: var(--text-color);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: 320px;
        gap: 24px;
        margin-top: 20px;
        overflow-y: auto;
        flex: 1;
        padding: 24px;
        padding-top: 4px;
        justify-content: center; /* Center the grid if there's extra space, or left align. User said 'leave empty space', usually left align is safer for reading order, but center looks nicer. Let's try left align first as 'leave empty space' implies distinct gaps. auto-fill naturally leaves space. */
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
