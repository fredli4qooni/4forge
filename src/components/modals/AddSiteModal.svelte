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
  <div class="fixed inset-0 z-50 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="rounded-xl border border-slate-200 bg-white w-full max-w-lg p-6 shadow-xl space-y-5 text-slate-900">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-slate-100 text-slate-800 border border-slate-200">
            <Globe class="w-5 h-5" />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-slate-900">Add Project (Zero-Config)</h3>
            <p class="text-xs text-slate-500">Automatic framework detection and reverse proxy SSL setup</p>
          </div>
        </div>
        <button onclick={onClose} class="text-slate-400 hover:text-slate-700 p-1 rounded-md hover:bg-slate-100 transition-colors">
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="space-y-3.5 text-xs">
        <div>
          <label for="site-path" class="block text-slate-700 font-medium mb-1">Project Folder Path</label>
          <input
            id="site-path"
            type="text"
            value={path}
            oninput={(e) => onPathChange((e.target as HTMLInputElement).value)}
            placeholder="C:\4forge\projects\my-app"
            class="w-full px-3 py-2 rounded-lg bg-slate-50 border border-slate-200 text-slate-900 focus:bg-white focus:outline-none focus:border-slate-900 focus:ring-1 focus:ring-slate-900 font-mono text-xs shadow-xs"
          />
        </div>

        {#if detectedProject}
          <div class="rounded-lg border border-emerald-200 bg-emerald-50 p-3 space-y-2 text-xs text-emerald-900 shadow-xs">
            <div class="flex items-center justify-between">
              <span class="text-emerald-800 font-semibold flex items-center gap-1.5">
                <Sparkles class="w-3.5 h-3.5 text-emerald-600" />
                Detected: {detectedProject.framework} ({detectedProject.runtime})
              </span>
              <span class="text-emerald-700 font-mono text-[10px] bg-white px-2 py-0.5 rounded border border-emerald-200">Auto-Configured</span>
            </div>
            <div class="text-[11px] text-emerald-800 space-y-1 pt-1.5 border-t border-emerald-200/60">
              <div class="flex justify-between">
                <span>Web Root:</span>
                <span class="font-mono text-emerald-900 truncate max-w-xs">{detectedProject.web_root}</span>
              </div>
              <div class="flex justify-between">
                <span>Backend Routing:</span>
                <span class="font-mono text-emerald-900">{detectedProject.backend_type.toUpperCase()} ({detectedProject.target})</span>
              </div>
            </div>
          </div>
        {/if}

        <div class="grid grid-cols-3 gap-3">
          <div class="col-span-2">
            <label for="site-domain" class="block text-slate-700 font-medium mb-1">Local Domain</label>
            <input
              id="site-domain"
              type="text"
              value={domain}
              oninput={(e) => onDomainChange((e.target as HTMLInputElement).value)}
              placeholder="e.g. my-app.test"
              class="w-full px-3 py-2 rounded-lg bg-slate-50 border border-slate-200 text-slate-900 focus:bg-white focus:outline-none focus:border-slate-900 focus:ring-1 focus:ring-slate-900 font-mono text-xs shadow-xs"
            />
          </div>
          <div>
            <label for="domain-suffix" class="block text-slate-700 font-medium mb-1">Domain Suffix</label>
            <select
              id="domain-suffix"
              value={domainSuffix}
              onchange={(e) => onSuffixChange((e.target as HTMLSelectElement).value)}
              class="w-full px-2.5 py-2 rounded-lg bg-slate-50 border border-slate-200 text-slate-800 font-mono font-medium focus:bg-white focus:outline-none focus:border-slate-900 text-xs shadow-xs"
            >
              <option value="test">.test</option>
              <option value="local">.local</option>
              <option value="localhost">.localhost</option>
            </select>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-3 pt-0.5">
          <div>
            <label for="site-type" class="block text-slate-500 text-[11px] mb-1">Backend Type</label>
            <select
              id="site-type"
              value={backendType}
              onchange={(e) => onTypeChange((e.target as HTMLSelectElement).value)}
              class="w-full px-2.5 py-1.5 rounded-lg bg-slate-50 border border-slate-200 text-slate-800 focus:bg-white focus:outline-none focus:border-slate-900 text-xs shadow-xs"
            >
              <option value="fastcgi">PHP FastCGI (php-cgi)</option>
              <option value="proxy">Reverse Proxy (Node/Python)</option>
              <option value="static">Static HTML</option>
            </select>
          </div>
          <div>
            <label for="site-target" class="block text-slate-500 text-[11px] mb-1">Upstream Target</label>
            <input
              id="site-target"
              type="text"
              value={target}
              oninput={(e) => onTargetChange((e.target as HTMLInputElement).value)}
              placeholder="127.0.0.1:9000"
              class="w-full px-2.5 py-1.5 rounded-lg bg-slate-50 border border-slate-200 text-slate-900 focus:bg-white focus:outline-none focus:border-slate-900 font-mono text-xs shadow-xs"
            />
          </div>
        </div>
      </div>

      <div class="flex items-center justify-end gap-2.5 pt-3 border-t border-slate-100">
        <button
          type="button"
          onclick={onClose}
          class="px-3.5 py-1.5 rounded-lg text-xs font-medium text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors"
        >
          Cancel
        </button>
        <button
          type="button"
          onclick={onSubmit}
          class="px-4 py-1.5 rounded-lg text-xs font-semibold bg-slate-900 hover:bg-slate-800 text-white shadow-xs transition-all flex items-center gap-1.5"
        >
          <CheckCircle2 class="w-3.5 h-3.5" />
          <span>Create Virtual Host</span>
        </button>
      </div>
    </div>
  </div>
{/if}
