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
    onCreateDatabase: (name: string, engine?: string) => void;
    onCopyEnv?: () => void;
  } = $props();

  let newDbName = $state("");
  let selectedEngine = $state("MariaDB");

  function handleCreate() {
    if (newDbName.trim()) {
      onCreateDatabase(newDbName.trim(), selectedEngine.toLowerCase());
      newDbName = "";
    }
  }
</script>

{#if show}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 backdrop-blur-xs p-4">
    <div class="bg-white border border-slate-200 w-full max-w-md rounded-xl p-6 space-y-4 shadow-xl text-slate-900">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Database class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-semibold text-sm text-slate-900">Database Quick Access</h3>
            <p class="text-xs text-slate-500">MariaDB / MySQL / PostgreSQL / MongoDB / SQLite</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="text-slate-400 hover:text-slate-700 p-1 rounded-md hover:bg-slate-100 transition-colors"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="rounded-lg border border-slate-200 bg-slate-50 p-3 space-y-2">
        <div class="flex items-center justify-between">
          <span class="text-xs font-semibold text-slate-800">Quick Create Database</span>
          <span class="text-[10px] text-slate-500 font-mono">UTF8MB4 Unicode</span>
        </div>
        <form onsubmit={(e) => { e.preventDefault(); handleCreate(); }} class="flex items-center gap-2">
          <input
            type="text"
            bind:value={newDbName}
            placeholder="e.g. db_ecommerce, blog_db"
            class="flex-1 px-3 py-1.5 rounded-lg bg-white border border-slate-200 text-xs text-slate-900 font-mono focus:outline-none focus:border-slate-900 focus:ring-1 focus:ring-slate-900"
          />
          <select
            bind:value={selectedEngine}
            class="px-2 py-1.5 rounded-lg border border-slate-200 text-xs font-medium text-slate-800 bg-white focus:outline-none focus:border-slate-900 cursor-pointer"
          >
            <option value="MariaDB">MariaDB</option>
            <option value="MySQL">MySQL</option>
            <option value="PostgreSQL">PostgreSQL</option>
            <option value="MongoDB">MongoDB</option>
            <option value="SQLite">SQLite</option>
          </select>
          <button
            type="submit"
            class="px-3.5 py-1.5 rounded-lg bg-slate-900 hover:bg-slate-800 text-white text-xs font-semibold shadow-xs transition-colors shrink-0"
          >
            Create
          </button>
        </form>
      </div>

      <div class="space-y-1.5 text-xs bg-slate-50 rounded-lg p-3 border border-slate-200 text-slate-600">
        <div class="flex justify-between items-center py-0.5 border-b border-slate-200/70">
          <span class="text-slate-500">Host</span>
          <span class="font-mono text-slate-900 font-medium">127.0.0.1 (localhost)</span>
        </div>
        <div class="flex justify-between items-center py-0.5 border-b border-slate-200/70">
          <span class="text-slate-500">Port</span>
          <span class="font-mono text-slate-900 font-medium">3306</span>
        </div>
        <div class="flex justify-between items-center py-0.5 border-b border-slate-200/70">
          <span class="text-slate-500">Username</span>
          <span class="font-mono text-slate-900 font-medium">root</span>
        </div>
        <div class="flex justify-between items-center py-0.5 border-b border-slate-200/70">
          <span class="text-slate-500">Password</span>
          <span class="font-mono text-slate-400 italic">(none / blank)</span>
        </div>
        <div class="flex justify-between items-center py-0.5">
          <span class="text-slate-500">Driver</span>
          <span class="font-mono text-slate-800 font-medium">MySQL / MariaDB</span>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-2">
        <button
          onclick={onOpenAdminer}
          class="flex items-center justify-center gap-2 p-2 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 hover:text-slate-900 text-xs font-medium transition-colors shadow-xs"
        >
          <Globe class="w-4 h-4 text-slate-500" />
          Open Adminer.php
        </button>
        <button
          onclick={onLaunchNative}
          class="flex items-center justify-center gap-2 p-2 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 hover:text-slate-900 text-xs font-medium transition-colors shadow-xs"
        >
          <ExternalLink class="w-4 h-4 text-slate-500" />
          Launch Desktop App
        </button>
      </div>

      <div class="flex items-center justify-between gap-3 pt-2 border-t border-slate-100">
        <button
          onclick={onCopyUrl}
          class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs transition-colors"
        >
          Copy Adminer URL
        </button>
        <button
          onclick={onClose}
          class="px-4 py-1.5 rounded-lg text-xs font-semibold bg-slate-900 hover:bg-slate-800 text-white shadow-xs transition-all"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
