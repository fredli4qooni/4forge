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

use crate::driver::{DatabaseCommand, DatabaseDriver};
use crate::{DatabaseEngine, DatabaseError, DatabaseInstanceInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDatabaseDto {
    pub name: String,
    pub engine: String,
    pub host: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub size_bytes: Option<u64>,
    pub tables_count: Option<usize>,
    pub status: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseUserDto {
    pub username: String,
    pub host: String,
    pub engine: String,
    pub privileges: Vec<String>,
    pub is_system_account: bool,
}

#[derive(Default, Clone)]
pub struct DatabaseManager {
    drivers: HashMap<DatabaseEngine, Arc<dyn DatabaseDriver>>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            drivers: HashMap::new(),
        }
    }

    pub fn register_driver(&mut self, driver: Arc<dyn DatabaseDriver>) {
        self.drivers.insert(driver.engine(), driver);
    }

    pub fn get_driver(&self, engine: DatabaseEngine) -> Option<Arc<dyn DatabaseDriver>> {
        self.drivers.get(&engine).cloned()
    }

    pub async fn check_port_health(port: u16, timeout_ms: u64) -> bool {
        let addr = format!("127.0.0.1:{}", port);
        matches!(
            timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await,
            Ok(Ok(_))
        )
    }

    pub async fn is_engine_healthy(&self, engine: DatabaseEngine) -> bool {
        if let Some(driver) = self.drivers.get(&engine) {
            if let Some(port) = driver.port() {
                Self::check_port_health(port, 500).await
            } else {
                driver.is_initialized()
            }
        } else {
            false
        }
    }

    pub fn prepare_engine(&self, engine: DatabaseEngine) -> Result<(), DatabaseError> {
        let driver = self
            .drivers
            .get(&engine)
            .ok_or(DatabaseError::DriverNotFound(engine))?;
        driver.prepare_environment()
    }

    pub fn get_init_command(
        &self,
        engine: DatabaseEngine,
    ) -> Result<Option<DatabaseCommand>, DatabaseError> {
        let driver = self
            .drivers
            .get(&engine)
            .ok_or(DatabaseError::DriverNotFound(engine))?;
        Ok(driver.build_init_command())
    }

    pub fn get_start_command(
        &self,
        engine: DatabaseEngine,
    ) -> Result<DatabaseCommand, DatabaseError> {
        let driver = self
            .drivers
            .get(&engine)
            .ok_or(DatabaseError::DriverNotFound(engine))?;
        driver.build_start_command()
    }

    pub fn get_stop_command(
        &self,
        engine: DatabaseEngine,
    ) -> Result<Option<DatabaseCommand>, DatabaseError> {
        let driver = self
            .drivers
            .get(&engine)
            .ok_or(DatabaseError::DriverNotFound(engine))?;
        Ok(driver.build_stop_command())
    }

    pub async fn list_instances(&self) -> Vec<DatabaseInstanceInfo> {
        let mut list = Vec::new();
        for (engine, driver) in &self.drivers {
            let is_running = if let Some(port) = driver.port() {
                Self::check_port_health(port, 300).await
            } else {
                false
            };

            list.push(DatabaseInstanceInfo {
                engine: *engine,
                port: driver.port(),
                data_dir: driver.data_dir().to_string_lossy().to_string(),
                is_initialized: driver.is_initialized(),
                is_running,
            });
        }
        list
    }

    pub fn find_executable(name: &str, extra_dirs: &[PathBuf]) -> Option<PathBuf> {
        let exe_name = if name.ends_with(".exe") {
            name.to_string()
        } else {
            format!("{name}.exe")
        };

        for dir in extra_dirs {
            let candidate = dir.join(&exe_name);
            if candidate.is_file() {
                return Some(candidate);
            }
            let candidate_raw = dir.join(name);
            if candidate_raw.is_file() {
                return Some(candidate_raw);
            }
        }

        if let Some(paths) = std::env::var_os("PATH") {
            for path in std::env::split_paths(&paths) {
                let candidate = path.join(&exe_name);
                if candidate.is_file() {
                    return Some(candidate);
                }
                let candidate_raw = path.join(name);
                if candidate_raw.is_file() {
                    return Some(candidate_raw);
                }
            }
        }

        let mut search_roots = vec![
            PathBuf::from("C:\\4forge\\runtimes"),
            PathBuf::from("D:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\laragon\\bin\\mysql"),
            PathBuf::from("D:\\laragon\\bin"),
            PathBuf::from("C:\\laragon\\bin"),
            PathBuf::from("C:\\Program Files\\PostgreSQL"),
            PathBuf::from("C:\\Program Files\\MariaDB"),
            PathBuf::from("C:\\Program Files\\MySQL"),
        ];
        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            search_roots.push(PathBuf::from(appdata).join("4Forge").join("runtimes"));
        }

        for root in &search_roots {
            if root.is_dir() {
                let direct_bin = root.join("bin").join(&exe_name);
                if direct_bin.is_file() {
                    return Some(direct_bin);
                }
                if let Ok(entries) = std::fs::read_dir(root) {
                    for entry in entries.flatten() {
                        let sub = entry.path();
                        if sub.is_dir() {
                            let sub_bin = sub.join("bin").join(&exe_name);
                            if sub_bin.is_file() {
                                return Some(sub_bin);
                            }
                            let sub_direct = sub.join(&exe_name);
                            if sub_direct.is_file() {
                                return Some(sub_direct);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn registry_file_path() -> PathBuf {
        let default_path = PathBuf::from("C:\\4forge\\config\\databases.json");
        if default_path.parent().map(|p| p.is_dir()).unwrap_or(false) || default_path.is_file() {
            return default_path;
        }
        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            let p = PathBuf::from(appdata).join("4Forge").join("databases.json");
            if let Some(parent) = p.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            p
        } else {
            if let Some(parent) = default_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            default_path
        }
    }

    pub fn load_registry() -> Vec<UserDatabaseDto> {
        let p = Self::registry_file_path();
        if p.is_file() {
            if let Ok(content) = std::fs::read_to_string(&p) {
                if let Ok(list) = serde_json::from_str::<Vec<UserDatabaseDto>>(&content) {
                    return list;
                }
            }
        }
        Vec::new()
    }

    pub fn save_registry(list: &[UserDatabaseDto]) -> Result<(), String> {
        let p = Self::registry_file_path();
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
        std::fs::write(&p, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list_all_databases(&self) -> Vec<UserDatabaseDto> {
        let mut list = Self::load_registry();

        let sqlite_dirs = [
            PathBuf::from("C:\\4forge\\data\\sqlite"),
            if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
                PathBuf::from(appdata)
                    .join("4Forge")
                    .join("data")
                    .join("sqlite")
            } else {
                PathBuf::from("C:\\4forge\\data\\sqlite")
            },
        ];

        let mut discovered_new = false;
        for dir in &sqlite_dirs {
            if dir.is_dir() {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                            if ext.eq_ignore_ascii_case("sqlite")
                                || ext.eq_ignore_ascii_case("db")
                                || ext.eq_ignore_ascii_case("sqlite3")
                            {
                                let stem = path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("database");
                                if !list.iter().any(|d| {
                                    d.engine == "sqlite" && d.name.eq_ignore_ascii_case(stem)
                                }) {
                                    let size_bytes = entry.metadata().ok().map(|m| m.len());
                                    list.push(UserDatabaseDto {
                                        name: stem.to_string(),
                                        engine: "sqlite".to_string(),
                                        host: path.to_string_lossy().to_string(),
                                        port: None,
                                        user: None,
                                        size_bytes,
                                        tables_count: None,
                                        status: "running".to_string(),
                                        created_at: Some("local".to_string()),
                                    });
                                    discovered_new = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        let maria_healthy = Self::check_port_health(3306, 300).await;
        let pg_healthy = Self::check_port_health(5432, 300).await;
        let mongo_healthy = Self::check_port_health(27017, 300).await;

        if maria_healthy {
            let extra_dirs = [PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin")];
            if let Some(cli) = Self::find_executable("mysql", &extra_dirs)
                .or_else(|| Self::find_executable("mariadb", &extra_dirs))
            {
                if let Ok(Ok(output)) = tokio::time::timeout(
                    Duration::from_millis(1500),
                    tokio::process::Command::new(&cli)
                        .args([
                            "-u",
                            "root",
                            "-h",
                            "127.0.0.1",
                            "--port=3306",
                            "-B",
                            "-N",
                            "-e",
                            "SHOW DATABASES;",
                        ])
                        .output(),
                )
                .await
                {
                    if output.status.success() {
                        let text = String::from_utf8_lossy(&output.stdout);
                        for line in text.lines() {
                            let db_name = line.trim();
                            if db_name.is_empty() {
                                continue;
                            }
                            let skip = ["information_schema", "performance_schema", "mysql", "sys"];
                            if skip.iter().any(|s| s.eq_ignore_ascii_case(db_name)) {
                                continue;
                            }
                            if !list.iter().any(|d| {
                                (d.engine == "mariadb" || d.engine == "mysql")
                                    && d.name.eq_ignore_ascii_case(db_name)
                            }) {
                                list.push(UserDatabaseDto {
                                    name: db_name.to_string(),
                                    engine: "mariadb".to_string(),
                                    host: "127.0.0.1".to_string(),
                                    port: Some(3306),
                                    user: Some("root".to_string()),
                                    size_bytes: None,
                                    tables_count: None,
                                    status: "running".to_string(),
                                    created_at: Some("active".to_string()),
                                });
                                discovered_new = true;
                            }
                        }
                    }
                }
            }
        }

        if pg_healthy {
            let extra_dirs = [PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin")];
            if let Some(cli) = Self::find_executable("psql", &extra_dirs) {
                if let Ok(Ok(output)) = tokio::time::timeout(
                    Duration::from_millis(1500),
                    tokio::process::Command::new(&cli)
                        .args([
                            "-h",
                            "127.0.0.1",
                            "-p",
                            "5432",
                            "-U",
                            "postgres",
                            "-t",
                            "-A",
                            "-c",
                            "SELECT datname FROM pg_database WHERE datistemplate = false AND datname != 'postgres';",
                        ])
                        .output(),
                )
                .await
                {
                    if output.status.success() {
                        let text = String::from_utf8_lossy(&output.stdout);
                        for line in text.lines() {
                            let db_name = line.trim();
                            if db_name.is_empty() {
                                continue;
                            }
                            if !list.iter().any(|d| {
                                (d.engine == "postgresql" || d.engine == "postgres")
                                    && d.name.eq_ignore_ascii_case(db_name)
                            }) {
                                list.push(UserDatabaseDto {
                                    name: db_name.to_string(),
                                    engine: "postgresql".to_string(),
                                    host: "127.0.0.1".to_string(),
                                    port: Some(5432),
                                    user: Some("postgres".to_string()),
                                    size_bytes: None,
                                    tables_count: None,
                                    status: "running".to_string(),
                                    created_at: Some("active".to_string()),
                                });
                                discovered_new = true;
                            }
                        }
                    }
                }
            }
        }

        for db in &mut list {
            match db.engine.as_str() {
                "mariadb" | "mysql" => {
                    db.status = if maria_healthy {
                        "running".to_string()
                    } else {
                        "stopped".to_string()
                    };
                }
                "postgresql" | "postgres" => {
                    db.status = if pg_healthy {
                        "running".to_string()
                    } else {
                        "stopped".to_string()
                    };
                }
                "mongodb" | "mongo" => {
                    db.status = if mongo_healthy {
                        "running".to_string()
                    } else {
                        "stopped".to_string()
                    };
                }
                "sqlite" | "sqlite3" => {
                    db.status = "running".to_string();
                }
                _ => {
                    db.status = "ready".to_string();
                }
            }
        }

        if discovered_new {
            let _ = Self::save_registry(&list);
        }

        list
    }

    pub async fn create_database(&self, engine: &str, db_name: &str) -> Result<(), String> {
        let clean = db_name.trim();
        if clean.is_empty() {
            return Err("Database name cannot be empty".to_string());
        }
        if !clean.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(
                "Database name may only contain alphanumeric characters and underscores"
                    .to_string(),
            );
        }

        let engine_lower = engine.to_lowercase();
        let result = match engine_lower.as_str() {
            "mariadb" | "mysql" => {
                let port = self
                    .drivers
                    .get(&DatabaseEngine::MariaDb)
                    .and_then(|d| d.port())
                    .unwrap_or(3306);
                let is_running = Self::check_port_health(port, 400).await;
                if !is_running {
                    return Err(format!(
                        "MariaDB / MySQL is not running on port {port}. Please start the database service first."
                    ));
                }

                let mut extra_dirs = Vec::new();
                if let Some(driver) = self.drivers.get(&DatabaseEngine::MariaDb) {
                    let base = driver
                        .data_dir()
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| driver.data_dir().to_path_buf());
                    extra_dirs.push(base.join("bin"));
                    extra_dirs.push(base);
                }
                extra_dirs.push(PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"));

                let admin_bin = Self::find_executable("mysqladmin", &extra_dirs)
                    .or_else(|| Self::find_executable("mariadb-admin", &extra_dirs));

                if let Some(bin) = admin_bin {
                    let output = tokio::process::Command::new(&bin)
                        .args([
                            "-u",
                            "root",
                            &format!("--port={port}"),
                            "-h",
                            "127.0.0.1",
                            "create",
                            clean,
                        ])
                        .output()
                        .await
                        .map_err(|e| format!("Failed to execute {}: {e}", bin.display()))?;

                    if !output.status.success() {
                        let err_msg = String::from_utf8_lossy(&output.stderr);
                        if !err_msg.contains("database exists")
                            && !err_msg.contains("already exists")
                        {
                            return Err(format!(
                                "Failed to create MySQL/MariaDB database: {err_msg}"
                            ));
                        }
                    }
                    Ok(())
                } else if let Some(cli_bin) = Self::find_executable("mysql", &extra_dirs) {
                    let output = tokio::process::Command::new(&cli_bin)
                        .args([
                            "-u",
                            "root",
                            &format!("--port={port}"),
                            "-h",
                            "127.0.0.1",
                            "-e",
                            &format!(
                                "CREATE DATABASE IF NOT EXISTS `{clean}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
                            ),
                        ])
                        .output()
                        .await
                        .map_err(|e| format!("Failed to execute {}: {e}", cli_bin.display()))?;

                    if !output.status.success() {
                        let err_msg = String::from_utf8_lossy(&output.stderr);
                        return Err(format!(
                            "Failed to create database via mysql CLI: {err_msg}"
                        ));
                    }
                    Ok(())
                } else {
                    Err("MySQL / MariaDB CLI binary (mysqladmin.exe or mysql.exe) not found on system PATH or runtimes.".to_string())
                }
            }
            "postgresql" | "postgres" => {
                let port = self
                    .drivers
                    .get(&DatabaseEngine::PostgreSql)
                    .and_then(|d| d.port())
                    .unwrap_or(5432);
                let is_running = Self::check_port_health(port, 400).await;
                if !is_running {
                    return Err(format!(
                        "PostgreSQL is not running on port {port}. Please install PostgreSQL or start the service first."
                    ));
                }

                let mut extra_dirs = Vec::new();
                if let Some(driver) = self.drivers.get(&DatabaseEngine::PostgreSql) {
                    let base = driver
                        .data_dir()
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| driver.data_dir().to_path_buf());
                    extra_dirs.push(base.join("bin"));
                    extra_dirs.push(base);
                }
                extra_dirs.push(PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"));

                let createdb_bin = Self::find_executable("createdb", &extra_dirs);
                let psql_bin = Self::find_executable("psql", &extra_dirs);

                if let Some(bin) = createdb_bin {
                    let output = tokio::process::Command::new(&bin)
                        .args([
                            "-h",
                            "127.0.0.1",
                            "-p",
                            &port.to_string(),
                            "-U",
                            "postgres",
                            clean,
                        ])
                        .output()
                        .await
                        .map_err(|e| format!("Failed to execute {}: {e}", bin.display()))?;

                    if !output.status.success() {
                        let err_msg = String::from_utf8_lossy(&output.stderr);
                        if !err_msg.contains("already exists") {
                            return Err(format!("Failed to create PostgreSQL database: {err_msg}"));
                        }
                    }
                    Ok(())
                } else if let Some(bin) = psql_bin {
                    let output = tokio::process::Command::new(&bin)
                        .args([
                            "-h",
                            "127.0.0.1",
                            "-p",
                            &port.to_string(),
                            "-U",
                            "postgres",
                            "-c",
                            &format!("CREATE DATABASE \"{clean}\";"),
                        ])
                        .output()
                        .await
                        .map_err(|e| format!("Failed to execute {}: {e}", bin.display()))?;

                    if !output.status.success() {
                        let err_msg = String::from_utf8_lossy(&output.stderr);
                        if !err_msg.contains("already exists") {
                            return Err(format!(
                                "Failed to create PostgreSQL database via psql: {err_msg}"
                            ));
                        }
                    }
                    Ok(())
                } else {
                    Err(
                        "PostgreSQL createdb / psql binary not found on system PATH or runtimes."
                            .to_string(),
                    )
                }
            }
            "sqlite" | "sqlite3" => {
                let sqlite_dir = self
                    .drivers
                    .get(&DatabaseEngine::Sqlite)
                    .map(|d| d.data_dir().to_path_buf())
                    .unwrap_or_else(|| std::path::PathBuf::from("C:\\4forge\\data\\sqlite"));

                if let Err(e) = std::fs::create_dir_all(&sqlite_dir) {
                    return Err(format!("Failed to create SQLite directory: {e}"));
                }
                let db_file = sqlite_dir.join(format!("{clean}.sqlite"));
                if !db_file.exists() {
                    if let Err(e) = std::fs::File::create(&db_file) {
                        return Err(format!("Failed to create SQLite database file: {e}"));
                    }
                }
                Ok(())
            }
            "mongodb" | "mongo" => {
                let port = self
                    .drivers
                    .get(&DatabaseEngine::MongoDb)
                    .and_then(|d| d.port())
                    .unwrap_or(27017);
                let is_running = Self::check_port_health(port, 400).await;
                if !is_running {
                    return Err(format!(
                        "MongoDB is not running on port {port}. Please start MongoDB service first."
                    ));
                }
                Ok(())
            }
            _ => Err(format!("Unsupported database engine: '{engine}'")),
        };

        if result.is_ok() {
            let (host, port, user) = match engine_lower.as_str() {
                "postgresql" | "postgres" => (
                    "127.0.0.1".to_string(),
                    Some(5432),
                    Some("postgres".to_string()),
                ),
                "mongodb" | "mongo" => ("127.0.0.1".to_string(), Some(27017), None),
                "sqlite" | "sqlite3" => (
                    format!("C:\\4forge\\data\\sqlite\\{clean}.sqlite"),
                    None,
                    None,
                ),
                _ => (
                    "127.0.0.1".to_string(),
                    Some(3306),
                    Some("root".to_string()),
                ),
            };

            let standard_engine = if engine_lower.contains("postgres") {
                "postgresql".to_string()
            } else if engine_lower.contains("mongo") {
                "mongodb".to_string()
            } else if engine_lower.contains("sqlite") {
                "sqlite".to_string()
            } else {
                "mariadb".to_string()
            };

            let mut list = Self::load_registry();
            if !list
                .iter()
                .any(|d| d.name.eq_ignore_ascii_case(clean) && d.engine == standard_engine)
            {
                list.push(UserDatabaseDto {
                    name: clean.to_string(),
                    engine: standard_engine,
                    host,
                    port,
                    user,
                    size_bytes: None,
                    tables_count: None,
                    status: "running".to_string(),
                    created_at: Some("recent".to_string()),
                });
                let _ = Self::save_registry(&list);
            }
        }

        result
    }

    pub async fn delete_database(&self, engine: &str, db_name: &str) -> Result<(), String> {
        let clean = db_name.trim();
        let engine_lower = engine.to_lowercase();

        match engine_lower.as_str() {
            "sqlite" | "sqlite3" => {
                let sqlite_dirs = [
                    PathBuf::from("C:\\4forge\\data\\sqlite"),
                    if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
                        PathBuf::from(appdata)
                            .join("4Forge")
                            .join("data")
                            .join("sqlite")
                    } else {
                        PathBuf::from("C:\\4forge\\data\\sqlite")
                    },
                ];
                for dir in &sqlite_dirs {
                    let candidates = [
                        dir.join(format!("{clean}.sqlite")),
                        dir.join(format!("{clean}.db")),
                        dir.join(format!("{clean}.sqlite3")),
                        dir.join(clean),
                    ];
                    for f in candidates {
                        if f.is_file() {
                            let _ = std::fs::remove_file(f);
                        }
                    }
                }
            }
            "mariadb" | "mysql" => {
                let mut extra_dirs = Vec::new();
                if let Some(driver) = self.drivers.get(&DatabaseEngine::MariaDb) {
                    let base = driver.data_dir().to_path_buf();
                    extra_dirs.push(base.join("bin"));
                }
                extra_dirs.push(PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"));

                if let Some(admin) = Self::find_executable("mysqladmin", &extra_dirs)
                    .or_else(|| Self::find_executable("mariadb-admin", &extra_dirs))
                {
                    let _ = tokio::process::Command::new(&admin)
                        .args([
                            "-u",
                            "root",
                            "-h",
                            "127.0.0.1",
                            "--port=3306",
                            "drop",
                            "-f",
                            clean,
                        ])
                        .output()
                        .await;
                } else if let Some(cli) = Self::find_executable("mysql", &extra_dirs) {
                    let _ = tokio::process::Command::new(&cli)
                        .args([
                            "-u",
                            "root",
                            "-h",
                            "127.0.0.1",
                            "--port=3306",
                            "-e",
                            &format!("DROP DATABASE IF EXISTS `{clean}`;"),
                        ])
                        .output()
                        .await;
                }
            }
            "postgresql" | "postgres" => {
                let mut extra_dirs = Vec::new();
                if let Some(driver) = self.drivers.get(&DatabaseEngine::PostgreSql) {
                    let base = driver.data_dir().to_path_buf();
                    extra_dirs.push(base.join("bin"));
                }
                extra_dirs.push(PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"));

                if let Some(dropdb) = Self::find_executable("dropdb", &extra_dirs) {
                    let _ = tokio::process::Command::new(&dropdb)
                        .args(["-h", "127.0.0.1", "-p", "5432", "-U", "postgres", clean])
                        .output()
                        .await;
                } else if let Some(psql) = Self::find_executable("psql", &extra_dirs) {
                    let _ = tokio::process::Command::new(&psql)
                        .args([
                            "-h",
                            "127.0.0.1",
                            "-p",
                            "5432",
                            "-U",
                            "postgres",
                            "-c",
                            &format!("DROP DATABASE IF EXISTS \"{clean}\";"),
                        ])
                        .output()
                        .await;
                }
            }
            _ => {}
        }

        let mut list = Self::load_registry();
        list.retain(|d| {
            !(d.name.eq_ignore_ascii_case(clean)
                && (d.engine.eq_ignore_ascii_case(&engine_lower)
                    || (d.engine.contains("mariadb") && engine_lower.contains("mysql"))
                    || (d.engine.contains("mysql") && engine_lower.contains("mariadb"))
                    || (d.engine.contains("postgres") && engine_lower.contains("postgres"))))
        });
        let _ = Self::save_registry(&list);
        Ok(())
    }

    pub async fn list_users(&self, engine: &str) -> Result<Vec<DatabaseUserDto>, String> {
        let eng = engine.to_lowercase();
        let extra_dirs = [
            PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"),
            PathBuf::from("D:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"),
        ];

        match eng.as_str() {
            "mariadb" | "mysql" => {
                let is_running = Self::check_port_health(3306, 400).await;
                if !is_running {
                    return Err("MySQL / MariaDB is not running on port 3306".to_string());
                }

                let cli_bin = Self::find_executable("mysql", &extra_dirs)
                    .or_else(|| Self::find_executable("mariadb", &extra_dirs))
                    .ok_or_else(|| {
                        "mysql.exe / mariadb.exe binary not found on system PATH or runtimes."
                            .to_string()
                    })?;

                let output = tokio::process::Command::new(&cli_bin)
                    .args([
                        "-u",
                        "root",
                        "-h",
                        "127.0.0.1",
                        "--port=3306",
                        "-N",
                        "-B",
                        "-e",
                        "SELECT User, Host FROM mysql.user WHERE User != '' ORDER BY User ASC;",
                    ])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to query MySQL users: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("MySQL user query failed: {err}"));
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut users = Vec::new();

                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split('\t').map(|s| s.trim()).collect();
                    if parts.is_empty() || parts[0].is_empty() {
                        continue;
                    }
                    let username = parts[0].to_string();
                    let host = if parts.len() > 1 && !parts[1].is_empty() {
                        parts[1].to_string()
                    } else {
                        "%".to_string()
                    };

                    let is_system_account = username == "root"
                        || username == "mysql.session"
                        || username == "mysql.sys"
                        || username == "mysql.infoschema"
                        || username == "mariadb.sys";

                    let mut privileges = Vec::new();
                    let grant_out = tokio::process::Command::new(&cli_bin)
                        .args([
                            "-u",
                            "root",
                            "-h",
                            "127.0.0.1",
                            "--port=3306",
                            "-N",
                            "-B",
                            "-e",
                            &format!("SHOW GRANTS FOR '{username}'@'{host}';"),
                        ])
                        .output()
                        .await;

                    if let Ok(grants) = grant_out {
                        if grants.status.success() {
                            let grants_txt = String::from_utf8_lossy(&grants.stdout);
                            for g in grants_txt.lines() {
                                let g_trim = g.trim();
                                if !g_trim.is_empty() {
                                    privileges.push(g_trim.to_string());
                                }
                            }
                        }
                    }

                    users.push(DatabaseUserDto {
                        username,
                        host,
                        engine: "mysql".to_string(),
                        privileges,
                        is_system_account,
                    });
                }

                Ok(users)
            }
            "postgresql" | "postgres" => {
                let is_running = Self::check_port_health(5432, 400).await;
                if !is_running {
                    return Err("PostgreSQL is not running on port 5432".to_string());
                }

                let psql_bin = Self::find_executable("psql", &extra_dirs).ok_or_else(|| {
                    "psql.exe binary not found on system PATH or runtimes.".to_string()
                })?;

                let output = tokio::process::Command::new(&psql_bin)
                    .args([
                        "-U",
                        "postgres",
                        "-h",
                        "127.0.0.1",
                        "-p",
                        "5432",
                        "-t",
                        "-A",
                        "-F",
                        "\t",
                        "-c",
                        "SELECT rolname, rolsuper, rolcreaterole, rolcreatedb, rolcanlogin FROM pg_roles WHERE rolname NOT LIKE 'pg_%' ORDER BY rolname ASC;",
                    ])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to query PostgreSQL users: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("PostgreSQL user query failed: {err}"));
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut users = Vec::new();

                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split('\t').map(|s| s.trim()).collect();
                    if parts.is_empty() || parts[0].is_empty() {
                        continue;
                    }
                    let username = parts[0].to_string();
                    let is_super = parts.get(1).copied().unwrap_or("f") == "t";
                    let is_createrole = parts.get(2).copied().unwrap_or("f") == "t";
                    let is_createdb = parts.get(3).copied().unwrap_or("f") == "t";

                    let mut privileges = Vec::new();
                    if is_super {
                        privileges.push("SUPERUSER".to_string());
                    }
                    if is_createrole {
                        privileges.push("CREATEROLE".to_string());
                    }
                    if is_createdb {
                        privileges.push("CREATEDB".to_string());
                    }
                    if privileges.is_empty() {
                        privileges.push("STANDARD USER".to_string());
                    }

                    let is_system_account = username == "postgres";

                    users.push(DatabaseUserDto {
                        username,
                        host: "all".to_string(),
                        engine: "postgresql".to_string(),
                        privileges,
                        is_system_account,
                    });
                }

                Ok(users)
            }
            "sqlite" | "sqlite3" => Err(
                "SQLite is a serverless database engine and does not use user accounts."
                    .to_string(),
            ),
            _ => Err(format!(
                "User management is not supported for engine '{engine}'"
            )),
        }
    }

    pub async fn create_user(
        &self,
        engine: &str,
        username: &str,
        host: &str,
        password: &str,
        privilege_scope: &str,
        target_db: Option<&str>,
    ) -> Result<(), String> {
        let clean_user = username.trim();
        if clean_user.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if !clean_user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(
                "Username must contain only alphanumeric characters and underscores".to_string(),
            );
        }

        let eng = engine.to_lowercase();
        let extra_dirs = [
            PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"),
            PathBuf::from("D:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"),
        ];

        match eng.as_str() {
            "mariadb" | "mysql" => {
                let is_running = Self::check_port_health(3306, 400).await;
                if !is_running {
                    return Err("MySQL / MariaDB is not running on port 3306".to_string());
                }

                let cli_bin = Self::find_executable("mysql", &extra_dirs)
                    .or_else(|| Self::find_executable("mariadb", &extra_dirs))
                    .ok_or_else(|| {
                        "mysql.exe / mariadb.exe binary not found on system PATH or runtimes."
                            .to_string()
                    })?;

                let clean_host = if host.trim().is_empty() {
                    "%"
                } else {
                    host.trim()
                };
                let safe_pw = password.replace('\'', "''");

                let mut query = format!(
                    "CREATE USER IF NOT EXISTS '{clean_user}'@'{clean_host}' IDENTIFIED BY '{safe_pw}'; "
                );

                if privilege_scope.eq_ignore_ascii_case("all")
                    || privilege_scope.eq_ignore_ascii_case("global")
                {
                    query.push_str(&format!(
                        "GRANT ALL PRIVILEGES ON *.* TO '{clean_user}'@'{clean_host}' WITH GRANT OPTION; "
                    ));
                } else if privilege_scope.eq_ignore_ascii_case("database") {
                    if let Some(db) = target_db {
                        let clean_db = db.trim();
                        if !clean_db.is_empty() {
                            query.push_str(&format!(
                                "GRANT ALL PRIVILEGES ON `{clean_db}`.* TO '{clean_user}'@'{clean_host}'; "
                            ));
                        }
                    }
                } else if privilege_scope.eq_ignore_ascii_case("readonly") {
                    query.push_str(&format!(
                        "GRANT SELECT ON *.* TO '{clean_user}'@'{clean_host}'; "
                    ));
                } else {
                    query.push_str(&format!(
                        "GRANT ALL PRIVILEGES ON *.* TO '{clean_user}'@'{clean_host}'; "
                    ));
                }
                query.push_str("FLUSH PRIVILEGES;");

                let output = tokio::process::Command::new(&cli_bin)
                    .args(["-u", "root", "-h", "127.0.0.1", "--port=3306", "-e", &query])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to execute MySQL user creation: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to create MySQL user: {err}"));
                }

                Ok(())
            }
            "postgresql" | "postgres" => {
                let is_running = Self::check_port_health(5432, 400).await;
                if !is_running {
                    return Err("PostgreSQL is not running on port 5432".to_string());
                }

                let psql_bin = Self::find_executable("psql", &extra_dirs).ok_or_else(|| {
                    "psql.exe binary not found on system PATH or runtimes.".to_string()
                })?;

                let safe_pw = password.replace('\'', "''");
                let mut query =
                    format!("CREATE ROLE \"{clean_user}\" WITH LOGIN PASSWORD '{safe_pw}'; ");

                if privilege_scope.eq_ignore_ascii_case("all")
                    || privilege_scope.eq_ignore_ascii_case("global")
                {
                    query.push_str(&format!(
                        "ALTER ROLE \"{clean_user}\" WITH SUPERUSER CREATEDB CREATEROLE; "
                    ));
                } else if privilege_scope.eq_ignore_ascii_case("database") {
                    if let Some(db) = target_db {
                        let clean_db = db.trim();
                        if !clean_db.is_empty() {
                            query.push_str(&format!("GRANT ALL PRIVILEGES ON DATABASE \"{clean_db}\" TO \"{clean_user}\"; "));
                        }
                    }
                }

                let output = tokio::process::Command::new(&psql_bin)
                    .args([
                        "-U",
                        "postgres",
                        "-h",
                        "127.0.0.1",
                        "-p",
                        "5432",
                        "-c",
                        &query,
                    ])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to execute PostgreSQL role creation: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to create PostgreSQL role: {err}"));
                }

                Ok(())
            }
            _ => Err(format!("Engine '{engine}' does not support creating users")),
        }
    }

    pub async fn update_user_password(
        &self,
        engine: &str,
        username: &str,
        host: &str,
        new_password: &str,
    ) -> Result<(), String> {
        let clean_user = username.trim();
        if clean_user.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        let eng = engine.to_lowercase();
        let extra_dirs = [
            PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"),
            PathBuf::from("D:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"),
        ];

        match eng.as_str() {
            "mariadb" | "mysql" => {
                let is_running = Self::check_port_health(3306, 400).await;
                if !is_running {
                    return Err("MySQL / MariaDB is not running on port 3306".to_string());
                }

                let cli_bin = Self::find_executable("mysql", &extra_dirs)
                    .or_else(|| Self::find_executable("mariadb", &extra_dirs))
                    .ok_or_else(|| {
                        "mysql.exe / mariadb.exe binary not found on system PATH or runtimes."
                            .to_string()
                    })?;

                let clean_host = if host.trim().is_empty() {
                    "%"
                } else {
                    host.trim()
                };
                let safe_pw = new_password.replace('\'', "''");
                let query = format!("ALTER USER '{clean_user}'@'{clean_host}' IDENTIFIED BY '{safe_pw}'; FLUSH PRIVILEGES;");

                let output = tokio::process::Command::new(&cli_bin)
                    .args(["-u", "root", "-h", "127.0.0.1", "--port=3306", "-e", &query])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to update MySQL password: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to update MySQL password: {err}"));
                }

                Ok(())
            }
            "postgresql" | "postgres" => {
                let is_running = Self::check_port_health(5432, 400).await;
                if !is_running {
                    return Err("PostgreSQL is not running on port 5432".to_string());
                }

                let psql_bin = Self::find_executable("psql", &extra_dirs).ok_or_else(|| {
                    "psql.exe binary not found on system PATH or runtimes.".to_string()
                })?;

                let safe_pw = new_password.replace('\'', "''");
                let query = format!("ALTER ROLE \"{clean_user}\" WITH PASSWORD '{safe_pw}';");

                let output = tokio::process::Command::new(&psql_bin)
                    .args([
                        "-U",
                        "postgres",
                        "-h",
                        "127.0.0.1",
                        "-p",
                        "5432",
                        "-c",
                        &query,
                    ])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to update PostgreSQL password: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to update PostgreSQL password: {err}"));
                }

                Ok(())
            }
            _ => Err(format!(
                "Engine '{engine}' does not support updating passwords"
            )),
        }
    }

    pub async fn drop_user(&self, engine: &str, username: &str, host: &str) -> Result<(), String> {
        let clean_user = username.trim();
        if clean_user.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        if clean_user.eq_ignore_ascii_case("root") || clean_user.eq_ignore_ascii_case("postgres") {
            return Err(
                "Cannot delete system default administrator user ('root' or 'postgres')."
                    .to_string(),
            );
        }

        let eng = engine.to_lowercase();
        let extra_dirs = [
            PathBuf::from("C:\\4forge\\runtimes\\mariadb\\bin"),
            PathBuf::from("D:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\laragon\\bin\\mysql"),
            PathBuf::from("C:\\4forge\\runtimes\\postgresql\\bin"),
        ];

        match eng.as_str() {
            "mariadb" | "mysql" => {
                let is_running = Self::check_port_health(3306, 400).await;
                if !is_running {
                    return Err("MySQL / MariaDB is not running on port 3306".to_string());
                }

                let cli_bin = Self::find_executable("mysql", &extra_dirs)
                    .or_else(|| Self::find_executable("mariadb", &extra_dirs))
                    .ok_or_else(|| {
                        "mysql.exe / mariadb.exe binary not found on system PATH or runtimes."
                            .to_string()
                    })?;

                let clean_host = if host.trim().is_empty() {
                    "%"
                } else {
                    host.trim()
                };
                let query =
                    format!("DROP USER IF EXISTS '{clean_user}'@'{clean_host}'; FLUSH PRIVILEGES;");

                let output = tokio::process::Command::new(&cli_bin)
                    .args(["-u", "root", "-h", "127.0.0.1", "--port=3306", "-e", &query])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to drop MySQL user: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to drop MySQL user: {err}"));
                }

                Ok(())
            }
            "postgresql" | "postgres" => {
                let is_running = Self::check_port_health(5432, 400).await;
                if !is_running {
                    return Err("PostgreSQL is not running on port 5432".to_string());
                }

                let psql_bin = Self::find_executable("psql", &extra_dirs).ok_or_else(|| {
                    "psql.exe binary not found on system PATH or runtimes.".to_string()
                })?;

                let query = format!("DROP ROLE IF EXISTS \"{clean_user}\";");

                let output = tokio::process::Command::new(&psql_bin)
                    .args([
                        "-U",
                        "postgres",
                        "-h",
                        "127.0.0.1",
                        "-p",
                        "5432",
                        "-c",
                        &query,
                    ])
                    .output()
                    .await
                    .map_err(|e| format!("Failed to drop PostgreSQL role: {e}"))?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to drop PostgreSQL role: {err}"));
                }

                Ok(())
            }
            _ => Err(format!("Engine '{engine}' does not support deleting users")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mariadb::MariaDbDriver;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_database_user_validation() {
        let manager = DatabaseManager::new();

        let res_empty = manager
            .create_user("mariadb", "", "%", "password", "all", None)
            .await;
        assert!(res_empty.is_err());
        assert!(res_empty.unwrap_err().contains("empty"));

        let res_invalid = manager
            .create_user("mariadb", "bad user; DROP TABLE", "%", "pass", "all", None)
            .await;
        assert!(res_invalid.is_err());
        assert!(res_invalid.unwrap_err().contains("alphanumeric"));

        let res_drop_root = manager.drop_user("mariadb", "root", "localhost").await;
        assert!(res_drop_root.is_err());
        assert!(res_drop_root
            .unwrap_err()
            .contains("system default administrator"));

        let res_drop_pg = manager.drop_user("postgres", "postgres", "all").await;
        assert!(res_drop_pg.is_err());
        assert!(res_drop_pg
            .unwrap_err()
            .contains("system default administrator"));
    }

    #[tokio::test]
    async fn test_database_manager_registration() {
        let mut manager = DatabaseManager::new();
        let base = PathBuf::from("C:\\tools\\mariadb");
        let data = PathBuf::from("C:\\tools\\mariadb\\data");
        let driver = Arc::new(MariaDbDriver::new(base, data, 3306));

        manager.register_driver(driver);

        assert!(manager.get_driver(DatabaseEngine::MariaDb).is_some());
        assert!(manager.get_driver(DatabaseEngine::PostgreSql).is_none());

        let start_cmd = manager.get_start_command(DatabaseEngine::MariaDb);
        assert!(start_cmd.is_ok());

        let instances = manager.list_instances().await;
        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].engine, DatabaseEngine::MariaDb);
        assert_eq!(instances[0].port, Some(3306));
    }

    #[tokio::test]
    async fn test_database_manager_create_database_validation() {
        let manager = DatabaseManager::new();

        let res_empty = manager.create_database("mariadb", "").await;
        assert!(res_empty.is_err());
        assert!(res_empty.unwrap_err().contains("empty"));

        let res_invalid = manager
            .create_database("mariadb", "db; DROP TABLE users;")
            .await;
        assert!(res_invalid.is_err());
        assert!(res_invalid.unwrap_err().contains("alphanumeric"));

        let res_unsupported = manager.create_database("oracle", "my_db").await;
        assert!(res_unsupported.is_err());
        assert!(res_unsupported.unwrap_err().contains("Unsupported"));

        let res_unregistered = manager.create_database("mariadb", "valid_db").await;
        assert!(res_unregistered.is_err());
        let err = res_unregistered.unwrap_err();
        assert!(
            err.contains("not running")
                || err.contains("not registered")
                || err.contains("not found")
        );
    }

    #[tokio::test]
    async fn test_database_registry_and_listing() {
        let manager = DatabaseManager::new();
        let list = manager.list_all_databases().await;
        let _ = list.len();
    }

    #[tokio::test]
    async fn test_database_manager_mongodb() {
        let mut manager = DatabaseManager::new();
        let base = PathBuf::from("C:\\tools\\mongodb");
        let data = PathBuf::from("C:\\tools\\mongodb\\data");
        let driver = Arc::new(crate::mongodb::MongoDbDriver::new(base, data, 27017));
        manager.register_driver(driver);

        assert!(manager.get_driver(DatabaseEngine::MongoDb).is_some());
        let instances = manager.list_instances().await;
        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].engine, DatabaseEngine::MongoDb);
        assert_eq!(instances[0].port, Some(27017));
    }
}
