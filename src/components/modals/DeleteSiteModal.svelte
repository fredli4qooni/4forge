<script lang="ts">
  import { AlertTriangle, Folder, Globe, Trash2, X } from "@lucide/svelte";
  import type { SiteItem } from "../../types";

  let {
    show,
    site,
    onClose,
    onConfirm,
  }: {
    show: boolean;
    site: SiteItem | null;
    onClose: () => void;
    onConfirm: (deleteFiles: boolean) => void;
  } = $props();

  let deleteFiles = $state(true);
</script>

{#if show && site}
  <div class="fixed inset-0 z-50 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="rounded-xl border border-slate-200 bg-white w-full max-w-md p-6 shadow-xl space-y-5 text-slate-900">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-rose-50 text-rose-600 border border-rose-100">
            <AlertTriangle class="w-5 h-5" />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-slate-900">Delete Project</h3>
            <p class="text-xs text-slate-500 font-mono">{site.domain}</p>
          </div>
        </div>
        <button onclick={onClose} class="text-slate-400 hover:text-slate-700 p-1 rounded-md hover:bg-slate-100 transition-colors">
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="p-3 bg-slate-50 border border-slate-200 rounded-lg text-xs space-y-1.5 font-mono text-slate-600">
        <div class="flex items-center gap-2">
          <Globe class="w-3.5 h-3.5 text-slate-400 shrink-0" />
          <span class="font-medium text-slate-800">{site.domain}</span>
        </div>
        <div class="flex items-center gap-2 truncate">
          <Folder class="w-3.5 h-3.5 text-slate-400 shrink-0" />
          <span class="truncate text-slate-500">{site.path}</span>
        </div>
      </div>

      <div class="space-y-2.5 text-xs">
        <label
          class="flex items-start gap-3 p-3 rounded-lg border transition-all cursor-pointer {deleteFiles ? 'border-rose-300 bg-rose-50/50' : 'border-slate-200 hover:border-slate-300 bg-white'}"
        >
          <input
            type="radio"
            name="deleteOption"
            checked={deleteFiles}
            onchange={() => (deleteFiles = true)}
            class="mt-0.5 text-rose-600 focus:ring-rose-500"
          />
          <div>
            <span class="font-semibold text-slate-900">Delete Project & Files (Clean Delete)</span>
            <p class="text-slate-500 text-[11px] mt-0.5">Permanently deletes all project files from disk and removes the virtual host configuration.</p>
          </div>
        </label>

        <label
          class="flex items-start gap-3 p-3 rounded-lg border transition-all cursor-pointer {!deleteFiles ? 'border-blue-300 bg-blue-50/50' : 'border-slate-200 hover:border-slate-300 bg-white'}"
        >
          <input
            type="radio"
            name="deleteOption"
            checked={!deleteFiles}
            onchange={() => (deleteFiles = false)}
            class="mt-0.5 text-blue-600 focus:ring-blue-500"
          />
          <div>
            <span class="font-semibold text-slate-900">Unlink Virtual Host Only</span>
            <p class="text-slate-500 text-[11px] mt-0.5">Removes the local domain and Caddy proxy. Project files on disk will be preserved.</p>
          </div>
        </label>
      </div>

      <div class="flex items-center justify-end gap-2 pt-2 border-t border-slate-100">
        <button
          onclick={onClose}
          class="px-3.5 py-1.5 rounded-lg border border-slate-200 text-xs font-medium text-slate-600 hover:bg-slate-50 transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={() => onConfirm(deleteFiles)}
          class="px-4 py-1.5 rounded-lg text-xs font-semibold text-white shadow-xs transition-colors flex items-center gap-1.5 {deleteFiles ? 'bg-rose-600 hover:bg-rose-700' : 'bg-slate-800 hover:bg-slate-900'}"
        >
          <Trash2 class="w-3.5 h-3.5" />
          <span>{deleteFiles ? 'Delete Files & Virtual Host' : 'Unlink Virtual Host'}</span>
        </button>
      </div>
    </div>
  </div>
{/if}
