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
    Terminal,
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
    onOpenProjectTerminal,
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
    onOpenProjectTerminal: (p: string) => void;
    onOpenProjectInVsCode: (p: string) => void;
    onDeleteSite: (d: string) => void;
  } = $props();
</script>

<div class="space-y-6">
  <div class="rounded-xl border border-slate-200 bg-white p-6 shadow-xs flex items-center justify-between">
    <div>
      <div class="flex items-center gap-2 mb-2">
        <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-medium bg-emerald-50 text-emerald-700 border border-emerald-200">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
          System Operational
        </span>
        {#if portConflicts.length > 0}
          <span class="text-xs text-amber-700 font-medium">{portConflicts.length} port conflict detected</span>
        {:else}
          <span class="text-xs text-slate-500">Zero port conflicts detected</span>
        {/if}
      </div>
      <h2 class="text-lg font-bold text-slate-900 tracking-tight">Polyglot Development Stack Active</h2>
      <p class="text-xs text-slate-500 mt-1 max-w-xl">
        Caddy auto-HTTPS proxy, multi-database architecture (MariaDB, PostgreSQL, SQLite), and isolated runtimes (PHP & Node.js) integrated natively.
      </p>
    </div>

    <div class="flex items-center gap-6">
      <div class="text-right">
        <p class="text-xs text-slate-500">Virtual Hosts</p>
        <p class="text-2xl font-bold text-slate-900 font-mono">{sites.length}</p>
      </div>
      <div class="h-10 w-px bg-slate-200"></div>
      <div class="text-right">
        <p class="text-xs text-slate-500">Active PHP</p>
        <p class="text-2xl font-bold text-slate-900 font-mono">v{activePhpVersion}</p>
      </div>
    </div>
  </div>

  <div>
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-xs font-semibold text-slate-700 uppercase tracking-wider">Services Status</h3>
      <button onclick={onNavigateServices} class="text-xs text-slate-600 hover:text-slate-900 hover:underline font-medium">View All Services &rarr;</button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {#each services as service}
        <div class="rounded-xl border border-slate-200 bg-white p-4 flex flex-col justify-between shadow-xs hover:border-slate-300 transition-all">
          <div>
            <div class="flex items-center justify-between mb-3">
              <div class="p-2 rounded-lg bg-slate-100 text-slate-700 border border-slate-200">
                <service.icon class="w-4 h-4" />
              </div>
              {#if service.status === 'running'}
                <span class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[11px] font-medium bg-emerald-50 text-emerald-700 border border-emerald-200">
                  <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                  Running
                </span>
              {:else}
                <span class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[11px] font-medium bg-slate-100 text-slate-600 border border-slate-200">
                  <span class="w-1.5 h-1.5 rounded-full bg-slate-400"></span>
                  Stopped
                </span>
              {/if}
            </div>

            <h4 class="font-semibold text-sm text-slate-900 mb-0.5">{service.name}</h4>
            <p class="text-xs text-slate-500 mb-3">{service.type}</p>

            <div class="space-y-1.5 pt-2 border-t border-slate-100 text-xs text-slate-500">
              <div class="flex justify-between items-center">
                <span>Ports:</span>
                <span class="text-slate-700 font-mono text-[11px] font-medium bg-slate-100 px-1.5 py-0.5 rounded border border-slate-200">{service.ports}</span>
              </div>
              {#if service.pid}
                <div class="flex justify-between items-center text-[11px]">
                  <span>Process PID:</span>
                  <span class="text-slate-700 font-mono font-medium bg-slate-100 px-1.5 py-0.5 rounded border border-slate-200">#{service.pid}</span>
                </div>
              {/if}

              {#if service.id === "php"}
                <div class="flex items-center justify-between pt-1 border-t border-slate-100 text-xs">
                  <span class="text-slate-500 font-medium">PHP Version:</span>
                  <select
                    value={activePhpVersion}
                    onchange={(e) => onSelectPhpVersion((e.target as HTMLSelectElement).value)}
                    class="bg-slate-50 text-slate-800 border border-slate-200 hover:border-slate-400 rounded-md px-2 py-0.5 text-xs font-mono font-medium transition-colors cursor-pointer focus:bg-white focus:border-slate-900 focus:outline-none"
                  >
                    <option value="8.4.3">8.4.3</option>
                    <option value="8.3.16">8.3.16</option>
                    <option value="8.2.27">8.2.27</option>
                  </select>
                </div>
              {:else if service.id === "node"}
                <div class="flex items-center justify-between pt-1 border-t border-slate-100 text-xs">
                  <span class="text-slate-500 font-medium">Node Version:</span>
                  <select
                    value={activeNodeVersion}
                    onchange={(e) => onSelectNodeVersion((e.target as HTMLSelectElement).value)}
                    class="bg-slate-50 text-slate-800 border border-slate-200 hover:border-slate-400 rounded-md px-2 py-0.5 text-xs font-mono font-medium transition-colors cursor-pointer focus:bg-white focus:border-slate-900 focus:outline-none"
                  >
                    <option value="22.14.0">22.14.0</option>
                    <option value="20.18.2">20.18.2</option>
                  </select>
                </div>
              {/if}
            </div>
          </div>

          <div class="mt-4 pt-3 border-t border-slate-100 space-y-2">
            <div class="flex items-center gap-1.5">
              <button
                onclick={() => onToggleService(service.id)}
                class="flex-1 py-1.5 px-3 rounded-lg text-xs font-medium transition-all flex items-center justify-center gap-1.5 shadow-xs {service.status === 'running'
                  ? 'bg-rose-50 text-rose-700 hover:bg-rose-100 border border-rose-200'
                  : 'bg-slate-900 text-white hover:bg-slate-800'}"
              >
                <Power class="w-3.5 h-3.5" />
                <span>{service.status === "running" ? "Stop" : "Start"}</span>
              </button>
              {#if service.id === "caddy"}
                <button
                  onclick={onOpenWeb}
                  title="Open in Browser"
                  class="p-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs transition-all"
                >
                  <ExternalLink class="w-3.5 h-3.5" />
                </button>
              {:else if service.id === "mariadb"}
                <button
                  onclick={onOpenDatabase}
                  title="Open Database GUI"
                  class="p-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs transition-all"
                >
                  <Database class="w-3.5 h-3.5" />
                </button>
              {/if}
            </div>
            <div class="grid grid-cols-2 gap-1.5">
              <button
                onclick={() => onOpenConfig(service.id)}
                title="Edit Configuration in Notepad"
                class="w-full py-1 px-2 rounded-lg text-[11px] font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center justify-center gap-1 transition-all"
              >
                <FileText class="w-3 h-3 text-slate-500" />
                <span>Config</span>
              </button>
              <button
                onclick={() => onOpenLogs(service.id)}
                title="View Live Service Logs"
                class="w-full py-1 px-2 rounded-lg text-[11px] font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center justify-center gap-1 transition-all"
              >
                <ScrollText class="w-3 h-3 text-slate-500" />
                <span>Logs</span>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div>
    <div class="flex items-center justify-between mb-3">
      <div>
        <h3 class="text-xs font-semibold text-slate-700 uppercase tracking-wider">Managed Sites & Projects</h3>
        <p class="text-[11px] text-slate-500">Zero-config automatic virtual hosts with internal SSL certificates</p>
      </div>
      <div class="flex items-center gap-2">
        <button
          onclick={onScanWorkspace}
          disabled={isScanningWorkspace}
          class="text-xs px-2.5 py-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 flex items-center gap-1.5 font-medium transition-all shadow-xs"
          title="Scan C:\4forge\projects and auto-register all subfolders as .test sites"
        >
          <Scan class="w-3.5 h-3.5 text-slate-500 {isScanningWorkspace ? 'animate-spin' : ''}" />
          <span>{isScanningWorkspace ? "Scanning..." : "Scan Workspace"}</span>
        </button>
        <button
          onclick={onOpenAddSite}
          class="text-xs px-3 py-1.5 rounded-lg bg-slate-900 hover:bg-slate-800 text-white flex items-center gap-1.5 font-semibold transition-all shadow-xs"
        >
          <Plus class="w-3.5 h-3.5" />
          <span>Add Project</span>
        </button>
      </div>
    </div>

    <div class="rounded-xl border border-slate-200 bg-white overflow-hidden shadow-xs">
      <table class="w-full text-left text-xs">
        <thead class="bg-slate-50 text-slate-600 uppercase font-semibold border-b border-slate-200">
          <tr>
            <th class="px-5 py-3">Domain</th>
            <th class="px-5 py-3">Runtime</th>
            <th class="px-5 py-3">SSL</th>
            <th class="px-5 py-3">Path</th>
            <th class="px-5 py-3 text-right">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100 text-slate-700">
          {#if sites.length === 0}
            <tr>
              <td colspan="5" class="px-5 py-8 text-center text-slate-400 italic">
                No virtual hosts or projects configured yet. Click "Add Project" or "Scan Workspace" to register a project.
              </td>
            </tr>
          {:else}
            {#each sites as site}
            <tr class="hover:bg-slate-50/70 transition-colors">
              <td class="px-5 py-3.5 font-medium text-slate-900 flex items-center gap-2">
                <Globe class="w-3.5 h-3.5 text-slate-500" />
                <button onclick={() => onOpenSiteBrowser(site.domain)} class="hover:text-blue-600 hover:underline font-mono font-semibold transition-colors text-left">{site.domain}</button>
              </td>
              <td class="px-5 py-3.5 font-mono text-[11px] text-slate-700 font-semibold">{site.runtime}</td>
              <td class="px-5 py-3.5">
                <span class="inline-flex items-center gap-1 text-emerald-700 font-medium bg-emerald-50 px-2 py-0.5 rounded border border-emerald-200 text-[11px]">
                  <CheckCircle2 class="w-3 h-3 text-emerald-600" /> Auto-HTTPS
                </span>
              </td>
              <td class="px-5 py-3.5 font-mono text-[11px] text-slate-500 truncate max-w-xs">{site.path}</td>
              <td class="px-5 py-3.5 text-right space-x-1">
                <button
                  onclick={() => onOpenSiteBrowser(site.domain)}
                  title="Open in Web Browser"
                  class="p-1.5 rounded-md text-slate-600 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 shadow-xs transition-colors inline-flex items-center"
                >
                  <ExternalLink class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenSiteFolder(site.path)}
                  title="Open Folder in File Explorer"
                  class="p-1.5 rounded-md text-slate-600 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 shadow-xs transition-colors inline-flex items-center"
                >
                  <FolderOpen class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenProjectTerminal(site.path)}
                  title="Open Terminal in Project Directory"
                  class="p-1.5 rounded-md text-slate-600 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 shadow-xs transition-colors inline-flex items-center"
                >
                  <Terminal class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onOpenProjectInVsCode(site.path)}
                  title="Open in VS Code"
                  class="p-1.5 rounded-md text-slate-600 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 shadow-xs transition-colors inline-flex items-center"
                >
                  <Code class="w-3.5 h-3.5" />
                </button>
                <button
                  onclick={() => onDeleteSite(site.domain)}
                  title="Remove Virtual Host"
                  class="p-1.5 rounded-md text-rose-600 hover:text-rose-700 bg-white hover:bg-rose-50 border border-slate-200 hover:border-rose-200 shadow-xs transition-colors inline-flex items-center"
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
