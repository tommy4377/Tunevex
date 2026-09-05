<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Globe } from "lucide-svelte";
  import SectionHeader from "../ui/SectionHeader.svelte";
  import TweakList from "../TweakList.svelte";
  import type { Tweak } from "$lib/types";

  export let tweaks: Tweak[] = [];

  async function applyAllSafe() {
    for (const tweak of tweaks.filter((t) => t.warning_level === "Safe")) {
      if (!tweak.enabled) {
        tweak.enabled = await invoke<boolean | null>("apply_tweak", { id: tweak.id });
      }
    }
    tweaks = tweaks;
  }
</script>

<div class="section-container">
  <SectionHeader
    icon={Globe}
    title="Microsoft Edge"
    description="Disable Edge sidebar, first run experience, sync, telemetry, and auto-start."
    actionLabel="Apply Safe Tweaks"
    onAction={applyAllSafe}
  />
  <div class="tweaks-wrapper">
    <TweakList {tweaks} showHeader={false} />
  </div>
</div>

<style>
  .section-container { display: flex; flex-direction: column; height: 100%; }
  .tweaks-wrapper { flex: 1; overflow-y: auto; overflow-x: hidden; padding-bottom: 24px; }
</style>