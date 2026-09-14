<script lang="ts">
  import { Eye, EyeOff, Plus, RefreshCw } from "@lucide/svelte";
  import type { UserDatabaseItem } from "../../types";

  let {
    eligibleDatabases = [],
    isCreating = false,
    onCancel,
    onSubmit,
  }: {
    eligibleDatabases: UserDatabaseItem[];
    isCreating: boolean;
    onCancel: () => void;
    onSubmit: (params: {
      username: string;
      host: string;
      password: string;
      privilegeScope: "all" | "database" | "readonly";
      targetDb?: string;
    }) => Promise<void>;
  } = $props();

  let newUsername = $state("");
  let newHost = $state("%");
  let newPassword = $state("");
  let showNewPassword = $state(false);
  let newPrivilegeScope = $state<"all" | "database" | "readonly">("all");
  let newTargetDb = $state("");

  function generatePassword() {
    const chars = "abcdefghjkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789!@#$%^&*";
    let generated = "";
    const array = new Uint32Array(16);
    crypto.getRandomValues(array);
    for (let i = 0; i < 16; i++) {
      generated += chars[array[i] % chars.length];
    }
    newPassword = generated;
    showNewPassword = true;
  }

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    onSubmit({
      username: newUsername.trim(),
      host: newHost.trim() || "%",
      password: newPassword,
      privilegeScope: newPrivilegeScope,
      targetDb: newPrivilegeScope === "database" ? newTargetDb : undefined,
    });
  }
</script>

<form onsubmit={handleSubmit} class="space-y-4">
  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
    <div>
      <label for="create-db-user-name" class="block text-[11px] font-medium text-slate-700 mb-1">
        Username *
      </label>
      <input
        id="create-db-user-name"
        type="text"
        bind:value={newUsername}
        placeholder="e.g. app_user"
        required
        class="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-900 text-xs font-mono focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
      />
    </div>

    <div>
      <label for="create-db-user-host" class="block text-[11px] font-medium text-slate-700 mb-1">
        Host * <span class="text-slate-400 font-normal">(% for any host)</span>
      </label>
      <div class="flex items-center gap-1.5">
        <input
          id="create-db-user-host"
          type="text"
          bind:value={newHost}
          placeholder="%"
          required
          class="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-900 text-xs font-mono focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
        />
        <button
          type="button"
          onclick={() => (newHost = "%")}
          class="px-2 py-1.5 rounded-lg border border-slate-200 bg-slate-50 text-[11px] font-mono text-slate-600 hover:bg-slate-100"
        >
          %
        </button>
        <button
          type="button"
          onclick={() => (newHost = "localhost")}
          class="px-2 py-1.5 rounded-lg border border-slate-200 bg-slate-50 text-[11px] font-mono text-slate-600 hover:bg-slate-100"
        >
          local
        </button>
      </div>
    </div>
  </div>

  <div>
    <div class="flex items-center justify-between mb-1">
      <label for="create-db-user-password" class="block text-[11px] font-medium text-slate-700">
        Password
      </label>
      <button
        type="button"
        onclick={generatePassword}
        class="text-[11px] font-medium text-[#94380C] hover:underline"
      >
        Generate Strong Password
      </button>
    </div>
    <div class="relative">
      <input
        id="create-db-user-password"
        type={showNewPassword ? "text" : "password"}
        bind:value={newPassword}
        placeholder="Enter or generate password"
        class="w-full pl-3 pr-9 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-900 text-xs font-mono focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
      />
      <button
        type="button"
        onclick={() => (showNewPassword = !showNewPassword)}
        class="absolute right-2.5 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 p-0.5"
      >
        {#if showNewPassword}
          <EyeOff class="w-3.5 h-3.5" />
        {:else}
          <Eye class="w-3.5 h-3.5" />
        {/if}
      </button>
    </div>
  </div>

  <div>
    <span class="block text-[11px] font-medium text-slate-700 mb-1.5">Privilege Scope</span>
    <div class="grid grid-cols-3 gap-2">
      <button
        type="button"
        onclick={() => (newPrivilegeScope = "all")}
        class="p-2.5 rounded-xl border text-left transition-all {newPrivilegeScope === 'all'
          ? 'border-[#94380C] bg-amber-50/60 ring-1 ring-[#94380C]'
          : 'border-slate-200 bg-white hover:bg-slate-50'}"
      >
        <div class="font-bold text-xs text-slate-900">Full Administrator</div>
        <div class="text-[10px] text-slate-500 mt-0.5">ALL PRIVILEGES on *.*</div>
      </button>

      <button
        type="button"
        onclick={() => (newPrivilegeScope = "database")}
        class="p-2.5 rounded-xl border text-left transition-all {newPrivilegeScope === 'database'
          ? 'border-[#94380C] bg-amber-50/60 ring-1 ring-[#94380C]'
          : 'border-slate-200 bg-white hover:bg-slate-50'}"
      >
        <div class="font-bold text-xs text-slate-900">Specific Database</div>
        <div class="text-[10px] text-slate-500 mt-0.5">ALL on target_db.*</div>
      </button>

      <button
        type="button"
        onclick={() => (newPrivilegeScope = "readonly")}
        class="p-2.5 rounded-xl border text-left transition-all {newPrivilegeScope === 'readonly'
          ? 'border-[#94380C] bg-amber-50/60 ring-1 ring-[#94380C]'
          : 'border-slate-200 bg-white hover:bg-slate-50'}"
      >
        <div class="font-bold text-xs text-slate-900">Read-Only</div>
        <div class="text-[10px] text-slate-500 mt-0.5">SELECT on *.*</div>
      </button>
    </div>
  </div>

  {#if newPrivilegeScope === "database"}
    <div>
      <label for="create-db-target-select" class="block text-[11px] font-medium text-slate-700 mb-1">
        Target Database *
      </label>
      <select
        id="create-db-target-select"
        bind:value={newTargetDb}
        required
        class="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-900 text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
      >
        <option value="" disabled>Select target database...</option>
        {#each eligibleDatabases as d}
          <option value={d.name}>{d.name}</option>
        {/each}
      </select>
    </div>
  {/if}

  <div class="pt-2 flex items-center justify-end gap-2 border-t border-slate-100">
    <button
      type="button"
      onclick={onCancel}
      class="px-4 py-2 rounded-lg border border-slate-200 text-xs font-medium text-slate-600 hover:bg-slate-50 transition-colors"
    >
      Cancel
    </button>

    <button
      type="submit"
      disabled={isCreating}
      class="flex items-center gap-1.5 px-4 py-2 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs transition-colors disabled:opacity-50"
    >
      {#if isCreating}
        <RefreshCw class="w-3.5 h-3.5 animate-spin" />
        <span>Creating User...</span>
      {:else}
        <Plus class="w-3.5 h-3.5" />
        <span>Create User Account</span>
      {/if}
    </button>
  </div>
</form>
