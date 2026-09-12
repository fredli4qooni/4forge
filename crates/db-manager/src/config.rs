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

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MariaDbConfig {
    pub port: u16,
    pub bind_address: String,
    pub base_dir: PathBuf,
    pub data_dir: PathBuf,
    pub max_connections: u32,
    pub character_set_server: String,
    pub collation_server: String,
    pub default_storage_engine: String,
}

impl MariaDbConfig {
    pub fn new(base_dir: PathBuf, data_dir: PathBuf, port: u16) -> Self {
        Self {
            port,
            bind_address: "127.0.0.1".to_string(),
            base_dir,
            data_dir,
            max_connections: 150,
            character_set_server: "utf8mb4".to_string(),
            collation_server: "utf8mb4_unicode_ci".to_string(),
            default_storage_engine: "INNODB".to_string(),
        }
    }

    pub fn to_ini_string(&self) -> String {
        let base_str = self.base_dir.to_string_lossy().replace('\\', "/");
        let data_str = self.data_dir.to_string_lossy().replace('\\', "/");

        format!(
            "[mysqld]\r\n\
             port = {}\r\n\
             bind-address = {}\r\n\
             basedir = \"{}\"\r\n\
             datadir = \"{}\"\r\n\
             max_connections = {}\r\n\
             default-storage-engine = {}\r\n\
             character-set-server = {}\r\n\
             collation-server = {}\r\n\
             \r\n\
             [client]\r\n\
             port = {}\r\n\
             default-character-set = {}\r\n\
             \r\n\
             [mysqladmin]\r\n\
             port = {}\r\n",
            self.port,
            self.bind_address,
            base_str,
            data_str,
            self.max_connections,
            self.default_storage_engine,
            self.character_set_server,
            self.collation_server,
            self.port,
            self.character_set_server,
            self.port
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgreSqlConfig {
    pub port: u16,
    pub bind_address: String,
    pub base_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl PostgreSqlConfig {
    pub fn new(base_dir: PathBuf, data_dir: PathBuf, port: u16) -> Self {
        Self {
            port,
            bind_address: "127.0.0.1".to_string(),
            base_dir,
            data_dir,
        }
    }

    pub fn to_conf_string(&self) -> String {
        format!(
            "listen_addresses = '{}'\r\n\
             port = {}\r\n\
             max_connections = 100\r\n\
             shared_buffers = 128MB\r\n",
            self.bind_address, self.port
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mariadb_config_ini_generation() {
        let base = PathBuf::from("C:\\Program Files\\4Forge\\mariadb");
        let data = PathBuf::from("C:\\ProgramData\\4Forge\\data\\mariadb");
        let config = MariaDbConfig::new(base, data, 3306);
        let ini = config.to_ini_string();

        assert!(ini.contains("port = 3306"));
        assert!(ini.contains("bind-address = 127.0.0.1"));
        assert!(ini.contains("basedir = \"C:/Program Files/4Forge/mariadb\""));
        assert!(ini.contains("datadir = \"C:/ProgramData/4Forge/data/mariadb\""));
        assert!(ini.contains("character-set-server = utf8mb4"));
    }

    #[test]
    fn test_postgresql_config_generation() {
        let base = PathBuf::from("C:\\4forge\\postgres");
        let data = PathBuf::from("C:\\4forge\\data\\postgres");
        let config = PostgreSqlConfig::new(base, data, 5432);
        let conf = config.to_conf_string();

        assert!(conf.contains("port = 5432"));
        assert!(conf.contains("listen_addresses = '127.0.0.1'"));
    }
}
