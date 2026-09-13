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

pub mod adminer;
pub mod config;
pub mod driver;
pub mod manager;
pub mod mariadb;
pub mod mongodb;
pub mod postgres;
pub mod sqlite;

pub use adminer::{AdminerManager, DatabaseLaunchResult};
pub use config::{MariaDbConfig, PostgreSqlConfig};
pub use driver::{DatabaseCommand, DatabaseDriver};
pub use manager::{DatabaseManager, UserDatabaseDto};
pub use mariadb::MariaDbDriver;
pub use mongodb::MongoDbDriver;
pub use postgres::PostgreSqlDriver;
pub use sqlite::SqliteDriver;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database driver for engine '{0:?}' not found")]
    DriverNotFound(DatabaseEngine),
    #[error("Serverless database engine has no background daemon")]
    ServerlessEngine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatabaseEngine {
    MariaDb,
    PostgreSql,
    Sqlite,
    MongoDb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInstanceInfo {
    pub engine: DatabaseEngine,
    pub port: Option<u16>,
    pub data_dir: String,
    pub is_initialized: bool,
    pub is_running: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_instance_info() {
        let info = DatabaseInstanceInfo {
            engine: DatabaseEngine::MariaDb,
            port: Some(3306),
            data_dir: "C:\\data\\mariadb".to_string(),
            is_initialized: true,
            is_running: false,
        };
        assert_eq!(info.port, Some(3306));
        assert_eq!(info.engine, DatabaseEngine::MariaDb);
        assert!(info.is_initialized);
    }
}
