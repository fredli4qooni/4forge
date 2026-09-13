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
  <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-800 rounded-2xl w-full max-w-md p-6 space-y-5 shadow-2xl shadow-cyan-950/40">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-cyan-500/15 text-cyan-400 border border-cyan-500/30">
            <Sparkles class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-base text-white">4Forge Release & Security</h3>
            <p class="text-xs text-slate-400">Release Automation & Supply Chain Integrity</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="text-slate-400 hover:text-white p-1 rounded-lg hover:bg-slate-800 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="space-y-3 text-xs bg-slate-950/60 rounded-xl p-4 border border-slate-800/80">
        <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
          <span class="text-slate-400">Installed Version</span>
          <span class="font-mono text-cyan-400 font-semibold">{updateStatus ? `v${updateStatus.current_version}` : "v0.1.0"}</span>
        </div>
        <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
          <span class="text-slate-400">Release Channel</span>
          <span class="font-mono text-slate-200 capitalize">{updateStatus ? updateStatus.channel : "stable"}</span>
        </div>
        <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
          <span class="text-slate-400">Signature Verification</span>
          <span class="text-emerald-400 flex items-center gap-1 font-mono">
            <ShieldCheck class="w-3.5 h-3.5" /> Authenticode & Minisign
          </span>
        </div>
        <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
          <span class="text-slate-400">Supply Chain SBOM</span>
          <span class="text-emerald-400 flex items-center gap-1 font-mono">
            <CheckCircle2 class="w-3.5 h-3.5" /> SPDX & CycloneDX (Syft)
          </span>
        </div>
        <div class="flex justify-between items-center py-1">
          <span class="text-slate-400">Release Status</span>
          <span class="text-cyan-300 font-medium">
            {updateStatus?.update_available ? "New Update Available" : "Up to Date (Latest Verified)"}
          </span>
        </div>
      </div>

      {#if updateStatus?.release_notes}
        <div class="p-3 rounded-xl bg-cyan-950/20 border border-cyan-500/20 text-xs text-cyan-200">
          <p>{updateStatus.release_notes}</p>
        </div>
      {/if}

      <div class="flex items-center justify-end gap-3 pt-2">
        <button
          onclick={onCheckUpdates}
          disabled={isCheckingUpdate}
          class="px-4 py-2 rounded-xl text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 flex items-center gap-2 transition-colors"
        >
          <RefreshCw class="w-3.5 h-3.5 {isCheckingUpdate ? 'animate-spin' : ''}" />
          {isCheckingUpdate ? "Checking..." : "Re-check"}
        </button>
        <button
          onclick={onClose}
          class="px-4 py-2 rounded-xl text-xs font-medium bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:brightness-110 shadow-lg shadow-cyan-500/20 transition-all"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
