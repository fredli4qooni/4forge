<script lang="ts">
  import {
    Database,
    FolderKanban,
    Globe,
    Minimize2,
    Play,
    RefreshCw,
    Sparkles,
    Square,
    Terminal,
  } from "@lucide/svelte";

  let {
    isLoading,
    allRunning,
    runningCount,
    stoppedCount,
    onToggleAll,
    onOpenWeb,
    onOpenDatabase,
    onOpenTerminal,
    onOpenProjects,
    onOpenUpdateModal,
    onRefresh,
    onSwitchToCompact,
  }: {
    isLoading: boolean;
    allRunning: boolean;
    runningCount: number;
    stoppedCount: number;
    onToggleAll: () => void;
    onOpenWeb: () => void;
    onOpenDatabase: () => void;
    onOpenTerminal: () => void;
    onOpenProjects: () => void;
    onOpenUpdateModal: () => void;
    onRefresh: () => void;
    onSwitchToCompact: () => void;
  } = $props();
</script>

<header class="h-16 border-b border-slate-200 px-8 flex items-center justify-between shrink-0 bg-white">
  <div>
    <h1 class="text-sm font-semibold text-slate-900">Local Development Environment</h1>
    <p class="text-xs text-slate-500">Windows Native • Apache-2.0 Pure Open Source</p>
  </div>

  <div class="flex items-center gap-2">
    <button
      disabled={isLoading}
      onclick={onToggleAll}
      class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg font-semibold text-xs transition-all shadow-xs {isLoading
        ? 'opacity-70 cursor-not-allowed bg-slate-100 text-slate-400 border border-slate-200'
        : allRunning
        ? 'bg-rose-50 text-rose-700 border border-rose-200 hover:bg-rose-100'
        : runningCount > 0
        ? 'bg-[#94380C] text-white hover:bg-[#7C2D12]'
        : 'bg-[#94380C] text-white hover:bg-[#7C2D12]'}"
    >
      {#if isLoading}
        <RefreshCw class="w-3.5 h-3.5 animate-spin text-slate-400" />
        <span>Processing...</span>
      {:else if allRunning}
        <Square class="w-3.5 h-3.5 fill-current" />
        <span>Stop All</span>
      {:else if runningCount > 0}
        <Play class="w-3.5 h-3.5 fill-current" />
        <span>Start All ({stoppedCount} stopped)</span>
      {:else}
        <Play class="w-3.5 h-3.5 fill-current" />
        <span>Start All</span>
      {/if}
    </button>

    <div class="h-5 w-px bg-slate-200 mx-1"></div>

    <button
      onclick={onOpenWeb}
      title="Open localhost in Default Web Browser"
      class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-slate-700 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 text-xs font-medium transition-all shadow-xs"
    >
      <Globe class="w-3.5 h-3.5 text-slate-500" />
      <span>Web</span>
    </button>

    <button
      onclick={onOpenDatabase}
      title="Open Database GUI / Management"
      class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-slate-700 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 text-xs font-medium transition-all shadow-xs"
    >
      <Database class="w-3.5 h-3.5 text-slate-500" />
      <span>Database</span>
    </button>

    <button
      onclick={onOpenTerminal}
      title="Open Terminal with 4Forge PHP, Node, MariaDB in PATH"
      class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-slate-700 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 text-xs font-medium transition-all shadow-xs"
    >
      <Terminal class="w-3.5 h-3.5 text-slate-500" />
      <span>Terminal</span>
    </button>

    <button
      onclick={onOpenProjects}
      title="Open Projects Directory in Windows Explorer"
      class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-slate-700 hover:text-slate-900 bg-white hover:bg-slate-50 border border-slate-200 text-xs font-medium transition-all shadow-xs"
    >
      <FolderKanban class="w-3.5 h-3.5 text-slate-500" />
      <span>Root</span>
    </button>

    <div class="h-5 w-px bg-slate-200 mx-1"></div>

    <button
      onclick={onOpenUpdateModal}
      title="Check for Updates & Security Integrity"
      class="p-1.5 rounded-lg text-slate-500 hover:text-slate-900 hover:bg-slate-100 border border-slate-200 transition-colors shadow-xs"
    >
      <Sparkles class="w-3.5 h-3.5" />
    </button>

    <button
      onclick={onRefresh}
      title="Reload Services, Ports & Logs"
      class="p-1.5 rounded-lg text-slate-500 hover:text-slate-900 hover:bg-slate-100 border border-slate-200 transition-colors shadow-xs"
    >
      <RefreshCw class="w-3.5 h-3.5" />
    </button>

    <button
      onclick={onSwitchToCompact}
      title="Switch to Compact Cockpit (Laragon-style single screen mode)"
      class="p-1.5 rounded-lg text-slate-500 hover:text-slate-900 hover:bg-slate-100 border border-slate-200 transition-colors shadow-xs"
    >
      <Minimize2 class="w-3.5 h-3.5" />
    </button>
  </div>
</header>
