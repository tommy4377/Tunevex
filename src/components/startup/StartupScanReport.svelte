<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {
        Sparkles, AlertTriangle, Check, X, Eye, ChevronDown, ChevronUp, RefreshCw
    } from "lucide-svelte";
    export let scanResult: any;
    export let allItems: any[];
    export let onBack: () => void;
    export let onRefresh: () => void;

    let collapsedSections: Record<string, boolean> = {
        disable: false,
        investigate: false,
        keep: false,
        all: false,
    };

    function toggleSection(key: string) {
        collapsedSections[key] = !collapsedSections[key];
    }

    function priorityColor(p: string) {
        return p === "high" ? "#f87171" : p === "medium" ? "#fbbf24" : "#34d399";
    }

    function safetyColor(s: string) {
        return s === "Safe" ? "#22c55e" : s === "Careful" ? "#facc15" : s === "Dangerous" ? "#f97316" : "#9ca3af";
    }

    $: recommendations = scanResult?.recommendations || [];
    $: disableRecs = recommendations.filter((r: any) => r.action === "disable");
    $: investigateRecs = recommendations.filter((r: any) => r.action === "investigate");
    $: keepRecs = recommendations.filter((r: any) => r.action === "keep");

    $: allItemIds = new Set(allItems.map((i: any) => i.id));
    $: recItemIds = new Set(recommendations.map((r: any) => r.item_id));

    $: unreviewedItems = allItems.filter((i: any) => !recItemIds.has(i.id));

    function getItem(itemId: string): any | undefined {
        return allItems.find((i: any) => i.id === itemId);
    }
</script>

<div class="scan-report">
    <div class="report-header">
        <div class="header-top">
            <div class="header-title">
                <Sparkles size={20} />
                <h1>AI Startup Scan Report</h1>
            </div>
            <div class="header-actions">
                <button class="btn-secondary" on:click={onRefresh}>
                    <RefreshCw size={13} /> New Scan
                </button>
                <button class="btn-secondary" on:click={onBack}>
                    ← Back
                </button>
            </div>
        </div>
        {#if scanResult?.summary}
            <div class="report-summary">
                <p>{scanResult.summary}</p>
            </div>
        {/if}
    </div>

    <div class="report-stats">
        <div class="stat-card danger">
            <div class="stat-num">{disableRecs.length}</div>
            <div class="stat-label">To Disable</div>
        </div>
        <div class="stat-card warning">
            <div class="stat-num">{investigateRecs.length}</div>
            <div class="stat-label">Investigate</div>
        </div>
        <div class="stat-card safe">
            <div class="stat-num">{keepRecs.length}</div>
            <div class="stat-label">Keep</div>
        </div>
        <div class="stat-card neutral">
            <div class="stat-num">{unreviewedItems.length}</div>
            <div class="stat-label">Not Reviewed</div>
        </div>
    </div>

    {#if disableRecs.length > 0}
        <div class="report-section">
            <button class="section-header danger" on:click={() => toggleSection("disable")}>
                <div class="section-title">
                    <X size={14} />
                    <span>Items to Disable</span>
                    <span class="section-count">{disableRecs.length}</span>
                </div>
                {#if collapsedSections.disable}<ChevronDown size={14} />{:else}<ChevronUp size={14} />{/if}
            </button>
            {#if !collapsedSections.disable}
                <div class="section-body">
                    {#each disableRecs as rec}
                        {@const item = getItem(rec.item_id)}
                        <div class="report-card danger-card">
                            <div class="card-left">
                                <div class="card-priority" style="color: {priorityColor(rec.priority)}">{rec.priority}</div>
                                <div class="card-action-badge danger">disable</div>
                            </div>
                            <div class="card-main">
                                <div class="card-title-row">
                                    <span class="card-name">{rec.item_name || rec.item_id.split("\\").pop()}</span>
                                    {#if item?.publisher}
                                        <span class="card-publisher">{item.publisher}</span>
                                    {/if}
                                </div>
                                <div class="card-id">{rec.item_id}</div>
                                {#if item?.command}
                                    <div class="card-command">{item.command}</div>
                                {/if}
                                <div class="card-reason">
                                    <strong>AI Reason:</strong> {rec.reason}
                                </div>
                                {#if item?.safety_rating}
                                    <div class="card-safety" style="color: {safetyColor(item.safety_rating)}">
                                        Safety: {item.safety_rating}
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    {/if}

    {#if investigateRecs.length > 0}
        <div class="report-section">
            <button class="section-header warning" on:click={() => toggleSection("investigate")}>
                <div class="section-title">
                    <Eye size={14} />
                    <span>Items to Investigate</span>
                    <span class="section-count">{investigateRecs.length}</span>
                </div>
                {#if collapsedSections.investigate}<ChevronDown size={14} />{:else}<ChevronUp size={14} />{/if}
            </button>
            {#if !collapsedSections.investigate}
                <div class="section-body">
                    {#each investigateRecs as rec}
                        {@const item = getItem(rec.item_id)}
                        <div class="report-card warning-card">
                            <div class="card-left">
                                <div class="card-priority" style="color: {priorityColor(rec.priority)}">{rec.priority}</div>
                                <div class="card-action-badge warning">investigate</div>
                            </div>
                            <div class="card-main">
                                <div class="card-title-row">
                                    <span class="card-name">{rec.item_name || rec.item_id.split("\\").pop()}</span>
                                    {#if item?.publisher}
                                        <span class="card-publisher">{item.publisher}</span>
                                    {/if}
                                </div>
                                <div class="card-id">{rec.item_id}</div>
                                {#if item?.command}
                                    <div class="card-command">{item.command}</div>
                                {/if}
                                <div class="card-reason">
                                    <strong>AI Reason:</strong> {rec.reason}
                                </div>
                                {#if item?.safety_rating}
                                    <div class="card-safety" style="color: {safetyColor(item.safety_rating)}">
                                        Safety: {item.safety_rating}
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    {/if}

    {#if keepRecs.length > 0}
        <div class="report-section">
            <button class="section-header safe" on:click={() => toggleSection("keep")}>
                <div class="section-title">
                    <Check size={14} />
                    <span>Items to Keep</span>
                    <span class="section-count">{keepRecs.length}</span>
                </div>
                {#if collapsedSections.keep}<ChevronDown size={14} />{:else}<ChevronUp size={14} />{/if}
            </button>
            {#if !collapsedSections.keep}
                <div class="section-body">
                    {#each keepRecs as rec}
                        {@const item = getItem(rec.item_id)}
                        <div class="report-card safe-card">
                            <div class="card-left">
                                <div class="card-priority" style="color: {priorityColor(rec.priority)}">{rec.priority}</div>
                                <div class="card-action-badge safe">keep</div>
                            </div>
                            <div class="card-main">
                                <div class="card-title-row">
                                    <span class="card-name">{rec.item_name || rec.item_id.split("\\").pop()}</span>
                                    {#if item?.publisher}
                                        <span class="card-publisher">{item.publisher}</span>
                                    {/if}
                                </div>
                                <div class="card-id">{rec.item_id}</div>
                                {#if item?.command}
                                    <div class="card-command">{item.command}</div>
                                {/if}
                                <div class="card-reason">
                                    <strong>AI Reason:</strong> {rec.reason}
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    {/if}

    {#if unreviewedItems.length > 0}
        <div class="report-section">
            <button class="section-header neutral" on:click={() => toggleSection("all")}>
                <div class="section-title">
                    <AlertTriangle size={14} />
                    <span>Not Reviewed by AI ({unreviewedItems.length})</span>
                    <span class="section-count">{unreviewedItems.length}</span>
                </div>
                {#if collapsedSections.all}<ChevronDown size={14} />{:else}<ChevronUp size={14} />{/if}
            </button>
            {#if !collapsedSections.all}
                <div class="section-body">
                    <p class="not-reviewed-note">
                        These items were scanned but not mentioned in the AI recommendations.
                        They were likely considered safe/unchanging by the AI.
                    </p>
                    {#each unreviewedItems as item}
                        <div class="report-card neutral-card">
                            <div class="card-main">
                                <div class="card-title-row">
                                    <span class="card-name">{item.name}</span>
                                    {#if item.publisher}
                                        <span class="card-publisher">{item.publisher}</span>
                                    {/if}
                                    <div class="card-safety" style="color: {safetyColor(item.safety_rating)}">
                                        {item.safety_rating}
                                    </div>
                                </div>
                                <div class="card-id">{item.id}</div>
                                <div class="card-command">{item.command}</div>
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    {/if}

    <div class="report-footer">
        <p>AI analysis complete — {recommendations.length} items reviewed out of {allItems.length} total startup items.</p>
    </div>
</div>

<style>
    .scan-report {
        padding: 24px 32px;
        height: 100%;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .report-header {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .header-top {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .header-title {
        display: flex;
        align-items: center;
        gap: 10px;
        color: var(--accent-color);
    }

    .header-title h1 {
        margin: 0;
        font-size: 22px;
        font-weight: 700;
        color: var(--text-color);
    }

    .header-actions {
        display: flex;
        gap: 8px;
    }

    .btn-secondary {
        display: flex;
        align-items: center;
        gap: 6px;
        background: var(--layer-card);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 7px 14px;
        color: var(--text-muted);
        font-size: 13px;
        cursor: pointer;
        transition: all 0.15s;
    }

    .btn-secondary:hover {
        color: var(--text-color);
        border-color: var(--text-muted);
    }

    .report-summary {
        background: rgba(129, 140, 248, 0.08);
        border: 1px solid rgba(129, 140, 248, 0.2);
        border-radius: 10px;
        padding: 14px 18px;
    }

    .report-summary p {
        margin: 0;
        font-size: 14px;
        color: var(--text-secondary);
        line-height: 1.6;
    }

    .report-stats {
        display: flex;
        gap: 16px;
        flex-wrap: wrap;
    }

    .stat-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
        padding: 14px 24px;
        border-radius: 12px;
        border: 1px solid;
        min-width: 100px;
    }

    .stat-card.danger { background: rgba(248, 113, 113, 0.08); border-color: rgba(248, 113, 113, 0.2); }
    .stat-card.warning { background: rgba(251, 191, 36, 0.08); border-color: rgba(251, 191, 36, 0.2); }
    .stat-card.safe { background: rgba(52, 211, 153, 0.08); border-color: rgba(52, 211, 153, 0.2); }
    .stat-card.neutral { background: rgba(255, 255, 255, 0.04); border-color: rgba(255, 255, 255, 0.1); }

    .stat-num {
        font-size: 28px;
        font-weight: 700;
        line-height: 1;
    }

    .stat-card.danger .stat-num { color: #f87171; }
    .stat-card.warning .stat-num { color: #fbbf24; }
    .stat-card.safe .stat-num { color: #34d399; }
    .stat-card.neutral .stat-num { color: var(--text-muted); }

    .stat-label {
        font-size: 12px;
        font-weight: 600;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .report-section {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .section-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 10px 16px;
        border-radius: 10px;
        border: 1px solid;
        cursor: pointer;
        transition: all 0.15s;
        font-size: 13px;
        font-weight: 600;
    }

    .section-header.danger { background: rgba(248, 113, 113, 0.08); border-color: rgba(248, 113, 113, 0.2); color: #f87171; }
    .section-header.warning { background: rgba(251, 191, 36, 0.08); border-color: rgba(251, 191, 36, 0.2); color: #fbbf24; }
    .section-header.safe { background: rgba(52, 211, 153, 0.08); border-color: rgba(52, 211, 153, 0.2); color: #34d399; }
    .section-header.neutral { background: rgba(255, 255, 255, 0.04); border-color: rgba(255, 255, 255, 0.1); color: var(--text-muted); }

    .section-title {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .section-count {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
        padding: 1px 8px;
        font-size: 11px;
    }

    .section-body {
        display: flex;
        flex-direction: column;
        gap: 8px;
        padding-left: 4px;
    }

    .not-reviewed-note {
        font-size: 12px;
        color: var(--text-muted);
        margin: 0 0 8px 0;
        padding: 8px 12px;
        background: rgba(255, 255, 255, 0.03);
        border-radius: 6px;
    }

    .report-card {
        display: flex;
        gap: 16px;
        padding: 14px 16px;
        border-radius: 10px;
        border: 1px solid rgba(255, 255, 255, 0.06);
        background: var(--layer-card);
    }

    .report-card.danger-card { border-left: 3px solid #f87171; }
    .report-card.warning-card { border-left: 3px solid #fbbf24; }
    .report-card.safe-card { border-left: 3px solid #34d399; }
    .report-card.neutral-card { border-left: 3px solid rgba(255, 255, 255, 0.15); }

    .card-left {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 6px;
        min-width: 60px;
    }

    .card-priority {
        font-size: 11px;
        font-weight: 700;
        text-transform: uppercase;
    }

    .card-action-badge {
        font-size: 10px;
        font-weight: 700;
        text-transform: uppercase;
        padding: 2px 7px;
        border-radius: 10px;
    }

    .card-action-badge.danger { background: rgba(248, 113, 113, 0.15); color: #f87171; }
    .card-action-badge.warning { background: rgba(251, 191, 36, 0.15); color: #fbbf24; }
    .card-action-badge.safe { background: rgba(52, 211, 153, 0.15); color: #34d399; }

    .card-main {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 4px;
        min-width: 0;
    }

    .card-title-row {
        display: flex;
        align-items: center;
        gap: 10px;
        flex-wrap: wrap;
    }

    .card-name {
        font-size: 14px;
        font-weight: 600;
        color: var(--text-color);
    }

    .card-publisher {
        font-size: 12px;
        color: var(--text-muted);
        font-style: italic;
    }

    .card-id {
        font-size: 11px;
        font-family: monospace;
        color: var(--text-muted);
        word-break: break-all;
    }

    .card-command {
        font-size: 11px;
        font-family: monospace;
        color: var(--text-secondary);
        background: rgba(0, 0, 0, 0.2);
        padding: 3px 7px;
        border-radius: 4px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .card-reason {
        font-size: 12px;
        color: var(--text-secondary);
        line-height: 1.5;
        margin-top: 4px;
    }

    .card-safety {
        font-size: 11px;
        font-weight: 600;
    }

    .report-footer {
        padding-top: 12px;
        border-top: 1px solid var(--border-color);
    }

    .report-footer p {
        margin: 0;
        font-size: 12px;
        color: var(--text-muted);
        text-align: center;
    }
</style>
