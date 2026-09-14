<script lang="ts">
  import {
    ChevronDown,
    ChevronUp,
    Copy,
    Info,
    KeyRound,
    Server,
    ShieldCheck,
  } from "@lucide/svelte";

  let {
    onOpenGeneralConnect,
    onShowToast,
  }: {
    onOpenGeneralConnect?: () => void;
    onShowToast: (msg: string) => void;
  } = $props();

  let isCollapsed = $state<boolean>(false);

  function copyDefault(text: string, label: string) {
    navigator.clipboard.writeText(text);
    onShowToast(`Copied ${label} default config`);
  }
</script>

<div class="bg-gradient-to-r from-amber-50/70 via-slate-50 to-orange-50/50 border border-amber-200/60 rounded-xl p-3.5 shadow-2xs">
  <div class="flex items-center justify-between gap-3">
    <div class="flex items-center gap-2.5">
      <div class="w-7 h-7 rounded-lg bg-[#94380C]/10 border border-[#94380C]/20 flex items-center justify-center shrink-0">
        <KeyRound class="w-4 h-4 text-[#94380C]" />
      </div>
      <div>
        <div class="flex items-center gap-2">
          <h2 class="text-xs font-bold text-slate-800">Connection Defaults & Environment Guide</h2>
          <span class="text-[10px] font-semibold px-1.5 py-0.2 rounded bg-amber-100/80 text-amber-800 border border-amber-200/60">
            Local Dev
          </span>
        </div>
        <p class="text-[11px] text-slate-500 mt-0.5">
          Standard credentials for local frameworks (Laravel, Prisma, Django, Express).
        </p>
      </div>
    </div>

    <div class="flex items-center gap-1.5 shrink-0">
      {#if onOpenGeneralConnect}
        <button
          onclick={onOpenGeneralConnect}
          class="flex items-center gap-1 px-2.5 py-1 rounded-lg bg-white border border-slate-200 hover:border-slate-300 text-slate-700 text-xs font-medium shadow-2xs hover:bg-slate-50 transition-colors"
          title="Open Config Generator"
        >
          <Server class="w-3.5 h-3.5 text-[#94380C]" />
          <span>Config Generator</span>
        </button>
      {/if}

      <button
        onclick={() => (isCollapsed = !isCollapsed)}
        class="p-1 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-500 hover:text-slate-800 transition-colors"
        title={isCollapsed ? "Expand Connection Guide" : "Collapse Connection Guide"}
      >
        {#if isCollapsed}
          <ChevronDown class="w-3.5 h-3.5" />
        {:else}
          <ChevronUp class="w-3.5 h-3.5" />
        {/if}
      </button>
    </div>
  </div>

  {#if !isCollapsed}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-2.5 mt-3 pt-3 border-t border-amber-200/40 text-[11px]">
      <div class="bg-white/80 rounded-lg p-2 border border-slate-200/70 space-y-1">
        <div class="flex items-center justify-between">
          <span class="font-bold text-slate-800 flex items-center gap-1">
            <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
            MySQL / MariaDB
          </span>
          <button
            onclick={() => copyDefault("DB_HOST=127.0.0.1\nDB_PORT=3306\nDB_USERNAME=root\nDB_PASSWORD=", "MySQL")}
            class="p-0.5 text-slate-400 hover:text-slate-700"
            title="Copy MySQL Defaults"
          >
            <Copy class="w-3 h-3" />
          </button>
        </div>
        <div class="font-mono text-slate-600 space-y-0.5 text-[10.5px]">
          <div>Host: <span class="text-slate-900 font-semibold">127.0.0.1:3306</span></div>
          <div>User: <span class="text-slate-900 font-semibold">root</span></div>
          <div>Pass: <span class="text-slate-400 italic">(empty)</span></div>
        </div>
      </div>

      <div class="bg-white/80 rounded-lg p-2 border border-slate-200/70 space-y-1">
        <div class="flex items-center justify-between">
          <span class="font-bold text-slate-800 flex items-center gap-1">
            <span class="w-1.5 h-1.5 rounded-full bg-blue-500"></span>
            PostgreSQL
          </span>
          <button
            onclick={() => copyDefault("DB_HOST=127.0.0.1\nDB_PORT=5432\nDB_USERNAME=postgres\nDB_PASSWORD=", "PostgreSQL")}
            class="p-0.5 text-slate-400 hover:text-slate-700"
            title="Copy PostgreSQL Defaults"
          >
            <Copy class="w-3 h-3" />
          </button>
        </div>
        <div class="font-mono text-slate-600 space-y-0.5 text-[10.5px]">
          <div>Host: <span class="text-slate-900 font-semibold">127.0.0.1:5432</span></div>
          <div>User: <span class="text-slate-900 font-semibold">postgres</span></div>
          <div>Pass: <span class="text-slate-400 italic">(empty / postgres)</span></div>
        </div>
      </div>

      <div class="bg-white/80 rounded-lg p-2 border border-slate-200/70 space-y-1">
        <div class="flex items-center justify-between">
          <span class="font-bold text-slate-800 flex items-center gap-1">
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
            MongoDB
          </span>
          <button
            onclick={() => copyDefault("MONGO_URI=mongodb://127.0.0.1:27017/", "MongoDB")}
            class="p-0.5 text-slate-400 hover:text-slate-700"
            title="Copy Mongo Defaults"
          >
            <Copy class="w-3 h-3" />
          </button>
        </div>
        <div class="font-mono text-slate-600 space-y-0.5 text-[10.5px]">
          <div>Host: <span class="text-slate-900 font-semibold">127.0.0.1:27017</span></div>
          <div>URI: <span class="text-slate-900 font-semibold">mongodb://127.0.0.1:27017</span></div>
          <div>Auth: <span class="text-slate-400 italic">(none by default)</span></div>
        </div>
      </div>

      <div class="bg-white/80 rounded-lg p-2 border border-slate-200/70 space-y-1">
        <div class="flex items-center justify-between">
          <span class="font-bold text-slate-800 flex items-center gap-1">
            <span class="w-1.5 h-1.5 rounded-full bg-indigo-500"></span>
            SQLite (Embedded)
          </span>
          <button
            onclick={() => copyDefault("DB_CONNECTION=sqlite\nDB_DATABASE=database/database.sqlite", "SQLite")}
            class="p-0.5 text-slate-400 hover:text-slate-700"
            title="Copy SQLite Defaults"
          >
            <Copy class="w-3 h-3" />
          </button>
        </div>
        <div class="font-mono text-slate-600 space-y-0.5 text-[10.5px]">
          <div>Driver: <span class="text-slate-900 font-semibold">sqlite</span></div>
          <div>File: <span class="text-slate-900 font-semibold">C:\4forge\data\sqlite\</span></div>
          <div>Mode: <span class="text-emerald-700 font-semibold">Zero-Config</span></div>
        </div>
      </div>
    </div>
  {/if}
</div>
