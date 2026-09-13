<script lang="ts">
  import { CheckCircle2, Copy, Database, ExternalLink, Globe, X } from "@lucide/svelte";

  let {
    show,
    onClose,
    onOpenAdminer,
    onLaunchNative,
    onCopyUrl,
    onCreateDatabase,
    onCopyEnv,
  }: {
    show: boolean;
    onClose: () => void;
    onOpenAdminer: () => void;
    onLaunchNative: () => void;
    onCopyUrl: () => void;
    onCreateDatabase: (name: string) => void;
    onCopyEnv?: () => void;
  } = $props();

  let newDbName = $state("");

  function handleCreate() {
    if (newDbName.trim()) {
      onCreateDatabase(newDbName.trim());
      newDbName = "";
    }
  }
</script>

{#if show}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
    <div class="bg-[#101827] border border-slate-800 w-full max-w-md rounded-2xl p-6 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
            <Database class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-base text-white">Database Quick Access</h3>
            <p class="text-xs text-slate-400">MariaDB / MySQL Local Connection</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="text-slate-400 hover:text-white p-1 rounded-lg hover:bg-slate-800 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="rounded-xl border border-slate-800/80 bg-[#0B101C]/80 p-3.5 space-y-2">
        <div class="flex items-center justify-between">
          <span class="text-xs font-bold text-emerald-300">Quick Create Database</span>
          <span class="text-[10px] text-slate-500 font-mono">UTF8MB4 Unicode</span>
        </div>
        <form onsubmit={(e) => { e.preventDefault(); handleCreate(); }} class="flex items-center gap-2">
          <input
            type="text"
            bind:value={newDbName}
            placeholder="e.g. db_ecommerce, blog_db"
            class="flex-1 px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-700 text-xs text-slate-200 font-mono focus:outline-none focus:border-emerald-500"
          />
          <button
            type="submit"
            class="px-3.5 py-1.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold transition-colors shrink-0"
          >
            Create
          </button>
        </form>
      </div>

      <div class="space-y-2 text-xs bg-slate-950/60 rounded-xl p-3.5 border border-slate-800/80">
        <div class="flex justify-between items-center py-0.5 border-b border-slate-800/50">
          <span class="text-slate-400">Host</span>
          <span class="font-mono text-cyan-400 font-semibold">127.0.0.1 (localhost)</span>
        </div>
        <div class="flex justify-between items-center py-0.5 border-b border-slate-800/50">
          <span class="text-slate-400">Port</span>
          <span class="font-mono text-emerald-400 font-semibold">3306</span>
        </div>
        <div class="flex justify-between items-center py-0.5 border-b border-slate-800/50">
          <span class="text-slate-400">Username</span>
          <span class="font-mono text-slate-200 font-semibold">root</span>
        </div>
        <div class="flex justify-between items-center py-0.5 border-b border-slate-800/50">
          <span class="text-slate-400">Password</span>
          <span class="font-mono text-slate-400 italic">(none / blank)</span>
        </div>
        <div class="flex justify-between items-center py-0.5">
          <span class="text-slate-400">Driver</span>
          <span class="font-mono text-amber-400">MySQL / MariaDB</span>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-2.5">
        <button
          onclick={onOpenAdminer}
          class="flex items-center justify-center gap-2 p-2.5 rounded-xl border border-indigo-500/30 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 hover:text-white text-xs font-semibold transition-colors shadow-sm"
        >
          <Globe class="w-4 h-4 text-indigo-400" />
          Open Adminer.php
        </button>
        <button
          onclick={onLaunchNative}
          class="flex items-center justify-center gap-2 p-2.5 rounded-xl border border-emerald-500/30 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-300 hover:text-white text-xs font-semibold transition-colors shadow-sm"
        >
          <ExternalLink class="w-4 h-4 text-emerald-400" />
          Launch Desktop App
        </button>
      </div>

      <div class="flex items-center justify-between gap-3 pt-1">
        <button
          onclick={onCopyUrl}
          class="px-3 py-1.5 rounded-xl text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 transition-colors"
        >
          Copy Adminer URL
        </button>
        <button
          onclick={onClose}
          class="px-5 py-1.5 rounded-xl text-xs font-medium bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-lg shadow-emerald-500/20 transition-all"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
