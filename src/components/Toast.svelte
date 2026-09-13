<script lang="ts">
  import { CheckCircle2 } from "@lucide/svelte";

  let { message }: { message: string | null } = $props();
</script>

{#if message}
  <div class="fixed bottom-6 right-6 z-50 flex items-center gap-2.5 px-4 py-2.5 rounded-xl bg-slate-900 border border-slate-800 text-white text-xs font-medium shadow-xl">
    <CheckCircle2 class="w-4 h-4 text-emerald-400" />
    <span>{message}</span>
  </div>
{/if}
