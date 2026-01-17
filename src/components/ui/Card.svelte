<script lang="ts">
    import type { SvelteComponent } from "svelte";

    export let icon: typeof SvelteComponent | null = null;
    export let title: string;
    export let description: string = "";
    export let status: string = "";
    export let featured: boolean = false;
    export let onclick: (() => void) | null = null;
</script>

<div
    class="card"
    class:featured
    class:clickable={onclick !== null}
    role={onclick ? "button" : undefined}
    tabindex={onclick ? 0 : undefined}
    on:click={onclick}
    on:keydown={(e) => e.key === "Enter" && onclick?.()}
>
    {#if icon}
        <div class="card-icon">
            <svelte:component this={icon} size={24} />
        </div>
    {/if}
    <h3>{title}</h3>
    {#if description}
        <p>{description}</p>
    {/if}
    {#if status}
        <div class="status" class:featured>{status}</div>
    {/if}
    <slot />
</div>

<style>
    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
        padding: 24px;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        position: relative;
        z-index: 1;
        transition: all 0.2s ease;
    }

    .card.clickable {
        cursor: pointer;
    }

    .card.clickable:hover {
        background: rgba(255, 255, 255, 0.06);
        transform: translateY(-2px);
        border-color: var(--accent-color);
        z-index: 10;
    }

    .card.featured {
        background: linear-gradient(
            135deg,
            rgba(59, 130, 246, 0.1) 0%,
            rgba(139, 92, 246, 0.1) 100%
        );
        border-color: rgba(59, 130, 246, 0.4);
    }

    .card.featured:hover {
        border-color: var(--accent-color);
        box-shadow: 0 0 20px rgba(59, 130, 246, 0.2);
    }

    .card-icon {
        color: var(--accent-color);
        margin-bottom: 16px;
    }

    h3 {
        margin: 0 0 8px 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--text-color);
    }

    p {
        margin: 0 0 16px 0;
        color: var(--text-muted);
        font-size: 14px;
        line-height: 1.5;
        flex-grow: 1;
    }

    .status {
        font-size: 12px;
        font-weight: 500;
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.1);
        padding: 6px 12px;
        border-radius: 20px;
    }

    .status.featured {
        background: linear-gradient(135deg, #3b82f6, #8b5cf6);
        color: white;
    }
</style>
