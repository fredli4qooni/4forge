<script lang="ts">
  import { CheckCircle2, RefreshCw, ShieldCheck, Sparkles, X } from "@lucide/svelte";
  import type { UpdateCheck } from "../../types";

  let {
    show,
    updateStatus,
    isCheckingUpdate,
    onClose,
    onCheckUpdates,
  }: {
    show: boolean;
    updateStatus: UpdateCheck | null;
    isCheckingUpdate: boolean;
    onClose: () => void;
    onCheckUpdates: () => void;
  } = $props();
</script>

{#if show}
  <div class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs z-50 flex items-center justify-center p-4">
    <div class="bg-white border border-slate-200 rounded-xl w-full max-w-md p-6 space-y-4 shadow-xl text-slate-900">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Sparkles class="w-4 h-4" />
          </div>
          <div>
            <h3 class="font-semibold text-sm text-slate-900">4Forge Release & Security</h3>
            <p class="text-xs text-slate-500">Release Automation & Supply Chain Integrity</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="text-slate-400 hover:text-slate-700 p-1 rounded-md hover:bg-slate-100 transition-colors"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="space-y-2 text-xs bg-slate-50 rounded-lg p-3.5 border border-slate-200 text-slate-600">
        <div class="flex justify-between items-center py-1 border-b border-slate-200/70">
          <span class="text-slate-500">Installed Version</span>
          <span class="font-mono text-slate-900 font-medium">{updateStatus ? `v${updateStatus.current_version}` : "v0.1.0"}</span>
        </div>
        <div class="flex justify-between items-center py-1 border-b border-slate-200/70">
          <span class="text-slate-500">Release Channel</span>
          <span class="font-mono text-slate-800 capitalize font-medium">{updateStatus ? updateStatus.channel : "stable"}</span>
        </div>
        <div class="flex justify-between items-center py-1 border-b border-slate-200/70">
          <span class="text-slate-500">Signature Verification</span>
          <span class="text-emerald-700 bg-emerald-50 px-1.5 py-0.5 rounded border border-emerald-200 flex items-center gap-1 font-mono text-[10px] font-medium">
            <ShieldCheck class="w-3 h-3 text-emerald-600" /> Minisign & Authenticode
          </span>
        </div>
        <div class="flex justify-between items-center py-1 border-b border-slate-200/70">
          <span class="text-slate-500">Supply Chain SBOM</span>
          <span class="text-emerald-700 bg-emerald-50 px-1.5 py-0.5 rounded border border-emerald-200 flex items-center gap-1 font-mono text-[10px] font-medium">
            <CheckCircle2 class="w-3 h-3 text-emerald-600" /> SPDX & CycloneDX
          </span>
        </div>
        <div class="flex justify-between items-center py-1">
          <span class="text-slate-500">Release Status</span>
          <span class="text-slate-900 font-medium">
            {updateStatus?.update_available ? "New Update Available" : "Up to Date (Latest Verified)"}
          </span>
        </div>
      </div>

      {#if updateStatus?.release_notes}
        <div class="p-3 rounded-lg bg-slate-50 border border-slate-200 text-xs text-slate-700">
          <p>{updateStatus.release_notes}</p>
        </div>
      {/if}

      <div class="flex items-center justify-end gap-2.5 pt-2 border-t border-slate-100">
        <button
          onclick={onCheckUpdates}
          disabled={isCheckingUpdate}
          class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center gap-1.5 transition-colors"
        >
          <RefreshCw class="w-3.5 h-3.5 {isCheckingUpdate ? 'animate-spin' : ''}" />
          <span>{isCheckingUpdate ? "Checking..." : "Re-check"}</span>
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
