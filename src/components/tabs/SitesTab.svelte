<script lang="ts">
  import {
    CheckCircle2,
    Code,
    ExternalLink,
    FolderOpen,
    Globe,
    Plus,
    RefreshCw,
    Scan,
    ShieldCheck,
    Terminal,
    Trash2,
  } from "@lucide/svelte";
  import type { SiteItem } from "../../types";

  let {
    sites,
    missingHosts,
    isScanningWorkspace,
    isSyncingHosts,
    onScanWorkspace,
    onOpenAddSite,
    onSyncHosts,
    onOpenSiteBrowser,
    onOpenSiteFolder,
    onOpenProjectTerminal,
    onOpenProjectInVsCode,
    onDeleteSite,
  }: {
    sites: SiteItem[];
    missingHosts: string[];
    isScanningWorkspace: boolean;
    isSyncingHosts: boolean;
    onScanWorkspace: () => void;
    onOpenAddSite: () => void;
    onSyncHosts: () => void;
    onOpenSiteBrowser: (d: string) => void;
    onOpenSiteFolder: (p: string) => void;
    onOpenProjectTerminal: (p: string) => void;
    onOpenProjectInVsCode: (p: string) => void;
    onDeleteSite: (d: string) => void;
  } = $props();
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-lg font-bold text-slate-900">Virtual Hosts & Projects</h2>
      <p class="text-xs text-slate-500 mt-0.5">Automatic reverse proxy routing, internal CA auto-HTTPS, and framework auto-detection.</p>
    </div>
    <div class="flex items-center gap-2">
      <button
        onclick={onScanWorkspace}
        disabled={isScanningWorkspace}
        class="px-3 py-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 flex items-center gap-1.5 font-medium text-xs transition-all shadow-xs"
        title="Scan C:\4forge\projects and auto-register all subfolders as .test sites"
      >
        <Scan class="w-3.5 h-3.5 text-slate-500 {isScanningWorkspace ? 'animate-spin' : ''}" />
        <span>{isScanningWorkspace ? "Scanning..." : "Scan Workspace"}</span>
      </button>
      <button
        onclick={onOpenAddSite}
        class="px-3 py-1.5 rounded-lg bg-slate-900 hover:bg-slate-800 text-white flex items-center gap-1.5 font-semibold text-xs transition-all shadow-xs"
      >
        <Plus class="w-3.5 h-3.5" />
        <span>+ Add Project</span>
      </button>
    </div>
  </div>

  {#if missingHosts.length > 0}
    <div class="p-3 rounded-lg bg-amber-50 border border-amber-200 flex items-center justify-between gap-3 text-xs shadow-xs">
      <div class="flex items-center gap-2 text-amber-900">
        <ShieldCheck class="w-4 h-4 text-amber-600 shrink-0" />
        <span>
          <strong>{missingHosts.length} virtual host</strong> ({missingHosts.slice(0, 2).join(", ")}{missingHosts.length > 2 ? '...' : ''}) belum terdaftar di Windows hosts.
        </span>
      </div>
      <button
        onclick={onSyncHosts}
        disabled={isSyncingHosts}
        class="px-2.5 py-1 rounded-md bg-amber-600 hover:bg-amber-700 text-white font-semibold text-xs flex items-center gap-1.5 transition-colors shadow-xs shrink-0"
      >
        {#if isSyncingHosts}
          <RefreshCw class="w-3 h-3 animate-spin" />
          <span>Syncing...</span>
        {:else}
          <ShieldCheck class="w-3 h-3" />
          <span>Sync Windows Hosts</span>
        {/if}
      </button>
    </div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 lg:gap-5">
    {#if sites.length === 0}
      <div class="col-span-full p-12 text-center rounded-xl border border-dashed border-slate-200 bg-white text-slate-400">
        <Globe class="w-8 h-8 mx-auto mb-2 text-slate-400" />
        <p class="font-medium text-sm text-slate-800">No projects registered yet</p>
        <p class="text-xs text-slate-500 mt-1">Click "Add Project" or "Scan Workspace" to register your local projects.</p>
      </div>
    {:else}
      {#each sites as site}
        <div class="rounded-xl border border-slate-200 bg-white p-5 flex flex-col justify-between shadow-xs hover:border-slate-300 transition-all">
          <div>
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center gap-2 text-slate-900 font-semibold">
                <Globe class="w-4 h-4 text-slate-500" />
                <button onclick={() => onOpenSiteBrowser(site.domain)} class="hover:text-blue-600 hover:underline font-mono text-sm transition-colors text-left">{site.domain}</button>
              </div>
              <span class="inline-flex items-center gap-1 text-xs text-emerald-700 bg-emerald-50 px-2 py-0.5 rounded border border-emerald-200 font-medium">
                <CheckCircle2 class="w-3 h-3 text-emerald-600" /> Auto-HTTPS
              </span>
            </div>

            <p class="text-xs text-slate-500 font-mono truncate mb-3 bg-slate-50 px-2.5 py-1.5 rounded-md border border-slate-200">{site.path}</p>

            <div class="text-xs bg-slate-50 rounded-lg p-3 border border-slate-200 space-y-1.5 text-slate-600">
              <div class="flex justify-between items-center">
                <span>Runtime:</span>
                <span class="text-slate-800 font-mono font-medium">{site.runtime}</span>
              </div>
              <div class="flex justify-between items-center">
                <span>Backend & Target:</span>
                <span class="text-slate-800 font-mono font-medium">{site.backend_type.toUpperCase()} ({site.target})</span>
              </div>
            </div>
          </div>

          <div class="mt-4 flex items-center justify-between border-t border-slate-100 pt-3">
            <div class="flex items-center gap-1.5">
              <button
                onclick={() => onOpenSiteBrowser(site.domain)}
                class="px-2.5 py-1 rounded-md text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center gap-1 transition-colors"
                title="Open in Web Browser"
              >
                <ExternalLink class="w-3.5 h-3.5 text-slate-500" />
                <span>Browse</span>
              </button>
              <button
                onclick={() => onOpenSiteFolder(site.path)}
                class="px-2.5 py-1 rounded-md text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center gap-1 transition-colors"
                title="Open Folder in File Explorer"
              >
                <FolderOpen class="w-3.5 h-3.5 text-slate-500" />
                <span>Folder</span>
              </button>
              <button
                onclick={() => onOpenProjectTerminal(site.path)}
                class="px-2.5 py-1 rounded-md text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center gap-1 transition-colors"
                title="Open Terminal in Project Directory"
              >
                <Terminal class="w-3.5 h-3.5 text-slate-500" />
                <span>Terminal</span>
              </button>
              <button
                onclick={() => onOpenProjectInVsCode(site.path)}
                class="px-2.5 py-1 rounded-md text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center gap-1 transition-colors"
                title="Open in VS Code"
              >
                <Code class="w-3.5 h-3.5 text-slate-500" />
                <span>Code</span>
              </button>
            </div>

            <button
              onclick={() => onDeleteSite(site.domain)}
              class="p-1.5 rounded-md text-rose-600 hover:text-rose-700 bg-white hover:bg-rose-50 border border-slate-200 hover:border-rose-200 shadow-xs transition-colors"
              title="Remove Virtual Host"
            >
              <Trash2 class="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>
