<script lang="ts">
    import { fade } from "svelte/transition";
    import TweakList from "../TweakList.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Navigation state
    let currentView: "dashboard" | "maintenance" | "services" | "system" =
        "dashboard";

    // --- Filters ---

    // Maintenance (Cleaning, Restore Points, etc)
    $: maintenanceTweaks = allTweaks.filter(
        (t) =>
            t.category === "System" &&
            (t.id.includes("maintenance") ||
                t.id.includes("restore") ||
                t.id.includes("cleanup") ||
                t.id.includes("bso_d") || // bsod auto restart usually
                t.id.includes("restart")),
    );

    // Services (Windows Update, Fax, etc)
    $: servicesTweaks = allTweaks.filter(
        (t) =>
            t.category === "System" &&
            (t.id.includes("service") ||
                t.id.includes("update") ||
                t.id.includes("fax") ||
                t.id.includes("print") ||
                t.id.includes("bloat")),
    );

    // System / Hardware / MSI (General System Tweaks)
    $: systemTweaks = allTweaks.filter(
        (t) =>
            t.category === "System" &&
            !maintenanceTweaks.includes(t) &&
            !servicesTweaks.includes(t),
    );

    // Helper for "Apply Safe Tweaks"
    async function applySafeTweaks(tweaks: Tweak[]) {
        for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
            if (!tweak.enabled) {
                try {
                    await invoke("apply_tweak", { id: tweak.id });
                    tweak.enabled = true;
                } catch (e) {
                    console.error(`Failed to apply tweak ${tweak.id}:`, e);
                }
            }
        }
        allTweaks = allTweaks; // Trigger updates
    }
</script>

<div class="system-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- Maintenance Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "maintenance")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "maintenance")}
            >
                <div class="card-icon">🧹</div>
                <h3>Maintenance</h3>
                <p>
                    System cleanup, restore points, and auto-restart settings.
                </p>
                <div class="status">{maintenanceTweaks.length} tweaks</div>
            </div>

            <!-- Services Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "services")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "services")}
            >
                <div class="card-icon">⚙️</div>
                <h3>Windows Services</h3>
                <p>Optimize background services and disable bloat.</p>
                <div class="status">{servicesTweaks.length} tweaks</div>
            </div>

            <!-- System / MSI Card -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "system")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "system")}
            >
                <div class="card-icon">💻</div>
                <h3>System & Hardware</h3>
                <p>General system tweaks and hardware configurations.</p>
                <div class="status">{systemTweaks.length} tweaks</div>
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
                {#if currentView === "maintenance"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🧹 Maintenance</h2>
                            <p>Keep your system clean and stable.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(maintenanceTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList
                            tweaks={maintenanceTweaks}
                            showHeader={false}
                        />
                    </div>
                {:else if currentView === "services"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>⚙️ Windows Services</h2>
                            <p>
                                Manage background services for better
                                performance.
                            </p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(servicesTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={servicesTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "system"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>💻 System & Hardware</h2>
                            <p>Core system behavior and hardware interrupts.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(systemTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={systemTweaks} showHeader={false} />
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .system-container {
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
    }

    .section-content {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .section-header {
        margin-bottom: 24px;
        padding-bottom: 16px;
        border-bottom: 1px solid var(--border-color);
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .header-text h2 {
        font-size: 20px;
        margin: 0 0 8px 0;
        color: var(--text-color);
    }
    .header-text p {
        margin: 0 0 16px 0;
        color: var(--text-muted);
        font-size: 14px;
    }

    .optimize-btn {
        border: none;
        padding: 8px 16px;
        border-radius: var(--radius-sm);
        font-weight: 500;
        cursor: pointer;
        color: white;
        white-space: nowrap;
    }
    .optimize-btn.safe {
        background: #10b981;
    }
    .optimize-btn.safe:hover {
        background: #059669;
    }

    .tweaks-wrapper {
        flex: 1;
        overflow: hidden;
        display: flex;
        flex-direction: column;
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
