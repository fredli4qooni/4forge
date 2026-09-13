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
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

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
        match engine_lower.as_str() {
            "mariadb" | "mysql" => {
                if let Some(driver) = self.drivers.get(&DatabaseEngine::MariaDb) {
                    let port = driver.port().unwrap_or(3306);
                    let is_running = Self::check_port_health(port, 400).await;
                    if !is_running {
                        return Err(format!(
                            "MariaDB is not running on port {port}. Please start MariaDB service first."
                        ));
                    }

                    let base = driver
                        .data_dir()
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| driver.data_dir().to_path_buf());
                    let candidates = vec![
                        base.join("bin").join("mysqladmin.exe"),
                        base.join("bin").join("mariadb-admin.exe"),
                        std::path::PathBuf::from("mysqladmin.exe"),
                        std::path::PathBuf::from("mariadb-admin.exe"),
                    ];

                    for candidate in candidates {
                        if candidate.exists() {
                            let output = tokio::process::Command::new(&candidate)
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
                                .await;

                            if let Ok(out) = output {
                                if out.status.success() {
                                    return Ok(());
                                }
                                let err_msg = String::from_utf8_lossy(&out.stderr);
                                if err_msg.contains("database exists")
                                    || err_msg.contains("already exists")
                                {
                                    return Ok(());
                                }
                                return Err(format!("Failed to create database: {err_msg}"));
                            }
                        }
                    }

                    Ok(())
                } else {
                    Err("MariaDB driver is not registered".to_string())
                }
            }
            "postgresql" | "postgres" => {
                if let Some(driver) = self.drivers.get(&DatabaseEngine::PostgreSql) {
                    let port = driver.port().unwrap_or(5432);
                    let is_running = Self::check_port_health(port, 400).await;
                    if !is_running {
                        return Err(format!(
                            "PostgreSQL is not running on port {port}. Please start PostgreSQL service first."
                        ));
                    }

                    let base = driver
                        .data_dir()
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| driver.data_dir().to_path_buf());
                    let candidates = vec![
                        base.join("bin").join("createdb.exe"),
                        std::path::PathBuf::from("createdb.exe"),
                    ];

                    for candidate in candidates {
                        if candidate.exists() {
                            let output = tokio::process::Command::new(&candidate)
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
                                .await;

                            if let Ok(out) = output {
                                if out.status.success() {
                                    return Ok(());
                                }
                                let err_msg = String::from_utf8_lossy(&out.stderr);
                                if err_msg.contains("already exists") {
                                    return Ok(());
                                }
                                return Err(format!(
                                    "Failed to create PostgreSQL database: {err_msg}"
                                ));
                            }
                        }
                    }

                    Ok(())
                } else {
                    Err("PostgreSQL driver is not registered".to_string())
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
                if let Some(driver) = self.drivers.get(&DatabaseEngine::MongoDb) {
                    let port = driver.port().unwrap_or(27017);
                    let is_running = Self::check_port_health(port, 400).await;
                    if !is_running {
                        return Err(format!(
                            "MongoDB is not running on port {port}. Please start MongoDB service first."
                        ));
                    }
                }
                Ok(())
            }
            _ => Err(format!("Unsupported database engine: '{engine}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mariadb::MariaDbDriver;
    use std::path::PathBuf;

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
        assert!(res_unregistered.unwrap_err().contains("not registered"));
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
