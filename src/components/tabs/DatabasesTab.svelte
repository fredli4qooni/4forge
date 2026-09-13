<script lang="ts">
  import {
    Database,
    FileText,
    Plus,
    Power,
    RefreshCw,
    Search,
    Trash2,
    X,
  } from "@lucide/svelte";
  import type { ServiceItem, UserDatabaseItem } from "../../types";
  import mariaLogo from "../../assets/logos/mysql-logo.svg";
  import postgresLogo from "../../assets/logos/postgresql-logo.svg";
  import mongoLogo from "../../assets/logos/mongo-logo.svg";
  import DatabaseCard from "../cards/DatabaseCard.svelte";
  import CreateDatabaseModal from "../modals/CreateDatabaseModal.svelte";

  let {
    databases = [],
    services = [],
    onCreateDatabase,
    onDeleteDatabase,
    onOpenAdminer,
    onLaunchNative,
    onToggleService,
    onRefresh,
    onShowToast,
  }: {
    databases: UserDatabaseItem[];
    services: ServiceItem[];
    onCreateDatabase: (name: string, engine: string) => Promise<void>;
    onDeleteDatabase: (engine: string, name: string) => Promise<void>;
    onOpenAdminer: (engine: string, dbName: string) => Promise<void>;
    onLaunchNative: (engine: string) => Promise<void>;
    onToggleService: (serviceId: string) => Promise<void>;
    onRefresh: () => Promise<void>;
    onShowToast: (msg: string) => void;
  } = $props();

  let selectedFilter = $state<string>("all");
  let searchQuery = $state<string>("");
  let showCreateModal = $state(false);
  let copiedDbName = $state<string | null>(null);
  let dbPendingDelete = $state<UserDatabaseItem | null>(null);

  let filteredDatabases = $derived(
    databases.filter((db) => {
      const matchFilter =
        selectedFilter === "all"
          ? true
          : selectedFilter === "mysql"
          ? db.engine === "mariadb" || db.engine === "mysql"
          : selectedFilter === "postgres"
          ? db.engine === "postgresql" || db.engine === "postgres"
          : selectedFilter === "sqlite"
          ? db.engine === "sqlite" || db.engine === "sqlite3"
          : selectedFilter === "mongodb"
          ? db.engine === "mongodb" || db.engine === "mongo"
          : db.engine === selectedFilter;

      const matchSearch =
        !searchQuery.trim() ||
        db.name.toLowerCase().includes(searchQuery.toLowerCase().trim()) ||
        db.engine.toLowerCase().includes(searchQuery.toLowerCase().trim());

      return matchFilter && matchSearch;
    })
  );

  let mariaCount = $derived(databases.filter((d) => d.engine === "mariadb" || d.engine === "mysql").length);
  let pgCount = $derived(databases.filter((d) => d.engine === "postgresql" || d.engine === "postgres").length);
  let sqliteCount = $derived(databases.filter((d) => d.engine === "sqlite" || d.engine === "sqlite3").length);
  let mongoCount = $derived(databases.filter((d) => d.engine === "mongodb" || d.engine === "mongo").length);

  let mariaSvc = $derived(services.find((s) => s.id === "mariadb"));
  let pgSvc = $derived(services.find((s) => s.id === "postgresql"));
  let mongoSvc = $derived(services.find((s) => s.id === "mongodb"));

  function copyEnvSnippet(db: UserDatabaseItem) {
    const eng = db.engine.toLowerCase();
    let snippet = "";
    if (eng.includes("postgres")) {
      snippet = `DB_CONNECTION=pgsql\nDB_HOST=127.0.0.1\nDB_PORT=5432\nDB_DATABASE=${db.name}\nDB_USERNAME=postgres\nDB_PASSWORD=`;
    } else if (eng.includes("sqlite")) {
      snippet = `DB_CONNECTION=sqlite\nDB_DATABASE=${db.host || `C:\\4forge\\data\\sqlite\\${db.name}.sqlite`}`;
    } else if (eng.includes("mongo")) {
      snippet = `MONGO_URI=mongodb://127.0.0.1:27017/${db.name}\nMONGO_DB=${db.name}`;
    } else {
      snippet = `DB_CONNECTION=mysql\nDB_HOST=127.0.0.1\nDB_PORT=3306\nDB_DATABASE=${db.name}\nDB_USERNAME=root\nDB_PASSWORD=`;
    }

    navigator.clipboard.writeText(snippet);
    copiedDbName = db.name;
    onShowToast(`Copied .env configuration for '${db.name}'`);
    setTimeout(() => {
      if (copiedDbName === db.name) copiedDbName = null;
    }, 2000);
  }

  async function confirmDelete() {
    if (!dbPendingDelete) return;
    const { engine, name } = dbPendingDelete;
    dbPendingDelete = null;
    await onDeleteDatabase(engine, name);
  }
</script>

<div class="space-y-6">
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
    <div>
      <h1 class="text-xl font-bold tracking-tight text-slate-900 flex items-center gap-2">
        <span>Databases</span>
        <span class="text-xs font-mono font-semibold px-2 py-0.5 rounded-full bg-slate-100 text-slate-700 border border-slate-200">
          {databases.length}
        </span>
      </h1>
      <p class="text-xs text-slate-500 mt-0.5">
        Manage relational, document, and local SQLite databases with instant connections & Adminer.
      </p>
    </div>

    <div class="flex items-center gap-2">
      <button
        onclick={onRefresh}
        class="p-2 rounded-lg border border-slate-200 bg-white text-slate-600 hover:text-slate-900 hover:bg-slate-50 shadow-xs transition-colors"
        title="Refresh Databases"
      >
        <RefreshCw class="w-4 h-4" />
      </button>

      <button
        onclick={() => (showCreateModal = true)}
        class="flex items-center gap-1.5 px-3.5 py-2 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs transition-colors"
      >
        <Plus class="w-3.5 h-3.5" />
        <span>Create Database</span>
      </button>
    </div>
  </div>

  <div class="bg-white rounded-xl border border-slate-200/80 p-3 shadow-xs flex flex-wrap items-center justify-between gap-3 text-xs">
    <div class="flex items-center gap-2 text-slate-500 font-medium">
      <Database class="w-4 h-4 text-slate-400" />
      <span>Database Services:</span>
    </div>

    <div class="flex items-center gap-2.5 flex-wrap">
      <div class="flex items-center gap-2 px-2.5 py-1 rounded-lg border border-slate-200/70 bg-slate-50/70">
        <span class="w-2 h-2 rounded-full {mariaSvc?.status === 'running' ? 'bg-emerald-500' : 'bg-slate-300'}"></span>
        <span class="font-medium text-slate-700">MariaDB:3306</span>
        <button
          onclick={() => onToggleService("mariadb")}
          class="p-1 rounded text-slate-500 hover:text-slate-900 hover:bg-white transition-colors"
          title="{mariaSvc?.status === 'running' ? 'Stop' : 'Start'} MariaDB"
        >
          <Power class="w-3 h-3 {mariaSvc?.status === 'running' ? 'text-rose-500' : 'text-slate-400'}" />
        </button>
      </div>

      <div class="flex items-center gap-2 px-2.5 py-1 rounded-lg border border-slate-200/70 bg-slate-50/70">
        <span class="w-2 h-2 rounded-full {pgSvc?.status === 'running' ? 'bg-emerald-500' : 'bg-slate-300'}"></span>
        <span class="font-medium text-slate-700">PostgreSQL:5432</span>
        <button
          onclick={() => onToggleService("postgresql")}
          class="p-1 rounded text-slate-500 hover:text-slate-900 hover:bg-white transition-colors"
          title="{pgSvc?.status === 'running' ? 'Stop' : 'Start'} PostgreSQL"
        >
          <Power class="w-3 h-3 {pgSvc?.status === 'running' ? 'text-rose-500' : 'text-slate-400'}" />
        </button>
      </div>

      <div class="flex items-center gap-2 px-2.5 py-1 rounded-lg border border-slate-200/70 bg-slate-50/70">
        <span class="w-2 h-2 rounded-full {mongoSvc?.status === 'running' ? 'bg-emerald-500' : 'bg-slate-300'}"></span>
        <span class="font-medium text-slate-700">MongoDB:27017</span>
        <button
          onclick={() => onToggleService("mongodb")}
          class="p-1 rounded text-slate-500 hover:text-slate-900 hover:bg-white transition-colors"
          title="{mongoSvc?.status === 'running' ? 'Stop' : 'Start'} MongoDB"
        >
          <Power class="w-3 h-3 {mongoSvc?.status === 'running' ? 'text-rose-500' : 'text-slate-400'}" />
        </button>
      </div>

      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg border border-slate-200/70 bg-slate-50/70">
        <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
        <span class="font-medium text-slate-700">SQLite (Embedded)</span>
      </div>
    </div>
  </div>

  <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3">
    <div class="flex items-center gap-1 bg-slate-100 p-1 rounded-xl border border-slate-200/80 overflow-x-auto">
      <button
        onclick={() => (selectedFilter = "all")}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {selectedFilter === 'all'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900 hover:bg-white/60'}"
      >
        <span>All</span>
        <span class="text-[10px] font-mono px-1.5 py-0.2 rounded-full {selectedFilter === 'all' ? 'bg-amber-100 text-amber-900' : 'bg-slate-200 text-slate-600'}">
          {databases.length}
        </span>
      </button>

      <button
        onclick={() => (selectedFilter = "mysql")}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {selectedFilter === 'mysql'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900 hover:bg-white/60'}"
      >
        <img src={mariaLogo} alt="MySQL" class="w-3.5 h-3.5 object-contain" />
        <span>MySQL / MariaDB</span>
        <span class="text-[10px] font-mono px-1.5 py-0.2 rounded-full {selectedFilter === 'mysql' ? 'bg-amber-100 text-amber-900' : 'bg-slate-200 text-slate-600'}">
          {mariaCount}
        </span>
      </button>

      <button
        onclick={() => (selectedFilter = "postgres")}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {selectedFilter === 'postgres'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900 hover:bg-white/60'}"
      >
        <img src={postgresLogo} alt="PostgreSQL" class="w-3.5 h-3.5 object-contain" />
        <span>PostgreSQL</span>
        <span class="text-[10px] font-mono px-1.5 py-0.2 rounded-full {selectedFilter === 'postgres' ? 'bg-blue-100 text-blue-900' : 'bg-slate-200 text-slate-600'}">
          {pgCount}
        </span>
      </button>

      <button
        onclick={() => (selectedFilter = "sqlite")}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {selectedFilter === 'sqlite'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900 hover:bg-white/60'}"
      >
        <FileText class="w-3.5 h-3.5 text-indigo-500" />
        <span>SQLite</span>
        <span class="text-[10px] font-mono px-1.5 py-0.2 rounded-full {selectedFilter === 'sqlite' ? 'bg-indigo-100 text-indigo-900' : 'bg-slate-200 text-slate-600'}">
          {sqliteCount}
        </span>
      </button>

      <button
        onclick={() => (selectedFilter = "mongodb")}
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {selectedFilter === 'mongodb'
          ? 'bg-white text-slate-900 shadow-xs font-semibold'
          : 'text-slate-600 hover:text-slate-900 hover:bg-white/60'}"
      >
        <img src={mongoLogo} alt="MongoDB" class="w-3.5 h-3.5 object-contain" />
        <span>MongoDB</span>
        <span class="text-[10px] font-mono px-1.5 py-0.2 rounded-full {selectedFilter === 'mongodb' ? 'bg-emerald-100 text-emerald-900' : 'bg-slate-200 text-slate-600'}">
          {mongoCount}
        </span>
      </button>
    </div>

    <div class="relative min-w-[220px]">
      <Search class="w-3.5 h-3.5 absolute left-3 top-1/2 -translate-y-1/2 text-slate-400" />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Filter databases..."
        class="w-full pl-8 pr-8 py-1.5 rounded-lg bg-white border border-slate-200 text-xs text-slate-900 focus:outline-none focus:border-slate-900 shadow-xs"
      />
      {#if searchQuery}
        <button
          onclick={() => (searchQuery = "")}
          class="absolute right-2.5 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 p-0.5"
        >
          <X class="w-3 h-3" />
        </button>
      {/if}
    </div>
  </div>

  {#if filteredDatabases.length === 0}
    <div class="bg-white border border-slate-200 rounded-xl p-12 text-center shadow-xs">
      <div class="w-12 h-12 rounded-xl bg-amber-50 text-[#94380C] border border-amber-200/80 mx-auto flex items-center justify-center mb-3">
        <Database class="w-6 h-6" />
      </div>
      <h3 class="text-sm font-semibold text-slate-900">No databases found</h3>
      <p class="text-xs text-slate-500 max-w-sm mx-auto mt-1 mb-4">
        {searchQuery ? `No database names matching "${searchQuery}".` : `No databases configured under the "${selectedFilter}" filter yet.`}
      </p>
      <button
        onclick={() => (showCreateModal = true)}
        class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs transition-colors"
      >
        <Plus class="w-3.5 h-3.5" />
        <span>Create Database Now</span>
      </button>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
      {#each filteredDatabases as db (db.name + db.engine)}
        <DatabaseCard
          {db}
          copied={copiedDbName === db.name}
          onCopyEnv={copyEnvSnippet}
          onOpenAdminer={onOpenAdminer}
          onLaunchNative={onLaunchNative}
          onRequestDelete={(item) => (dbPendingDelete = item)}
        />
      {/each}
    </div>
  {/if}
</div>

<CreateDatabaseModal
  show={showCreateModal}
  onClose={() => (showCreateModal = false)}
  onCreate={onCreateDatabase}
  {onShowToast}
/>

{#if dbPendingDelete}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 backdrop-blur-xs p-4">
    <div class="bg-white border border-slate-200 w-full max-w-sm rounded-xl p-6 space-y-4 shadow-xl text-slate-900">
      <div class="w-10 h-10 rounded-full bg-rose-50 text-rose-600 border border-rose-200 flex items-center justify-center mx-auto">
        <Trash2 class="w-5 h-5" />
      </div>

      <div class="text-center space-y-1">
        <h3 class="font-bold text-sm text-slate-900">Delete Database?</h3>
        <p class="text-xs text-slate-500">
          Are you sure you want to drop or remove database <span class="font-mono font-semibold text-slate-800">{dbPendingDelete.name}</span> ({dbPendingDelete.engine})? This action cannot be undone.
        </p>
      </div>

      <div class="flex items-center justify-center gap-2 pt-2">
        <button
          onclick={() => (dbPendingDelete = null)}
          class="px-4 py-1.5 rounded-lg border border-slate-200 text-xs font-medium text-slate-600 hover:bg-slate-50 transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={confirmDelete}
          class="px-4 py-1.5 rounded-lg bg-rose-600 hover:bg-rose-700 text-white text-xs font-semibold shadow-xs transition-colors"
        >
          Yes, Delete
        </button>
      </div>
    </div>
  </div>
{/if}
