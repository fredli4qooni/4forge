<script lang="ts">
  import { Eye, EyeOff, KeyRound, RefreshCw, X } from "@lucide/svelte";
  import type { DatabaseUserItem } from "../../lib/actions";

  let {
    user,
    isOpen = false,
    isUpdating = false,
    onClose,
    onSave,
  }: {
    user: DatabaseUserItem | null;
    isOpen: boolean;
    isUpdating: boolean;
    onClose: () => void;
    onSave: (newPassword: string) => Promise<void>;
  } = $props();

  let password = $state("");
  let showPassword = $state(false);

  function handleClose() {
    password = "";
    showPassword = false;
    onClose();
  }

  function generatePassword() {
    const chars = "abcdefghjkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789!@#$%^&*";
    let generated = "";
    const array = new Uint32Array(16);
    crypto.getRandomValues(array);
    for (let i = 0; i < 16; i++) {
      generated += chars[array[i] % chars.length];
    }
    password = generated;
    showPassword = true;
  }

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    onSave(password);
  }
</script>

{#if isOpen && user}
  <div
    class="fixed inset-0 z-[70] flex items-center justify-center bg-slate-900/60 backdrop-blur-sm p-4"
    style="z-index: 70;"
  >
    <div class="bg-white border border-slate-200 w-full max-w-sm rounded-xl p-5 shadow-2xl space-y-4 text-slate-900">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="w-7 h-7 rounded-lg bg-amber-50 text-[#94380C] border border-amber-200 flex items-center justify-center">
            <KeyRound class="w-3.5 h-3.5" />
          </div>
          <div>
            <h3 class="font-bold text-xs text-slate-900">Change Password</h3>
            <p class="text-[10px] text-slate-400 font-mono">{user.username}@{user.host}</p>
          </div>
        </div>
        <button
          type="button"
          onclick={handleClose}
          class="p-1 text-slate-400 hover:text-slate-600 rounded"
        >
          <X class="w-3.5 h-3.5" />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="space-y-4">
        <div class="space-y-1">
          <div class="flex items-center justify-between">
            <label for="change-password-input" class="block text-[11px] font-medium text-slate-700">New Password</label>
            <button
              type="button"
              onclick={generatePassword}
              class="text-[10px] font-medium text-[#94380C] hover:underline"
            >
              Generate
            </button>
          </div>
          <div class="relative">
            <input
              id="change-password-input"
              type={showPassword ? "text" : "password"}
              bind:value={password}
              placeholder="New password"
              class="w-full pl-3 pr-8 py-1.5 bg-white border border-slate-200 rounded-lg text-xs font-mono text-slate-900 focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
            />
            <button
              type="button"
              onclick={() => (showPassword = !showPassword)}
              class="absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 p-0.5"
            >
              {#if showPassword}
                <EyeOff class="w-3.5 h-3.5" />
              {:else}
                <Eye class="w-3.5 h-3.5" />
              {/if}
            </button>
          </div>
        </div>

        <div class="flex items-center justify-end gap-2 pt-2">
          <button
            type="button"
            onclick={handleClose}
            class="px-3 py-1.5 rounded-lg border border-slate-200 text-xs font-medium text-slate-600 hover:bg-slate-50"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isUpdating}
            class="px-3 py-1.5 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs transition-colors disabled:opacity-50 flex items-center gap-1"
          >
            {#if isUpdating}
              <RefreshCw class="w-3 h-3 animate-spin" />
              <span>Updating...</span>
            {:else}
              <span>Save Password</span>
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
