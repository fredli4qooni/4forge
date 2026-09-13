<script lang="ts">
  import { Server, Layers, Terminal, Activity } from "@lucide/svelte";

  let {
    activePhpVersion,
    activeNodeVersion,
    activePythonVersion,
    activeRubyVersion,
    onSelectPhpVersion,
    onSelectNodeVersion,
    onSelectPythonVersion,
    onSelectRubyVersion
  }: {
    activePhpVersion: string;
    activeNodeVersion: string;
    activePythonVersion: string;
    activeRubyVersion: string;
    onSelectPhpVersion: (version: string) => void;
    onSelectNodeVersion: (version: string) => void;
    onSelectPythonVersion: (version: string) => void;
    onSelectRubyVersion: (version: string) => void;
  } = $props();
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-xl font-bold text-white">Polyglot Runtime Manager</h2>
      <p class="text-xs text-slate-400 mt-1">Switch runtime versions without restarting the system (mise model).</p>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-purple-500/10 text-purple-400 border border-purple-500/20">
            <Server class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-base text-white">PHP Engine</h3>
            <p class="text-xs text-slate-400">FastCGI process daemon</p>
          </div>
        </div>
        <span class="text-xs font-mono text-purple-300 bg-purple-500/20 px-2.5 py-1 rounded-full border border-purple-500/30">
          Active: v{activePhpVersion}
        </span>
      </div>

      <div class="space-y-2">
        {#each ["8.4.3", "8.3.16", "8.2.27"] as ver}
          <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
            <span class="text-sm font-mono font-medium text-slate-200">PHP {ver}</span>
            <button
              onclick={() => onSelectPhpVersion(ver)}
              class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activePhpVersion === ver
                ? 'bg-purple-500 text-white shadow-lg shadow-purple-500/30'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
            >
              {activePhpVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
            <Layers class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-base text-white">Node.js Engine</h3>
            <p class="text-xs text-slate-400">Integrated npm, npx, corepack</p>
          </div>
        </div>
        <span class="text-xs font-mono text-emerald-300 bg-emerald-500/20 px-2.5 py-1 rounded-full border border-emerald-500/30">
          Active: v{activeNodeVersion}
        </span>
      </div>

      <div class="space-y-2">
        {#each ["22.14.0", "20.18.3"] as ver}
          <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
            <span class="text-sm font-mono font-medium text-slate-200">Node.js {ver}</span>
            <button
              onclick={() => onSelectNodeVersion(ver)}
              class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activeNodeVersion === ver
                ? 'bg-emerald-500 text-white shadow-lg shadow-emerald-500/30'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
            >
              {activeNodeVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-blue-500/10 text-blue-400 border border-blue-500/20">
            <Terminal class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-base text-white">Python Engine</h3>
            <p class="text-xs text-slate-400">Isolated .venv & ASGI/WSGI runner</p>
          </div>
        </div>
        <span class="text-xs font-mono text-blue-300 bg-blue-500/20 px-2.5 py-1 rounded-full border border-blue-500/30">
          Active: v{activePythonVersion}
        </span>
      </div>

      <div class="space-y-2">
        {#each ["3.12.9"] as ver}
          <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
            <span class="text-sm font-mono font-medium text-slate-200">Python {ver} (embed x64)</span>
            <button
              onclick={() => onSelectPythonVersion(ver)}
              class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activePythonVersion === ver
                ? 'bg-blue-500 text-white shadow-lg shadow-blue-500/30'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
            >
              {activePythonVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-rose-500/10 text-rose-400 border border-rose-500/20">
            <Activity class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-base text-white">Ruby Engine</h3>
            <p class="text-xs text-slate-400">Gem isolation & Puma / Rack runner</p>
          </div>
        </div>
        <span class="text-xs font-mono text-rose-300 bg-rose-500/20 px-2.5 py-1 rounded-full border border-rose-500/30">
          Active: v{activeRubyVersion}
        </span>
      </div>

      <div class="space-y-2">
        {#each ["3.3.7"] as ver}
          <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
            <span class="text-sm font-mono font-medium text-slate-200">Ruby {ver} (x64)</span>
            <button
              onclick={() => onSelectRubyVersion(ver)}
              class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activeRubyVersion === ver
                ? 'bg-rose-500 text-white shadow-lg shadow-rose-500/30'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
            >
              {activeRubyVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
