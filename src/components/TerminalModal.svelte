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
    <div class="modal-backdrop" on:click|self={close}>
        <div class="modal">
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

    .modal {
        width: 80%;
        max-width: 800px;
        height: 70%;
        background: #1e1e1e;
        border: 1px solid var(--border-color);
        border-radius: 20px; /* High rounding */
        display: flex;
        flex-direction: column;
        box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
        overflow: hidden; /* Ensure rounded corners clip content */
    }

    .header {
        padding: 16px 24px;
        border-bottom: 1px solid var(--border-color);
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: #252526;
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
        background: #0c0c0c;
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
        background: #252526;
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
