<script lang="ts">
    import TweakCard from "../TweakCard.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";
    import { fade } from "svelte/transition";

    export let tweaks: Tweak[] = [];

    async function toggleTweak(tweak: Tweak) {
        try {
            if (tweak.enabled) {
                await invoke("undo_tweak", { id: tweak.id });
                tweak.enabled = false;
            } else {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
            tweaks = tweaks;
        } catch (e) {
            console.error("Failed to toggle tweak:", e);
        }
    }
</script>

<div class="msi-section" in:fade>
    <div class="header">
        <div class="header-text">
            <h2>Network MSI Mode</h2>
            <p>
                Enable Message Signaled Interrupts (MSI) for lower latency and
                better stability.
            </p>
        </div>
    </div>

    <div class="grid">
        {#each tweaks as tweak (tweak.id)}
            <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
        {/each}
        {#if tweaks.length === 0}
            <div class="empty">No MSI tweaks available</div>
        {/if}
    </div>
</div>

<style>
    .msi-section {
        padding-bottom: 24px;
        display: flex;
        flex-direction: column;
        flex: 1;
        overflow: hidden;
    }

    .header {
        margin-bottom: 24px;
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        flex-shrink: 0;
    }

    .header-text h2 {
        font-size: 24px;
        font-weight: 600;
        margin: 0 0 8px 0;
        color: var(--text-color);
    }

    .header-text p {
        color: var(--text-muted);
        margin: 0;
        font-size: 14px;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 16px;
        overflow-y: auto;
        flex: 1;
        padding: 4px; /* Padding for hover transform clearance */
        padding-bottom: 48px;
    }

    .empty {
        grid-column: 1 / -1;
        text-align: center;
        padding: 40px;
        color: var(--text-muted);
        font-style: italic;
    }
</style>
