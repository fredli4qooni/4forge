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

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct NodeRuntime {
    version: String,
    base_dir: PathBuf,
}

impl NodeRuntime {
    pub fn new(version: &str, base_dir: PathBuf) -> Self {
        Self {
            version: version.to_string(),
            base_dir,
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn node_bin(&self) -> PathBuf {
        self.base_dir.join("node.exe")
    }

    pub fn npm_cmd(&self) -> PathBuf {
        self.base_dir.join("npm.cmd")
    }

    pub fn npx_cmd(&self) -> PathBuf {
        self.base_dir.join("npx.cmd")
    }

    pub fn corepack_cmd(&self) -> PathBuf {
        self.base_dir.join("corepack.cmd")
    }

    pub fn is_valid(&self) -> bool {
        self.node_bin().is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_runtime_binary_paths() {
        let base = PathBuf::from("C:\\4forge\\runtimes\\node\\22.14.0");
        let node = NodeRuntime::new("22.14.0", base);

        assert_eq!(node.version(), "22.14.0");
        assert!(node.node_bin().ends_with("node.exe"));
        assert!(node.npm_cmd().ends_with("npm.cmd"));
        assert!(node.npx_cmd().ends_with("npx.cmd"));
    }
}
