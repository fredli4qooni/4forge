<script lang="ts">
  import { AlertTriangle, X } from "@lucide/svelte";
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
  <div class="rounded-xl border border-amber-200 bg-amber-50 p-4 flex items-center justify-between text-xs text-amber-900 shadow-xs">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-lg bg-amber-100 text-amber-700">
        <AlertTriangle class="w-4 h-4" />
      </div>
      <div>
        <h4 class="font-semibold text-xs text-amber-950">Port Conflict Detected</h4>
        <p class="text-amber-800 mt-0.5">
          {conflicts.map((c) => `${c.service_name} (Port ${c.port} occupied -> Suggested: ${c.alternative_port})`).join(" • ")}
        </p>
      </div>
    </div>
    <div class="flex items-center gap-2">
      {#if conflicts[0]?.alternative_port}
        <button
          onclick={() => onApplyAlternative(conflicts[0])}
          class="px-3 py-1.5 rounded-lg bg-amber-600 hover:bg-amber-700 text-white font-semibold shadow-xs transition-colors"
        >
          Use Port {conflicts[0].alternative_port}
        </button>
      {/if}
      <button
        onclick={onDismiss}
        class="p-1.5 rounded-lg hover:bg-amber-100 text-amber-700 transition-colors"
        title="Dismiss warning"
      >
        <X class="w-4 h-4" />
      </button>
    </div>
  </div>
{/if}
