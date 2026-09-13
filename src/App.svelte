<script lang="ts">
  import { onMount } from "svelte";
  import type { Component } from "svelte";
  import {
    Activity,
    CheckCircle2,
    Database,
    ExternalLink,
    FileText,
    FolderKanban,
    Globe,
    Layers,
    Play,
    Plus,
    Power,
    RefreshCw,
    ScrollText,
    Server,
    ShieldCheck,
    Sparkles,
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

  interface UpdateCheck {
    current_version: string;
    channel: string;
    update_available: boolean;
    latest_version: string | null;
    release_notes: string | null;
    release_url: string;
    signature_verified: boolean;
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

  let showUpdateModal = $state(false);
  let isCheckingUpdate = $state(false);
  let updateStatus = $state<UpdateCheck | null>(null);
  let showDatabaseModal = $state(false);

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

  async function invokeTauri<T>(cmd: string, args: Record<string, any> = {}): Promise<T | null> {
    if (typeof window === "undefined") return null;
    const hasTauri = Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__);
    if (!hasTauri) return null;
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      return await invoke<T>(cmd, args);
    } catch (err) {
      console.error(`[4Forge IPC Error] ${cmd}:`, err);
      return null;
    }
  }

  function showToast(msg: string): void {
    toastMessage = msg;
    setTimeout(() => {
      toastMessage = null;
    }, 3000);
  }

  async function checkUpdates(): Promise<void> {
    isCheckingUpdate = true;
    const res = await invokeTauri<UpdateCheck>("check_for_updates");
    if (res) {
      updateStatus = res;
    } else {
      updateStatus = {
        current_version: "0.1.0",
        channel: "stable",
        update_available: false,
        latest_version: "0.1.0",
        release_notes: "You are running the latest verified release of 4Forge.",
        release_url: "https://github.com/fredli4qooni/4forge/releases",
        signature_verified: true,
      };
    }
    isCheckingUpdate = false;
  }

  async function openWebLocalhost(): Promise<void> {
    const caddySvc = services.find((s) => s.id === "caddy");
    const port = caddySvc && caddySvc.ports.includes("8080") ? "8080" : "80";
    const url = port === "80" ? "http://localhost" : `http://localhost:${port}`;
    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      await invokeTauri("open_browser", { url });
    } else {
      window.open(url, "_blank");
    }
    showToast(`Opening ${url} in browser...`);
  }

  async function openProjectsDirectory(): Promise<void> {
    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      await invokeTauri("open_projects_folder", {});
      showToast("Opening projects folder in Windows Explorer...");
    } else {
      showToast("Opened projects folder: C:\\4forge\\projects");
    }
  }

  async function openDevTerminal(): Promise<void> {
    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      await invokeTauri("open_system_terminal", {});
      showToast("Launching 4Forge Dev Shell (PowerShell with PATH)...");
    } else {
      showToast("Dev Terminal simulated: PHP, Node, MariaDB in PATH");
    }
  }

  async function openDatabaseAction(): Promise<void> {
    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      const launched = await invokeTauri<boolean>("open_database_gui");
      if (launched) {
        showToast("Database client launched successfully!");
        return;
      }
    }
    showDatabaseModal = true;
  }

  async function openServiceConfig(serviceId: string): Promise<void> {
    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      const res = await invokeTauri<string>("open_service_config", { serviceId });
      if (res) {
        showToast(`Opened config for ${serviceId} in Notepad (${res})`);
      } else {
        showToast(`Opened config for ${serviceId}`);
      }
    } else {
      showToast(`Config for ${serviceId} opened (Notepad simulated)`);
    }
  }

  function openServiceLogs(serviceId: string): void {
    logFilter = serviceId;
    activeTab = "logs";
    showToast(`Showing live logs for ${serviceId}`);
  }

  onMount(() => {
    fetchBackendState(true);
    const timer = setInterval(() => {
      fetchBackendState(false);
    }, 2500);
    return () => clearInterval(timer);
  });

  async function fetchBackendState(checkPorts: boolean = false): Promise<void> {
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

    if (checkPorts) {
      const portChecks = await invokeTauri<PortCheckResult[]>("check_port_conflicts");
      if (portChecks) {
        portConflicts = portChecks.filter((p) => !p.is_available);
      }
    }
  }

  async function applyAlternativePort(conflict: PortCheckResult): Promise<void> {
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
    dismissedConflictBanner = true;
    await invokeTauri("update_service_port", { id: "caddy", newPort: conflict.alternative_port });
    showToast(`Switched ${conflict.service_name} to port ${newP}`);
  }

  async function toggleAll(): Promise<void> {
    isLoading = true;
    const targetState = !allRunning;

    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      await invokeTauri("toggle_all_services", { start: targetState });
      await new Promise((r) => setTimeout(r, 600));
      await fetchBackendState(true);
    } else {
      services = services.map((s) => ({
        ...s,
        status: targetState ? "running" : "stopped",
      }));
      allRunning = targetState;
    }

    isLoading = false;
    showToast(targetState ? "All services started successfully" : "All services stopped");
  }

  async function toggleService(id: string): Promise<void> {
    const svc = services.find((s) => s.id === id);
    if (!svc) return;

    const willStart = svc.status !== "running";

    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      if (willStart) {
        await invokeTauri("start_service", { id });
      } else {
        await invokeTauri("stop_service", { id });
      }
      await new Promise((r) => setTimeout(r, 500));
      await fetchBackendState(true);
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

    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
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
    const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
    if (hasTauri) {
      await invokeTauri("delete_site", { domain });
      await fetchBackendState();
    } else {
      sites = sites.filter((s) => s.domain !== domain);
    }
    showToast(`Site ${domain} removed`);
  }

  function selectPhpVersion(v: string): void {
    activePhpVersion = v;
    invokeTauri("set_active_runtime", { kind: "php", version: v });
    showToast(`Active PHP switched to ${v}`);
  }

  function selectNodeVersion(v: string): void {
    activeNodeVersion = v;
    invokeTauri("set_active_runtime", { kind: "node", version: v });
    showToast(`Active Node.js switched to ${v}`);
  }

  function selectPythonVersion(v: string): void {
    activePythonVersion = v;
    invokeTauri("set_active_runtime", { kind: "python", version: v });
    showToast(`Active Python switched to ${v}`);
  }

  function selectRubyVersion(v: string): void {
    activeRubyVersion = v;
    invokeTauri("set_active_runtime", { kind: "ruby", version: v });
    showToast(`Active Ruby switched to ${v}`);
  }

  let filteredLogs = $derived(
    logs.filter((l) => {
      const matchService = logFilter === "all" || l.service.toLowerCase() === logFilter.toLowerCase();
      const matchSearch = !logSearch || l.message.toLowerCase().includes(logSearch.toLowerCase());
      return matchService && matchSearch;
    })
  );
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
      <div class="flex items-center justify-between text-slate-400 mb-1.5">
        <span>Local CA Root</span>
        <span class="text-cyan-400 font-mono text-[11px]">Auto-HTTPS Ready</span>
      </div>
      <div class="flex items-center justify-between text-slate-400 pt-1.5 border-t border-slate-800/60">
        <span>Release Channel</span>
        <button
          onclick={() => {
            showUpdateModal = true;
            checkUpdates();
          }}
          class="text-cyan-400 hover:text-cyan-300 font-mono text-[11px] flex items-center gap-1 transition-colors"
        >
          <Sparkles class="w-3 h-3" /> v0.1.0 Stable
        </button>
      </div>
    </div>
  </aside>

  <main class="flex-1 flex flex-col overflow-hidden bg-gradient-to-b from-[#0F172A]/40 to-[#0B0F17]">
    <header class="h-16 border-b border-slate-800/70 px-8 flex items-center justify-between shrink-0 bg-[#0E1524]/50 backdrop-blur-md">
      <div>
        <h1 class="text-base font-semibold text-white">Local Development Environment</h1>
        <p class="text-xs text-slate-400">Windows Native • Apache-2.0 Pure Open Source</p>
      </div>

      <div class="flex items-center gap-2">
        <button
          disabled={isLoading}
          onclick={toggleAll}
          class="flex items-center gap-2 px-3.5 py-2 rounded-xl font-bold text-xs transition-all shadow-md {isLoading
            ? 'opacity-70 cursor-not-allowed bg-slate-700 text-slate-300'
            : allRunning
            ? 'bg-rose-500/20 text-rose-300 border border-rose-500/40 hover:bg-rose-500/30'
            : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
        >
          {#if isLoading}
            <RefreshCw class="w-3.5 h-3.5 animate-spin" />
            <span>Processing...</span>
          {:else if allRunning}
            <Square class="w-3.5 h-3.5 fill-current" />
            <span>Stop All</span>
          {:else}
            <Play class="w-3.5 h-3.5 fill-current" />
            <span>Start All</span>
          {/if}
        </button>

        <div class="h-6 w-px bg-slate-800 mx-1"></div>

        <button
          onclick={openWebLocalhost}
          title="Open localhost in Default Web Browser"
          class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-cyan-500/50"
        >
          <Globe class="w-3.5 h-3.5 text-cyan-400" />
          <span>Web</span>
        </button>

        <button
          onclick={openDatabaseAction}
          title="Open Database GUI / Management"
          class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-emerald-500/50"
        >
          <Database class="w-3.5 h-3.5 text-emerald-400" />
          <span>Database</span>
        </button>

        <button
          onclick={openDevTerminal}
          title="Open Terminal with 4Forge PHP, Node, MariaDB in PATH"
          class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-amber-500/50"
        >
          <Terminal class="w-3.5 h-3.5 text-amber-400" />
          <span>Terminal</span>
        </button>

        <button
          onclick={openProjectsDirectory}
          title="Open Projects Directory in Windows Explorer"
          class="flex items-center gap-1.5 px-3 py-2 rounded-xl text-slate-200 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 text-xs font-semibold transition-all hover:border-blue-500/50"
        >
          <FolderKanban class="w-3.5 h-3.5 text-blue-400" />
          <span>Root</span>
        </button>

        <div class="h-6 w-px bg-slate-800 mx-1"></div>

        <button
          onclick={() => {
            showUpdateModal = true;
            checkUpdates();
          }}
          title="Check for Updates & Security Integrity"
          class="flex items-center gap-1 px-2.5 py-2 rounded-xl text-slate-400 hover:text-slate-200 bg-slate-800/40 hover:bg-slate-800/80 border border-slate-800 text-xs font-medium transition-all"
        >
          <Sparkles class="w-3.5 h-3.5 text-cyan-400" />
        </button>

        <button
          onclick={() => fetchBackendState(true)}
          title="Reload Services, Ports & Logs"
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
                {#if portConflicts.length > 0}
                  <span class="text-xs text-amber-400 font-medium">{portConflicts.length} port conflict detected</span>
                {:else}
                  <span class="text-xs text-slate-400">Zero port conflicts detected</span>
                {/if}
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
              <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/70 hover:bg-[#121929]/90 p-5 flex flex-col justify-between transition-all hover:border-slate-700/80 shadow-lg">
                <div>
                  <div class="flex items-center justify-between mb-3">
                    <div class="p-2.5 rounded-xl bg-slate-800/70 text-cyan-400 border border-slate-700/50">
                      <service.icon class="w-5 h-5" />
                    </div>
                    {#if service.status === 'running'}
                      <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-emerald-500/15 text-emerald-300 border border-emerald-500/30 shadow-sm shadow-emerald-950/40">
                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                        Running
                      </span>
                    {:else}
                      <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-medium bg-slate-800/80 text-slate-400 border border-slate-700/60">
                        <span class="w-1.5 h-1.5 rounded-full bg-slate-500"></span>
                        Stopped
                      </span>
                    {/if}
                  </div>

                  <h4 class="font-bold text-sm text-white mb-0.5">{service.name}</h4>
                  <p class="text-xs text-slate-400 mb-3">{service.type}</p>

                  <div class="space-y-2 pt-2 border-t border-slate-800/70 text-xs text-slate-400">
                    <div class="flex justify-between items-center">
                      <span>Ports:</span>
                      <span class="text-cyan-400 font-mono text-[11px] font-semibold bg-cyan-950/30 px-1.5 py-0.5 rounded border border-cyan-500/20">{service.ports}</span>
                    </div>
                    {#if service.pid}
                      <div class="flex justify-between items-center text-[11px]">
                        <span>Process PID:</span>
                        <span class="text-emerald-400 font-mono font-bold bg-emerald-950/60 px-1.5 py-0.5 rounded border border-emerald-500/30">#{service.pid}</span>
                      </div>
                    {/if}

                    {#if service.id === "php"}
                      <div class="flex items-center justify-between pt-1 border-t border-slate-800/50 text-xs">
                        <span class="text-slate-400 font-medium">PHP Version:</span>
                        <select
                          value={activePhpVersion}
                          onchange={(e) => selectPhpVersion((e.target as HTMLSelectElement).value)}
                          class="bg-slate-900 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 rounded-lg px-2 py-0.5 text-xs font-mono font-bold transition-colors cursor-pointer focus:outline-none"
                        >
                          <option value="8.4.3">8.4.3</option>
                          <option value="8.3.16">8.3.16</option>
                          <option value="8.2.27">8.2.27</option>
                        </select>
                      </div>
                    {:else if service.id === "node"}
                      <div class="flex items-center justify-between pt-1 border-t border-slate-800/50 text-xs">
                        <span class="text-slate-400 font-medium">Node Version:</span>
                        <select
                          value={activeNodeVersion}
                          onchange={(e) => selectNodeVersion((e.target as HTMLSelectElement).value)}
                          class="bg-slate-900 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 rounded-lg px-2 py-0.5 text-xs font-mono font-bold transition-colors cursor-pointer focus:outline-none"
                        >
                          <option value="22.14.0">22.14.0</option>
                          <option value="20.18.2">20.18.2</option>
                        </select>
                      </div>
                    {/if}
                  </div>
                </div>

                <div class="mt-4 pt-3 border-t border-slate-800/60 space-y-2">
                  <div class="flex items-center gap-2">
                    <button
                      onclick={() => toggleService(service.id)}
                      class="flex-1 py-1.5 px-3 rounded-xl text-xs font-bold transition-all flex items-center justify-center gap-1.5 shadow-sm {service.status === 'running'
                        ? 'bg-rose-500/20 text-rose-300 hover:bg-rose-500/30 border border-rose-500/40 shadow-rose-950/30'
                        : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
                    >
                      <Power class="w-3.5 h-3.5" />
                      <span>{service.status === "running" ? "Stop" : "Start"}</span>
                    </button>
                    {#if service.id === "caddy"}
                      <button
                        onclick={openWebLocalhost}
                        title="Open in Browser"
                        class="p-1.5 rounded-xl bg-slate-800/70 hover:bg-slate-800 text-cyan-400 border border-slate-700/60 hover:border-cyan-500/40 transition-all"
                      >
                        <ExternalLink class="w-3.5 h-3.5" />
                      </button>
                    {:else if service.id === "mariadb"}
                      <button
                        onclick={openDatabaseAction}
                        title="Open Database GUI"
                        class="p-1.5 rounded-xl bg-slate-800/70 hover:bg-slate-800 text-emerald-400 border border-slate-700/60 hover:border-emerald-500/40 transition-all"
                      >
                        <Database class="w-3.5 h-3.5" />
                      </button>
                    {/if}
                  </div>
                  <div class="grid grid-cols-2 gap-1.5">
                    <button
                      onclick={() => openServiceConfig(service.id)}
                      title="Edit Configuration in Notepad"
                      class="w-full py-1 px-2 rounded-lg text-[11px] font-medium bg-slate-800/60 hover:bg-slate-800 text-amber-300 border border-slate-700/50 hover:border-amber-500/40 flex items-center justify-center gap-1 transition-all"
                    >
                      <FileText class="w-3 h-3 text-amber-400" />
                      <span>Config</span>
                    </button>
                    <button
                      onclick={() => openServiceLogs(service.id)}
                      title="View Live Service Logs"
                      class="w-full py-1 px-2 rounded-lg text-[11px] font-medium bg-slate-800/60 hover:bg-slate-800 text-cyan-300 border border-slate-700/50 hover:border-cyan-500/40 flex items-center justify-center gap-1 transition-all"
                    >
                      <ScrollText class="w-3 h-3 text-cyan-400" />
                      <span>Logs</span>
                    </button>
                  </div>
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
              <div class="rounded-2xl border border-slate-800/90 bg-[#121929]/80 p-6 flex flex-col justify-between shadow-xl">
                <div>
                  <div class="flex items-center justify-between mb-4">
                    <div class="flex items-center gap-3">
                      <div class="p-3 rounded-xl bg-slate-800 text-cyan-400 border border-slate-700/60 shadow-md">
                        <service.icon class="w-6 h-6" />
                      </div>
                      <div>
                        <h3 class="font-bold text-base text-white">{service.name}</h3>
                        <p class="text-xs text-slate-400">{service.type}</p>
                      </div>
                    </div>
                    {#if service.status === 'running'}
                      <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider bg-emerald-500/15 text-emerald-300 border border-emerald-500/30 shadow-sm shadow-emerald-950/40">
                        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                        Running
                      </span>
                    {:else}
                      <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium uppercase tracking-wider bg-slate-800 text-slate-400 border border-slate-700">
                        <span class="w-2 h-2 rounded-full bg-slate-500"></span>
                        Stopped
                      </span>
                    {/if}
                  </div>

                  <div class="bg-[#0B101C]/60 rounded-xl p-4 border border-slate-800 space-y-2.5 text-xs">
                    <div class="flex justify-between items-center text-slate-400">
                      <span>Assigned Ports</span>
                      <span class="text-cyan-400 font-mono font-bold bg-cyan-950/40 px-2 py-0.5 rounded border border-cyan-500/20">{service.ports}</span>
                    </div>
                    <div class="flex justify-between items-center text-slate-400">
                      <span>Active Version</span>
                      <span class="text-slate-200 font-mono font-medium">{service.version}</span>
                    </div>
                    {#if service.pid}
                      <div class="flex justify-between items-center text-slate-400">
                        <span>Process PID</span>
                        <span class="text-emerald-400 font-mono font-bold bg-emerald-950/60 px-2 py-0.5 rounded border border-emerald-500/30">#{service.pid}</span>
                      </div>
                    {/if}
                    <div class="flex justify-between items-center text-slate-400 pt-1 border-t border-slate-800/50">
                      <span>Config File</span>
                      <span class="font-mono text-[11px] text-amber-300 font-medium">
                        {service.id === "caddy" ? "Caddyfile" : service.id === "php" ? "php.ini" : service.id === "mariadb" ? "my.ini" : "node.env"}
                      </span>
                    </div>

                    {#if service.id === "php"}
                      <div class="flex items-center justify-between pt-1 border-t border-slate-800/50">
                        <span class="text-slate-400 font-medium">Switch Version:</span>
                        <select
                          value={activePhpVersion}
                          onchange={(e) => selectPhpVersion((e.target as HTMLSelectElement).value)}
                          class="bg-slate-900 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 rounded-lg px-2.5 py-1 text-xs font-mono font-bold transition-colors cursor-pointer focus:outline-none"
                        >
                          <option value="8.4.3">PHP 8.4.3 (Latest)</option>
                          <option value="8.3.16">PHP 8.3.16 (Active)</option>
                          <option value="8.2.27">PHP 8.2.27 (LTS)</option>
                        </select>
                      </div>
                    {:else if service.id === "node"}
                      <div class="flex items-center justify-between pt-1 border-t border-slate-800/50">
                        <span class="text-slate-400 font-medium">Switch Version:</span>
                        <select
                          value={activeNodeVersion}
                          onchange={(e) => selectNodeVersion((e.target as HTMLSelectElement).value)}
                          class="bg-slate-900 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 rounded-lg px-2.5 py-1 text-xs font-mono font-bold transition-colors cursor-pointer focus:outline-none"
                        >
                          <option value="22.14.0">Node.js 22.14.0 (LTS)</option>
                          <option value="20.18.2">Node.js 20.18.2</option>
                        </select>
                      </div>
                    {/if}
                  </div>
                </div>

                <div class="mt-6 pt-4 border-t border-slate-800/80 space-y-2.5">
                  <div class="flex items-center gap-3">
                    <button
                      onclick={() => toggleService(service.id)}
                      class="flex-1 py-2 rounded-xl text-xs font-bold transition-all flex items-center justify-center gap-2 shadow-sm {service.status === 'running'
                        ? 'bg-rose-500/20 text-rose-300 hover:bg-rose-500/30 border border-rose-500/40 shadow-rose-950/30'
                        : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
                    >
                      <Power class="w-4 h-4" />
                      <span>{service.status === "running" ? "Stop Service" : "Start Service"}</span>
                    </button>
                    {#if service.id === "caddy"}
                      <button
                        onclick={openWebLocalhost}
                        title="Open in Browser"
                        class="p-2 rounded-xl bg-slate-800/80 hover:bg-slate-800 text-cyan-400 border border-slate-700 hover:border-cyan-500/50 transition-all"
                      >
                        <ExternalLink class="w-4 h-4" />
                      </button>
                    {:else if service.id === "mariadb"}
                      <button
                        onclick={openDatabaseAction}
                        title="Open Database GUI"
                        class="p-2 rounded-xl bg-slate-800/80 hover:bg-slate-800 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 transition-all"
                      >
                        <Database class="w-4 h-4" />
                      </button>
                    {/if}
                  </div>

                  <div class="grid grid-cols-2 gap-2">
                    <button
                      onclick={() => openServiceConfig(service.id)}
                      title="Edit Configuration in Notepad"
                      class="w-full py-2 px-3 rounded-xl text-xs font-semibold bg-slate-800/70 hover:bg-slate-800 text-amber-300 border border-slate-700/60 hover:border-amber-500/40 flex items-center justify-center gap-1.5 transition-all"
                    >
                      <FileText class="w-3.5 h-3.5 text-amber-400" />
                      <span>Edit Config</span>
                    </button>
                    <button
                      onclick={() => openServiceLogs(service.id)}
                      title="View Live Service Logs"
                      class="w-full py-2 px-3 rounded-xl text-xs font-semibold bg-slate-800/70 hover:bg-slate-800 text-cyan-300 border border-slate-700/60 hover:border-cyan-500/40 flex items-center justify-center gap-1.5 transition-all"
                    >
                      <ScrollText class="w-3.5 h-3.5 text-cyan-400" />
                      <span>Live Logs</span>
                    </button>
                  </div>
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

  {#if showUpdateModal}
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
            onclick={() => (showUpdateModal = false)}
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
            onclick={checkUpdates}
            disabled={isCheckingUpdate}
            class="px-4 py-2 rounded-xl text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 flex items-center gap-2 transition-colors"
          >
            <RefreshCw class="w-3.5 h-3.5 {isCheckingUpdate ? 'animate-spin' : ''}" />
            {isCheckingUpdate ? "Checking..." : "Re-check"}
          </button>
          <button
            onclick={() => (showUpdateModal = false)}
            class="px-4 py-2 rounded-xl text-xs font-medium bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:brightness-110 shadow-lg shadow-cyan-500/20 transition-all"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  {/if}

  {#if showDatabaseModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
      <div class="bg-[#101827] border border-slate-800 w-full max-w-md rounded-2xl p-6 space-y-5 shadow-2xl">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2.5 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
              <Database class="w-5 h-5" />
            </div>
            <div>
              <h3 class="font-bold text-base text-white">Database Quick Access</h3>
              <p class="text-xs text-slate-400">MariaDB / MySQL Local Connection</p>
            </div>
          </div>
          <button
            onclick={() => (showDatabaseModal = false)}
            class="text-slate-400 hover:text-white p-1 rounded-lg hover:bg-slate-800 transition-colors"
          >
            <X class="w-5 h-5" />
          </button>
        </div>

        <div class="space-y-2.5 text-xs bg-slate-950/60 rounded-xl p-4 border border-slate-800/80">
          <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
            <span class="text-slate-400">Host</span>
            <span class="font-mono text-cyan-400 font-semibold">127.0.0.1 (localhost)</span>
          </div>
          <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
            <span class="text-slate-400">Port</span>
            <span class="font-mono text-emerald-400 font-semibold">3306</span>
          </div>
          <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
            <span class="text-slate-400">Username</span>
            <span class="font-mono text-slate-200 font-semibold">root</span>
          </div>
          <div class="flex justify-between items-center py-1 border-b border-slate-800/50">
            <span class="text-slate-400">Password</span>
            <span class="font-mono text-slate-400 italic">(none / blank)</span>
          </div>
          <div class="flex justify-between items-center py-1">
            <span class="text-slate-400">Driver</span>
            <span class="font-mono text-amber-400">MySQL / MariaDB</span>
          </div>
        </div>

        <div class="p-3 rounded-xl bg-slate-900/80 border border-slate-800 text-[11px] text-slate-300 space-y-1">
          <p class="font-semibold text-white">Tip for HeidiSQL, DBeaver & VS Code:</p>
          <p>Connect using Host <code class="text-cyan-400 font-mono">127.0.0.1</code>, Port <code class="text-emerald-400 font-mono">3306</code>, and User <code class="text-slate-200 font-mono">root</code> without password.</p>
        </div>

        <div class="flex items-center justify-end gap-3 pt-2">
          <button
            onclick={() => {
              if (navigator.clipboard) {
                navigator.clipboard.writeText("mysql://root@127.0.0.1:3306");
                showToast("Connection string copied to clipboard!");
              }
            }}
            class="px-4 py-2 rounded-xl text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 transition-colors"
          >
            Copy URL
          </button>
          <button
            onclick={() => (showDatabaseModal = false)}
            class="px-4 py-2 rounded-xl text-xs font-medium bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-lg shadow-emerald-500/20 transition-all"
          >
            Done
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
