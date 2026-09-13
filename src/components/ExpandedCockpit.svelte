<script lang="ts">
  import type {
    LogMessage,
    PortCheckResult,
    ServiceItem,
    SiteItem,
    TabType,
  } from "../types";
  import Sidebar from "./Sidebar.svelte";
  import HeaderCockpit from "./HeaderCockpit.svelte";
  import PortConflictBanner from "./PortConflictBanner.svelte";
  import DashboardTab from "./tabs/DashboardTab.svelte";
  import ServicesTab from "./tabs/ServicesTab.svelte";
  import SitesTab from "./tabs/SitesTab.svelte";
  import RuntimesTab from "./tabs/RuntimesTab.svelte";
  import LogsTab from "./tabs/LogsTab.svelte";

  let {
    activeTab = $bindable("dashboard"),
    isLoading,
    allRunning,
    runningCount,
    stoppedCount,
    services,
    sites,
    logs = $bindable([]),
    portConflicts,
    dismissedConflictBanner = $bindable(false),
    activePhpVersion,
    activeNodeVersion,
    activePythonVersion,
    activeRubyVersion,
    isScanningWorkspace,
    missingHosts,
    isSyncingHosts,
    logFilter = $bindable("all"),
    logSearch = $bindable(""),
    onSelectTab,
    onToggleAll,
    onToggleService,
    onOpenWeb,
    onOpenDatabase,
    onOpenTerminal,
    onOpenProjects,
    onOpenUpdateModal,
    onRefresh,
    onSwitchToCompact,
    onApplyAlternativePort,
    onDismissConflict,
    onOpenConfig,
    onOpenLogs,
    onSelectPhpVersion,
    onSelectNodeVersion,
    onSelectPythonVersion,
    onSelectRubyVersion,
    onScanWorkspace,
    onOpenAddSite,
    onSyncHosts,
    onOpenSiteBrowser,
    onOpenSiteFolder,
    onOpenProjectTerminal,
    onOpenProjectInVsCode,
    onDeleteSite,
    onClearLogs,
  }: {
    activeTab: TabType;
    isLoading: boolean;
    allRunning: boolean;
    runningCount: number;
    stoppedCount: number;
    services: ServiceItem[];
    sites: SiteItem[];
    logs: LogMessage[];
    portConflicts: PortCheckResult[];
    dismissedConflictBanner: boolean;
    activePhpVersion: string;
    activeNodeVersion: string;
    activePythonVersion: string;
    activeRubyVersion: string;
    isScanningWorkspace: boolean;
    missingHosts: string[];
    isSyncingHosts: boolean;
    logFilter: string;
    logSearch: string;
    onSelectTab: (tab: TabType) => void;
    onToggleAll: () => void;
    onToggleService: (id: string) => void;
    onOpenWeb: () => void;
    onOpenDatabase: (engine?: string) => void;
    onOpenTerminal: () => void;
    onOpenProjects: () => void;
    onOpenUpdateModal: () => void;
    onRefresh: () => void;
    onSwitchToCompact: () => void;
    onApplyAlternativePort: (conflict: PortCheckResult) => void;
    onDismissConflict: () => void;
    onOpenConfig: (serviceId: string) => void;
    onOpenLogs: (serviceId: string) => void;
    onSelectPhpVersion: (v: string) => void;
    onSelectNodeVersion: (v: string) => void;
    onSelectPythonVersion: (v: string) => void;
    onSelectRubyVersion: (v: string) => void;
    onScanWorkspace: () => void;
    onOpenAddSite: () => void;
    onSyncHosts: () => void;
    onOpenSiteBrowser: (domain: string) => void;
    onOpenSiteFolder: (path?: string) => void;
    onOpenProjectTerminal: (path: string) => void;
    onOpenProjectInVsCode: (path: string) => void;
    onDeleteSite: (domain: string) => void;
    onClearLogs: () => void;
  } = $props();
</script>

<div class="flex h-screen w-screen bg-[#F8FAFC] text-slate-900 font-sans antialiased overflow-hidden select-none">
  <Sidebar {activeTab} onSelectTab={onSelectTab} onOpenUpdateModal={onOpenUpdateModal} />

  <div class="flex-1 flex flex-col min-w-0 bg-[#F8FAFC]">
    <HeaderCockpit
      {isLoading}
      {allRunning}
      {runningCount}
      {stoppedCount}
      onToggleAll={onToggleAll}
      onOpenWeb={onOpenWeb}
      onOpenDatabase={onOpenDatabase}
      onOpenTerminal={onOpenTerminal}
      onOpenProjects={onOpenProjects}
      onOpenUpdateModal={onOpenUpdateModal}
      onRefresh={onRefresh}
      onSwitchToCompact={onSwitchToCompact}
    />

    <main class="flex-1 overflow-y-auto p-8 space-y-6">
      <PortConflictBanner
        conflicts={portConflicts}
        dismissed={dismissedConflictBanner}
        onApplyAlternative={onApplyAlternativePort}
        onDismiss={onDismissConflict}
      />

      {#if activeTab === "dashboard"}
        <DashboardTab
          {services}
          {sites}
          {portConflicts}
          {activePhpVersion}
          {activeNodeVersion}
          {isScanningWorkspace}
          onNavigateServices={() => onSelectTab("services")}
          onToggleService={onToggleService}
          onOpenWeb={onOpenWeb}
          onOpenDatabase={onOpenDatabase}
          onOpenConfig={onOpenConfig}
          onOpenLogs={onOpenLogs}
          onSelectPhpVersion={onSelectPhpVersion}
          onSelectNodeVersion={onSelectNodeVersion}
          onScanWorkspace={onScanWorkspace}
          onOpenAddSite={onOpenAddSite}
          onOpenSiteBrowser={onOpenSiteBrowser}
          onOpenSiteFolder={onOpenSiteFolder}
          onOpenProjectTerminal={onOpenProjectTerminal}
          onOpenProjectInVsCode={onOpenProjectInVsCode}
          onDeleteSite={onDeleteSite}
        />
      {:else if activeTab === "services"}
        <ServicesTab
          {services}
          {activePhpVersion}
          {activeNodeVersion}
          onToggleService={onToggleService}
          onOpenWeb={onOpenWeb}
          onOpenDatabase={onOpenDatabase}
          onOpenConfig={onOpenConfig}
          onOpenLogs={onOpenLogs}
          onSelectPhpVersion={onSelectPhpVersion}
          onSelectNodeVersion={onSelectNodeVersion}
        />
      {:else if activeTab === "sites"}
        <SitesTab
          {sites}
          {missingHosts}
          {isScanningWorkspace}
          {isSyncingHosts}
          onScanWorkspace={onScanWorkspace}
          onOpenAddSite={onOpenAddSite}
          onSyncHosts={onSyncHosts}
          onOpenSiteBrowser={onOpenSiteBrowser}
          onOpenSiteFolder={onOpenSiteFolder}
          onOpenProjectTerminal={onOpenProjectTerminal}
          onOpenProjectInVsCode={onOpenProjectInVsCode}
          onDeleteSite={onDeleteSite}
        />
      {:else if activeTab === "runtimes"}
        <RuntimesTab
          {activePhpVersion}
          {activeNodeVersion}
          {activePythonVersion}
          {activeRubyVersion}
          onSelectPhpVersion={onSelectPhpVersion}
          onSelectNodeVersion={onSelectNodeVersion}
          onSelectPythonVersion={onSelectPythonVersion}
          onSelectRubyVersion={onSelectRubyVersion}
        />
      {:else if activeTab === "logs"}
        <LogsTab {logs} bind:logFilter bind:logSearch onClearLogs={onClearLogs} />
      {/if}
    </main>
  </div>
</div>
