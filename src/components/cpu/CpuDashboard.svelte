<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import TweakCard from "../TweakCard.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter tweaks by CPU-related categories
    $: schedulingTweaks = allTweaks.filter(
        (t) =>
            t.category === "CpuPerformance" &&
            (t.id.includes("priority") ||
                t.id.includes("mmcss") ||
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
                t.id.includes("superfetch") ||
                t.id.includes("prefetch") ||
                t.id.includes("background_apps")),
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

<div class="cpu-dashboard">
    <h2>⚡ CPU Performance Tweaks</h2>
    <p class="subtitle">
        Optimize CPU scheduling, power management, and memory settings
    </p>

    <div class="sections">
        <!-- Scheduling Section -->
        <section class="tweak-section">
            <h3>🎯 Scheduling & Priority</h3>
            <div class="tweaks-grid">
                {#each schedulingTweaks as tweak}
                    <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
                {/each}
            </div>
        </section>

        <!-- Power Section -->
        <section class="tweak-section">
            <h3>🔌 Power Management</h3>
            <div class="tweaks-grid">
                {#each powerTweaks as tweak}
                    <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
                {/each}
            </div>
        </section>

        <!-- Memory Section -->
        <section class="tweak-section">
            <h3>💾 Memory & Storage</h3>
            <div class="tweaks-grid">
                {#each memoryTweaks as tweak}
                    <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
                {/each}
            </div>
        </section>

        <!-- Timer Section -->
        <section class="tweak-section">
            <h3>⏱️ Timer & Boot Config</h3>
            <div class="tweaks-grid">
                {#each timerTweaks as tweak}
                    <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
                {/each}
            </div>
        </section>
    </div>
</div>

<style>
    .cpu-dashboard {
        height: 100%;
        overflow-y: auto;
        padding: 24px;
        color: var(--text-color);
    }

    h2 {
        margin: 0 0 8px 0;
        font-size: 24px;
        font-weight: 600;
    }

    .subtitle {
        color: var(--text-muted);
        margin: 0 0 24px 0;
        font-size: 14px;
    }

    .sections {
        display: flex;
        flex-direction: column;
        gap: 32px;
    }

    .tweak-section {
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 20px;
    }

    .tweak-section h3 {
        margin: 0 0 16px 0;
        font-size: 16px;
        font-weight: 600;
        color: var(--accent-color);
    }

    .tweaks-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 12px;
    }
</style>
