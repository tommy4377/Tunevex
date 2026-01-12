<script lang="ts">
    import "../app.css";
    import Titlebar from "../components/Titlebar.svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let isAdmin = true; // Default true to avoid flash, check immediately

    onMount(async () => {
        // ... (existing admin check code) ...
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
    <Titlebar />
    {#if !isAdmin}
        <div class="admin-warning">
            ⚠️ Administrator privileges required. Some tweaks may not apply.
        </div>
    {/if}
    <div class="content-area">
        <slot />
    </div>
</div>

<style>
    .content-area {
        flex: 1;
        display: flex;
        overflow: hidden;
        position: relative;
    }
    .admin-warning {
        background: #ef4444;
        color: white;
        text-align: center;
        padding: 8px;
        font-size: 13px;
        font-weight: 500;
        animation: slideIn 0.3s ease-out;
    }
    @keyframes slideIn {
        from {
            transform: translateY(-100%);
            opacity: 0;
        }
        to {
            transform: translateY(0);
            opacity: 1;
        }
    }
</style>
