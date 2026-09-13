<script lang="ts">
  import { onMount } from "svelte";
  import type {
    DetectedProject, LogMessage, PortCheckResult, ServiceItem, SiteItem, TabType, UpdateCheck, UserDatabaseItem,
  } from "./types";
  import { INITIAL_SERVICES, INITIAL_SITES, INITIAL_LOGS } from "./lib/constants";
  import {
    checkSystemUpdates, openUrl, openProjectsFolder, openDevTerminal as launchTerminal,
    launchDbManager, openAdminer, launchNativeClient, openConfig, syncHosts, openVsCode,
    detectProject, scanWorkspace, fetchServices, fetchSites, fetchMissingHosts, fetchLogs,
    fetchPortConflicts, startService, stopService, toggleAllServices, addVirtualHost,
    deleteVirtualHost, updateServicePort, setRuntimeVersion, autoRegisterProject, createDb,
    fetchAllDatabases, deleteDb, openProjectTerminal,
  } from "./lib/actions";
  import CompactCockpit from "./components/CompactCockpit.svelte";
  import SitesTab from "./components/tabs/SitesTab.svelte";
  import DatabasesTab from "./components/tabs/DatabasesTab.svelte";
  import RuntimesTab from "./components/tabs/RuntimesTab.svelte";
  import LogsTab from "./components/tabs/LogsTab.svelte";
  import TerminalTab from "./components/tabs/TerminalTab.svelte";
  import PortConflictBanner from "./components/PortConflictBanner.svelte";
  import TopNav from "./components/TopNav.svelte";
  import ModalsContainer from "./components/ModalsContainer.svelte";
  import type { ScaffoldRequest } from "./components/modals/CreateProjectModal.svelte";

  let activeTab = $state<TabType>("cockpit"), showSettingsModal = $state(false), allRunning = $state(false), isLoading = $state(false);
  let toastMessage = $state<string | null>(null), uptimeSeconds = $state(0), activeTerminalCwd = $state("C:\\4forge\\projects"), activeTerminalCommand = $state("");
  let portConflicts = $state<PortCheckResult[]>([]), dismissedConflictBanner = $state(false);
  let showAddSiteModal = $state(false), showCreateProjectModal = $state(false), newSiteDomain = $state(""), newSitePath = $state(""), newSiteType = $state("fastcgi"), newSiteTarget = $state("127.0.0.1:9000");
  let detectedProjectItem = $state<DetectedProject | null>(null), domainSuffix = $state("test"), isScanningWorkspace = $state(false), missingHosts = $state<string[]>([]), isSyncingHosts = $state(false);
  let showUpdateModal = $state(false), isCheckingUpdate = $state(false), updateStatus = $state<UpdateCheck | null>(null), showDatabaseModal = $state(false), showDeleteModal = $state(false), siteToDelete = $state<SiteItem | null>(null);
  let logFilter = $state<string>("all"), logSearch = $state("");
  let services = $state<ServiceItem[]>(INITIAL_SERVICES), sites = $state<SiteItem[]>(INITIAL_SITES), logs = $state<LogMessage[]>(INITIAL_LOGS), databases = $state<UserDatabaseItem[]>([]);
  let runningCount = $derived(services.filter((s) => s.status === "running").length);
  let stoppedCount = $derived(services.filter((s) => s.status !== "running").length);
  let activePhpVersion = $state("8.3.16"), activeNodeVersion = $state("22.14.0"), activePythonVersion = $state("3.12.9"), activeRubyVersion = $state("3.3.7");

  function showToast(msg: string): void {
    toastMessage = msg;
    setTimeout(() => { toastMessage = null; }, 3000);
  }

  async function checkUpdates(): Promise<void> {
    isCheckingUpdate = true;
    updateStatus = await checkSystemUpdates();
    isCheckingUpdate = false;
  }

  async function openWebLocalhost(): Promise<void> {
    const caddySvc = services.find((s) => s.id === "caddy");
    if (caddySvc && caddySvc.status !== "running") {
      showToast("Starting Caddy web server...");
      await startService("caddy");
      await new Promise((r) => setTimeout(r, 600));
      await fetchBackendState(false);
    }
    const port = caddySvc?.ports.includes("8080") ? "8080" : "80";
    const url = port === "80" ? "http://localhost" : `http://localhost:${port}`;
    await openUrl(url);
    showToast(`Opening ${url} in browser...`);
  }

  async function openDatabaseAction(engine?: string): Promise<void> {
    try {
      const eng = (engine || "mariadb").toLowerCase();
      const svcId = eng === "postgresql" || eng === "postgres" ? "postgresql" : eng === "redis" ? "redis" : eng === "mongodb" || eng === "mongo" ? "mongodb" : eng === "sqlite" ? null : "mariadb";
      if (svcId) {
        const svc = services.find((s) => s.id === svcId);
        if (svc && svc.status !== "running") {
          showToast(`Starting ${svc.name}...`);
          await startService(svcId);
          await new Promise((r) => setTimeout(r, 600));
          await fetchBackendState(false);
        }
      }
      const result = await launchDbManager(eng);
      if (result?.launched_type === "native") {
        showToast(`Native database client launched: ${result.client_name}`);
      } else if (result?.launched_type === "web_adminer") {
        showToast("Opened built-in Adminer database manager in browser");
      } else if (result?.launched_type === "folder") {
        showToast(eng === "mongodb" || eng === "mongo" ? "Opened MongoDB data directory" : "Opened SQLite directory in Explorer");
      } else if (result?.launched_type === "terminal") {
        showToast("Opened Redis CLI terminal");
      } else {
        showDatabaseModal = true;
      }
    } catch (err: any) {
      showToast(err?.message || String(err));
    }
  }

  async function openWebAdminerExplicit(): Promise<void> {
    const mariaSvc = services.find((s) => s.id === "mariadb");
    const caddySvc = services.find((s) => s.id === "caddy");
    if (mariaSvc && mariaSvc.status !== "running") await startService("mariadb");
    if (caddySvc && caddySvc.status !== "running") await startService("caddy");
    try {
      const url = await openAdminer();
      if (url) {
        showToast(`Adminer database manager opened: ${url}`);
        showDatabaseModal = false;
      }
    } catch (err: any) {
      showToast(`Failed to open Adminer: ${err}`);
    }
  }

  async function launchNativeClientExplicit(): Promise<void> {
    const mariaSvc = services.find((s) => s.id === "mariadb");
    if (mariaSvc && mariaSvc.status !== "running") {
      showToast("Starting MariaDB service...");
      await startService("mariadb");
      await new Promise((r) => setTimeout(r, 600));
      await fetchBackendState(false);
    }
    try {
      const launched = await launchNativeClient();
      if (launched) {
        showToast("Desktop database client launched successfully!");
        showDatabaseModal = false;
      } else {
        showToast("No native desktop client (HeidiSQL, DBeaver, TablePlus) found.");
      }
    } catch (err: any) {
      showToast(err?.message || String(err));
    }
  }

  async function handleCreateDatabase(dbName: string, engine?: string): Promise<void> {
    try {
      await createDb(dbName, engine);
      databases = await fetchAllDatabases();
      showToast(`Database '${dbName}' created successfully!`);
    } catch (err: any) {
      showToast(`Failed to create database: ${err?.message || err}`);
      throw err;
    }
  }

  async function handleDeleteDatabase(engine: string, name: string): Promise<void> {
    try {
      await deleteDb(engine, name);
      databases = await fetchAllDatabases();
      showToast(`Database '${name}' removed.`);
    } catch (err: any) {
      showToast(`Failed to delete database: ${err}`);
    }
  }

  async function handleOpenAdminerWithParams(engine?: string, dbName?: string): Promise<void> {
    const caddySvc = services.find((s) => s.id === "caddy");
    const phpSvc = services.find((s) => s.id === "php");
    let needed = false;
    if (caddySvc && caddySvc.status !== "running") { await startService("caddy"); needed = true; }
    if (phpSvc && phpSvc.status !== "running") { await startService("php"); needed = true; }
    if (needed) { await new Promise((r) => setTimeout(r, 600)); await fetchBackendState(false); }
    try {
      const url = await openAdminer(engine, dbName);
      if (url) showToast(`Adminer opened: ${url}`);
    } catch (err: any) {
      showToast(`Failed to open Adminer: ${err}`);
    }
  }

  async function handleOpenFolder(p?: string): Promise<void> { await openProjectsFolder(p); showToast(p ? "Opening folder in Windows Explorer..." : "Opening projects directory..."); }
  function handleOpenProjectTerminal(path: string): void { activeTerminalCwd = path; activeTab = "terminal"; showToast(`Opened 4Forge Dev Shell in ${path}`); }
  function handleLaunchIntegratedTerminal(): void { activeTerminalCwd = "C:\\4forge\\projects"; activeTab = "terminal"; }
  async function handleLaunchTerminal(): Promise<void> { await launchTerminal(); showToast("Launching External Windows Terminal..."); }
  async function handleOpenVsCode(path: string): Promise<void> { await openVsCode(path); showToast("Opening project in VS Code..."); }
  function promptDeleteSite(d: string): void { siteToDelete = sites.find((s) => s.domain === d) || { domain: d, path: `C:\\4forge\\projects\\${d.split('.')[0]}`, runtime: "PHP 8.3", ssl: true, backend_type: "fastcgi", target: "127.0.0.1:9000" }; showDeleteModal = true; }
  async function handleConfirmDelete(delFiles: boolean): Promise<void> { if (!siteToDelete) return; const d = siteToDelete.domain; showDeleteModal = false; await deleteVirtualHost(d, delFiles); await fetchBackendState(); showToast(delFiles ? `Project ${d} and files deleted` : `Virtual host ${d} removed`); siteToDelete = null; }
  async function openServiceConfig(serviceId: string): Promise<void> { const res = await openConfig(serviceId); showToast(res ? `Opened config for ${serviceId} in Notepad (${res})` : `Opened config for ${serviceId}`); }
  function openServiceLogs(serviceId: string): void { logFilter = serviceId; activeTab = "logs"; showToast(`Showing live logs for ${serviceId}`); }

  function handleOpenTerminalWithCommand(cmd: string, cwd?: string): void {
    activeTerminalCwd = cwd || "C:\\4forge\\projects";
    activeTerminalCommand = cmd;
    activeTab = "terminal";
    showToast(`Opening 4Forge Terminal for '${cmd.slice(0, 32)}...'`);
  }

  async function handleScaffoldGui(req: ScaffoldRequest): Promise<void> {
    showToast(`Scaffolding ${req.template.toUpperCase()} project '${req.projectName}'...`);
    if (req.createDatabase) {
      try {
        const dbEng = req.template === "mern" ? "mongodb" : "mariadb";
        await createDb(req.databaseName, dbEng);
        showToast(`Created database '${req.databaseName}' in ${dbEng === "mongodb" ? "MongoDB" : "MariaDB"}`);
      } catch (e) {}
    }
    if (req.autoVhost) {
      try {
        if (req.template === "mern") {
          await addVirtualHost(req.domain, req.targetPath, "proxy", "127.0.0.1:5000");
        } else {
          await addVirtualHost(req.domain, `${req.targetPath}\\public`, "fastcgi", "127.0.0.1:9000");
        }
      } catch (e) {}
    }
    handleOpenTerminalWithCommand(req.command, "C:\\4forge\\projects");
  }

  onMount(() => {
    fetchBackendState(true);
    scanWorkspaceProjects(true);
    const backendTimer = setInterval(() => fetchBackendState(false), 2500);
    const uptimeTimer = setInterval(() => {
      if (runningCount > 0) uptimeSeconds += 1;
      else uptimeSeconds = 0;
    }, 1000);
    return () => {
      clearInterval(backendTimer);
      clearInterval(uptimeTimer);
    };
  });

  async function fetchBackendState(checkPorts: boolean = false): Promise<void> {
    const backendServices = await fetchServices();
    if (backendServices && backendServices.length > 0) {
      services = services.map((s) => {
        const found = backendServices.find((bs) => bs.id === s.id);
        return found ? { ...s, status: found.status, pid: found.pid } : s;
      });
      allRunning = services.length > 0 && services.every((s) => s.status === "running");
    }
    const backendSites = await fetchSites();
    if (backendSites) sites = backendSites;
    const backendDatabases = await fetchAllDatabases();
    if (backendDatabases) databases = backendDatabases;
    const missing = await fetchMissingHosts();
    if (missing) missingHosts = missing;
    const backendLogs = await fetchLogs(100);
    if (backendLogs && backendLogs.length > 0) logs = backendLogs;
    if (checkPorts) {
      const portChecks = await fetchPortConflicts();
      if (portChecks) portConflicts = portChecks.filter((p) => !p.is_available);
    }
  }

  async function applyAlternativePort(conflict: PortCheckResult): Promise<void> {
    if (!conflict.alternative_port) return;
    const oldP = String(conflict.port);
    const newP = String(conflict.alternative_port);
    services = services.map((s) => s.ports.includes(oldP) ? { ...s, ports: s.ports.replace(oldP, newP) } : s);
    portConflicts = portConflicts.filter((c) => c.port !== conflict.port);
    dismissedConflictBanner = true;
    await updateServicePort("caddy", conflict.alternative_port);
    showToast(`Switched ${conflict.service_name} to port ${newP}`);
  }

  async function syncHostsNow(): Promise<void> {
    isSyncingHosts = true;
    try {
      const ok = await syncHosts();
      if (ok) {
        showToast("Windows hosts file synced successfully!");
        missingHosts = [];
      } else {
        showToast("Hosts sync cancelled or failed.");
      }
    } catch (err: any) {
      showToast(`Failed to sync hosts: ${err}`);
    }
    isSyncingHosts = false;
  }

  async function toggleAll(): Promise<void> {
    isLoading = true;
    const targetState = !allRunning;
    await toggleAllServices(targetState);
    await new Promise((r) => setTimeout(r, 600));
    await fetchBackendState(true);
    isLoading = false;
    showToast(targetState ? "All services started successfully" : "All services stopped");
  }

  async function toggleService(id: string): Promise<void> {
    const svc = services.find((s) => s.id === id);
    if (!svc) return;
    const willStart = svc.status !== "running";
    if (willStart) await startService(id);
    else await stopService(id);
    await new Promise((r) => setTimeout(r, 500));
    await fetchBackendState(true);
    showToast(`Service '${svc.name}' ${willStart ? "started" : "stopped"}`);
  }

  async function handleAddSite(): Promise<void> {
    if (!newSiteDomain.trim() || !newSitePath.trim()) {
      showToast("Please enter a domain and local directory path");
      return;
    }
    const domain = newSiteDomain.trim();
    await addVirtualHost(domain, newSitePath.trim(), newSiteType, newSiteTarget.trim());
    await fetchBackendState();
    showAddSiteModal = false;
    newSiteDomain = "";
    newSitePath = "";
    showToast(`Site ${domain} created and SSL certificate prepared`);
  }

  async function detectPathFramework(path: string): Promise<void> {
    const res = await detectProject(path, domainSuffix);
    if (res) {
      detectedProjectItem = res;
      newSiteDomain = res.domain;
      newSiteType = res.backend_type;
      newSiteTarget = res.target;
    } else {
      detectedProjectItem = null;
    }
  }

  async function scanWorkspaceProjects(silent: boolean = false): Promise<void> {
    isScanningWorkspace = true;
    const list = await scanWorkspace(domainSuffix);
    if (list && list.length > 0) {
      for (const p of list) await autoRegisterProject(p);
      await fetchBackendState();
      if (!silent) showToast(`Auto-scanned & registered ${list.length} virtual hosts from workspace!`);
    } else if (!silent) {
      showToast("No new project directories found in C:\\4forge\\projects");
    }
    isScanningWorkspace = false;
  }

  async function openSiteBrowser(domain: string): Promise<void> {
    const caddySvc = services.find((s) => s.id === "caddy");
    const phpSvc = services.find((s) => s.id === "php");
    let neededStart = false;
    if (caddySvc && caddySvc.status !== "running") {
      showToast("Starting Caddy web server...");
      await startService("caddy");
      neededStart = true;
    }
    if (phpSvc && phpSvc.status !== "running") {
      await startService("php");
      neededStart = true;
    }
    if (neededStart) {
      await new Promise((r) => setTimeout(r, 600));
      await fetchBackendState(false);
    }
    if (missingHosts.includes(domain)) await syncHostsNow();
    const url = `http://${domain}`;
    await openUrl(url);
    showToast(`Opening http://${domain}...`);
  }

  function switchRuntime(kind: "php" | "node" | "python" | "ruby", version: string): void {
    if (kind === "php") activePhpVersion = version;
    else if (kind === "node") activeNodeVersion = version;
    else if (kind === "python") activePythonVersion = version;
    else if (kind === "ruby") activeRubyVersion = version;
    setRuntimeVersion(kind, version);
    showToast(`Active ${kind.toUpperCase()} switched to ${version}`);
  }
</script>

<div class="w-screen h-screen flex flex-col bg-[#F8F9FA] text-slate-900 overflow-hidden font-sans select-none">
  <TopNav
    {activeTab} sitesCount={sites.length} databasesCount={databases.length} {isLoading} {allRunning}
    onSelectTab={(tab) => (activeTab = tab)} onToggleAll={toggleAll}
    onOpenUpdateModal={() => (showUpdateModal = true)} onRefresh={() => fetchBackendState(true)}
    onOpenSettings={() => (showSettingsModal = true)}
  />

  <main class="flex-1 overflow-y-auto">
    {#if portConflicts.length > 0 && !dismissedConflictBanner}
      <div class="p-4 pb-0">
        <PortConflictBanner
          conflicts={portConflicts}
          dismissed={dismissedConflictBanner}
          onApplyAlternative={applyAlternativePort}
          onDismiss={() => (dismissedConflictBanner = true)}
        />
      </div>
    {/if}

    {#if activeTab === "cockpit"}
      <CompactCockpit
        {services}
        {sites}
        {isLoading}
        {allRunning}
        {runningCount}
        {stoppedCount}
        {isScanningWorkspace}
        {uptimeSeconds}
        onToggleAll={toggleAll}
        onToggleService={toggleService}
        onOpenWeb={openWebLocalhost}
        onOpenDatabase={openDatabaseAction}
        onOpenTerminal={handleLaunchIntegratedTerminal}
        onOpenProjects={() => handleOpenFolder()}
        onOpenConfig={openServiceConfig}
        onOpenLogs={openServiceLogs}
        onOpenSiteBrowser={openSiteBrowser}
        onOpenProjectTerminal={handleOpenProjectTerminal}
        onOpenSiteFolder={handleOpenFolder}
        onScanWorkspace={scanWorkspaceProjects}
        onOpenAddSite={() => (showAddSiteModal = true)}
        onOpenCreateProject={() => (showCreateProjectModal = true)}
        onCreateDatabase={handleCreateDatabase}
      />
    {:else if activeTab === "sites"}
      <div class="w-full p-5 lg:px-8 lg:py-6">
        <SitesTab
          {sites}
          {missingHosts}
          {isScanningWorkspace}
          {isSyncingHosts}
          onScanWorkspace={scanWorkspaceProjects}
          onOpenAddSite={() => (showAddSiteModal = true)}
          onOpenCreateProject={() => (showCreateProjectModal = true)}
          onSyncHosts={syncHostsNow}
          onOpenSiteBrowser={openSiteBrowser}
          onOpenSiteFolder={handleOpenFolder}
          onOpenProjectTerminal={handleOpenProjectTerminal}
          onOpenProjectInVsCode={handleOpenVsCode}
          onDeleteSite={promptDeleteSite}
        />
      </div>
    {:else if activeTab === "databases"}
      <div class="w-full p-5 lg:px-8 lg:py-6">
        <DatabasesTab
          {databases}
          {services}
          onCreateDatabase={handleCreateDatabase}
          onDeleteDatabase={handleDeleteDatabase}
          onOpenAdminer={handleOpenAdminerWithParams}
          onLaunchNative={openDatabaseAction}
          onToggleService={toggleService}
          onRefresh={async () => { databases = await fetchAllDatabases(); showToast("Databases list refreshed."); }}
          onShowToast={showToast}
        />
      </div>
    {:else if activeTab === "runtimes"}
      <div class="w-full p-5 lg:px-8 lg:py-6">
        <RuntimesTab
          {activePhpVersion}
          {activeNodeVersion}
          {activePythonVersion}
          {activeRubyVersion}
          onSelectPhpVersion={(v: string) => switchRuntime("php", v)}
          onSelectNodeVersion={(v: string) => switchRuntime("node", v)}
          onSelectPythonVersion={(v: string) => switchRuntime("python", v)}
          onSelectRubyVersion={(v: string) => switchRuntime("ruby", v)}
          onOpenSettings={() => (showSettingsModal = true)}
        />
      </div>
    {:else if activeTab === "logs"}
      <div class="w-full p-5 lg:px-8 lg:py-6">
        <LogsTab
          {logs}
          bind:logFilter
          bind:logSearch
          onClearLogs={() => (logs = [])}
        />
      </div>
    {:else if activeTab === "terminal"}
      <div class="w-full p-5 lg:px-8 lg:py-6 h-[calc(100vh-56px)] flex flex-col">
        <TerminalTab
          initialCwd={activeTerminalCwd}
          initialCommand={activeTerminalCommand}
          onCommandExecuted={() => (activeTerminalCommand = "")}
          onOpenExternal={handleLaunchTerminal}
        />
      </div>
    {/if}
  </main>
</div>

<ModalsContainer
  {toastMessage} {showAddSiteModal} {showCreateProjectModal} showDeleteSiteModal={showDeleteModal}
  {siteToDelete} onCloseDeleteSite={() => (showDeleteModal = false)} onConfirmDeleteSite={handleConfirmDelete}
  {newSitePath} {newSiteDomain} {newSiteType} {newSiteTarget} {domainSuffix} {detectedProjectItem}
  {showDatabaseModal} {showUpdateModal} {showSettingsModal} {updateStatus} {isCheckingUpdate}
  onCloseAddSite={() => (showAddSiteModal = false)} onCloseCreateProject={() => (showCreateProjectModal = false)}
  onRunInTerminal={handleOpenTerminalWithCommand} onScaffoldGui={handleScaffoldGui}
  onPathChange={(p: string) => { newSitePath = p; detectPathFramework(p); }}
  onDomainChange={(d: string) => (newSiteDomain = d)}
  onSuffixChange={(s: string) => { domainSuffix = s; detectPathFramework(newSitePath); }}
  onTypeChange={(t: string) => (newSiteType = t)} onTargetChange={(t: string) => (newSiteTarget = t)}
  onSubmitAddSite={handleAddSite} onCloseDatabase={() => (showDatabaseModal = false)}
  onOpenAdminer={openWebAdminerExplicit} onLaunchNative={launchNativeClientExplicit}
  onCreateDatabase={handleCreateDatabase}
  onCopyUrl={() => { navigator.clipboard?.writeText("http://localhost/__4forge/db"); showToast("Adminer URL copied to clipboard!"); }}
  onCloseUpdate={() => (showUpdateModal = false)} onCheckUpdates={checkUpdates}
  onSaveDomainSuffix={(s: string) => { domainSuffix = s; showToast(`Default domain suffix updated to .${s}`); }}
  onCloseSettings={() => (showSettingsModal = false)}
/>
