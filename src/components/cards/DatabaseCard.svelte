<script lang="ts">
  import { Check, Copy, ExternalLink, FileText, Globe, KeyRound, Trash2 } from "@lucide/svelte";
  import type { UserDatabaseItem } from "../../types";
  import mariaLogo from "../../assets/logos/mysql-logo.svg";
  import postgresLogo from "../../assets/logos/postgresql-logo.svg";
  import mongoLogo from "../../assets/logos/mongo-logo.svg";

  let {
    db,
    copied = false,
    onCopyEnv,
    onOpenConnect,
    onOpenAdminer,
    onLaunchNative,
    onRequestDelete,
  }: {
    db: UserDatabaseItem;
    copied?: boolean;
    onCopyEnv: (db: UserDatabaseItem) => void;
    onOpenConnect: (db: UserDatabaseItem) => void;
    onOpenAdminer: (engine: string, name: string) => void;
    onLaunchNative: (engine: string) => void;
    onRequestDelete: (db: UserDatabaseItem) => void;
  } = $props();

  function getEngineDetails(engine: string) {
    const eng = engine.toLowerCase();
    if (eng.includes("postgres")) {
      return {
        label: "PostgreSQL",
        logo: postgresLogo,
        port: 5432,
        user: "postgres",
        badgeBg: "bg-blue-50 text-blue-700 border-blue-200",
      };
    }
    if (eng.includes("mongo")) {
      return {
        label: "MongoDB",
        logo: mongoLogo,
        port: 27017,
        user: "admin",
        badgeBg: "bg-emerald-50 text-emerald-700 border-emerald-200",
      };
    }
    if (eng.includes("sqlite")) {
      return {
        label: "SQLite",
        logo: null,
        port: null,
        user: "file",
        badgeBg: "bg-indigo-50 text-indigo-700 border-indigo-200",
      };
    }
    return {
      label: "MySQL / MariaDB",
      logo: mariaLogo,
      port: 3306,
      user: "root",
      badgeBg: "bg-amber-50 text-amber-800 border-amber-200",
    };
  }

  let details = $derived(getEngineDetails(db.engine));
</script>

<div class="bg-white border border-slate-200/90 rounded-xl p-4 shadow-xs hover:shadow-md transition-shadow flex flex-col justify-between group">
  <div class="space-y-3">
    <div class="flex items-start justify-between gap-2">
      <div class="flex items-center gap-2.5 min-w-0">
        <div class="w-8 h-8 rounded-lg bg-slate-50 border border-slate-200 flex items-center justify-center p-1 shrink-0">
          {#if details.logo}
            <img src={details.logo} alt={details.label} class="w-5 h-5 object-contain" />
          {:else}
            <FileText class="w-4 h-4 text-indigo-600" />
          {/if}
        </div>
        <div class="min-w-0">
          <h3 class="font-bold text-sm text-slate-900 truncate font-mono" title={db.name}>
            {db.name}
          </h3>
          <div class="flex items-center gap-1.5 mt-0.5">
            <span class="text-[10px] font-medium px-1.5 py-0.2 rounded border {details.badgeBg}">
              {details.label}
            </span>
            <span class="inline-flex items-center gap-1 text-[10px] {db.status === 'running' ? 'text-emerald-700' : 'text-slate-500'}">
              <span class="w-1.5 h-1.5 rounded-full {db.status === 'running' ? 'bg-emerald-500' : 'bg-slate-300'}"></span>
              <span>{db.status === 'running' ? 'Ready' : 'Stopped'}</span>
            </span>
          </div>
        </div>
      </div>

      <button
        onclick={() => onRequestDelete(db)}
        class="opacity-0 group-hover:opacity-100 p-1 rounded-md text-slate-400 hover:text-rose-600 hover:bg-rose-50 transition-all"
        title="Delete Database"
      >
        <Trash2 class="w-3.5 h-3.5" />
      </button>
    </div>

    <div class="bg-slate-50 rounded-lg p-2.5 border border-slate-100 text-xs space-y-1 font-mono text-slate-600">
      <div class="flex justify-between items-center text-[11px]">
        <span class="text-slate-400 font-sans">Host:</span>
        <span class="text-slate-800 truncate max-w-[180px]" title={db.host}>{db.host}</span>
      </div>
      {#if details.port}
        <div class="flex justify-between items-center text-[11px]">
          <span class="text-slate-400 font-sans">Port:</span>
          <span class="text-slate-800">{details.port}</span>
        </div>
      {/if}
      <div class="flex justify-between items-center text-[11px]">
        <span class="text-slate-400 font-sans">User:</span>
        <span class="text-slate-800">{db.user || details.user}</span>
      </div>
    </div>
  </div>

  <div class="pt-3 mt-3 border-t border-slate-100 flex items-center justify-between gap-1.5">
    <div class="flex items-center gap-1.5">
      <button
        onclick={() => onOpenAdminer(db.engine, db.name)}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 text-xs font-medium transition-colors shadow-2xs"
        title="Open in Adminer Web UI"
      >
        <Globe class="w-3.5 h-3.5 text-slate-500" />
        <span>Adminer</span>
      </button>

      <button
        onclick={() => onLaunchNative(db.engine)}
        class="flex items-center gap-1 px-2 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 text-xs font-medium transition-colors shadow-2xs"
        title="Launch Desktop Database App"
      >
        <ExternalLink class="w-3.5 h-3.5 text-slate-500" />
      </button>
    </div>

    <div class="flex items-center gap-1">
      <button
        onclick={() => onCopyEnv(db)}
        class="p-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-500 hover:text-slate-800 transition-colors shadow-2xs"
        title="Quick Copy Default .env"
      >
        {#if copied}
          <Check class="w-3.5 h-3.5 text-emerald-600" />
        {:else}
          <Copy class="w-3.5 h-3.5" />
        {/if}
      </button>

      <button
        onclick={() => onOpenConnect(db)}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 text-xs font-medium transition-colors shadow-2xs hover:border-[#94380C]/40 hover:text-[#94380C]"
        title="Open Connection & .env Helper"
      >
        <KeyRound class="w-3.5 h-3.5 text-[#94380C]" />
        <span>.env</span>
      </button>
    </div>
  </div>
</div>
