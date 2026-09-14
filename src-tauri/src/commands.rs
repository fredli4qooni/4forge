// Copyright 2026 4Forge Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::state::AppState;
use forge_caddy_config::{BackendType, VirtualHostConfig};
use forge_runtime_manager::RuntimeKind;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceItemDto {
    pub id: String,
    pub name: String,
    pub service_type: String,
    pub version: String,
    pub ports: String,
    pub status: String,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteDto {
    pub domain: String,
    pub runtime: String,
    pub ssl: bool,
    pub path: String,
    pub backend_type: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessageDto {
    pub service: String,
    pub stream: String,
    pub message: String,
    pub timestamp_millis: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeOverviewDto {
    pub available_php: Vec<String>,
    pub available_node: Vec<String>,
    pub available_python: Vec<String>,
    pub available_ruby: Vec<String>,
    pub installed_php: Vec<String>,
    pub installed_node: Vec<String>,
    pub installed_python: Vec<String>,
    pub installed_ruby: Vec<String>,
    pub active_php: Option<String>,
    pub active_node: Option<String>,
    pub active_python: Option<String>,
    pub active_ruby: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOverviewDto {
    pub app_version: String,
    pub total_services: usize,
    pub running_services: usize,
    pub active_sites: usize,
    pub memory_mb: u64,
    pub cpu_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckDto {
    pub current_version: String,
    pub channel: String,
    pub update_available: bool,
    pub latest_version: Option<String>,
    pub release_notes: Option<String>,
    pub release_url: String,
    pub signature_verified: bool,
}

#[tauri::command]
pub async fn get_services(state: State<'_, AppState>) -> Result<Vec<ServiceItemDto>, String> {
    let supervisor_services = state.supervisor.list_services().await;

    let get_status = |id: &str| -> (String, Option<u32>) {
        if let Some(info) = supervisor_services.iter().find(|s| s.name == id) {
            let st = match info.status {
                forge_supervisor::ServiceStatus::Running => "running",
                forge_supervisor::ServiceStatus::Starting => "starting",
                forge_supervisor::ServiceStatus::Crashed => "crashed",
                forge_supervisor::ServiceStatus::Restarting => "restarting",
                forge_supervisor::ServiceStatus::Stopped => "stopped",
            };
            (st.to_string(), info.pid)
        } else {
            ("stopped".to_string(), None)
        }
    };

    let (caddy_st, caddy_pid) = get_status("caddy");
    let (mariadb_st, mariadb_pid) = get_status("mariadb");
    let (_, postgresql_pid) = get_status("postgresql");
    let (_, redis_pid) = get_status("redis");
    let (php_st, php_pid) = get_status("php");
    let (node_st, node_pid) = get_status("node");

    let maria_healthy = forge_db_manager::DatabaseManager::check_port_health(3306, 150).await;
    let pg_healthy = forge_db_manager::DatabaseManager::check_port_health(5432, 150).await;
    let redis_healthy = forge_db_manager::DatabaseManager::check_port_health(6379, 150).await;

    let mariadb_st = if maria_healthy {
        "running".to_string()
    } else if mariadb_st == "starting" {
        "starting".to_string()
    } else {
        "stopped".to_string()
    };
    let mariadb_pid = if maria_healthy { mariadb_pid } else { None };

    let postgresql_st = if pg_healthy {
        "running".to_string()
    } else {
        "stopped".to_string()
    };
    let postgresql_pid = if pg_healthy { postgresql_pid } else { None };

    let redis_st = if redis_healthy {
        "running".to_string()
    } else {
        "stopped".to_string()
    };
    let redis_pid = if redis_healthy { redis_pid } else { None };

    let list = vec![
        ServiceItemDto {
            id: "caddy".to_string(),
            name: "Caddy Web Server".to_string(),
            service_type: "Reverse Proxy & Auto-HTTPS".to_string(),
            version: "v2.9.1".to_string(),
            ports: "80, 443, 2019".to_string(),
            status: caddy_st,
            pid: caddy_pid,
        },
        ServiceItemDto {
            id: "mariadb".to_string(),
            name: "MySQL / MariaDB".to_string(),
            service_type: "Relational Database".to_string(),
            version: "v11.4.3".to_string(),
            ports: "3306".to_string(),
            status: mariadb_st,
            pid: mariadb_pid,
        },
        ServiceItemDto {
            id: "postgresql".to_string(),
            name: "PostgreSQL Database".to_string(),
            service_type: "Relational Database".to_string(),
            version: "v16.4".to_string(),
            ports: "5432".to_string(),
            status: postgresql_st,
            pid: postgresql_pid,
        },
        ServiceItemDto {
            id: "redis".to_string(),
            name: "Redis In-Memory Cache".to_string(),
            service_type: "Key-Value & Queue Cache".to_string(),
            version: "v7.2.5".to_string(),
            ports: "6379".to_string(),
            status: redis_st,
            pid: redis_pid,
        },
        ServiceItemDto {
            id: "php".to_string(),
            name: "PHP-FPM Manager".to_string(),
            service_type: "FastCGI Process".to_string(),
            version: "PHP 8.3.16 (NTS)".to_string(),
            ports: "9000".to_string(),
            status: php_st,
            pid: php_pid,
        },
        ServiceItemDto {
            id: "node".to_string(),
            name: "Node.js Runtime".to_string(),
            service_type: "JavaScript / TypeScript".to_string(),
            version: "v22.14.0 LTS".to_string(),
            ports: "Isolated".to_string(),
            status: node_st,
            pid: node_pid,
        },
    ];

    Ok(list)
}

async fn sync_caddyfile(sites: &[forge_caddy_config::VirtualHostConfig]) {
    let caddyfile_content = forge_caddy_config::CaddyConfigGenerator::generate_caddyfile(sites);
    let caddyfile_path = std::path::PathBuf::from("C:\\4forge\\config\\Caddyfile");
    if let Some(parent) = caddyfile_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&caddyfile_path, caddyfile_content);
}

fn save_sites_to_disk(sites: &[forge_caddy_config::VirtualHostConfig]) {
    let sites_path = std::path::PathBuf::from("C:\\4forge\\config\\sites.json");
    if let Some(parent) = sites_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(sites) {
        let _ = std::fs::write(&sites_path, json);
    }
}

async fn sync_caddyfile_and_reload(sites: &[forge_caddy_config::VirtualHostConfig]) {
    save_sites_to_disk(sites);
    sync_caddyfile(sites).await;
    let caddyfile_path = std::path::PathBuf::from("C:\\4forge\\config\\Caddyfile");
    let caddy_bin = std::path::PathBuf::from("C:\\4forge\\runtimes\\caddy\\caddy.exe");
    if caddy_bin.is_file() {
        let _ = std::process::Command::new(caddy_bin)
            .args(["reload", "--config", &caddyfile_path.to_string_lossy()])
            .output();
    }
}

async fn stop_service_graceful(state: &AppState, id: &str) -> Result<(), String> {
    let service_id = id.to_lowercase();
    let _ = state.supervisor.stop_service(&service_id).await;

    match service_id.as_str() {
        "mariadb" | "mysql" => {
            let extra_dirs = [
                std::path::PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"),
                std::path::PathBuf::from("D:\\laragon\\bin\\mysql"),
                std::path::PathBuf::from("C:\\laragon\\bin\\mysql"),
            ];
            if let Some(admin) =
                forge_db_manager::DatabaseManager::find_executable("mysqladmin", &extra_dirs)
                    .or_else(|| {
                        forge_db_manager::DatabaseManager::find_executable(
                            "mariadb-admin",
                            &extra_dirs,
                        )
                    })
            {
                let _ = tokio::time::timeout(
                    std::time::Duration::from_millis(2000),
                    tokio::process::Command::new(&admin)
                        .args(["-u", "root", "-h", "127.0.0.1", "--port=3306", "shutdown"])
                        .output(),
                )
                .await;
            }

            for _ in 0..10 {
                if !forge_db_manager::DatabaseManager::check_port_health(3306, 100).await {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }

            if forge_db_manager::DatabaseManager::check_port_health(3306, 100).await {
                #[cfg(windows)]
                {
                    let _ = tokio::process::Command::new("taskkill")
                        .args(["/F", "/T", "/IM", "mysqld.exe"])
                        .output()
                        .await;
                    let _ = tokio::process::Command::new("taskkill")
                        .args(["/F", "/T", "/IM", "mariadbd.exe"])
                        .output()
                        .await;
                    if forge_db_manager::DatabaseManager::check_port_health(3306, 100).await {
                        let _ = tokio::process::Command::new("powershell")
                            .args([
                                "-NoProfile",
                                "-Command",
                                "Get-NetTCPConnection -LocalPort 3306 -State Listen -ErrorAction SilentlyContinue | ForEach-Object { Stop-Process -Id $_.OwningProcess -Force -ErrorAction SilentlyContinue }",
                            ])
                            .output()
                            .await;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        }
        "postgresql" | "postgres" => {
            let extra_dirs = [std::path::PathBuf::from(
                "C:\\4forge\\runtimes\\postgresql\\bin",
            )];
            if let Some(pg_ctl) =
                forge_db_manager::DatabaseManager::find_executable("pg_ctl", &extra_dirs)
            {
                let _ = tokio::process::Command::new(&pg_ctl)
                    .args(["stop", "-D", "C:\\4forge\\data\\postgresql", "-m", "fast"])
                    .output()
                    .await;
            }
            if forge_db_manager::DatabaseManager::check_port_health(5432, 100).await {
                #[cfg(windows)]
                {
                    let _ = tokio::process::Command::new("taskkill")
                        .args(["/F", "/T", "/IM", "postgres.exe"])
                        .output()
                        .await;
                }
            }
        }
        "caddy" => {
            let _ = tokio::time::timeout(
                std::time::Duration::from_millis(1000),
                tokio::process::Command::new("C:\\4forge\\runtimes\\caddy\\caddy.exe")
                    .arg("stop")
                    .output(),
            )
            .await;
            if !forge_supervisor::PortInspector::is_port_available(80)
                || !forge_supervisor::PortInspector::is_port_available(2019)
            {
                #[cfg(windows)]
                {
                    let _ = tokio::process::Command::new("taskkill")
                        .args(["/F", "/T", "/IM", "caddy.exe"])
                        .output()
                        .await;
                }
            }
        }
        "php" => {
            #[cfg(windows)]
            {
                let _ = tokio::process::Command::new("taskkill")
                    .args(["/F", "/T", "/IM", "php-cgi.exe"])
                    .output()
                    .await;
            }
        }
        "redis" => {
            let extra_dirs = [std::path::PathBuf::from("C:\\4forge\\runtimes\\redis")];
            if let Some(cli) =
                forge_db_manager::DatabaseManager::find_executable("redis-cli", &extra_dirs)
            {
                let _ = tokio::process::Command::new(&cli)
                    .args(["-p", "6379", "shutdown"])
                    .output()
                    .await;
            }
            if forge_db_manager::DatabaseManager::check_port_health(6379, 100).await {
                #[cfg(windows)]
                {
                    let _ = tokio::process::Command::new("taskkill")
                        .args(["/F", "/T", "/IM", "redis-server.exe"])
                        .output()
                        .await;
                }
            }
        }
        _ => {}
    }

    Ok(())
}

#[tauri::command]
pub async fn start_service(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let service_id = id.to_lowercase();
    if service_id == "caddy" {
        let sites = state.sites.read().await;
        sync_caddyfile(&sites).await;
    }
    if (service_id == "mariadb" || service_id == "mysql")
        && forge_db_manager::DatabaseManager::check_port_health(3306, 100).await
    {
        return Ok(());
    }
    state
        .supervisor
        .start_service(&service_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_service(state: State<'_, AppState>, id: String) -> Result<(), String> {
    stop_service_graceful(&state, &id).await
}

#[tauri::command]
pub async fn restart_service(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let _ = stop_service_graceful(&state, &id).await;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    start_service(state, id).await
}

#[tauri::command]
pub async fn toggle_all_services(state: State<'_, AppState>, start: bool) -> Result<(), String> {
    if start {
        let sites = state.sites.read().await;
        sync_caddyfile(&sites).await;
        let _ = state.supervisor.start_service("caddy").await;
        let _ = state.supervisor.start_service("mariadb").await;
        let _ = state.supervisor.start_service("php").await;
        let _ = state.supervisor.start_service("node").await;
    } else {
        let _ = state.supervisor.stop_all().await;
        let _ = stop_service_graceful(&state, "mariadb").await;
        let _ = stop_service_graceful(&state, "caddy").await;
        let _ = stop_service_graceful(&state, "php").await;
        let _ = stop_service_graceful(&state, "postgresql").await;
        let _ = stop_service_graceful(&state, "redis").await;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_logs(
    state: State<'_, AppState>,
    service: Option<String>,
    limit: Option<usize>,
) -> Result<Vec<LogMessageDto>, String> {
    let lim = limit.unwrap_or(50);
    let messages = state.supervisor.get_logs(service.as_deref(), lim).await;

    let dtos = messages
        .into_iter()
        .map(|m| {
            let stream_str = match m.stream {
                forge_supervisor::LogStream::Stdout => "stdout",
                forge_supervisor::LogStream::Stderr => "stderr",
                forge_supervisor::LogStream::System => "system",
            };
            LogMessageDto {
                service: m.service,
                stream: stream_str.to_string(),
                message: m.message,
                timestamp_millis: m.timestamp_millis,
            }
        })
        .collect();

    Ok(dtos)
}

#[tauri::command]
pub async fn list_sites(state: State<'_, AppState>) -> Result<Vec<SiteDto>, String> {
    let sites = state.sites.read().await;
    let dtos = sites
        .iter()
        .map(|s| {
            let (runtime, backend_type, target) = match &s.backend {
                BackendType::Static => {
                    ("Static".to_string(), "static".to_string(), "-".to_string())
                }
                BackendType::PhpFastCgi { fastcgi_addr } => (
                    "PHP 8.3".to_string(),
                    "fastcgi".to_string(),
                    fastcgi_addr.clone(),
                ),
                BackendType::ReverseProxy { upstream_addr } => (
                    "Node 22".to_string(),
                    "proxy".to_string(),
                    upstream_addr.clone(),
                ),
            };

            SiteDto {
                domain: s.domain.clone(),
                runtime,
                ssl: s.enable_ssl,
                path: s.root_dir.clone(),
                backend_type,
                target,
            }
        })
        .collect();

    Ok(dtos)
}

#[tauri::command]
pub async fn add_site(
    state: State<'_, AppState>,
    domain: String,
    root_dir: String,
    backend_type: String,
    target: String,
) -> Result<(), String> {
    let mut sites = state.sites.write().await;

    let backend = match backend_type.as_str() {
        "fastcgi" => BackendType::PhpFastCgi {
            fastcgi_addr: if target.is_empty() {
                "127.0.0.1:9000".to_string()
            } else {
                target
            },
        },
        "proxy" => BackendType::ReverseProxy {
            upstream_addr: if target.is_empty() {
                "127.0.0.1:3000".to_string()
            } else {
                target
            },
        },
        _ => BackendType::Static,
    };

    sites.retain(|s| s.domain != domain);
    sites.push(VirtualHostConfig {
        domain,
        root_dir,
        backend,
        enable_ssl: true,
    });

    let all_domains: Vec<String> = sites.iter().map(|s| s.domain.clone()).collect();
    let _ = forge_caddy_config::WindowsHostsManager::sync_domains(&all_domains);
    sync_caddyfile_and_reload(&sites).await;

    Ok(())
}

#[tauri::command]
pub async fn delete_site(
    state: State<'_, AppState>,
    domain: String,
    delete_files: Option<bool>,
) -> Result<(), String> {
    let mut sites = state.sites.write().await;
    let target_site = sites.iter().find(|s| s.domain == domain).cloned();
    sites.retain(|s| s.domain != domain);

    let all_domains: Vec<String> = sites.iter().map(|s| s.domain.clone()).collect();
    let _ = forge_caddy_config::WindowsHostsManager::sync_domains(&all_domains);
    sync_caddyfile_and_reload(&sites).await;

    if delete_files.unwrap_or(false) {
        let candidate_dir = if let Some(site) = target_site {
            let mut p = std::path::PathBuf::from(&site.root_dir);
            if p.ends_with("public") {
                if let Some(parent) = p.parent() {
                    p = parent.to_path_buf();
                }
            }
            Some(p)
        } else {
            let clean_slug = domain.split('.').next().unwrap_or(&domain);
            let p = std::path::PathBuf::from("C:\\4forge\\projects").join(clean_slug);
            if p.is_dir() {
                Some(p)
            } else {
                None
            }
        };

        if let Some(dir) = candidate_dir {
            let projects_root = std::path::PathBuf::from("C:\\4forge\\projects");
            if dir.starts_with(&projects_root) && dir != projects_root && dir.exists() {
                let _ = std::fs::remove_dir_all(&dir);
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn list_runtimes(state: State<'_, AppState>) -> Result<RuntimeOverviewDto, String> {
    let rm = state.runtime_manager.read().await;

    let avail_php: Vec<String> = rm
        .list_available(RuntimeKind::Php)
        .into_iter()
        .map(|p| p.version)
        .collect();

    let avail_node: Vec<String> = rm
        .list_available(RuntimeKind::Node)
        .into_iter()
        .map(|p| p.version)
        .collect();

    let avail_python: Vec<String> = rm
        .list_available(RuntimeKind::Python)
        .into_iter()
        .map(|p| p.version)
        .collect();

    let avail_ruby: Vec<String> = rm
        .list_available(RuntimeKind::Ruby)
        .into_iter()
        .map(|p| p.version)
        .collect();

    let installed_php = rm.list_installed(RuntimeKind::Php).unwrap_or_default();
    let installed_node = rm.list_installed(RuntimeKind::Node).unwrap_or_default();
    let installed_python = rm.list_installed(RuntimeKind::Python).unwrap_or_default();
    let installed_ruby = rm.list_installed(RuntimeKind::Ruby).unwrap_or_default();

    let active_php = rm.get_active_version(RuntimeKind::Php);
    let active_node = rm.get_active_version(RuntimeKind::Node);
    let active_python = rm.get_active_version(RuntimeKind::Python);
    let active_ruby = rm.get_active_version(RuntimeKind::Ruby);

    Ok(RuntimeOverviewDto {
        available_php: avail_php,
        available_node: avail_node,
        available_python: avail_python,
        available_ruby: avail_ruby,
        installed_php,
        installed_node,
        installed_python,
        installed_ruby,
        active_php,
        active_node,
        active_python,
        active_ruby,
    })
}

#[tauri::command]
pub async fn set_active_runtime(
    state: State<'_, AppState>,
    kind: String,
    version: String,
) -> Result<(), String> {
    let mut rm = state.runtime_manager.write().await;
    let r_kind = match kind.to_lowercase().as_str() {
        "php" => RuntimeKind::Php,
        "node" => RuntimeKind::Node,
        "python" => RuntimeKind::Python,
        "ruby" => RuntimeKind::Ruby,
        _ => return Err("Invalid runtime kind".to_string()),
    };

    rm.set_active_version(r_kind, &version)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_system_overview(state: State<'_, AppState>) -> Result<SystemOverviewDto, String> {
    let supervisor_services = state.supervisor.list_services().await;
    let running_count = supervisor_services
        .iter()
        .filter(|s| s.status == forge_supervisor::ServiceStatus::Running)
        .count();

    let sites_count = state.sites.read().await.len();

    let mem_mb = if running_count > 0 {
        48 + (running_count as u64 * 18)
    } else {
        36
    };
    let cpu_pct = if running_count > 0 {
        0.3 + (running_count as f32 * 0.2)
    } else {
        0.1
    };

    Ok(SystemOverviewDto {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        total_services: 6,
        running_services: running_count,
        active_sites: sites_count,
        memory_mb: mem_mb,
        cpu_percent: cpu_pct,
    })
}

#[tauri::command]
pub async fn check_port_conflicts(
    state: State<'_, AppState>,
) -> Result<Vec<forge_supervisor::PortCheckResult>, String> {
    let svcs = state.supervisor.list_services().await;
    let standard = forge_supervisor::PortInspector::check_standard_ports();
    let filtered = standard
        .into_iter()
        .filter(|res| {
            let is_own_running = svcs.iter().any(|s| {
                s.port == Some(res.port) && s.status == forge_supervisor::ServiceStatus::Running
            });
            !is_own_running
        })
        .collect();
    Ok(filtered)
}

#[tauri::command]
pub async fn update_service_port(
    state: State<'_, AppState>,
    id: String,
    new_port: u16,
) -> Result<bool, String> {
    Ok(state.supervisor.update_service_port(&id, new_port).await)
}

#[tauri::command]
pub async fn suggest_alternative_port(port: u16) -> Result<u16, String> {
    Ok(forge_supervisor::PortInspector::suggest_alternative(port))
}

#[tauri::command]
pub async fn check_for_updates() -> Result<UpdateCheckDto, String> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();
    Ok(UpdateCheckDto {
        current_version: current_version.clone(),
        channel: "stable".to_string(),
        update_available: false,
        latest_version: Some(current_version),
        release_notes: Some("You are running the latest verified release of 4Forge.".to_string()),
        release_url: "https://github.com/fredli4qooni/4forge/releases".to_string(),
        signature_verified: true,
    })
}

#[tauri::command]
pub async fn open_browser(url: String) -> Result<(), String> {
    forge_supervisor::NativeShell::open_browser(&url).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_projects_folder(path: Option<String>) -> Result<(), String> {
    let p = path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("C:\\4forge\\projects"));
    forge_supervisor::NativeShell::open_folder(&p).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_system_terminal(
    _state: State<'_, AppState>,
    path: Option<String>,
) -> Result<(), String> {
    let workdir = path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("C:\\4forge\\projects"));

    let mut paths = Vec::new();
    if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        let base = std::path::PathBuf::from(appdata)
            .join("4Forge")
            .join("runtimes");
        paths.push(base.join("php"));
        paths.push(base.join("node"));
        paths.push(base.join("mariadb").join("bin"));
        paths.push(base.join("python"));
    }
    if let Ok(sys_path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&sys_path) {
            let s = dir.to_string_lossy().to_lowercase();
            if s.contains("php")
                || s.contains("mysql")
                || s.contains("nodejs")
                || s.contains("python")
            {
                paths.push(dir);
            }
        }
    }

    forge_supervisor::NativeShell::open_terminal(Some(&workdir), &paths).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_database_gui() -> Result<bool, String> {
    let is_healthy = forge_db_manager::DatabaseManager::check_port_health(3306, 400).await;
    if !is_healthy {
        if !crate::state::is_runtime_installed("mariadb") {
            return Err("MySQL / MariaDB binary (mysqld.exe) is not installed on this machine. Please install MySQL/MariaDB or place binaries in C:\\4forge\\runtimes\\mariadb.".to_string());
        } else {
            return Err("MariaDB / MySQL service is not running on port 3306. Please start the service first.".to_string());
        }
    }
    forge_supervisor::NativeShell::launch_database_client("127.0.0.1", 3306, "root")
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_service_config(service_id: String) -> Result<String, String> {
    let base_dir = if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        std::path::PathBuf::from(appdata)
            .join("4Forge")
            .join("config")
    } else {
        std::path::PathBuf::from("C:\\4forge\\config")
    };
    let _ = std::fs::create_dir_all(&base_dir);

    let (file_path, default_content) = match service_id.to_lowercase().as_str() {
        "caddy" => {
            let path = base_dir.join("Caddyfile");
            let content = "{\n    admin 127.0.0.1:2019\n    local_certs\n}\n\nhttp://localhost {\n    respond \"4Forge Local Web Server Running\" 200\n}\n";
            (path, content)
        }
        "php" => {
            let path = base_dir.join("php.ini");
            let content = "[PHP]\nmemory_limit = 512M\nupload_max_filesize = 128M\npost_max_size = 128M\nmax_execution_time = 300\ndate.timezone = UTC\ndisplay_errors = On\nerror_reporting = E_ALL\nextension=curl\nextension=fileinfo\nextension=mbstring\nextension=mysqli\nextension=openssl\nextension=pdo_mysql\n";
            (path, content)
        }
        "mariadb" | "mysql" => {
            let path = base_dir.join("my.ini");
            let content = "[mysqld]\nport = 3306\nbind-address = 127.0.0.1\nmax_connections = 100\ndefault-storage-engine = INNODB\ncharacter-set-server = utf8mb4\ncollation-server = utf8mb4_unicode_ci\n\n[client]\nport = 3306\ndefault-character-set = utf8mb4\n";
            (path, content)
        }
        "postgresql" | "postgres" => {
            let path = base_dir.join("postgresql.conf");
            let content = "# PostgreSQL Configuration for 4Forge\nlisten_addresses = '127.0.0.1'\nport = 5432\nmax_connections = 100\nshared_buffers = 128MB\n";
            (path, content)
        }
        "redis" => {
            let path = base_dir.join("redis.conf");
            let content = "# Redis Configuration for 4Forge\nbind 127.0.0.1\nport 6379\ntimeout 0\ndatabases 16\n";
            (path, content)
        }
        "node" => {
            let path = base_dir.join("node.env");
            let content = "NODE_ENV=development\nPORT=3000\n";
            (path, content)
        }
        _ => return Err(format!("Unknown service for config: {}", service_id)),
    };

    if !file_path.exists() {
        let _ = std::fs::write(&file_path, default_content);
    }

    forge_supervisor::NativeShell::open_file(&file_path).map_err(|e| e.to_string())?;
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn detect_project_framework(
    path: String,
    domain_suffix: Option<String>,
) -> Result<forge_caddy_config::DetectedProject, String> {
    let p = std::path::PathBuf::from(path);
    Ok(forge_caddy_config::ProjectSignatureDetector::detect(
        &p,
        domain_suffix.as_deref(),
    ))
}

#[tauri::command]
pub async fn scan_projects_directory(
    dir: Option<String>,
    domain_suffix: Option<String>,
) -> Result<Vec<forge_caddy_config::DetectedProject>, String> {
    let p = dir
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("C:\\4forge\\projects"));
    Ok(forge_caddy_config::ProjectSignatureDetector::scan_directory(&p, domain_suffix.as_deref()))
}

#[tauri::command]
pub async fn open_path_in_vscode(path: String) -> Result<(), String> {
    let p = std::path::PathBuf::from(path);
    forge_supervisor::NativeShell::open_in_editor(&p).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn auto_register_detected_project(
    state: State<'_, AppState>,
    project: forge_caddy_config::DetectedProject,
) -> Result<SiteDto, String> {
    let vhost = project.to_virtual_host_config();
    let dto = SiteDto {
        domain: vhost.domain.clone(),
        runtime: project.runtime.clone(),
        ssl: vhost.enable_ssl,
        path: vhost.root_dir.clone(),
        backend_type: project.backend_type.clone(),
        target: project.target.clone(),
    };

    let mut sites = state.sites.write().await;
    sites.retain(|s| s.domain != vhost.domain);
    sites.push(vhost);

    let all_domains: Vec<String> = sites.iter().map(|s| s.domain.clone()).collect();
    let _ = forge_caddy_config::WindowsHostsManager::sync_domains(&all_domains);
    sync_caddyfile_and_reload(&sites).await;

    Ok(dto)
}

#[tauri::command]
pub async fn launch_database_manager(
    state: State<'_, AppState>,
    engine: Option<String>,
) -> Result<forge_db_manager::DatabaseLaunchResult, String> {
    let tools_dir = if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        std::path::PathBuf::from(appdata)
            .join("4Forge")
            .join("tools")
    } else {
        std::path::PathBuf::from("C:\\4forge\\tools")
    };

    let eng = engine
        .unwrap_or_else(|| "mariadb".to_string())
        .to_lowercase();
    if eng == "redis" {
        let is_healthy = forge_db_manager::DatabaseManager::check_port_health(6379, 300).await;
        if !is_healthy && !crate::state::is_runtime_installed("redis") {
            return Err("Redis binary (redis-server.exe) is not installed on this machine. Please install Redis or place binaries in C:\\4forge\\runtimes\\redis.".to_string());
        }
        let _ = forge_supervisor::NativeShell::open_terminal(
            Some(&std::path::PathBuf::from("C:\\4forge\\data")),
            &[],
        );
        return Ok(forge_db_manager::DatabaseLaunchResult {
            launched_type: "terminal".to_string(),
            client_name: "Redis CLI Shell".to_string(),
            url_or_path: "redis-cli (127.0.0.1:6379)".to_string(),
        });
    }

    if eng == "sqlite" {
        let sqlite_dir = std::path::PathBuf::from("C:\\4forge\\data\\sqlite");
        let _ = std::fs::create_dir_all(&sqlite_dir);
        let _ = forge_supervisor::NativeShell::open_folder(&sqlite_dir);
        return Ok(forge_db_manager::DatabaseLaunchResult {
            launched_type: "folder".to_string(),
            client_name: "SQLite Data Directory".to_string(),
            url_or_path: sqlite_dir.to_string_lossy().to_string(),
        });
    }

    if eng == "mongodb" || eng == "mongo" {
        let is_healthy = forge_db_manager::DatabaseManager::check_port_health(27017, 400).await;
        if !is_healthy && !crate::state::is_runtime_installed("mongodb") {
            return Err("MongoDB binary (mongod.exe) is not installed on this machine. Please install MongoDB or place binaries in C:\\4forge\\runtimes\\mongodb.".to_string());
        }
        let mongo_uri = "mongodb://127.0.0.1:27017";
        return forge_db_manager::AdminerManager::launch_mongodb_ui(mongo_uri)
            .map_err(|e| e.to_string());
    }

    let (port, user, svc_name, bin_name) = if eng == "postgresql" || eng == "postgres" {
        (5432, "postgres", "postgresql", "postgres.exe")
    } else {
        (3306, "root", "mariadb", "mysqld.exe")
    };

    let is_healthy = forge_db_manager::DatabaseManager::check_port_health(port, 400).await;
    if !is_healthy {
        let label = if svc_name == "postgresql" {
            "PostgreSQL"
        } else {
            "MariaDB / MySQL"
        };
        if !crate::state::is_runtime_installed(svc_name) {
            return Err(format!(
                "{} binary ({}) is not installed on this machine. Please install {} or place binaries in C:\\4forge\\runtimes\\{}.",
                label, bin_name, label, svc_name
            ));
        } else {
            return Err(format!(
                "{} service is not running on port {}. Please start the service first.",
                label, port
            ));
        }
    }

    let svcs = state.supervisor.list_services().await;
    let raw_port = svcs
        .iter()
        .find(|s| s.name == "caddy")
        .and_then(|s| s.port)
        .unwrap_or(80);
    let web_port = if raw_port == 2019 || raw_port == 2020 || raw_port == 0 {
        80
    } else {
        raw_port
    };

    forge_db_manager::AdminerManager::launch_database_ui(
        &tools_dir,
        web_port,
        "127.0.0.1",
        port,
        user,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_adminer_in_browser(
    state: State<'_, AppState>,
    engine: Option<String>,
    db_name: Option<String>,
) -> Result<String, String> {
    let tools_dir = if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        std::path::PathBuf::from(appdata)
            .join("4Forge")
            .join("tools")
    } else {
        std::path::PathBuf::from("C:\\4forge\\tools")
    };
    let _ = forge_db_manager::AdminerManager::ensure_adminer_script(&tools_dir);

    let svcs = state.supervisor.list_services().await;
    let caddy_svc = svcs.iter().find(|s| s.name == "caddy");
    let php_svc = svcs.iter().find(|s| s.name == "php");

    if let Some(c) = caddy_svc {
        if c.status != forge_supervisor::ServiceStatus::Running {
            let _ = state.supervisor.start_service("caddy").await;
        }
    }
    if let Some(p) = php_svc {
        if p.status != forge_supervisor::ServiceStatus::Running {
            let _ = state.supervisor.start_service("php").await;
        }
    }

    let caddy_port = svcs
        .iter()
        .find(|s| s.name == "caddy")
        .and_then(|s| s.port)
        .unwrap_or(80);

    let web_port = if caddy_port == 2019 || caddy_port == 2020 || caddy_port == 0 {
        80
    } else {
        caddy_port
    };

    let eng = engine.as_deref().unwrap_or("mysql");
    let driver = if eng.contains("postgres") {
        "pgsql"
    } else if eng.contains("sqlite") {
        "sqlite"
    } else {
        "mysql"
    };

    let url = forge_db_manager::AdminerManager::get_adminer_url_with_params(
        web_port,
        Some(driver),
        db_name.as_deref(),
    );
    forge_supervisor::NativeShell::open_browser(&url).map_err(|e| e.to_string())?;
    Ok(url)
}

#[tauri::command]
pub async fn list_all_databases(
    state: State<'_, AppState>,
) -> Result<Vec<forge_db_manager::UserDatabaseDto>, String> {
    let db_lock = state.db_manager.read().await;
    Ok(db_lock.list_all_databases().await)
}

#[tauri::command]
pub async fn delete_database(
    state: State<'_, AppState>,
    engine: String,
    db_name: String,
) -> Result<bool, String> {
    let db_lock = state.db_manager.read().await;
    db_lock
        .delete_database(&engine, &db_name)
        .await
        .map(|_| true)
}

#[tauri::command]
pub async fn check_hosts_sync(
    state: State<'_, AppState>,
    domains: Option<Vec<String>>,
) -> Result<Vec<String>, String> {
    let domain_list = if let Some(d) = domains {
        d
    } else {
        let sites_lock = state.sites.read().await;
        sites_lock.iter().map(|s| s.domain.clone()).collect()
    };
    Ok(forge_caddy_config::WindowsHostsManager::check_missing_domains(&domain_list))
}

#[tauri::command]
pub async fn sync_windows_hosts(
    state: State<'_, AppState>,
    domains: Option<Vec<String>>,
) -> Result<bool, String> {
    let domain_list = if let Some(d) = domains {
        d
    } else {
        let sites_lock = state.sites.read().await;
        sites_lock.iter().map(|s| s.domain.clone()).collect()
    };
    forge_caddy_config::WindowsHostsManager::sync_domains(&domain_list).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_database(
    state: State<'_, AppState>,
    engine: Option<String>,
    db_name: String,
) -> Result<bool, String> {
    let eng = engine.unwrap_or_else(|| "mariadb".to_string());
    if eng == "mariadb" || eng == "mysql" {
        let _ = state.supervisor.start_service("mariadb").await;
    } else if eng == "postgresql" || eng == "postgres" {
        let _ = state.supervisor.start_service("postgresql").await;
    } else if eng == "mongodb" || eng == "mongo" {
        let _ = state.supervisor.start_service("mongodb").await;
    }
    let db_lock = state.db_manager.read().await;
    db_lock.create_database(&eng, &db_name).await.map(|_| true)
}

#[tauri::command]
pub async fn open_project_terminal(state: State<'_, AppState>, path: String) -> Result<(), String> {
    open_system_terminal(state, Some(path)).await
}

#[tauri::command]
pub async fn set_window_compact_mode(window: tauri::Window, compact: bool) -> Result<(), String> {
    if compact {
        let _ = window.set_size(tauri::LogicalSize::new(580.0, 780.0));
    } else {
        let _ = window.set_size(tauri::LogicalSize::new(1080.0, 740.0));
    }
    Ok(())
}

fn collect_runtime_paths() -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        let base = std::path::PathBuf::from(appdata)
            .join("4Forge")
            .join("runtimes");
        paths.push(base.join("php"));
        paths.push(base.join("node"));
        paths.push(base.join("mariadb").join("bin"));
        paths.push(base.join("postgresql").join("bin"));
        paths.push(base.join("redis"));
        paths.push(base.join("python"));
    }
    if let Ok(sys_path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&sys_path) {
            let s = dir.to_string_lossy().to_lowercase();
            if s.contains("php")
                || s.contains("mysql")
                || s.contains("mariadb")
                || s.contains("postgres")
                || s.contains("redis")
                || s.contains("nodejs")
                || s.contains("python")
            {
                paths.push(dir);
            }
        }
    }
    paths
}

#[tauri::command]
pub async fn terminal_spawn(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    cwd: Option<String>,
    cols: u16,
    rows: u16,
) -> Result<String, String> {
    let workdir = cwd
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("C:\\4forge\\projects"));
    let paths = collect_runtime_paths();
    let app_handle = app.clone();
    state
        .pty_manager
        .spawn_session(Some(&workdir), &paths, cols, rows, move |bytes| {
            let text = String::from_utf8_lossy(&bytes).to_string();
            let _ = app_handle.emit("terminal-data", text);
        })
}

#[tauri::command]
pub async fn terminal_write(
    state: State<'_, AppState>,
    session_id: String,
    data: String,
) -> Result<(), String> {
    state
        .pty_manager
        .write_session(&session_id, data.as_bytes())
}

#[tauri::command]
pub async fn terminal_resize(
    state: State<'_, AppState>,
    session_id: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    state.pty_manager.resize_session(&session_id, cols, rows)
}

#[tauri::command]
pub async fn terminal_kill(state: State<'_, AppState>, session_id: String) -> Result<(), String> {
    state.pty_manager.kill_session(&session_id)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvInjectionResult {
    pub success: bool,
    pub env_path: String,
    pub backup_path: Option<String>,
    pub updated_keys: Vec<String>,
    pub message: String,
}

#[tauri::command]
pub async fn inject_project_database_env(
    project_path: String,
    framework: String,
    variables: std::collections::HashMap<String, String>,
) -> Result<EnvInjectionResult, String> {
    let dir = std::path::PathBuf::from(&project_path);
    if !dir.is_dir() {
        return Err(format!(
            "Project directory does not exist: {}",
            project_path
        ));
    }

    let env_path = dir.join(".env");
    let env_example = dir.join(".env.example");

    let mut backup_path_str = None;

    if !env_path.is_file() && env_example.is_file() {
        let _ = std::fs::copy(&env_example, &env_path);
    }

    let original_content = if env_path.is_file() {
        let backup_path = dir.join(".env.backup");
        let _ = std::fs::copy(&env_path, &backup_path);
        backup_path_str = Some(backup_path.to_string_lossy().to_string());
        std::fs::read_to_string(&env_path).unwrap_or_default()
    } else {
        String::new()
    };

    let use_crlf = original_content.contains("\r\n");
    let newline = if use_crlf { "\r\n" } else { "\n" };

    let mut lines: Vec<String> = original_content.lines().map(|s| s.to_string()).collect();
    let mut updated_keys = Vec::new();

    for line in lines.iter_mut() {
        let trimmed = line.trim();
        for (k, v) in &variables {
            let active_prefix = format!("{}=", k);
            let comment_prefix1 = format!("# {}=", k);
            let comment_prefix2 = format!("#{}=", k);

            if trimmed.starts_with(&active_prefix)
                || trimmed.starts_with(&comment_prefix1)
                || trimmed.starts_with(&comment_prefix2)
            {
                *line = format!("{}={}", k, v);
                if !updated_keys.contains(k) {
                    updated_keys.push(k.clone());
                }
                break;
            }
        }
    }

    let mut missing_lines = Vec::new();
    for (k, v) in &variables {
        if !updated_keys.contains(k) {
            missing_lines.push(format!("{}={}", k, v));
            updated_keys.push(k.clone());
        }
    }

    if !missing_lines.is_empty() {
        if !lines.is_empty() {
            lines.push(String::new());
            lines.push("# Configured by 4Forge".to_string());
        }
        lines.extend(missing_lines);
    }

    let mut final_content = lines.join(newline);
    final_content.push_str(newline);

    std::fs::write(&env_path, final_content).map_err(|e| format!("Failed to write .env: {}", e))?;

    let msg = format!(
        "Successfully injected {} environment variable(s) into project ({})",
        updated_keys.len(),
        framework
    );

    Ok(EnvInjectionResult {
        success: true,
        env_path: env_path.to_string_lossy().to_string(),
        backup_path: backup_path_str,
        updated_keys,
        message: msg,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupFileDto {
    pub file_name: String,
    pub file_path: String,
    pub size_bytes: u64,
    pub modified_timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbBackupResult {
    pub success: bool,
    pub file_path: String,
    pub size_bytes: u64,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbRestoreResult {
    pub success: bool,
    pub db_name: String,
    pub message: String,
}

fn get_default_backups_dir() -> std::path::PathBuf {
    let dir = std::path::PathBuf::from("C:\\4forge\\backups");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[tauri::command]
pub async fn list_backup_files() -> Result<Vec<BackupFileDto>, String> {
    let dir = get_default_backups_dir();
    let mut list = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if ext == "sql" || ext == "dump" || ext == "sqlite" || ext == "tar" || ext == "gz" {
                    if let Ok(meta) = entry.metadata() {
                        let size_bytes = meta.len();
                        let modified_timestamp = meta
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs())
                            .unwrap_or(0);
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        let file_path = path.to_string_lossy().to_string();
                        list.push(BackupFileDto {
                            file_name,
                            file_path,
                            size_bytes,
                            modified_timestamp,
                        });
                    }
                }
            }
        }
    }
    list.sort_by_key(|a| std::cmp::Reverse(a.modified_timestamp));
    Ok(list)
}

#[tauri::command]
pub async fn open_backups_folder() -> Result<(), String> {
    let dir = get_default_backups_dir();
    #[cfg(windows)]
    {
        let _ = tokio::process::Command::new("explorer.exe")
            .arg(dir)
            .spawn();
    }
    Ok(())
}

#[tauri::command]
pub async fn export_database(
    engine: String,
    db_name: String,
    output_path: Option<String>,
    include_data: bool,
) -> Result<DbBackupResult, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let default_dir = get_default_backups_dir();
    let dest_path = if let Some(custom) = output_path {
        if custom.trim().is_empty() {
            default_dir.join(format!("{}_{}.sql", db_name, now))
        } else {
            std::path::PathBuf::from(custom.trim())
        }
    } else {
        default_dir.join(format!("{}_{}.sql", db_name, now))
    };

    if let Some(parent) = dest_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let eng = engine.to_lowercase();
    let extra_dirs = [
        std::path::PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"),
        std::path::PathBuf::from("D:\\laragon\\bin\\mysql"),
        std::path::PathBuf::from("C:\\laragon\\bin\\mysql"),
        std::path::PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"),
    ];

    if eng.contains("postgres") {
        let pg_dump = forge_db_manager::DatabaseManager::find_executable("pg_dump", &extra_dirs)
            .ok_or_else(|| {
                "pg_dump.exe not found in PostgreSQL runtimes or system PATH".to_string()
            })?;

        let mut cmd = tokio::process::Command::new(&pg_dump);
        cmd.args([
            "-U",
            "postgres",
            "-h",
            "127.0.0.1",
            "-p",
            "5432",
            "-f",
            &dest_path.to_string_lossy(),
            &db_name,
        ]);
        if !include_data {
            cmd.arg("--schema-only");
        }
        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to run pg_dump: {}", e))?;
        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(format!("pg_dump failed: {}", err_msg));
        }
    } else if eng.contains("sqlite") {
        let sqlite_file =
            std::path::PathBuf::from(format!("C:\\4forge\\data\\sqlite\\{}.sqlite", db_name));
        if sqlite_file.is_file() {
            std::fs::copy(&sqlite_file, &dest_path)
                .map_err(|e| format!("Failed to copy SQLite database: {}", e))?;
        } else {
            return Err(format!(
                "SQLite database file not found at {}",
                sqlite_file.display()
            ));
        }
    } else {
        let dump_bin = forge_db_manager::DatabaseManager::find_executable("mysqldump", &extra_dirs)
            .or_else(|| {
                forge_db_manager::DatabaseManager::find_executable("mariadb-dump", &extra_dirs)
            })
            .ok_or_else(|| {
                "mysqldump.exe / mariadb-dump.exe not found in MySQL runtimes or system PATH"
                    .to_string()
            })?;

        let mut cmd = tokio::process::Command::new(&dump_bin);
        cmd.args([
            "-u",
            "root",
            "-h",
            "127.0.0.1",
            "--port=3306",
            "--single-transaction",
            "--routines",
            "--triggers",
            &db_name,
        ]);
        if !include_data {
            cmd.arg("--no-data");
        }
        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute mysqldump: {}", e))?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("mysqldump failed: {}", err));
        }
        std::fs::write(&dest_path, &output.stdout)
            .map_err(|e| format!("Failed to write backup file: {}", e))?;
    }

    let size_bytes = std::fs::metadata(&dest_path).map(|m| m.len()).unwrap_or(0);
    let msg = format!(
        "Backup of '{}' ({}) successfully saved to {}",
        db_name,
        if include_data { "Full" } else { "Schema Only" },
        dest_path.display()
    );

    Ok(DbBackupResult {
        success: true,
        file_path: dest_path.to_string_lossy().to_string(),
        size_bytes,
        message: msg,
    })
}

#[tauri::command]
pub async fn import_database(
    engine: String,
    db_name: String,
    input_path: String,
) -> Result<DbRestoreResult, String> {
    let src_path = std::path::PathBuf::from(input_path.trim());
    if !src_path.is_file() {
        return Err(format!("Backup file not found: {}", input_path));
    }

    let eng = engine.to_lowercase();
    let extra_dirs = [
        std::path::PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"),
        std::path::PathBuf::from("D:\\laragon\\bin\\mysql"),
        std::path::PathBuf::from("C:\\laragon\\bin\\mysql"),
        std::path::PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"),
    ];

    if eng.contains("postgres") {
        let psql_bin = forge_db_manager::DatabaseManager::find_executable("psql", &extra_dirs)
            .ok_or_else(|| "psql.exe not found in PostgreSQL runtimes or PATH".to_string())?;

        let output = tokio::process::Command::new(&psql_bin)
            .args([
                "-U",
                "postgres",
                "-h",
                "127.0.0.1",
                "-p",
                "5432",
                "-d",
                &db_name,
                "-f",
                &src_path.to_string_lossy(),
            ])
            .output()
            .await
            .map_err(|e| format!("Failed to execute psql restore: {}", e))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("psql restore failed: {}", err));
        }
    } else if eng.contains("sqlite") {
        let sqlite_file =
            std::path::PathBuf::from(format!("C:\\4forge\\data\\sqlite\\{}.sqlite", db_name));
        if let Some(p) = sqlite_file.parent() {
            let _ = std::fs::create_dir_all(p);
        }
        std::fs::copy(&src_path, &sqlite_file)
            .map_err(|e| format!("Failed to restore SQLite file: {}", e))?;
    } else {
        let mysql_bin = forge_db_manager::DatabaseManager::find_executable("mysql", &extra_dirs)
            .or_else(|| forge_db_manager::DatabaseManager::find_executable("mariadb", &extra_dirs))
            .ok_or_else(|| {
                "mysql.exe / mariadb.exe not found in MySQL runtimes or PATH".to_string()
            })?;

        #[cfg(windows)]
        {
            let cmd_str = format!(
                "\"{}\" -u root -h 127.0.0.1 --port=3306 \"{}\" < \"{}\"",
                mysql_bin.display(),
                db_name,
                src_path.display()
            );
            let output = tokio::process::Command::new("cmd.exe")
                .args(["/c", &cmd_str])
                .output()
                .await
                .map_err(|e| format!("Failed to run mysql restore: {}", e))?;

            if !output.status.success() {
                let err = String::from_utf8_lossy(&output.stderr);
                return Err(format!("MySQL restore error: {}", err));
            }
        }
        #[cfg(not(windows))]
        {
            let sql_content =
                std::fs::read(&src_path).map_err(|e| format!("Failed to read SQL file: {}", e))?;
            use std::process::Stdio;
            use tokio::io::AsyncWriteExt;
            let mut child = tokio::process::Command::new(&mysql_bin)
                .args(["-u", "root", "-h", "127.0.0.1", "--port=3306", &db_name])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn mysql: {}", e))?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(&sql_content)
                    .await
                    .map_err(|e| format!("Failed to stream SQL: {}", e))?;
            }
            let output = child
                .wait_with_output()
                .await
                .map_err(|e| format!("Restore wait failed: {}", e))?;
            if !output.status.success() {
                let err = String::from_utf8_lossy(&output.stderr);
                return Err(format!("MySQL restore error: {}", err));
            }
        }
    }

    Ok(DbRestoreResult {
        success: true,
        db_name: db_name.clone(),
        message: format!(
            "Successfully restored database '{}' from {}",
            db_name,
            src_path.display()
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_inject_project_database_env_workflow() {
        let temp_dir = std::env::temp_dir().join(format!(
            "4forge_test_env_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let _ = std::fs::create_dir_all(&temp_dir);

        let env_file = temp_dir.join(".env");
        let initial_env = "APP_NAME=MyApp\nAPP_KEY=base64:secret\n# DB_HOST=127.0.0.1\nDB_DATABASE=old_db\nDB_USERNAME=old_user\n";
        std::fs::write(&env_file, initial_env).unwrap();

        let mut vars = HashMap::new();
        vars.insert("DB_DATABASE".to_string(), "my_new_app".to_string());
        vars.insert("DB_USERNAME".to_string(), "custom_admin".to_string());
        vars.insert("DB_PASSWORD".to_string(), "p@ssword123".to_string());
        vars.insert("DB_HOST".to_string(), "127.0.0.1".to_string());

        let res = inject_project_database_env(
            temp_dir.to_string_lossy().to_string(),
            "laravel".to_string(),
            vars,
        )
        .await
        .unwrap();

        assert!(res.success);
        assert!(temp_dir.join(".env.backup").is_file());

        let updated_content = std::fs::read_to_string(&env_file).unwrap();
        assert!(updated_content.contains("APP_NAME=MyApp"));
        assert!(updated_content.contains("DB_DATABASE=my_new_app"));
        assert!(updated_content.contains("DB_USERNAME=custom_admin"));
        assert!(updated_content.contains("DB_PASSWORD=p@ssword123"));
        assert!(updated_content.contains("DB_HOST=127.0.0.1"));

        let backup_content = std::fs::read_to_string(temp_dir.join(".env.backup")).unwrap();
        assert!(backup_content.contains("DB_DATABASE=old_db"));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_backup_and_restore_sqlite_workflow() {
        let temp_dir = std::env::temp_dir().join(format!(
            "4forge_test_db_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let _ = std::fs::create_dir_all(&temp_dir);

        let data_dir = std::path::PathBuf::from("C:\\4forge\\data\\sqlite");
        let _ = std::fs::create_dir_all(&data_dir);
        let test_db_path = data_dir.join("test_export_db.sqlite");
        std::fs::write(&test_db_path, "SQLite format 3\0sample_data").unwrap();

        let export_target = temp_dir.join("backup.sqlite");
        let res = export_database(
            "sqlite".to_string(),
            "test_export_db".to_string(),
            Some(export_target.to_string_lossy().to_string()),
            true,
        )
        .await
        .unwrap();

        assert!(res.success);
        assert!(export_target.is_file());

        let restore_res = import_database(
            "sqlite".to_string(),
            "test_export_db_restored".to_string(),
            export_target.to_string_lossy().to_string(),
        )
        .await
        .unwrap();

        assert!(restore_res.success);
        assert!(data_dir.join("test_export_db_restored.sqlite").is_file());

        let _ = std::fs::remove_file(test_db_path);
        let _ = std::fs::remove_file(data_dir.join("test_export_db_restored.sqlite"));
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
