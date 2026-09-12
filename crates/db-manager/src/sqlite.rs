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
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SqliteDriver {
    data_dir: PathBuf,
}

impl SqliteDriver {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }
}

impl DatabaseDriver for SqliteDriver {
    fn engine(&self) -> DatabaseEngine {
        DatabaseEngine::Sqlite
    }

    fn port(&self) -> Option<u16> {
        None
    }

    fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    fn is_initialized(&self) -> bool {
        self.data_dir.exists()
    }

    fn prepare_environment(&self) -> Result<(), DatabaseError> {
        std::fs::create_dir_all(&self.data_dir)?;
        Ok(())
    }

    fn build_init_command(&self) -> Option<DatabaseCommand> {
        None
    }

    fn build_start_command(&self) -> Result<DatabaseCommand, DatabaseError> {
        Err(DatabaseError::ServerlessEngine)
    }

    fn build_stop_command(&self) -> Option<DatabaseCommand> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_driver() {
        let data = PathBuf::from("C:\\tools\\sqlite\\data");
        let driver = SqliteDriver::new(data);

        assert_eq!(driver.engine(), DatabaseEngine::Sqlite);
        assert_eq!(driver.port(), None);
        assert!(matches!(
            driver.build_start_command(),
            Err(DatabaseError::ServerlessEngine)
        ));
    }
}
