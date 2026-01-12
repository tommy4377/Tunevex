<script lang="ts">
    import type { Tweak } from "$lib/types";
    import MouseDashboard from "./MouseDashboard.svelte";
    import KeyboardDashboard from "./KeyboardDashboard.svelte";
    import UsbDashboard from "./UsbDashboard.svelte";
    import { MousePointer, Keyboard, Usb } from "lucide-svelte";

    export let allTweaks: Tweak[] = [];

    let activeTab: "mouse" | "keyboard" | "usb" = "mouse";
</script>

<div class="input-dashboard">
    <div class="header-section">
        <h1>Input Latency & Optimization</h1>
        <p>
            Optimize your mouse, keyboard, and USB peripherals for competitive
            gaming and lowest latency.
        </p>
    </div>

    <div class="tabs">
        <button
            class:active={activeTab === "mouse"}
            on:click={() => (activeTab = "mouse")}
        >
            <MousePointer size={18} />
            <span>Mouse Tweaks</span>
        </button>
        <button
            class:active={activeTab === "keyboard"}
            on:click={() => (activeTab = "keyboard")}
        >
            <Keyboard size={18} />
            <span>Keyboard Tweaks</span>
        </button>
        <button
            class:active={activeTab === "usb"}
            on:click={() => (activeTab = "usb")}
        >
            <Usb size={18} />
            <span>USB Tweaks</span>
        </button>
    </div>

    <div class="content-container">
        {#if activeTab === "mouse"}
            <MouseDashboard {allTweaks} />
        {:else if activeTab === "keyboard"}
            <KeyboardDashboard {allTweaks} />
        {:else if activeTab === "usb"}
            <UsbDashboard {allTweaks} />
        {/if}
    </div>
</div>

<style>
    .input-dashboard {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        overflow: hidden;
    }

    .header-section {
        margin-bottom: 24px;
    }

    h1 {
        font-size: 24px;
        font-weight: 700;
        margin-bottom: 8px;
        color: var(--text-color);
    }

    p {
        color: var(--text-muted);
        font-size: 14px;
    }

    .tabs {
        display: flex;
        gap: 12px;
        margin-bottom: 24px;
        border-bottom: 1px solid var(--border-color);
        padding-bottom: 0;
    }

    button {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 12px 16px;
        background: transparent;
        border: none;
        border-bottom: 2px solid transparent;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        transition: all 0.2s;
    }

    button:hover {
        color: var(--text-color);
        background: rgba(255, 255, 255, 0.03);
    }

    button.active {
        color: var(--accent-color);
        border-bottom-color: var(--accent-color);
    }

    .content-container {
        flex: 1;
        overflow-y: auto;
        /* Padding bottom to ensure content isn't cut off */
        padding-bottom: 24px;
    }
</style>
