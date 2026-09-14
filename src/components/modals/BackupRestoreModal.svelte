<script lang="ts">
  import {
    AlertTriangle,
    Archive,
    Check,
    Database,
    Download,
    ExternalLink,
    FileText,
    FolderOpen,
    HardDrive,
    RefreshCw,
    Upload,
    X,
  } from "@lucide/svelte";
  import type { UserDatabaseItem } from "../../types";
  import {
    exportDatabase,
    fetchBackupFiles,
    importDatabase,
    openBackupsFolder,
    type BackupFileItem,
  } from "../../lib/actions";

  let {
    databases = [],
    initialDb = null,
    isOpen = false,
    onClose,
    onShowToast,
    onRefreshDatabases,
  }: {
    databases: UserDatabaseItem[];
    initialDb: UserDatabaseItem | null;
    isOpen: boolean;
    onClose: () => void;
    onShowToast: (msg: string) => void;
    onRefreshDatabases: () => Promise<void>;
  } = $props();

  let activeTab = $state<"export" | "import">("export");
  let selectedDbName = $state<string>("");
  let includeData = $state<boolean>(true);
  let customOutputPath = $state<string>("");
  let isExporting = $state<boolean>(false);
  let lastExportedFile = $state<{ path: string; size: number } | null>(null);

  let importTargetDb = $state<string>("");
  let importFilePath = $state<string>("");
  let isImporting = $state<boolean>(false);
  let backupFiles = $state<BackupFileItem[]>([]);
  let isLoadingBackups = $state<boolean>(false);

  $effect(() => {
    if (isOpen) {
      if (initialDb) {
        selectedDbName = initialDb.name;
        importTargetDb = initialDb.name;
      } else if (databases.length > 0) {
        selectedDbName = databases[0].name;
        importTargetDb = databases[0].name;
      }
      loadBackups();
    }
  });

  async function loadBackups() {
    try {
      isLoadingBackups = true;
      backupFiles = await fetchBackupFiles();
    } catch {
      backupFiles = [];
    } finally {
      isLoadingBackups = false;
    }
  }

  let selectedDbItem = $derived(databases.find((d) => d.name === selectedDbName));
  let importTargetDbItem = $derived(databases.find((d) => d.name === importTargetDb));

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  }

  function formatTimestamp(ts: number): string {
    if (!ts) return "-";
    return new Date(ts * 1000).toLocaleString();
  }

  async function handleExport() {
    if (!selectedDbName) {
      onShowToast("Please select a database to export");
      return;
    }
    const engine = selectedDbItem?.engine || "mysql";
    try {
      isExporting = true;
      const res = await exportDatabase(
        engine,
        selectedDbName,
        customOutputPath.trim() || undefined,
        includeData
      );
      lastExportedFile = { path: res.file_path, size: res.size_bytes };
      onShowToast(res.message);
      await loadBackups();
    } catch (e: any) {
      onShowToast(`Export failed: ${e?.message || e}`);
    } finally {
      isExporting = false;
    }
  }

  async function handleImport() {
    if (!importTargetDb) {
      onShowToast("Please select a target database");
      return;
    }
    if (!importFilePath.trim()) {
      onShowToast("Please provide or pick a .sql backup file");
      return;
    }
    const engine = importTargetDbItem?.engine || "mysql";
    try {
      isImporting = true;
      const res = await importDatabase(engine, importTargetDb, importFilePath.trim());
      onShowToast(res.message);
      await onRefreshDatabases();
    } catch (e: any) {
      onShowToast(`Restore failed: ${e?.message || e}`);
    } finally {
      isImporting = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") onClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-xs animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="bg-white rounded-2xl max-w-2xl w-full border border-slate-200 shadow-2xl overflow-hidden flex flex-col max-h-[90vh]"
    >
      <div class="px-6 py-4 border-b border-slate-100 flex items-center justify-between bg-slate-50/50">
        <div class="flex items-center gap-3">
          <div class="w-9 h-9 rounded-xl bg-[#94380C]/10 border border-[#94380C]/20 flex items-center justify-center text-[#94380C]">
            <Archive class="w-5 h-5" />
          </div>
          <div>
            <h2 class="text-sm font-bold text-slate-900">Database Backup & Restore (.sql)</h2>
            <p class="text-xs text-slate-500 mt-0.5">
              Export full dumps or restore SQL statements into any local database.
            </p>
          </div>
        </div>

        <button
          onclick={onClose}
          class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition-colors"
          title="Close dialog"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="p-6 overflow-y-auto space-y-5">
        <div class="flex items-center gap-1.5 p-1 bg-slate-100 rounded-xl border border-slate-200/80">
          <button
            onclick={() => (activeTab = "export")}
            class="flex-1 flex items-center justify-center gap-2 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'export'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900'}"
          >
            <Download class="w-3.5 h-3.5 text-emerald-600" />
            <span>Export / Backup (.sql)</span>
          </button>

          <button
            onclick={() => (activeTab = "import")}
            class="flex-1 flex items-center justify-center gap-2 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'import'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900'}"
          >
            <Upload class="w-3.5 h-3.5 text-blue-600" />
            <span>Import / Restore (.sql)</span>
          </button>
        </div>

        {#if activeTab === "export"}
          <div class="space-y-4">
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
              <div>
                <label for="export-db-select" class="block text-[11px] font-medium text-slate-600 mb-1">
                  Source Database
                </label>
                <select
                  id="export-db-select"
                  bind:value={selectedDbName}
                  class="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
                >
                  {#each databases as db (db.name)}
                    <option value={db.name}>{db.name} ({db.engine})</option>
                  {/each}
                </select>
              </div>

              <div>
                <span class="block text-[11px] font-medium text-slate-600 mb-1">Export Mode</span>
                <div class="grid grid-cols-2 gap-1.5 p-1 bg-slate-100 rounded-lg border border-slate-200/70">
                  <button
                    type="button"
                    onclick={() => (includeData = true)}
                    class="px-2 py-1 text-[11px] font-medium rounded {includeData ? 'bg-white text-slate-900 shadow-2xs font-semibold' : 'text-slate-600'}"
                  >
                    Full (Data & Schema)
                  </button>
                  <button
                    type="button"
                    onclick={() => (includeData = false)}
                    class="px-2 py-1 text-[11px] font-medium rounded {!includeData ? 'bg-white text-slate-900 shadow-2xs font-semibold' : 'text-slate-600'}"
                  >
                    Structure Only
                  </button>
                </div>
              </div>
            </div>

            <div>
              <label for="export-custom-path" class="block text-[11px] font-medium text-slate-600 mb-1">
                Output Path <span class="text-slate-400 font-normal">(Leave empty to save in C:\4forge\backups)</span>
              </label>
              <input
                id="export-custom-path"
                type="text"
                bind:value={customOutputPath}
                placeholder={`C:\\4forge\\backups\\${selectedDbName || 'database'}.sql`}
                class="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 text-xs font-mono focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
              />
            </div>

            <div class="pt-2 flex items-center justify-between">
              <button
                type="button"
                onclick={openBackupsFolder}
                class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 text-xs font-medium shadow-2xs transition-colors"
              >
                <FolderOpen class="w-3.5 h-3.5 text-amber-600" />
                <span>Open Backups Folder</span>
              </button>

              <button
                type="button"
                onclick={handleExport}
                disabled={isExporting || !selectedDbName}
                class="flex items-center gap-1.5 px-4 py-2 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] disabled:bg-slate-300 text-white text-xs font-semibold shadow-xs transition-colors"
              >
                {#if isExporting}
                  <RefreshCw class="w-3.5 h-3.5 animate-spin" />
                  <span>Dumping Database...</span>
                {:else}
                  <Download class="w-3.5 h-3.5" />
                  <span>Start Backup</span>
                {/if}
              </button>
            </div>

            {#if lastExportedFile}
              <div class="bg-emerald-50/70 border border-emerald-200 rounded-xl p-3 flex items-start gap-2.5 text-xs text-emerald-800">
                <Check class="w-4 h-4 text-emerald-600 mt-0.5 shrink-0" />
                <div class="min-w-0 flex-1">
                  <div class="font-bold">Backup Completed Successfully</div>
                  <div class="font-mono text-[11px] text-emerald-900 truncate mt-0.5" title={lastExportedFile.path}>
                    {lastExportedFile.path}
                  </div>
                  <div class="text-[10.5px] text-emerald-700 mt-0.5">
                    File size: {formatBytes(lastExportedFile.size)}
                  </div>
                </div>
              </div>
            {/if}
          </div>
        {:else}
          <div class="space-y-4">
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
              <div>
                <label for="import-db-select" class="block text-[11px] font-medium text-slate-600 mb-1">
                  Target Database (Restore Destination)
                </label>
                <select
                  id="import-db-select"
                  bind:value={importTargetDb}
                  class="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
                >
                  {#each databases as db (db.name)}
                    <option value={db.name}>{db.name} ({db.engine})</option>
                  {/each}
                </select>
              </div>

              <div>
                <label for="import-file-input" class="block text-[11px] font-medium text-slate-600 mb-1">
                  SQL Backup File Path
                </label>
                <input
                  id="import-file-input"
                  type="text"
                  bind:value={importFilePath}
                  placeholder="C:\path\to\backup.sql"
                  class="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 text-xs font-mono focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
                />
              </div>
            </div>

            <div class="space-y-2">
              <div class="flex items-center justify-between text-xs text-slate-600 font-medium">
                <span>Recent Backups in C:\4forge\backups</span>
                <button
                  type="button"
                  onclick={loadBackups}
                  class="text-[11px] text-slate-500 hover:text-slate-800 flex items-center gap-1"
                >
                  <RefreshCw class="w-3 h-3 {isLoadingBackups ? 'animate-spin' : ''}" />
                  <span>Refresh</span>
                </button>
              </div>

              {#if backupFiles.length === 0}
                <div class="bg-slate-50 rounded-xl p-4 text-center border border-slate-200/80 text-xs text-slate-500">
                  No previous backup files found in <code class="font-mono">C:\4forge\backups</code>. You can enter an absolute file path above.
                </div>
              {:else}
                <div class="max-h-36 overflow-y-auto border border-slate-200 rounded-xl divide-y divide-slate-100 bg-slate-50/50">
                  {#each backupFiles as file (file.file_path)}
                    <button
                      type="button"
                      onclick={() => (importFilePath = file.file_path)}
                      class="w-full p-2 text-left flex items-center justify-between gap-3 text-xs hover:bg-amber-50/60 transition-colors {importFilePath === file.file_path ? 'bg-amber-50 border-l-2 border-[#94380C]' : ''}"
                    >
                      <div class="min-w-0">
                        <div class="font-mono font-medium text-slate-800 truncate" title={file.file_name}>
                          {file.file_name}
                        </div>
                        <div class="text-[10px] text-slate-400">
                          {formatTimestamp(file.modified_timestamp)}
                        </div>
                      </div>
                      <span class="text-[11px] font-mono font-semibold text-slate-500 shrink-0">
                        {formatBytes(file.size_bytes)}
                      </span>
                    </button>
                  {/each}
                </div>
              {/if}
            </div>

            <div class="bg-amber-50/80 border border-amber-200/80 rounded-xl p-3 flex items-start gap-2.5 text-xs text-amber-900">
              <AlertTriangle class="w-4 h-4 text-amber-600 mt-0.5 shrink-0" />
              <div>
                <div class="font-bold">Restore Precaution</div>
                <div class="text-[11px] text-amber-800 mt-0.5">
                  Restoring will execute SQL statements into database <span class="font-mono font-bold text-amber-950">{importTargetDb}</span>. Conflicting existing tables may be overwritten.
                </div>
              </div>
            </div>

            <div class="pt-2 flex items-center justify-between">
              <button
                type="button"
                onclick={openBackupsFolder}
                class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 text-xs font-medium shadow-2xs transition-colors"
              >
                <FolderOpen class="w-3.5 h-3.5 text-amber-600" />
                <span>Open Backups Folder</span>
              </button>

              <button
                type="button"
                onclick={handleImport}
                disabled={isImporting || !importTargetDb || !importFilePath.trim()}
                class="flex items-center gap-1.5 px-4 py-2 rounded-lg bg-slate-900 hover:bg-slate-800 disabled:bg-slate-300 text-white text-xs font-semibold shadow-xs transition-colors"
              >
                {#if isImporting}
                  <RefreshCw class="w-3.5 h-3.5 animate-spin" />
                  <span>Restoring Database...</span>
                {:else}
                  <Upload class="w-3.5 h-3.5" />
                  <span>Execute Restore</span>
                {/if}
              </button>
            </div>
          </div>
        {/if}
      </div>

      <div class="px-6 py-3.5 bg-slate-50 border-t border-slate-100 flex items-center justify-between text-xs text-slate-500">
        <span>Backups directory: <code class="px-1 py-0.5 rounded bg-slate-200 text-slate-700 font-mono">C:\4forge\backups</code></span>
        <button
          onclick={onClose}
          class="px-3.5 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-100 text-slate-700 font-medium transition-colors"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
