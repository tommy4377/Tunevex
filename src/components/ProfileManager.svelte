<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import { Download, FileJson, Upload, X, AlertTriangle, CheckCircle2 } from "lucide-svelte";
    import type { ProfileImportPreview, ProfileOperation, ProfilePreviewEntry } from "$lib/types";

    type EditableEntry = ProfilePreviewEntry & { selected: boolean; result?: string };

    const dispatch = createEventDispatcher<{ applied: void }>();
    let preview: ProfileImportPreview | null = null;
    let entries: EditableEntry[] = [];
    let busy = false;
    let progress = "";
    let notice: { kind: "success" | "error"; text: string } | null = null;

    function defaultFileName(mode: "active" | "template") {
        const date = new Date().toISOString().slice(0, 10);
        return mode === "active" ? `tommytweaker-active-${date}.json` : `tommytweaker-template-${date}.json`;
    }

    async function exportProfile(mode: "active" | "template") {
        notice = null;
        let path: string | null;
        try {
            path = await save({
                defaultPath: defaultFileName(mode),
                filters: [{ name: "TommyTweaker profile", extensions: ["json"] }],
            });
        } catch (error) {
            notice = { kind: "error", text: `Could not open the save dialog: ${String(error)}` };
            return;
        }
        if (!path) return;

        busy = true;
        progress = mode === "active" ? "Reading current system state..." : "Generating editable catalog...";
        try {
            const result = await invoke<{ path: string; entries: number }>("export_tweak_profile", { path, mode });
            notice = { kind: "success", text: `Saved ${result.entries} entries to ${result.path}` };
        } catch (error) {
            notice = { kind: "error", text: String(error) };
        } finally {
            busy = false;
            progress = "";
        }
    }

    async function importProfile() {
        notice = null;
        let path: string | string[] | null;
        try {
            path = await open({
                multiple: false,
                directory: false,
                filters: [{ name: "TommyTweaker profile", extensions: ["json"] }],
            });
        } catch (error) {
            notice = { kind: "error", text: `Could not open the file dialog: ${String(error)}` };
            return;
        }
        if (!path || Array.isArray(path)) return;

        busy = true;
        progress = "Validating profile against this tweak catalog...";
        try {
            preview = await invoke<ProfileImportPreview>("preview_tweak_profile", { path });
            entries = preview.entries.map((entry) => ({
                ...entry,
                selected: entry.operation !== "skip" && !isNoOp(entry),
            }));
        } catch (error) {
            notice = { kind: "error", text: String(error) };
        } finally {
            busy = false;
            progress = "";
        }
    }

    function isNoOp(entry: ProfilePreviewEntry) {
        return (entry.operation === "enable" && entry.current_enabled) ||
            (entry.operation === "disable" && !entry.current_enabled) || entry.operation === "skip";
    }

    function setOperation(entry: EditableEntry, operation: ProfileOperation) {
        entry.operation = operation;
        entry.selected = operation !== "skip" && !isNoOp(entry);
        entry.result = undefined;
        entries = entries;
    }

    function availableOperations(entry: EditableEntry): ProfileOperation[] {
        return entry.tweak_type === "Action" ? ["skip", "run"] : ["skip", "enable", "disable"];
    }

    async function applyProfile() {
        const selected = entries.filter((entry) => entry.selected && !isNoOp(entry));
        if (selected.length === 0) {
            notice = { kind: "error", text: "Select at least one change to apply." };
            return;
        }
        if (!confirm(`Apply ${selected.length} selected profile operations?`)) return;

        busy = true;
        notice = null;
        let completed = 0;
        let failed = 0;
        let skipped = 0;
        for (const [index, entry] of selected.entries()) {
            progress = `${index + 1} / ${selected.length}: ${entry.name}`;
            let dangerousAcknowledgement: string | null = null;
            if (entry.warning_level === "Dangerous" && (entry.operation === "enable" || entry.operation === "run")) {
                const expected = `APPLY ${entry.id}`;
                dangerousAcknowledgement = prompt(
                    `POWER USER CONTROL\n\n${entry.name}\n\nType ${expected} to include this dangerous operation. Cancel to skip it.`
                );
                if (dangerousAcknowledgement !== expected) {
                    entry.result = "Skipped: acknowledgement not provided";
                    skipped += 1;
                    entries = entries;
                    continue;
                }
            }

            try {
                if (entry.operation === "disable") {
                    await invoke("undo_tweak", { id: entry.id });
                    entry.current_enabled = false;
                } else {
                    await invoke("apply_tweak", { id: entry.id, dangerousAcknowledgement });
                    if (entry.operation === "enable") entry.current_enabled = true;
                }
                entry.result = "Applied";
                entry.selected = false;
                completed += 1;
            } catch (error) {
                entry.result = `Failed: ${String(error)}`;
                failed += 1;
            }
            entries = entries;
        }
        busy = false;
        progress = "";
        notice = failed
            ? { kind: "error", text: `Applied ${completed}; ${failed} failed; ${skipped} skipped. Review the rows below.` }
            : { kind: "success", text: `Applied ${completed} profile operations${skipped ? `; ${skipped} skipped` : ""}.` };
        dispatch("applied");
    }

    function closePreview() {
        if (busy) return;
        preview = null;
        entries = [];
    }
</script>

<div class="profile-actions">
    <button on:click={() => exportProfile("active")} disabled={busy} title="Export only enabled stateful tweaks">
        <Download size={14} /> Active JSON
    </button>
    <button on:click={() => exportProfile("template")} disabled={busy} title="Export the complete editable tweak catalog">
        <FileJson size={14} /> Template
    </button>
    <button class="primary" on:click={importProfile} disabled={busy}>
        <Upload size={14} /> Import
    </button>
</div>

{#if progress && !preview}<div class="toast working">{progress}</div>{/if}
{#if notice && !preview}<div class:failure={notice.kind === "error"} class="toast">{notice.text}</div>{/if}

{#if preview}
    <div class="backdrop" role="presentation" on:click={closePreview}>
        <div class="profile-modal" role="dialog" aria-modal="true" aria-labelledby="profile-title" tabindex="-1" on:click|stopPropagation on:keydown={(event) => event.key === "Escape" && closePreview()}>
            <header>
                <div>
                    <span class="eyebrow">Validated profile</span>
                    <h2 id="profile-title">{preview.name}</h2>
                    <p>Choose exactly which operations to apply. Existing matching states are left untouched.</p>
                </div>
                <button class="icon-button" on:click={closePreview} disabled={busy} aria-label="Close"><X size={18} /></button>
            </header>

            <div class="summary">
                <span><strong>{entries.length}</strong> recognized</span>
                <span><strong>{entries.filter((entry) => entry.selected && !isNoOp(entry)).length}</strong> selected changes</span>
                {#if preview.issues.length}<span class="warning"><strong>{preview.issues.length}</strong> issues</span>{/if}
            </div>

            {#if preview.issues.length}
                <details class="issues">
                    <summary><AlertTriangle size={14} /> Show ignored entries</summary>
                    {#each preview.issues as issue}<p><code>{issue.id ?? "profile"}</code> — {issue.reason}</p>{/each}
                </details>
            {/if}

            <div class="entry-list">
                {#each entries as entry (entry.id)}
                    <div class:selected={entry.selected} class:dangerous={entry.warning_level === "Dangerous"} class="entry">
                        <input type="checkbox" bind:checked={entry.selected} disabled={busy || isNoOp(entry)} aria-label={`Select ${entry.name}`} />
                        <div class="entry-copy">
                            <div class="entry-title">
                                <strong>{entry.name}</strong>
                                <code>{entry.id}</code>
                            </div>
                            <span>{entry.category} · {entry.warning_level}{entry.requires_restart ? " · restart required" : ""}</span>
                            {#if entry.result}<small class:result-error={entry.result.startsWith("Failed")}>{entry.result}</small>{/if}
                        </div>
                        <div class="state">
                            {#if entry.tweak_type === "Toggle"}<small>Now: {entry.current_enabled ? "enabled" : "disabled"}</small>{/if}
                            <select value={entry.operation} on:change={(event) => setOperation(entry, event.currentTarget.value as ProfileOperation)} disabled={busy}>
                                {#each availableOperations(entry) as operation}<option value={operation}>{operation}</option>{/each}
                            </select>
                        </div>
                    </div>
                {/each}
                {#if entries.length === 0}<p class="empty">No compatible tweak entries were found.</p>{/if}
            </div>

            <footer>
                <div class="status">
                    {#if busy}<span class="spinner"></span>{progress}{:else if notice}
                        {#if notice.kind === "success"}<CheckCircle2 size={14} />{/if}{notice.text}
                    {/if}
                </div>
                <button on:click={closePreview} disabled={busy}>Close</button>
                <button class="apply" on:click={applyProfile} disabled={busy || entries.every((entry) => !entry.selected || isNoOp(entry))}>Apply selected</button>
            </footer>
        </div>
    </div>
{/if}

<style>
    .profile-actions { display: flex; gap: 6px; }
    button, select { font: inherit; }
    .profile-actions button, footer button, .icon-button {
        display: inline-flex; align-items: center; justify-content: center; gap: 6px;
        border: 1px solid var(--border-color); border-radius: var(--radius-md);
        background: rgba(255,255,255,.04); color: var(--text-secondary); padding: 7px 10px;
        font-size: 12px; cursor: pointer; transition: .15s ease;
    }
    button:hover:not(:disabled) { background: rgba(255,255,255,.08); color: var(--text-color); border-color: rgba(var(--accent-rgb),.45); }
    button:disabled { opacity: .45; cursor: not-allowed; }
    .profile-actions .primary, footer .apply { color: var(--accent-hover); background: rgba(var(--accent-rgb),.14); border-color: rgba(var(--accent-rgb),.35); }
    .toast { position: fixed; z-index: 200; right: 22px; bottom: 22px; max-width: 520px; padding: 10px 14px; border-radius: var(--radius-md); background: #17352d; border: 1px solid rgba(52,211,153,.35); color: #6ee7b7; font-size: 12px; box-shadow: 0 12px 32px rgba(0,0,0,.35); }
    .toast.failure { background: #3a2023; border-color: rgba(248,113,113,.35); color: #fca5a5; }
    .toast.working { background: #25263c; border-color: rgba(var(--accent-rgb),.35); color: var(--accent-hover); }
    .backdrop { position: fixed; inset: 0; z-index: 300; display: grid; place-items: center; padding: 28px; background: rgba(0,0,0,.65); backdrop-filter: blur(8px); }
    .profile-modal { width: min(920px, 92vw); max-height: min(760px, 88vh); display: flex; flex-direction: column; overflow: hidden; border: 1px solid rgba(255,255,255,.12); border-radius: var(--radius-lg); background: rgba(27,27,30,.98); box-shadow: 0 24px 80px rgba(0,0,0,.55); }
    header { display: flex; justify-content: space-between; gap: 20px; padding: 20px 22px 16px; border-bottom: 1px solid var(--border-color); }
    header h2 { margin: 2px 0 5px; font-size: 20px; }
    header p { margin: 0; color: var(--text-muted); font-size: 12px; }
    .eyebrow { color: var(--accent-hover); font-size: 10px; font-weight: 700; letter-spacing: .12em; text-transform: uppercase; }
    .icon-button { width: 32px; height: 32px; padding: 0; flex: 0 0 auto; }
    .summary { display: flex; gap: 18px; padding: 10px 22px; background: rgba(255,255,255,.025); color: var(--text-muted); font-size: 12px; }
    .summary strong { color: var(--text-color); }
    .summary .warning, .issues { color: #fbbf24; }
    .issues { margin: 10px 22px 0; padding: 10px 12px; border: 1px solid rgba(251,191,36,.2); border-radius: var(--radius-md); background: rgba(251,191,36,.05); font-size: 11px; }
    .issues summary { display: flex; align-items: center; gap: 6px; cursor: pointer; }
    .issues p { margin: 7px 0 0; color: var(--text-muted); }
    code { color: var(--text-muted); font-family: "Cascadia Code", monospace; font-size: 10px; }
    .entry-list { flex: 1; min-height: 0; overflow-y: auto; padding: 12px 22px; }
    .entry { display: grid; grid-template-columns: auto minmax(0,1fr) auto; align-items: center; gap: 12px; margin-bottom: 8px; padding: 11px 12px; border: 1px solid var(--border-color); border-radius: var(--radius-md); background: rgba(255,255,255,.02); }
    .entry.selected { border-color: rgba(var(--accent-rgb),.32); background: rgba(var(--accent-rgb),.06); }
    .entry.dangerous { border-left-color: #f87171; }
    .entry input { accent-color: var(--accent-color); }
    .entry-copy { min-width: 0; display: flex; flex-direction: column; gap: 4px; }
    .entry-title { display: flex; align-items: baseline; gap: 9px; min-width: 0; }
    .entry-title strong { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 13px; }
    .entry-copy > span, .state small { color: var(--text-muted); font-size: 10px; }
    .entry-copy small { color: #6ee7b7; font-size: 10px; }
    .entry-copy small.result-error { color: #fca5a5; }
    .state { display: flex; align-items: center; gap: 9px; }
    .state select { width: 92px; padding: 6px 8px; color: var(--text-color); background: #242428; border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: 11px; text-transform: capitalize; }
    .empty { padding: 40px; text-align: center; color: var(--text-muted); }
    footer { display: flex; align-items: center; gap: 8px; padding: 13px 22px; border-top: 1px solid var(--border-color); }
    footer .status { flex: 1; display: flex; align-items: center; gap: 7px; min-width: 0; color: var(--text-muted); font-size: 11px; }
    footer button { padding: 8px 14px; }
    .spinner { width: 12px; height: 12px; flex: 0 0 auto; border: 2px solid currentColor; border-top-color: transparent; border-radius: 50%; animation: spin .8s linear infinite; }
    @keyframes spin { to { transform: rotate(360deg); } }
    @media (max-width: 940px) {
        .profile-actions button { padding: 7px 8px; }
        .backdrop { padding: 14px; }
        .profile-modal { width: 96vw; max-height: 92vh; }
        .entry { grid-template-columns: auto minmax(0,1fr); }
        .state { grid-column: 2; justify-content: flex-end; }
    }
</style>
