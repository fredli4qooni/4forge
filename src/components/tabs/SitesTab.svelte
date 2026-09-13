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
      <h2 class="text-xl font-bold text-white">Virtual Hosts & Projects</h2>
      <p class="text-xs text-slate-400 mt-1">Automatic reverse proxy routing, internal CA auto-HTTPS, and framework auto-detection.</p>
    </div>
    <div class="flex items-center gap-2">
      <button
        onclick={onScanWorkspace}
        disabled={isScanningWorkspace}
        class="px-4 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 flex items-center gap-2 font-medium text-xs transition-all hover:border-cyan-500/40"
        title="Scan C:\4forge\projects and auto-register all subfolders as .test sites"
      >
        <Scan class="w-4 h-4 text-cyan-400 {isScanningWorkspace ? 'animate-spin' : ''}" />
        <span>{isScanningWorkspace ? "Scanning..." : "Scan Workspace"}</span>
      </button>
      <button
        onclick={onOpenAddSite}
        class="px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:brightness-110 flex items-center gap-2 font-bold text-xs transition-all shadow-md shadow-cyan-500/20"
      >
        <Plus class="w-4 h-4" />
        <span>+ Add Project</span>
      </button>
    </div>
  </div>

  {#if missingHosts.length > 0}
    <div class="p-3.5 rounded-xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-between gap-3 text-xs">
      <div class="flex items-center gap-2.5 text-amber-300">
        <ShieldCheck class="w-4 h-4 text-amber-400 shrink-0" />
        <span>
          <strong>{missingHosts.length} virtual host</strong> ({missingHosts.slice(0, 2).join(", ")}{missingHosts.length > 2 ? '...' : ''}) belum terdaftar di Windows hosts.
        </span>
      </div>
      <button
        onclick={onSyncHosts}
        disabled={isSyncingHosts}
        class="px-3 py-1.5 rounded-lg bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold text-xs flex items-center gap-1.5 transition-colors shadow-sm shrink-0"
      >
        {#if isSyncingHosts}
          <RefreshCw class="w-3.5 h-3.5 animate-spin" />
          <span>Syncing...</span>
        {:else}
          <ShieldCheck class="w-3.5 h-3.5" />
          <span>Sync Windows Hosts</span>
        {/if}
      </button>
    </div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#if sites.length === 0}
      <div class="col-span-full p-12 text-center rounded-2xl border border-dashed border-slate-800 bg-[#121929]/30 text-slate-500">
        <Globe class="w-8 h-8 mx-auto mb-2 text-slate-600" />
        <p class="font-medium text-sm text-slate-300">No projects registered yet</p>
        <p class="text-xs text-slate-500 mt-1">Click "Add Project" or "Scan Workspace" to register your local projects.</p>
      </div>
    {:else}
      {#each sites as site}
        <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/70 hover:bg-[#121929]/90 p-5 flex flex-col justify-between transition-all hover:border-slate-700/80 shadow-lg">
          <div>
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center gap-2 text-white font-bold">
                <Globe class="w-4 h-4 text-cyan-400" />
                <button onclick={() => onOpenSiteBrowser(site.domain)} class="hover:text-cyan-300 hover:underline font-mono text-sm transition-colors text-left">{site.domain}</button>
              </div>
              <span class="inline-flex items-center gap-1 text-xs text-emerald-400 bg-emerald-500/10 px-2.5 py-0.5 rounded-full border border-emerald-500/20 font-medium">
                <CheckCircle2 class="w-3 h-3" /> Auto-HTTPS
              </span>
            </div>

            <p class="text-xs text-slate-400 font-mono truncate mb-3 bg-[#0B101C]/60 px-2.5 py-1.5 rounded-lg border border-slate-800">{site.path}</p>

            <div class="text-xs bg-[#0B101C]/60 rounded-xl p-3 border border-slate-800 space-y-1.5">
              <div class="flex justify-between items-center">
                <span class="text-slate-400">Runtime:</span>
                <span class="text-emerald-400 font-mono font-semibold">{site.runtime}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-slate-400">Backend & Target:</span>
                <span class="text-cyan-300 font-mono font-medium">{site.backend_type.toUpperCase()} ({site.target})</span>
              </div>
            </div>
          </div>

          <div class="mt-4 flex items-center justify-between border-t border-slate-800/80 pt-3">
            <div class="flex items-center gap-2">
              <button
                onclick={() => onOpenSiteBrowser(site.domain)}
                class="px-2.5 py-1 rounded-lg text-xs font-medium bg-slate-800 hover:bg-slate-700 text-cyan-300 border border-slate-700 flex items-center gap-1.5 transition-colors"
                title="Open in Web Browser"
              >
                <ExternalLink class="w-3.5 h-3.5 text-cyan-400" />
                <span>Browse</span>
              </button>
              <button
                onclick={() => onOpenSiteFolder(site.path)}
                class="px-2.5 py-1 rounded-lg text-xs font-medium bg-slate-800 hover:bg-slate-700 text-amber-300 border border-slate-700 flex items-center gap-1.5 transition-colors"
                title="Open Folder in File Explorer"
              >
                <FolderOpen class="w-3.5 h-3.5 text-amber-400" />
                <span>Folder</span>
              </button>
              <button
                onclick={() => onOpenProjectTerminal(site.path)}
                class="px-2.5 py-1 rounded-lg text-xs font-medium bg-slate-800 hover:bg-slate-700 text-emerald-300 border border-slate-700 flex items-center gap-1.5 transition-colors"
                title="Open Terminal in Project Directory"
              >
                <Terminal class="w-3.5 h-3.5 text-emerald-400" />
                <span>Terminal</span>
              </button>
              <button
                onclick={() => onOpenProjectInVsCode(site.path)}
                class="px-2.5 py-1 rounded-lg text-xs font-medium bg-slate-800 hover:bg-slate-700 text-blue-300 border border-slate-700 flex items-center gap-1.5 transition-colors"
                title="Open in VS Code"
              >
                <Code class="w-3.5 h-3.5 text-blue-400" />
                <span>Code</span>
              </button>
            </div>

            <button
              onclick={() => onDeleteSite(site.domain)}
              class="p-1.5 rounded-lg text-rose-400 hover:text-rose-200 bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/20 transition-colors"
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
