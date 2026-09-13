<script lang="ts">
  import { Database, X } from "@lucide/svelte";

  let {
    show,
    onClose,
    onCreate,
    onShowToast,
  }: {
    show: boolean;
    onClose: () => void;
    onCreate: (name: string, engine: string) => Promise<void>;
    onShowToast: (msg: string) => void;
  } = $props();

  let newDbName = $state("");
  let selectedEngine = $state("mariadb");
  let isSubmitting = $state(false);

  async function handleSubmit() {
    if (!newDbName.trim()) return;
    isSubmitting = true;
    try {
      await onCreate(newDbName.trim(), selectedEngine);
      newDbName = "";
      onClose();
    } catch (err: any) {
      onShowToast(`Failed to create database: ${err?.message || err}`);
    } finally {
      isSubmitting = false;
    }
  }
</script>

{#if show}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 backdrop-blur-xs p-4">
    <div class="bg-white border border-slate-200 w-full max-w-md rounded-xl p-6 space-y-4 shadow-xl text-slate-900">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div class="p-2 rounded-lg bg-amber-50 text-[#94380C] border border-amber-200">
            <Database class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-sm text-slate-900">Create New Database</h3>
            <p class="text-xs text-slate-500">Pick engine and database name</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="text-slate-400 hover:text-slate-700 p-1 rounded-md hover:bg-slate-100 transition-colors"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-3">
        <div>
          <label for="db-name-input" class="block text-xs font-semibold text-slate-700 mb-1">Database Name</label>
          <input
            id="db-name-input"
            type="text"
            bind:value={newDbName}
            placeholder="e.g. laravel_app, my_store"
            class="w-full px-3 py-2 rounded-lg bg-white border border-slate-200 text-xs font-mono text-slate-900 focus:outline-none focus:border-slate-900 shadow-xs"
            required
          />
        </div>

        <div>
          <label for="db-engine-select" class="block text-xs font-semibold text-slate-700 mb-1">Database Engine</label>
          <select
            id="db-engine-select"
            bind:value={selectedEngine}
            class="w-full px-3 py-2 rounded-lg bg-white border border-slate-200 text-xs font-medium text-slate-900 focus:outline-none focus:border-slate-900 shadow-xs cursor-pointer"
          >
            <option value="mariadb">MySQL / MariaDB (Port 3306)</option>
            <option value="postgresql">PostgreSQL (Port 5432)</option>
            <option value="sqlite">SQLite (Embedded / File-based)</option>
            <option value="mongodb">MongoDB (Port 27017)</option>
          </select>
        </div>

        <div class="bg-slate-50 rounded-lg p-3 text-xs text-slate-600 border border-slate-200 space-y-1">
          <div class="flex justify-between items-center">
            <span class="text-slate-500">Default Host:</span>
            <span class="font-mono font-medium text-slate-800">127.0.0.1</span>
          </div>
          <div class="flex justify-between items-center">
            <span class="text-slate-500">Charset:</span>
            <span class="font-mono font-medium text-slate-800">utf8mb4 / UTF8 Unicode</span>
          </div>
        </div>

        <div class="flex items-center justify-end gap-2 pt-3 border-t border-slate-100">
          <button
            type="button"
            onclick={onClose}
            class="px-3.5 py-1.5 rounded-lg border border-slate-200 text-xs font-medium text-slate-600 hover:bg-slate-50 transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isSubmitting || !newDbName.trim()}
            class="px-4 py-1.5 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs disabled:opacity-50 transition-colors"
          >
            {isSubmitting ? "Creating..." : "Create Database"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
