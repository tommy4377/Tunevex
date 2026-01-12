<script lang="ts">
    import TweakList from "../TweakList.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";
    export let tweaks: Tweak[] = [];

    async function applySafe() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    await invoke("apply_tweak", { id: tweak.id });
                    tweak.enabled = true;
                } catch (e) {
                    console.error(e);
                }
            }
        }
        tweaks = tweaks;
    }
</script>

<div class="section-container">
    <div class="header">
        <h2>🛡️ SmartScreen Filter</h2>
        <p>
            Control SmartScreen protection for apps, Microsoft Edge, and Store
            apps.
        </p>
        <div class="warning-banner">
            ⚠️ Disabling SmartScreen reduces protection against malware and
            phishing
        </div>
        <button class="optimize-btn safe" on:click={applySafe}>
            ✅ Apply Safe Tweaks
        </button>
    </div>
    <div class="tweaks-wrapper">
        <TweakList {tweaks} showHeader={false} />
    </div>
</div>

<style>
    .section-container {
        display: flex;
        flex-direction: column;
        height: 100%;
    }
    .header {
        margin-bottom: 24px;
        padding-bottom: 16px;
        border-bottom: 1px solid var(--border-color);
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
    h2 {
        font-size: 20px;
        margin-bottom: 8px;
    }
    p {
        color: var(--text-muted);
        font-size: 14px;
        margin: 0 0 16px 0;
    }
    .warning-banner {
        background: rgba(239, 68, 68, 0.1);
        color: #ef4444;
        padding: 12px;
        border-radius: 8px;
        font-size: 13px;
        margin-bottom: 16px;
        align-self: flex-start;
    }
    .optimize-btn {
        border: none;
        padding: 8px 16px;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        color: white;
        white-space: nowrap;
        margin: 0;
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
</style>
