<script lang="ts">
    import { fade } from "svelte/transition";
    import TweakCard from "../TweakCard.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    let activeTab: "general" | "advanced" = "general";

    // Filter GPU-related tweaks
    $: gpuTweaks = allTweaks.filter((t) => t.category === "GpuOptimization");

    // Split tweaks by ID prefix or logic
    $: generalTweaks = gpuTweaks.filter((t) => !t.id.includes("msi"));
    $: advancedTweaks = gpuTweaks.filter((t) => t.id.includes("msi"));

    async function toggleTweak(tweak: Tweak) {
        try {
            if (tweak.enabled) {
                await invoke("undo_tweak", { id: tweak.id });
                tweak.enabled = false;
            } else {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
            allTweaks = allTweaks;
        } catch (e) {
            console.error("Failed to toggle tweak:", e);
        }
    }
</script>

<div class="gpu-dashboard">
    <div class="header-section">
        <h2>🎨 GPU Optimizations</h2>
        <p class="subtitle">
            Hardware GPU scheduling, overlay settings, and driver optimizations
        </p>
    </div>

    <div class="tabs">
        <button
            class:active={activeTab === "general"}
            on:click={() => (activeTab = "general")}
        >
            <span class="icon">🖥️</span>
            <span>General</span>
        </button>
        <button
            class:active={activeTab === "advanced"}
            on:click={() => (activeTab = "advanced")}
        >
            <span class="icon">⚡</span>
            <span>Advanced (MSI)</span>
        </button>
    </div>

    <div class="content">
        {#if activeTab === "general"}
            <div in:fade>
                {#if generalTweaks.length > 0}
                    <div class="tweaks-grid">
                        {#each generalTweaks as tweak (tweak.id)}
                            <TweakCard
                                {tweak}
                                on:toggle={() => toggleTweak(tweak)}
                            />
                        {/each}
                    </div>
                {:else}
                    <div class="empty-state">No general tweaks available</div>
                {/if}
            </div>
        {:else if activeTab === "advanced"}
            <div in:fade>
                <div class="info-banner">
                    <span class="icon">ℹ️</span>
                    <p>
                        MSI (Message Signaled Interrupts) Mode can reduce DPC
                        latency and improve GPU responsiveness. High Priority
                        (3) is recommended for gaming GPUs.
                    </p>
                </div>
                {#if advancedTweaks.length > 0}
                    <div class="tweaks-grid">
                        {#each advancedTweaks as tweak (tweak.id)}
                            <TweakCard
                                {tweak}
                                on:toggle={() => toggleTweak(tweak)}
                            />
                        {/each}
                    </div>
                {:else}
                    <div class="empty-state">No advanced tweaks available</div>
                {/if}
            </div>
        {/if}
    </div>
</div>

<style>
    .gpu-dashboard {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        color: var(--text-color);
        box-sizing: border-box;
        overflow: hidden;
    }

    .header-section {
        margin-bottom: 24px;
        flex-shrink: 0;
    }

    h2 {
        margin: 0 0 8px 0;
        font-size: 24px;
        font-weight: 600;
    }

    .subtitle {
        color: var(--text-muted);
        margin: 0;
        font-size: 14px;
    }

    .tabs {
        display: flex;
        gap: 12px;
        margin-bottom: 24px;
        border-bottom: 1px solid var(--border-color);
        padding-bottom: 0;
        flex-shrink: 0;
    }

    .tabs button {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 12px 16px;
        background: transparent;
        border: none;
        border-bottom: 2px solid transparent;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        transition: all 0.2s;
    }

    .tabs button:hover {
        color: var(--text-color);
        background: rgba(255, 255, 255, 0.03);
    }

    .tabs button.active {
        color: var(--accent-color);
        border-bottom-color: var(--accent-color);
    }

    .content {
        flex: 1;
        overflow-y: auto;
        padding-bottom: 20px;
    }

    .tweaks-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 12px;
    }

    .info-banner {
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.2);
        border-radius: 8px;
        padding: 12px 16px;
        display: flex;
        gap: 12px;
        align-items: flex-start;
        margin-bottom: 20px;
        color: var(--text-color);
        font-size: 14px;
        line-height: 1.5;
    }

    .empty-state {
        text-align: center;
        padding: 40px;
        color: var(--text-muted);
        font-style: italic;
    }
</style>
