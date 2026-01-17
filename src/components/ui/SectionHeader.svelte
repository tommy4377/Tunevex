<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import { Check } from "lucide-svelte";
    import Button from "./Button.svelte";

    export let icon: typeof SvelteComponent | null = null;
    export let title: string;
    export let description: string = "";
    export let actionLabel: string = "";
    export let onAction: (() => void) | null = null;
</script>

<div class="section-header">
    <div class="header-text">
        <h2>
            {#if icon}
                <span class="icon"
                    ><svelte:component this={icon} size={20} /></span
                >
            {/if}
            {title}
        </h2>
        {#if description}
            <p>{description}</p>
        {/if}
    </div>
    {#if actionLabel && onAction}
        <Button variant="safe" on:click={onAction}>
            <Check size={14} />
            {actionLabel}
        </Button>
    {/if}
    <slot />
</div>

<style>
    .section-header {
        margin-bottom: 24px;
        padding-bottom: 16px;
        border-bottom: 1px solid var(--border-color);
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 16px;
    }

    .header-text h2 {
        display: flex;
        align-items: center;
        gap: 10px;
        font-size: 20px;
        margin: 0 0 8px 0;
        color: var(--text-color);
    }

    .icon {
        color: var(--accent-color);
    }

    .header-text p {
        margin: 0;
        color: var(--text-muted);
        font-size: 14px;
    }
</style>
