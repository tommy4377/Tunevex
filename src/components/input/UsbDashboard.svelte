<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Filter USB tweaks
    $: usbTweaks = allTweaks.filter(
        (t) =>
            t.id.includes("usb") ||
            t.id.includes("usbstor") ||
            t.id.includes("xhci"),
    );

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
        allTweaks = allTweaks;
    }
</script>

<div class="section-container">
    <div class="section-header">
        <div class="header-text">
            <h2>🔌 USB & Devices</h2>
            <p>Optimize USB controllers and manage polling behavior.</p>
        </div>
        <button
            class="optimize-btn safe"
            on:click={() => applySafeTweaks(usbTweaks)}
        >
            ✅ Apply Safe Tweaks
        </button>
    </div>

    <div class="info-banner">
        <span class="icon">ℹ️</span>
        <p>
            Enabling **MSI Mode** for USB Controllers can significantly reduce
            input variance and latency for connected devices.
        </p>
    </div>

    <div class="tweaks-wrapper">
        <TweakList tweaks={usbTweaks} showHeader={false} />
    </div>
</div>

<style>
    .section-container {
        height: 100%;
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

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, 320px);
        gap: 16px;
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

    .info-banner {
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.2);
        border-radius: var(--radius-sm);
        padding: 12px 16px;
        display: flex;
        gap: 12px;
        align-items: flex-start;
        margin-bottom: 24px;
        color: var(--text-color);
        font-size: 14px;
        line-height: 1.5;
        flex-shrink: 0;
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
    }
</style>
