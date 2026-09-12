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
pub struct PythonRuntime {
    version: String,
    base_dir: PathBuf,
}

impl PythonRuntime {
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

    pub fn python_bin(&self) -> PathBuf {
        self.base_dir.join("python.exe")
    }

    pub fn pip_bin(&self) -> PathBuf {
        let scripts_pip = self.base_dir.join("Scripts").join("pip.exe");
        if scripts_pip.is_file() {
            scripts_pip
        } else {
            self.base_dir.join("pip.exe")
        }
    }

    pub fn build_venv_command(&self, project_dir: &Path) -> (PathBuf, Vec<String>) {
        (
            self.python_bin(),
            vec![
                "-m".to_string(),
                "venv".to_string(),
                project_dir.join(".venv").display().to_string(),
            ],
        )
    }

    pub fn build_asgi_command(
        &self,
        app_module: &str,
        port: u16,
        use_venv: Option<&Path>,
    ) -> (PathBuf, Vec<String>) {
        let bin = if let Some(venv) = use_venv {
            venv.join("Scripts").join("python.exe")
        } else {
            self.python_bin()
        };

        (
            bin,
            vec![
                "-m".to_string(),
                "uvicorn".to_string(),
                app_module.to_string(),
                "--host".to_string(),
                "127.0.0.1".to_string(),
                "--port".to_string(),
                port.to_string(),
            ],
        )
    }

    pub fn get_env_vars(&self, project_dir: Option<&Path>) -> HashMap<String, String> {
        let mut envs = HashMap::new();
        let mut path_entries = Vec::new();

        if let Some(p) = project_dir {
            let venv_dir = p.join(".venv");
            let venv_scripts = venv_dir.join("Scripts");
            path_entries.push(venv_scripts.to_string_lossy().to_string());
            envs.insert(
                "VIRTUAL_ENV".to_string(),
                venv_dir.to_string_lossy().to_string(),
            );
        }

        path_entries.push(self.base_dir.to_string_lossy().to_string());
        path_entries.push(self.base_dir.join("Scripts").to_string_lossy().to_string());

        envs.insert("PATH".to_string(), path_entries.join(";"));
        envs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_runtime_commands() {
        let base = PathBuf::from("C:\\4forge\\runtimes\\python\\3.12.9");
        let py = PythonRuntime::new("3.12.9", base);

        assert_eq!(py.version(), "3.12.9");
        assert!(py.python_bin().ends_with("python.exe"));

        let project = PathBuf::from("C:\\projects\\fastapi-app");
        let (cmd, args) = py.build_venv_command(&project);
        assert!(cmd.ends_with("python.exe"));
        assert!(args.contains(&"-m".to_string()));
        assert!(args.contains(&"venv".to_string()));

        let (asgi_cmd, asgi_args) = py.build_asgi_command("main:app", 8000, None);
        assert!(asgi_cmd.ends_with("python.exe"));
        assert!(asgi_args.contains(&"uvicorn".to_string()));
        assert!(asgi_args.contains(&"8000".to_string()));
    }
}
