<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";

    export let tweaks: Tweak[] = [];

    async function toggleTweak(tweak: Tweak) {
        tweak.enabled = !tweak.enabled;
        tweaks = [...tweaks];
        try {
            if (tweak.enabled) {
                await invoke("apply_tweak", { id: tweak.id });
            } else {
                await invoke("undo_tweak", { id: tweak.id });
            }
        } catch (e) {
            console.error(e);
            tweak.enabled = !tweak.enabled;
            tweaks = [...tweaks];
        }
    }
</script>

<div class="section-container">
    <h2>⚙️ System Latency</h2>
    <p class="section-desc">
        Deep system modifications to reduce input latency.
    </p>

    <div class="tweaks-list">
        {#each tweaks as tweak (tweak.id)}
            <div class="tweak-item">
                <div class="info">
                    <div class="top-row">
                        <span class="name">{tweak.name}</span>
                        {#if tweak.warning_level === "Dangerous"}
                            <span class="badge danger">Dangerous</span>
                        {:else if tweak.warning_level === "Careful"}
                            <span class="badge warning">Careful</span>
                        {/if}
                    </div>
                    <p class="description">{tweak.description}</p>
                </div>

                <button
                    class="toggle-btn"
                    class:on={tweak.enabled}
                    on:click={() => toggleTweak(tweak)}
                >
                    {tweak.enabled ? "Enabled" : "Disabled"}
                </button>
            </div>
        {/each}

        {#if tweaks.length === 0}
            <div class="empty">No system tweaks found.</div>
        {/if}
    </div>
</div>

<style>
    .section-container {
        padding-bottom: 40px;
    }

    h2 {
        font-size: 20px;
        margin-bottom: 8px;
    }

    .section-desc {
        color: var(--text-muted);
        margin-bottom: 24px;
        font-size: 14px;
    }

    .tweaks-list {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .tweak-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 16px;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        transition: background 0.2s;
    }

    .tweak-item:hover {
        background: rgba(255, 255, 255, 0.04);
    }

    .info {
        flex: 1;
        margin-right: 24px;
    }

    .top-row {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 4px;
    }

    .name {
        font-weight: 500;
        font-size: 15px;
    }

    .description {
        margin: 0;
        font-size: 13px;
        color: var(--text-muted);
        line-height: 1.4;
    }

    .badge {
        font-size: 10px;
        padding: 2px 6px;
        border-radius: 4px;
        font-weight: 600;
        text-transform: uppercase;
    }

    .badge.danger {
        background: rgba(239, 68, 68, 0.2);
        color: var(--danger-color);
    }

    .badge.warning {
        background: rgba(245, 158, 11, 0.2);
        color: var(--warning-color);
    }

    .toggle-btn {
        min-width: 80px;
        padding: 8px 16px;
        border-radius: 6px;
        border: 1px solid var(--border-color);
        background: transparent;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 13px;
        font-weight: 500;
        transition: all 0.2s;
    }

    .toggle-btn:hover {
        border-color: var(--text-muted);
        color: var(--text-color);
    }

    .toggle-btn.on {
        background: var(--accent-color);
        border-color: var(--accent-color);
        color: white;
    }

    .toggle-btn.on:hover {
        background: var(--accent-hover);
    }

    .empty {
        text-align: center;
        color: var(--text-muted);
        padding: 20px;
    }
</style>
