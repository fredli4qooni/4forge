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

use crate::manifest::RuntimeKind;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstalledRuntime {
    pub kind: RuntimeKind,
    pub version: String,
    pub path: PathBuf,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct RuntimeStore {
    root_dir: PathBuf,
}

impl RuntimeStore {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    pub fn runtime_dir(&self, kind: RuntimeKind, version: &str) -> PathBuf {
        self.root_dir.join(kind.to_string()).join(version)
    }

    pub fn is_installed(&self, kind: RuntimeKind, version: &str) -> bool {
        let dir = self.runtime_dir(kind, version);
        if !dir.is_dir() {
            return false;
        }

        match kind {
            RuntimeKind::Php => dir.join("php.exe").is_file(),
            RuntimeKind::Node => dir.join("node.exe").is_file(),
            RuntimeKind::Python => dir.join("python.exe").is_file(),
            RuntimeKind::Ruby => {
                dir.join("bin").join("ruby.exe").is_file() || dir.join("ruby.exe").is_file()
            }
        }
    }

    pub fn list_installed(&self, kind: RuntimeKind) -> Result<Vec<String>, std::io::Error> {
        let kind_dir = self.root_dir.join(kind.to_string());
        if !kind_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut versions = Vec::new();
        for entry in std::fs::read_dir(&kind_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if self.is_installed(kind, name) {
                        versions.push(name.to_string());
                    }
                }
            }
        }
        versions.sort();
        Ok(versions)
    }

    pub fn list_all(
        &self,
        active_versions: &std::collections::HashMap<RuntimeKind, String>,
    ) -> Result<Vec<InstalledRuntime>, std::io::Error> {
        let mut list = Vec::new();
        let kinds = [
            RuntimeKind::Php,
            RuntimeKind::Node,
            RuntimeKind::Python,
            RuntimeKind::Ruby,
        ];

        for kind in kinds {
            let versions = self.list_installed(kind)?;
            let active = active_versions.get(&kind);
            for version in versions {
                let is_active = active.map(|v| v == &version).unwrap_or(false);
                let path = self.runtime_dir(kind, &version);
                list.push(InstalledRuntime {
                    kind,
                    version,
                    path,
                    is_active,
                });
            }
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_runtime_store_paths() {
        let temp = tempdir().expect("tempdir failed");
        let store = RuntimeStore::new(temp.path().to_path_buf());

        let php_path = store.runtime_dir(RuntimeKind::Php, "8.3.16");
        assert!(php_path.ends_with("php\\8.3.16"));

        let node_path = store.runtime_dir(RuntimeKind::Node, "22.14.0");
        assert!(node_path.ends_with("node\\22.14.0"));
    }
}
