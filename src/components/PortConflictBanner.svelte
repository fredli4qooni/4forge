<script lang="ts">
  import { Activity, X } from "@lucide/svelte";
  import type { PortCheckResult } from "../types";

  let {
    conflicts,
    dismissed,
    onApplyAlternative,
    onDismiss,
  }: {
    conflicts: PortCheckResult[];
    dismissed: boolean;
    onApplyAlternative: (conflict: PortCheckResult) => void;
    onDismiss: () => void;
  } = $props();
</script>

{#if conflicts.length > 0 && !dismissed}
  <div class="rounded-2xl border border-amber-500/40 bg-amber-500/10 p-4 flex items-center justify-between text-xs text-amber-200 shadow-xl shadow-amber-950/30">
    <div class="flex items-center gap-3">
      <div class="p-2.5 rounded-xl bg-amber-500/20 text-amber-400">
        <Activity class="w-5 h-5" />
      </div>
      <div>
        <h4 class="font-bold text-sm text-amber-300">Port Conflict Detected</h4>
        <p class="text-slate-300 mt-0.5">
          {conflicts.map((c) => `${c.service_name} (Port ${c.port} is occupied -> Suggested: ${c.alternative_port})`).join(" • ")}
        </p>
      </div>
    </div>
    <div class="flex items-center gap-3">
      {#if conflicts[0]?.alternative_port}
        <button
          onclick={() => onApplyAlternative(conflicts[0])}
          class="px-3 py-1.5 rounded-xl bg-gradient-to-r from-amber-500 to-amber-600 text-slate-950 font-bold hover:brightness-110 shadow-md transition-all"
        >
          Use Port {conflicts[0].alternative_port}
        </button>
      {/if}
      <button
        onclick={onDismiss}
        class="p-1.5 rounded-xl hover:bg-amber-500/20 text-amber-300 transition-colors"
        title="Dismiss warning"
      >
        <X class="w-4 h-4" />
      </button>
    </div>
  </div>
{/if}
