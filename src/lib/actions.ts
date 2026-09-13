import { invokeTauri } from "./api";
import type { DatabaseLaunchResult, DetectedProject, LogMessage, PortCheckResult, ServiceItem, SiteItem, UpdateCheck } from "../types";

export async function checkSystemUpdates(): Promise<UpdateCheck> {
  const res = await invokeTauri<UpdateCheck>("check_for_updates");
  if (res) return res;
  return {
    current_version: "0.1.0",
    channel: "stable",
    update_available: false,
    latest_version: "0.1.0",
    release_notes: "You are running the latest verified release of 4Forge.",
    release_url: "https://github.com/fredli4qooni/4forge/releases",
    signature_verified: true,
  };
}

export async function openUrl(url: string): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_browser", { url });
  } else {
    window.open(url, "_blank");
  }
}

export async function openProjectsFolder(path?: string): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_projects_folder", path ? { path } : {});
  }
}

export async function openDevTerminal(): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_system_terminal", {});
  }
}

export async function launchDbManager(): Promise<DatabaseLaunchResult | null> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return await invokeTauri<DatabaseLaunchResult>("launch_database_manager");
  }
  return null;
}

export async function openAdminer(): Promise<string | null> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return await invokeTauri<string>("open_adminer_in_browser");
  }
  window.open("http://localhost/4forge-adminer/index.php", "_blank");
  return "http://localhost/4forge-adminer/index.php";
}

export async function launchNativeClient(): Promise<boolean> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    const launched = await invokeTauri<boolean>("open_database_gui");
    return Boolean(launched);
  }
  return false;
}

export async function openConfig(serviceId: string): Promise<string | null> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return await invokeTauri<string>("open_service_config", { serviceId });
  }
  return null;
}

export async function syncHosts(): Promise<boolean> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    const ok = await invokeTauri<boolean>("sync_windows_hosts");
    return Boolean(ok);
  }
  return true;
}

export async function openVsCode(path: string): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_path_in_vscode", { path });
  }
}

export async function detectProject(path: string, domainSuffix: string): Promise<DetectedProject | null> {
  if (!path || path.trim().length < 2) return null;
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return await invokeTauri<DetectedProject>("detect_project_framework", {
      path: path.trim(),
      domainSuffix,
    });
  }
  const parts = path.trim().split(/[/\\]/).filter(Boolean);
  const name = parts.length > 0 ? parts[parts.length - 1] : "project";
  const cleanSlug = name.toLowerCase().replace(/[^a-z0-9]/g, "-");
  return {
    name,
    path: path.trim(),
    domain: `${cleanSlug}.${domainSuffix}`,
    framework: "Auto-Detected Stack",
    runtime: "PHP 8.3",
    web_root: path.trim(),
    backend_type: "fastcgi",
    target: "127.0.0.1:9000",
  };
}

export async function scanWorkspace(domainSuffix: string): Promise<DetectedProject[]> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    const list = await invokeTauri<DetectedProject[]>("scan_projects_directory", {
      dir: null,
      domainSuffix,
    });
    return list || [];
  }
  return [];
}

export async function fetchServices(): Promise<any[] | null> {
  return await invokeTauri<any[]>("get_services");
}

export async function fetchSites(): Promise<SiteItem[] | null> {
  return await invokeTauri<SiteItem[]>("list_sites");
}

export async function fetchMissingHosts(): Promise<string[] | null> {
  return await invokeTauri<string[]>("check_hosts_sync");
}

export async function fetchLogs(limit: number = 100): Promise<LogMessage[] | null> {
  return await invokeTauri<LogMessage[]>("get_logs", { limit });
}

export async function fetchPortConflicts(): Promise<PortCheckResult[] | null> {
  return await invokeTauri<PortCheckResult[]>("check_port_conflicts");
}

export async function startService(id: string): Promise<void> {
  await invokeTauri("start_service", { id });
}

export async function stopService(id: string): Promise<void> {
  await invokeTauri("stop_service", { id });
}

export async function toggleAllServices(start: boolean): Promise<void> {
  await invokeTauri("toggle_all_services", { start });
}

export async function addVirtualHost(domain: string, rootDir: string, backendType: string, target: string): Promise<void> {
  await invokeTauri("add_site", { domain, rootDir, backendType, target });
}

export async function deleteVirtualHost(domain: string): Promise<void> {
  await invokeTauri("delete_site", { domain });
}

export async function updateServicePort(id: string, newPort: number): Promise<void> {
  await invokeTauri("update_service_port", { id, newPort });
}

export async function setRuntimeVersion(kind: string, version: string): Promise<void> {
  await invokeTauri("set_active_runtime", { kind, version });
}

export async function autoRegisterProject(project: DetectedProject): Promise<void> {
  await invokeTauri("auto_register_detected_project", { project });
}
