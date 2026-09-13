<script lang="ts">
  import {
    Activity,
    FolderKanban,
    Layers,
    Server,
    ShieldCheck,
    Sparkles,
    Terminal,
  } from "@lucide/svelte";
  import type { TabType } from "../types";

  let {
    activeTab,
    onSelectTab,
    onOpenUpdateModal,
  }: {
    activeTab: TabType;
    onSelectTab: (tab: TabType) => void;
    onOpenUpdateModal: () => void;
  } = $props();
</script>

<aside class="w-64 border-r border-slate-800/80 bg-[#0E1524]/70 backdrop-blur-2xl flex flex-col shrink-0">
  <div class="h-16 px-6 flex items-center gap-3 border-b border-slate-800/70">
    <div class="h-9 w-9 rounded-xl bg-gradient-to-tr from-cyan-500 via-blue-600 to-indigo-600 flex items-center justify-center shadow-lg shadow-cyan-500/25">
      <Server class="w-5 h-5 text-white" />
    </div>
    <div>
      <div class="flex items-center gap-2">
        <span class="font-bold text-lg tracking-tight text-white">4Forge</span>
        <span class="text-[10px] font-semibold uppercase px-1.5 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">v0.1.0</span>
      </div>
      <p class="text-[11px] text-slate-400">Local Dev Cockpit</p>
    </div>
  </div>

  <nav class="flex-1 px-3 py-4 space-y-1.5">
    <button
      onclick={() => onSelectTab("dashboard")}
      class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'dashboard'
        ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
    >
      <Activity class="w-4 h-4" />
      Dashboard
    </button>

    <button
      onclick={() => onSelectTab("services")}
      class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'services'
        ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
    >
      <Server class="w-4 h-4" />
      Core Services
    </button>

    <button
      onclick={() => onSelectTab("sites")}
      class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'sites'
        ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
    >
      <FolderKanban class="w-4 h-4" />
      Virtual Hosts & Sites
    </button>

    <button
      onclick={() => onSelectTab("runtimes")}
      class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'runtimes'
        ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
    >
      <Layers class="w-4 h-4" />
      Polyglot Runtimes
    </button>

    <button
      onclick={() => onSelectTab("logs")}
      class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'logs'
        ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
    >
      <Terminal class="w-4 h-4" />
      Live Process Logs
    </button>
  </nav>

  <div class="p-4 border-t border-slate-800/70 bg-[#0B101C]/60 text-xs">
    <div class="flex items-center justify-between text-slate-400 mb-1.5">
      <span>Process Shield</span>
      <span class="text-emerald-400 font-mono text-[11px] flex items-center gap-1">
        <ShieldCheck class="w-3.5 h-3.5" /> JobObject Active
      </span>
    </div>
    <div class="flex items-center justify-between text-slate-400 mb-1.5">
      <span>Local CA Root</span>
      <span class="text-cyan-400 font-mono text-[11px]">Auto-HTTPS Ready</span>
    </div>
    <div class="flex items-center justify-between text-slate-400 pt-1.5 border-t border-slate-800/60">
      <span>Release Channel</span>
      <button
        onclick={onOpenUpdateModal}
        class="text-cyan-400 hover:text-cyan-300 font-mono text-[11px] flex items-center gap-1 transition-colors"
      >
        <Sparkles class="w-3 h-3" /> v0.1.0 Stable
      </button>
    </div>
  </div>
</aside>
