<script lang="ts">
  import {
    AlertTriangle,
    KeyRound,
    Plus,
    RefreshCw,
    Trash2,
    User,
    Users,
    X,
  } from "@lucide/svelte";
  import type { UserDatabaseItem } from "../../types";
  import {
    createDatabaseUser,
    dropDatabaseUser,
    fetchDatabaseUsers,
    updateDatabaseUserPassword,
    type DatabaseUserItem,
  } from "../../lib/actions";
  import mariaLogo from "../../assets/logos/mysql-logo.svg";
  import postgresLogo from "../../assets/logos/postgresql-logo.svg";
  import CreateUserForm from "./CreateUserForm.svelte";
  import ChangePasswordModal from "./ChangePasswordModal.svelte";

  let {
    databases = [],
    isOpen = false,
    onClose,
    onShowToast,
  }: {
    databases: UserDatabaseItem[];
    isOpen: boolean;
    onClose: () => void;
    onShowToast: (msg: string) => void;
  } = $props();

  let selectedEngine = $state<string>("mariadb");
  let activeView = $state<"list" | "create">("list");
  let users = $state<DatabaseUserItem[]>([]);
  let isLoading = $state(false);
  let errorMessage = $state<string | null>(null);

  let isCreating = $state(false);
  let userToChangePassword = $state<DatabaseUserItem | null>(null);
  let isUpdatingPassword = $state(false);

  let userPendingDelete = $state<DatabaseUserItem | null>(null);
  let isDeleting = $state(false);

  let eligibleDatabases = $derived(
    databases.filter((db) => {
      if (selectedEngine === "mariadb") {
        return db.engine === "mariadb" || db.engine === "mysql";
      }
      return db.engine === "postgresql" || db.engine === "postgres";
    })
  );

  $effect(() => {
    if (isOpen) {
      loadUsers();
    }
  });

  async function loadUsers() {
    isLoading = true;
    errorMessage = null;
    try {
      users = await fetchDatabaseUsers(selectedEngine);
    } catch (err: any) {
      errorMessage = err?.message || String(err);
      users = [];
    } finally {
      isLoading = false;
    }
  }

  function handleEngineSwitch(engine: string) {
    if (selectedEngine === engine) return;
    selectedEngine = engine;
    loadUsers();
  }

  async function handleCreateUserSubmit(params: {
    username: string;
    host: string;
    password: string;
    privilegeScope: "all" | "database" | "readonly";
    targetDb?: string;
  }) {
    if (!params.username.trim()) {
      onShowToast("Username cannot be empty");
      return;
    }
    isCreating = true;
    try {
      await createDatabaseUser(
        selectedEngine,
        params.username,
        params.host,
        params.password,
        params.privilegeScope,
        params.targetDb
      );
      onShowToast(`Database user '${params.username}' created successfully!`);
      activeView = "list";
      await loadUsers();
    } catch (err: any) {
      onShowToast(`Failed to create user: ${err?.message || err}`);
    } finally {
      isCreating = false;
    }
  }

  async function handleSavePassword(newPassword: string) {
    if (!userToChangePassword) return;
    isUpdatingPassword = true;
    try {
      await updateDatabaseUserPassword(
        selectedEngine,
        userToChangePassword.username,
        userToChangePassword.host,
        newPassword
      );
      onShowToast(`Password updated for '${userToChangePassword.username}'@'${userToChangePassword.host}'`);
      userToChangePassword = null;
    } catch (err: any) {
      onShowToast(`Failed to update password: ${err?.message || err}`);
    } finally {
      isUpdatingPassword = false;
    }
  }

  async function handleConfirmDelete() {
    if (!userPendingDelete) return;
    isDeleting = true;
    try {
      await dropDatabaseUser(
        selectedEngine,
        userPendingDelete.username,
        userPendingDelete.host
      );
      onShowToast(`User '${userPendingDelete.username}'@'${userPendingDelete.host}' dropped successfully!`);
      userPendingDelete = null;
      await loadUsers();
    } catch (err: any) {
      onShowToast(`Failed to drop user: ${err?.message || err}`);
    } finally {
      isDeleting = false;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 backdrop-blur-xs p-4">
    <div class="bg-white border border-slate-200 w-full max-w-2xl rounded-2xl shadow-2xl flex flex-col max-h-[90vh] overflow-hidden">
      <div class="px-6 py-4 border-b border-slate-100 flex items-center justify-between bg-slate-50/50">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-amber-50 text-[#94380C] border border-amber-200/80 flex items-center justify-center">
            <Users class="w-4 h-4" />
          </div>
          <div>
            <h2 class="font-bold text-sm text-slate-900">Database Users & Privileges</h2>
            <p class="text-[11px] text-slate-500">Manage database accounts, grants, hosts, and credentials</p>
          </div>
        </div>

        <button
          onclick={onClose}
          class="p-1 rounded-lg text-slate-400 hover:text-slate-600 hover:bg-slate-100 transition-colors"
          title="Close modal"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="px-6 pt-3 pb-2 border-b border-slate-100 flex items-center justify-between gap-3 bg-white">
        <div class="flex items-center gap-1.5 p-1 bg-slate-100 rounded-xl border border-slate-200/70">
          <button
            onclick={() => handleEngineSwitch("mariadb")}
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {selectedEngine === 'mariadb'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900'}"
          >
            <img src={mariaLogo} alt="MySQL" class="w-3.5 h-3.5 object-contain" />
            <span>MySQL / MariaDB</span>
          </button>

          <button
            onclick={() => handleEngineSwitch("postgresql")}
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {selectedEngine === 'postgresql'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900'}"
          >
            <img src={postgresLogo} alt="PostgreSQL" class="w-3.5 h-3.5 object-contain" />
            <span>PostgreSQL</span>
          </button>
        </div>

        <div class="flex items-center gap-1.5">
          <button
            onclick={() => (activeView = "list")}
            class="px-3 py-1.5 rounded-lg text-xs font-medium transition-colors {activeView === 'list'
              ? 'bg-slate-900 text-white shadow-xs'
              : 'bg-white border border-slate-200 text-slate-600 hover:bg-slate-50'}"
          >
            <span>Users</span>
            <span class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] {activeView === 'list' ? 'bg-slate-700 text-slate-200' : 'bg-slate-100 text-slate-600'}">
              {users.length}
            </span>
          </button>

          <button
            onclick={() => (activeView = "create")}
            class="flex items-center gap-1 px-3 py-1.5 rounded-lg text-xs font-medium transition-colors {activeView === 'create'
              ? 'bg-[#94380C] text-white shadow-xs'
              : 'bg-white border border-slate-200 text-slate-600 hover:bg-slate-50'}"
          >
            <Plus class="w-3.5 h-3.5" />
            <span>Add User</span>
          </button>

          <button
            onclick={loadUsers}
            class="p-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-600 transition-colors"
            title="Refresh Users"
          >
            <RefreshCw class="w-3.5 h-3.5 {isLoading ? 'animate-spin' : ''}" />
          </button>
        </div>
      </div>

      <div class="p-6 overflow-y-auto space-y-4 flex-1">
        {#if errorMessage}
          <div class="p-3 bg-rose-50 border border-rose-200 rounded-xl flex items-start gap-2 text-rose-800 text-xs">
            <AlertTriangle class="w-4 h-4 text-rose-600 shrink-0 mt-0.5" />
            <div>
              <div class="font-semibold">Engine Connection Notice</div>
              <div class="text-[11px] mt-0.5">{errorMessage}</div>
            </div>
          </div>
        {/if}

        {#if activeView === "list"}
          {#if isLoading && users.length === 0}
            <div class="py-12 text-center text-xs text-slate-400 flex items-center justify-center gap-2">
              <RefreshCw class="w-4 h-4 animate-spin text-[#94380C]" />
              <span>Querying database users...</span>
            </div>
          {:else if users.length === 0}
            <div class="py-12 text-center text-slate-400 space-y-2">
              <User class="w-8 h-8 mx-auto text-slate-300" />
              <div class="text-xs font-medium text-slate-600">No database users found</div>
              <p class="text-[11px] text-slate-400 max-w-xs mx-auto">
                Make sure the database service is running and credentials are valid.
              </p>
            </div>
          {:else}
            <div class="space-y-2.5">
              {#each users as u (u.username + u.host)}
                <div class="bg-white border border-slate-200/90 rounded-xl p-3 shadow-2xs hover:shadow-xs transition-shadow flex items-center justify-between gap-3">
                  <div class="flex items-center gap-3 min-w-0">
                    <div class="w-8 h-8 rounded-lg {u.is_system_account ? 'bg-slate-100 text-slate-700' : 'bg-amber-50 text-[#94380C]'} border border-slate-200 flex items-center justify-center font-bold text-xs uppercase shrink-0">
                      {u.username.charAt(0) || 'U'}
                    </div>

                    <div class="min-w-0">
                      <div class="flex items-center gap-1.5">
                        <span class="font-mono font-bold text-xs text-slate-900 truncate">{u.username}</span>
                        <span class="text-slate-400 text-xs font-mono">@{u.host}</span>
                        {#if u.is_system_account}
                          <span class="text-[10px] px-1.5 py-0.2 rounded bg-slate-100 text-slate-600 border border-slate-200 font-medium">
                            System Default
                          </span>
                        {/if}
                      </div>

                      <div class="flex items-center gap-1 mt-1 flex-wrap">
                        {#if u.privileges.length === 0}
                          <span class="text-[10px] text-slate-400 italic">No specific grants listed</span>
                        {:else}
                          {#each u.privileges.slice(0, 2) as priv}
                            <span class="text-[10px] px-1.5 py-0.2 rounded bg-emerald-50 text-emerald-800 border border-emerald-200 font-mono truncate max-w-[280px]" title={priv}>
                              {priv}
                            </span>
                          {/each}
                          {#if u.privileges.length > 2}
                            <span class="text-[10px] text-slate-400">+{u.privileges.length - 2} more</span>
                          {/if}
                        {/if}
                      </div>
                    </div>
                  </div>

                  <div class="flex items-center gap-1.5 shrink-0">
                    <button
                      onclick={() => (userToChangePassword = u)}
                      class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 text-xs font-medium shadow-2xs transition-colors"
                      title="Set New Password"
                    >
                      <KeyRound class="w-3.5 h-3.5 text-slate-500" />
                      <span>Password</span>
                    </button>

                    {#if !u.is_system_account}
                      <button
                        onclick={() => (userPendingDelete = u)}
                        class="p-1.5 rounded-lg border border-slate-200 bg-white hover:bg-rose-50 text-slate-400 hover:text-rose-600 transition-colors shadow-2xs"
                        title="Delete User"
                      >
                        <Trash2 class="w-3.5 h-3.5" />
                      </button>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        {:else}
          <CreateUserForm
            {eligibleDatabases}
            {isCreating}
            onCancel={() => (activeView = "list")}
            onSubmit={handleCreateUserSubmit}
          />
        {/if}
      </div>

      <div class="px-6 py-3 border-t border-slate-100 bg-slate-50/50 flex items-center justify-between text-[11px] text-slate-500">
        <span>Engine: <span class="font-semibold text-slate-700">{selectedEngine === 'mariadb' ? 'MySQL / MariaDB (Port 3306)' : 'PostgreSQL (Port 5432)'}</span></span>
        <span>Standard system admin account is protected</span>
      </div>
    </div>
  </div>
{/if}

<ChangePasswordModal
  user={userToChangePassword}
  isOpen={userToChangePassword !== null}
  isUpdating={isUpdatingPassword}
  onClose={() => (userToChangePassword = null)}
  onSave={handleSavePassword}
/>

{#if userPendingDelete}
  <div class="fixed inset-0 z-60 flex items-center justify-center bg-slate-900/50 backdrop-blur-xs p-4">
    <div class="bg-white border border-slate-200 w-full max-w-sm rounded-xl p-5 shadow-2xl space-y-4 text-slate-900">
      <div class="w-10 h-10 rounded-full bg-rose-50 text-rose-600 border border-rose-200 flex items-center justify-center mx-auto">
        <Trash2 class="w-5 h-5" />
      </div>

      <div class="text-center space-y-1">
        <h3 class="font-bold text-sm text-slate-900">Drop Database User?</h3>
        <p class="text-xs text-slate-500">
          Are you sure you want to drop user <span class="font-mono font-semibold text-slate-800">{userPendingDelete.username}@{userPendingDelete.host}</span>? All associated permissions will be revoked.
        </p>
      </div>

      <div class="flex items-center justify-center gap-2 pt-2">
        <button
          onclick={() => (userPendingDelete = null)}
          class="px-4 py-1.5 rounded-lg border border-slate-200 text-xs font-medium text-slate-600 hover:bg-slate-50 transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={handleConfirmDelete}
          disabled={isDeleting}
          class="px-4 py-1.5 rounded-lg bg-rose-600 hover:bg-rose-700 text-white text-xs font-semibold shadow-xs transition-colors disabled:opacity-50 flex items-center gap-1"
        >
          {#if isDeleting}
            <RefreshCw class="w-3 h-3 animate-spin" />
            <span>Dropping...</span>
          {:else}
            <span>Yes, Drop User</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
