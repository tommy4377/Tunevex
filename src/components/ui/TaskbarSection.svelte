<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Tweak } from "$lib/types";
    import Select from "./Select.svelte";
    import { LayoutTemplate, Monitor, ArrowUp } from "lucide-svelte";
    import TweakCard from "../TweakCard.svelte";

    export let tweaks: Tweak[] = [];

    // Helper to check status
    const isEnabled = (id: string) =>
        tweaks.find((t) => t.id === id)?.enabled ?? false;

    // --- Computed Values ---

    // --- Computed Values ---

    // 0 = Left, 1 = Center (Default)
    $: alignValue = isEnabled("taskbar_align_left") ? "left" : "center";

    // Small (16), Large (32), or Medium (Default)
    $: sizeValue = isEnabled("taskbar_small_ep")
        ? "small"
        : isEnabled("taskbar_large_ep")
          ? "large"
          : "medium";

    // Top, Left, Right, Bottom (Default)
    $: posValue = isEnabled("taskbar_top_ep")
        ? "top"
        : isEnabled("taskbar_left_ep")
          ? "left"
          : isEnabled("taskbar_right_ep")
            ? "right"
            : "bottom";

    // Style: Win10 (EP Init Active) vs Win11 (Default)
    // We check 'ep_config_init' which sets TaskbarStyle=1
    $: styleValue = isEnabled("ep_config_init") ? "win10" : "win11";

    // Other standalone tweaks (filter out position/size/style ones)
    $: otherTweaks = tweaks.filter(
        (t) =>
            !t.id.includes("size") &&
            !t.id.includes("align") &&
            !t.id.includes("position") &&
            !t.id.includes("_ep") && // Hide raw EP tweaks from toggle list
            t.id !== "ep_config_init",
    );

    // --- Handlers ---

    let loadingMap: Record<string, boolean> = {};

    async function handleAlignChange(e: CustomEvent) {
        loadingMap["align"] = true;
        const val = e.detail.value;
        try {
            if (val === "left") {
                await invoke("apply_tweak", { id: "taskbar_align_left" });
            } else {
                await invoke("undo_tweak", { id: "taskbar_align_left" });
            }
        } catch (err) {
            console.error(err);
        } finally {
            loadingMap["align"] = false;
        }
    }

    async function handleSizeChange(e: CustomEvent) {
        loadingMap["size"] = true;
        const val = e.detail.value;
        try {
            // Undo current size tweaks first
            if (isEnabled("taskbar_small_ep"))
                await invoke("undo_tweak", { id: "taskbar_small_ep" });
            if (isEnabled("taskbar_large_ep"))
                await invoke("undo_tweak", { id: "taskbar_large_ep" });

            // Apply new
            if (val === "small")
                await invoke("apply_tweak", { id: "taskbar_small_ep" });
            if (val === "large")
                await invoke("apply_tweak", { id: "taskbar_large_ep" });
            // medium = nothing (default)
        } catch (err) {
            console.error(err);
        } finally {
            loadingMap["size"] = false;
        }
    }

    async function handlePosChange(e: CustomEvent) {
        loadingMap["pos"] = true;
        const val = e.detail.value;
        try {
            // Undo all positional tweaks
            if (isEnabled("taskbar_top_ep"))
                await invoke("undo_tweak", { id: "taskbar_top_ep" });
            if (isEnabled("taskbar_left_ep"))
                await invoke("undo_tweak", { id: "taskbar_left_ep" });
            if (isEnabled("taskbar_right_ep"))
                await invoke("undo_tweak", { id: "taskbar_right_ep" });

            // Apply new
            if (val === "top")
                await invoke("apply_tweak", { id: "taskbar_top_ep" });
            if (val === "left")
                await invoke("apply_tweak", { id: "taskbar_left_ep" });
            if (val === "right")
                await invoke("apply_tweak", { id: "taskbar_right_ep" });
            // bottom = nothing (default)
        } catch (err) {
            console.error(err);
        } finally {
            loadingMap["pos"] = false;
        }
    }

    async function handleStyleChange(e: CustomEvent) {
        loadingMap["style"] = true;
        const val = e.detail.value;
        try {
            if (val === "win10") {
                await invoke("apply_tweak", { id: "ep_config_init" });
            } else {
                await invoke("undo_tweak", { id: "ep_config_init" }); // Needs revert op in back
                // If no revert op, we might need manual reset, but ep_config_init currently OneTime
                // Ideally ep_config_init should be Toggle for this to work perfectly.
            }
        } catch (err) {
            console.error(err);
        } finally {
            loadingMap["style"] = false;
        }
    }
</script>

<div class="taskbar-settings">
    <!-- Config Grid -->
    <div class="config-grid">
        <!-- Style (EP Base) -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <LayoutTemplate size={20} />
                </div>
                <div class="text-info">
                    <h3>Style</h3>
                    <p>Core taskbar engine</p>
                </div>
                <div class="action-area">
                    <Select
                        value={styleValue}
                        options={[
                            { value: "win11", label: "Windows 11" },
                            { value: "win10", label: "Windows 10" },
                        ]}
                        loading={loadingMap["style"]}
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
                        loading={loadingMap["pos"]}
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
                            { value: "medium", label: "Medium" },
                            { value: "large", label: "Large" },
                        ]}
                        loading={loadingMap["size"]}
                        on:change={handleSizeChange}
                    />
                </div>
            </div>
        </div>

        <!-- Alignment -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <LayoutTemplate size={20} />
                </div>
                <div class="text-info">
                    <h3>Align</h3>
                    <p>Icon grouping</p>
                </div>
                <div class="action-area">
                    <Select
                        value={alignValue}
                        options={[
                            { value: "left", label: "Left" },
                            { value: "center", label: "Center" },
                        ]}
                        loading={loadingMap["align"]}
                        on:change={handleAlignChange}
                    />
                </div>
            </div>
        </div>
    </div>

    <!-- Other Tweaks (Toggles) -->
    <h3 class="section-title">Behavior</h3>
    <div class="toggles-grid">
        {#each otherTweaks as tweak (tweak.id)}
            <TweakCard {tweak} on:toggle />
        {/each}
    </div>
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
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
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
        gap: 16px;
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
