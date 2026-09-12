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

use forge_caddy_config::VirtualHostConfig;
use forge_db_manager::DatabaseManager;
use forge_runtime_manager::RuntimeManager;
use forge_supervisor::SupervisorManager;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub supervisor: Arc<SupervisorManager>,
    pub db_manager: Arc<RwLock<DatabaseManager>>,
    pub runtime_manager: Arc<RwLock<RuntimeManager>>,
    pub sites: Arc<RwLock<Vec<VirtualHostConfig>>>,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let supervisor = Arc::new(SupervisorManager::new()?);
        let db_manager = Arc::new(RwLock::new(DatabaseManager::new()));

        let runtimes_root = dirs_next_or_default();
        let runtime_manager = Arc::new(RwLock::new(RuntimeManager::new(runtimes_root)));

        let default_sites = vec![
            VirtualHostConfig::new_php(
                "laravel-app.test",
                "C:\\projects\\laravel-app",
                "127.0.0.1:9000",
            ),
            VirtualHostConfig::new_proxy(
                "dashboard-api.test",
                "C:\\projects\\dashboard-api",
                "127.0.0.1:3000",
            ),
        ];

        Ok(Self {
            supervisor,
            db_manager,
            runtime_manager,
            sites: Arc::new(RwLock::new(default_sites)),
        })
    }
}

fn dirs_next_or_default() -> PathBuf {
    if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(appdata).join("4Forge").join("runtimes")
    } else {
        PathBuf::from("C:\\4Forge\\runtimes")
    }
}
