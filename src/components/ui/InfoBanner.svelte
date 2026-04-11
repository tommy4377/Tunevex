<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import { Info, AlertTriangle, AlertCircle } from "lucide-svelte";

    export let variant: "info" | "warning" | "danger" = "info";
    export let message: string;
</script>

<div class="banner {variant}">
    <span class="icon">
        {#if variant === "info"}
            <Info size={18} />
        {:else if variant === "warning"}
            <AlertTriangle size={18} />
        {:else}
            <AlertCircle size={18} />
        {/if}
    </span>
    <div class="content">
        <slot>{message}</slot>
    </div>
</div>

<style>
    .banner {
        display: flex;
        align-items: flex-start;
        gap: 12px;
        padding: 12px 16px;
        border-radius: var(--radius-md);
        margin-bottom: 20px;
    }
    .banner.info {
        background: rgba(var(--accent-rgb), 0.10);
        border: 1px solid rgba(var(--accent-rgb), 0.25);
    }
    .banner.info .icon { color: var(--accent-color); }

    .banner.warning {
        background: rgba(251, 191, 36, 0.10);
        border: 1px solid rgba(251, 191, 36, 0.30);
    }
    .banner.warning .icon { color: var(--warning); }

    .banner.danger {
        background: rgba(248, 113, 113, 0.10);
        border: 1px solid rgba(248, 113, 113, 0.30);
    }
    .banner.danger .icon { color: var(--danger); }

    .icon { flex-shrink: 0; margin-top: 2px; }
    .content { font-size: 13px; color: var(--text-color); line-height: 1.5; }
</style>
