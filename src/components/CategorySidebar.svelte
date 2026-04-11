<script lang="ts">
    import { activeCategory } from "$lib/stores";
    import type { TweakCategory } from "$lib/types";

    // Using simple SVG icons or lucide-svelte if available.
    // Package.json has lucide-svelte, so let's use it.
    import {
        Network,
        Shield,
        Lock,
        Zap,
        Trash2,
        Cpu,
        PlayCircle,
        Settings,
        MousePointer,
        Monitor,
        HardDrive,
        Gamepad,
        Palette,
        Key,
        Home,
        Sparkles,
    } from "lucide-svelte";

    const categories: { id: TweakCategory; label: string; icon: any }[] = [
        { id: "Home", label: "Home", icon: Home },
        { id: "Activation", label: "Activation", icon: Key },
        { id: "Network", label: "Network", icon: Network },
        { id: "SecurityPrivacy", label: "Security", icon: Lock },
        { id: "Privacy", label: "Privacy", icon: Shield },
        { id: "DebloatTelemetry", label: "Debloat", icon: Trash2 },
        { id: "CpuPerformance", label: "CPU", icon: Cpu },
        { id: "GpuOptimization", label: "GPU", icon: Palette },
        { id: "GameOptimizations", label: "Gaming", icon: Gamepad },
        { id: "System", label: "System", icon: Settings },
        { id: "InterfaceUx", label: "Interface", icon: Monitor },
        { id: "StartupServices", label: "Startup", icon: PlayCircle },
        { id: "MouseInput", label: "Input", icon: MousePointer },
        { id: "DisplayMonitor", label: "Display", icon: Monitor },
        { id: "FileSystem", label: "Storage", icon: HardDrive },
    ];

    function selectCategory(id: TweakCategory) {
        activeCategory.set(id);
    }
</script>

<div class="sidebar">
    <div class="menu">
        {#each categories as cat}
            <button
                class:active={$activeCategory === cat.id}
                on:click={() => selectCategory(cat.id)}
            >
                <svelte:component this={cat.icon} size={18} />
                <span class="label">{cat.label}</span>
            </button>
        {/each}
    </div>
</div>

<style>
    .sidebar {
        width: 240px;
        background: transparent;
        border-right: 1px solid var(--border-color);
        padding: 16px 0;
        display: flex;
        flex-direction: column;
        justify-content: center; /* Center items vertically */
    }

    .menu {
        display: flex;
        flex-direction: column;
        gap: 2px;
        padding: 0 12px;
        /* Ensure menu doesn't overflow if too tall when centered */
        max-height: 100%;
        overflow-y: auto;
    }
    /* ... rest of styles ... */

    button {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 8px 12px;
        border: none;
        background: transparent;
        color: var(--text-muted);
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        border-radius: var(--radius-md);
        transition: all 0.2s;
        text-align: left;
    }

    button:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-color);
    }

    button.active {
        background: rgba(255, 255, 255, 0.08);
        color: var(--text-color);
    }

    button.active :global(svg) {
        color: var(--accent-color);
    }

    .label {
        margin-top: 1px; /* Optical handling */
    }
</style>
