<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";
  import {
    ExternalLink,
    FolderKanban,
    RefreshCw,
    RotateCcw,
    Terminal as TerminalIcon,
    Trash2,
  } from "@lucide/svelte";
  import { listen } from "@tauri-apps/api/event";
  import {
    spawnTerminal,
    writeTerminal,
    resizeTerminal,
    killTerminal,
  } from "../../lib/actions";

  let {
    initialCwd = "C:\\4forge\\projects",
    onOpenExternal,
  }: {
    initialCwd?: string;
    onOpenExternal?: () => void;
  } = $props();

  let terminalContainer = $state<HTMLDivElement | null>(null);
  let term: Terminal | null = null;
  let fitAddon: FitAddon | null = null;
  let sessionId = $state("");
  let currentCwd = $state("");
  let isSpawning = $state(false);
  let unlistenFn: (() => void) | null = null;
  let resizeObserver: ResizeObserver | null = null;

  async function startTerminalSession(cwdToUse?: string) {
    if (sessionId) {
      await killTerminal(sessionId);
      sessionId = "";
    }
    if (term) {
      term.clear();
      term.reset();
    }
    isSpawning = true;
    const path = cwdToUse || currentCwd || initialCwd || "C:\\4forge\\projects";
    currentCwd = path;
    const cols = term ? term.cols : 80;
    const rows = term ? term.rows : 24;
    try {
      const id = await spawnTerminal(path, cols, rows);
      sessionId = id;
    } catch (err: any) {
      if (term) {
        term.writeln(`\x1b[31mFailed to start 4Forge terminal: ${err?.message || err}\x1b[0m`);
      }
    } finally {
      isSpawning = false;
    }
  }

  function sendQuickCommand(cmd: string) {
    if (sessionId) {
      writeTerminal(sessionId, `${cmd}\r`);
    }
  }

  function clearTerminal() {
    if (term) {
      term.clear();
      if (sessionId) {
        writeTerminal(sessionId, "Clear-Host\r");
      }
    }
  }

  onMount(async () => {
    if (!terminalContainer) return;

    term = new Terminal({
      cursorBlink: true,
      cursorStyle: "bar",
      fontFamily: "Consolas, 'Cascadia Code', 'Fira Code', monospace",
      fontSize: 13,
      lineHeight: 1.25,
      scrollback: 5000,
      theme: {
        background: "#18181B",
        foreground: "#F4F4F5",
        cursor: "#E4E4E7",
        selectionBackground: "#3F3F46",
        black: "#27272A",
        red: "#F87171",
        green: "#4ADE80",
        yellow: "#FBBF24",
        blue: "#60A5FA",
        magenta: "#C084FC",
        cyan: "#38BDF8",
        white: "#F4F4F5",
        brightBlack: "#52525B",
        brightRed: "#EF4444",
        brightGreen: "#22C55E",
        brightYellow: "#EAB308",
        brightBlue: "#3B82F6",
        brightMagenta: "#A855F7",
        brightCyan: "#06B6D4",
        brightWhite: "#FFFFFF",
      },
    });

    fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(terminalContainer);
    fitAddon.fit();

    try {
      unlistenFn = await listen<string>("terminal-data", (event) => {
        term?.write(event.payload);
      });
    } catch (e) {}

    term.onData((data) => {
      if (sessionId) {
        writeTerminal(sessionId, data);
      }
    });

    currentCwd = initialCwd || "C:\\4forge\\projects";
    await startTerminalSession(currentCwd);

    resizeObserver = new ResizeObserver(() => {
      if (fitAddon && term && sessionId) {
        fitAddon.fit();
        resizeTerminal(sessionId, term.cols, term.rows);
      }
    });
    resizeObserver.observe(terminalContainer);
  });

  onDestroy(() => {
    if (resizeObserver) {
      resizeObserver.disconnect();
    }
    if (unlistenFn) {
      unlistenFn();
    }
    if (sessionId) {
      killTerminal(sessionId);
    }
    if (term) {
      term.dispose();
    }
  });
</script>

<div class="space-y-3 flex flex-col h-full">
  <div class="flex items-center justify-between gap-4 shrink-0">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-lg bg-slate-900 text-white shadow-xs">
        <TerminalIcon class="w-4 h-4" />
      </div>
      <div>
        <h2 class="text-base font-bold text-slate-900 leading-tight">4Forge Dev Shell</h2>
        <div class="flex items-center gap-1.5 text-xs text-slate-500 font-mono mt-0.5">
          <FolderKanban class="w-3 h-3 text-slate-400" />
          <span class="truncate max-w-md">{currentCwd || initialCwd}</span>
        </div>
      </div>
    </div>

    <div class="flex items-center gap-1.5 flex-wrap">
      <div class="hidden sm:flex items-center gap-1 bg-slate-100 p-0.5 rounded-lg border border-slate-200/80 mr-1">
        <button
          onclick={() => sendQuickCommand("php -v")}
          class="px-2 py-1 rounded text-[11px] font-mono font-medium text-slate-700 hover:text-slate-900 hover:bg-white transition-colors"
        >
          php -v
        </button>
        <button
          onclick={() => sendQuickCommand("node -v")}
          class="px-2 py-1 rounded text-[11px] font-mono font-medium text-slate-700 hover:text-slate-900 hover:bg-white transition-colors"
        >
          node -v
        </button>
        <button
          onclick={() => sendQuickCommand("composer --version")}
          class="px-2 py-1 rounded text-[11px] font-mono font-medium text-slate-700 hover:text-slate-900 hover:bg-white transition-colors"
        >
          composer
        </button>
        <button
          onclick={() => sendQuickCommand("Get-Command php, node, mysql, redis-cli -ErrorAction SilentlyContinue | Format-Table Name, Source")}
          class="px-2 py-1 rounded text-[11px] font-mono font-medium text-slate-700 hover:text-slate-900 hover:bg-white transition-colors"
        >
          paths
        </button>
      </div>

      <button
        onclick={clearTerminal}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium shadow-xs transition-colors"
        title="Clear terminal buffer"
      >
        <Trash2 class="w-3.5 h-3.5 text-slate-500" />
        <span>Clear</span>
      </button>

      <button
        onclick={() => startTerminalSession()}
        disabled={isSpawning}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium shadow-xs transition-colors"
        title="Restart terminal shell session"
      >
        <RotateCcw class="w-3.5 h-3.5 text-slate-500 {isSpawning ? 'animate-spin' : ''}" />
        <span>Restart</span>
      </button>

      {#if onOpenExternal}
        <button
          onclick={onOpenExternal}
          class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg bg-white hover:bg-slate-50 text-slate-700 border border-slate-200 text-xs font-medium shadow-xs transition-colors"
          title="Open external Windows Terminal"
        >
          <ExternalLink class="w-3.5 h-3.5 text-slate-500" />
          <span>External</span>
        </button>
      {/if}
    </div>
  </div>

  <div class="rounded-xl border border-slate-800 bg-[#18181B] p-2.5 shadow-md overflow-hidden flex-1 min-h-[440px] lg:min-h-[560px] max-h-[calc(100vh-190px)] flex flex-col">
    <div bind:this={terminalContainer} class="w-full h-full flex-1"></div>
  </div>
</div>
