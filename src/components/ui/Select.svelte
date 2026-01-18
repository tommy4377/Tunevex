<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { ChevronDown } from "lucide-svelte";
    import { slide } from "svelte/transition";
    import { clickOutside } from "$lib/utils/clickOutside";

    export let options: { value: string | number; label: string }[] = [];
    export let value: string | number;
    export let placeholder: string = "Select...";
    export let disabled: boolean = false;
    export let loading: boolean = false;

    const dispatch = createEventDispatcher();
    let isOpen = false;

    $: selectedOption = options.find((o) => o.value === value);

    function toggle() {
        if (!disabled && !loading) {
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
</script>

<div
    class="select-container"
    class:disabled
    use:clickOutside
    on:click_outside={close}
>
    <button
        class="select-trigger"
        on:click|stopPropagation={toggle}
        class:active={isOpen}
        class:loading
    >
        {#if loading}
            <div class="spinner-sm"></div>
            <span class="value-text">Applying...</span>
        {:else}
            <span class="value-text">
                {selectedOption ? selectedOption.label : placeholder}
            </span>
            <span class="icon" class:rotated={isOpen}>
                <ChevronDown size={16} />
            </span>
        {/if}
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
        width: 140px;
        min-width: 120px;
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
        background: #1e1e1e; /* Solid background for visibility */
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 4px;
        z-index: 9999;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.6);
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

    .spinner-sm {
        width: 14px;
        height: 14px;
        border: 2px solid var(--text-muted);
        border-top-color: transparent;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
        margin-right: 8px;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .select-trigger.loading {
        cursor: wait;
        opacity: 0.8;
    }
</style>
