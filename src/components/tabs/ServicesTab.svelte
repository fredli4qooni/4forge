<script lang="ts">
  import {
    Database,
    ExternalLink,
    FileText,
    Power,
    ScrollText,
  } from "@lucide/svelte";
  import type { ServiceItem } from "../../types";

  let {
    services,
    activePhpVersion,
    activeNodeVersion,
    onToggleService,
    onOpenWeb,
    onOpenDatabase,
    onOpenConfig,
    onOpenLogs,
    onSelectPhpVersion,
    onSelectNodeVersion,
  }: {
    services: ServiceItem[];
    activePhpVersion: string;
    activeNodeVersion: string;
    onToggleService: (id: string) => void;
    onOpenWeb: () => void;
    onOpenDatabase: () => void;
    onOpenConfig: (id: string) => void;
    onOpenLogs: (id: string) => void;
    onSelectPhpVersion: (v: string) => void;
    onSelectNodeVersion: (v: string) => void;
  } = $props();
</script>

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
                  onchange={(e) => onSelectPhpVersion((e.target as HTMLSelectElement).value)}
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
                  onchange={(e) => onSelectNodeVersion((e.target as HTMLSelectElement).value)}
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
              onclick={() => onToggleService(service.id)}
              class="flex-1 py-2 rounded-xl text-xs font-bold transition-all flex items-center justify-center gap-2 shadow-sm {service.status === 'running'
                ? 'bg-rose-500/20 text-rose-300 hover:bg-rose-500/30 border border-rose-500/40 shadow-rose-950/30'
                : 'bg-gradient-to-r from-emerald-500 to-teal-600 text-white hover:brightness-110 shadow-emerald-500/20'}"
            >
              <Power class="w-4 h-4" />
              <span>{service.status === "running" ? "Stop Service" : "Start Service"}</span>
            </button>
            {#if service.id === "caddy"}
              <button
                onclick={onOpenWeb}
                title="Open in Browser"
                class="p-2 rounded-xl bg-slate-800/80 hover:bg-slate-800 text-cyan-400 border border-slate-700 hover:border-cyan-500/50 transition-all"
              >
                <ExternalLink class="w-4 h-4" />
              </button>
            {:else if service.id === "mariadb"}
              <button
                onclick={onOpenDatabase}
                title="Open Database GUI"
                class="p-2 rounded-xl bg-slate-800/80 hover:bg-slate-800 text-emerald-400 border border-slate-700 hover:border-emerald-500/50 transition-all"
              >
                <Database class="w-4 h-4" />
              </button>
            {/if}
          </div>

          <div class="grid grid-cols-2 gap-2">
            <button
              onclick={() => onOpenConfig(service.id)}
              title="Edit Configuration in Notepad"
              class="w-full py-2 px-3 rounded-xl text-xs font-semibold bg-slate-800/70 hover:bg-slate-800 text-amber-300 border border-slate-700/60 hover:border-amber-500/40 flex items-center justify-center gap-1.5 transition-all"
            >
              <FileText class="w-3.5 h-3.5 text-amber-400" />
              <span>Edit Config</span>
            </button>
            <button
              onclick={() => onOpenLogs(service.id)}
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
