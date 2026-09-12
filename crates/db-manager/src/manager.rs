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
}
