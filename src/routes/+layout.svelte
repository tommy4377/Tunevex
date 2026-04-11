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

<div id="app-mount">
    <Titlebar />

    {#if !isAdmin}
        <div class="admin-warning" transition:slide>
            <ShieldAlert size={16} />
            <span
                >Running with restricted privileges. Run as Administrator for
                full access.</span
            >
        </div>
    {/if}

    <div class="content-area">
        <slot />
    </div>
</div>

<style>
    /* New Minimal Layout Styles */
    #app-mount {
        background: rgba(32, 32, 32, 0.95);
        border-radius: 12px;
        overflow: hidden;
    }

    .content-area {
        flex: 1;
        display: flex;
        overflow: hidden;
        position: relative;
    }

    .admin-warning {
        background: rgba(220, 38, 38, 0.8);
        color: white;
        text-align: center;
        padding: 8px;
        font-size: 13px;
        font-weight: 600;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
    }
</style>
