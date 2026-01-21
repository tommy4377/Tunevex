<script lang="ts">
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { Globe, Check } from "lucide-svelte";

    export let tweaks: Tweak[] = [];

    async function applyAllSafe() {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                await invoke("apply_tweak", { id: tweak.id });
                tweak.enabled = true;
            }
        }
        tweaks = tweaks; // Trigger reactivity
    }
</script>

<div class="section-container">
    <div class="header">
        <h2><Globe size={20} /> Microsoft Edge</h2>
        <p>
            Disable Edge sidebar, first run experience, sync, telemetry, and
            auto-start.
        </p>
        <button class="optimize-btn safe" on:click={applyAllSafe}>
            <Check size={14} />
            Apply Safe Tweaks
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
        border-bottom: var(--border-glass);
    }
    h2 {
        font-size: 20px;
        margin-bottom: 8px;
        display: flex;
        align-items: center;
        gap: 10px;
        color: var(--text-color);
    }
    h2 :global(svg) {
        color: var(--accent-color);
    }
    p {
        color: var(--text-muted);
        font-size: 14px;
        margin-bottom: 16px;
    }
    .optimize-btn {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        border: none;
        padding: 8px 16px;
        border-radius: 8px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
    }
    .optimize-btn.safe {
        background: var(--btn-safe-bg);
        border: 1px solid var(--btn-safe-border);
        color: var(--btn-safe-color);
    }
    .optimize-btn.safe:hover {
        background: var(--btn-safe-hover-bg);
    }
    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }
</style>
