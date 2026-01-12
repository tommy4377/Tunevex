<script lang="ts">
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/core";
    import TweakList from "../TweakList.svelte";
    import type { Tweak } from "$lib/types";

    export let allTweaks: Tweak[] = [];

    // Sub-routes
    let currentView:
        | "dashboard"
        | "windows"
        | "apps"
        | "settings"
        | "advertising"
        | "policies"
        | "maintenance" = "dashboard";

    // --- Filters ---

    $: windowsTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_disable_telemetry" ||
                t.id === "priv_disable_ceip" ||
                t.id === "priv_disable_wer" ||
                t.id === "priv_disable_input_telemetry" ||
                t.id === "priv_disable_telemetry_tasks"),
    );

    $: appTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_nvidia_telemetry" ||
                t.id === "priv_office_telemetry" ||
                t.id === "priv_vs_telemetry" ||
                t.id === "priv_vscode_telemetry" ||
                t.id === "priv_dotnet_telemetry" ||
                t.id === "priv_powershell_telemetry" ||
                t.id === "priv_chrome_telemetry" ||
                t.id === "priv_firefox_telemetry"),
    );

    $: settingsTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id.includes("activity") ||
                t.id.includes("timeline") ||
                t.id.includes("location") ||
                t.id.includes("speech") ||
                t.id.includes("sync") ||
                t.id === "priv_app_permissions" ||
                t.id === "priv_disable_lang_list" ||
                t.id === "priv_lockscreen_camera" ||
                t.id === "priv_config_wmp" ||
                t.id === "priv_disable_app_tracking" ||
                t.id === "priv_disable_perftrack"),
    );

    $: adTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id.includes("advertising") ||
                t.id.includes("tailored") ||
                t.id.includes("suggestions") ||
                t.id === "priv_disable_sync_notifs" ||
                t.id === "priv_all_in_one"),
    );

    $: policiesTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_disable_oobe_privacy" ||
                t.id === "priv_disallow_ms_accounts" ||
                t.id === "priv_disable_rsop" ||
                t.id === "priv_disable_experimentation" ||
                t.id === "priv_disable_smart_app_control"),
    );

    $: maintenanceTweaks = allTweaks.filter(
        (t) =>
            t.category === "Privacy" &&
            (t.id === "priv_config_storage_sense" ||
                t.id === "priv_disable_reserved_storage" ||
                t.id === "priv_disable_maintenance_wakeup"),
    );

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

<div class="privacy-container">
    {#if currentView === "dashboard"}
        <div class="dashboard-grid" in:fade>
            <!-- Windows Telemetry -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "windows")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "windows")}
            >
                <div class="card-icon">🖥️</div>
                <h3>Windows Telemetry</h3>
                <p>Core telemetry, DiagTrack, CEIP, and input collection.</p>
                <div class="status">{windowsTweaks.length} tweaks</div>
            </div>

            <!-- App Telemetry -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "apps")}
                on:keydown={(e) => e.key === "Enter" && (currentView = "apps")}
            >
                <div class="card-icon">📱</div>
                <h3>App Telemetry</h3>
                <p>NVIDIA, Office, VS Code, Chrome, Firefox, .NET.</p>
                <div class="status">{appTweaks.length} tweaks</div>
            </div>

            <!-- Privacy Settings -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "settings")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "settings")}
            >
                <div class="card-icon">⚙️</div>
                <h3>Privacy Settings</h3>
                <p>Activity, location, speech, app permissions, camera.</p>
                <div class="status">{settingsTweaks.length} tweaks</div>
            </div>

            <!-- Advertising -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "advertising")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "advertising")}
            >
                <div class="card-icon">📢</div>
                <h3>Advertising & Ads</h3>
                <p>Ad ID, tailored experiences, suggestions, explorer ads.</p>
                <div class="status">{adTweaks.length} tweaks</div>
            </div>

            <!-- System Policies -->
            <div
                class="card"
                role="button"
                tabindex="0"
                on:click={() => (currentView = "policies")}
                on:keydown={(e) =>
                    e.key === "Enter" && (currentView = "policies")}
            >
                <div class="card-icon">🛡️</div>
                <h3>System Policies</h3>
                <p>OOBE, MS Accounts, RSoP, Experiments, Smart App Control.</p>
                <div class="status">{policiesTweaks.length} tweaks</div>
            </div>

            <!-- Maintenance -->
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
                <p>Storage Sense, Reserved Storage, Wake-up timers.</p>
                <div class="status">{maintenanceTweaks.length} tweaks</div>
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

            <!-- 
               section-content structure mimics components/network/TcpSection.svelte structure
               using 'section-container' (but we call it section-content here).
               We ensure padding matches.
            -->
            <div class="section-content">
                {#if currentView === "windows"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🖥️ Windows Telemetry</h2>
                            <p>Core system tracking and diagnostics.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(windowsTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={windowsTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "apps"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>📱 App Telemetry</h2>
                            <p>Telemetry for apps and drivers.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(appTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={appTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "settings"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>⚙️ Privacy Settings</h2>
                            <p>General privacy and permissions.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(settingsTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={settingsTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "advertising"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>📢 Advertising & Tracking</h2>
                            <p>Ads, suggestions, and ID.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(adTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={adTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "policies"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🛡️ System Policies</h2>
                            <p>Group policies and restrictions.</p>
                        </div>
                        <button
                            class="optimize-btn safe"
                            on:click={() => applySafeTweaks(policiesTweaks)}
                        >
                            ✅ Apply Safe Tweaks
                        </button>
                    </div>
                    <div class="tweaks-wrapper">
                        <TweakList tweaks={policiesTweaks} showHeader={false} />
                    </div>
                {:else if currentView === "maintenance"}
                    <div class="section-header">
                        <div class="header-text">
                            <h2>🧹 Maintenance</h2>
                            <p>Storage and tasks.</p>
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
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .privacy-container {
        height: 100%;
        color: var(--text-color);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
        gap: 20px;
        margin-top: 16px;
        overflow-y: auto;
        flex: 1;
        padding: 20px;
        padding-top: 4px;
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 20px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .card:hover {
        background: rgba(255, 255, 255, 0.06);
        transform: translateY(-2px);
        border-color: var(--accent-color);
    }

    .card-icon {
        font-size: 28px;
        margin-bottom: 12px;
    }
    h3 {
        margin: 0 0 6px 0;
        font-size: 16px;
        font-weight: 600;
    }
    p {
        margin: 0 0 16px 0;
        color: var(--text-muted);
        font-size: 13px;
        line-height: 1.4;
        flex-grow: 1;
    }

    .status {
        font-size: 11px;
        font-weight: 500;
        color: var(--accent-color);
        background: rgba(59, 130, 246, 0.1);
        padding: 5px 10px;
        border-radius: 16px;
    }

    .detail-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 24px;
        overflow: hidden;
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
        flex-direction: column; /* Stack vertically */
        align-items: flex-start;
    }

    .header-text h2 {
        font-size: 20px;
        margin: 0 0 8px 0;
    }
    .header-text p {
        margin: 0 0 16px 0; /* Add bottom margin to separate from button */
        color: var(--text-muted);
        font-size: 14px;
    }

    .optimize-btn {
        border: none;
        padding: 8px 16px;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        color: white;
        white-space: nowrap;
        margin-left: 0; /* Reset margin */
        margin-top: 0;
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
        /* Replicate Network logic: Ensure TweakList fills this */
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
