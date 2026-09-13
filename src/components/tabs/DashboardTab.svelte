<script lang="ts">
  import {
    CheckCircle2,
    Code,
    Database,
    ExternalLink,
    FileText,
    FolderOpen,
    Globe,
    Plus,
    Power,
    Scan,
    ScrollText,
    Trash2,
  } from "@lucide/svelte";
  import type { PortCheckResult, ServiceItem, SiteItem } from "../../types";

  let {
    services,
    sites,
    portConflicts,
    activePhpVersion,
    activeNodeVersion,
    isScanningWorkspace,
    onNavigateServices,
    onToggleService,
    onOpenWeb,
    onOpenDatabase,
    onOpenConfig,
    onOpenLogs,
    onSelectPhpVersion,
    onSelectNodeVersion,
    onScanWorkspace,
    onOpenAddSite,
    onOpenSiteBrowser,
    onOpenSiteFolder,
    onOpenProjectInVsCode,
    onDeleteSite,
  }: {
    services: ServiceItem[];
    sites: SiteItem[];
    portConflicts: PortCheckResult[];
    activePhpVersion: string;
    activeNodeVersion: string;
    isScanningWorkspace: boolean;
    onNavigateServices: () => void;
    onToggleService: (id: string) => void;
    onOpenWeb: () => void;
    onOpenDatabase: () => void;
    onOpenConfig: (id: string) => void;
    onOpenLogs: (id: string) => void;
    onSelectPhpVersion: (v: string) => void;
    onSelectNodeVersion: (v: string) => void;
    onScanWorkspace: () => void;
    onOpenAddSite: () => void;
    onOpenSiteBrowser: (d: string) => void;
    onOpenSiteFolder: (p: string) => void;
    onOpenProjectInVsCode: (p: string) => void;
    onDeleteSite: (d: string) => void;
  } = $props();
</script>

<div class="space-y-8">
  <div class="relative overflow-hidden rounded-2xl border border-slate-800/90 bg-gradient-to-br from-slate-900/90 via-[#101827] to-[#0D1321] p-6 shadow-xl">
    <div class="absolute -top-24 -right-24 w-64 h-64 bg-cyan-500/10 rounded-full blur-3xl pointer-events-none"></div>
    <div class="flex items-center justify-between relative z-10">
      <div>
        <div class="flex items-center gap-2 mb-2">
          <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-emerald-500/15 text-emerald-400 border border-emerald-500/30">
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
            System Operational
          </span>
          {#if portConflicts.length > 0}
            <span class="text-xs text-amber-400 font-medium">{portConflicts.length} port conflict detected</span>
          {:else}
            <span class="text-xs text-slate-400">Zero port conflicts detected</span>
          {/if}
        </div>
        <h2 class="text-xl font-bold text-white tracking-tight">Polyglot Development Stack Active</h2>
        <p class="text-xs text-slate-400 mt-1 max-w-xl">
          Caddy auto-HTTPS proxy, multi-database architecture (MariaDB, PostgreSQL, SQLite), and isolated runtimes (PHP & Node.js) integrated natively.
        </p>
      </div>

      <div class="flex items-center gap-6">
        <div class="text-right">
          <p class="text-xs text-slate-400">Virtual Hosts</p>
          <p class="text-2xl font-bold text-cyan-400 font-mono">{sites.length}</p>
        </div>
        <div class="h-10 w-px bg-slate-800"></div>
        <div class="text-right">
          <p class="text-xs text-slate-400">Active PHP</p>
          <p class="text-2xl font-bold text-emerald-400 font-mono">v{activePhpVersion}</p>
        </div>
      </div>
    </div>
  </div>

  <div>
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-semibold text-slate-200 uppercase tracking-wider">Services Status</h3>
      <button onclick={onNavigateServices} class="text-xs text-cyan-400 hover:underline font-medium">View All Services &rarr;</button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {#each services as service}
        <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/70 hover:bg-[#121929]/90 p-5 flex flex-col justify-between transition-all hover:border-slate-700/80 shadow-lg">
          <div>
            <div class="flex items-center justify-between mb-3">
              <div class="p-2.5 rounded-xl bg-slate-800/70 text-cyan-400 border border-slate-700/50">
                <service.icon class="w-5 h-5" />
              </div>
              {#if service.status === 'running'}
                <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-emerald-500/15 text-emerald-300 border border-emerald-500/30 shadow-sm shadow-emerald-950/40">
                  <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                  Running
                </span>
              {:else}
                <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-medium bg-slate-800/80 text-slate-400 border border-slate-700/60">
                  <span class="w-1.5 h-1.5 rounded-full bg-slate-500"></span>
                  Stopped
                </span>
              {/if}
            </div>

            <h4 class="font-bold text-sm text-white mb-0.5">{service.name}</h4>
            <p class="text-xs text-slate-400 mb-3">{service.type}</p>

            <div class="space-y-2 pt-2 border-t border-slate-800/70 text-xs text-slate-400">
              <div class="flex justify-between items-center">
                <span>Ports:</span>
                <span class="text-cyan-400 font-mono text-[11px] font-semibold bg-cyan-950/30 px-1.5 py-0.5 rounded border border-cyan-500/20">{service.ports}</span>
              </div>
              {#if service.pid}
                <div class="flex justify-between items-center text-[11px]">
                  <span>Process PID:</span>
                  <span class="text-emerald-400 font-mono font-bold bg-emerald-950/60 px-1.5 py-0.5 rounded border border-emerald-500/30">#{service.pid}</span>
                </div>
              {/if}

              {#if service.id === "php"}
                <div class="flex items-center justify-between pt-1 border-t border-slate-800/50 text-xs">
                  <span class="text-slate-400 font-medium">PHP Version:</span>
                  <select
                    value={activePhpVersion}
                    onchange={(e) => onSelectPhpVersion((e.target as HTMLSelectElement).value)}
                    class="bg-slate-900 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 rounded-lg px-2 py-0.5 text-xs font-mono font-bold transition-colors cursor-pointer focus:outline-none"
                  >
                    <option value="8.4.3">8.4.3</option>
                    <option value="8.3.16">8.3.16</option>
                    <option value="8.2.27">8.2.27</option>
                  </select>
                </div>
              {:else if service.id === "node"}
                <div class="flex items-center justify-between pt-1 border-t border-slate-800/50 text-xs">
                  <span class="text-slate-400 font-medium">Node Version:</span>
                  <select
                    value={activeNodeVersion}
                    onchange={(e) => onSelectNodeVersion((e.target as HTMLSelectElement).value)}
                    class="bg-slate-900 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 rounded-lg px-2 py-0.5 text-xs font-mono font-bold transition-colors cursor-pointer focus:outline-none"
                  >
                    <option value="22.14.0">22.14.0</option>
                    <option value="20.18.2">20.18.2</option>
                  </select>
                </div>
              {/if}
            </div>
          </div>

          <div class="mt-4 pt-3 border-t border-slate-800/60 space-y-2">
            <div class="flex items-center gap-2">
              <button
                onclick={() => onToggleService(service.id)}
                class="flex-1 py-1.5 px-3 rounded-xl text-xs font-bold transition-all flex items-center justify-center gap-1.5 shadow-sm {service.status === 'running'
                  ? 'bg-rose-500/20 text-rose-300 hover:bg-rose-500/30 border border-rose-500/40 shadow-rose-950/30'
                  : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
              >
                <Power class="w-3.5 h-3.5" />
                <span>{service.status === "running" ? "Stop" : "Start"}</span>
              </button>
              {#if service.id === "caddy"}
                <button
                  onclick={onOpenWeb}
                  title="Open in Browser"
                  class="p-1.5 rounded-xl bg-slate-800/70 hover:bg-slate-800 text-cyan-400 border border-slate-700/60 hover:border-cyan-500/40 transition-all"
                >
                  <ExternalLink class="w-3.5 h-3.5" />
                </button>
              {:else if service.id === "mariadb"}
                <button
                  onclick={onOpenDatabase}
                  title="Open Database GUI"
                  class="p-1.5 rounded-xl bg-slate-800/70 hover:bg-slate-800 text-emerald-400 border border-slate-700/60 hover:border-emerald-500/40 transition-all"
                >
                  <Database class="w-3.5 h-3.5" />
                </button>
              {/if}
            </div>
            <div class="grid grid-cols-2 gap-1.5">
              <button
                onclick={() => onOpenConfig(service.id)}
                title="Edit Configuration in Notepad"
                class="w-full py-1 px-2 rounded-lg text-[11px] font-medium bg-slate-800/60 hover:bg-slate-800 text-amber-300 border border-slate-700/50 hover:border-amber-500/40 flex items-center justify-center gap-1 transition-all"
              >
                <FileText class="w-3 h-3 text-amber-400" />
                <span>Config</span>
              </button>
              <button
                onclick={() => onOpenLogs(service.id)}
                title="View Live Service Logs"
                class="w-full py-1 px-2 rounded-lg text-[11px] font-medium bg-slate-800/60 hover:bg-slate-800 text-cyan-300 border border-slate-700/50 hover:border-cyan-500/40 flex items-center justify-center gap-1 transition-all"
              >
                <ScrollText class="w-3 h-3 text-cyan-400" />
                <span>Logs</span>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div>
    <div class="flex items-center justify-between mb-4">
      <div>
        <h3 class="text-sm font-semibold text-slate-200 uppercase tracking-wider">Managed Sites & Projects</h3>
        <p class="text-[11px] text-slate-400">Zero-config automatic virtual hosts with internal SSL certificates</p>
      </div>
      <div class="flex items-center gap-2">
        <button
          onclick={onScanWorkspace}
          disabled={isScanningWorkspace}
          class="text-xs px-3 py-1.5 rounded-xl bg-slate-800/80 hover:bg-slate-800 text-slate-200 border border-slate-700 flex items-center gap-1.5 font-medium transition-all hover:border-cyan-500/40"
          title="Scan C:\4forge\projects and auto-register all subfolders as .test sites"
        >
          <Scan class="w-3.5 h-3.5 text-cyan-400 {isScanningWorkspace ? 'animate-spin' : ''}" />
          <span>{isScanningWorkspace ? "Scanning..." : "Scan Workspace"}</span>
        </button>
        <button
          onclick={onOpenAddSite}
          class="text-xs px-3 py-1.5 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:brightness-110 flex items-center gap-1.5 font-bold transition-all shadow-md shadow-cyan-500/20"
        >
          <Plus class="w-3.5 h-3.5" />
          <span>Add Project</span>
        </button>
      </div>
    </div>

    <div class="rounded-xl border border-slate-800/90 bg-[#121929]/50 overflow-hidden shadow-lg">
      <table class="w-full text-left text-xs">
        <thead class="bg-slate-900/70 text-slate-400 uppercase font-semibold border-b border-slate-800">
          <tr>
            <th class="px-5 py-3">Domain</th>
            <th class="px-5 py-3">Runtime</th>
            <th class="px-5 py-3">SSL</th>
            <th class="px-5 py-3">Path</th>
            <th class="px-5 py-3 text-right">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-800/70 text-slate-300">
          {#if sites.length === 0}
            <tr>
              <td colspan="5" class="px-5 py-8 text-center text-slate-500 italic">
                No virtual hosts or projects configured yet. Click "Add Project" or "Scan Workspace" to register a project.
              </td>
            </tr>
          {:else}
            {#each sites as site}
            <tr class="hover:bg-slate-800/30 transition-colors">
              <td class="px-5 py-3.5 font-medium text-white flex items-center gap-2">
                <Globe class="w-3.5 h-3.5 text-cyan-400" />
                <button onclick={() => onOpenSiteBrowser(site.domain)} class="hover:text-cyan-300 hover:underline font-mono font-semibold transition-colors text-left">{site.domain}</button>
              </td>
              <td class="px-5 py-3.5 font-mono text-[11px] text-emerald-400 font-semibold">{site.runtime}</td>
              <td class="px-5 py-3.5">
                <span class="inline-flex items-center gap-1 text-cyan-400 font-medium bg-cyan-950/30 px-2 py-0.5 rounded border border-cyan-500/20 text-[11px]">
                  <CheckCircle2 class="w-3 h-3" /> Auto-HTTPS
                </span>
              </td>
              <td class="px-5 py-3.5 font-mono text-[11px] text-slate-400 truncate max-w-xs">{site.path}</td>
              <td class="px-5 py-3.5 text-right space-x-1.5">
                <button
                  onclick={() => onOpenSiteBrowser(site.domain)}
                  title="Open in Web Browser"
                  class="p-1.5 rounded-lg text-cyan-400 hover:text-white bg-slate-800 hover:bg-slate-700 border border-slate-700 transition-colors inline-flex items-center"
                >
                  <ExternalLink class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenSiteFolder(site.path)}
                  title="Open Folder in File Explorer"
                  class="p-1.5 rounded-lg text-amber-400 hover:text-white bg-slate-800 hover:bg-slate-700 border border-slate-700 transition-colors inline-flex items-center"
                >
                  <FolderOpen class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenProjectInVsCode(site.path)}
                  title="Open in VS Code"
                  class="p-1.5 rounded-lg text-blue-400 hover:text-white bg-slate-800 hover:bg-slate-700 border border-slate-700 transition-colors inline-flex items-center"
                >
                  <Code class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onDeleteSite(site.domain)}
                  title="Remove Virtual Host"
                  class="p-1.5 rounded-lg text-rose-400 hover:text-rose-200 bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/20 transition-colors inline-flex items-center"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              </td>
            </tr>
          {/each}
        {/if}
        </tbody>
      </table>
    </div>
  </div>
</div>
