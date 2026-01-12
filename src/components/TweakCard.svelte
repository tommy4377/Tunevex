<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import type { Tweak } from "$lib/types";

    export let tweak: Tweak;

    const dispatch = createEventDispatcher();
    let isApplying = false;

    async function handleToggle() {
        if (isApplying) return;
        isApplying = true;
        try {
            if (tweak.enabled) {
                await invoke("undo_tweak", { id: tweak.id });
            } else {
                await invoke("apply_tweak", { id: tweak.id });
            }
            dispatch("toggle");
        } catch (e) {
            console.error("Failed to toggle tweak:", e);
            // Optionally dispatch error event
        } finally {
            isApplying = false;
        }
    }

    function getWarningColor(level: string) {
        switch (level) {
            case "Safe":
                return "#22c55e";
            case "Careful":
                return "#f59e0b";
            case "Dangerous":
                return "#ef4444";
            default:
                return "#64748b";
        }
    }
</script>

<div class="tweak-card" class:enabled={tweak.enabled}>
    <div class="header">
        <span class="name">{tweak.name}</span>
        <span
            class="warning-badge"
            style="background: {getWarningColor(
                tweak.warning_level,
            )}20; color: {getWarningColor(tweak.warning_level)}"
        >
            {tweak.warning_level}
        </span>
    </div>

    <p class="description">{tweak.description}</p>

    {#if isApplying}
        <div class="loading-bar animate-pulse" />
    {/if}

    <div class="footer">
        {#if tweak.requires_restart}
            <span class="restart-badge">🔄 Restart</span>
        {/if}
        <button
            class="apply-btn"
            class:applied={tweak.enabled}
            class:loading={isApplying}
            on:click={handleToggle}
            disabled={isApplying}
        >
            {#if isApplying}
                Applying...
            {:else if tweak.tweak_type === "Action"}
                Run
            {:else}
                {tweak.enabled ? "Enabled" : "Disabled"}
            {/if}
        </button>
    </div>
</div>

<style>
    .tweak-card {
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
        padding: 16px;
        transition: all 0.2s;
    }

    .tweak-card:hover {
        background: rgba(255, 255, 255, 0.04);
        border-color: var(--accent-color);
        transform: translateY(-2px);
    }

    /* ... skipped ... */

    .warning-badge {
        font-size: 11px;
        font-weight: 600;
        padding: 4px 10px;
        border-radius: 20px;
        white-space: nowrap;
    }

    /* ... skipped ... */

    .apply-btn {
        min-width: 80px;
        padding: 8px 16px;
        border-radius: var(--radius-sm);
        border: 1px solid var(--border-color);
        background: transparent;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 13px;
        font-weight: 500;
        transition: all 0.2s;
        margin-left: auto;
    }

    .apply-btn:hover {
        border-color: var(--text-muted);
        color: var(--text-color);
        filter: none;
    }

    .apply-btn.applied {
        background: var(--accent-color);
        border-color: var(--accent-color);
        color: white;
    }

    .apply-btn.applied:hover {
        background: var(--accent-hover);
        filter: none;
    }
    .apply-btn.loading {
        opacity: 0.7;
        cursor: wait;
    }

    .loading-bar {
        height: 2px;
        width: 100%;
        background: var(--accent-color);
        margin-top: 8px;
        border-radius: 2px;
        opacity: 0.8;
    }

    @keyframes pulse {
        0%,
        100% {
            opacity: 1;
        }
        50% {
            opacity: 0.5;
        }
    }

    .animate-pulse {
        animation: pulse 1.5s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }
</style>
