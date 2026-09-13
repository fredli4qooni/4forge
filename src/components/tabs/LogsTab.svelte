<script lang="ts">
  import type { LogMessage } from "../../types";

  let {
    logs,
    logFilter = $bindable("all"),
    logSearch = $bindable(""),
    onClearLogs
  }: {
    logs: LogMessage[];
    logFilter?: string;
    logSearch?: string;
    onClearLogs: () => void;
  } = $props();

  let filteredLogs = $derived(
    logs.filter((l) => {
      const matchFilter = logFilter === "all" || l.service.toLowerCase().includes(logFilter.toLowerCase());
      const matchSearch = !logSearch || l.message.toLowerCase().includes(logSearch.toLowerCase());
      return matchFilter && matchSearch;
    })
  );
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-xl font-bold text-white">Live Process Logs</h2>
      <p class="text-xs text-slate-400 mt-1">Real-time asynchronous streaming via LogHub.</p>
    </div>

    <div class="flex items-center gap-3">
      <input
        type="text"
        bind:value={logSearch}
        placeholder="Search log lines..."
        class="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-xs text-slate-200 focus:outline-none focus:border-cyan-500"
      />
      <button
        onclick={onClearLogs}
        class="px-3 py-1.5 rounded-lg text-xs bg-slate-800 text-slate-300 hover:bg-slate-700 border border-slate-700"
      >
        Clear
      </button>
    </div>
  </div>

  <div class="flex items-center gap-2">
    {#each ["all", "caddy", "mariadb", "php"] as filter}
      <button
        onclick={() => (logFilter = filter)}
        class="px-3 py-1 rounded-lg text-xs font-medium uppercase tracking-wider transition-all {logFilter === filter
          ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/40'
          : 'bg-slate-800/60 text-slate-400 border border-slate-800 hover:bg-slate-800'}"
      >
        {filter}
      </button>
    {/each}
  </div>

  <div class="rounded-xl border border-slate-800/90 bg-[#090D14] p-4 font-mono text-xs overflow-x-auto min-h-[420px] max-h-[560px] overflow-y-auto space-y-1.5 shadow-inner">
    {#if filteredLogs.length === 0}
      <div class="text-slate-500 py-12 text-center italic">No log messages found for current filter.</div>
    {:else}
      {#each filteredLogs as log}
        <div class="flex items-start gap-3 py-0.5 leading-relaxed hover:bg-slate-900/50 px-2 rounded">
          <span class="text-slate-500 shrink-0 select-none">
            {new Date(log.timestamp_millis).toLocaleTimeString()}
          </span>
          <span
            class="px-1.5 py-0.2 rounded text-[10px] font-semibold uppercase shrink-0 border {log.stream === 'stderr'
              ? 'bg-rose-500/15 text-rose-300 border-rose-500/30'
              : log.stream === 'system'
              ? 'bg-cyan-500/15 text-cyan-300 border-cyan-500/30'
              : 'bg-emerald-500/15 text-emerald-300 border-emerald-500/30'}"
          >
            {log.service}
          </span>
          <span class="text-slate-300 break-all">{log.message}</span>
        </div>
      {/each}
    {/if}
  </div>
</div>
