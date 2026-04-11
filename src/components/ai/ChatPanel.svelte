<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { marked } from "marked";
  import Button from "../ui/Button.svelte";

  marked.setOptions({ breaks: true, gfm: true });

  function md(text: string): string {
    return marked.parse(text) as string;
  }

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
        history: history.slice(0, -1),
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
      <div class="msg {msg.role}">
        {#if msg.role === 'model'}
          <div class="msg-content md-body">{@html md(msg.content)}</div>
        {:else}
          {msg.content}
        {/if}
      </div>
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
    align-self: flex-start; }
  .msg-model .msg-content { color: var(--text-secondary); }
  .typing { opacity: 0.6; font-style: italic; }
  .input-row { display: flex; gap: 8px; }
  .input-row input { flex: 1; background: rgba(255,255,255,0.04);
    border: var(--border-glass); border-radius: var(--radius-md);
    padding: 9px 13px; color: var(--text-color); font-size: 13px; outline: none; }
  .input-row input:focus { border-color: var(--accent-color); }

  :global(.md-body p)           { margin: 0 0 8px 0; }
  :global(.md-body p:last-child){ margin-bottom: 0; }
  :global(.md-body strong)      { color: var(--text-color); font-weight: 600; }
  :global(.md-body em)          { color: var(--text-secondary); font-style: italic; }
  :global(.md-body code) {
    background: rgba(255,255,255,0.08);
    border-radius: 4px;
    padding: 1px 5px;
    font-family: "Cascadia Code", "Consolas", monospace;
    font-size: 12px;
    color: var(--accent-hover);
  }
  :global(.md-body pre) {
    background: rgba(0,0,0,0.25);
    border: var(--border-glass);
    border-radius: var(--radius-md);
    padding: 12px 14px;
    overflow-x: auto;
    margin: 8px 0;
  }
  :global(.md-body pre code) { background: none; padding: 0; color: var(--text-secondary); }
  :global(.md-body ul), :global(.md-body ol) { padding-left: 18px; margin: 4px 0 8px; }
  :global(.md-body li)  { margin-bottom: 3px; font-size: 13px; }
  :global(.md-body h3)  { font-size: 14px; font-weight: 600; margin: 10px 0 4px; }
  :global(.md-body a)   { color: var(--accent-color); text-decoration: none; }
  :global(.md-body a:hover) { text-decoration: underline; }
</style>