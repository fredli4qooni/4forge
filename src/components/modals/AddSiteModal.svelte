<script lang="ts">
  import { CheckCircle2, Globe, Sparkles, X } from "@lucide/svelte";
  import type { DetectedProject } from "../../types";

  let {
    show,
    path,
    domain,
    backendType,
    target,
    domainSuffix,
    detectedProject,
    onClose,
    onPathChange,
    onDomainChange,
    onSuffixChange,
    onTypeChange,
    onTargetChange,
    onSubmit,
  }: {
    show: boolean;
    path: string;
    domain: string;
    backendType: string;
    target: string;
    domainSuffix: string;
    detectedProject: DetectedProject | null;
    onClose: () => void;
    onPathChange: (path: string) => void;
    onDomainChange: (domain: string) => void;
    onSuffixChange: (suffix: string) => void;
    onTypeChange: (type: string) => void;
    onTargetChange: (target: string) => void;
    onSubmit: () => void;
  } = $props();
</script>

{#if show}
  <div class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4">
    <div class="rounded-2xl border border-slate-800 bg-[#121929] w-full max-w-lg p-6 shadow-2xl space-y-5">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
            <Globe class="w-5 h-5" />
          </div>
          <div>
            <h3 class="text-base font-bold text-white">Add Project (Zero-Config)</h3>
            <p class="text-xs text-slate-400">Automatic framework detection and reverse proxy SSL setup</p>
          </div>
        </div>
        <button onclick={onClose} class="text-slate-400 hover:text-white p-1 rounded-lg hover:bg-slate-800 transition-colors">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="space-y-4 text-xs">
        <div>
          <div class="flex justify-between items-center mb-1">
            <label for="site-path" class="text-slate-300 font-medium">Project Folder Path</label>
            <button
              type="button"
              onclick={() => onPathChange("C:\\4forge\\projects\\new-laravel-app")}
              class="text-cyan-400 hover:text-cyan-300 text-[11px] underline"
            >
              Sample Folder
            </button>
          </div>
          <input
            id="site-path"
            type="text"
            value={path}
            oninput={(e) => onPathChange((e.target as HTMLInputElement).value)}
            placeholder="C:\4forge\projects\my-app"
            class="w-full px-3.5 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-white focus:outline-none focus:border-cyan-500 font-mono text-xs shadow-inner"
          />
        </div>

        {#if detectedProject}
          <div class="rounded-xl border border-emerald-500/30 bg-emerald-950/20 p-3.5 space-y-2 text-xs text-slate-300 shadow-md shadow-emerald-950/20">
            <div class="flex items-center justify-between">
              <span class="text-emerald-300 font-bold flex items-center gap-1.5">
                <Sparkles class="w-4 h-4 text-emerald-400" />
                Detected: {detectedProject.framework} ({detectedProject.runtime})
              </span>
              <span class="text-emerald-400 font-mono text-[10px] bg-emerald-900/40 px-2 py-0.5 rounded border border-emerald-500/30">Auto-Configured</span>
            </div>
            <div class="text-[11px] text-slate-400 space-y-1 pt-1.5 border-t border-emerald-500/20">
              <div class="flex justify-between">
                <span>Web Root:</span>
                <span class="font-mono text-emerald-200 truncate max-w-xs">{detectedProject.web_root}</span>
              </div>
              <div class="flex justify-between">
                <span>Backend Routing:</span>
                <span class="font-mono text-cyan-300">{detectedProject.backend_type.toUpperCase()} ({detectedProject.target})</span>
              </div>
            </div>
          </div>
        {/if}

        <div class="grid grid-cols-3 gap-3">
          <div class="col-span-2">
            <label for="site-domain" class="block text-slate-300 font-medium mb-1">Local Domain</label>
            <input
              id="site-domain"
              type="text"
              value={domain}
              oninput={(e) => onDomainChange((e.target as HTMLInputElement).value)}
              placeholder="e.g. my-app.test"
              class="w-full px-3 py-2 rounded-xl bg-slate-900 border border-slate-700 text-white focus:outline-none focus:border-cyan-500 font-mono text-xs"
            />
          </div>
          <div>
            <label for="domain-suffix" class="block text-slate-300 font-medium mb-1">Domain Suffix</label>
            <select
              id="domain-suffix"
              value={domainSuffix}
              onchange={(e) => onSuffixChange((e.target as HTMLSelectElement).value)}
              class="w-full px-2.5 py-2 rounded-xl bg-slate-900 border border-slate-700 text-cyan-400 font-mono font-bold focus:outline-none focus:border-cyan-500 text-xs"
            >
              <option value="test">.test</option>
              <option value="local">.local</option>
              <option value="localhost">.localhost</option>
            </select>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-3 pt-1">
          <div>
            <label for="site-type" class="block text-slate-400 text-[11px] mb-1">Backend Type</label>
            <select
              id="site-type"
              value={backendType}
              onchange={(e) => onTypeChange((e.target as HTMLSelectElement).value)}
              class="w-full px-3 py-1.5 rounded-xl bg-slate-900/80 border border-slate-700 text-slate-200 focus:outline-none focus:border-cyan-500 text-xs"
            >
              <option value="fastcgi">PHP FastCGI (php-cgi)</option>
              <option value="proxy">Reverse Proxy (Node/Python)</option>
              <option value="static">Static HTML</option>
            </select>
          </div>
          <div>
            <label for="site-target" class="block text-slate-400 text-[11px] mb-1">Upstream Target</label>
            <input
              id="site-target"
              type="text"
              value={target}
              oninput={(e) => onTargetChange((e.target as HTMLInputElement).value)}
              placeholder="127.0.0.1:9000"
              class="w-full px-3 py-1.5 rounded-xl bg-slate-900/80 border border-slate-700 text-slate-200 focus:outline-none focus:border-cyan-500 font-mono text-xs"
            />
          </div>
        </div>
      </div>

      <div class="flex items-center justify-end gap-3 pt-3 border-t border-slate-800">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 rounded-xl text-xs text-slate-400 hover:text-white transition-colors"
        >
          Cancel
        </button>
        <button
          type="button"
          onclick={onSubmit}
          class="px-5 py-2.5 rounded-xl text-xs font-bold bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:brightness-110 shadow-lg shadow-cyan-500/25 transition-all flex items-center gap-1.5"
        >
          <CheckCircle2 class="w-4 h-4" />
          <span>Create Virtual Host</span>
        </button>
      </div>
    </div>
  </div>
{/if}
