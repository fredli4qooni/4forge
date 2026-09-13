<script lang="ts">
  import {
    ChevronDown, Database, ExternalLink, FileText, FolderOpen, Globe, Plus, Scan, ScrollText, Sparkles, Terminal, Zap,
  } from "@lucide/svelte";
  import type { ServiceItem, SiteItem } from "../types";

  let {
    services, sites, isLoading, allRunning, runningCount, stoppedCount, isScanningWorkspace,
    uptimeSeconds = 0, cpuPercent = 0, memoryMb = 0, onToggleAll, onToggleService, onOpenWeb,
    onOpenDatabase, onOpenTerminal, onOpenProjects, onOpenConfig, onOpenLogs, onOpenSiteBrowser,
    onOpenProjectTerminal, onOpenSiteFolder, onScanWorkspace, onOpenAddSite, onOpenCreateProject, onCreateDatabase,
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
    onToggleAll?: () => void;
    onToggleService: (id: string) => void;
    onOpenWeb: () => void;
    onOpenDatabase: (engine?: string) => void;
    onOpenTerminal: () => void;
    onOpenProjects: () => void;
    onOpenConfig: (id: string) => void;
    onOpenLogs: (id: string) => void;
    onOpenSiteBrowser: (domain: string) => void;
    onOpenProjectTerminal: (path: string) => void;
    onOpenSiteFolder: (path: string) => void;
    onScanWorkspace: () => void;
    onOpenAddSite: () => void;
    onOpenCreateProject?: () => void;
    onCreateDatabase: (dbName: string, engine?: string) => void;
  } = $props();

  let quickDbName = $state("");
  let selectedDbEngine = $state("MariaDB");
  let showDbQuickMenu = $state(false);

  function getServiceStatus(id: string): string {
    return services.find((s) => s.id === id)?.status || "stopped";
  }
  function getServiceLogo(id: string): string | undefined {
    return services.find((s) => s.id === id)?.logo;
  }

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

  function getDisplayTitle(svc: ServiceItem): string {
    if (svc.id === "caddy") return "Caddy";
    if (svc.id === "mariadb") return "MySQL / MariaDB";
    if (svc.id === "postgresql") return "PostgreSQL";
    if (svc.id === "mongodb") return "MongoDB";
    if (svc.id === "redis") return "Redis";
    if (svc.id === "php") return "PHP-FPM";
    if (svc.id === "node") return "Node.js";
    return svc.name;
  }

  function getServiceRole(id: string): string {
    if (id === "caddy") return "Web server";
    if (id === "mariadb") return "MySQL Database";
    if (id === "postgresql") return "PostgreSQL DB";
    if (id === "mongodb") return "NoSQL Document DB";
    if (id === "redis") return "Cache & Queue";
    if (id === "php") return "PHP runtime";
    if (id === "node") return "JS runtime";
    return "Service";
  }

  function getDisplayPort(svc: ServiceItem): string {
    if (!svc.ports) return ":--";
    const firstPort = svc.ports.split(",")[0].trim();
    if (firstPort === "Isolated") return "env";
    return `:${firstPort}`;
  }
</script>

<div class="w-full h-full flex flex-col bg-[#F8F9FA] text-slate-900 select-none overflow-hidden font-sans p-5 lg:px-8 lg:py-6 space-y-4">

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
              <div class="w-9 h-9 rounded-lg bg-white flex items-center justify-center border border-slate-200/80 shrink-0 p-1.5 shadow-xs">
                {#if svc.logo}
                  <img src={svc.logo} alt={svc.name} class="w-6 h-6 object-contain" />
                {:else if svc.icon}
                  <svc.icon class="w-4.5 h-4.5 text-slate-600" />
                {/if}
              </div>
              <div>
                <h3 class="font-semibold text-xs text-slate-900 leading-tight">{getDisplayTitle(svc)}</h3>
                <p class="text-[11px] text-slate-500 leading-tight mt-0.5">{getServiceRole(svc.id)}</p>
              </div>
            </div>

            <div class="flex items-center gap-1.5">
              <span class="font-mono text-[11px] text-slate-600 bg-slate-100 px-2 py-0.5 rounded border border-slate-200/80">
                {getDisplayPort(svc)}
              </span>

              <span class="text-[10px] font-medium px-2 py-0.5 rounded-full border {svc.status === 'running' ? 'bg-emerald-50 text-emerald-700 border-emerald-200' : 'bg-slate-100 text-slate-500 border-slate-200'}">
                {svc.status === 'running' ? 'Running' : 'Stopped'}
              </span>

              <button
                onclick={() => onToggleService(svc.id)}
                title={svc.status === 'running' ? 'Stop Service' : 'Start Service'}
                class="px-2.5 py-1 rounded-lg text-xs font-medium border shadow-xs transition-colors {svc.status === 'running' ? 'bg-rose-50 hover:bg-rose-100 text-rose-700 border-rose-200' : 'bg-slate-900 hover:bg-slate-800 text-white border-transparent'}"
              >
                {svc.status === 'running' ? 'Stop' : 'Start'}
              </button>

              <button
                onclick={() => onOpenConfig(svc.id)}
                title="Edit Configuration"
                class="p-1.5 rounded-lg text-slate-500 hover:text-slate-800 hover:bg-slate-100 border border-slate-200 transition-colors"
              >
                <FileText class="w-3.5 h-3.5" />
              </button>

              <button
                onclick={() => onOpenLogs(svc.id)}
                title="View Live Logs"
                class="p-1.5 rounded-lg text-slate-500 hover:text-slate-800 hover:bg-slate-100 border border-slate-200 transition-colors"
              >
                <ScrollText class="w-3.5 h-3.5" />
              </button>

              <button
                onclick={() => {
                  if (svc.id === 'caddy') onOpenWeb();
                  else if (svc.id === 'mariadb') onOpenDatabase('mariadb');
                  else if (svc.id === 'postgresql') onOpenDatabase('postgresql');
                  else if (svc.id === 'mongodb') onOpenDatabase('mongodb');
                  else if (svc.id === 'redis') onOpenDatabase('redis');
                }}
                title={svc.id === 'caddy' ? 'Open in Browser' : svc.id === 'redis' ? 'Open Redis CLI' : 'Open Database Client'}
                class="p-1.5 rounded-lg text-slate-500 hover:text-slate-800 hover:bg-slate-100 border border-slate-200 transition-colors"
              >
                {#if svc.id === 'caddy'}
                  <Globe class="w-3.5 h-3.5" />
                {:else if svc.id === 'redis'}
                  <Terminal class="w-3.5 h-3.5" />
                {:else}
                  <Database class="w-3.5 h-3.5" />
                {/if}
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="space-y-1.5 shrink-0">
      <h2 class="text-xs font-semibold text-slate-900">Quick access</h2>
      <div class="grid grid-cols-4 gap-2">
        <button
          onclick={onOpenWeb}
          class="rounded-xl border border-slate-200 bg-white p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 hover:border-slate-300 shadow-xs transition-all cursor-pointer group"
        >
          <Globe class="w-4 h-4 text-slate-600 group-hover:text-[#94380C] transition-colors" />
          <span class="text-xs font-medium text-slate-700 group-hover:text-slate-900">Web</span>
        </button>

        <div class="relative">
          <button
            onclick={() => (showDbQuickMenu = !showDbQuickMenu)}
            class="w-full rounded-xl border border-slate-200 bg-white p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 hover:border-slate-300 shadow-xs transition-all cursor-pointer group {showDbQuickMenu ? 'border-[#94380C]/50 ring-1 ring-[#94380C]/20 bg-slate-50' : ''}"
          >
            <Database class="w-4 h-4 text-slate-600 group-hover:text-[#94380C] transition-colors" />
            <div class="flex items-center gap-0.5">
              <span class="text-xs font-medium text-slate-700 group-hover:text-slate-900">Database</span>
              <ChevronDown class="w-3 h-3 text-slate-400 group-hover:text-slate-600 transition-transform {showDbQuickMenu ? 'rotate-180' : ''}" />
            </div>
          </button>

          {#if showDbQuickMenu}
            <button
              type="button"
              tabindex="-1"
              aria-label="Close menu backdrop"
              class="fixed inset-0 z-40 bg-transparent border-0 cursor-default p-0 m-0 w-full h-full"
              onclick={() => (showDbQuickMenu = false)}
            ></button>
            <div class="absolute bottom-full mb-2 -left-12 sm:left-0 w-64 bg-white rounded-xl border border-slate-200 shadow-xl p-2 z-50 space-y-1">
              <div class="px-2 py-1 border-b border-slate-100 flex items-center justify-between">
                <span class="text-[11px] font-semibold text-slate-900">Select Database</span>
                <span class="text-[10px] text-slate-400">Quick connect</span>
              </div>
              <button
                onclick={() => { showDbQuickMenu = false; onOpenDatabase("mariadb"); }}
                class="w-full p-2 rounded-lg flex items-center justify-between hover:bg-slate-50 transition-colors text-left group cursor-pointer"
              >
                <div class="flex items-center gap-2.5">
                  <div class="w-7 h-7 rounded-lg bg-white flex items-center justify-center border border-slate-200 shrink-0 p-1 shadow-xs">
                    {#if getServiceLogo('mariadb')}<img src={getServiceLogo('mariadb')} alt="MariaDB" class="w-5 h-5 object-contain" />{:else}<Database class="w-3.5 h-3.5 text-amber-700" />{/if}
                  </div>
                  <div>
                    <p class="text-xs font-medium text-slate-800 group-hover:text-slate-900">MySQL / MariaDB</p>
                    <p class="text-[10px] text-slate-400 font-mono">:3306 • root</p>
                  </div>
                </div>
                <span class="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded {getServiceStatus('mariadb') === 'running' ? 'bg-emerald-50 text-emerald-700' : 'bg-slate-100 text-slate-400'}">
                  {getServiceStatus('mariadb') === 'running' ? 'Active' : 'Offline'}
                </span>
              </button>

              <button
                onclick={() => { showDbQuickMenu = false; onOpenDatabase("postgresql"); }}
                class="w-full p-2 rounded-lg flex items-center justify-between hover:bg-slate-50 transition-colors text-left group cursor-pointer"
              >
                <div class="flex items-center gap-2.5">
                  <div class="w-7 h-7 rounded-lg bg-white flex items-center justify-center border border-slate-200 shrink-0 p-1 shadow-xs">
                    {#if getServiceLogo('postgresql')}<img src={getServiceLogo('postgresql')} alt="PostgreSQL" class="w-5 h-5 object-contain" />{:else}<Database class="w-3.5 h-3.5 text-blue-700" />{/if}
                  </div>
                  <div>
                    <p class="text-xs font-medium text-slate-800 group-hover:text-slate-900">PostgreSQL</p>
                    <p class="text-[10px] text-slate-400 font-mono">:5432 • postgres</p>
                  </div>
                </div>
                <span class="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded {getServiceStatus('postgresql') === 'running' ? 'bg-emerald-50 text-emerald-700' : 'bg-slate-100 text-slate-400'}">
                  {getServiceStatus('postgresql') === 'running' ? 'Active' : 'Offline'}
                </span>
              </button>

              <button
                onclick={() => { showDbQuickMenu = false; onOpenDatabase("mongodb"); }}
                class="w-full p-2 rounded-lg flex items-center justify-between hover:bg-slate-50 transition-colors text-left group cursor-pointer"
              >
                <div class="flex items-center gap-2.5">
                  <div class="w-7 h-7 rounded-lg bg-white flex items-center justify-center border border-slate-200 shrink-0 p-1 shadow-xs">
                    {#if getServiceLogo('mongodb')}<img src={getServiceLogo('mongodb')} alt="MongoDB" class="w-5 h-5 object-contain" />{:else}<Database class="w-3.5 h-3.5 text-emerald-700" />{/if}
                  </div>
                  <div>
                    <p class="text-xs font-medium text-slate-800 group-hover:text-slate-900">MongoDB</p>
                    <p class="text-[10px] text-slate-400 font-mono">:27017 • NoSQL</p>
                  </div>
                </div>
                <span class="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded {getServiceStatus('mongodb') === 'running' ? 'bg-emerald-50 text-emerald-700' : 'bg-slate-100 text-slate-400'}">
                  {getServiceStatus('mongodb') === 'running' ? 'Active' : 'Offline'}
                </span>
              </button>

              <button
                onclick={() => { showDbQuickMenu = false; onOpenDatabase("redis"); }}
                class="w-full p-2 rounded-lg flex items-center justify-between hover:bg-slate-50 transition-colors text-left group cursor-pointer"
              >
                <div class="flex items-center gap-2.5">
                  <div class="w-7 h-7 rounded-lg bg-white flex items-center justify-center border border-slate-200 shrink-0 p-1 shadow-xs">
                    {#if getServiceLogo('redis')}<img src={getServiceLogo('redis')} alt="Redis" class="w-5 h-5 object-contain" />{:else}<Zap class="w-3.5 h-3.5 text-rose-700" />{/if}
                  </div>
                  <div>
                    <p class="text-xs font-medium text-slate-800 group-hover:text-slate-900">Redis Cache</p>
                    <p class="text-[10px] text-slate-400 font-mono">:6379 • In-Memory</p>
                  </div>
                </div>
                <span class="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded {getServiceStatus('redis') === 'running' ? 'bg-emerald-50 text-emerald-700' : 'bg-slate-100 text-slate-400'}">
                  {getServiceStatus('redis') === 'running' ? 'Active' : 'Offline'}
                </span>
              </button>

              <button
                onclick={() => { showDbQuickMenu = false; onOpenDatabase("sqlite"); }}
                class="w-full p-2 rounded-lg flex items-center justify-between hover:bg-slate-50 transition-colors text-left group cursor-pointer"
              >
                <div class="flex items-center gap-2.5">
                  <div class="w-6 h-6 rounded-md bg-slate-100 text-slate-700 flex items-center justify-center border border-slate-200/60 shrink-0">
                    <FolderOpen class="w-3 h-3" />
                  </div>
                  <div>
                    <p class="text-xs font-medium text-slate-800 group-hover:text-slate-900">SQLite Files</p>
                    <p class="text-[10px] text-slate-400 font-mono">.sqlite / .db folder</p>
                  </div>
                </div>
                <span class="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded bg-slate-100 text-slate-600">
                  Folder
                </span>
              </button>
            </div>
          {/if}
        </div>

        <button
          onclick={onOpenTerminal}
          class="rounded-xl border border-slate-200 bg-white p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 hover:border-slate-300 shadow-xs transition-all cursor-pointer group"
        >
          <Terminal class="w-4 h-4 text-slate-600 group-hover:text-[#94380C] transition-colors" />
          <span class="text-xs font-medium text-slate-700 group-hover:text-slate-900">Terminal</span>
        </button>

        <button
          onclick={onOpenProjects}
          class="rounded-xl border border-slate-200 bg-white p-3 flex flex-col items-center justify-center gap-1.5 hover:bg-slate-50 hover:border-slate-300 shadow-xs transition-all cursor-pointer group"
        >
          <FolderOpen class="w-4 h-4 text-slate-600 group-hover:text-[#94380C] transition-colors" />
          <span class="text-xs font-medium text-slate-700 group-hover:text-slate-900">Root</span>
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
            <option value="MongoDB">MongoDB</option>
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
            class="px-2 py-1 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium shadow-xs flex items-center gap-1 transition-colors"
            title="Link an existing folder"
          >
            <FolderOpen class="w-3 h-3 text-slate-500" />
            <span>Link</span>
          </button>

          <button
            onclick={() => onOpenCreateProject?.()}
            class="px-2 py-1 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs flex items-center gap-1 transition-colors"
            title="Create new project from template"
          >
            <Sparkles class="w-3 h-3" />
            <span>New</span>
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
            <p class="text-[11px] text-slate-500 mt-0.5">Scaffold from a template or link an existing folder.</p>
          </div>
          <div class="flex items-center justify-center gap-2">
            <button
              onclick={() => onOpenCreateProject?.()}
              class="px-3.5 py-1.5 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs transition-colors inline-block"
            >
              + New Project
            </button>
            <button
              onclick={onOpenAddSite}
              class="px-3.5 py-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium shadow-xs transition-colors inline-block"
            >
              Link Folder
            </button>
          </div>
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
