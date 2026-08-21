<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { marked } from "marked";
  import DOMPurify from "dompurify";
  import { History, Trash2, Plus, AlertTriangle, RotateCcw, Play } from "lucide-svelte";
  import type { Tweak } from "$lib/types";

  interface ChatAction {
    id: string;
    name: string;
    operation: "apply" | "undo";
    warning_level: "Safe" | "Careful" | "Dangerous";
    completed?: boolean;
  }
  interface ChatReply { content: string; tweak_actions: ChatAction[]; }
  interface Msg { role: string; content: string; timestamp: number; tweak_actions?: ChatAction[]; }
  interface SessionMeta {
    id: string; title: string; started_at: number; message_count: number;
  }

  marked.setOptions({ breaks: true, gfm: true });
  function md(text: string): string {
    return DOMPurify.sanitize(marked.parse(text) as string, {
      USE_PROFILES: { html: true },
      FORBID_TAGS: ["style", "iframe", "object", "embed", "form", "input"],
      FORBID_ATTR: ["style"],
    });
  }

  export let allTweaks: Tweak[] = [];

  let messages: Msg[] = [];
  let sessions: SessionMeta[] = [];
  let currentSessionId = crypto.randomUUID() as string;
  let showSidebar = false;
  let loading = false;
  let message = "";
  let error = "";
  let actionBusyId: string | null = null;

  onMount(async () => {
    sessions = await invoke<SessionMeta[]>("list_chats");
  });

  async function send() {
    if (!message.trim() || loading) return;
    const userMsg: Msg = { role: "user", content: message, timestamp: Date.now() };
    messages = [...messages, userMsg];
    const sent = message;
    message = "";
    loading = true;
    error = "";
    try {
      const reply = await invoke<ChatReply>("ai_chat", {
        message: sent,
        history: messages.slice(0, -1),
        sessionId: currentSessionId,
      });
      messages = [...messages, {
        role: "model",
        content: reply.content,
        timestamp: Date.now(),
        tweak_actions: reply.tweak_actions,
      }];
      sessions = await invoke<SessionMeta[]>("list_chats");
    } catch (e) {
      error = e as string;
    } finally {
      loading = false;
    }
  }

  async function executeTweakAction(action: ChatAction) {
    if (actionBusyId) return;
    const tweak = allTweaks.find((item) => item.id === action.id);
    if (!tweak) {
      error = `Tweak is no longer in the live catalog: ${action.id}`;
      return;
    }

    let dangerousAcknowledgement: string | null = null;
    if (action.operation === "apply" && action.warning_level === "Dangerous") {
      const expected = `APPLY ${action.id}`;
      dangerousAcknowledgement = prompt(
        `POWER USER CONTROL\n\n${tweak.name}\n\n${tweak.description}\n\nType ${expected} to apply.`
      );
      if (dangerousAcknowledgement !== expected) return;
    }

    actionBusyId = action.id;
    error = "";
    try {
      if (action.operation === "undo") {
        await invoke("undo_tweak", { id: action.id });
        tweak.enabled = false;
        action.operation = "apply";
      } else {
        await invoke("apply_tweak", { id: action.id, dangerousAcknowledgement });
        if (tweak.tweak_type === "Action") {
          action.completed = true;
        } else {
          tweak.enabled = true;
          action.operation = "undo";
        }
      }
      allTweaks = [...allTweaks];
      messages = [...messages];
    } catch (e) {
      error = `Could not ${action.operation} ${action.name}: ${e as string}`;
    } finally {
      actionBusyId = null;
    }
  }

  async function loadSession(id: string) {
    const session = await invoke<any>("load_chat", { id });
    messages = session.messages;
    currentSessionId = id;
    showSidebar = false;
  }

  async function deleteSession(id: string, e: MouseEvent) {
    e.stopPropagation();
    await invoke("delete_chat", { id });
    sessions = sessions.filter(s => s.id !== id);
    if (currentSessionId === id) newChat();
  }

  function newChat() {
    messages = [];
    currentSessionId = crypto.randomUUID();
  }

  function formatDate(ts: number) {
    return new Date(ts * 1000).toLocaleDateString("en-US", {
      month: "short", day: "numeric", hour: "2-digit", minute: "2-digit"
    });
  }
</script>

<div class="chat-layout">
  {#if showSidebar}
    <div class="history-sidebar">
      <div class="sidebar-header">
        <span>Chat History</span>
        <button class="new-chat-btn" on:click={newChat}>
          <Plus size={13}/> New
        </button>
      </div>
      <div class="session-list">
        {#each sessions as s}
          <div class="session-item" role="button" tabindex="0" on:click={() => loadSession(s.id)} on:keydown={(e) => e.key === "Enter" && loadSession(s.id)}>
            <div class="session-title">{s.title || "Untitled"}</div>
            <div class="session-meta">
              {formatDate(s.started_at)} · {s.message_count} msgs
            </div>
            <button class="del-btn" on:click={(e) => deleteSession(s.id, e)}>
              <Trash2 size={10}/>
            </button>
          </div>
        {/each}
        {#if sessions.length === 0}
          <p class="no-sessions">No saved chats yet</p>
        {/if}
      </div>
    </div>
  {/if}

  <div class="chat-main">
    <div class="chat-topbar">
      <button class="icon-btn" on:click={() => showSidebar = !showSidebar}
              title="Chat history">
        <History size={15}/>
        {#if sessions.length > 0}
          <span class="badge">{sessions.length}</span>
        {/if}
      </button>
      <button class="icon-btn" on:click={newChat} title="New chat">
        <Plus size={15}/>
      </button>
    </div>

    <div class="messages">
      {#if messages.length === 0}
        <p class="hint">Ask anything — "is my system optimized for gaming?",
          "why do I have stutters?", "is TCP No Delay safe for me?"</p>
      {/if}
      {#each messages as msg}
        <div class="message {msg.role}">
          <div class="msg-content md-body">
            {#if msg.role === 'model'}
              {@html md(msg.content)}
            {:else}
              {msg.content}
            {/if}
          </div>
          {#if msg.role === "model" && msg.tweak_actions?.length}
            <div class="tweak-actions" aria-label="AI referenced tweaks">
              {#each msg.tweak_actions as action (action.id)}
                <div class:dangerous={action.warning_level === "Dangerous"} class="tweak-action">
                  <div class="action-copy">
                    <span class="action-name">{action.name}</span>
                    <code>{action.id}</code>
                    {#if action.warning_level !== "Safe"}
                      <span class="risk"><AlertTriangle size={11}/>{action.warning_level}</span>
                    {/if}
                  </div>
                  <button
                    class:undo={action.operation === "undo"}
                    class:dangerous-action={action.warning_level === "Dangerous" && action.operation === "apply"}
                    disabled={actionBusyId !== null || action.completed}
                    on:click={() => executeTweakAction(action)}
                  >
                    {#if action.operation === "undo"}<RotateCcw size={12}/>{:else}<Play size={12}/>{/if}
                    {action.completed ? "Completed" : action.operation === "undo" ? "Undo" : "Apply"}
                  </button>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
      {#if loading}
        <div class="message model">
          <div class="msg-content typing">···</div>
        </div>
      {/if}
    </div>

    {#if error}
      <div class="error-box">{error}</div>
    {/if}

    <div class="input-row">
      <textarea
        bind:value={message}
        placeholder="Ask anything about your system…"
        on:keydown={(e) => e.key === "Enter" && !e.shiftKey && (e.preventDefault(), send())}
        rows={2}
      ></textarea>
      <button on:click={send} disabled={loading || !message.trim()}>Send</button>
    </div>
  </div>
</div>

<style>
  .chat-layout        { display: flex; height: 100%; overflow: hidden; }
  .history-sidebar    {
    width: 220px; flex-shrink: 0;
    background: var(--layer-card);
    border-right: var(--border-glass);
    display: flex; flex-direction: column;
    overflow: hidden;
  }
  .sidebar-header     {
    display: flex; align-items: center; justify-content: space-between;
    padding: 12px 14px; font-size: 12px; font-weight: 600;
    color: var(--text-secondary);
    border-bottom: var(--border-glass);
  }
  .new-chat-btn       {
    display: flex; align-items: center; gap: 4px;
    background: rgba(129,140,248,0.12); border: 1px solid rgba(129,140,248,0.2);
    border-radius: var(--radius-sm); padding: 3px 8px;
    color: var(--accent-color); font-size: 11px; cursor: pointer;
  }
  .session-list       { overflow-y: auto; flex: 1; padding: 6px; }
  .session-item       {
    width: 100%; text-align: left; background: none;
    border: 1px solid transparent; border-radius: var(--radius-md);
    padding: 8px 10px; cursor: pointer; position: relative;
    transition: background 0.15s, border-color 0.15s;
  }
  .session-item:hover { background: var(--layer-hover); border-color: rgba(255,255,255,0.08); }
  .session-title      { font-size: 12px; color: var(--text-color); font-weight: 500;
                        white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
                        max-width: 160px; }
  .session-meta       { font-size: 10px; color: var(--text-muted); margin-top: 2px; }
  .del-btn            {
    position: absolute; top: 6px; right: 6px;
    background: none; border: none; color: var(--text-muted);
    opacity: 0; cursor: pointer; transition: opacity 0.15s;
    padding: 2px;
  }
  .session-item:hover .del-btn { opacity: 1; }
  .del-btn:hover      { color: var(--danger); }
  .no-sessions        { font-size: 11px; color: var(--text-muted); text-align: center; padding: 20px; }
  .chat-main          { flex: 1; display: flex; flex-direction: column; overflow: hidden; }
  .chat-topbar        { display: flex; gap: 6px; padding: 8px 12px;
                        border-bottom: var(--border-glass); }
  .icon-btn           {
    background: none; border: 1px solid rgba(255,255,255,0.08);
    border-radius: var(--radius-sm); padding: 5px 8px;
    color: var(--text-muted); cursor: pointer;
    display: flex; align-items: center; gap: 5px; font-size: 11px;
    transition: color 0.15s, border-color 0.15s; position: relative;
  }
  .icon-btn:hover     { color: var(--accent-color); border-color: var(--accent-color); }
  .badge              {
    position: absolute; top: -4px; right: -4px;
    background: var(--accent-color); color: white;
    font-size: 9px; border-radius: 999px; padding: 1px 4px; line-height: 1;
  }
  .messages { flex: 1; overflow-y: auto; display: flex; flex-direction: column;
    gap: 10px; padding: 12px; }
  .hint { color: var(--text-muted); font-size: 13px; text-align: center; margin: auto; }
  .message { padding: 10px 14px; border-radius: var(--radius-md); min-width: 0;
    font-size: 13px; line-height: 1.4; max-width: min(85%, 640px); white-space: pre-wrap; word-break: break-word; overflow-wrap: break-word; }
  .message.user  { background: rgba(129,140,248,0.15); border: 1px solid rgba(129,140,248,0.25);
    align-self: flex-end; color: var(--text-color); }
  .message.model { background: var(--layer-card); border: var(--border-glass);
    align-self: flex-start; }
  .msg-content { color: var(--text-secondary); }
  .tweak-actions { display: grid; gap: 6px; margin-top: 10px; width: min(520px, 100%); min-width: 0; }
  .tweak-action { display: flex; align-items: center; justify-content: space-between; gap: 10px;
    padding: 8px 9px; border: 1px solid rgba(255,255,255,0.09); border-radius: var(--radius-sm);
    background: rgba(255,255,255,0.025); }
  .tweak-action.dangerous { border-color: rgba(248,113,113,0.28); }
  .action-copy { display: flex; flex-wrap: wrap; align-items: center; gap: 5px 7px; min-width: 0; }
  .action-name { width: 100%; color: var(--text-color); font-size: 12px; font-weight: 600; }
  .action-copy code { color: var(--text-muted); font-size: 10px; overflow-wrap: anywhere; }
  .risk { display: inline-flex; align-items: center; gap: 3px; color: #fbbf24; font-size: 10px; }
  .tweak-action button { display: inline-flex; align-items: center; gap: 5px; flex-shrink: 0;
    padding: 6px 9px; border-radius: var(--radius-sm); border: 1px solid rgba(129,140,248,0.35);
    background: rgba(129,140,248,0.13); color: var(--accent-hover); cursor: pointer; font-size: 11px; }
  .tweak-action button.undo { border-color: rgba(251,191,36,0.35); color: #fbbf24; background: rgba(251,191,36,0.08); }
  .tweak-action button.dangerous-action { border-color: rgba(248,113,113,0.4); color: #f87171; background: rgba(248,113,113,0.1); }
  .tweak-action button:disabled { opacity: 0.5; cursor: not-allowed; }
  .typing { letter-spacing: 4px; color: var(--text-muted); }
  .error-box { padding: 8px 12px; background: rgba(248,113,113,0.1);
    border: 1px solid rgba(248,113,113,0.2); border-radius: var(--radius-sm);
    color: #f87171; font-size: 12px; margin: 0 12px; }
  .input-row { display: flex; gap: 8px; padding: 12px; border-top: var(--border-glass); }
  .input-row textarea { flex: 1; background: rgba(255,255,255,0.04);
    border: var(--border-glass); border-radius: var(--radius-md);
    padding: 9px 13px; color: var(--text-color); font-size: 13px; outline: none;
    resize: none; }
  .input-row textarea:focus { border-color: var(--accent-color); }
  .input-row button { padding: 9px 18px; background: var(--accent-color);
    border: none; border-radius: var(--radius-md); color: white;
    font-size: 13px; cursor: pointer; }
  .input-row button:disabled { opacity: 0.5; cursor: not-allowed; }

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

  @media (max-width: 940px) {
    .history-sidebar { width: 176px; }
    .session-title { max-width: 120px; }
    .message { max-width: 92%; }
    .tweak-action { align-items: flex-start; flex-wrap: wrap; }
  }
</style>
