─────────────────────────────────────────────────────
 1. profiler.rs — wire real RAM/CPU usage
─────────────────────────────────────────────────────
// Replace the two stub functions at the bottom of profiler.rs:

fn get_ram_usage_pct() -> f32 {
    use crate::modules::system::monitoring::get_quick_stats;
    get_quick_stats()
        .map(|s| {
            if s.ram_total > 0.0 {
                (s.ram_usage / s.ram_total) * 100.0
            } else {
                0.0
            }
        })
        .unwrap_or(0.0)
}

fn get_cpu_usage() -> f32 {
    use crate::modules::system::monitoring::get_quick_stats;
    get_quick_stats()
        .map(|s| s.cpu_usage)
        .unwrap_or(0.0)
}
// (check the exact struct field names in monitoring.rs — cpu_usage / ram_usage / ram_total)

─────────────────────────────────────────────────────
 2. ScanPanel.svelte — build the analyze UI
─────────────────────────────────────────────────────
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

─────────────────────────────────────────────────────
 3. ChatPanel.svelte
─────────────────────────────────────────────────────
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Button from "../ui/Button.svelte";

  interface Msg { role: "user" | "model"; content: string; }
  let history: Msg[] = [];
  let input = "";
  let loading = false;
  let chatEl: HTMLDivElement;

  async function send() {
    if (!input.trim() || loading) return;
    const msg = input.trim();
    history = [...history, { role: "user", content: msg }];
    input = "";
    loading = true;
    try {
      const reply = await invoke<string>("ai_chat", {
        message: msg,
        history: history.slice(0, -1),   // exclude the one we just pushed
      });
      history = [...history, { role: "model", content: reply }];
    } catch (e) {
      history = [...history, { role: "model", content: `Error: ${e}` }];
    } finally {
      loading = false;
      setTimeout(() => chatEl?.scrollTo(0, chatEl.scrollHeight), 50);
    }
  }
</script>

<div class="chat-panel">
  <div class="messages" bind:this={chatEl}>
    {#if history.length === 0}
      <p class="hint">Ask anything — "is my system optimized for gaming?",
        "why do I have stutters?", "is TCP No Delay safe for me?"</p>
    {/if}
    {#each history as msg}
      <div class="msg {msg.role}">{msg.content}</div>
    {/each}
    {#if loading}
      <div class="msg model typing">Thinking…</div>
    {/if}
  </div>
  <div class="input-row">
    <input
      bind:value={input}
      placeholder="Ask the AI…"
      onkeydown={(e) => e.key === "Enter" && send()}
      disabled={loading}
    />
    <Button onclick={send} disabled={loading || !input.trim()}>Send</Button>
  </div>
</div>

<style>
  .chat-panel { display: flex; flex-direction: column; height: 100%; gap: 12px; }
  .messages { flex: 1; overflow-y: auto; display: flex; flex-direction: column;
    gap: 10px; padding-right: 4px; }
  .hint { color: var(--text-muted); font-size: 13px; text-align: center; margin: auto; }
  .msg { padding: 10px 14px; border-radius: var(--radius-md);
    font-size: 13px; line-height: 1.5; max-width: 85%; white-space: pre-wrap; }
  .msg.user  { background: rgba(129,140,248,0.15); border: 1px solid rgba(129,140,248,0.25);
    align-self: flex-end; color: var(--text-color); }
  .msg.model { background: var(--layer-card); border: var(--border-glass);
    align-self: flex-start; color: var(--text-secondary); }
  .typing { opacity: 0.6; font-style: italic; }
  .input-row { display: flex; gap: 8px; }
  .input-row input { flex: 1; background: rgba(255,255,255,0.04);
    border: var(--border-glass); border-radius: var(--radius-md);
    padding: 9px 13px; color: var(--text-color); font-size: 13px; outline: none; }
  .input-row input:focus { border-color: var(--accent-color); }
</style>

─────────────────────────────────────────────────────
 4. DiagnosePanel.svelte
─────────────────────────────────────────────────────
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Search } from "lucide-svelte";
  import Button from "../ui/Button.svelte";

  interface Cause { tweak_id: string; confidence: "high"|"medium"|"low"; explanation: string; }
  interface Diagnosis { likely_causes: Cause[]; suggested_fix: string; safe_to_revert: string[]; }

  let problemText = "";
  let diagnosis: Diagnosis | null = null;
  let loading = false;
  let error = "";

  const confidenceColor = { high: "#f87171", medium: "#fbbf24", low: "#34d399" };

  async function diagnose() {
    if (!problemText.trim()) return;
    loading = true; error = "";
    try {
      diagnosis = await invoke<Diagnosis>("ai_diagnose", { problem: problemText });
    } catch (e) { error = e as string; }
    finally { loading = false; }
  }

  async function revertCauses() {
    if (!diagnosis) return;
    for (const id of diagnosis.safe_to_revert)
      await invoke("undo_tweak", { id });
  }
</script>

<div class="diagnose-panel">
  <textarea
    bind:value={problemText}
    placeholder="Describe your problem in detail:&#10;• Game stutters every 2 seconds&#10;• WiFi disconnects after sleep&#10;• High DPC latency in LatencyMon&#10;• Audio crackling during load"
    rows={5}
  />
  <Button onclick={diagnose} disabled={loading || !problemText.trim()}>
    <Search size={13} /> {loading ? "Analyzing…" : "Find Cause"}
  </Button>

  {#if error}<p class="error">{error}</p>{/if}

  {#if diagnosis}
    <h3>Likely causes</h3>
    {#each diagnosis.likely_causes as c}
      <div class="cause-card" style="border-color: {confidenceColor[c.confidence]}33">
        <div class="cause-header">
          <code>{c.tweak_id}</code>
          <span class="conf" style="color: {confidenceColor[c.confidence]}">{c.confidence}</span>
        </div>
        <p>{c.explanation}</p>
      </div>
    {/each}

    <div class="fix-box">
      <h3>Suggested fix</h3>
      <pre>{diagnosis.suggested_fix}</pre>
    </div>

    {#if diagnosis.safe_to_revert.length}
      <Button onclick={revertCauses}>
        Revert Likely Causes ({diagnosis.safe_to_revert.length})
      </Button>
    {/if}
  {/if}
</div>

<style>
  .diagnose-panel { display: flex; flex-direction: column; gap: 16px; }
  textarea { background: rgba(255,255,255,0.04); border: var(--border-glass);
    border-radius: var(--radius-md); padding: 12px; color: var(--text-color);
    font-size: 13px; resize: vertical; outline: none; font-family: inherit; }
  textarea:focus { border-color: var(--accent-color); }
  h3 { font-size: 13px; font-weight: 600; color: var(--text-muted); margin: 0; }
  .cause-card { padding: 12px 16px; background: var(--layer-card);
    border: 1px solid; border-radius: var(--radius-md); }
  .cause-header { display: flex; justify-content: space-between; margin-bottom: 6px; }
  code { font-size: 12px; color: var(--accent-color); }
  .conf { font-size: 11px; font-weight: 600; text-transform: uppercase; }
  .cause-card p { margin: 0; font-size: 13px; color: var(--text-muted); }
  .fix-box { background: var(--layer-card); border: var(--border-glass);
    border-radius: var(--radius-md); padding: 16px; }
  pre { margin: 8px 0 0; font-size: 12px; color: var(--text-secondary);
    white-space: pre-wrap; line-height: 1.6; }
  .error { color: var(--danger); font-size: 13px; }
</style>

─────────────────────────────────────────────────────
 5. HomeDashboard.svelte — AI Advisor entry point
    (open from home instead of from the sidebar)
─────────────────────────────────────────────────────

// In HomeDashboard.svelte, add at the top of <script>:
import { Sparkles } from "lucide-svelte";
import { activeCategory } from "$lib/stores";

// Add this button anywhere in the template, e.g. below the header div:
<button class="ai-cta" onclick={() => ($activeCategory = "AiAdvisor")}>
  <Sparkles size={18} />
  <div>
    <span class="ai-cta-title">AI Advisor</span>
    <span class="ai-cta-sub">Scan your system and get personalized recommendations</span>
  </div>
  <span class="ai-cta-arrow">→</span>
</button>

// Add to the <style> block:
.ai-cta {
  display: flex; align-items: center; gap: 14px;
  width: 100%; padding: 14px 18px;
  background: linear-gradient(135deg, rgba(129,140,248,0.12), rgba(129,140,248,0.04));
  border: 1px solid rgba(129,140,248,0.3);
  border-radius: var(--radius-card);
  color: var(--text-color); cursor: pointer;
  transition: all 0.2s;
}
.ai-cta:hover {
  background: linear-gradient(135deg, rgba(129,140,248,0.2), rgba(129,140,248,0.08));
  transform: translateY(-1px);
  box-shadow: 0 8px 20px rgba(129,140,248,0.15);
}
.ai-cta > :global(svg) { color: #818cf8; flex-shrink: 0; }
.ai-cta div { flex: 1; text-align: left; }
.ai-cta-title { display: block; font-size: 14px; font-weight: 600; color: #a5b4fc; }
.ai-cta-sub { display: block; font-size: 12px; color: var(--text-muted); margin-top: 2px; }
.ai-cta-arrow { font-size: 18px; color: rgba(129,140,248,0.5); }

// Also remove "AiAdvisor" from CategorySidebar.svelte's categories array
// so it no longer appears as a sidebar item.
─────────────────────────────────────────────────────