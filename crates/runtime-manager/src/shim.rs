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

pub struct EnvironmentShim;

impl EnvironmentShim {
    pub fn build_path_env(runtime_dirs: &[PathBuf], current_path: Option<&str>) -> String {
        let mut parts = Vec::new();
        for dir in runtime_dirs {
            let s = dir.to_string_lossy();
            if !s.is_empty() {
                parts.push(s.to_string());
            }
        }

        if let Some(existing) = current_path {
            if !existing.is_empty() {
                parts.push(existing.to_string());
            }
        }

        parts.join(";")
    }

    pub fn generate_batch_shim(
        target_binary: &Path,
        shim_dest: &Path,
    ) -> Result<(), std::io::Error> {
        let content = format!(
            "@echo off\r\n\
             \"{}\" %*\r\n",
            target_binary.display()
        );
        std::fs::write(shim_dest, content)?;
        Ok(())
    }

    pub fn generate_ps1_shim(target_binary: &Path, shim_dest: &Path) -> Result<(), std::io::Error> {
        let content = format!("& \"{}\" @args\r\n", target_binary.display());
        std::fs::write(shim_dest, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_build_path_env() {
        let dirs = vec![
            PathBuf::from("C:\\4forge\\runtimes\\php\\8.3.16"),
            PathBuf::from("C:\\4forge\\runtimes\\node\\22.14.0"),
        ];
        let original_path = "C:\\Windows\\System32;C:\\Windows";
        let new_path = EnvironmentShim::build_path_env(&dirs, Some(original_path));

        assert!(new_path
            .starts_with("C:\\4forge\\runtimes\\php\\8.3.16;C:\\4forge\\runtimes\\node\\22.14.0"));
        assert!(new_path.ends_with("C:\\Windows\\System32;C:\\Windows"));
    }

    #[test]
    fn test_generate_batch_shim() {
        let temp = tempdir().expect("tempdir failed");
        let target = PathBuf::from("C:\\tools\\php\\php.exe");
        let shim = temp.path().join("php.cmd");

        let res = EnvironmentShim::generate_batch_shim(&target, &shim);
        assert!(res.is_ok());

        let content = std::fs::read_to_string(&shim).expect("read shim failed");
        assert!(content.contains("@echo off"));
        assert!(content.contains("php.exe"));
    }
}
