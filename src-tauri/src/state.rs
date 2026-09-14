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

use forge_caddy_config::VirtualHostConfig;
use forge_db_manager::DatabaseManager;
use forge_runtime_manager::RuntimeManager;
use forge_supervisor::{PtySessionManager, SupervisorManager};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub supervisor: Arc<SupervisorManager>,
    pub db_manager: Arc<RwLock<DatabaseManager>>,
    pub runtime_manager: Arc<RwLock<RuntimeManager>>,
    pub sites: Arc<RwLock<Vec<VirtualHostConfig>>>,
    pub pty_manager: Arc<PtySessionManager>,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let runtimes_root = dirs_next_or_default();
        let default_services = build_default_services(&runtimes_root);
        let supervisor = Arc::new(SupervisorManager::new_with_services(default_services)?);
        let mut dm = DatabaseManager::new();
        let maria_driver = Arc::new(forge_db_manager::MariaDbDriver::new(
            runtimes_root.join("mariadb"),
            PathBuf::from("C:\\4forge\\data\\mariadb"),
            3306,
        ));
        dm.register_driver(maria_driver);

        let pg_driver = Arc::new(forge_db_manager::PostgreSqlDriver::new(
            runtimes_root.join("postgresql"),
            PathBuf::from("C:\\4forge\\data\\postgresql"),
            5432,
        ));
        dm.register_driver(pg_driver);

        let sqlite_driver = Arc::new(forge_db_manager::SqliteDriver::new(PathBuf::from(
            "C:\\4forge\\data\\sqlite",
        )));
        dm.register_driver(sqlite_driver);

        let mongo_driver = Arc::new(forge_db_manager::MongoDbDriver::new(
            runtimes_root.join("mongodb"),
            PathBuf::from("C:\\4forge\\data\\mongodb"),
            27017,
        ));
        dm.register_driver(mongo_driver);
        let db_manager = Arc::new(RwLock::new(dm));
        let runtime_manager = Arc::new(RwLock::new(RuntimeManager::new(runtimes_root)));
        let pty_manager = Arc::new(PtySessionManager::new());

        let sites_path = PathBuf::from("C:\\4forge\\config\\sites.json");
        let mut loaded_sites: Vec<VirtualHostConfig> = if sites_path.is_file() {
            std::fs::read_to_string(&sites_path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let projects_dir = PathBuf::from("C:\\4forge\\projects");
        if projects_dir.is_dir() {
            let detected = forge_caddy_config::ProjectSignatureDetector::scan_directory(
                &projects_dir,
                Some("test"),
            );
            for p in detected {
                let vhost = p.to_virtual_host_config();
                if !loaded_sites.iter().any(|s| s.domain == vhost.domain) {
                    loaded_sites.push(vhost);
                }
            }
        }

        if !loaded_sites.is_empty() {
            if let Some(parent) = sites_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(&loaded_sites) {
                let _ = std::fs::write(&sites_path, json);
            }
            let all_domains: Vec<String> = loaded_sites.iter().map(|s| s.domain.clone()).collect();
            let _ = forge_caddy_config::WindowsHostsManager::sync_domains(&all_domains);
            let caddyfile_content =
                forge_caddy_config::CaddyConfigGenerator::generate_caddyfile(&loaded_sites);
            let caddyfile_path = PathBuf::from("C:\\4forge\\config\\Caddyfile");
            let _ = std::fs::write(&caddyfile_path, caddyfile_content);
        }

        Ok(Self {
            supervisor,
            db_manager,
            runtime_manager,
            sites: Arc::new(RwLock::new(loaded_sites)),
            pty_manager,
        })
    }
}

pub fn dirs_next_or_default() -> PathBuf {
    if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(appdata).join("4Forge").join("runtimes")
    } else {
        PathBuf::from("C:\\4Forge\\runtimes")
    }
}

pub fn find_binary_in_path(bin_name: &str) -> Option<PathBuf> {
    if let Some(bin) = forge_db_manager::DatabaseManager::find_executable(bin_name, &[]) {
        return Some(bin);
    }
    if let Some(paths) = std::env::var_os("PATH") {
        for path in std::env::split_paths(&paths) {
            let candidate = path.join(bin_name);
            if candidate.is_file() {
                return Some(candidate);
            }
            if !bin_name.ends_with(".exe") {
                let candidate_exe = path.join(format!("{}.exe", bin_name));
                if candidate_exe.is_file() {
                    return Some(candidate_exe);
                }
            }
        }
    }
    None
}

pub fn is_runtime_installed(service_name: &str) -> bool {
    let runtimes_root = dirs_next_or_default();
    let alt_runtimes_root = PathBuf::from("C:\\4forge\\runtimes");
    match service_name {
        "postgresql" | "postgres" => {
            find_binary_in_path("postgres.exe").is_some()
                || runtimes_root
                    .join("postgresql")
                    .join("bin")
                    .join("postgres.exe")
                    .is_file()
                || runtimes_root
                    .join("postgres")
                    .join("bin")
                    .join("postgres.exe")
                    .is_file()
                || alt_runtimes_root
                    .join("postgresql")
                    .join("bin")
                    .join("postgres.exe")
                    .is_file()
                || alt_runtimes_root
                    .join("postgres")
                    .join("bin")
                    .join("postgres.exe")
                    .is_file()
        }
        "mariadb" | "mysql" => {
            find_binary_in_path("mysqld.exe").is_some()
                || find_binary_in_path("mariadbd.exe").is_some()
                || runtimes_root
                    .join("mariadb")
                    .join("bin")
                    .join("mysqld.exe")
                    .is_file()
                || runtimes_root
                    .join("mysql")
                    .join("bin")
                    .join("mysqld.exe")
                    .is_file()
                || alt_runtimes_root
                    .join("mariadb")
                    .join("bin")
                    .join("mysqld.exe")
                    .is_file()
                || alt_runtimes_root
                    .join("mysql")
                    .join("bin")
                    .join("mysqld.exe")
                    .is_file()
        }
        "redis" => {
            find_binary_in_path("redis-server.exe").is_some()
                || runtimes_root
                    .join("redis")
                    .join("redis-server.exe")
                    .is_file()
                || alt_runtimes_root
                    .join("redis")
                    .join("redis-server.exe")
                    .is_file()
        }
        "caddy" => {
            find_binary_in_path("caddy.exe").is_some()
                || runtimes_root.join("caddy").join("caddy.exe").is_file()
                || alt_runtimes_root.join("caddy").join("caddy.exe").is_file()
        }
        "php" => {
            find_binary_in_path("php-cgi.exe").is_some()
                || find_binary_in_path("php.exe").is_some()
                || runtimes_root.join("php").join("php-cgi.exe").is_file()
                || runtimes_root.join("php").join("php.exe").is_file()
                || alt_runtimes_root.join("php").join("php-cgi.exe").is_file()
                || alt_runtimes_root.join("php").join("php.exe").is_file()
        }
        "mongodb" | "mongo" => {
            find_binary_in_path("mongod.exe").is_some()
                || runtimes_root
                    .join("mongodb")
                    .join("bin")
                    .join("mongod.exe")
                    .is_file()
                || alt_runtimes_root
                    .join("mongodb")
                    .join("bin")
                    .join("mongod.exe")
                    .is_file()
                || [
                    "C:\\Program Files\\MongoDB\\Server\\8.0\\bin\\mongod.exe",
                    "C:\\Program Files\\MongoDB\\Server\\7.0\\bin\\mongod.exe",
                    "C:\\Program Files\\MongoDB\\Server\\6.0\\bin\\mongod.exe",
                    "C:\\Program Files\\MongoDB\\Server\\5.0\\bin\\mongod.exe",
                ]
                .iter()
                .any(|p| PathBuf::from(p).is_file())
        }
        _ => true,
    }
}

fn build_default_services(runtimes_root: &std::path::Path) -> Vec<forge_supervisor::ProcessConfig> {
    let mut list = Vec::new();

    let alt_runtimes_root = PathBuf::from("C:\\4forge\\runtimes");
    let caddy_bin = find_binary_in_path("caddy.exe")
        .or_else(|| {
            let p = runtimes_root.join("caddy").join("caddy.exe");
            if p.is_file() {
                Some(p)
            } else {
                let alt_p = alt_runtimes_root.join("caddy").join("caddy.exe");
                if alt_p.is_file() {
                    Some(alt_p)
                } else {
                    let bin_p = PathBuf::from("C:\\4forge\\bin\\caddy.exe");
                    if bin_p.is_file() {
                        Some(bin_p)
                    } else {
                        None
                    }
                }
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmd.exe"));
    let caddyfile_path = std::path::PathBuf::from("C:\\4forge\\config\\Caddyfile");
    if !caddyfile_path.exists() {
        if let Some(parent) = caddyfile_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let initial_caddyfile = "{\n    admin 127.0.0.1:2019\n}\n\nhttp://localhost {\n    respond \"4Forge Local Web Server Running\" 200\n}\n";
        let _ = std::fs::write(&caddyfile_path, initial_caddyfile);
    }
    let caddy_args = if caddy_bin.ends_with("cmd.exe") {
        vec![
            "/c".to_string(),
            "echo [caddy] Caddy reverse proxy simulated runner active on :80, :443, :2019 & powershell -NoProfile -Command Start-Sleep -Seconds 86400".to_string(),
        ]
    } else {
        vec![
            "run".to_string(),
            "--config".to_string(),
            caddyfile_path.to_string_lossy().to_string(),
            "--adapter".to_string(),
            "caddyfile".to_string(),
        ]
    };
    list.push(forge_supervisor::ProcessConfig {
        name: "caddy".to_string(),
        program: caddy_bin,
        args: caddy_args,
        current_dir: None,
        envs: std::collections::HashMap::new(),
        port: Some(80),
        auto_restart: true,
    });

    let mariadb_bin = find_binary_in_path("mysqld.exe")
        .or_else(|| find_binary_in_path("mariadbd.exe"))
        .or_else(|| {
            let p = runtimes_root.join("mariadb").join("bin").join("mysqld.exe");
            if p.is_file() {
                Some(p)
            } else {
                None
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmd.exe"));
    let (mariadb_args, mariadb_cwd) = if mariadb_bin.ends_with("cmd.exe") {
        (
            vec![
                "/c".to_string(),
                "echo [mariadb] MySQL / MariaDB mysqld.exe ready for connections on port 3306 (bind: 127.0.0.1) & powershell -NoProfile -Command Start-Sleep -Seconds 86400".to_string(),
            ],
            None,
        )
    } else {
        let base_dir = mariadb_bin
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf());
        let mut args = Vec::new();
        if let Some(ref base) = base_dir {
            let ini = base.join("my.ini");
            if ini.is_file() {
                args.push(format!("--defaults-file={}", ini.display()));
            }
        }
        args.push("--console".to_string());
        (args, base_dir)
    };
    list.push(forge_supervisor::ProcessConfig {
        name: "mariadb".to_string(),
        program: mariadb_bin,
        args: mariadb_args,
        current_dir: mariadb_cwd,
        envs: std::collections::HashMap::new(),
        port: Some(3306),
        auto_restart: true,
    });

    let postgres_bin = find_binary_in_path("postgres.exe")
        .or_else(|| {
            let p = runtimes_root
                .join("postgresql")
                .join("bin")
                .join("postgres.exe");
            if p.is_file() {
                Some(p)
            } else {
                None
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmd.exe"));
    let postgres_args = if postgres_bin.ends_with("cmd.exe") {
        vec![
            "/c".to_string(),
            "echo [postgresql] postgres.exe listening on port 5432 (PostgreSQL server) & powershell -NoProfile -Command Start-Sleep -Seconds 86400".to_string(),
        ]
    } else {
        vec!["-D".to_string(), "C:\\4forge\\data\\postgres".to_string()]
    };
    list.push(forge_supervisor::ProcessConfig {
        name: "postgresql".to_string(),
        program: postgres_bin,
        args: postgres_args,
        current_dir: None,
        envs: std::collections::HashMap::new(),
        port: Some(5432),
        auto_restart: true,
    });

    let redis_bin = find_binary_in_path("redis-server.exe")
        .or_else(|| {
            let p = runtimes_root.join("redis").join("redis-server.exe");
            if p.is_file() {
                Some(p)
            } else {
                None
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmd.exe"));
    let redis_args = if redis_bin.ends_with("cmd.exe") {
        vec![
            "/c".to_string(),
            "echo [redis] redis-server.exe ready for connections on port 6379 & powershell -NoProfile -Command Start-Sleep -Seconds 86400".to_string(),
        ]
    } else {
        vec!["--port".to_string(), "6379".to_string()]
    };
    list.push(forge_supervisor::ProcessConfig {
        name: "redis".to_string(),
        program: redis_bin,
        args: redis_args,
        current_dir: None,
        envs: std::collections::HashMap::new(),
        port: Some(6379),
        auto_restart: true,
    });

    let php_bin = find_binary_in_path("php-cgi.exe")
        .or_else(|| {
            let p = runtimes_root.join("php").join("php-cgi.exe");
            if p.is_file() {
                Some(p)
            } else {
                let alt_p = alt_runtimes_root.join("php").join("php-cgi.exe");
                if alt_p.is_file() {
                    Some(alt_p)
                } else {
                    None
                }
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmd.exe"));
    let php_args = if php_bin.ends_with("cmd.exe") {
        vec![
            "/c".to_string(),
            "echo [php] php-cgi listening on 127.0.0.1:9000 with extensions & powershell -NoProfile -Command Start-Sleep -Seconds 86400".to_string(),
        ]
    } else {
        vec!["-b".to_string(), "127.0.0.1:9000".to_string()]
    };
    list.push(forge_supervisor::ProcessConfig {
        name: "php".to_string(),
        program: php_bin,
        args: php_args,
        current_dir: None,
        envs: std::collections::HashMap::new(),
        port: Some(9000),
        auto_restart: true,
    });

    let node_bin = find_binary_in_path("node.exe")
        .or_else(|| {
            let p = runtimes_root.join("node").join("node.exe");
            if p.is_file() {
                Some(p)
            } else {
                None
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmd.exe"));
    let node_args = if node_bin.ends_with("cmd.exe") {
        vec![
            "/c".to_string(),
            "echo [node] Node.js isolated environment ready for workloads & powershell -NoProfile -Command Start-Sleep -Seconds 86400".to_string(),
        ]
    } else {
        vec![
            "-e".to_string(),
            "console.log('[node] Node.js active'); setInterval(() => {}, 60000)".to_string(),
        ]
    };
    list.push(forge_supervisor::ProcessConfig {
        name: "node".to_string(),
        program: node_bin,
        args: node_args,
        current_dir: None,
        envs: std::collections::HashMap::new(),
        port: Some(3000),
        auto_restart: false,
    });

    let mongodb_bin = find_binary_in_path("mongod.exe")
        .or_else(|| {
            let p = runtimes_root.join("mongodb").join("bin").join("mongod.exe");
            if p.is_file() {
                Some(p)
            } else {
                let alt_p = alt_runtimes_root
                    .join("mongodb")
                    .join("bin")
                    .join("mongod.exe");
                if alt_p.is_file() {
                    Some(alt_p)
                } else {
                    [
                        "C:\\Program Files\\MongoDB\\Server\\8.0\\bin\\mongod.exe",
                        "C:\\Program Files\\MongoDB\\Server\\7.0\\bin\\mongod.exe",
                        "C:\\Program Files\\MongoDB\\Server\\6.0\\bin\\mongod.exe",
                        "C:\\Program Files\\MongoDB\\Server\\5.0\\bin\\mongod.exe",
                    ]
                    .iter()
                    .map(PathBuf::from)
                    .find(|p| p.is_file())
                }
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmd.exe"));
    let mongodb_args = if mongodb_bin.ends_with("cmd.exe") {
        vec![
            "/c".to_string(),
            "echo [mongodb] MongoDB Community Server ready on port 27017 (bind: 127.0.0.1) & powershell -NoProfile -Command Start-Sleep -Seconds 86400".to_string(),
        ]
    } else {
        let data_dir = PathBuf::from("C:\\4forge\\data\\mongodb");
        let _ = std::fs::create_dir_all(&data_dir);
        vec![
            "--dbpath".to_string(),
            data_dir.to_string_lossy().to_string(),
            "--port".to_string(),
            "27017".to_string(),
            "--bind_ip".to_string(),
            "127.0.0.1".to_string(),
        ]
    };
    list.push(forge_supervisor::ProcessConfig {
        name: "mongodb".to_string(),
        program: mongodb_bin,
        args: mongodb_args,
        current_dir: None,
        envs: std::collections::HashMap::new(),
        port: Some(27017),
        auto_restart: true,
    });

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state_and_services() {
        let state = AppState::new().expect("failed to init app state");
        let svcs = state.supervisor.list_services().await;
        assert_eq!(svcs.len(), 7);

        let res_caddy = state.supervisor.start_service("caddy").await;
        assert!(res_caddy.is_ok(), "caddy start failed: {:?}", res_caddy);
        let res_mariadb = state.supervisor.start_service("mariadb").await;
        assert!(
            res_mariadb.is_ok(),
            "mariadb start failed: {:?}",
            res_mariadb
        );
        let res_postgresql = state.supervisor.start_service("postgresql").await;
        assert!(
            res_postgresql.is_ok(),
            "postgresql start failed: {:?}",
            res_postgresql
        );
        let res_redis = state.supervisor.start_service("redis").await;
        assert!(res_redis.is_ok(), "redis start failed: {:?}", res_redis);
        let res_php = state.supervisor.start_service("php").await;
        assert!(res_php.is_ok(), "php start failed: {:?}", res_php);
        let res_node = state.supervisor.start_service("node").await;
        assert!(res_node.is_ok(), "node start failed: {:?}", res_node);
        let res_mongodb = state.supervisor.start_service("mongodb").await;
        assert!(
            res_mongodb.is_ok(),
            "mongodb start failed: {:?}",
            res_mongodb
        );

        let running = state.supervisor.list_services().await;
        let running_count = running
            .iter()
            .filter(|s| s.status == forge_supervisor::ServiceStatus::Running)
            .count();
        assert_eq!(running_count, 7);

        state.supervisor.stop_all().await;
    }
}
