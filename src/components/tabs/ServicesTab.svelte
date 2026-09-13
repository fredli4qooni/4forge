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
      <h2 class="text-lg font-bold text-slate-900">Core Services Manager</h2>
      <p class="text-xs text-slate-500 mt-0.5">Lifecycle control with Windows Job Objects enforcement.</p>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each services as service}
      <div class="rounded-xl border border-slate-200 bg-white p-5 flex flex-col justify-between shadow-xs hover:border-slate-300 transition-all">
        <div>
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-3">
              <div class="p-2.5 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
                <service.icon class="w-5 h-5" />
              </div>
              <div>
                <h3 class="font-semibold text-sm text-slate-900">{service.name}</h3>
                <p class="text-xs text-slate-500">{service.type}</p>
              </div>
            </div>
            {#if service.status === 'running'}
              <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-medium bg-emerald-50 text-emerald-700 border border-emerald-200">
                <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                Running
              </span>
            {:else}
              <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-medium bg-slate-100 text-slate-600 border border-slate-200">
                <span class="w-1.5 h-1.5 rounded-full bg-slate-400"></span>
                Stopped
              </span>
            {/if}
          </div>

          <div class="bg-slate-50 rounded-lg p-3.5 border border-slate-200 space-y-2 text-xs">
            <div class="flex justify-between items-center text-slate-600">
              <span>Assigned Ports</span>
              <span class="text-slate-800 font-mono font-medium bg-white px-2 py-0.5 rounded border border-slate-200">{service.ports}</span>
            </div>
            <div class="flex justify-between items-center text-slate-600">
              <span>Active Version</span>
              <span class="text-slate-800 font-mono font-medium">{service.version}</span>
            </div>
            {#if service.pid}
              <div class="flex justify-between items-center text-slate-600">
                <span>Process PID</span>
                <span class="text-slate-800 font-mono font-medium bg-white px-2 py-0.5 rounded border border-slate-200">#{service.pid}</span>
              </div>
            {/if}
            <div class="flex justify-between items-center text-slate-600 pt-1 border-t border-slate-200/80">
              <span>Config File</span>
              <span class="font-mono text-[11px] text-slate-800 font-medium">
                {service.id === "caddy" ? "Caddyfile" : service.id === "php" ? "php.ini" : service.id === "mariadb" ? "my.ini" : service.id === "postgresql" ? "postgresql.conf" : service.id === "redis" ? "redis.conf" : "node.env"}
              </span>
            </div>

            {#if service.id === "php"}
              <div class="flex items-center justify-between pt-1 border-t border-slate-200/80">
                <span class="text-slate-600 font-medium">Switch Version:</span>
                <select
                  value={activePhpVersion}
                  onchange={(e) => onSelectPhpVersion((e.target as HTMLSelectElement).value)}
                  class="bg-white text-slate-800 border border-slate-200 hover:border-slate-400 rounded-md px-2 py-0.5 text-xs font-mono font-medium transition-colors cursor-pointer focus:outline-none focus:border-slate-900"
                >
                  <option value="8.4.3">PHP 8.4.3 (Latest)</option>
                  <option value="8.3.16">PHP 8.3.16 (Active)</option>
                  <option value="8.2.27">PHP 8.2.27 (LTS)</option>
                </select>
              </div>
            {:else if service.id === "node"}
              <div class="flex items-center justify-between pt-1 border-t border-slate-200/80">
                <span class="text-slate-600 font-medium">Switch Version:</span>
                <select
                  value={activeNodeVersion}
                  onchange={(e) => onSelectNodeVersion((e.target as HTMLSelectElement).value)}
                  class="bg-white text-slate-800 border border-slate-200 hover:border-slate-400 rounded-md px-2 py-0.5 text-xs font-mono font-medium transition-colors cursor-pointer focus:outline-none focus:border-slate-900"
                >
                  <option value="22.14.0">Node.js 22.14.0 (LTS)</option>
                  <option value="20.18.2">Node.js 20.18.2</option>
                </select>
              </div>
            {/if}
          </div>
        </div>

        <div class="mt-5 pt-3.5 border-t border-slate-100 space-y-2">
          <div class="flex items-center gap-2">
            <button
              onclick={() => onToggleService(service.id)}
              class="flex-1 py-1.5 px-3 rounded-lg text-xs font-medium transition-all flex items-center justify-center gap-1.5 shadow-xs {service.status === 'running'
                ? 'bg-rose-50 text-rose-700 hover:bg-rose-100 border border-rose-200'
                : 'bg-slate-900 text-white hover:bg-slate-800'}"
            >
              <Power class="w-3.5 h-3.5" />
              <span>{service.status === "running" ? "Stop Service" : "Start Service"}</span>
            </button>
            {#if service.id === "caddy"}
              <button
                onclick={onOpenWeb}
                title="Open in Browser"
                class="p-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs transition-all"
              >
                <ExternalLink class="w-3.5 h-3.5" />
              </button>
            {:else if service.id === "mariadb"}
              <button
                onclick={onOpenDatabase}
                title="Open Database GUI"
                class="p-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs transition-all"
              >
                <Database class="w-3.5 h-3.5" />
              </button>
            {/if}
          </div>

          <div class="grid grid-cols-2 gap-2">
            <button
              onclick={() => onOpenConfig(service.id)}
              title="Edit Configuration in Notepad"
              class="w-full py-1.5 px-2.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center justify-center gap-1.5 transition-all"
            >
              <FileText class="w-3 h-3 text-slate-500" />
              <span>Edit Config</span>
            </button>
            <button
              onclick={() => onOpenLogs(service.id)}
              title="View Live Service Logs"
              class="w-full py-1.5 px-2.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 shadow-xs flex items-center justify-center gap-1.5 transition-all"
            >
              <ScrollText class="w-3 h-3 text-slate-500" />
              <span>Live Logs</span>
            </button>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>
