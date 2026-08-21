<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Loader2, AlertTriangle } from "lucide-svelte";
    import Badge from "./ui/Badge.svelte";
    import type { Tweak } from "$lib/types";

    export let tweak: Tweak;

    const dispatch = createEventDispatcher();
    let isApplying = false;
    let showRevertConfirm = false;

    function handleRevertClick() {
        if (!tweak.enabled && tweak.warning_level === 'Dangerous') {
            showRevertConfirm = true;
        } else {
            doToggle();
        }
    }

    function cancelRevert() {
        showRevertConfirm = false;
    }

    async function doToggle() {
        if (isApplying) return;
        let dangerousAcknowledgement: string | null = null;
        if (!tweak.enabled && tweak.warning_level === 'Dangerous') {
            const expected = `APPLY ${tweak.id}`;
            dangerousAcknowledgement = prompt(
                `POWER USER CONTROL\n\n${tweak.name}\n\n${tweak.description}\n\n` +
                `Only continue for a specific reason and with a recovery plan. Type ${expected} to apply.`
            );
            if (dangerousAcknowledgement !== expected) return;
        }
        isApplying = true;
        showRevertConfirm = false;
        try {
            if (tweak.enabled) {
                await invoke("undo_tweak", { id: tweak.id });
            } else {
                await invoke("apply_tweak", { id: tweak.id, dangerousAcknowledgement });
            }
            dispatch("toggle");
        } catch (e) {
            console.error("Failed to toggle tweak:", e);
        } finally {
            isApplying = false;
        }
    }

    $: badgeLevel = tweak.warning_level?.toLowerCase() as "safe" | "careful" | "dangerous" | "default";
</script>

<div class="tweak-card" class:enabled={tweak.enabled}>
    <div class="header">
        <span class="name">{tweak.name}</span>
        <Badge level={badgeLevel} />
    </div>

    <p class="description">{tweak.description}</p>

    {#if showRevertConfirm}
        <div class="revert-warning">
            <div class="warning-header">
                <AlertTriangle size={14} />
                <span>Apply Dangerous Tweak</span>
            </div>
            <p class="warning-text">
                This tweak may reduce system stability or security. Review its description before continuing.
            </p>
            <div class="warning-actions">
                <button class="cancel-btn" onclick={cancelRevert} disabled={isApplying}>
                    Cancel
                </button>
                <button class="confirm-btn" onclick={doToggle} disabled={isApplying}>
                    {isApplying ? "Applying..." : "Apply Anyway"}
                </button>
            </div>
        </div>
    {:else}
        <div class="footer">
            <button
                class="apply-btn"
                class:applied={tweak.enabled}
                class:loading={isApplying}
                class:dangerous={tweak.enabled && tweak.warning_level === 'Dangerous'}
                onclick={handleRevertClick}
                disabled={isApplying}
            >
                {#if isApplying}
                    <Loader2 class="spinner" size={16} />
                {:else if tweak.tweak_type === "Action"}
                    Run
                {:else}
                    {tweak.enabled ? "Enabled" : "Disabled"}
                {/if}
            </button>
        </div>
    {/if}
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

    .description {
        font-size: 13px;
        color: var(--text-muted);
        line-height: 1.5;
        margin: 0 0 16px 0;
        flex: 1;
        display: -webkit-box;
        -webkit-line-clamp: 3;
        line-clamp: 3;
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

    .apply-btn.dangerous {
        border-color: rgba(248, 113, 113, 0.4);
    }

    .apply-btn.dangerous:hover {
        border-color: #f87171;
        color: #f87171;
    }

    .apply-btn :global(.spinner) {
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    .revert-warning {
        margin-top: auto;
        padding: 12px;
        background: rgba(248, 113, 113, 0.08);
        border: 1px solid rgba(248, 113, 113, 0.25);
        border-radius: var(--radius-md);
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .warning-header {
        display: flex;
        align-items: center;
        gap: 6px;
        color: #f87171;
        font-size: 12px;
        font-weight: 600;
    }

    .warning-text {
        margin: 0;
        font-size: 12px;
        color: var(--text-secondary);
        line-height: 1.4;
    }

    .warning-actions {
        display: flex;
        gap: 8px;
        justify-content: flex-end;
    }

    .cancel-btn, .confirm-btn {
        padding: 5px 12px;
        border-radius: var(--radius-sm);
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s;
        border: 1px solid;
    }

    .cancel-btn {
        background: transparent;
        border-color: rgba(255, 255, 255, 0.12);
        color: var(--text-secondary);
    }

    .cancel-btn:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.05);
        border-color: rgba(255, 255, 255, 0.2);
    }

    .confirm-btn {
        background: rgba(248, 113, 113, 0.15);
        border-color: rgba(248, 113, 113, 0.3);
        color: #f87171;
    }

    .confirm-btn:hover:not(:disabled) {
        background: rgba(248, 113, 113, 0.25);
    }

    .cancel-btn:disabled, .confirm-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

</style>
