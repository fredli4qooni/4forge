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
  import type { ServiceItem, SiteItem } from "../types";

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

<div class="w-full h-full flex flex-col bg-[#F8FAFC] text-slate-900 select-none overflow-hidden font-sans">
  <div class="h-14 px-5 border-b border-slate-200 bg-white flex items-center justify-between shrink-0">
    <div class="flex items-center gap-2.5">
      <div class="w-7 h-7 rounded-lg bg-slate-900 flex items-center justify-center shadow-xs">
        <Server class="w-4 h-4 text-white" />
      </div>
      <div class="flex items-center gap-1.5">
        <span class="font-bold text-sm text-slate-900 tracking-tight">4Forge</span>
        <span class="text-[9px] font-semibold uppercase px-1.5 py-0.5 rounded bg-slate-100 text-slate-600 border border-slate-200">Cockpit</span>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <span class="text-[10px] text-emerald-700 font-mono flex items-center gap-1 bg-emerald-50 px-2 py-0.5 rounded border border-emerald-200 font-medium">
        <ShieldCheck class="w-3 h-3 text-emerald-600" /> JobObject
      </span>
      <button
        onclick={onSwitchToExpanded}
        class="p-1.5 rounded-lg text-slate-500 hover:text-slate-900 hover:bg-slate-100 border border-slate-200 transition-colors shadow-xs"
        title="Switch to Full Dashboard"
      >
        <Maximize2 class="w-4 h-4" />
      </button>
    </div>
  </div>

  <div class="p-5 flex-1 flex flex-col justify-between space-y-4 overflow-y-auto">
    <div class="space-y-3">
      <div class="flex items-center gap-2.5">
        <button
          disabled={isLoading}
          onclick={onToggleAll}
          class="flex-1 py-2.5 px-4 rounded-xl font-semibold text-xs transition-all shadow-xs flex items-center justify-center gap-2 {isLoading
            ? 'opacity-70 cursor-not-allowed bg-slate-100 text-slate-400 border border-slate-200'
            : allRunning
            ? 'bg-rose-50 text-rose-700 border border-rose-200 hover:bg-rose-100'
            : runningCount > 0
            ? 'bg-slate-900 text-white hover:bg-slate-800'
            : 'bg-slate-900 text-white hover:bg-slate-800'}"
        >
          {#if isLoading}
            <RefreshCw class="w-4 h-4 animate-spin text-slate-400" />
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
          class="p-2.5 rounded-xl bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 transition-colors shadow-xs"
        >
          <RefreshCw class="w-4 h-4" />
        </button>
      </div>

      <div class="grid grid-cols-4 gap-2">
        {#each services as svc}
          <div class="rounded-xl p-2 bg-white border border-slate-200 text-center shadow-xs">
            <div class="text-[11px] font-semibold text-slate-800 truncate">{svc.name.split(" ")[0]}</div>
            <div class="mt-0.5 flex items-center justify-center gap-1">
              <span class="w-1.5 h-1.5 rounded-full {svc.status === 'running' ? 'bg-emerald-500' : 'bg-slate-300'}"></span>
              <span class="text-[10px] font-mono {svc.status === 'running' ? 'text-emerald-700 font-medium' : 'text-slate-400'}">
                {svc.status === 'running' ? svc.ports.split(',')[0] : 'Off'}
              </span>
            </div>
          </div>
        {/each}
      </div>

      <div class="grid grid-cols-4 gap-2 pt-1">
        <button
          onclick={onOpenWeb}
          class="py-2 px-1 rounded-xl bg-white hover:bg-slate-50 border border-slate-200 flex flex-col items-center gap-1 text-xs transition-all shadow-xs hover:border-slate-300"
        >
          <Globe class="w-4 h-4 text-slate-600" />
          <span class="font-medium text-[11px] text-slate-700">Web</span>
        </button>

        <button
          onclick={onOpenDatabase}
          class="py-2 px-1 rounded-xl bg-white hover:bg-slate-50 border border-slate-200 flex flex-col items-center gap-1 text-xs transition-all shadow-xs hover:border-slate-300"
        >
          <Database class="w-4 h-4 text-slate-600" />
          <span class="font-medium text-[11px] text-slate-700">Database</span>
        </button>

        <button
          onclick={onOpenTerminal}
          class="py-2 px-1 rounded-xl bg-white hover:bg-slate-50 border border-slate-200 flex flex-col items-center gap-1 text-xs transition-all shadow-xs hover:border-slate-300"
        >
          <Terminal class="w-4 h-4 text-slate-600" />
          <span class="font-medium text-[11px] text-slate-700">Terminal</span>
        </button>

        <button
          onclick={onOpenProjects}
          class="py-2 px-1 rounded-xl bg-white hover:bg-slate-50 border border-slate-200 flex flex-col items-center gap-1 text-xs transition-all shadow-xs hover:border-slate-300"
        >
          <FolderKanban class="w-4 h-4 text-slate-600" />
          <span class="font-medium text-[11px] text-slate-700">Root</span>
        </button>
      </div>
    </div>

    <div class="rounded-xl border border-slate-200 bg-white p-3 space-y-2 shadow-xs">
      <div class="flex items-center justify-between">
        <span class="text-[11px] font-semibold text-slate-700 uppercase tracking-wider flex items-center gap-1.5">
          <Database class="w-3.5 h-3.5 text-slate-600" /> Quick DB
        </span>
        <span class="text-[10px] text-slate-400 font-mono">root@127.0.0.1:3306</span>
      </div>
      <form onsubmit={(e) => { e.preventDefault(); submitQuickDb(); }} class="flex items-center gap-1.5">
        <input
          type="text"
          bind:value={quickDbName}
          placeholder="New database name..."
          class="flex-1 px-2.5 py-1.5 rounded-lg bg-slate-50 border border-slate-200 text-xs text-slate-900 font-mono focus:bg-white focus:border-slate-900 focus:outline-none"
        />
        <button
          type="submit"
          class="px-3 py-1.5 rounded-lg bg-slate-900 hover:bg-slate-800 text-white text-xs font-semibold shadow-xs transition-colors"
        >
          Create
        </button>
      </form>
    </div>

    <div class="space-y-2">
      <div class="flex items-center justify-between text-xs">
        <span class="font-semibold text-slate-700">Projects ({sites.length})</span>
        <div class="flex items-center gap-1.5">
          <button
            onclick={onScanWorkspace}
            disabled={isScanningWorkspace}
            class="text-[11px] px-2 py-0.5 rounded bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 flex items-center gap-1 shadow-xs transition-colors"
          >
            <Scan class="w-3 h-3 text-slate-500 {isScanningWorkspace ? 'animate-spin' : ''}" />
            <span>Scan</span>
          </button>
          <button
            onclick={onOpenAddSite}
            class="text-[11px] px-2 py-0.5 rounded bg-slate-900 hover:bg-slate-800 text-white flex items-center gap-1 shadow-xs transition-colors"
          >
            <Plus class="w-3 h-3" />
            <span>Add</span>
          </button>
        </div>
      </div>

      <div class="max-h-36 overflow-y-auto space-y-1.5 pr-0.5">
        {#if sites.length === 0}
          <div class="p-3 rounded-xl border border-dashed border-slate-200 bg-white text-center text-xs text-slate-400 italic">
            No projects registered. Click "Add" or "Scan" to begin.
          </div>
        {:else}
          {#each sites as site}
            <div class="p-2 rounded-lg bg-white border border-slate-200 flex items-center justify-between text-xs shadow-xs hover:border-slate-300 transition-all">
              <div class="min-w-0 flex-1 pr-2">
                <button
                  onclick={() => onOpenSiteBrowser(site.domain)}
                  class="font-mono font-semibold text-slate-900 hover:text-blue-600 truncate block text-left"
                >
                  {site.domain}
                </button>
                <div class="text-[10px] text-slate-500 font-mono truncate">{site.runtime} • {site.path}</div>
              </div>
              <div class="flex items-center gap-1 shrink-0">
                <button
                  onclick={() => onOpenProjectTerminal(site.path)}
                  title="Open Terminal in Project Directory"
                  class="p-1 rounded-md bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 shadow-xs transition-colors"
                >
                  <Terminal class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenSiteFolder(site.path)}
                  title="Open Project Folder"
                  class="p-1 rounded-md bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 shadow-xs transition-colors"
                >
                  <FolderOpen class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenSiteBrowser(site.domain)}
                  title="Open in Browser"
                  class="p-1 rounded-md bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 shadow-xs transition-colors"
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
