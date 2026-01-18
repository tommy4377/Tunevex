<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import { createEventDispatcher } from "svelte";

    // Props
    export let icon: typeof SvelteComponent | null = null;
    export let title: string;
    export let description: string = "";
    export let status: string = "";
    export let featured: boolean = false;
    export let onclick: (() => void) | null = null;

    // Events (for compatibility)
    const dispatch = createEventDispatcher();

    function handleClick() {
        if (onclick) onclick();
        dispatch("click");
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<div
    class="card"
    class:featured
    class:clickable={onclick !== null}
    role={onclick ? "button" : undefined}
    tabindex={onclick ? 0 : undefined}
    on:click={handleClick}
    on:keydown={(e) => e.key === "Enter" && handleClick()}
>
    {#if icon}
        <div class="card-icon">
            <svelte:component this={icon} size={20} />
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
        box-sizing: border-box;
        background: var(--layer-card);
        backdrop-filter: blur(12px) saturate(150%); /* Glassmorphism */
        -webkit-backdrop-filter: blur(12px) saturate(150%);
        border: var(--border-glass);
        border-radius: 12px; /* Slightly tighter radius for compact cards */
        padding: 16px; /* REDUCED from 24px */
        position: relative;
        z-index: 1;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        height: 100%;
        width: 100%;
        overflow: hidden;
    }

    /* Gradient overlay for sheen */
    .card::before {
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(
            90deg,
            transparent 0%,
            rgba(255, 255, 255, 0.1) 20%,
            rgba(255, 255, 255, 0.2) 50%,
            rgba(255, 255, 255, 0.1) 80%,
            transparent 100%
        );
        opacity: 0.5;
        pointer-events: none;
    }

    .clickable {
        cursor: pointer;
    }

    .card:hover {
        background: var(--layer-hover);
        border-color: var(--accent);
        transform: translateY(-4px);
        box-shadow: 0 12px 24px -8px rgba(0, 0, 0, 0.3);
        z-index: 10;
    }

    .card-icon {
        color: var(--accent-color);
        margin-bottom: 12px; /* REDUCED from 16px */
        filter: drop-shadow(0 0 8px rgba(59, 130, 246, 0.3));
        transition: transform 0.3s ease;
    }

    .card:hover .card-icon {
        transform: scale(1.1);
        color: #60a5fa; /* Lighter blue on hover */
    }

    h3 {
        margin: 0 0 6px 0; /* REDUCED from 8px */
        font-size: 15px; /* REDUCED from 18px */
        font-weight: 600;
        color: var(--text-color);
        width: 100%;
    }

    p {
        margin: 0 0 16px 0; /* REDUCED from 24px */
        color: var(--text-muted);
        font-size: 13px; /* REDUCED from 14px */
        line-height: 1.5;
        flex-grow: 1;
        width: 100%;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
        overflow: hidden;
    }

    .status {
        font-size: 10px; /* REDUCED from 11px */
        font-weight: 600;
        letter-spacing: 0.025em;
        text-transform: uppercase;
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.15);
        padding: 3px 8px; /* REDUCED from 4px 10px */
        border-radius: 999px;
        display: inline-flex;
        align-items: center;
        border: 1px solid rgba(59, 130, 246, 0.2);
    }
</style>
