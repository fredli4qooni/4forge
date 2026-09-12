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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseEngine {
    MariaDb,
    PostgreSql,
    Sqlite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInstance {
    pub engine: DatabaseEngine,
    pub port: u16,
    pub data_dir: String,
    pub is_running: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_instance() {
        let db = DatabaseInstance {
            engine: DatabaseEngine::MariaDb,
            port: 3306,
            data_dir: "C:\\data\\mariadb".to_string(),
            is_running: false,
        };
        assert_eq!(db.port, 3306);
        assert_eq!(db.engine, DatabaseEngine::MariaDb);
    }
}
