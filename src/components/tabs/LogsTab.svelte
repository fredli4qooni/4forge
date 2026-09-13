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
      <h2 class="text-lg font-bold text-slate-900">Live Process Logs</h2>
      <p class="text-xs text-slate-500 mt-0.5">Real-time asynchronous streaming via LogHub.</p>
    </div>

    <div class="flex items-center gap-2.5">
      <input
        type="text"
        bind:value={logSearch}
        placeholder="Search log lines..."
        class="px-3 py-1.5 rounded-lg bg-white border border-slate-200 text-xs text-slate-900 focus:outline-none focus:border-slate-900 focus:ring-1 focus:ring-slate-900 placeholder:text-slate-400 shadow-xs"
      />
      <button
        onclick={onClearLogs}
        class="px-3 py-1.5 rounded-lg text-xs bg-white text-slate-700 hover:bg-slate-50 border border-slate-200 font-medium shadow-xs transition-colors"
      >
        Clear
      </button>
    </div>
  </div>

  <div class="flex items-center gap-1.5 flex-wrap">
    {#each ["all", "caddy", "mariadb", "postgresql", "redis", "php"] as filter}
      <button
        onclick={() => (logFilter = filter)}
        class="px-2.5 py-1 rounded-md text-xs font-medium uppercase tracking-wider transition-all {logFilter === filter
          ? 'bg-slate-900 text-white shadow-xs'
          : 'bg-white text-slate-600 border border-slate-200 hover:bg-slate-50 shadow-xs'}"
      >
        {filter}
      </button>
    {/each}
  </div>

  <div class="rounded-xl border border-slate-200 bg-white p-4 font-mono text-xs overflow-x-auto min-h-[440px] lg:min-h-[580px] max-h-[calc(100vh-210px)] overflow-y-auto space-y-1 shadow-xs">
    {#if filteredLogs.length === 0}
      <div class="text-slate-400 py-12 text-center italic">No log messages found for current filter.</div>
    {:else}
      {#each filteredLogs as log}
        <div class="flex items-start gap-3 py-1 leading-relaxed hover:bg-slate-50 px-2 rounded transition-colors">
          <span class="text-slate-400 shrink-0 select-none text-[11px]">
            {new Date(log.timestamp_millis).toLocaleTimeString()}
          </span>
          <span
            class="px-1.5 py-0.2 rounded text-[10px] font-semibold uppercase shrink-0 border {log.stream === 'stderr'
              ? 'bg-rose-50 text-rose-700 border-rose-200'
              : log.stream === 'system'
              ? 'bg-blue-50 text-blue-700 border-blue-200'
              : 'bg-slate-100 text-slate-700 border-slate-200'}"
          >
            {log.service}
          </span>
          <span class="text-slate-800 break-all text-[11px]">{log.message}</span>
        </div>
      {/each}
    {/if}
  </div>
</div>
