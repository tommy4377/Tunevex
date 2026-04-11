<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Sparkles, AlertTriangle, Plus, Minus, Check } from "lucide-svelte";
  import Button from "../ui/Button.svelte";

  type Priority = "high" | "medium" | "low";
  interface Rec { id: string; priority: Priority; reason: string; selected: boolean; }
  interface Conflict { tweak_ids: string[]; issue: string; }
  interface Analysis {
    system_summary: string;
    add: Rec[];
    remove: Rec[];
    conflicts: Conflict[];
  }

  let scanning = false;
  let analysis: Analysis | null = null;
  let error = "";

  async function scan() {
    scanning = true;
    error = "";
    try {
      const raw = await invoke<Analysis>("ai_analyze");
      analysis = {
        ...raw,
        add:    raw.add.map(r => ({ ...r, selected: true })),
        remove: raw.remove.map(r => ({ ...r, selected: true })),
      };
    } catch (e) {
      error = e as string;
    } finally {
      scanning = false;
    }
  }

  async function applySelected() {
    if (!analysis) return;
    for (const r of analysis.remove.filter(r => r.selected))
      await invoke("undo_tweak", { id: r.id });
    for (const a of analysis.add.filter(a => a.selected))
      await invoke("apply_tweak", { id: a.id });
  }
</script>

<div class="scan-panel">
  <Button onclick={scan} disabled={scanning}>
    <Sparkles size={14} />
    {scanning ? "Scanning system…" : "Scan & Recommend"}
  </Button>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if analysis}
    <p class="summary">{analysis.system_summary}</p>

    {#if analysis.conflicts.length}
      <div class="section-label">
        <AlertTriangle size={11} /> Conflicts detected
      </div>
      {#each analysis.conflicts as c}
        <div class="conflict-row">
          <AlertTriangle size={12} />
          <div class="conflict-body">
            <span>{c.issue}</span>
            <span class="conflict-ids">{c.tweak_ids.join(" + ")}</span>
          </div>
        </div>
      {/each}
    {/if}

    {#if analysis.remove.length}
      <h3><Minus size={13} /> Remove — counterproductive on your system</h3>
      {#each analysis.remove as r (r.id)}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div
          class="rec-row danger"
          class:selected={r.selected}
          role="checkbox"
          aria-checked={r.selected}
          tabindex="0"
          on:click={() => r.selected = !r.selected}
          on:keydown={(e) => e.key === " " && (r.selected = !r.selected)}
        >
          <div class="rec-check" class:checked={r.selected}>
            {#if r.selected}<Check size={9} />{/if}
          </div>
          <div class="rec-body">
            <div class="rec-header">
              <code class="rec-id">{r.id}</code>
              <span class="rec-priority {r.priority}">{r.priority}</span>
            </div>
            <p class="rec-reason">{r.reason}</p>
          </div>
        </div>
      {/each}
    {/if}

    {#if analysis.add.length}
      <h3><Plus size={13} /> Recommended for your system</h3>
      {#each analysis.add.sort((a, b) =>
        ["high","medium","low"].indexOf(a.priority) -
        ["high","medium","low"].indexOf(b.priority)
      ) as a (a.id)}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div
          class="rec-row"
          class:selected={a.selected}
          role="checkbox"
          aria-checked={a.selected}
          tabindex="0"
          on:click={() => a.selected = !a.selected}
          on:keydown={(e) => e.key === " " && (a.selected = !a.selected)}
        >
          <div class="rec-check" class:checked={a.selected}>
            {#if a.selected}<Check size={9} />{/if}
          </div>
          <div class="rec-body">
            <div class="rec-header">
              <code class="rec-id">{a.id}</code>
              <span class="rec-priority {a.priority}">{a.priority}</span>
            </div>
            <p class="rec-reason">{a.reason}</p>
          </div>
        </div>
      {/each}
    {/if}

    <div class="action-bar">
      <Button onclick={applySelected}>Apply Selected</Button>
    </div>
  {/if}
</div>

<style>
  .scan-panel { display: flex; flex-direction: column; gap: 16px; }
  .summary { color: var(--text-muted); font-size: 13px; }

  .section-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.6px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding-bottom: 8px;
    border-bottom: var(--border-glass);
    margin-top: 8px;
  }

  .conflict-row {
    display: grid;
    grid-template-columns: 14px 1fr;
    gap: 10px;
    align-items: start;
    padding: 9px 12px;
    background: var(--layer-card);
    border: var(--border-glass);
    border-left: 2px solid rgba(251,191,36,0.4);
    border-radius: var(--radius-md);
    font-size: 12px;
  }
  .conflict-row :global(svg) { color: #fbbf24; margin-top: 1px; }
  .conflict-body { display: flex; flex-direction: column; gap: 2px; }
  .conflict-body span { color: var(--text-secondary); line-height: 1.4; }
  .conflict-ids { font-size: 10px; color: var(--text-muted); font-family: monospace; }

  h3 { font-size: 13px; font-weight: 600; color: var(--text-muted);
    display: flex; align-items: center; gap: 6px; margin: 8px 0 4px; }

  .rec-row {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 10px 14px;
    background: var(--layer-card);
    border: 1px solid rgba(255,255,255,0.06);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
    user-select: none;
    outline: none;
  }
  .rec-row:hover        { background: var(--layer-hover); border-color: rgba(255,255,255,0.1); }
  .rec-row:focus-visible{ border-color: var(--accent-color); }
  .rec-row.selected     { border-color: rgba(129,140,248,0.28); background: rgba(129,140,248,0.05); }
  .rec-row.danger.selected { border-color: rgba(248,113,113,0.22); background: rgba(248,113,113,0.04); }

  .rec-check {
    width: 16px; height: 16px;
    border-radius: 4px;
    border: 1px solid rgba(255,255,255,0.18);
    background: rgba(255,255,255,0.04);
    flex-shrink: 0;
    margin-top: 2px;
    display: flex; align-items: center; justify-content: center;
    transition: all 0.12s;
    color: white;
  }
  .rec-check.checked          { background: var(--accent-color); border-color: var(--accent-color); }
  .danger .rec-check.checked  { background: #f87171;             border-color: #f87171; }

  .rec-body { flex: 1; min-width: 0; }
  .rec-header { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; flex-wrap: wrap; }
  .rec-id     { font-size: 12px; font-family: monospace; color: var(--text-secondary); font-weight: 600; }
  .rec-reason { margin: 0; font-size: 12px; color: var(--text-muted); line-height: 1.5; }

  .rec-priority {
    font-size: 10px; font-weight: 700;
    padding: 1px 6px; border-radius: 999px;
    text-transform: uppercase; letter-spacing: 0.3px;
  }
  .rec-priority.high   { background: rgba(248,113,113,0.12); color: #f87171; }
  .rec-priority.medium { background: rgba(251,191,36,0.12);  color: #fbbf24; }
  .rec-priority.low    { background: rgba(52,211,153,0.12);  color: #34d399; }

  .action-bar {
    display: flex; justify-content: flex-end;
    padding-top: 12px;
    border-top: var(--border-glass);
  }
  .error { color: var(--danger); font-size: 13px; }
</style>