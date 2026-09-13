<script lang="ts">
  import {
    Activity,
    CheckCircle2,
    Database,
    ExternalLink,
    FolderKanban,
    FolderOpen,
    Globe,
    Maximize2,
    Play,
    Plus,
    RefreshCw,
    Scan,
    Server,
    ShieldCheck,
    Square,
    Terminal,
  } from "@lucide/svelte";
  import type { PortCheckResult, ServiceItem, SiteItem } from "../types";

  let {
    services,
    sites,
    isLoading,
    allRunning,
    runningCount,
    stoppedCount,
    isScanningWorkspace,
    onToggleAll,
    onOpenWeb,
    onOpenDatabase,
    onOpenTerminal,
    onOpenProjects,
    onRefresh,
    onSwitchToExpanded,
    onOpenSiteBrowser,
    onOpenProjectTerminal,
    onOpenSiteFolder,
    onScanWorkspace,
    onOpenAddSite,
    onCreateDatabase,
  }: {
    services: ServiceItem[];
    sites: SiteItem[];
    isLoading: boolean;
    allRunning: boolean;
    runningCount: number;
    stoppedCount: number;
    isScanningWorkspace: boolean;
    onToggleAll: () => void;
    onOpenWeb: () => void;
    onOpenDatabase: () => void;
    onOpenTerminal: () => void;
    onOpenProjects: () => void;
    onRefresh: () => void;
    onSwitchToExpanded: () => void;
    onOpenSiteBrowser: (domain: string) => void;
    onOpenProjectTerminal: (path: string) => void;
    onOpenSiteFolder: (path: string) => void;
    onScanWorkspace: () => void;
    onOpenAddSite: () => void;
    onCreateDatabase: (dbName: string) => void;
  } = $props();

  let quickDbName = $state("");

  function submitQuickDb() {
    if (quickDbName.trim()) {
      onCreateDatabase(quickDbName.trim());
      quickDbName = "";
    }
  }
</script>

<div class="w-full h-full flex flex-col bg-[#0C101A] text-slate-100 select-none overflow-hidden font-sans">
  <div class="h-14 px-5 border-b border-slate-800/80 bg-[#0F1626]/80 flex items-center justify-between shrink-0">
    <div class="flex items-center gap-2.5">
      <div class="w-7 h-7 rounded-lg bg-gradient-to-tr from-cyan-500 to-blue-600 flex items-center justify-center shadow-md shadow-cyan-500/20">
        <Server class="w-4 h-4 text-white" />
      </div>
      <div>
        <div class="flex items-center gap-1.5">
          <span class="font-bold text-sm text-white tracking-tight">4Forge</span>
          <span class="text-[9px] font-semibold uppercase px-1 py-0.2 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">Cockpit</span>
        </div>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <span class="text-[10px] text-emerald-400 font-mono flex items-center gap-1 bg-emerald-500/10 px-2 py-0.5 rounded-full border border-emerald-500/20">
        <ShieldCheck class="w-3 h-3" /> JobObject
      </span>
      <button
        onclick={onSwitchToExpanded}
        class="p-1.5 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors border border-transparent hover:border-slate-700"
        title="Switch to Full Dashboard"
      >
        <Maximize2 class="w-4 h-4" />
      </button>
    </div>
  </div>

  <div class="p-5 flex-1 flex flex-col justify-between space-y-4 overflow-y-auto">
    <div class="space-y-3">
      <div class="flex items-center gap-3">
        <button
          disabled={isLoading}
          onclick={onToggleAll}
          class="flex-1 py-3 px-4 rounded-2xl font-bold text-sm transition-all shadow-lg flex items-center justify-center gap-2 {isLoading
            ? 'opacity-70 cursor-not-allowed bg-slate-700 text-slate-300'
            : allRunning
            ? 'bg-rose-500/20 text-rose-300 border border-rose-500/40 hover:bg-rose-500/30'
            : runningCount > 0
            ? 'bg-gradient-to-r from-amber-500 to-emerald-600 text-white hover:brightness-110 shadow-amber-500/20'
            : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
        >
          {#if isLoading}
            <RefreshCw class="w-4 h-4 animate-spin" />
            <span>Processing...</span>
          {:else if allRunning}
            <Square class="w-4 h-4 fill-current" />
            <span>Stop All</span>
          {:else if runningCount > 0}
            <Play class="w-4 h-4 fill-current" />
            <span>Start All ({stoppedCount} stopped)</span>
          {:else}
            <Play class="w-4 h-4 fill-current" />
            <span>Start All</span>
          {/if}
        </button>

        <button
          onclick={onRefresh}
          title="Reload Status"
          class="p-3 rounded-2xl bg-slate-800/80 hover:bg-slate-800 text-slate-300 border border-slate-700/80 transition-colors"
        >
          <RefreshCw class="w-4 h-4" />
        </button>
      </div>

      <div class="grid grid-cols-4 gap-2">
        {#each services as svc}
          <div class="rounded-xl p-2 bg-[#121929]/70 border border-slate-800 text-center">
            <div class="text-[11px] font-bold text-white truncate">{svc.name.split(" ")[0]}</div>
            <div class="mt-0.5 flex items-center justify-center gap-1">
              <span class="w-1.5 h-1.5 rounded-full {svc.status === 'running' ? 'bg-emerald-400 animate-pulse' : 'bg-slate-600'}"></span>
              <span class="text-[10px] font-mono {svc.status === 'running' ? 'text-emerald-300' : 'text-slate-500'}">
                {svc.status === 'running' ? svc.ports.split(',')[0] : 'Off'}
              </span>
            </div>
          </div>
        {/each}
      </div>

      <div class="grid grid-cols-4 gap-2 pt-1">
        <button
          onclick={onOpenWeb}
          class="py-2 px-1 rounded-xl bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 flex flex-col items-center gap-1 text-xs transition-all hover:border-cyan-500/40"
        >
          <Globe class="w-4 h-4 text-cyan-400" />
          <span class="font-medium text-[11px] text-slate-300">Web</span>
        </button>

        <button
          onclick={onOpenDatabase}
          class="py-2 px-1 rounded-xl bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 flex flex-col items-center gap-1 text-xs transition-all hover:border-emerald-500/40"
        >
          <Database class="w-4 h-4 text-emerald-400" />
          <span class="font-medium text-[11px] text-slate-300">Database</span>
        </button>

        <button
          onclick={onOpenTerminal}
          class="py-2 px-1 rounded-xl bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 flex flex-col items-center gap-1 text-xs transition-all hover:border-amber-500/40"
        >
          <Terminal class="w-4 h-4 text-amber-400" />
          <span class="font-medium text-[11px] text-slate-300">Terminal</span>
        </button>

        <button
          onclick={onOpenProjects}
          class="py-2 px-1 rounded-xl bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 flex flex-col items-center gap-1 text-xs transition-all hover:border-blue-500/40"
        >
          <FolderKanban class="w-4 h-4 text-blue-400" />
          <span class="font-medium text-[11px] text-slate-300">Root</span>
        </button>
      </div>
    </div>

    <div class="rounded-2xl border border-slate-800/80 bg-[#101726]/60 p-3 space-y-2">
      <div class="flex items-center justify-between">
        <span class="text-[11px] font-semibold text-slate-300 uppercase tracking-wider flex items-center gap-1.5">
          <Database class="w-3.5 h-3.5 text-emerald-400" /> Quick DB
        </span>
        <span class="text-[10px] text-slate-500 font-mono">root@127.0.0.1:3306</span>
      </div>
      <form onsubmit={(e) => { e.preventDefault(); submitQuickDb(); }} class="flex items-center gap-1.5">
        <input
          type="text"
          bind:value={quickDbName}
          placeholder="New database name..."
          class="flex-1 px-2.5 py-1.5 rounded-lg bg-slate-900 border border-slate-700 text-xs text-slate-200 font-mono focus:outline-none focus:border-emerald-500"
        />
        <button
          type="submit"
          class="px-3 py-1.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold transition-colors"
        >
          Create
        </button>
      </form>
    </div>

    <div class="space-y-2">
      <div class="flex items-center justify-between text-xs">
        <span class="font-semibold text-slate-300">Projects ({sites.length})</span>
        <div class="flex items-center gap-1.5">
          <button
            onclick={onScanWorkspace}
            disabled={isScanningWorkspace}
            class="text-[11px] px-2 py-0.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 flex items-center gap-1 transition-colors"
          >
            <Scan class="w-3 h-3 text-cyan-400 {isScanningWorkspace ? 'animate-spin' : ''}" />
            <span>Scan</span>
          </button>
          <button
            onclick={onOpenAddSite}
            class="text-[11px] px-2 py-0.5 rounded bg-cyan-500/20 hover:bg-cyan-500/30 text-cyan-300 border border-cyan-500/30 flex items-center gap-1 transition-colors"
          >
            <Plus class="w-3 h-3" />
            <span>Add</span>
          </button>
        </div>
      </div>

      <div class="max-h-36 overflow-y-auto space-y-1.5 pr-0.5">
        {#if sites.length === 0}
          <div class="p-3 rounded-xl border border-dashed border-slate-800 text-center text-xs text-slate-500 italic">
            No projects registered. Click "Add" or "Scan" to begin.
          </div>
        {:else}
          {#each sites as site}
            <div class="p-2 rounded-xl bg-[#0E1524] border border-slate-800/90 flex items-center justify-between text-xs">
              <div class="min-w-0 flex-1 pr-2">
                <button
                  onclick={() => onOpenSiteBrowser(site.domain)}
                  class="font-mono font-semibold text-cyan-300 hover:underline truncate block text-left"
                >
                  {site.domain}
                </button>
                <div class="text-[10px] text-slate-500 font-mono truncate">{site.runtime} • {site.path}</div>
              </div>
              <div class="flex items-center gap-1 shrink-0">
                <button
                  onclick={() => onOpenProjectTerminal(site.path)}
                  title="Open Terminal in Project Directory"
                  class="p-1 rounded-lg bg-slate-800 hover:bg-slate-700 text-amber-400 border border-slate-700 transition-colors"
                >
                  <Terminal class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenSiteFolder(site.path)}
                  title="Open Project Folder"
                  class="p-1 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 transition-colors"
                >
                  <FolderOpen class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenSiteBrowser(site.domain)}
                  title="Open in Browser"
                  class="p-1 rounded-lg bg-slate-800 hover:bg-slate-700 text-cyan-400 border border-slate-700 transition-colors"
                >
                  <ExternalLink class="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>
