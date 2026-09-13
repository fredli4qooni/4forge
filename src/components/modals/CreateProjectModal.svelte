<script lang="ts">
  import {
    Check,
    Copy,
    FolderKanban,
    Globe,
    Sparkles,
    Terminal,
    X,
  } from "@lucide/svelte";

  export interface ScaffoldRequest {
    projectName: string;
    template: string;
    version: string;
    domain: string;
    targetPath: string;
    createDatabase: boolean;
    databaseName: string;
    autoVhost: boolean;
    mode: "gui" | "terminal";
    command: string;
  }

  let {
    show = false,
    domainSuffix = "test",
    onClose,
    onRunInTerminal,
    onScaffoldGui,
  }: {
    show: boolean;
    domainSuffix?: string;
    onClose: () => void;
    onRunInTerminal: (command: string, cwd?: string) => void;
    onScaffoldGui: (req: ScaffoldRequest) => void;
  } = $props();

  let activeMode = $state<"gui" | "terminal">("gui");
  let selectedTemplate = $state<"laravel" | "nextjs" | "wordpress">("laravel");
  let projectName = $state("my-laravel-app");
  let selectedVersion = $state("11");
  let autoCreateDb = $state(true);
  let autoRegisterVhost = $state(true);
  let copied = $state(false);

  let cleanSlug = $derived(
    projectName
      .toLowerCase()
      .trim()
      .replace(/[^a-z0-9-_]/g, "-")
      .replace(/--+/g, "-") || "my-app"
  );

  let calculatedDomain = $derived(`${cleanSlug}.${domainSuffix}`);
  let calculatedPath = $derived(`C:\\4forge\\projects\\${cleanSlug}`);
  let calculatedDbName = $derived(cleanSlug.replace(/[^a-z0-9_]/g, "_"));

  let generatedCommand = $derived.by(() => {
    if (selectedTemplate === "laravel") {
      if (selectedVersion === "11") {
        return `composer create-project laravel/laravel:^11.0 ${cleanSlug}`;
      } else if (selectedVersion === "10") {
        return `composer create-project laravel/laravel:^10.0 ${cleanSlug}`;
      }
      return `composer create-project laravel/laravel ${cleanSlug}`;
    } else if (selectedTemplate === "nextjs") {
      return `npx create-next-app@latest ${cleanSlug}`;
    }
    return `composer create-project roots/bedrock ${cleanSlug}`;
  });

  function handleCopy() {
    navigator.clipboard?.writeText(generatedCommand);
    copied = true;
    setTimeout(() => {
      copied = false;
    }, 2000);
  }

  function handleRunTerminal() {
    onRunInTerminal(generatedCommand, "C:\\4forge\\projects");
    onClose();
  }

  function handleGuiSubmit() {
    onScaffoldGui({
      projectName: cleanSlug,
      template: selectedTemplate,
      version: selectedVersion,
      domain: calculatedDomain,
      targetPath: calculatedPath,
      createDatabase: autoCreateDb,
      databaseName: calculatedDbName,
      autoVhost: autoRegisterVhost,
      mode: "gui",
      command: generatedCommand,
    });
    onClose();
  }
</script>

{#if show}
  <div class="fixed inset-0 z-50 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="rounded-2xl border border-slate-200 bg-white w-full max-w-xl p-6 shadow-2xl space-y-5 text-slate-900">
      <div class="flex items-center justify-between pb-3 border-b border-slate-100">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-xl bg-gradient-to-br from-[#B44816] to-[#7C2D12] flex items-center justify-center text-white shadow-xs">
            <Sparkles class="w-4 h-4" />
          </div>
          <div>
            <h3 class="text-sm font-bold text-slate-900 leading-tight">Create New Project</h3>
            <p class="text-xs text-slate-500 mt-0.5">Scaffold and configure your web application from official templates</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="text-slate-400 hover:text-slate-700 p-1.5 rounded-lg hover:bg-slate-100 transition-colors"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="space-y-4">
        <div>
          <span class="block text-xs font-semibold text-slate-700 mb-1.5">Select Framework Template</span>
          <div class="grid grid-cols-3 gap-2.5">
            <button
              type="button"
              onclick={() => (selectedTemplate = "laravel")}
              class="flex flex-col items-start p-3 rounded-xl border text-left transition-all {selectedTemplate === 'laravel'
                ? 'border-[#94380C] bg-amber-50/50 shadow-xs ring-1 ring-[#94380C]'
                : 'border-slate-200 bg-white hover:border-slate-300'}"
            >
              <div class="flex items-center justify-between w-full">
                <span class="font-bold text-xs text-slate-900">Laravel</span>
                <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-amber-100 text-[#94380C]">PHP 8.2+</span>
              </div>
              <p class="text-[11px] text-slate-500 mt-1 leading-snug">The PHP Framework for Web Artisans</p>
            </button>

            <button
              type="button"
              disabled
              class="flex flex-col items-start p-3 rounded-xl border border-slate-200/80 bg-slate-50 text-left opacity-60 cursor-not-allowed"
            >
              <div class="flex items-center justify-between w-full">
                <span class="font-bold text-xs text-slate-700">Next.js</span>
                <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-slate-200 text-slate-600">Soon</span>
              </div>
              <p class="text-[11px] text-slate-400 mt-1 leading-snug">React fullstack framework</p>
            </button>

            <button
              type="button"
              disabled
              class="flex flex-col items-start p-3 rounded-xl border border-slate-200/80 bg-slate-50 text-left opacity-60 cursor-not-allowed"
            >
              <div class="flex items-center justify-between w-full">
                <span class="font-bold text-xs text-slate-700">WordPress</span>
                <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-slate-200 text-slate-600">Soon</span>
              </div>
              <p class="text-[11px] text-slate-400 mt-1 leading-snug">Bedrock modern CMS stack</p>
            </button>
          </div>
        </div>

        <div class="flex items-center gap-1 bg-slate-100 p-1 rounded-xl border border-slate-200/70">
          <button
            type="button"
            onclick={() => (activeMode = "gui")}
            class="flex-1 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center justify-center gap-1.5 {activeMode === 'gui'
              ? 'bg-white text-slate-900 shadow-xs'
              : 'text-slate-600 hover:text-slate-900'}"
          >
            <Sparkles class="w-3.5 h-3.5 text-[#94380C]" />
            <span>Option B: Guided Setup (Auto)</span>
          </button>
          <button
            type="button"
            onclick={() => (activeMode = "terminal")}
            class="flex-1 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center justify-center gap-1.5 {activeMode === 'terminal'
              ? 'bg-white text-slate-900 shadow-xs'
              : 'text-slate-600 hover:text-slate-900'}"
          >
            <Terminal class="w-3.5 h-3.5 text-slate-700" />
            <span>Option A: Terminal Quick Action</span>
          </button>
        </div>

        <div class="space-y-3">
          <div>
            <label for="project-name-input" class="block text-xs font-semibold text-slate-700 mb-1">Project Name</label>
            <input
              id="project-name-input"
              type="text"
              bind:value={projectName}
              placeholder="e.g. my-laravel-app"
              class="w-full px-3 py-2 rounded-lg bg-slate-50 border border-slate-200 text-slate-900 focus:bg-white focus:outline-none focus:border-slate-900 focus:ring-1 focus:ring-slate-900 font-mono text-xs shadow-xs"
            />
          </div>

          <div>
            <span class="block text-xs font-semibold text-slate-700 mb-1">Version</span>
            <div class="flex items-center gap-2">
              <button
                type="button"
                onclick={() => (selectedVersion = "11")}
                class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-all {selectedVersion === '11'
                  ? 'bg-slate-900 text-white border-slate-900 shadow-xs'
                  : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
              >
                Laravel 11.x (Latest)
              </button>
              <button
                type="button"
                onclick={() => (selectedVersion = "10")}
                class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-all {selectedVersion === '10'
                  ? 'bg-slate-900 text-white border-slate-900 shadow-xs'
                  : 'bg-white text-slate-700 border-slate-200 hover:bg-slate-50'}"
              >
                Laravel 10.x (LTS)
              </button>
            </div>
          </div>

          {#if activeMode === "gui"}
            <div class="p-3.5 rounded-xl bg-slate-50 border border-slate-200/80 space-y-2.5 text-xs">
              <div class="flex items-center justify-between text-slate-700 font-medium">
                <span class="flex items-center gap-1.5">
                  <FolderKanban class="w-3.5 h-3.5 text-slate-500" />
                  Target Directory:
                </span>
                <span class="font-mono text-slate-900 truncate max-w-xs">{calculatedPath}</span>
              </div>

              <div class="flex items-center justify-between text-slate-700 font-medium">
                <span class="flex items-center gap-1.5">
                  <Globe class="w-3.5 h-3.5 text-slate-500" />
                  Virtual Host:
                </span>
                <span class="font-mono text-[#94380C] font-semibold">{calculatedDomain}</span>
              </div>

              <div class="pt-2 border-t border-slate-200/60 space-y-2">
                <label class="flex items-center gap-2 cursor-pointer text-slate-800">
                  <input
                    type="checkbox"
                    bind:checked={autoCreateDb}
                    class="rounded text-[#94380C] focus:ring-[#94380C] border-slate-300"
                  />
                  <span>Create database in MariaDB/MySQL: <strong class="font-mono text-slate-900">{calculatedDbName}</strong></span>
                </label>

                <label class="flex items-center gap-2 cursor-pointer text-slate-800">
                  <input
                    type="checkbox"
                    bind:checked={autoRegisterVhost}
                    class="rounded text-[#94380C] focus:ring-[#94380C] border-slate-300"
                  />
                  <span>Auto-register Caddy Virtual Host & SSL (<strong class="font-mono text-slate-900">/public</strong> root)</span>
                </label>
              </div>
            </div>
          {:else}
            <div class="space-y-2">
              <span class="block text-xs font-semibold text-slate-700">Generated Command</span>
              <div class="relative bg-slate-900 rounded-xl p-3.5 font-mono text-xs text-amber-300 border border-slate-800 shadow-inner flex items-center justify-between gap-2">
                <span class="select-all break-all">$ {generatedCommand}</span>
                <button
                  type="button"
                  onclick={handleCopy}
                  class="shrink-0 p-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 hover:text-white transition-colors border border-slate-700"
                  title="Copy command"
                >
                  {#if copied}
                    <Check class="w-3.5 h-3.5 text-emerald-400" />
                  {:else}
                    <Copy class="w-3.5 h-3.5" />
                  {/if}
                </button>
              </div>
              <p class="text-[11px] text-slate-500">
                You can run this command directly in the built-in 4Forge terminal or copy it to your clipboard.
              </p>
            </div>
          {/if}
        </div>
      </div>

      <div class="flex items-center justify-between pt-2 border-t border-slate-100">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 rounded-lg text-slate-600 hover:text-slate-900 text-xs font-medium hover:bg-slate-100 transition-colors"
        >
          Cancel
        </button>

        {#if activeMode === "gui"}
          <button
            type="button"
            onclick={handleGuiSubmit}
            class="px-4 py-2 rounded-lg bg-[#94380C] hover:bg-[#7C2D12] text-white font-semibold text-xs transition-all shadow-xs flex items-center gap-1.5"
          >
            <Sparkles class="w-3.5 h-3.5" />
            <span>Create & Install Project</span>
          </button>
        {:else}
          <div class="flex items-center gap-2">
            <button
              type="button"
              onclick={handleCopy}
              class="px-3 py-2 rounded-lg bg-white border border-slate-200 text-slate-700 hover:bg-slate-50 font-medium text-xs transition-colors flex items-center gap-1.5 shadow-xs"
            >
              <Copy class="w-3.5 h-3.5" />
              <span>{copied ? "Copied!" : "Copy"}</span>
            </button>
            <button
              type="button"
              onclick={handleRunTerminal}
              class="px-4 py-2 rounded-lg bg-slate-900 hover:bg-slate-800 text-white font-semibold text-xs transition-all shadow-xs flex items-center gap-1.5"
            >
              <Terminal class="w-3.5 h-3.5" />
              <span>Run in 4Forge Terminal</span>
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
