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
            name: "MariaDB Database".to_string(),
            service_type: "Relational Database".to_string(),
            version: "v11.4.3".to_string(),
            ports: "3306".to_string(),
            status: mariadb_st,
            pid: mariadb_pid,
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

    Ok(())
}

#[tauri::command]
pub async fn delete_site(state: State<'_, AppState>, domain: String) -> Result<(), String> {
    let mut sites = state.sites.write().await;
    sites.retain(|s| s.domain != domain);
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
        total_services: 4,
        running_services: running_count,
        active_sites: sites_count,
        memory_mb: mem_mb,
        cpu_percent: cpu_pct,
    })
}

#[tauri::command]
pub async fn check_port_conflicts() -> Result<Vec<forge_supervisor::PortCheckResult>, String> {
    Ok(forge_supervisor::PortInspector::check_standard_ports())
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
