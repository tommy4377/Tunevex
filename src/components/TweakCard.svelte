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
        <div class="loading-bar animate-pulse"></div>
    {/if}

    <div class="footer">
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
        background: var(--layer-card);
        backdrop-filter: blur(12px) saturate(150%);
        -webkit-backdrop-filter: blur(12px) saturate(150%);
        border: var(--border-glass);
        border-radius: var(--radius-lg);
        padding: 16px;
        transition: all 0.25s ease;
        display: flex;
        flex-direction: column;
        position: relative;
        z-index: 1;
    }

    .tweak-card:hover {
        background: var(--layer-hover);
        border-color: var(--toggle-on-border);
        transform: translateY(-2px);
        z-index: 10;
        box-shadow: 0 8px 24px -8px rgba(0, 0, 0, 0.3);
    }

    .tweak-card.enabled {
        border-color: var(--toggle-on-border);
        /* background: var(--toggle-on-bg); Removed per user request */
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 12px;
        margin-bottom: 12px;
    }

    .name {
        font-size: 15px;
        font-weight: 600;
        color: var(--text-color);
        line-height: 1.3;
        flex: 1;
    }

    .warning-badge {
        font-size: 11px;
        font-weight: 600;
        padding: 4px 10px;
        border-radius: 20px;
        white-space: nowrap;
        flex-shrink: 0;
    }

    .description {
        font-size: 13px;
        color: var(--text-muted);
        line-height: 1.5;
        margin: 0 0 16px 0;
        flex: 1;
        display: -webkit-box;
        -webkit-line-clamp: 3;
        -webkit-box-orient: vertical;
        overflow: hidden;
    }

    .footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 12px;
        margin-top: auto;
    }

    .apply-btn {
        min-width: 90px;
        padding: 8px 16px;
        border-radius: 10px;
        border: 1px solid var(--toggle-off-border);
        background: var(--toggle-off-bg);
        color: var(--toggle-off-color);
        cursor: pointer;
        font-size: 13px;
        font-weight: 600;
        transition: all 0.25s ease;
        margin-left: auto;
    }

    .apply-btn:hover {
        border-color: rgba(255, 255, 255, 0.2);
        color: var(--text-color);
        background: rgba(255, 255, 255, 0.08);
    }

    /* Applied/Enabled State - Azure Glow */
    .apply-btn.applied {
        background: var(--toggle-on-bg);
        border-color: var(--toggle-on-border);
        color: var(--toggle-on-color);
        box-shadow: var(--toggle-on-glow);
    }

    .apply-btn.applied:hover {
        background: rgba(96, 205, 255, 0.3);
        box-shadow: 0 0 16px rgba(96, 205, 255, 0.4);
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
