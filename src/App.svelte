<script lang="ts">
  import {
    Activity,
    CheckCircle2,
    Database,
    FolderKanban,
    Globe,
    Layers,
    Play,
    Power,
    RefreshCw,
    Server,
    Settings,
    ShieldCheck,
    Square,
    Terminal,
  } from "lucide-svelte";

  // Svelte 5 Runes
  let activeTab = $state<"dashboard" | "services" | "sites" | "runtimes" | "logs">("dashboard");
  let allRunning = $state(false);

  let services = $state([
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
      status: "ready",
      icon: Layers,
    },
  ]);

  let sites = $state([
    {
      domain: "laravel-app.test",
      runtime: "PHP 8.3",
      ssl: true,
      path: "C:\\projects\\laravel-app",
      status: "active",
    },
    {
      domain: "dashboard-api.test",
      runtime: "Node 22",
      ssl: true,
      path: "C:\\projects\\dashboard-api",
      status: "active",
    },
  ]);

  function toggleAll() {
    allRunning = !allRunning;
    services = services.map((s) => ({
      ...s,
      status: allRunning ? "running" : "stopped",
    }));
  }

  function toggleService(id: string) {
    services = services.map((s) => {
      if (s.id === id) {
        const nextStatus = s.status === "running" ? "stopped" : "running";
        return { ...s, status: nextStatus };
      }
      return s;
    });
    allRunning = services.some((s) => s.status === "running");
  }
</script>

<div class="flex h-screen w-screen bg-[#0B0F17] text-slate-100 font-sans antialiased overflow-hidden">
  <!-- Sidebar Navigation -->
  <aside class="w-64 border-r border-slate-800/80 bg-[#0E1524]/60 backdrop-blur-xl flex flex-col shrink-0">
    <!-- Brand Logo -->
    <div class="h-16 px-6 flex items-center gap-3 border-b border-slate-800/70">
      <div class="h-9 w-9 rounded-xl bg-gradient-to-tr from-cyan-500 to-blue-600 flex items-center justify-center shadow-lg shadow-cyan-500/25">
        <Server class="w-5 h-5 text-white" />
      </div>
      <div>
        <div class="flex items-center gap-2">
          <span class="font-bold text-lg tracking-tight text-white">4Forge</span>
          <span class="text-[10px] font-semibold uppercase px-1.5 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">v0.1 Alpha</span>
        </div>
        <p class="text-[11px] text-slate-400">Local Dev Manager</p>
      </div>
    </div>

    <!-- Nav Items -->
    <nav class="flex-1 px-3 py-4 space-y-1">
      <button
        onclick={() => (activeTab = "dashboard")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all {activeTab === 'dashboard'
          ? 'bg-cyan-500/15 text-cyan-400 border border-cyan-500/30'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Activity class="w-4 h-4" />
        Dashboard
      </button>

      <button
        onclick={() => (activeTab = "services")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all {activeTab === 'services'
          ? 'bg-cyan-500/15 text-cyan-400 border border-cyan-500/30'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Server class="w-4 h-4" />
        Services
      </button>

      <button
        onclick={() => (activeTab = "sites")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all {activeTab === 'sites'
          ? 'bg-cyan-500/15 text-cyan-400 border border-cyan-500/30'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <FolderKanban class="w-4 h-4" />
        Projects & Sites
      </button>

      <button
        onclick={() => (activeTab = "runtimes")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all {activeTab === 'runtimes'
          ? 'bg-cyan-500/15 text-cyan-400 border border-cyan-500/30'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Layers class="w-4 h-4" />
        Runtimes (mise)
      </button>

      <button
        onclick={() => (activeTab = "logs")}
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all {activeTab === 'logs'
          ? 'bg-cyan-500/15 text-cyan-400 border border-cyan-500/30'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'}"
      >
        <Terminal class="w-4 h-4" />
        Live Logs
      </button>
    </nav>

    <!-- Bottom Metadata & Status -->
    <div class="p-4 border-t border-slate-800/70 bg-[#0B101C]/50 text-xs">
      <div class="flex items-center justify-between text-slate-400 mb-1">
        <span>Memory Shell</span>
        <span class="text-emerald-400 font-mono">~42 MB</span>
      </div>
      <div class="flex items-center justify-between text-slate-400">
        <span>SSL Root CA</span>
        <span class="text-cyan-400 flex items-center gap-1">
          <ShieldCheck class="w-3.5 h-3.5" /> Trusted
        </span>
      </div>
    </div>
  </aside>

  <!-- Main Content Area -->
  <main class="flex-1 flex flex-col overflow-hidden bg-gradient-to-b from-[#0F172A]/40 to-[#0B0F17]">
    <!-- Top Action Bar -->
    <header class="h-16 border-b border-slate-800/70 px-8 flex items-center justify-between shrink-0 bg-[#0E1524]/40 backdrop-blur-md">
      <div>
        <h1 class="text-base font-semibold text-white">Local Development Environment</h1>
        <p class="text-xs text-slate-400">Windows Platform • Apache-2.0 Pure Open Source</p>
      </div>

      <div class="flex items-center gap-3">
        <button
          onclick={toggleAll}
          class="flex items-center gap-2 px-4 py-2 rounded-lg font-medium text-xs transition-all shadow-md {allRunning
            ? 'bg-rose-500/20 text-rose-300 border border-rose-500/30 hover:bg-rose-500/30'
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
          title="Reload Services"
          class="p-2 rounded-lg text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 border border-slate-800 transition-colors"
        >
          <RefreshCw class="w-4 h-4" />
        </button>
      </div>
    </header>

    <!-- Scrollable Workspace View -->
    <div class="flex-1 overflow-y-auto p-8 space-y-8">
      <!-- Overview Banner -->
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
            <h2 class="text-xl font-bold text-white tracking-tight">Polyglot Development Stack Ready</h2>
            <p class="text-xs text-slate-400 mt-1 max-w-xl">
              Caddy dynamic reverse proxy handles local HTTPS certificates natively. Swap PHP versions, run Node services, and manage MariaDB databases from a single unified cockpit.
            </p>
          </div>

          <div class="flex items-center gap-4">
            <div class="text-right">
              <p class="text-xs text-slate-400">Managed Sites</p>
              <p class="text-2xl font-bold text-cyan-400 font-mono">{sites.length}</p>
            </div>
            <div class="h-10 w-px bg-slate-800"></div>
            <div class="text-right">
              <p class="text-xs text-slate-400">Runtimes</p>
              <p class="text-2xl font-bold text-emerald-400 font-mono">4</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Services Grid -->
      <div>
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-sm font-semibold text-slate-200 uppercase tracking-wider">Core Services</h3>
          <span class="text-xs text-slate-400">Windows Job Object Guard Active</span>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {#each services as service}
            <div class="group relative rounded-xl border border-slate-800/90 bg-[#121929]/70 hover:border-slate-700/80 transition-all p-5 flex flex-col justify-between">
              <div>
                <div class="flex items-center justify-between mb-3">
                  <div class="p-2.5 rounded-lg bg-slate-800/70 text-cyan-400 border border-slate-700/50">
                    <service.icon class="w-5 h-5" />
                  </div>
                  <span
                    class="px-2 py-0.5 rounded-full text-[11px] font-medium capitalize border {service.status === 'running'
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
                    <span>Version:</span>
                    <span class="text-slate-200 font-mono text-[11px]">{service.version}</span>
                  </div>
                  <div class="flex justify-between">
                    <span>Ports:</span>
                    <span class="text-cyan-400 font-mono text-[11px]">{service.ports}</span>
                  </div>
                </div>
              </div>

              <div class="mt-4 pt-3">
                <button
                  onclick={() => toggleService(service.id)}
                  class="w-full py-1.5 px-3 rounded-lg text-xs font-medium transition-colors flex items-center justify-center gap-1.5 {service.status === 'running'
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

      <!-- Sites Preview Table -->
      <div>
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-sm font-semibold text-slate-200 uppercase tracking-wider">Virtual Hosts & Projects</h3>
          <button class="text-xs text-cyan-400 hover:underline font-medium">+ Add New Project</button>
        </div>

        <div class="rounded-xl border border-slate-800/90 bg-[#121929]/50 overflow-hidden">
          <table class="w-full text-left text-xs">
            <thead class="bg-slate-900/70 text-slate-400 uppercase font-semibold border-b border-slate-800">
              <tr>
                <th class="px-5 py-3">Domain</th>
                <th class="px-5 py-3">Runtime</th>
                <th class="px-5 py-3">SSL</th>
                <th class="px-5 py-3">Path</th>
                <th class="px-5 py-3 text-right">Action</th>
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
                  <td class="px-5 py-3.5 text-right">
                    <button class="text-xs text-slate-400 hover:text-white px-2 py-1 rounded bg-slate-800/80 border border-slate-700">Open</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </main>
</div>
