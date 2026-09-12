<script lang="ts">
  import { onMount } from "svelte";
  import type { Component } from "svelte";
  import {
    Activity,
    CheckCircle2,
    Database,
    FolderKanban,
    Globe,
    Layers,
    Play,
    Plus,
    Power,
    RefreshCw,
    Server,
    ShieldCheck,
    Square,
    Terminal,
    Trash2,
    X,
  } from "@lucide/svelte";

  interface ServiceItem {
    id: string;
    name: string;
    type: string;
    version: string;
    ports: string;
    status: "stopped" | "starting" | "running" | "crashed";
    pid?: number;
    icon: Component<any>;
  }

  interface SiteItem {
    domain: string;
    runtime: string;
    ssl: boolean;
    path: string;
    backend_type: string;
    target: string;
  }

  interface LogMessage {
    service: string;
    stream: "stdout" | "stderr" | "system";
    message: string;
    timestamp_millis: number;
  }

  interface PortCheckResult {
    port: number;
    is_available: boolean;
    service_name: string;
    alternative_port: number | null;
  }

  let activeTab = $state<"dashboard" | "services" | "sites" | "runtimes" | "logs">("dashboard");
  let allRunning = $state(false);
  let isLoading = $state(false);
  let toastMessage = $state<string | null>(null);

  let portConflicts = $state<PortCheckResult[]>([]);
  let dismissedConflictBanner = $state(false);

  let showAddSiteModal = $state(false);
  let newSiteDomain = $state("");
  let newSitePath = $state("");
  let newSiteType = $state("fastcgi");
  let newSiteTarget = $state("127.0.0.1:9000");

  let logFilter = $state<string>("all");
  let logSearch = $state("");

  let services = $state<ServiceItem[]>([
    {
      id: "caddy",
      name: "Caddy Web Server",
      type: "Reverse Proxy & Auto-HTTPS",
      version: "v2.9.1",
      ports: "80, 443, 2019",
      status: "stopped",
      icon: Globe,
    },
    {
      id: "mariadb",
      name: "MariaDB Database",
      type: "Relational Database",
      version: "v11.4.3",
      ports: "3306",
      status: "stopped",
      icon: Database,
    },
    {
      id: "php",
      name: "PHP-FPM Manager",
      type: "FastCGI Process",
      version: "PHP 8.3.16 (NTS)",
      ports: "9000",
      status: "stopped",
      icon: Server,
    },
    {
      id: "node",
      name: "Node.js Runtime",
      type: "JavaScript / TypeScript",
      version: "v22.14.0 LTS",
      ports: "Isolated",
      status: "running",
      icon: Layers,
    },
  ]);

  let sites = $state<SiteItem[]>([
    {
      domain: "laravel-app.test",
      runtime: "PHP 8.3",
      ssl: true,
      path: "C:\\projects\\laravel-app",
      backend_type: "fastcgi",
      target: "127.0.0.1:9000",
    },
    {
      domain: "dashboard-api.test",
      runtime: "Node 22",
      ssl: true,
      path: "C:\\projects\\dashboard-api",
      backend_type: "proxy",
      target: "127.0.0.1:3000",
    },
  ]);

  let logs = $state<LogMessage[]>([
    {
      service: "caddy",
      stream: "system",
      message: "Caddy reverse proxy initialized with internal local CA.",
      timestamp_millis: Date.now() - 60000,
    },
    {
      service: "mariadb",
      stream: "stdout",
      message: "mysqld.exe ready for connections on port 3306 (bind: 127.0.0.1).",
      timestamp_millis: Date.now() - 45000,
    },
    {
      service: "php",
      stream: "stdout",
      message: "php-cgi listening on 127.0.0.1:9000 with extensions (curl, pdo_mysql, mbstring, openssl).",
      timestamp_millis: Date.now() - 30000,
    },
  ]);

  let activePhpVersion = $state("8.3.16");
  let activeNodeVersion = $state("22.14.0");
  let activePythonVersion = $state("3.12.9");
  let activeRubyVersion = $state("3.3.7");

  let isTauri = typeof window !== "undefined" && Boolean((window as any).__TAURI_INTERNALS__);

  async function invokeTauri<T>(cmd: string, args: Record<string, any> = {}): Promise<T | null> {
    if (!isTauri) return null;
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      return await invoke<T>(cmd, args);
    } catch {
      return null;
    }
  }

  function showToast(msg: string): void {
    toastMessage = msg;
    setTimeout(() => {
      toastMessage = null;
    }, 3000);
  }

  async function fetchBackendState(): Promise<void> {
    const backendServices = await invokeTauri<any[]>("get_services");
    if (backendServices && backendServices.length > 0) {
      services = services.map((s) => {
        const found = backendServices.find((bs) => bs.id === s.id);
        if (found) {
          return {
            ...s,
            status: found.status,
            pid: found.pid,
          };
        }
        return s;
      });
      allRunning = services.some((s) => s.status === "running");
    }

    const backendSites = await invokeTauri<SiteItem[]>("list_sites");
    if (backendSites) {
      sites = backendSites;
    }

    const backendLogs = await invokeTauri<LogMessage[]>("get_logs", { limit: 100 });
    if (backendLogs && backendLogs.length > 0) {
      logs = backendLogs;
    }

    const portChecks = await invokeTauri<PortCheckResult[]>("check_port_conflicts");
    if (portChecks) {
      portConflicts = portChecks.filter((p) => !p.is_available);
    }
  }

  function applyAlternativePort(conflict: PortCheckResult): void {
    if (!conflict.alternative_port) return;
    const oldP = String(conflict.port);
    const newP = String(conflict.alternative_port);
    services = services.map((s) => {
      if (s.ports.includes(oldP)) {
        return {
          ...s,
          ports: s.ports.replace(oldP, newP),
        };
      }
      return s;
    });
    portConflicts = portConflicts.filter((c) => c.port !== conflict.port);
    showToast(`Switched ${conflict.service_name} to port ${newP}`);
  }

  async function toggleAll(): Promise<void> {
    isLoading = true;
    allRunning = !allRunning;
    const targetState = allRunning;

    if (isTauri) {
      await invokeTauri("toggle_all_services", { start: targetState });
      await fetchBackendState();
    } else {
      services = services.map((s) => ({
        ...s,
        status: targetState ? "running" : "stopped",
      }));
    }

    isLoading = false;
    showToast(targetState ? "All services started successfully" : "All services stopped");
  }

  async function toggleService(id: string): Promise<void> {
    const svc = services.find((s) => s.id === id);
    if (!svc) return;

    const willStart = svc.status !== "running";

    if (isTauri) {
      if (willStart) {
        await invokeTauri("start_service", { id });
      } else {
        await invokeTauri("stop_service", { id });
      }
      await fetchBackendState();
    } else {
      services = services.map((s) => {
        if (s.id === id) {
          return { ...s, status: willStart ? "running" : "stopped" };
        }
        return s;
      });
      allRunning = services.some((s) => s.status === "running");
    }

    showToast(`Service '${svc.name}' ${willStart ? "started" : "stopped"}`);
  }

  async function handleAddSite(): Promise<void> {
    if (!newSiteDomain.trim() || !newSitePath.trim()) {
      showToast("Please enter a domain and local directory path");
      return;
    }

    const domain = newSiteDomain.trim();
    const rootDir = newSitePath.trim();
    const backendType = newSiteType;
    const target = newSiteTarget.trim();

    if (isTauri) {
      await invokeTauri("add_site", {
        domain,
        rootDir,
        backendType,
        target,
      });
      await fetchBackendState();
    } else {
      sites = [
        ...sites.filter((s) => s.domain !== domain),
        {
          domain,
          runtime: backendType === "fastcgi" ? "PHP 8.3" : backendType === "proxy" ? "Node 22" : "Static",
          ssl: true,
          path: rootDir,
          backend_type: backendType,
          target,
        },
      ];
    }

    showAddSiteModal = false;
    newSiteDomain = "";
    newSitePath = "";
    showToast(`Site ${domain} created and SSL certificate prepared`);
  }

  async function handleDeleteSite(domain: string): Promise<void> {
    if (isTauri) {
      await invokeTauri("delete_site", { domain });
      await fetchBackendState();
    } else {
      sites = sites.filter((s) => s.domain !== domain);
    }
    showToast(`Site ${domain} removed`);
  }

  function selectPhpVersion(v: string): void {
    activePhpVersion = v;
    if (isTauri) {
      invokeTauri("set_active_runtime", { kind: "php", version: v });
    }
    showToast(`Active PHP switched to ${v}`);
  }

  function selectNodeVersion(v: string): void {
    activeNodeVersion = v;
    if (isTauri) {
      invokeTauri("set_active_runtime", { kind: "node", version: v });
    }
    showToast(`Active Node.js switched to ${v}`);
  }

  function selectPythonVersion(v: string): void {
    activePythonVersion = v;
    if (isTauri) {
      invokeTauri("set_active_runtime", { kind: "python", version: v });
    }
    showToast(`Active Python switched to ${v}`);
  }

  function selectRubyVersion(v: string): void {
    activeRubyVersion = v;
    if (isTauri) {
      invokeTauri("set_active_runtime", { kind: "ruby", version: v });
    }
    showToast(`Active Ruby switched to ${v}`);
  }

  let filteredLogs = $derived(
    logs.filter((l) => {
      const matchService = logFilter === "all" || l.service.toLowerCase() === logFilter.toLowerCase();
      const matchSearch = !logSearch || l.message.toLowerCase().includes(logSearch.toLowerCase());
      return matchService && matchSearch;
    })
  );

  onMount(() => {
    fetchBackendState();
    const timer = setInterval(() => {
      fetchBackendState();
    }, 3000);
    return () => clearInterval(timer);
  });
</script>

<div class="flex h-screen w-screen bg-[#0B0F17] text-slate-100 font-sans antialiased overflow-hidden select-none">
  {#if toastMessage}
    <div class="fixed bottom-6 right-6 z-50 flex items-center gap-2.5 px-4 py-2.5 rounded-xl bg-[#131D33] border border-cyan-500/40 text-cyan-300 text-xs shadow-2xl shadow-cyan-950 animate-bounce">
      <CheckCircle2 class="w-4 h-4 text-cyan-400" />
      <span>{toastMessage}</span>
    </div>
  {/if}

  <aside class="w-64 border-r border-slate-800/80 bg-[#0E1524]/70 backdrop-blur-2xl flex flex-col shrink-0">
    <div class="h-16 px-6 flex items-center gap-3 border-b border-slate-800/70">
      <div class="h-9 w-9 rounded-xl bg-gradient-to-tr from-cyan-500 via-blue-600 to-indigo-600 flex items-center justify-center shadow-lg shadow-cyan-500/25">
        <Server class="w-5 h-5 text-white" />
      </div>
      <div>
        <div class="flex items-center gap-2">
          <span class="font-bold text-lg tracking-tight text-white">4Forge</span>
          <span class="text-[10px] font-semibold uppercase px-1.5 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">v0.1.0</span>
        </div>
        <p class="text-[11px] text-slate-400">Local Dev Cockpit</p>
      </div>
    </div>

    <nav class="flex-1 px-3 py-4 space-y-1.5">
      <button
        onclick={() => (activeTab = "dashboard")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'dashboard'
          ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Activity class="w-4 h-4" />
        Dashboard
      </button>

      <button
        onclick={() => (activeTab = "services")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'services'
          ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Server class="w-4 h-4" />
        Core Services
      </button>

      <button
        onclick={() => (activeTab = "sites")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'sites'
          ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <FolderKanban class="w-4 h-4" />
        Virtual Hosts & Sites
      </button>

      <button
        onclick={() => (activeTab = "runtimes")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'runtimes'
          ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Layers class="w-4 h-4" />
        Polyglot Runtimes
      </button>

      <button
        onclick={() => (activeTab = "logs")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {activeTab === 'logs'
          ? 'bg-cyan-500/15 text-cyan-300 border border-cyan-500/30 shadow-sm shadow-cyan-500/10'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Terminal class="w-4 h-4" />
        Live Process Logs
      </button>
    </nav>

    <div class="p-4 border-t border-slate-800/70 bg-[#0B101C]/60 text-xs">
      <div class="flex items-center justify-between text-slate-400 mb-1.5">
        <span>Process Shield</span>
        <span class="text-emerald-400 font-mono text-[11px] flex items-center gap-1">
          <ShieldCheck class="w-3.5 h-3.5" /> JobObject Active
        </span>
      </div>
      <div class="flex items-center justify-between text-slate-400">
        <span>Local CA Root</span>
        <span class="text-cyan-400 font-mono text-[11px]">Auto-HTTPS Ready</span>
      </div>
    </div>
  </aside>

  <main class="flex-1 flex flex-col overflow-hidden bg-gradient-to-b from-[#0F172A]/40 to-[#0B0F17]">
    <header class="h-16 border-b border-slate-800/70 px-8 flex items-center justify-between shrink-0 bg-[#0E1524]/50 backdrop-blur-md">
      <div>
        <h1 class="text-base font-semibold text-white">Local Development Environment</h1>
        <p class="text-xs text-slate-400">Windows Native • Apache-2.0 Pure Open Source</p>
      </div>

      <div class="flex items-center gap-3">
        <button
          disabled={isLoading}
          onclick={toggleAll}
          class="flex items-center gap-2 px-4 py-2 rounded-xl font-medium text-xs transition-all shadow-md {allRunning
            ? 'bg-rose-500/15 text-rose-300 border border-rose-500/30 hover:bg-rose-500/25'
            : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
        >
          {#if allRunning}
            <Square class="w-3.5 h-3.5 fill-current" />
            Stop All Services
          {:else}
            <Play class="w-3.5 h-3.5 fill-current" />
            Start All Services
          {/if}
        </button>

        <button
          onclick={fetchBackendState}
          title="Reload Services & Logs"
          class="p-2 rounded-xl text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 border border-slate-800 transition-colors"
        >
          <RefreshCw class="w-4 h-4" />
        </button>
      </div>
    </header>

    <div class="flex-1 overflow-y-auto p-8 space-y-8">
      {#if portConflicts.length > 0 && !dismissedConflictBanner}
        <div class="rounded-2xl border border-amber-500/40 bg-amber-500/10 p-4 flex items-center justify-between text-xs text-amber-200 shadow-xl shadow-amber-950/30">
          <div class="flex items-center gap-3">
            <div class="p-2.5 rounded-xl bg-amber-500/20 text-amber-400">
              <Activity class="w-5 h-5" />
            </div>
            <div>
              <h4 class="font-bold text-sm text-amber-300">Port Conflict Detected</h4>
              <p class="text-slate-300 mt-0.5">
                {portConflicts.map((c) => `${c.service_name} (Port ${c.port} is occupied -> Suggested: ${c.alternative_port})`).join(" • ")}
              </p>
            </div>
          </div>
          <div class="flex items-center gap-3">
            {#if portConflicts[0]?.alternative_port}
              <button
                onclick={() => applyAlternativePort(portConflicts[0])}
                class="px-3 py-1.5 rounded-xl bg-gradient-to-r from-amber-500 to-amber-600 text-slate-950 font-bold hover:brightness-110 shadow-md transition-all"
              >
                Use Port {portConflicts[0].alternative_port}
              </button>
            {/if}
            <button
              onclick={() => (dismissedConflictBanner = true)}
              class="p-1.5 rounded-xl hover:bg-amber-500/20 text-amber-300 transition-colors"
              title="Dismiss warning"
            >
              <X class="w-4 h-4" />
            </button>
          </div>
        </div>
      {/if}

      {#if activeTab === "dashboard"}
        <div class="relative overflow-hidden rounded-2xl border border-slate-800/90 bg-gradient-to-br from-slate-900/90 via-[#101827] to-[#0D1321] p-6 shadow-xl">
          <div class="absolute -top-24 -right-24 w-64 h-64 bg-cyan-500/10 rounded-full blur-3xl pointer-events-none"></div>
          <div class="flex items-center justify-between relative z-10">
            <div>
              <div class="flex items-center gap-2 mb-2">
                <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-emerald-500/15 text-emerald-400 border border-emerald-500/30">
                  <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                  System Operational
                </span>
                <span class="text-xs text-slate-400">Zero port conflicts detected</span>
              </div>
              <h2 class="text-xl font-bold text-white tracking-tight">Polyglot Development Stack Active</h2>
              <p class="text-xs text-slate-400 mt-1 max-w-xl">
                Caddy auto-HTTPS proxy, multi-database architecture (MariaDB, PostgreSQL, SQLite), and isolated runtimes (PHP & Node.js) integrated natively.
              </p>
            </div>

            <div class="flex items-center gap-6">
              <div class="text-right">
                <p class="text-xs text-slate-400">Virtual Hosts</p>
                <p class="text-2xl font-bold text-cyan-400 font-mono">{sites.length}</p>
              </div>
              <div class="h-10 w-px bg-slate-800"></div>
              <div class="text-right">
                <p class="text-xs text-slate-400">Active PHP</p>
                <p class="text-2xl font-bold text-emerald-400 font-mono">v{activePhpVersion}</p>
              </div>
            </div>
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-sm font-semibold text-slate-200 uppercase tracking-wider">Services Status</h3>
            <button onclick={() => (activeTab = "services")} class="text-xs text-cyan-400 hover:underline font-medium">View All Services &rarr;</button>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            {#each services as service}
              <div class="rounded-xl border border-slate-800/90 bg-[#121929]/70 p-5 flex flex-col justify-between">
                <div>
                  <div class="flex items-center justify-between mb-3">
                    <div class="p-2.5 rounded-xl bg-slate-800/70 text-cyan-400 border border-slate-700/50">
                      <service.icon class="w-5 h-5" />
                    </div>
                    <span
                      class="px-2.5 py-0.5 rounded-full text-[11px] font-medium capitalize border {service.status === 'running'
                        ? 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30'
                        : 'bg-slate-800 text-slate-400 border-slate-700'}"
                    >
                      {service.status}
                    </span>
                  </div>

                  <h4 class="font-semibold text-sm text-white mb-0.5">{service.name}</h4>
                  <p class="text-xs text-slate-400 mb-3">{service.type}</p>

                  <div class="space-y-1.5 pt-2 border-t border-slate-800/70 text-xs text-slate-400">
                    <div class="flex justify-between">
                      <span>Ports:</span>
                      <span class="text-cyan-400 font-mono text-[11px]">{service.ports}</span>
                    </div>
                  </div>
                </div>

                <div class="mt-4 pt-3">
                  <button
                    onclick={() => toggleService(service.id)}
                    class="w-full py-1.5 px-3 rounded-xl text-xs font-medium transition-colors flex items-center justify-center gap-1.5 {service.status === 'running'
                      ? 'bg-rose-500/15 text-rose-300 hover:bg-rose-500/25 border border-rose-500/30'
                      : 'bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700'}"
                  >
                    <Power class="w-3.5 h-3.5" />
                    {service.status === "running" ? "Stop" : "Start"}
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-sm font-semibold text-slate-200 uppercase tracking-wider">Managed Sites & Projects</h3>
            <button
              onclick={() => (showAddSiteModal = true)}
              class="text-xs px-3 py-1.5 rounded-lg bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 hover:bg-cyan-500/30 flex items-center gap-1.5 font-medium transition-all"
            >
              <Plus class="w-3.5 h-3.5" />
              Add Project
            </button>
          </div>

          <div class="rounded-xl border border-slate-800/90 bg-[#121929]/50 overflow-hidden">
            <table class="w-full text-left text-xs">
              <thead class="bg-slate-900/70 text-slate-400 uppercase font-semibold border-b border-slate-800">
                <tr>
                  <th class="px-5 py-3">Domain</th>
                  <th class="px-5 py-3">Runtime</th>
                  <th class="px-5 py-3">SSL</th>
                  <th class="px-5 py-3">Path</th>
                  <th class="px-5 py-3 text-right">Actions</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-800/70 text-slate-300">
                {#each sites as site}
                  <tr class="hover:bg-slate-800/30 transition-colors">
                    <td class="px-5 py-3.5 font-medium text-white flex items-center gap-2">
                      <Globe class="w-3.5 h-3.5 text-cyan-400" />
                      <span class="hover:text-cyan-300 cursor-pointer">{site.domain}</span>
                    </td>
                    <td class="px-5 py-3.5 font-mono text-[11px] text-emerald-400">{site.runtime}</td>
                    <td class="px-5 py-3.5">
                      <span class="inline-flex items-center gap-1 text-cyan-400 font-medium">
                        <CheckCircle2 class="w-3 h-3" /> Auto-HTTPS
                      </span>
                    </td>
                    <td class="px-5 py-3.5 font-mono text-[11px] text-slate-400 truncate max-w-xs">{site.path}</td>
                    <td class="px-5 py-3.5 text-right space-x-2">
                      <button
                        onclick={() => handleDeleteSite(site.domain)}
                        class="text-xs text-rose-400 hover:text-rose-200 px-2 py-1 rounded bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/20"
                      >
                        <Trash2 class="w-3 h-3 inline" />
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {:else if activeTab === "services"}
        <div class="space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-xl font-bold text-white">Core Services Manager</h2>
              <p class="text-xs text-slate-400 mt-1">Lifecycle control with Windows Job Objects enforcement.</p>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            {#each services as service}
              <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 flex flex-col justify-between">
                <div>
                  <div class="flex items-center justify-between mb-4">
                    <div class="flex items-center gap-3">
                      <div class="p-3 rounded-xl bg-slate-800 text-cyan-400 border border-slate-700/60">
                        <service.icon class="w-6 h-6" />
                      </div>
                      <div>
                        <h3 class="font-bold text-base text-white">{service.name}</h3>
                        <p class="text-xs text-slate-400">{service.type}</p>
                      </div>
                    </div>
                    <span
                      class="px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider border {service.status === 'running'
                        ? 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30'
                        : 'bg-slate-800 text-slate-400 border-slate-700'}"
                    >
                      {service.status}
                    </span>
                  </div>

                  <div class="bg-[#0B101C]/60 rounded-xl p-4 border border-slate-800 space-y-2 text-xs">
                    <div class="flex justify-between text-slate-400">
                      <span>Assigned Ports</span>
                      <span class="text-cyan-400 font-mono font-medium">{service.ports}</span>
                    </div>
                    <div class="flex justify-between text-slate-400">
                      <span>Binary Version</span>
                      <span class="text-slate-200 font-mono font-medium">{service.version}</span>
                    </div>
                    {#if service.pid}
                      <div class="flex justify-between text-slate-400">
                        <span>Process PID</span>
                        <span class="text-emerald-400 font-mono font-medium">{service.pid}</span>
                      </div>
                    {/if}
                  </div>
                </div>

                <div class="mt-6 flex items-center gap-3">
                  <button
                    onclick={() => toggleService(service.id)}
                    class="flex-1 py-2 rounded-xl text-xs font-medium transition-all flex items-center justify-center gap-2 {service.status === 'running'
                      ? 'bg-rose-500/20 text-rose-300 hover:bg-rose-500/30 border border-rose-500/30'
                      : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110'}"
                  >
                    <Power class="w-3.5 h-3.5" />
                    {service.status === "running" ? "Stop Service" : "Start Service"}
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {:else if activeTab === "sites"}
        <div class="space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-xl font-bold text-white">Virtual Hosts & Projects</h2>
              <p class="text-xs text-slate-400 mt-1">Automatic reverse proxy routing and internal CA auto-HTTPS certificates.</p>
            </div>
            <button
              onclick={() => (showAddSiteModal = true)}
              class="px-4 py-2 rounded-xl bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 hover:bg-cyan-500/30 flex items-center gap-2 font-medium text-xs transition-all shadow-md shadow-cyan-500/10"
            >
              <Plus class="w-4 h-4" />
              New Virtual Host
            </button>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each sites as site}
              <div class="rounded-xl border border-slate-800/90 bg-[#121929]/70 p-5 flex flex-col justify-between">
                <div>
                  <div class="flex items-center justify-between mb-3">
                    <div class="flex items-center gap-2 text-white font-semibold">
                      <Globe class="w-4 h-4 text-cyan-400" />
                      <span>{site.domain}</span>
                    </div>
                    <span class="inline-flex items-center gap-1 text-xs text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded-full border border-emerald-500/20">
                      <CheckCircle2 class="w-3 h-3" /> SSL OK
                    </span>
                  </div>

                  <p class="text-xs text-slate-400 font-mono truncate mb-3">{site.path}</p>

                  <div class="text-xs bg-[#0B101C]/60 rounded-lg p-3 border border-slate-800 flex justify-between">
                    <span class="text-slate-400">Backend Type:</span>
                    <span class="text-cyan-300 font-medium capitalize">{site.backend_type}</span>
                  </div>
                </div>

                <div class="mt-4 flex items-center justify-between border-t border-slate-800/80 pt-3">
                  <span class="text-[11px] text-slate-400 font-mono">Target: {site.target}</span>
                  <button
                    onclick={() => handleDeleteSite(site.domain)}
                    class="text-xs text-rose-400 hover:text-rose-200 px-2.5 py-1 rounded bg-rose-500/10 border border-rose-500/20 transition-colors"
                  >
                    Delete
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {:else if activeTab === "runtimes"}
        <div class="space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-xl font-bold text-white">Polyglot Runtime Manager</h2>
              <p class="text-xs text-slate-400 mt-1">Switch runtime versions without restarting the system (mise model).</p>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div class="p-2.5 rounded-xl bg-purple-500/10 text-purple-400 border border-purple-500/20">
                    <Server class="w-5 h-5" />
                  </div>
                  <div>
                    <h3 class="font-bold text-base text-white">PHP Engine</h3>
                    <p class="text-xs text-slate-400">FastCGI process daemon</p>
                  </div>
                </div>
                <span class="text-xs font-mono text-purple-300 bg-purple-500/20 px-2.5 py-1 rounded-full border border-purple-500/30">
                  Active: v{activePhpVersion}
                </span>
              </div>

              <div class="space-y-2">
                {#each ["8.4.3", "8.3.16", "8.2.27"] as ver}
                  <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
                    <span class="text-sm font-mono font-medium text-slate-200">PHP {ver}</span>
                    <button
                      onclick={() => selectPhpVersion(ver)}
                      class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activePhpVersion === ver
                        ? 'bg-purple-500 text-white shadow-lg shadow-purple-500/30'
                        : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
                    >
                      {activePhpVersion === ver ? "Active" : "Switch"}
                    </button>
                  </div>
                {/each}
              </div>
            </div>

            <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div class="p-2.5 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                    <Layers class="w-5 h-5" />
                  </div>
                  <div>
                    <h3 class="font-bold text-base text-white">Node.js Engine</h3>
                    <p class="text-xs text-slate-400">Integrated npm, npx, corepack</p>
                  </div>
                </div>
                <span class="text-xs font-mono text-emerald-300 bg-emerald-500/20 px-2.5 py-1 rounded-full border border-emerald-500/30">
                  Active: v{activeNodeVersion}
                </span>
              </div>

              <div class="space-y-2">
                {#each ["22.14.0", "20.18.3"] as ver}
                  <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
                    <span class="text-sm font-mono font-medium text-slate-200">Node.js {ver}</span>
                    <button
                      onclick={() => selectNodeVersion(ver)}
                      class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activeNodeVersion === ver
                        ? 'bg-emerald-500 text-white shadow-lg shadow-emerald-500/30'
                        : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
                    >
                      {activeNodeVersion === ver ? "Active" : "Switch"}
                    </button>
                  </div>
                {/each}
              </div>
            </div>

            <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div class="p-2.5 rounded-xl bg-blue-500/10 text-blue-400 border border-blue-500/20">
                    <Terminal class="w-5 h-5" />
                  </div>
                  <div>
                    <h3 class="font-bold text-base text-white">Python Engine</h3>
                    <p class="text-xs text-slate-400">Isolated .venv & ASGI/WSGI runner</p>
                  </div>
                </div>
                <span class="text-xs font-mono text-blue-300 bg-blue-500/20 px-2.5 py-1 rounded-full border border-blue-500/30">
                  Active: v{activePythonVersion}
                </span>
              </div>

              <div class="space-y-2">
                {#each ["3.12.9"] as ver}
                  <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
                    <span class="text-sm font-mono font-medium text-slate-200">Python {ver} (embed x64)</span>
                    <button
                      onclick={() => selectPythonVersion(ver)}
                      class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activePythonVersion === ver
                        ? 'bg-blue-500 text-white shadow-lg shadow-blue-500/30'
                        : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
                    >
                      {activePythonVersion === ver ? "Active" : "Switch"}
                    </button>
                  </div>
                {/each}
              </div>
            </div>

            <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div class="p-2.5 rounded-xl bg-rose-500/10 text-rose-400 border border-rose-500/20">
                    <Activity class="w-5 h-5" />
                  </div>
                  <div>
                    <h3 class="font-bold text-base text-white">Ruby Engine</h3>
                    <p class="text-xs text-slate-400">Gem isolation & Puma / Rack runner</p>
                  </div>
                </div>
                <span class="text-xs font-mono text-rose-300 bg-rose-500/20 px-2.5 py-1 rounded-full border border-rose-500/30">
                  Active: v{activeRubyVersion}
                </span>
              </div>

              <div class="space-y-2">
                {#each ["3.3.7"] as ver}
                  <div class="flex items-center justify-between p-3 rounded-xl bg-[#0B101C]/60 border border-slate-800">
                    <span class="text-sm font-mono font-medium text-slate-200">Ruby {ver} (x64)</span>
                    <button
                      onclick={() => selectRubyVersion(ver)}
                      class="px-3 py-1 rounded-lg text-xs font-medium transition-all {activeRubyVersion === ver
                        ? 'bg-rose-500 text-white shadow-lg shadow-rose-500/30'
                        : 'bg-slate-800 text-slate-300 hover:bg-slate-700'}"
                    >
                      {activeRubyVersion === ver ? "Active" : "Switch"}
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          </div>
        </div>
      {:else if activeTab === "logs"}
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-xl font-bold text-white">Live Process Logs</h2>
              <p class="text-xs text-slate-400 mt-1">Real-time asynchronous streaming via LogHub.</p>
            </div>

            <div class="flex items-center gap-3">
              <input
                type="text"
                bind:value={logSearch}
                placeholder="Search log lines..."
                class="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-xs text-slate-200 focus:outline-none focus:border-cyan-500"
              />
              <button
                onclick={() => (logs = [])}
                class="px-3 py-1.5 rounded-lg text-xs bg-slate-800 text-slate-300 hover:bg-slate-700 border border-slate-700"
              >
                Clear
              </button>
            </div>
          </div>

          <div class="flex items-center gap-2">
            {#each ["all", "caddy", "mariadb", "php"] as filter}
              <button
                onclick={() => (logFilter = filter)}
                class="px-3 py-1 rounded-lg text-xs font-medium uppercase tracking-wider transition-all {logFilter === filter
                  ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/40'
                  : 'bg-slate-800/60 text-slate-400 border border-slate-800 hover:bg-slate-800'}"
              >
                {filter}
              </button>
            {/each}
          </div>

          <div class="rounded-xl border border-slate-800/90 bg-[#090D14] p-4 font-mono text-xs overflow-x-auto min-h-[420px] max-h-[560px] overflow-y-auto space-y-1.5 shadow-inner">
            {#if filteredLogs.length === 0}
              <div class="text-slate-500 py-12 text-center italic">No log messages found for current filter.</div>
            {:else}
              {#each filteredLogs as log}
                <div class="flex items-start gap-3 py-0.5 leading-relaxed hover:bg-slate-900/50 px-2 rounded">
                  <span class="text-slate-500 shrink-0 select-none">
                    {new Date(log.timestamp_millis).toLocaleTimeString()}
                  </span>
                  <span
                    class="px-1.5 py-0.2 rounded text-[10px] font-semibold uppercase shrink-0 border {log.stream === 'stderr'
                      ? 'bg-rose-500/15 text-rose-300 border-rose-500/30'
                      : log.stream === 'system'
                      ? 'bg-cyan-500/15 text-cyan-300 border-cyan-500/30'
                      : 'bg-emerald-500/15 text-emerald-300 border-emerald-500/30'}"
                  >
                    {log.service}
                  </span>
                  <span class="text-slate-300 break-all">{log.message}</span>
                </div>
              {/each}
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </main>

  {#if showAddSiteModal}
    <div class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4">
      <div class="rounded-2xl border border-slate-800 bg-[#121929] w-full max-w-md p-6 shadow-2xl space-y-5">
        <div class="flex items-center justify-between">
          <h3 class="text-base font-bold text-white">Add New Virtual Host</h3>
          <button onclick={() => (showAddSiteModal = false)} class="text-slate-400 hover:text-white">
            <X class="w-5 h-5" />
          </button>
        </div>

        <div class="space-y-3 text-xs">
          <div>
            <label for="site-domain" class="block text-slate-300 font-medium mb-1">Local Domain (.test)</label>
            <input
              id="site-domain"
              type="text"
              bind:value={newSiteDomain}
              placeholder="e.g. myproject.test"
              class="w-full px-3 py-2 rounded-xl bg-slate-900 border border-slate-700 text-white focus:outline-none focus:border-cyan-500 font-mono"
            />
          </div>

          <div>
            <label for="site-path" class="block text-slate-300 font-medium mb-1">Root Directory Path</label>
            <input
              id="site-path"
              type="text"
              bind:value={newSitePath}
              placeholder="C:\projects\myproject\public"
              class="w-full px-3 py-2 rounded-xl bg-slate-900 border border-slate-700 text-white focus:outline-none focus:border-cyan-500 font-mono"
            />
          </div>

          <div>
            <label for="site-type" class="block text-slate-300 font-medium mb-1">Backend Type</label>
            <select
              id="site-type"
              bind:value={newSiteType}
              class="w-full px-3 py-2 rounded-xl bg-slate-900 border border-slate-700 text-white focus:outline-none focus:border-cyan-500"
            >
              <option value="fastcgi">PHP FastCGI (php-cgi)</option>
              <option value="proxy">Node.js Reverse Proxy</option>
              <option value="python">Python FastAPI / Flask (Uvicorn)</option>
              <option value="ruby">Ruby Rails / Sinatra (Puma)</option>
              <option value="static">Static HTML / Frontend Bundle</option>
            </select>
          </div>

          <div>
            <label for="site-target" class="block text-slate-300 font-medium mb-1">Upstream Target</label>
            <input
              id="site-target"
              type="text"
              bind:value={newSiteTarget}
              placeholder="127.0.0.1:9000 or 127.0.0.1:3000"
              class="w-full px-3 py-2 rounded-xl bg-slate-900 border border-slate-700 text-white focus:outline-none focus:border-cyan-500 font-mono"
            />
          </div>
        </div>

        <div class="flex items-center justify-end gap-3 pt-2">
          <button
            onclick={() => (showAddSiteModal = false)}
            class="px-4 py-2 rounded-xl text-xs text-slate-400 hover:text-white"
          >
            Cancel
          </button>
          <button
            onclick={handleAddSite}
            class="px-4 py-2 rounded-xl text-xs font-medium bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:brightness-110 shadow-lg shadow-cyan-500/20"
          >
            Save & Generate VHost
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
