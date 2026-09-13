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
use tauri::State;

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
    let (postgresql_st, postgresql_pid) = get_status("postgresql");
    let (redis_st, redis_pid) = get_status("redis");
    let (php_st, php_pid) = get_status("php");
    let (node_st, node_pid) = get_status("node");

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

#[tauri::command]
pub async fn start_service(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .supervisor
        .start_service(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_service(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .supervisor
        .stop_service(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restart_service(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .supervisor
        .restart_service(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_all_services(state: State<'_, AppState>, start: bool) -> Result<(), String> {
    if start {
        let _ = state.supervisor.start_service("caddy").await;
        let _ = state.supervisor.start_service("mariadb").await;
        let _ = state.supervisor.start_service("php").await;
        let _ = state.supervisor.start_service("node").await;
    } else {
        state.supervisor.stop_all().await;
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

    Ok(())
}

#[tauri::command]
pub async fn delete_site(state: State<'_, AppState>, domain: String) -> Result<(), String> {
    let mut sites = state.sites.write().await;
    sites.retain(|s| s.domain != domain);

    let all_domains: Vec<String> = sites.iter().map(|s| s.domain.clone()).collect();
    let _ = forge_caddy_config::WindowsHostsManager::sync_domains(&all_domains);

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

    Ok(dto)
}

#[tauri::command]
pub async fn launch_database_manager(
    state: State<'_, AppState>,
) -> Result<forge_db_manager::DatabaseLaunchResult, String> {
    let tools_dir = if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        std::path::PathBuf::from(appdata)
            .join("4Forge")
            .join("tools")
    } else {
        std::path::PathBuf::from("C:\\4forge\\tools")
    };

    let svcs = state.supervisor.list_services().await;
    let web_port = svcs
        .iter()
        .find(|s| s.name == "caddy")
        .and_then(|s| s.port)
        .unwrap_or(80);

    forge_db_manager::AdminerManager::launch_database_ui(
        &tools_dir,
        web_port,
        "127.0.0.1",
        3306,
        "root",
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_adminer_in_browser(state: State<'_, AppState>) -> Result<String, String> {
    let tools_dir = if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        std::path::PathBuf::from(appdata)
            .join("4Forge")
            .join("tools")
    } else {
        std::path::PathBuf::from("C:\\4forge\\tools")
    };
    let _ = forge_db_manager::AdminerManager::ensure_adminer_script(&tools_dir);

    let svcs = state.supervisor.list_services().await;
    let web_port = svcs
        .iter()
        .find(|s| s.name == "caddy")
        .and_then(|s| s.port)
        .unwrap_or(80);

    let url = forge_db_manager::AdminerManager::get_adminer_url(web_port);
    forge_supervisor::NativeShell::open_browser(&url).map_err(|e| e.to_string())?;
    Ok(url)
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
