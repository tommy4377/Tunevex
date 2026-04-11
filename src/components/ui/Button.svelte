<script lang="ts">
    export let variant: "primary" | "accent" | "safe" | "ghost" | "danger" =
        "primary";
    export let disabled: boolean = false;
    export let loading: boolean = false;
    export let onclick: (() => void) | undefined = undefined;
</script>

<button class="btn {variant}" class:loading {disabled} onclick={onclick}>
    {#if loading}
        <span class="spinner"></span>
    {/if}
    <slot />
</button>

<style>
    .btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        padding: 8px 16px;
        border-radius: var(--radius-sm);
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.18s ease;
        border: none;
        white-space: nowrap;
    }
    .btn:disabled { opacity: 0.45; cursor: not-allowed; }

    .btn.primary { background: var(--accent-color); color: white; }
    .btn.primary:hover:not(:disabled) { background: var(--accent-hover); }

    .btn.accent {
        background: var(--toggle-on-bg);
        border: 1px solid var(--toggle-on-border);
        color: var(--toggle-on-color);
    }
    .btn.accent:hover:not(:disabled) { background: rgba(129, 140, 248, 0.28); }

    .btn.safe {
        background: var(--btn-safe-bg);
        border: 1px solid var(--btn-safe-border);
        color: var(--btn-safe-color);
    }
    .btn.safe:hover:not(:disabled) { background: var(--btn-safe-hover-bg); }

    .btn.ghost {
        background: transparent;
        color: var(--text-muted);
        border: 1px solid var(--border-color);
    }
    .btn.ghost:hover:not(:disabled) {
        color: var(--text-color);
        border-color: rgba(255, 255, 255, 0.2);
    }

    .btn.danger { background: var(--danger); color: white; }
    .btn.danger:hover:not(:disabled) { background: #ef4444; }

    .spinner {
        width: 14px; height: 14px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top-color: white;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }
    @keyframes spin { to { transform: rotate(360deg); } }
    .loading { pointer-events: none; }
</style>