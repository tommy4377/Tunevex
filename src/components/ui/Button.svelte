<script lang="ts">
    export let variant: "primary" | "safe" | "ghost" | "danger" = "primary";
    export let disabled: boolean = false;
    export let loading: boolean = false;
</script>

<button class="btn {variant}" class:loading {disabled} on:click>
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
        transition: all 0.2s;
        border: none;
        white-space: nowrap;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    /* Primary - Accent Blue */
    .btn.primary {
        background: var(--accent-color);
        color: white;
    }
    .btn.primary:hover:not(:disabled) {
        background: var(--accent-hover);
    }

    /* Safe - Green */
    .btn.safe {
        background: var(--success-color);
        color: white;
    }
    .btn.safe:hover:not(:disabled) {
        background: #059669;
    }

    /* Ghost - Transparent */
    .btn.ghost {
        background: transparent;
        color: var(--text-muted);
        border: 1px solid var(--border-color);
    }
    .btn.ghost:hover:not(:disabled) {
        color: var(--text-color);
        border-color: var(--text-muted);
    }

    /* Danger - Red */
    .btn.danger {
        background: var(--danger-color);
        color: white;
    }
    .btn.danger:hover:not(:disabled) {
        background: #dc2626;
    }

    /* Loading Spinner */
    .spinner {
        width: 14px;
        height: 14px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top-color: white;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .loading {
        pointer-events: none;
    }
</style>
