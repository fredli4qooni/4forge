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

use crate::config::PostgreSqlConfig;
use crate::driver::{DatabaseCommand, DatabaseDriver};
use crate::{DatabaseEngine, DatabaseError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PostgreSqlDriver {
    base_dir: PathBuf,
    data_dir: PathBuf,
    config: PostgreSqlConfig,
}

impl PostgreSqlDriver {
    pub fn new(base_dir: PathBuf, data_dir: PathBuf, port: u16) -> Self {
        let config = PostgreSqlConfig::new(base_dir.clone(), data_dir.clone(), port);
        Self {
            base_dir,
            data_dir,
            config,
        }
    }

    pub fn conf_path(&self) -> PathBuf {
        self.data_dir.join("postgresql.conf")
    }

    pub fn config(&self) -> &PostgreSqlConfig {
        &self.config
    }
}

impl DatabaseDriver for PostgreSqlDriver {
    fn engine(&self) -> DatabaseEngine {
        DatabaseEngine::PostgreSql
    }

    fn port(&self) -> Option<u16> {
        Some(self.config.port)
    }

    fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    fn is_initialized(&self) -> bool {
        self.data_dir.exists() && self.data_dir.join("PG_VERSION").is_file()
    }

    fn prepare_environment(&self) -> Result<(), DatabaseError> {
        std::fs::create_dir_all(&self.data_dir)?;
        let conf_content = self.config.to_conf_string();
        std::fs::write(self.conf_path(), conf_content)?;
        Ok(())
    }

    fn build_init_command(&self) -> Option<DatabaseCommand> {
        let init_bin = self.base_dir.join("bin").join("initdb.exe");

        Some(DatabaseCommand {
            program: init_bin,
            args: vec![
                "-D".to_string(),
                self.data_dir.display().to_string(),
                "-U".to_string(),
                "postgres".to_string(),
                "-E".to_string(),
                "UTF8".to_string(),
                "--auth=trust".to_string(),
            ],
            current_dir: Some(self.base_dir.clone()),
            envs: HashMap::new(),
        })
    }

    fn build_start_command(&self) -> Result<DatabaseCommand, DatabaseError> {
        let pg_bin = self.base_dir.join("bin").join("postgres.exe");

        Ok(DatabaseCommand {
            program: pg_bin,
            args: vec![
                "-D".to_string(),
                self.data_dir.display().to_string(),
                "-p".to_string(),
                self.config.port.to_string(),
            ],
            current_dir: Some(self.base_dir.clone()),
            envs: HashMap::new(),
        })
    }

    fn build_stop_command(&self) -> Option<DatabaseCommand> {
        let ctl_bin = self.base_dir.join("bin").join("pg_ctl.exe");

        Some(DatabaseCommand {
            program: ctl_bin,
            args: vec![
                "stop".to_string(),
                "-D".to_string(),
                self.data_dir.display().to_string(),
                "-m".to_string(),
                "fast".to_string(),
            ],
            current_dir: Some(self.base_dir.clone()),
            envs: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postgresql_driver_commands() {
        let base = PathBuf::from("C:\\tools\\postgres");
        let data = PathBuf::from("C:\\tools\\postgres\\data");
        let driver = PostgreSqlDriver::new(base, data, 5432);

        assert_eq!(driver.engine(), DatabaseEngine::PostgreSql);
        assert_eq!(driver.port(), Some(5432));

        let start_cmd = driver.build_start_command().expect("start command failed");
        assert!(start_cmd.program.ends_with("postgres.exe"));

        let stop_cmd = driver.build_stop_command().expect("stop command failed");
        assert!(stop_cmd.program.ends_with("pg_ctl.exe"));
        assert!(stop_cmd.args.contains(&"fast".to_string()));

        let init_cmd = driver.build_init_command().expect("init command failed");
        assert!(init_cmd.program.ends_with("initdb.exe"));
    }
}
