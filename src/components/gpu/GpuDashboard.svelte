<script lang="ts">
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/core";
    import TweakList from "../TweakList.svelte";
    import MsiSection from "../network/MsiSection.svelte"; // Reusing the generic MSI section logic/layout
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Navigation state
    let currentView: "dashboard" | "general" | "msi" = "dashboard";

    // --- Filters ---

    $: schedulingTweaks = allTweaks.filter(
        (t) =>
            t.category === "GpuOptimization" &&
            (t.id.includes("hags") ||
                t.id.includes("preemption") ||
                t.id.includes("mpo") ||
                t.id.includes("priority")),
    );

    $: msiTweaks = allTweaks.filter(
        (t) => t.category === "GpuOptimization" && t.id.includes("msi"),
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

<div class="gpu-container">
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
                <div class="card-icon">🚀</div>
                <h3>General Optimization</h3>
                <p>HAGS, Game Mode, and Priority adjustments.</p>
                <div class="status">{schedulingTweaks.length} tweaks</div>
            </div>

            <!-- MSI Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "msi")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "msi")}
            >
                <div class="card-icon">⚡</div>
                <h3>MSI Mode</h3>
                <p>Message Signaled Interrupts for GPU.</p>
                <div class="status">{msiTweaks.length} tweaks</div>
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
                            <h2>🚀 General Optimization</h2>
                            <p>Core GPU scheduling and system settings.</p>
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
                {:else if currentView === "msi"}
                    <!-- Reusing MsiSection but overriding title isn't easy via props if it doesn't support it 
                         Looking at MsiSection source, it has hardcoded title "Network MSI Mode" if not generic.
                         Let's check MsiSection source. It uses: <h2>⚡ Network MSI Mode</h2>
                         We should probably make MsiSection generic or just use TweakList here with a custom header.
                         Let's stick to the pattern: custom header + TweakList.
                    -->
                    <div class="section-header">
                        <div class="header-text">
                            <h2>⚡ GPU MSI Mode</h2>
                            <p>
                                Enable Message Signaled Interrupts for lower
                                latency.
                            </p>
                        </div>
                        <div class="info-banner">
                            <span class="icon">ℹ️</span>
                            <span
                                >High Priority is recommended for dedicated
                                GPUs.</span
                            >
                        </div>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={msiTweaks} showHeader={false} />
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .gpu-container {
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
        margin: 0 0 8px 0;
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

    .info-banner {
        margin-top: 8px;
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.2);
        border-radius: var(--radius-sm);
        padding: 8px 12px;
        display: flex;
        gap: 8px;
        align-items: center;
        color: var(--text-muted);
        font-size: 13px;
    }
</style>
