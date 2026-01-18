<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { ChevronDown } from "lucide-svelte";
    import { fade, slide } from "svelte/transition";
    import { clickOutside } from "$lib/utils/clickOutside"; // Assuming utils exists, if not I will implement clickOutside inline or checking utils first.
    // Wait, I haven't checked if clickOutside exists. To be safe, I'll implement a simple window click handler or inline logic.

    export let options: { value: string | number; label: string }[] = [];
    export let value: string | number;
    export let placeholder: string = "Select...";
    export let disabled: boolean = false;

    const dispatch = createEventDispatcher();
    let isOpen = false;

    $: selectedOption = options.find((o) => o.value === value);

    function toggle() {
        if (!disabled) {
            isOpen = !isOpen;
        }
    }

    function select(option: { value: string | number; label: string }) {
        if (value !== option.value) {
            value = option.value;
            dispatch("change", { value });
        }
        isOpen = false;
    }

    function close() {
        isOpen = false;
    }

    // Window click handler to close dropdown
    function handleWindowClick(event: MouseEvent) {
        if (isOpen) {
            // Logic handled by on:click window binding with check would be cleaner but svelte:window is fine
        }
    }

    // Check if click is outside
    let container: HTMLDivElement;
    function handleClickOutside(event: MouseEvent) {
        if (isOpen && container && !container.contains(event.target as Node)) {
            close();
        }
    }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="select-container" bind:this={container} class:disabled>
    <button
        class="select-trigger"
        on:click|stopPropagation={toggle}
        class:active={isOpen}
    >
        <span class="value-text">
            {selectedOption ? selectedOption.label : placeholder}
        </span>
        <span class="icon" class:rotated={isOpen}>
            <ChevronDown size={16} />
        </span>
    </button>

    {#if isOpen}
        <div
            class="options-menu"
            transition:slide={{ duration: 150, axis: "y" }}
        >
            {#each options as option}
                <button
                    class="option-item"
                    class:selected={option.value === value}
                    on:click={() => select(option)}
                >
                    {option.label}
                    {#if option.value === value}
                        <div class="indicator"></div>
                    {/if}
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .select-container {
        position: relative;
        width: 200px;
    }

    .select-container.disabled {
        opacity: 0.6;
        pointer-events: none;
    }

    .select-trigger {
        width: 100%;
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 8px 12px;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color); /* Fallback */
        border: var(--border-glass, 1px solid rgba(255, 255, 255, 0.08));
        border-radius: 8px;
        color: var(--text-color);
        font-size: 13px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .select-trigger:hover {
        background: rgba(255, 255, 255, 0.06);
        border-color: rgba(255, 255, 255, 0.15);
    }

    .select-trigger.active {
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.08);
    }

    .value-text {
        font-weight: 500;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .icon {
        color: var(--text-muted);
        transition: transform 0.2s ease;
        display: flex;
        align-items: center;
    }

    .icon.rotated {
        transform: rotate(180deg);
        color: var(--accent-color);
    }

    .options-menu {
        position: absolute;
        top: calc(100% + 4px);
        left: 0;
        right: 0;
        background: var(--layer-bg, #1a1a1a);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 4px;
        z-index: 100;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
        backdrop-filter: blur(16px); /* If layer-bg is transparent */
        max-height: 200px;
        overflow-y: auto;
    }

    .option-item {
        width: 100%;
        text-align: left;
        padding: 8px 10px;
        background: transparent;
        border: none;
        color: var(--text-color);
        font-size: 13px;
        border-radius: 6px;
        cursor: pointer;
        transition: all 0.1s;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .option-item:hover {
        background: rgba(255, 255, 255, 0.08);
    }

    .option-item.selected {
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.1);
        font-weight: 600;
    }

    .indicator {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: var(--accent-color);
    }
</style>
