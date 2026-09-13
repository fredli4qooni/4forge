<script lang="ts">
  import { CheckCircle2 } from "@lucide/svelte";

  let { message }: { message: string | null } = $props();
</script>

{#if message}
  <div class="fixed bottom-6 right-6 z-50 flex items-center gap-2.5 px-4 py-2.5 rounded-xl bg-[#131D33] border border-cyan-500/40 text-cyan-300 text-xs shadow-2xl shadow-cyan-950 animate-bounce">
    <CheckCircle2 class="w-4 h-4 text-cyan-400" />
    <span>{message}</span>
  </div>
{/if}
