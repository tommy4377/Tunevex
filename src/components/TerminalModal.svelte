<script lang="ts">
    import { createEventDispatcher, afterUpdate } from "svelte";
    import { X } from "lucide-svelte";

    export let show = false;
    export let title = "Terminal Output";
    export let logs: string[] = [];
    export let processing = false;

    const dispatch = createEventDispatcher();
    let scrollContainer: HTMLDivElement;

    function close() {
        dispatch("close");
    }

    // Auto-scroll to bottom
    afterUpdate(() => {
        if (show && scrollContainer) {
            scrollContainer.scrollTop = scrollContainer.scrollHeight;
        }
    });
</script>

{#if show}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="modal-backdrop"
        on:click|self={close}
        on:keydown={(e) => e.key === "Escape" && close()}
    >
        <div class="modal-content">
            <div class="header">
                <h3>{title}</h3>
                <button class="close-btn" on:click={close}>
                    <X size={18} />
                </button>
            </div>

            <div class="terminal" bind:this={scrollContainer}>
                {#if logs.length === 0}
                    <div class="placeholder">Waiting for output...</div>
                {/if}
                {#each logs as line}
                    <div class="line">{line}</div>
                {/each}
                {#if processing}
                    <div class="line processing">_</div>
                {/if}
            </div>

            <div class="footer">
                {#if processing}
                    <span class="status processing">Processing...</span>
                {:else}
                    <span class="status done">Done</span>
                    <button class="action-btn" on:click={close}>Close</button>
                {/if}
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
        backdrop-filter: blur(4px);
    }

    .modal-content {
        width: 90%;
        max-width: 800px;
        height: 80vh;
        background: rgba(
            30,
            41,
            59,
            0.95
        ); /* Nearly opaque for terminal readability */
        backdrop-filter: blur(16px);
        border-radius: var(--radius);
        border: 1px solid var(--border-color);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
    }

    .header {
        padding: 16px 24px;
        border-bottom: 1px solid var(--border-color);
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: rgba(0, 0, 0, 0.2);
    }

    h3 {
        margin: 0;
        font-size: 16px;
        font-weight: 600;
        color: var(--text-color);
    }

    .close-btn {
        background: transparent;
        border: none;
        color: var(--text-muted);
        cursor: pointer;
        padding: 6px;
        border-radius: 50%; /* Rounded button */
        transition: all 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .close-btn:hover {
        color: var(--text-color);
        background: rgba(255, 255, 255, 0.1);
    }

    .terminal {
        flex: 1;
        background: var(--layer-nav);
        backdrop-filter: blur(40px);
        border: var(--border-glass);
        padding: 24px;
        overflow-y: auto;
        font-family: "JetBrains Mono", "Courier New", monospace;
        font-size: 13px;
        line-height: 1.6;
        color: #cccccc;
        white-space: pre-wrap;
        word-break: break-all;
    }

    .footer {
        padding: 16px 24px;
        border-top: 1px solid var(--border-color);
        background: rgba(0, 0, 0, 0.2);
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .action-btn {
        padding: 8px 20px;
        background: var(--accent-color);
        color: white;
        border: none;
        border-radius: 10px; /* Rounded button */
        cursor: pointer;
        font-weight: 600;
        font-size: 13px;
        transition: background 0.2s;
    }

    .action-btn:hover {
        background: var(--accent-hover);
    }
</style>
