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

<header class="h-16 border-b border-slate-800/70 px-8 flex items-center justify-between shrink-0 bg-[#0E1524]/50 backdrop-blur-md">
  <div>
    <h1 class="text-base font-semibold text-white">Local Development Environment</h1>
    <p class="text-xs text-slate-400">Windows Native • Apache-2.0 Pure Open Source</p>
  </div>

  <div class="flex items-center gap-2">
    <button
      disabled={isLoading}
      onclick={onToggleAll}
      class="flex items-center gap-2 px-3.5 py-2 rounded-xl font-bold text-xs transition-all shadow-md {isLoading
        ? 'opacity-70 cursor-not-allowed bg-slate-700 text-slate-300'
        : allRunning
        ? 'bg-rose-500/20 text-rose-300 border border-rose-500/40 hover:bg-rose-500/30'
        : runningCount > 0
        ? 'bg-gradient-to-r from-amber-500 to-emerald-600 text-white hover:brightness-110 shadow-amber-500/20'
        : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
    >
      {#if isLoading}
        <RefreshCw class="w-3.5 h-3.5 animate-spin" />
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

    <div class="h-6 w-px bg-slate-800 mx-1"></div>

    <button
      onclick={onOpenWeb}
      title="Open localhost in Default Web Browser"
      class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-cyan-500/50"
    >
      <Globe class="w-3.5 h-3.5 text-cyan-400" />
      <span>Web</span>
    </button>

    <button
      onclick={onOpenDatabase}
      title="Open Database GUI / Management"
      class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-emerald-500/50"
    >
      <Database class="w-3.5 h-3.5 text-emerald-400" />
      <span>Database</span>
    </button>

    <button
      onclick={onOpenTerminal}
      title="Open Terminal with 4Forge PHP, Node, MariaDB in PATH"
      class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-amber-500/50"
    >
      <Terminal class="w-3.5 h-3.5 text-amber-400" />
      <span>Terminal</span>
    </button>

    <button
      onclick={onOpenProjects}
      title="Open Projects Directory in Windows Explorer"
      class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-blue-500/50"
    >
      <FolderKanban class="w-3.5 h-3.5 text-blue-400" />
      <span>Root</span>
    </button>

    <div class="h-6 w-px bg-slate-800 mx-1"></div>

    <button
      onclick={onOpenUpdateModal}
      title="Check for Updates & Security Integrity"
      class="flex items-center gap-1 px-2.5 py-2 rounded-xl text-slate-400 hover:text-slate-200 bg-slate-800/40 hover:bg-slate-800/80 border border-slate-800 text-xs font-medium transition-all"
    >
      <Sparkles class="w-3.5 h-3.5 text-cyan-400" />
    </button>

    <button
      onclick={onRefresh}
      title="Reload Services, Ports & Logs"
      class="p-2 rounded-xl text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 border border-slate-800 transition-colors"
    >
      <RefreshCw class="w-4 h-4" />
    </button>

    <button
      onclick={onSwitchToCompact}
      title="Switch to Compact Cockpit (Laragon-style single screen mode)"
      class="p-2 rounded-xl text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 border border-slate-800 transition-colors"
    >
      <Minimize2 class="w-4 h-4" />
    </button>
  </div>
</header>
