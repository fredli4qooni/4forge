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
use crate::{DatabaseEngine, DatabaseError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct MongoDbDriver {
    base_dir: PathBuf,
    data_dir: PathBuf,
    port: u16,
    config_path: PathBuf,
}

impl MongoDbDriver {
    pub fn new(base_dir: PathBuf, data_dir: PathBuf, port: u16) -> Self {
        let config_path = PathBuf::from("C:\\4forge\\config\\mongod.cfg");
        Self {
            base_dir,
            data_dir,
            port,
            config_path,
        }
    }

    pub fn with_config_path(
        base_dir: PathBuf,
        data_dir: PathBuf,
        port: u16,
        config_path: PathBuf,
    ) -> Self {
        Self {
            base_dir,
            data_dir,
            port,
            config_path,
        }
    }

    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    pub fn generate_yaml_config(&self) -> String {
        let clean_path = self.data_dir.to_string_lossy().replace('\\', "/");
        format!(
            "storage:\n  dbPath: \"{}\"\nnet:\n  port: {}\n  bindIp: 127.0.0.1\n",
            clean_path, self.port
        )
    }
}

impl DatabaseDriver for MongoDbDriver {
    fn engine(&self) -> DatabaseEngine {
        DatabaseEngine::MongoDb
    }

    fn port(&self) -> Option<u16> {
        Some(self.port)
    }

    fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    fn is_initialized(&self) -> bool {
        self.data_dir.is_dir()
    }

    fn prepare_environment(&self) -> Result<(), DatabaseError> {
        std::fs::create_dir_all(&self.data_dir)?;
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if !self.config_path.exists() {
            let content = self.generate_yaml_config();
            std::fs::write(&self.config_path, content)?;
        }
        Ok(())
    }

    fn build_init_command(&self) -> Option<DatabaseCommand> {
        None
    }

    fn build_start_command(&self) -> Result<DatabaseCommand, DatabaseError> {
        let mongod_bin = self.base_dir.join("bin").join("mongod.exe");
        let program = if mongod_bin.exists() {
            mongod_bin
        } else {
            PathBuf::from("mongod.exe")
        };

        Ok(DatabaseCommand {
            program,
            args: vec![
                "--config".to_string(),
                self.config_path.to_string_lossy().to_string(),
            ],
            current_dir: Some(self.base_dir.clone()),
            envs: HashMap::new(),
        })
    }

    fn build_stop_command(&self) -> Option<DatabaseCommand> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mongodb_driver_commands() {
        let base = PathBuf::from("C:\\4forge\\runtimes\\mongodb");
        let data = PathBuf::from("C:\\4forge\\data\\mongodb");
        let driver = MongoDbDriver::new(base.clone(), data.clone(), 27017);

        assert_eq!(driver.engine(), DatabaseEngine::MongoDb);
        assert_eq!(driver.port(), Some(27017));
        assert_eq!(driver.data_dir(), data.as_path());
        assert!(driver.build_init_command().is_none());

        let start_cmd = driver.build_start_command().unwrap();
        assert!(start_cmd.args.contains(&"--config".to_string()));

        let yaml = driver.generate_yaml_config();
        assert!(yaml.contains("port: 27017"));
        assert!(yaml.contains("127.0.0.1"));
    }
}
