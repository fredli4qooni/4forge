<script lang="ts">
  import { onMount } from "svelte";
  import type {
    DetectedProject,
    LogMessage,
    PortCheckResult,
    ServiceItem,
    SiteItem,
    TabType,
    UpdateCheck,
    ViewMode,
  } from "./types";
  import { INITIAL_SERVICES, INITIAL_SITES, INITIAL_LOGS } from "./lib/constants";
  import {
    checkSystemUpdates,
    openUrl,
    openProjectsFolder,
    openDevTerminal as launchTerminal,
    launchDbManager,
    openAdminer,
    launchNativeClient,
    openConfig,
    syncHosts,
    openVsCode,
    detectProject,
    scanWorkspace,
    fetchServices,
    fetchSites,
    fetchMissingHosts,
    fetchLogs,
    fetchPortConflicts,
    startService,
    stopService,
    toggleAllServices,
    addVirtualHost,
    deleteVirtualHost,
    updateServicePort,
    setRuntimeVersion,
    autoRegisterProject,
    createDb,
    openProjectTerminal,
  } from "./lib/actions";
  import ExpandedCockpit from "./components/ExpandedCockpit.svelte";
  import Toast from "./components/Toast.svelte";
  import AddSiteModal from "./components/modals/AddSiteModal.svelte";
  import DatabaseModal from "./components/modals/DatabaseModal.svelte";
  import UpdateModal from "./components/modals/UpdateModal.svelte";
  import SettingsModal from "./components/modals/SettingsModal.svelte";

  let activeTab = $state<TabType>("services");
  let showSettingsModal = $state(false);
  let allRunning = $state(false);
  let isLoading = $state(false);
  let toastMessage = $state<string | null>(null);
  let uptimeSeconds = $state(0);
  let portConflicts = $state<PortCheckResult[]>([]);
  let dismissedConflictBanner = $state(false);
  let showAddSiteModal = $state(false);
  let newSiteDomain = $state("");
  let newSitePath = $state("");
  let newSiteType = $state("fastcgi");
  let newSiteTarget = $state("127.0.0.1:9000");
  let detectedProjectItem = $state<DetectedProject | null>(null);
  let domainSuffix = $state("test");
  let isScanningWorkspace = $state(false);
  let missingHosts = $state<string[]>([]);
  let isSyncingHosts = $state(false);
  let showUpdateModal = $state(false);
  let isCheckingUpdate = $state(false);
  let updateStatus = $state<UpdateCheck | null>(null);
  let showDatabaseModal = $state(false);
  let logFilter = $state<string>("all");
  let logSearch = $state("");
  let services = $state<ServiceItem[]>(INITIAL_SERVICES);
  let sites = $state<SiteItem[]>(INITIAL_SITES);
  let logs = $state<LogMessage[]>(INITIAL_LOGS);
  let runningCount = $derived(services.filter((s) => s.status === "running").length);
  let stoppedCount = $derived(services.filter((s) => s.status !== "running").length);
  let activePhpVersion = $state("8.3.16");
  let activeNodeVersion = $state("22.14.0");
  let activePythonVersion = $state("3.12.9");
  let activeRubyVersion = $state("3.3.7");

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
      const svcId = eng === "postgresql" || eng === "postgres" ? "postgresql" : eng === "redis" ? "redis" : eng === "sqlite" ? null : "mariadb";
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
        showToast("Opened SQLite directory in Explorer");
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
      showToast(`Database '${dbName}' created successfully!`);
    } catch (err: any) {
      showToast(`Failed to create database: ${err}`);
    }
  }

  async function handleOpenProjectTerminal(path: string): Promise<void> {
    try {
      await openProjectTerminal(path);
      showToast(`Opened PowerShell terminal in ${path}`);
    } catch (err: any) {
      showToast(`Failed to open project terminal: ${err}`);
    }
  }

  async function handleOpenFolder(p?: string): Promise<void> {
    await openProjectsFolder(p);
    showToast(p ? "Opening folder in Windows Explorer..." : "Opening projects directory...");
  }

  async function handleLaunchTerminal(): Promise<void> {
    await launchTerminal();
    showToast("Launching 4Forge Dev Shell...");
  }

  async function handleOpenVsCode(path: string): Promise<void> {
    await openVsCode(path);
    showToast("Opening project in VS Code...");
  }

  async function handleDeleteSite(domain: string): Promise<void> {
    await deleteVirtualHost(domain);
    await fetchBackendState();
    showToast(`Site ${domain} removed`);
  }

  async function openServiceConfig(serviceId: string): Promise<void> {
    const res = await openConfig(serviceId);
    showToast(res ? `Opened config for ${serviceId} in Notepad (${res})` : `Opened config for ${serviceId}`);
  }

  function openServiceLogs(serviceId: string): void {
    logFilter = serviceId;
    activeTab = "logs";
    showToast(`Showing live logs for ${serviceId}`);
  }

  onMount(() => {
    fetchBackendState(true);
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

  async function scanWorkspaceProjects(): Promise<void> {
    isScanningWorkspace = true;
    const list = await scanWorkspace(domainSuffix);
    if (list && list.length > 0) {
      for (const p of list) await autoRegisterProject(p);
      await fetchBackendState();
      showToast(`Auto-scanned & registered ${list.length} virtual hosts from workspace!`);
    } else {
      showToast("No new project directories found in C:\\4forge\\projects");
    }
    isScanningWorkspace = false;
  }

  async function openSiteBrowser(domain: string): Promise<void> {
    const caddySvc = services.find((s) => s.id === "caddy");
    if (caddySvc && caddySvc.status !== "running") {
      showToast("Starting Caddy web server...");
      await startService("caddy");
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

<ExpandedCockpit
  bind:activeTab
  {isLoading}
  {allRunning}
  {runningCount}
  {stoppedCount}
  {services}
  {sites}
  bind:logs
  {portConflicts}
  bind:dismissedConflictBanner
  {activePhpVersion}
  {activeNodeVersion}
  {activePythonVersion}
  {activeRubyVersion}
  {isScanningWorkspace}
  {missingHosts}
  {isSyncingHosts}
  bind:logFilter
  bind:logSearch
  onSelectTab={(t) => (activeTab = t)}
  onToggleAll={toggleAll}
  onToggleService={toggleService}
  onOpenWeb={openWebLocalhost}
  onOpenDatabase={openDatabaseAction}
  onOpenTerminal={handleLaunchTerminal}
  onOpenProjects={handleOpenFolder}
  onOpenUpdateModal={() => (showUpdateModal = true)}
  onRefresh={() => fetchBackendState(true)}
  onOpenSettings={() => (showSettingsModal = true)}
  onApplyAlternativePort={applyAlternativePort}
  onDismissConflict={() => (dismissedConflictBanner = true)}
  onOpenConfig={openServiceConfig}
  onOpenLogs={openServiceLogs}
  onSelectPhpVersion={(v) => switchRuntime("php", v)}
  onSelectNodeVersion={(v) => switchRuntime("node", v)}
  onSelectPythonVersion={(v) => switchRuntime("python", v)}
  onSelectRubyVersion={(v) => switchRuntime("ruby", v)}
  onScanWorkspace={scanWorkspaceProjects}
  onOpenAddSite={() => (showAddSiteModal = true)}
  onSyncHosts={syncHostsNow}
  onOpenSiteBrowser={openSiteBrowser}
  onOpenSiteFolder={handleOpenFolder}
  onOpenProjectTerminal={handleOpenProjectTerminal}
  onOpenProjectInVsCode={handleOpenVsCode}
  onDeleteSite={handleDeleteSite}
  onClearLogs={() => (logs = [])}
/>

<Toast message={toastMessage} />

<AddSiteModal
  show={showAddSiteModal}
  path={newSitePath}
  domain={newSiteDomain}
  backendType={newSiteType}
  target={newSiteTarget}
  {domainSuffix}
  detectedProject={detectedProjectItem}
  onClose={() => (showAddSiteModal = false)}
  onPathChange={(p) => { newSitePath = p; detectPathFramework(p); }}
  onDomainChange={(d) => (newSiteDomain = d)}
  onSuffixChange={(s) => { domainSuffix = s; detectPathFramework(newSitePath); }}
  onTypeChange={(t) => (newSiteType = t)}
  onTargetChange={(t) => (newSiteTarget = t)}
  onSubmit={handleAddSite}
/>

<DatabaseModal
  show={showDatabaseModal}
  onClose={() => (showDatabaseModal = false)}
  onOpenAdminer={openWebAdminerExplicit}
  onLaunchNative={launchNativeClientExplicit}
  onCreateDatabase={handleCreateDatabase}
  onCopyUrl={() => {
    navigator.clipboard?.writeText("http://localhost/4forge-adminer/index.php");
    showToast("Adminer URL copied to clipboard!");
  }}
/>

<UpdateModal
  show={showUpdateModal}
  {updateStatus}
  {isCheckingUpdate}
  onClose={() => (showUpdateModal = false)}
  onCheckUpdates={checkUpdates}
/>

<SettingsModal
  show={showSettingsModal}
  {domainSuffix}
  onSaveDomainSuffix={(s) => {
    domainSuffix = s;
    showToast(`Default domain suffix updated to .${s}`);
  }}
  onClose={() => (showSettingsModal = false)}
/>
