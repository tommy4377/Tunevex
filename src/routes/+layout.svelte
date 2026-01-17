<script lang="ts">
    import "../app.css";
    import Titlebar from "../components/Titlebar.svelte";
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { ShieldAlert } from "lucide-svelte";
    import { fade, slide } from "svelte/transition";

    let isAdmin = true; // Optimistic default

    // Mica needs transparent window. This div effectively acts as the "tint" layer
    // The actual blur comes from the OS via window-vibrancy in Rust

    onMount(async () => {
        try {
            isAdmin = await invoke("check_is_admin");
            if (!isAdmin) {
                console.warn(
                    "App is not running as Admin. Restricted functionality.",
                );
            }
        } catch (e) {
            console.error("Failed to check admin status:", e);
        }
    });
</script>

<div class="rounded-window">
    <div class="noise-overlay"></div>

    <Titlebar />

    {#if !isAdmin}
        <div class="admin-warning" transition:slide>
            <ShieldAlert size={16} />
            <span
                >Administrator privileges required. Some tweaks may not apply.</span
            >
        </div>
    {/if}

    <div class="content-area">
        <slot />
    </div>
</div>

<style>
    /* Noise texture for "premium" feel over Mica */
    .noise-overlay {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)' opacity='0.03'/%3E%3C/svg%3E");
        pointer-events: none;
        z-index: 0;
        opacity: 0.4;
    }

    .content-area {
        flex: 1;
        display: flex;
        overflow: hidden;
        position: relative;
        z-index: 1; /* Above noise */
    }

    .admin-warning {
        background: rgba(239, 68, 68, 0.85); /* Red glass */
        backdrop-filter: blur(10px);
        color: white;
        text-align: center;
        padding: 10px;
        font-size: 13px;
        font-weight: 600;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        z-index: 20;
    }
</style>
