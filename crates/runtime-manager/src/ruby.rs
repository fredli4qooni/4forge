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

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RubyRuntime {
    version: String,
    base_dir: PathBuf,
}

impl RubyRuntime {
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

    pub fn ruby_bin(&self) -> PathBuf {
        let bin = self.base_dir.join("bin").join("ruby.exe");
        if bin.is_file() {
            bin
        } else {
            self.base_dir.join("ruby.exe")
        }
    }

    pub fn gem_cmd(&self) -> PathBuf {
        self.base_dir.join("bin").join("gem.cmd")
    }

    pub fn bundle_cmd(&self) -> PathBuf {
        self.base_dir.join("bin").join("bundle.cmd")
    }

    pub fn build_puma_command(&self, port: u16) -> (PathBuf, Vec<String>) {
        (
            self.bundle_cmd(),
            vec![
                "exec".to_string(),
                "puma".to_string(),
                "-b".to_string(),
                format!("tcp://127.0.0.1:{}", port),
            ],
        )
    }

    pub fn get_env_vars(&self, project_dir: Option<&Path>) -> HashMap<String, String> {
        let mut envs = HashMap::new();
        let mut path_entries = Vec::new();

        if let Some(p) = project_dir {
            let gem_home = p.join(".bundle").join("gem");
            envs.insert(
                "GEM_HOME".to_string(),
                gem_home.to_string_lossy().to_string(),
            );
            path_entries.push(gem_home.join("bin").to_string_lossy().to_string());
        }

        path_entries.push(self.base_dir.join("bin").to_string_lossy().to_string());
        envs.insert("PATH".to_string(), path_entries.join(";"));
        envs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruby_runtime_commands() {
        let base = PathBuf::from("C:\\4forge\\runtimes\\ruby\\3.3.7");
        let rb = RubyRuntime::new("3.3.7", base);

        assert_eq!(rb.version(), "3.3.7");
        assert!(rb.gem_cmd().ends_with("gem.cmd"));
        assert!(rb.bundle_cmd().ends_with("bundle.cmd"));

        let (puma_cmd, puma_args) = rb.build_puma_command(3000);
        assert!(puma_cmd.ends_with("bundle.cmd"));
        assert!(puma_args.contains(&"puma".to_string()));
        assert!(puma_args.contains(&"tcp://127.0.0.1:3000".to_string()));
    }
}
