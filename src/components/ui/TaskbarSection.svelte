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

    // 0 = Left, 1 = Center (Default)
    $: alignValue = isEnabled("taskbar_align_left") ? "left" : "center";

    // Small, Large, or Medium (Default if neither)
    $: sizeValue = isEnabled("taskbar_size_small")
        ? "small"
        : isEnabled("taskbar_size_large")
          ? "large"
          : "medium";

    // Top, Bottom (Default)
    $: posValue = isEnabled("taskbar_position_top") ? "top" : "bottom";

    // Other standalone tweaks
    $: otherTweaks = tweaks.filter(
        (t) =>
            !t.id.includes("size") &&
            !t.id.includes("align") &&
            !t.id.includes("position"),
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
            // Trigger refresh manually if stores don't update fast enough?
            // The activeCategory store in parent might fetch updates.
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
            if (val === "small") {
                if (isEnabled("taskbar_size_large"))
                    await invoke("undo_tweak", { id: "taskbar_size_large" });
                await invoke("apply_tweak", { id: "taskbar_size_small" });
            } else if (val === "large") {
                if (isEnabled("taskbar_size_small"))
                    await invoke("undo_tweak", { id: "taskbar_size_small" });
                await invoke("apply_tweak", { id: "taskbar_size_large" });
            } else {
                // Medium = disable both
                if (isEnabled("taskbar_size_small"))
                    await invoke("undo_tweak", { id: "taskbar_size_small" });
                if (isEnabled("taskbar_size_large"))
                    await invoke("undo_tweak", { id: "taskbar_size_large" });
            }
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
            if (val === "top") {
                await invoke("apply_tweak", { id: "taskbar_position_top" });
            } else {
                await invoke("undo_tweak", { id: "taskbar_position_top" });
            }
        } catch (err) {
            console.error(err);
        } finally {
            loadingMap["pos"] = false;
        }
    }
</script>

<div class="taskbar-settings">
    <!-- Config Grid -->
    <div class="config-grid">
        <!-- Alignment -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <LayoutTemplate size={20} />
                </div>
                <div class="text-info">
                    <h3>Alignment</h3>
                    <p>Icon position on taskbar</p>
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

        <!-- Size -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <Monitor size={20} />
                </div>
                <div class="text-info">
                    <h3>Icon Size</h3>
                    <p>Taskbar height & scale</p>
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

        <!-- Position -->
        <div class="config-card">
            <div class="card-content">
                <div class="icon-box">
                    <ArrowUp size={20} />
                </div>
                <div class="text-info">
                    <h3>Position</h3>
                    <p>Screen edge location</p>
                </div>
                <div class="action-area">
                    <Select
                        value={posValue}
                        options={[
                            { value: "top", label: "Top" },
                            { value: "bottom", label: "Bottom" },
                        ]}
                        loading={loadingMap["pos"]}
                        on:change={handlePosChange}
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
