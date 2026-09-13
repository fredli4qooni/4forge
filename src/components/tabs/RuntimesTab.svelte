<script lang="ts">
  import { Server, Layers, Terminal, Activity, Settings } from "@lucide/svelte";

  let {
    activePhpVersion,
    activeNodeVersion,
    activePythonVersion,
    activeRubyVersion,
    onSelectPhpVersion,
    onSelectNodeVersion,
    onSelectPythonVersion,
    onSelectRubyVersion,
    onOpenSettings,
  }: {
    activePhpVersion: string;
    activeNodeVersion: string;
    activePythonVersion: string;
    activeRubyVersion: string;
    onSelectPhpVersion: (version: string) => void;
    onSelectNodeVersion: (version: string) => void;
    onSelectPythonVersion: (version: string) => void;
    onSelectRubyVersion: (version: string) => void;
    onOpenSettings?: () => void;
  } = $props();
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-lg font-bold text-slate-900">Polyglot Runtime Manager</h2>
      <p class="text-xs text-slate-500 mt-0.5">Switch runtime versions without restarting the system (mise model).</p>
    </div>
    {#if onOpenSettings}
      <button
        onclick={onOpenSettings}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium transition-all shadow-xs"
        title="Runtime & Environment Preferences"
      >
        <Settings class="w-3.5 h-3.5 text-slate-500" />
        <span>Settings</span>
      </button>
    {/if}
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div class="rounded-xl border border-slate-200 bg-white p-5 space-y-4 shadow-xs">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Server class="w-4 h-4" />
          </div>
          <div>
            <h3 class="font-semibold text-sm text-slate-900">PHP Engine</h3>
            <p class="text-xs text-slate-500">FastCGI process daemon</p>
          </div>
        </div>
        <span class="text-xs font-mono font-medium text-slate-700 bg-slate-100 px-2.5 py-0.5 rounded border border-slate-200">
          Active: v{activePhpVersion}
        </span>
      </div>

      <div class="space-y-1.5">
        {#each ["8.4.3", "8.3.16", "8.2.27"] as ver}
          <div class="flex items-center justify-between p-2.5 rounded-lg bg-slate-50 border border-slate-200">
            <span class="text-xs font-mono font-medium text-slate-800">PHP {ver}</span>
            <button
              onclick={() => onSelectPhpVersion(ver)}
              class="px-2.5 py-1 rounded-md text-xs font-medium transition-all {activePhpVersion === ver
                ? 'bg-slate-900 text-white shadow-xs'
                : 'bg-white hover:bg-slate-100 text-slate-700 border border-slate-200 shadow-xs'}"
            >
              {activePhpVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="rounded-xl border border-slate-200 bg-white p-5 space-y-4 shadow-xs">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Layers class="w-4 h-4" />
          </div>
          <div>
            <h3 class="font-semibold text-sm text-slate-900">Node.js Engine</h3>
            <p class="text-xs text-slate-500">Integrated npm, npx, corepack</p>
          </div>
        </div>
        <span class="text-xs font-mono font-medium text-slate-700 bg-slate-100 px-2.5 py-0.5 rounded border border-slate-200">
          Active: v{activeNodeVersion}
        </span>
      </div>

      <div class="space-y-1.5">
        {#each ["22.14.0", "20.18.3"] as ver}
          <div class="flex items-center justify-between p-2.5 rounded-lg bg-slate-50 border border-slate-200">
            <span class="text-xs font-mono font-medium text-slate-800">Node.js {ver}</span>
            <button
              onclick={() => onSelectNodeVersion(ver)}
              class="px-2.5 py-1 rounded-md text-xs font-medium transition-all {activeNodeVersion === ver
                ? 'bg-slate-900 text-white shadow-xs'
                : 'bg-white hover:bg-slate-100 text-slate-700 border border-slate-200 shadow-xs'}"
            >
              {activeNodeVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="rounded-xl border border-slate-200 bg-white p-5 space-y-4 shadow-xs">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Terminal class="w-4 h-4" />
          </div>
          <div>
            <h3 class="font-semibold text-sm text-slate-900">Python Engine</h3>
            <p class="text-xs text-slate-500">Isolated .venv & ASGI/WSGI runner</p>
          </div>
        </div>
        <span class="text-xs font-mono font-medium text-slate-700 bg-slate-100 px-2.5 py-0.5 rounded border border-slate-200">
          Active: v{activePythonVersion}
        </span>
      </div>

      <div class="space-y-1.5">
        {#each ["3.12.9"] as ver}
          <div class="flex items-center justify-between p-2.5 rounded-lg bg-slate-50 border border-slate-200">
            <span class="text-xs font-mono font-medium text-slate-800">Python {ver} (embed x64)</span>
            <button
              onclick={() => onSelectPythonVersion(ver)}
              class="px-2.5 py-1 rounded-md text-xs font-medium transition-all {activePythonVersion === ver
                ? 'bg-slate-900 text-white shadow-xs'
                : 'bg-white hover:bg-slate-100 text-slate-700 border border-slate-200 shadow-xs'}"
            >
              {activePythonVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="rounded-xl border border-slate-200 bg-white p-5 space-y-4 shadow-xs">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Activity class="w-4 h-4" />
          </div>
          <div>
            <h3 class="font-semibold text-sm text-slate-900">Ruby Engine</h3>
            <p class="text-xs text-slate-500">Gem isolation & Puma / Rack runner</p>
          </div>
        </div>
        <span class="text-xs font-mono font-medium text-slate-700 bg-slate-100 px-2.5 py-0.5 rounded border border-slate-200">
          Active: v{activeRubyVersion}
        </span>
      </div>

      <div class="space-y-1.5">
        {#each ["3.3.7"] as ver}
          <div class="flex items-center justify-between p-2.5 rounded-lg bg-slate-50 border border-slate-200">
            <span class="text-xs font-mono font-medium text-slate-800">Ruby {ver} (x64)</span>
            <button
              onclick={() => onSelectRubyVersion(ver)}
              class="px-2.5 py-1 rounded-md text-xs font-medium transition-all {activeRubyVersion === ver
                ? 'bg-slate-900 text-white shadow-xs'
                : 'bg-white hover:bg-slate-100 text-slate-700 border border-slate-200 shadow-xs'}"
            >
              {activeRubyVersion === ver ? "Active" : "Switch"}
            </button>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
