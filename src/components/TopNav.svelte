<script lang="ts">
  import {
    Globe,
    Layers,
    LayoutDashboard,
    Play,
    RefreshCw,
    ScrollText,
    Settings,
    Sparkles,
    Square,
  } from "@lucide/svelte";
  import type { TabType } from "../types";

  let {
    activeTab,
    sitesCount = 0,
    isLoading = false,
    allRunning = false,
    onSelectTab,
    onToggleAll,
    onOpenUpdateModal,
    onRefresh,
    onOpenSettings,
  }: {
    activeTab: TabType;
    sitesCount?: number;
    isLoading?: boolean;
    allRunning?: boolean;
    onSelectTab: (tab: TabType) => void;
    onToggleAll: () => void;
    onOpenUpdateModal: () => void;
    onRefresh: () => void;
    onOpenSettings: () => void;
  } = $props();
</script>

<header class="h-14 border-b border-slate-200 px-4 lg:px-8 flex items-center justify-between shrink-0 bg-white shadow-xs z-30">
  <div class="flex items-center gap-3">
    <div class="flex items-center gap-2">
      <div class="w-7 h-7 rounded-lg bg-gradient-to-br from-[#B44816] to-[#7C2D12] flex items-center justify-center text-white shadow-xs">
        <span class="font-black text-xs tracking-tighter">4F</span>
      </div>
      <div class="flex items-center gap-1.5">
        <span class="font-bold text-sm tracking-tight text-slate-900">4Forge</span>
        <span class="text-[10px] font-mono font-semibold px-1.5 py-0.5 rounded bg-amber-50 text-[#94380C] border border-amber-200/60">v0.1.0</span>
      </div>
    </div>

    <div class="h-4 w-px bg-slate-200 mx-1"></div>

    <nav class="flex items-center gap-1 bg-slate-100 p-1 rounded-lg border border-slate-200/60">
      <button
        onclick={() => onSelectTab("cockpit")}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all {activeTab === 'cockpit'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900'}"
      >
        <LayoutDashboard class="w-3.5 h-3.5 {activeTab === 'cockpit' ? 'text-[#94380C]' : 'text-slate-400'}" />
        <span>Cockpit</span>
      </button>

      <button
        onclick={() => onSelectTab("sites")}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all {activeTab === 'sites'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900'}"
      >
        <Globe class="w-3.5 h-3.5 {activeTab === 'sites' ? 'text-[#94380C]' : 'text-slate-400'}" />
        <span>Sites</span>
        {#if sitesCount > 0}
          <span class="text-[10px] font-mono px-1.5 py-0.2 rounded-full {activeTab === 'sites' ? 'bg-amber-100 text-amber-900' : 'bg-slate-200 text-slate-600'}">
            {sitesCount}
          </span>
        {/if}
      </button>

      <button
        onclick={() => onSelectTab("runtimes")}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all {activeTab === 'runtimes'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900'}"
      >
        <Layers class="w-3.5 h-3.5 {activeTab === 'runtimes' ? 'text-[#94380C]' : 'text-slate-400'}" />
        <span>Runtimes</span>
      </button>

      <button
        onclick={() => onSelectTab("logs")}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all {activeTab === 'logs'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900'}"
      >
        <ScrollText class="w-3.5 h-3.5 {activeTab === 'logs' ? 'text-[#94380C]' : 'text-slate-400'}" />
        <span>Logs</span>
      </button>
    </nav>
  </div>

  <div class="flex items-center gap-1.5">
    <button
      disabled={isLoading}
      onclick={onToggleAll}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg font-semibold text-xs transition-all shadow-xs {isLoading
        ? 'opacity-70 cursor-not-allowed bg-slate-100 text-slate-400 border border-slate-200'
        : allRunning
        ? 'bg-rose-50 text-rose-700 border border-rose-200 hover:bg-rose-100'
        : 'bg-[#94380C] hover:bg-[#7C2D12] text-white'}"
    >
      {#if isLoading}
        <RefreshCw class="w-3 h-3 animate-spin text-slate-400" />
        <span>Wait...</span>
      {:else if allRunning}
        <Square class="w-3 h-3 fill-current" />
        <span>Stop All</span>
      {:else}
        <Play class="w-3 h-3 fill-current" />
        <span>Start All</span>
      {/if}
    </button>

    <div class="h-4 w-px bg-slate-200 mx-1"></div>

    <button
      onclick={onOpenUpdateModal}
      title="Check for updates"
      class="p-1.5 rounded-lg text-slate-500 hover:text-slate-900 hover:bg-slate-100 border border-slate-200 transition-colors shadow-xs"
    >
      <Sparkles class="w-3.5 h-3.5" />
    </button>

    <button
      onclick={onRefresh}
      title="Refresh status"
      class="p-1.5 rounded-lg text-slate-500 hover:text-slate-900 hover:bg-slate-100 border border-slate-200 transition-colors shadow-xs"
    >
      <RefreshCw class="w-3.5 h-3.5" />
    </button>

    <button
      onclick={onOpenSettings}
      title="Settings & Preferences"
      class="p-1.5 rounded-lg text-slate-500 hover:text-slate-900 hover:bg-slate-100 border border-slate-200 transition-colors shadow-xs"
    >
      <Settings class="w-3.5 h-3.5" />
    </button>
  </div>
</header>
