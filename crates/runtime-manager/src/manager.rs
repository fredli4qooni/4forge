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

use crate::manifest::{RuntimeKind, RuntimeManifest, RuntimePackage};
use crate::php::PhpConfig;
use crate::shim::EnvironmentShim;
use crate::store::{InstalledRuntime, RuntimeStore};
use crate::RuntimeError;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct RuntimeManager {
    store: RuntimeStore,
    active_versions: HashMap<RuntimeKind, String>,
}

impl RuntimeManager {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            store: RuntimeStore::new(root_dir),
            active_versions: HashMap::new(),
        }
    }

    pub fn store(&self) -> &RuntimeStore {
        &self.store
    }

    pub fn get_active_version(&self, kind: RuntimeKind) -> Option<String> {
        self.active_versions.get(&kind).cloned()
    }

    pub fn set_active_version(
        &mut self,
        kind: RuntimeKind,
        version: &str,
    ) -> Result<(), RuntimeError> {
        if !self.store.is_installed(kind, version) {
            return Err(RuntimeError::NotInstalled(kind, version.to_string()));
        }
        self.active_versions.insert(kind, version.to_string());
        Ok(())
    }

    pub fn list_available(&self, kind: RuntimeKind) -> Vec<RuntimePackage> {
        RuntimeManifest::get_available_packages(kind)
    }

    pub fn list_installed(&self, kind: RuntimeKind) -> Result<Vec<String>, std::io::Error> {
        self.store.list_installed(kind)
    }

    pub fn list_all_installed(&self) -> Result<Vec<InstalledRuntime>, std::io::Error> {
        self.store.list_all(&self.active_versions)
    }

    pub fn get_active_path(&self, kind: RuntimeKind) -> Option<PathBuf> {
        let version = self.active_versions.get(&kind)?;
        Some(self.store.runtime_dir(kind, version))
    }

    pub fn build_fastcgi_service(
        &self,
        version: Option<&str>,
        port: u16,
    ) -> Result<(PathBuf, Vec<String>), RuntimeError> {
        let ver = if let Some(v) = version {
            v.to_string()
        } else if let Some(v) = self.active_versions.get(&RuntimeKind::Php) {
            v.clone()
        } else {
            return Err(RuntimeError::NoActiveVersion(RuntimeKind::Php));
        };

        if !self.store.is_installed(RuntimeKind::Php, &ver) {
            return Err(RuntimeError::NotInstalled(RuntimeKind::Php, ver));
        }

        let dir = self.store.runtime_dir(RuntimeKind::Php, &ver);
        let config = PhpConfig::new(&ver, dir, port);
        Ok(config.build_fastcgi_command())
    }

    pub fn get_project_env(
        &self,
        php_ver: Option<&str>,
        node_ver: Option<&str>,
        current_path: Option<&str>,
    ) -> HashMap<String, String> {
        let mut dirs = Vec::new();

        if let Some(pv) = php_ver {
            dirs.push(self.store.runtime_dir(RuntimeKind::Php, pv));
        } else if let Some(pv) = self.active_versions.get(&RuntimeKind::Php) {
            dirs.push(self.store.runtime_dir(RuntimeKind::Php, pv));
        }

        if let Some(nv) = node_ver {
            dirs.push(self.store.runtime_dir(RuntimeKind::Node, nv));
        } else if let Some(nv) = self.active_versions.get(&RuntimeKind::Node) {
            dirs.push(self.store.runtime_dir(RuntimeKind::Node, nv));
        }

        let new_path = EnvironmentShim::build_path_env(&dirs, current_path);
        let mut map = HashMap::new();
        map.insert("PATH".to_string(), new_path);
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_runtime_manager_active_and_env() {
        let temp = tempdir().expect("tempdir failed");
        let manager = RuntimeManager::new(temp.path().to_path_buf());

        let env = manager.get_project_env(Some("8.3.16"), Some("22.14.0"), Some("C:\\Windows"));
        let path = env.get("PATH").unwrap();

        assert!(path.contains("php\\8.3.16"));
        assert!(path.contains("node\\22.14.0"));
        assert!(path.ends_with("C:\\Windows"));
    }
}
