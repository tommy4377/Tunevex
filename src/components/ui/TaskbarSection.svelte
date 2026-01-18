<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";
    import Select from "./Select.svelte";
    import { LayoutTemplate, Monitor, ArrowUp, AlignLeft } from "lucide-svelte";
    import TweakCard from "../TweakCard.svelte";
    import { createEventDispatcher } from "svelte";

    export let tweaks: Tweak[] = [];

    const dispatch = createEventDispatcher();

    // Helper to check enabled status from tweak list
    const isEnabled = (id: string) =>
        tweaks.find((t) => t.id === id)?.enabled ?? false;

    // --- Computed Values ---

    // Alignment: 0 = Left, 1 = Center (Default)
    $: alignValue = isEnabled("taskbar_align_left") ? "left" : "center";

    // Size: Small (0), Large (1 - default)
    $: sizeValue = isEnabled("taskbar_small_ep") ? "small" : "large";

    // Position: 0=Bottom, 1=Left, 2=Top, 3=Right
    $: posValue = isEnabled("taskbar_top_ep")
        ? "top"
        : isEnabled("taskbar_left_ep")
          ? "left"
          : isEnabled("taskbar_right_ep")
            ? "right"
            : "bottom";

    // Style: TaskbarStyle 0=Win11 (default), 1=Win10
    $: styleValue = isEnabled("ep_config_init") ? "win10" : "win11";

    // Other standalone tweaks (filter out position/size/style dropdowns)
    $: otherTweaks = tweaks.filter(
        (t) =>
            !t.id.includes("_ep") &&
            t.id !== "ep_config_init" &&
            t.id !== "taskbar_align_left",
    );

    // --- Loading states ---
    let loadingMap: Record<string, boolean> = {};

    // Force reactivity on loadingMap
    function setLoading(key: string, value: boolean) {
        loadingMap = { ...loadingMap, [key]: value };
    }

    // Refresh tweaks after applying
    async function refreshTweaks() {
        dispatch("refresh");
    }

    // --- Handlers ---

    async function handleAlignChange(e: CustomEvent) {
        setLoading("align", true);
        const val = e.detail.value;
        try {
            if (val === "left") {
                await invoke("apply_tweak", { id: "taskbar_align_left" });
            } else {
                await invoke("undo_tweak", { id: "taskbar_align_left" });
            }
            await refreshTweaks();
        } catch (err) {
            console.error(err);
        } finally {
            setLoading("align", false);
        }
    }

    async function handleSizeChange(e: CustomEvent) {
        setLoading("size", true);
        const val = e.detail.value;
        try {
            // Always undo both first
            if (isEnabled("taskbar_small_ep"))
                await invoke("undo_tweak", { id: "taskbar_small_ep" });
            if (isEnabled("taskbar_large_ep"))
                await invoke("undo_tweak", { id: "taskbar_large_ep" });

            // Apply new selection
            if (val === "small")
                await invoke("apply_tweak", { id: "taskbar_small_ep" });
            // "large" is default when neither is enabled
            await refreshTweaks();
        } catch (err) {
            console.error(err);
        } finally {
            setLoading("size", false);
        }
    }

    async function handlePosChange(e: CustomEvent) {
        setLoading("pos", true);
        const val = e.detail.value;
        try {
            // Undo all position tweaks first
            if (isEnabled("taskbar_top_ep"))
                await invoke("undo_tweak", { id: "taskbar_top_ep" });
            if (isEnabled("taskbar_left_ep"))
                await invoke("undo_tweak", { id: "taskbar_left_ep" });
            if (isEnabled("taskbar_right_ep"))
                await invoke("undo_tweak", { id: "taskbar_right_ep" });

            // Apply new position
            if (val === "top")
                await invoke("apply_tweak", { id: "taskbar_top_ep" });
            else if (val === "left")
                await invoke("apply_tweak", { id: "taskbar_left_ep" });
            else if (val === "right")
                await invoke("apply_tweak", { id: "taskbar_right_ep" });
            // "bottom" is default (no tweak needed)
            await refreshTweaks();
        } catch (err) {
            console.error(err);
        } finally {
            setLoading("pos", false);
        }
    }

    async function handleStyleChange(e: CustomEvent) {
        setLoading("style", true);
        const val = e.detail.value;
        try {
            if (val === "win10") {
                await invoke("apply_tweak", { id: "ep_config_init" });
            } else {
                await invoke("undo_tweak", { id: "ep_config_init" });
            }
            await refreshTweaks();
        } catch (err) {
            console.error(err);
        } finally {
            setLoading("style", false);
        }
    }
</script>

<div class="taskbar-settings">
    <!-- Config Grid -->
    <div class="config-grid">
        <!-- Style (Win10 vs Win11) -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <LayoutTemplate size={20} />
                </div>
                <div class="text-info">
                    <h3>Style</h3>
                    <p>Taskbar engine</p>
                </div>
                <div class="action-area">
                    <Select
                        value={styleValue}
                        options={[
                            { value: "win11", label: "Windows 11" },
                            { value: "win10", label: "Windows 10" },
                        ]}
                        loading={loadingMap["style"] ?? false}
                        on:change={handleStyleChange}
                    />
                </div>
            </div>
        </div>

        <!-- Position -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <ArrowUp size={20} />
                </div>
                <div class="text-info">
                    <h3>Position</h3>
                    <p>Screen edge</p>
                </div>
                <div class="action-area">
                    <Select
                        value={posValue}
                        options={[
                            { value: "bottom", label: "Bottom" },
                            { value: "top", label: "Top" },
                            { value: "left", label: "Left" },
                            { value: "right", label: "Right" },
                        ]}
                        loading={loadingMap["pos"] ?? false}
                        on:change={handlePosChange}
                    />
                </div>
            </div>
        </div>

        <!-- Size -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <Monitor size={20} />
                </div>
                <div class="text-info">
                    <h3>Size</h3>
                    <p>Icon scale</p>
                </div>
                <div class="action-area">
                    <Select
                        value={sizeValue}
                        options={[
                            { value: "small", label: "Small" },
                            { value: "large", label: "Large" },
                        ]}
                        loading={loadingMap["size"] ?? false}
                        on:change={handleSizeChange}
                    />
                </div>
            </div>
        </div>

        <!-- Alignment -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <AlignLeft size={20} />
                </div>
                <div class="text-info">
                    <h3>Align</h3>
                    <p>Icon position</p>
                </div>
                <div class="action-area">
                    <Select
                        value={alignValue}
                        options={[
                            { value: "center", label: "Center" },
                            { value: "left", label: "Left" },
                        ]}
                        loading={loadingMap["align"] ?? false}
                        on:change={handleAlignChange}
                    />
                </div>
            </div>
        </div>
    </div>

    <!-- Other Tweaks (Toggles) -->
    {#if otherTweaks.length > 0}
        <h3 class="section-title">Behavior</h3>
        <div class="toggles-grid">
            {#each otherTweaks as tweak (tweak.id)}
                <TweakCard {tweak} on:toggle />
            {/each}
        </div>
    {/if}
</div>

<style>
    .taskbar-settings {
        display: flex;
        flex-direction: column;
        gap: 24px;
        padding-bottom: 24px;
    }

    .config-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
        gap: 16px;
    }

    .config-card {
        background: var(--layer-card, rgba(255, 255, 255, 0.03));
        border: 1px solid var(--border-color, rgba(255, 255, 255, 0.08));
        border-radius: 12px;
        padding: 16px;
        transition: all 0.2s;
    }

    .config-card:hover {
        background: rgba(255, 255, 255, 0.05);
        border-color: rgba(255, 255, 255, 0.15);
    }

    .card-content {
        display: flex;
        align-items: center;
        gap: 12px;
        justify-content: space-between;
        width: 100%;
    }

    .icon-box {
        width: 40px;
        height: 40px;
        border-radius: 10px;
        background: rgba(96, 205, 255, 0.1);
        color: var(--accent-color);
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    .text-info {
        flex: 1;
        min-width: 0;
    }

    .text-info h3 {
        margin: 0 0 2px 0;
        font-size: 14px;
        font-weight: 600;
        color: var(--text-color);
    }

    .text-info p {
        margin: 0;
        font-size: 12px;
        color: var(--text-muted);
        white-space: nowrap;
    }

    .action-area {
        flex-shrink: 0;
    }

    .section-title {
        font-size: 16px;
        font-weight: 600;
        margin: 0;
        padding-left: 4px;
        color: var(--text-color);
    }

    .toggles-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 16px;
    }
</style>
