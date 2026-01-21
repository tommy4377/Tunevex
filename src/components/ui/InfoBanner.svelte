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
        padding: 14px 16px;
        border-radius: 8px;
        margin-bottom: 20px;
    }

    .banner.info {
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.3);
    }
    .banner.info .icon {
        color: var(--accent-color);
    }

    .banner.warning {
        background: rgba(245, 158, 11, 0.1);
        border: 1px solid rgba(245, 158, 11, 0.3);
    }
    .banner.warning .icon {
        color: var(--warning-color);
    }

    .banner.danger {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.3);
    }
    .banner.danger .icon {
        color: var(--danger-color);
    }

    .icon {
        flex-shrink: 0;
        margin-top: 2px;
    }

    .content {
        font-size: 13px;
        color: var(--text-color);
        line-height: 1.5;
    }
</style>
