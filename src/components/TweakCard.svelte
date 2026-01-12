<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { Tweak } from "$lib/types";

    export let tweak: Tweak;

    const dispatch = createEventDispatcher();

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

    <div class="footer">
        {#if tweak.requires_restart}
            <span class="restart-badge">🔄 Restart</span>
        {/if}
        <button
            class="apply-btn"
            class:applied={tweak.enabled}
            on:click={() => dispatch("toggle")}
        >
            {tweak.enabled ? "Enabled" : "Disabled"}
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
</style>
