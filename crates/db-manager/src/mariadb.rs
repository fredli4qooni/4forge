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

use crate::config::MariaDbConfig;
use crate::driver::{DatabaseCommand, DatabaseDriver};
use crate::{DatabaseEngine, DatabaseError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct MariaDbDriver {
    base_dir: PathBuf,
    data_dir: PathBuf,
    config: MariaDbConfig,
}

impl MariaDbDriver {
    pub fn new(base_dir: PathBuf, data_dir: PathBuf, port: u16) -> Self {
        let config = MariaDbConfig::new(base_dir.clone(), data_dir.clone(), port);
        Self {
            base_dir,
            data_dir,
            config,
        }
    }

    pub fn ini_path(&self) -> PathBuf {
        self.data_dir.join("my.ini")
    }

    pub fn config(&self) -> &MariaDbConfig {
        &self.config
    }

    pub fn build_create_database_command(&self, db_name: &str) -> DatabaseCommand {
        let admin_bin = self.base_dir.join("bin").join("mysqladmin.exe");
        DatabaseCommand {
            program: admin_bin,
            args: vec![
                "-u".to_string(),
                "root".to_string(),
                format!("--port={}", self.config.port),
                "-h".to_string(),
                "127.0.0.1".to_string(),
                "create".to_string(),
                db_name.to_string(),
            ],
            current_dir: Some(self.base_dir.clone()),
            envs: HashMap::new(),
        }
    }
}

impl DatabaseDriver for MariaDbDriver {
    fn engine(&self) -> DatabaseEngine {
        DatabaseEngine::MariaDb
    }

    fn port(&self) -> Option<u16> {
        Some(self.config.port)
    }

    fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    fn is_initialized(&self) -> bool {
        self.data_dir.exists()
            && (self.data_dir.join("mysql").is_dir()
                || self.data_dir.join("aria_log_control").is_file())
    }

    fn prepare_environment(&self) -> Result<(), DatabaseError> {
        std::fs::create_dir_all(&self.data_dir)?;
        let ini_content = self.config.to_ini_string();
        std::fs::write(self.ini_path(), ini_content)?;
        Ok(())
    }

    fn build_init_command(&self) -> Option<DatabaseCommand> {
        let init_bin = self.base_dir.join("bin").join("mysql_install_db.exe");
        let datadir_arg = format!("--datadir={}", self.data_dir.display());
        let basedir_arg = format!("--basedir={}", self.base_dir.display());

        Some(DatabaseCommand {
            program: init_bin,
            args: vec![datadir_arg, basedir_arg, "--default-user".to_string()],
            current_dir: Some(self.base_dir.clone()),
            envs: HashMap::new(),
        })
    }

    fn build_start_command(&self) -> Result<DatabaseCommand, DatabaseError> {
        let mysqld_bin = self.base_dir.join("bin").join("mysqld.exe");
        let defaults_arg = format!("--defaults-file={}", self.ini_path().display());

        Ok(DatabaseCommand {
            program: mysqld_bin,
            args: vec![defaults_arg, "--console".to_string()],
            current_dir: Some(self.base_dir.clone()),
            envs: HashMap::new(),
        })
    }

    fn build_stop_command(&self) -> Option<DatabaseCommand> {
        let admin_bin = self.base_dir.join("bin").join("mysqladmin.exe");

        Some(DatabaseCommand {
            program: admin_bin,
            args: vec![
                "-u".to_string(),
                "root".to_string(),
                format!("--port={}", self.config.port),
                "shutdown".to_string(),
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
    fn test_mariadb_driver_commands() {
        let base = PathBuf::from("C:\\tools\\mariadb");
        let data = PathBuf::from("C:\\tools\\mariadb\\data");
        let driver = MariaDbDriver::new(base, data, 3306);

        assert_eq!(driver.engine(), DatabaseEngine::MariaDb);
        assert_eq!(driver.port(), Some(3306));

        let start_cmd = driver.build_start_command().expect("start command failed");
        assert!(start_cmd.program.ends_with("mysqld.exe"));
        assert!(start_cmd.args[0].contains("--defaults-file="));

        let stop_cmd = driver.build_stop_command().expect("stop command failed");
        assert!(stop_cmd.program.ends_with("mysqladmin.exe"));
        assert!(stop_cmd.args.contains(&"shutdown".to_string()));

        let create_cmd = driver.build_create_database_command("app_db");
        assert!(create_cmd.program.ends_with("mysqladmin.exe"));
        assert!(create_cmd.args.contains(&"create".to_string()));
        assert!(create_cmd.args.contains(&"app_db".to_string()));

        let init_cmd = driver.build_init_command().expect("init command failed");
        assert!(init_cmd.program.ends_with("mysql_install_db.exe"));
    }
}
