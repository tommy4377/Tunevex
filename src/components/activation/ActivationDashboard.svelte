<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import type { Tweak } from "$lib/types";
    import TerminalModal from "../TerminalModal.svelte";
    import Button from "../ui/Button.svelte";
    import {
        Layout,
        Monitor,
        Key,
        ShieldAlert,
        CheckCircle,
        RefreshCw,
    } from "lucide-svelte";

    export let allTweaks: Tweak[] = [];

    function getButtonVariant(tweakId: string): "primary" | "ghost" | "danger" {
        if (tweakId.includes("remove")) return "danger";
        if (tweakId.includes("check")) return "ghost";
        return "primary";
    }

    function getButtonLabel(tweakId: string): string {
        if (tweakId.includes("check")) return "Status";
        if (tweakId.includes("remove")) return "Deactivate";
        return "Activate";
    }

    // Split tweaks into groups
    $: windowsTweaks = allTweaks.filter(
        (t) =>
            t.id.includes("check_windows") ||
            t.id === "activation_hwid" ||
            t.id === "activation_remove_windows",
    );

    $: officeTweaks = allTweaks.filter(
        (t) =>
            t.id.includes("check_office") ||
            t.id === "activation_ohook" ||
            t.id === "activation_remove_office",
    );

    // Modal State
    let showModal = false;
    let modalTitle = "";
    let modalLogs: string[] = [];
    let processingId: string | null = null;
    let unlisten: (() => void) | null = null;

    onMount(async () => {
        unlisten = await listen<any>("tweak-output", (event: any) => {
            const { id, line } = event.payload;
            if (id === processingId) {
                if (!showModal) {
                    showModal = true;
                    modalTitle = "Execution Output";
                }
                modalLogs = [...modalLogs, line];
            }
        });
    });

    onDestroy(() => {
        if (unlisten) unlisten();
    });

    async function execute(tweak: Tweak) {
        if (processingId) return;
        processingId = tweak.id;
        modalLogs = [];
        modalTitle = `Running: ${tweak.name}`;
        showModal = true;

        try {
            await invoke("apply_tweak", { id: tweak.id });
        } catch (e) {
            modalLogs = [...modalLogs, `Error: ${e}`];
        } finally {
        processingId = null;
        }
    }
</script>

<div class="activation-dashboard">
    <div class="header">
        <h1>Activation Center</h1>
        <p>Manage your Windows and Office licenses safely.</p>
    </div>

    <div class="grid">
        <!-- Windows Section -->
        <div class="card windows">
            <div class="card-header">
                <span class="icon"><Monitor /></span>
                <h2>Windows</h2>
            </div>
            <div class="card-body">
                {#each windowsTweaks as t}
                    <div class="action-row">
                        <div class="info">
                            <h3>{t.name}</h3>
                            <p>{t.description}</p>
                        </div>
                        <Button
                            variant={getButtonVariant(t.id)}
                            on:click={() => execute(t)}
                            disabled={!!processingId}
                        >
                            {getButtonLabel(t.id)}
                        </Button>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Office Section -->
        <div class="card office">
            <div class="card-header">
                <span class="icon"><Layout /></span>
                <h2>Office</h2>
            </div>
            <div class="card-body">
                {#each officeTweaks as t}
                    <div class="action-row">
                        <div class="info">
                            <h3>{t.name}</h3>
                            <p>{t.description}</p>
                        </div>
                        <Button
                            variant={getButtonVariant(t.id)}
                            on:click={() => execute(t)}
                            disabled={!!processingId}
                        >
                            {getButtonLabel(t.id)}
                        </Button>
                    </div>
                {/each}
            </div>
        </div>
    </div>
</div>

<TerminalModal
    bind:show={showModal}
    title={modalTitle}
    logs={modalLogs}
    processing={!!processingId}
    on:close={async () => {
        if (processingId) {
            try {
                await invoke("kill_tweak_process", { id: processingId });
            } catch (e) {
                console.error("Failed to kill process:", e);
            }
        }
        showModal = false;
        processingId = null;
    }}
/>

<style>
    .activation-dashboard {
        padding: 0 32px 32px;
        height: 100%;
        overflow-y: auto;
    }

    .header h1 {
        font-size: 28px;
        font-weight: 700;
        margin-bottom: 8px;
        background: linear-gradient(to right, var(--text-color), var(--text-muted));
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .header p {
        color: var(--text-muted);
        font-size: 16px;
        margin-bottom: 32px;
    }

    .grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 24px;
        align-items: stretch;
    }

    .card {
        background: var(--layer-card);
        backdrop-filter: blur(12px);
        border: var(--border-glass);
        border-radius: var(--radius-card);
        overflow: hidden;
        transition:
            transform 0.2s,
            background 0.2s;
        display: flex;
        flex-direction: column;
    }

    .card:hover {
        background: rgba(255, 255, 255, 0.04);
        transform: translateY(-2px);
    }

    .card-header {
        padding: 20px 24px;
        background: rgba(255, 255, 255, 0.02);
        border-bottom: 1px solid var(--border-color);
        display: flex;
        align-items: center;
        gap: 16px;
    }

    .card-header h2 {
        font-size: 18px;
        font-weight: 600;
        margin: 0;
    }

    .icon {
        color: var(--accent-color);
        display: flex;
        align-items: center;
    }

    .card-body {
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 20px;
        flex: 1;
    }

    .action-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 20px;
        padding-bottom: 20px;
        border-bottom: 1px solid var(--border-color);
    }

    .action-row:last-child {
        border-bottom: none;
        padding-bottom: 0;
    }

    .info h3 {
        font-size: 15px;
        font-weight: 600;
        margin: 0 0 6px 0;
        color: var(--text-color);
    }

    .info p {
        font-size: 13px;
        color: var(--text-muted);
        margin: 0;
        line-height: 1.4;
        max-width: 300px;
    }
</style>
