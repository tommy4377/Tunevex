<script lang="ts">
    import { fade } from "svelte/transition";
    import MouseDashboard from "./MouseDashboard.svelte";
    import KeyboardDashboard from "./KeyboardDashboard.svelte";
    import UsbDashboard from "./UsbDashboard.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Navigation state
    let currentView: "dashboard" | "mouse" | "keyboard" | "usb" = "dashboard";

    // Filters for count badges
    $: mouseTweaks = allTweaks.filter((t) => t.category === "MouseInput");
    $: keyboardTweaks = allTweaks.filter((t) => t.category === "Input"); // Legacy or new? Let's check logic
    // Actually, Input category might be split. Let's use the same logic as the sub-components.
    // MouseDashboard uses category "MouseInput"
    // KeyboardDashboard uses category "Input" and filters for keyboard-like IDs or just all "Input" (if not mouse/usb)
    // UsbDashboard uses filtering (likely "usb")

    // Correction based on sub-components:
    // Mouse filtering matching MouseDashboard logic
    $: mouseCount = allTweaks.filter(
        (t) => t.category === "MouseInput" && t.id.includes("mouse"),
    ).length;

    // Keyboard filtering matching KeyboardDashboard logic
    $: keyboardCount = allTweaks.filter(
        (t) =>
            (t.category === "Input" || t.category === "MouseInput") &&
            (t.id.includes("keyboard") || t.id.includes("filter")),
    ).length;

    // USB
    $: usbCount = allTweaks.filter((t) => t.id.includes("usb")).length;
</script>

<div class="input-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- Mouse Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "mouse")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "mouse")}
            >
                <div class="card-icon">🖱️</div>
                <h3>Mouse Optimization</h3>
                <p>Reduce input lag and adjust acceleration curves.</p>
                <div class="status">{mouseCount} tweaks</div>
            </div>

            <!-- Keyboard Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "keyboard")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "keyboard")}
            >
                <div class="card-icon">⌨️</div>
                <h3>Keyboard Response</h3>
                <p>Data queue sizes and repeat rates.</p>
                <div class="status">{keyboardCount} tweaks</div>
            </div>

            <!-- USB Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "usb")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "usb")}
            >
                <div class="card-icon">🔌</div>
                <h3>USB & Devices</h3>
                <p>MSI Mode for USB controllers and pollution reduction.</p>
                <div class="status">{usbCount} tweaks</div>
            </div>
        </div>
    {:else}
        <div class="detail-view" in:fade>
            <button
                class="back-btn"
                on:click={() => (currentView = "dashboard")}
            >
                ← Back to Dashboard
            </button>

            <div class="section-content">
                {#if currentView === "mouse"}
                    <MouseDashboard {allTweaks} />
                {:else if currentView === "keyboard"}
                    <KeyboardDashboard {allTweaks} />
                {:else if currentView === "usb"}
                    <UsbDashboard {allTweaks} />
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .input-container {
        height: 100%;
        color: var(--text-color);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 24px;
        margin-top: 20px;
        overflow-y: auto;
        flex: 1;
        padding: 24px;
        padding-top: 4px;
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
        padding: 24px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        position: relative;
        z-index: 1;
    }

    .card:hover {
        background: rgba(255, 255, 255, 0.06);
        transform: translateY(-2px);
        border-color: var(--accent-color);
        z-index: 10;
        position: relative;
    }

    .card-icon {
        font-size: 32px;
        margin-bottom: 16px;
    }

    h3 {
        margin: 0 0 8px 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--text-color);
    }

    p {
        margin: 0 0 24px 0;
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

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        box-sizing: border-box;
    }

    .section-content {
        flex: 1;
        overflow: hidden;
    }

    .back-btn {
        align-self: flex-start;
        background: none;
        border: none;
        color: var(--text-muted);
        font-size: 14px;
        cursor: pointer;
        padding: 8px 0;
        margin-bottom: 16px;
        transition: color 0.2s;
    }

    .back-btn:hover {
        color: var(--text-color);
    }
</style>
