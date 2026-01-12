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
            {tweak.enabled ? "Undo" : "Apply"}
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
        background: var(--accent-color);
        color: white;
        border: none;
        padding: 8px 16px;
        border-radius: var(--radius-sm);
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        margin-left: auto;
    }

    .apply-btn:hover {
        filter: brightness(1.1);
    }

    .apply-btn.applied {
        background: #ef4444; /* Red for undo/remove */
    }

    .apply-btn.applied:hover {
        background: #dc2626;
    }
</style>
