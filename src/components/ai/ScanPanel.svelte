<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Sparkles, AlertTriangle, Plus, Minus } from "lucide-svelte";
  import Button from "../ui/Button.svelte";
  import Badge from "../ui/Badge.svelte";

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

    {#each analysis.conflicts as c}
      <div class="conflict">
        <AlertTriangle size={13} />
        <span>{c.issue}</span>
        <small>{c.tweak_ids.join(" + ")}</small>
      </div>
    {/each}

    {#if analysis.remove.length}
      <h3><Minus size={13} /> Remove — counterproductive on your system</h3>
      {#each analysis.remove as r}
        <label class="rec-row danger">
          <input type="checkbox" bind:checked={r.selected} />
          <div>
            <strong>{r.id}</strong>
            <Badge level="careful" />
            <p>{r.reason}</p>
          </div>
        </label>
      {/each}
    {/if}

    {#if analysis.add.length}
      <h3><Plus size={13} /> Recommended for your system</h3>
      {#each analysis.add.sort((a, b) =>
        ["high","medium","low"].indexOf(a.priority) -
        ["high","medium","low"].indexOf(b.priority)
      ) as a}
        <label class="rec-row">
          <input type="checkbox" bind:checked={a.selected} />
          <div>
            <strong>{a.id}</strong>
            <span class="priority {a.priority}">{a.priority}</span>
            <p>{a.reason}</p>
          </div>
        </label>
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
  .conflict { display: flex; align-items: center; gap: 8px; padding: 10px 14px;
    background: rgba(251,191,36,0.08); border: 1px solid rgba(251,191,36,0.2);
    border-radius: var(--radius-sm); font-size: 13px; }
  h3 { font-size: 13px; font-weight: 600; color: var(--text-muted);
    display: flex; align-items: center; gap: 6px; margin: 8px 0 4px; }
  .rec-row { display: flex; align-items: flex-start; gap: 12px; padding: 12px 16px;
    background: var(--layer-card); border: var(--border-glass);
    border-radius: var(--radius-md); cursor: pointer; }
  .rec-row.danger { border-color: rgba(248,113,113,0.2); }
  .rec-row p { margin: 4px 0 0; font-size: 12px; color: var(--text-muted); }
  .priority { font-size: 10px; font-weight: 600; padding: 2px 7px; border-radius: 999px; }
  .priority.high   { background: rgba(248,113,113,0.15); color: #f87171; }
  .priority.medium { background: rgba(251,191,36,0.15); color: #fbbf24; }
  .priority.low    { background: rgba(52,211,153,0.15); color: #34d399; }
  .action-bar { padding-top: 8px; border-top: var(--border-glass); }
  .error { color: var(--danger); font-size: 13px; }
</style>