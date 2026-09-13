<script lang="ts">
  import {
    CheckCircle2,
    Globe,
    HardDrive,
    Laptop,
    Settings,
    Shield,
    X,
  } from "@lucide/svelte";

  let {
    show,
    onClose,
    domainSuffix = "test",
    onSaveDomainSuffix,
  }: {
    show: boolean;
    onClose: () => void;
    domainSuffix?: string;
    onSaveDomainSuffix?: (suffix: string) => void;
  } = $props();

  let selectedSuffix = $state("test");
  let selectedEditor = $state("notepad");
  let autoStartServices = $state(false);
  let showSavedToast = $state(false);

  $effect(() => {
    selectedSuffix = domainSuffix;
  });

  function handleSave() {
    if (onSaveDomainSuffix) {
      onSaveDomainSuffix(selectedSuffix);
    }
    showSavedToast = true;
    setTimeout(() => {
      showSavedToast = false;
      onClose();
    }, 600);
  }
</script>

{#if show}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 backdrop-blur-xs p-4">
    <div class="bg-white border border-slate-200 w-full max-w-lg rounded-xl p-6 space-y-5 shadow-xl text-slate-900">
      <div class="flex items-center justify-between border-b border-slate-100 pb-3">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Settings class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-semibold text-sm text-slate-900">4Forge Preferences & Settings</h3>
            <p class="text-xs text-slate-500">Global environment configuration and runtime paths</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="text-slate-400 hover:text-slate-700 p-1 rounded-md hover:bg-slate-100 transition-colors"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="space-y-4 text-xs">
        <div class="space-y-1.5">
          <label for="domain-suffix" class="font-semibold text-slate-800 flex items-center gap-1.5">
            <Globe class="w-3.5 h-3.5 text-slate-500" />
            Default Virtual Host Domain Suffix
          </label>
          <div class="grid grid-cols-3 gap-2">
            {#each ["test", "localhost", "local"] as sfx}
              <button
                type="button"
                onclick={() => (selectedSuffix = sfx)}
                class="px-3 py-2 rounded-lg border text-xs font-mono font-medium transition-all text-center {selectedSuffix === sfx
                  ? 'border-slate-900 bg-slate-900 text-white shadow-xs'
                  : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50'}"
              >
                .{sfx}
              </button>
            {/each}
          </div>
        </div>

        <div class="space-y-1.5">
          <label for="editor-pref" class="font-semibold text-slate-800 flex items-center gap-1.5">
            <Laptop class="w-3.5 h-3.5 text-slate-500" />
            Default Config & Project Editor
          </label>
          <select
            id="editor-pref"
            bind:value={selectedEditor}
            class="w-full px-3 py-2 rounded-lg border border-slate-200 bg-white text-slate-800 text-xs focus:outline-none focus:border-slate-900"
          >
            <option value="notepad">Windows Notepad (Default)</option>
            <option value="code">Visual Studio Code (code.exe)</option>
            <option value="cursor">Cursor AI Editor (cursor.exe)</option>
            <option value="sublime">Sublime Text (subl.exe)</option>
          </select>
        </div>

        <div class="space-y-1.5 bg-slate-50 p-3 rounded-lg border border-slate-200">
          <span class="font-semibold text-slate-800 flex items-center gap-1.5">
            <HardDrive class="w-3.5 h-3.5 text-slate-500" />
            System Runtime Root Directories
          </span>
          <div class="space-y-1 font-mono text-[11px] text-slate-600">
            <div class="flex justify-between items-center py-0.5">
              <span>Primary Runtimes:</span>
              <span class="text-slate-900 font-medium">C:\4forge\runtimes</span>
            </div>
            <div class="flex justify-between items-center py-0.5">
              <span>AppData Fallback:</span>
              <span class="text-slate-900 font-medium">%LOCALAPPDATA%\4Forge\runtimes</span>
            </div>
            <div class="flex justify-between items-center py-0.5">
              <span>Projects Workspace:</span>
              <span class="text-slate-900 font-medium">C:\4forge\projects</span>
            </div>
          </div>
        </div>

        <div class="flex items-center justify-between p-3 rounded-lg border border-slate-200 bg-white">
          <div class="flex items-center gap-2">
            <Shield class="w-4 h-4 text-emerald-600" />
            <div>
              <div class="font-semibold text-slate-800">Auto-Start Core Services</div>
              <div class="text-[11px] text-slate-500">Automatically boot Caddy & MariaDB on 4Forge launch</div>
            </div>
          </div>
          <input
            type="checkbox"
            bind:checked={autoStartServices}
            class="w-4 h-4 accent-slate-900 rounded cursor-pointer"
          />
        </div>
      </div>

      <div class="flex items-center justify-between pt-3 border-t border-slate-100">
        {#if showSavedToast}
          <span class="text-xs text-emerald-600 font-medium flex items-center gap-1">
            <CheckCircle2 class="w-3.5 h-3.5" /> Preferences saved!
          </span>
        {:else}
          <span class="text-[11px] text-slate-400 font-mono">4Forge v0.1.0 • Apache-2.0</span>
        {/if}
        <div class="flex items-center gap-2">
          <button
            onclick={onClose}
            class="px-3.5 py-1.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 transition-colors"
          >
            Cancel
          </button>
          <button
            onclick={handleSave}
            class="px-4 py-1.5 rounded-lg text-xs font-semibold bg-slate-900 hover:bg-slate-800 text-white shadow-xs transition-colors"
          >
            Save Changes
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
