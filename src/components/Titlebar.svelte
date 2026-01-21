<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  const appWindow = getCurrentWindow();
  let isMaximized = false;

  onMount(() => {
    let unlisten: () => void;

    (async () => {
      try {
        isMaximized = await appWindow.isMaximized();
        unlisten = await appWindow.onResized(async () => {
          isMaximized = await appWindow.isMaximized();
        });
      } catch (e) {
        console.error(e);
      }
    })();

    return () => {
      if (unlisten) unlisten();
    };
  });

  function minimize() {
    appWindow.minimize();
  }

  async function toggleMaximize() {
    await appWindow.toggleMaximize();
    isMaximized = !isMaximized;
  }

  function close() {
    appWindow.close();
  }
</script>

<div class="titlebar">
  <div class="drag-region" data-tauri-drag-region>
    <div class="window-title">Tunevex</div>
  </div>

  <div class="window-controls">
    <button class="control green" title="Maximize"></button>
    <button class="control yellow" on:click={minimize} title="Minimize"
    ></button>
    <button class="control red" on:click={close} title="Close"></button>
  </div>
</div>

<style>
  .titlebar {
    height: var(--titlebar-height, 32px);
    min-height: 32px;
    max-height: 32px;
    width: 100%;
    background: transparent !important;
    background-color: transparent !important;
    user-select: none;
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: relative;
    z-index: 9999;
    border-radius: var(--radius, 12px) var(--radius, 12px) 0 0;
    flex-shrink: 0;
    overflow: hidden;
  }

  .drag-region {
    flex: 1;
    height: 100%;
    display: flex;
    align-items: center;
    padding-left: 16px;
    /* When data-tauri-drag-region is present, this element handles window dragging. */
  }

  .window-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-color, #ccc);
    pointer-events: none;
  }

  .window-controls {
    display: flex;
    align-items: center;
    gap: 8px;
    padding-right: 16px;
    height: 100%;
    /* Ensure no drag region here so clicks work */
    -webkit-app-region: no-drag;
    z-index: 10;
  }

  .control {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: none;
    padding: 0;
    cursor: pointer;
    transition:
      transform 0.1s,
      opacity 0.2s;
    /* Explicitly mark buttons non-draggable just in case */
    -webkit-app-region: no-drag;
  }

  .control:hover {
    opacity: 0.8;
  }
  .control:active {
    transform: scale(0.9);
  }

  .green {
    background-color: #22c55e;
  }
  .yellow {
    background-color: #eab308;
  }
  .red {
    background-color: #ef4444;
  }
</style>
