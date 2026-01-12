<script lang="ts">
    import TweakCard from "../TweakCard.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    $: usbTweaks = allTweaks.filter((t) => t.id.includes("usb")); // Simple filter for now

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

<div class="section">
    <div class="info">
        <h3>🔌 USB Optimization</h3>
        <p>
            Enable Message Signaled Interrupts (MSI) for USB controllers to
            reduce input latency.
        </p>
    </div>

    <div class="grid">
        {#each usbTweaks as tweak}
            <TweakCard {tweak} on:toggle={() => toggleTweak(tweak)} />
        {/each}
        {#if usbTweaks.length === 0}
            <div class="empty">No USB tweaks available</div>
        {/if}
    </div>
</div>

<style>
    .section {
        padding-top: 8px; /* Tab clearance */
    }

    h3 {
        margin: 0 0 8px 0;
        color: var(--text-color);
        font-size: 18px;
    }

    p {
        color: var(--text-muted);
        margin: 0 0 24px 0;
        font-size: 14px;
        line-height: 1.5;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 16px;
    }

    .empty {
        text-align: center;
        padding: 32px;
        color: var(--text-muted);
        font-style: italic;
    }
</style>
