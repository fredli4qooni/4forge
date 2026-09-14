<script lang="ts">
  import {
    Check,
    Code,
    Copy,
    Database,
    Eye,
    EyeOff,
    FileCode,
    KeyRound,
    Terminal,
    X,
  } from "@lucide/svelte";
  import type { UserDatabaseItem } from "../../types";

  let {
    database = null,
    isOpen = false,
    onClose,
    onShowToast,
  }: {
    database: UserDatabaseItem | null;
    isOpen: boolean;
    onClose: () => void;
    onShowToast: (msg: string) => void;
  } = $props();

  type FrameworkKey = "laravel" | "prisma" | "django" | "uri";

  let activeTab = $state<FrameworkKey>("laravel");
  let showPassword = $state<boolean>(false);
  let copied = $state<boolean>(false);

  let customHost = $state<string>("127.0.0.1");
  let customPort = $state<string>("");
  let customUser = $state<string>("");
  let customPassword = $state<string>("");
  let customDbName = $state<string>("");

  $effect(() => {
    if (database) {
      customDbName = database.name;
      const eng = database.engine.toLowerCase();
      if (eng.includes("postgres")) {
        customPort = "5432";
        customUser = "postgres";
      } else if (eng.includes("mongo")) {
        customPort = "27017";
        customUser = "";
      } else if (eng.includes("sqlite")) {
        customPort = "";
        customUser = "";
      } else {
        customPort = "3306";
        customUser = "root";
      }
      customHost = "127.0.0.1";
      customPassword = "";
      copied = false;
    } else {
      customDbName = "app_db";
      customPort = "3306";
      customUser = "root";
      customHost = "127.0.0.1";
      customPassword = "";
      copied = false;
    }
  });

  let engine = $derived((database?.engine || "mysql").toLowerCase());
  let isPostgres = $derived(engine.includes("postgres"));
  let isSqlite = $derived(engine.includes("sqlite"));
  let isMongo = $derived(engine.includes("mongo"));

  let snippet = $derived.by(() => {
    const host = customHost.trim() || "127.0.0.1";
    const port = customPort.trim() || (isPostgres ? "5432" : isMongo ? "27017" : "3306");
    const user = customUser.trim();
    const pass = customPassword;
    const db = customDbName.trim() || "app_db";

    if (activeTab === "laravel") {
      if (isSqlite) {
        return `DB_CONNECTION=sqlite\nDB_DATABASE=${database?.host || `C:/4forge/data/sqlite/${db}.sqlite`}`;
      }
      if (isMongo) {
        const auth = user ? `${user}:${pass}@` : "";
        return `DB_CONNECTION=mongodb\nMONGO_URI=mongodb://${auth}${host}:${port}/${db}\nMONGO_DB=${db}`;
      }
      const conn = isPostgres ? "pgsql" : "mysql";
      const u = user || (isPostgres ? "postgres" : "root");
      return `DB_CONNECTION=${conn}\nDB_HOST=${host}\nDB_PORT=${port}\nDB_DATABASE=${db}\nDB_USERNAME=${u}\nDB_PASSWORD=${pass}`;
    }

    if (activeTab === "prisma") {
      if (isSqlite) {
        return `DATABASE_URL="file:./${db}.sqlite"`;
      }
      if (isMongo) {
        const auth = user ? `${user}:${pass}@` : "";
        return `DATABASE_URL="mongodb://${auth}${host}:${port}/${db}?authSource=admin"`;
      }
      const proto = isPostgres ? "postgresql" : "mysql";
      const u = user || (isPostgres ? "postgres" : "root");
      const auth = pass ? `${u}:${pass}` : `${u}:`;
      const schema = isPostgres ? "?schema=public" : "";
      return `DATABASE_URL="${proto}://${auth}@${host}:${port}/${db}${schema}"`;
    }

    if (activeTab === "django") {
      if (isSqlite) {
        return `# In settings.py\nDATABASES = {\n    'default': {\n        'ENGINE': 'django.db.backends.sqlite3',\n        'NAME': BASE_DIR / '${db}.sqlite',\n    }\n}`;
      }
      if (isMongo) {
        const auth = user ? `${user}:${pass}@` : "";
        return `# djongo or pymongo\nDATABASES = {\n    'default': {\n        'ENGINE': 'djongo',\n        'NAME': '${db}',\n        'CLIENT': {\n            'host': 'mongodb://${auth}${host}:${port}/${db}',\n        }\n    }\n}`;
      }
      const engName = isPostgres ? "django.db.backends.postgresql" : "django.db.backends.mysql";
      const u = user || (isPostgres ? "postgres" : "root");
      return `# In settings.py or .env (django-environ)\nDATABASE_URL="${isPostgres ? 'psql' : 'mysql'}://${u}:${pass}@${host}:${port}/${db}"\n\n# Or settings.py dict:\nDATABASES = {\n    'default': {\n        'ENGINE': '${engName}',\n        'NAME': '${db}',\n        'USER': '${u}',\n        'PASSWORD': '${pass}',\n        'HOST': '${host}',\n        'PORT': '${port}',\n    }\n}`;
    }

    if (isSqlite) {
      return `sqlite:///${database?.host || `C:/4forge/data/sqlite/${db}.sqlite`}`;
    }
    if (isMongo) {
      const auth = user ? `${user}:${pass}@` : "";
      return `mongodb://${auth}${host}:${port}/${db}`;
    }
    const proto = isPostgres ? "postgresql" : "mysql";
    const u = user || (isPostgres ? "postgres" : "root");
    const auth = pass ? `${u}:${pass}` : `${u}`;
    return `${proto}://${auth}@${host}:${port}/${db}`;
  });

  function copySnippet() {
    navigator.clipboard.writeText(snippet);
    copied = true;
    onShowToast(`Copied ${activeTab.toUpperCase()} connection snippet`);
    setTimeout(() => {
      copied = false;
    }, 2000);
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
            <KeyRound class="w-5 h-5" />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h2 class="text-sm font-bold text-slate-900">Database Connection & .env</h2>
              <span class="text-[10px] font-semibold px-2 py-0.5 rounded-full bg-slate-100 text-slate-700 border border-slate-200 uppercase font-mono">
                {database?.engine || "Generic"}
              </span>
            </div>
            <p class="text-xs text-slate-500 mt-0.5">
              Copy framework-ready environment variables or adjust credentials.
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
        <div class="flex items-center gap-1.5 p-1 bg-slate-100 rounded-xl border border-slate-200/80 overflow-x-auto">
          <button
            onclick={() => (activeTab = "laravel")}
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'laravel'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900 hover:bg-white/50'}"
          >
            <FileCode class="w-3.5 h-3.5 text-rose-600" />
            <span>Laravel / PHP (.env)</span>
          </button>

          <button
            onclick={() => (activeTab = "prisma")}
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'prisma'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900 hover:bg-white/50'}"
          >
            <Code class="w-3.5 h-3.5 text-emerald-600" />
            <span>Node / Prisma / Next</span>
          </button>

          <button
            onclick={() => (activeTab = "django")}
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'django'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900 hover:bg-white/50'}"
          >
            <Terminal class="w-3.5 h-3.5 text-blue-600" />
            <span>Python / Django / FastAPI</span>
          </button>

          <button
            onclick={() => (activeTab = "uri")}
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'uri'
              ? 'bg-white text-slate-900 shadow-xs font-semibold'
              : 'text-slate-600 hover:text-slate-900 hover:bg-white/50'}"
          >
            <Database class="w-3.5 h-3.5 text-amber-600" />
            <span>Raw URI / DSN</span>
          </button>
        </div>

        <div class="bg-slate-50/70 border border-slate-200/80 rounded-xl p-3.5 space-y-3">
          <div class="flex items-center justify-between text-xs text-slate-600 font-medium">
            <span>Customize Connection Parameters</span>
            <span class="text-[11px] text-slate-400">Updates snippet dynamically</span>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 text-xs">
            <div>
              <label for="conn-db-name" class="block text-[11px] font-medium text-slate-600 mb-1">Database</label>
              <input
                id="conn-db-name"
                type="text"
                bind:value={customDbName}
                class="w-full px-2.5 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 font-mono text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
              />
            </div>

            <div>
              <label for="conn-user" class="block text-[11px] font-medium text-slate-600 mb-1">Username</label>
              <input
                id="conn-user"
                type="text"
                bind:value={customUser}
                placeholder="root"
                class="w-full px-2.5 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 font-mono text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
              />
            </div>

            <div>
              <label for="conn-password" class="block text-[11px] font-medium text-slate-600 mb-1">Password</label>
              <div class="relative">
                <input
                  id="conn-password"
                  type={showPassword ? "text" : "password"}
                  bind:value={customPassword}
                  placeholder="(empty by default)"
                  class="w-full px-2.5 py-1.5 pr-8 bg-white border border-slate-200 rounded-lg text-slate-800 font-mono text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
                />
                <button
                  type="button"
                  onclick={() => (showPassword = !showPassword)}
                  class="absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600"
                  title={showPassword ? "Hide password" : "Show password"}
                >
                  {#if showPassword}
                    <EyeOff class="w-3.5 h-3.5" />
                  {:else}
                    <Eye class="w-3.5 h-3.5" />
                  {/if}
                </button>
              </div>
            </div>

            <div>
              <label for="conn-host-port" class="block text-[11px] font-medium text-slate-600 mb-1">Host & Port</label>
              <div class="flex items-center gap-1.5">
                <input
                  id="conn-host-port"
                  type="text"
                  bind:value={customHost}
                  class="w-2/3 px-2.5 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 font-mono text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
                  placeholder="127.0.0.1"
                />
                <input
                  type="text"
                  bind:value={customPort}
                  class="w-1/3 px-2 py-1.5 bg-white border border-slate-200 rounded-lg text-slate-800 font-mono text-xs focus:ring-2 focus:ring-[#94380C]/20 focus:border-[#94380C] outline-hidden"
                  placeholder="Port"
                />
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-1.5">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-slate-800">Generated Configuration Snippet</span>
            <button
              onclick={copySnippet}
              class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white text-xs font-semibold shadow-xs transition-colors"
            >
              {#if copied}
                <Check class="w-3.5 h-3.5" />
                <span>Copied to Clipboard!</span>
              {:else}
                <Copy class="w-3.5 h-3.5" />
                <span>Copy Snippet</span>
              {/if}
            </button>
          </div>

          <div class="relative bg-slate-900 rounded-xl p-4 border border-slate-800 overflow-hidden font-mono text-xs shadow-inner">
            <pre class="text-slate-100 whitespace-pre-wrap break-all select-all font-mono leading-relaxed">{snippet}</pre>
          </div>
        </div>
      </div>

      <div class="px-6 py-3.5 bg-slate-50 border-t border-slate-100 flex items-center justify-between text-xs text-slate-500">
        <span>Paste into your local project <code class="px-1 py-0.5 rounded bg-slate-200 text-slate-700 font-mono">.env</code> file</span>
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
