<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Zap } from "lucide-svelte";
  import SectionHeader from "../ui/SectionHeader.svelte";
  import TweakList from "../TweakList.svelte";
  import type { Tweak } from "$lib/types";

  export let tweaks: Tweak[] = [];

  async function applySafe() {
    for (const tweak of tweaks.filter(t => t.warning_level === "Safe")) {
      if (!tweak.enabled) {
        await invoke("apply_tweak", { id: tweak.id });
        tweak.enabled = true;
      }
    }
    tweaks = tweaks;
  }
</script>

<div class="section-container">
  <SectionHeader
    icon={Zap}
    title="Network MSI Mode"
    description="Enable Message Signaled Interrupts for lower latency and better stability."
    actionLabel="Apply Safe Tweaks"
    onAction={applySafe}
  />
  <div class="tweaks-wrapper">
    <TweakList {tweaks} showHeader={false} />
  </div>
</div>

<style>
  .section-container { height: 100%; display: flex; flex-direction: column; }
  .tweaks-wrapper { flex: 1; overflow-y: auto; overflow-x: hidden; padding-bottom: 24px; }
</style>