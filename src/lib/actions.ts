import { invokeTauri } from "./api";
import type { DatabaseLaunchResult, DetectedProject, LogMessage, PortCheckResult, ServiceItem, SiteItem, UpdateCheck, UserDatabaseItem } from "../types";

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

export async function openProjectsFolder(customPath?: string): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_projects_folder", { path: customPath || null });
  }
}

export async function openDevTerminal(): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_system_terminal", { cwd: null });
  }
}

export async function launchDbManager(engine?: string): Promise<DatabaseLaunchResult | null> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return await invokeTauri<DatabaseLaunchResult>("launch_database_manager", { engine: engine || null });
  }
  return null;
}

export async function openAdminer(engine?: string, dbName?: string): Promise<string | null> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return await invokeTauri<string>("open_adminer_in_browser", { engine: engine || null, dbName: dbName || null });
  }
  const url = `http://localhost/__4forge/db${engine ? `?driver=${engine}${dbName ? `&db=${dbName}` : ""}` : ""}`;
  window.open(url, "_blank");
  return url;
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
  try {
    return await invokeTauri<any[]>("get_services");
  } catch {
    return null;
  }
}

export async function fetchSites(): Promise<SiteItem[] | null> {
  try {
    return await invokeTauri<SiteItem[]>("list_sites");
  } catch {
    return null;
  }
}

export async function fetchMissingHosts(): Promise<string[] | null> {
  try {
    return await invokeTauri<string[]>("check_hosts_sync");
  } catch {
    return null;
  }
}

export async function fetchLogs(limit: number = 100): Promise<LogMessage[] | null> {
  try {
    return await invokeTauri<LogMessage[]>("get_logs", { limit });
  } catch {
    return null;
  }
}

export async function fetchPortConflicts(): Promise<PortCheckResult[] | null> {
  try {
    return await invokeTauri<PortCheckResult[]>("check_port_conflicts");
  } catch {
    return null;
  }
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

export async function deleteVirtualHost(domain: string, deleteFiles: boolean = false): Promise<void> {
  await invokeTauri("delete_site", { domain, deleteFiles });
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

export async function createDb(dbName: string, engine?: string): Promise<boolean> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    const res = await invokeTauri<boolean>("create_database", { dbName, engine: engine || "mariadb" });
    return Boolean(res);
  }
  return true;
}

export async function fetchAllDatabases(): Promise<UserDatabaseItem[]> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    try {
      const res = await invokeTauri<UserDatabaseItem[]>("list_all_databases");
      return res || [];
    } catch {
      return [];
    }
  }
  return [];
}

export async function deleteDb(engine: string, dbName: string): Promise<boolean> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    const res = await invokeTauri<boolean>("delete_database", { engine, dbName });
    return Boolean(res);
  }
  return true;
}

export async function openProjectTerminal(path: string): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_project_terminal", { path });
  }
}

export async function setWindowCompactMode(compact: boolean): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("set_window_compact_mode", { compact });
  }
}

export async function spawnTerminal(cwd?: string, cols?: number, rows?: number): Promise<string> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    const id = await invokeTauri<string>("terminal_spawn", { cwd: cwd || null, cols: cols || 80, rows: rows || 24 });
    return id || "";
  }
  return "mock-session";
}

export async function writeTerminal(sessionId: string, data: string): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("terminal_write", { sessionId, data });
  }
}

export async function resizeTerminal(sessionId: string, cols: number, rows: number): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("terminal_resize", { sessionId, cols, rows });
  }
}

export async function killTerminal(sessionId: string): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("terminal_kill", { sessionId });
  }
}

export async function injectProjectEnv(
  projectPath: string,
  framework: string,
  variables: Record<string, string>
): Promise<{ success: boolean; env_path: string; backup_path?: string; updated_keys: string[]; message: string }> {
  return await invokeTauri("inject_project_database_env", { projectPath, framework, variables });
}

export interface BackupFileItem {
  file_name: string;
  file_path: string;
  size_bytes: number;
  modified_timestamp: number;
}

export async function exportDatabase(
  engine: string,
  dbName: string,
  outputPath?: string,
  includeData: boolean = true
): Promise<{ success: boolean; file_path: string; size_bytes: number; message: string }> {
  return await invokeTauri("export_database", {
    engine,
    dbName,
    outputPath: outputPath || null,
    includeData,
  });
}

export async function importDatabase(
  engine: string,
  dbName: string,
  inputPath: string
): Promise<{ success: boolean; db_name: string; message: string }> {
  return await invokeTauri("import_database", {
    engine,
    dbName,
    inputPath,
  });
}

export async function fetchBackupFiles(): Promise<BackupFileItem[]> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return (await invokeTauri<BackupFileItem[]>("list_backup_files")) || [];
  }
  return [];
}

export async function openBackupsFolder(): Promise<void> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    await invokeTauri("open_backups_folder");
  }
}

export interface DatabaseUserItem {
  username: string;
  host: string;
  engine: string;
  privileges: string[];
  is_system_account: boolean;
}

export async function fetchDatabaseUsers(engine: string): Promise<DatabaseUserItem[]> {
  const hasTauri = typeof window !== "undefined" && (Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__));
  if (hasTauri) {
    return (await invokeTauri<DatabaseUserItem[]>("list_database_users", { engine })) || [];
  }
  return [];
}

export async function createDatabaseUser(
  engine: string,
  username: string,
  host: string,
  password: string,
  privilegeScope: string = "all",
  targetDb?: string
): Promise<void> {
  await invokeTauri("create_database_user", {
    engine,
    username,
    host,
    password,
    privilegeScope,
    targetDb: targetDb || null,
  });
}

export async function updateDatabaseUserPassword(
  engine: string,
  username: string,
  host: string,
  newPassword: string
): Promise<void> {
  await invokeTauri("update_database_user_password", {
    engine,
    username,
    host,
    newPassword,
  });
}

export async function dropDatabaseUser(
  engine: string,
  username: string,
  host: string
): Promise<void> {
  await invokeTauri("drop_database_user", {
    engine,
    username,
    host,
  });
}

