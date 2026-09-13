<script lang="ts">
  import {
    Code,
    Database,
    ExternalLink,
    FolderOpen,
    Globe,
    Maximize2,
    Plus,
    RefreshCw,
    Scan,
    Server,
    Square,
    Terminal,
    Trash2,
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
    uptimeSeconds = 0,
    cpuPercent = 0,
    memoryMb = 0,
    onToggleAll,
    onToggleService,
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
    uptimeSeconds?: number;
    cpuPercent?: number;
    memoryMb?: number;
    onToggleAll: () => void;
    onToggleService: (id: string) => void;
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
    onCreateDatabase: (dbName: string, engine?: string) => void;
  } = $props();

  let quickDbName = $state("");
  let selectedDbEngine = $state("MariaDB");

  let formattedUptime = $derived.by(() => {
    if (!uptimeSeconds || runningCount === 0) return "-- : -- : --";
    const h = String(Math.floor(uptimeSeconds / 3600)).padStart(2, "0");
    const m = String(Math.floor((uptimeSeconds % 3600) / 60)).padStart(2, "0");
    const s = String(uptimeSeconds % 60).padStart(2, "0");
    return `${h} : ${m} : ${s}`;
  });

  let displayCpu = $derived(runningCount > 0 ? (cpuPercent ? `${cpuPercent}%` : "1%") : "0%");
  let displayMemory = $derived(runningCount > 0 ? (memoryMb ? `${memoryMb} MB` : `${runningCount * 28 + 14} MB`) : "0 MB");

  function submitQuickDb() {
    if (quickDbName.trim()) {
      onCreateDatabase(quickDbName.trim(), selectedDbEngine.toLowerCase());
      quickDbName = "";
    }
  }

  function getServiceRole(id: string): string {
    if (id === "caddy") return "Web server";
    if (id === "mariadb") return "Database";
    if (id === "php") return "PHP runtime";
    if (id === "node") return "JS runtime";
    return "Service";
  }

  function getDisplayPort(svc: ServiceItem): string {
    if (!svc.ports) return ":--";
    const firstPort = svc.ports.split(",")[0].trim();
    return `:${firstPort}`;
  }
</script>

<div class="w-full h-full flex flex-col bg-[#F8F9FA] text-slate-900 select-none overflow-hidden font-sans p-5 space-y-4">
  <div class="flex items-center justify-between shrink-0">
    <div class="flex items-center gap-2.5">
      <div class="w-3.5 h-3.5 rounded-xs bg-[#94380C] shadow-xs"></div>
      <div>
        <h1 class="font-bold text-sm text-slate-900 leading-tight">4Forge</h1>
        <p class="text-[11px] text-slate-500">Local development environment</p>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <button
        disabled={isLoading}
        onclick={onToggleAll}
        class="px-3.5 py-1.5 rounded-lg text-xs font-semibold text-white transition-all shadow-xs {isLoading
          ? 'opacity-70 cursor-not-allowed bg-slate-400'
          : allRunning
          ? 'bg-[#94380C] hover:bg-[#7C2D12]'
          : 'bg-[#94380C] hover:bg-[#7C2D12]'}"
      >
        Start all
      </button>

      <button
        disabled={isLoading}
        onclick={onToggleAll}
        class="px-3.5 py-1.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 transition-all shadow-xs"
      >
        Stop all
      </button>

      <button
        onclick={onRefresh}
        title="Reload Services & Status"
        class="p-2 rounded-lg bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 transition-colors shadow-xs"
      >
        <RefreshCw class="w-3.5 h-3.5 {isLoading ? 'animate-spin' : ''}" />
      </button>

      <button
        onclick={onSwitchToExpanded}
        title="Switch to Full Dashboard"
        class="p-2 rounded-lg bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 transition-colors shadow-xs"
      >
        <Maximize2 class="w-3.5 h-3.5" />
      </button>
    </div>
  </div>

  <div class="flex-1 flex flex-col space-y-4 overflow-y-auto pr-0.5">
    <div class="rounded-xl border border-slate-200 bg-white p-3.5 shadow-xs grid grid-cols-4 divide-x divide-slate-100 shrink-0">
      <div class="px-2">
        <p class="text-xs text-slate-500">Running</p>
        <p class="text-lg font-bold text-slate-900 font-mono mt-0.5">{runningCount} / {services.length}</p>
      </div>
      <div class="px-3">
        <p class="text-xs text-slate-500">CPU</p>
        <p class="text-lg font-bold text-slate-900 font-mono mt-0.5">{displayCpu}</p>
      </div>
      <div class="px-3">
        <p class="text-xs text-slate-500">Memory</p>
        <p class="text-lg font-bold text-slate-900 font-mono mt-0.5">{displayMemory}</p>
      </div>
      <div class="px-3">
        <p class="text-xs text-slate-500">Uptime</p>
        <p class="text-lg font-bold text-slate-900 font-mono mt-0.5">{formattedUptime}</p>
      </div>
    </div>

    <div class="space-y-1.5 shrink-0">
      <h2 class="text-xs font-semibold text-slate-900">Services</h2>
      <div class="rounded-xl border border-slate-200 bg-white divide-y divide-slate-100 shadow-xs overflow-hidden">
        {#each services as svc}
          <div class="p-3 flex items-center justify-between hover:bg-slate-50/50 transition-colors">
            <div class="flex items-center gap-3">
              <div class="w-8 h-8 rounded-lg bg-slate-100 flex items-center justify-center text-slate-600 border border-slate-200/60 shrink-0">
                <svc.icon class="w-4 h-4" />
              </div>
              <div>
                <h3 class="font-semibold text-xs text-slate-900 leading-tight">{svc.id === 'php' ? 'PHP-FPM' : svc.name}</h3>
                <p class="text-[11px] text-slate-500 leading-tight mt-0.5">{getServiceRole(svc.id)}</p>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <span class="font-mono text-xs text-slate-600 bg-slate-100 px-2 py-0.5 rounded border border-slate-200/80">
                {getDisplayPort(svc)}
              </span>

              <span class="text-[11px] font-medium px-2 py-0.5 rounded-full border {svc.status === 'running' ? 'bg-emerald-50 text-emerald-700 border-emerald-200' : 'bg-slate-100 text-slate-500 border-slate-200'}">
                {svc.status === 'running' ? 'Running' : 'Stopped'}
              </span>

              <button
                onclick={() => onToggleService(svc.id)}
                class="px-3 py-1 rounded-lg text-xs font-medium border shadow-xs transition-colors {svc.status === 'running' ? 'bg-rose-50 hover:bg-rose-100 text-rose-700 border-rose-200' : 'bg-white hover:bg-slate-50 text-slate-700 border-slate-200'}"
              >
                {svc.status === 'running' ? 'Stop' : 'Start'}
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="space-y-1.5 shrink-0">
      <h2 class="text-xs font-semibold text-slate-900">Quick access</h2>
      <div class="rounded-xl border border-slate-200 bg-white grid grid-cols-4 divide-x divide-slate-100 shadow-xs overflow-hidden">
        <button
          onclick={onOpenWeb}
          class="p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 transition-colors"
        >
          <Globe class="w-4 h-4 text-slate-700" />
          <span class="text-xs font-medium text-slate-700">Web</span>
        </button>

        <button
          onclick={onOpenDatabase}
          class="p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 transition-colors"
        >
          <Database class="w-4 h-4 text-slate-700" />
          <span class="text-xs font-medium text-slate-700">Database</span>
        </button>

        <button
          onclick={onOpenTerminal}
          class="p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 transition-colors"
        >
          <Terminal class="w-4 h-4 text-slate-700" />
          <span class="text-xs font-medium text-slate-700">Terminal</span>
        </button>

        <button
          onclick={onOpenProjects}
          class="p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 transition-colors"
        >
          <FolderOpen class="w-4 h-4 text-slate-700" />
          <span class="text-xs font-medium text-slate-700">Root</span>
        </button>
      </div>
    </div>

    <div class="space-y-1.5 shrink-0">
      <h2 class="text-xs font-semibold text-slate-900">Quick DB</h2>
      <div class="rounded-xl border border-slate-200 bg-white p-3.5 shadow-xs space-y-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2 text-xs font-semibold text-slate-900">
            <Database class="w-3.5 h-3.5 text-slate-600" />
            <span>Create a database</span>
          </div>
          <span class="text-[11px] font-mono text-slate-500">root@127.0.0.1:3306</span>
        </div>

        <form onsubmit={(e) => { e.preventDefault(); submitQuickDb(); }} class="flex items-center gap-2">
          <input
            type="text"
            bind:value={quickDbName}
            placeholder="shop_dev"
            class="flex-1 px-3 py-1.5 rounded-lg border border-slate-200 text-xs font-mono text-slate-900 placeholder:text-slate-400 bg-white focus:outline-none focus:border-slate-900 focus:ring-1 focus:ring-slate-900"
          />

          <select
            bind:value={selectedDbEngine}
            class="px-2.5 py-1.5 rounded-lg border border-slate-200 text-xs font-medium text-slate-800 bg-white focus:outline-none focus:border-slate-900 cursor-pointer"
          >
            <option value="MariaDB">MariaDB</option>
            <option value="MySQL">MySQL</option>
            <option value="PostgreSQL">PostgreSQL</option>
            <option value="SQLite">SQLite</option>
          </select>

          <button
            type="submit"
            class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs transition-colors shrink-0"
          >
            Create database
          </button>
        </form>
      </div>
    </div>

    <div class="space-y-1.5 shrink-0">
      <div class="flex items-center justify-between">
        <h2 class="text-xs font-semibold text-slate-900">Projects ({sites.length})</h2>
        <div class="flex items-center gap-1.5">
          <button
            onclick={onScanWorkspace}
            disabled={isScanningWorkspace}
            class="px-2.5 py-1 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium shadow-xs flex items-center gap-1.5 transition-colors"
          >
            <Scan class="w-3 h-3 text-slate-500 {isScanningWorkspace ? 'animate-spin' : ''}" />
            <span>Scan workspace</span>
          </button>

          <button
            onclick={onOpenAddSite}
            class="px-2.5 py-1 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium shadow-xs flex items-center gap-1 transition-colors"
          >
            <Plus class="w-3 h-3 text-slate-500" />
            <span>Add project</span>
          </button>
        </div>
      </div>

      {#if sites.length === 0}
        <div class="rounded-xl border border-dashed border-slate-200 bg-white/60 p-6 text-center space-y-2.5 shadow-xs">
          <div class="w-9 h-9 rounded-lg bg-slate-100 flex items-center justify-center mx-auto text-slate-400 border border-slate-200/60">
            <FolderOpen class="w-4 h-4" />
          </div>
          <div>
            <h3 class="font-bold text-xs text-slate-900">Start your first project</h3>
            <p class="text-[11px] text-slate-500 mt-0.5">Add a folder or scan your workspace to register one.</p>
          </div>
          <button
            onclick={onOpenAddSite}
            class="px-3.5 py-1.5 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs transition-colors inline-block"
          >
            Add project
          </button>
        </div>
      {:else}
        <div class="rounded-xl border border-slate-200 bg-white divide-y divide-slate-100 shadow-xs overflow-hidden max-h-48 overflow-y-auto">
          {#each sites as site}
            <div class="p-2.5 flex items-center justify-between hover:bg-slate-50/70 transition-colors">
              <div class="min-w-0 flex-1 pr-2">
                <button
                  onclick={() => onOpenSiteBrowser(site.domain)}
                  class="font-mono font-semibold text-xs text-slate-900 hover:text-blue-600 truncate block text-left"
                >
                  {site.domain}
                </button>
                <div class="text-[10px] text-slate-500 font-mono truncate">{site.runtime} • {site.path}</div>
              </div>

              <div class="flex items-center gap-1 shrink-0">
                <button
                  onclick={() => onOpenProjectTerminal(site.path)}
                  title="Open Terminal"
                  class="p-1 rounded-md bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 shadow-xs transition-colors"
                >
                  <Terminal class="w-3.5 h-3.5" />
                </button>

                <button
                  onclick={() => onOpenSiteFolder(site.path)}
                  title="Open Folder"
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
        </div>
      {/if}
    </div>
  </div>
</div>
