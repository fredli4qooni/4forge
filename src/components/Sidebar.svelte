<script lang="ts">
  import {
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

<aside class="w-64 border-r border-slate-200 bg-slate-50/75 flex flex-col shrink-0">
  <div class="h-16 px-6 flex items-center gap-3 border-b border-slate-200 bg-white">
    <div class="h-8 w-8 rounded-lg bg-[#94380C] flex items-center justify-center shadow-xs">
      <Server class="w-4 h-4 text-white" />
    </div>
    <div>
      <div class="flex items-center gap-2">
        <span class="font-bold text-base tracking-tight text-slate-900">4Forge</span>
        <span class="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded bg-slate-100 text-slate-600 border border-slate-200">v0.1.0</span>
      </div>
      <p class="text-[11px] text-slate-500">Local Dev Cockpit</p>
    </div>
  </div>

  <nav class="flex-1 px-3 py-4 space-y-1">
    <button
      onclick={() => onSelectTab("services")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all {activeTab === 'services'
        ? 'bg-white text-slate-900 font-semibold shadow-xs border border-slate-200'
        : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100/80'}"
    >
      <Server class="w-4 h-4" />
      Core Services
    </button>

    <button
      onclick={() => onSelectTab("sites")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all {activeTab === 'sites'
        ? 'bg-white text-slate-900 font-semibold shadow-xs border border-slate-200'
        : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100/80'}"
    >
      <FolderKanban class="w-4 h-4" />
      Virtual Hosts & Sites
    </button>

    <button
      onclick={() => onSelectTab("runtimes")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all {activeTab === 'runtimes'
        ? 'bg-white text-slate-900 font-semibold shadow-xs border border-slate-200'
        : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100/80'}"
    >
      <Layers class="w-4 h-4" />
      Polyglot Runtimes
    </button>

    <button
      onclick={() => onSelectTab("logs")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all {activeTab === 'logs'
        ? 'bg-white text-slate-900 font-semibold shadow-xs border border-slate-200'
        : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100/80'}"
    >
      <Terminal class="w-4 h-4" />
      Live Process Logs
    </button>
  </nav>

  <div class="p-4 border-t border-slate-200 bg-white text-xs space-y-2">
    <div class="flex items-center justify-between text-slate-500">
      <span>Process Shield</span>
      <span class="text-emerald-700 bg-emerald-50 px-1.5 py-0.5 rounded border border-emerald-200 font-mono text-[10px] font-medium flex items-center gap-1">
        <ShieldCheck class="w-3 h-3 text-emerald-600" /> Active
      </span>
    </div>
    <div class="flex items-center justify-between text-slate-500">
      <span>Local CA Root</span>
      <span class="text-slate-700 font-mono text-[11px]">Auto-HTTPS Ready</span>
    </div>
    <div class="flex items-center justify-between text-slate-500 pt-1.5 border-t border-slate-100">
      <span>Release Channel</span>
      <button
        onclick={onOpenUpdateModal}
        class="text-slate-700 hover:text-slate-900 font-mono text-[11px] flex items-center gap-1 transition-colors"
      >
        <Sparkles class="w-3 h-3 text-slate-500" /> v0.1.0 Stable
      </button>
    </div>
  </div>
</aside>
