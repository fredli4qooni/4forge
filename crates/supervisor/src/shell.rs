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
use std::process::Command;

pub struct NativeShell;

impl NativeShell {
    pub fn open_browser(url: &str) -> std::io::Result<()> {
        #[cfg(windows)]
        {
            Command::new("rundll32.exe")
                .args(["url.dll,FileProtocolHandler", url])
                .spawn()?;
            Ok(())
        }
        #[cfg(not(windows))]
        {
            Command::new("xdg-open").arg(url).spawn()?;
            Ok(())
        }
    }

    pub fn open_folder(path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        #[cfg(windows)]
        {
            Command::new("explorer.exe").arg(path).spawn()?;
            Ok(())
        }
        #[cfg(not(windows))]
        {
            Command::new("xdg-open").arg(path).spawn()?;
            Ok(())
        }
    }

    pub fn open_terminal(cwd: Option<&Path>, extra_paths: &[PathBuf]) -> std::io::Result<()> {
        let workdir = cwd
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("C:\\4forge\\projects"));

        if !workdir.exists() {
            let _ = std::fs::create_dir_all(&workdir);
        }

        let mut path_prefix = String::new();
        for p in extra_paths {
            if p.is_dir() {
                path_prefix.push_str(&format!("{};", p.display()));
            } else if let Some(parent) = p.parent() {
                path_prefix.push_str(&format!("{};", parent.display()));
            }
        }

        #[cfg(windows)]
        {
            let init_cmd = format!(
                "$env:Path = '{path_prefix}' + $env:Path; Write-Host '=========================================' -ForegroundColor Cyan; Write-Host '  4Forge Dev Shell Active' -ForegroundColor Green; Write-Host '  PHP, Node.js, Python, MariaDB in PATH' -ForegroundColor Yellow; Write-Host '=========================================' -ForegroundColor Cyan;"
            );

            let has_wt = Command::new("where.exe")
                .arg("wt.exe")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if has_wt {
                let mut cmd = Command::new("wt.exe");
                cmd.arg("-d")
                    .arg(&workdir)
                    .arg("powershell.exe")
                    .arg("-NoExit")
                    .arg("-Command")
                    .arg(&init_cmd);
                cmd.spawn()?;
            } else {
                let mut cmd = Command::new("powershell.exe");
                cmd.current_dir(&workdir)
                    .arg("-NoExit")
                    .arg("-Command")
                    .arg(&init_cmd);
                cmd.spawn()?;
            }
            Ok(())
        }
        #[cfg(not(windows))]
        {
            Command::new("bash").current_dir(&workdir).spawn()?;
            Ok(())
        }
    }

    pub fn find_database_client() -> Option<PathBuf> {
        let candidates = [
            "heidisql.exe",
            "dbeaver.exe",
            "tableplus.exe",
            "C:\\Program Files\\HeidiSQL\\heidisql.exe",
            "D:\\laragon\\bin\\heidisql\\heidisql.exe",
            "C:\\Program Files\\DBeaver\\dbeaver.exe",
        ];

        for c in candidates {
            let p = PathBuf::from(c);
            if p.is_file() {
                return Some(p);
            }
            if let Ok(paths) = std::env::var("PATH") {
                for dir in std::env::split_paths(&paths) {
                    let full = dir.join(c);
                    if full.is_file() {
                        return Some(full);
                    }
                }
            }
        }

        None
    }

    pub fn launch_database_client(host: &str, port: u16, user: &str) -> std::io::Result<bool> {
        if let Some(client) = Self::find_database_client() {
            let filename = client
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("")
                .to_lowercase();

            let mut cmd = Command::new(&client);
            if filename.contains("heidisql") {
                cmd.arg(format!("--host={}", host))
                    .arg(format!("--port={}", port))
                    .arg(format!("--user={}", user));
            }
            cmd.spawn()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn open_file(path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        if !path.exists() {
            std::fs::write(path, "")?;
        }
        #[cfg(windows)]
        {
            Command::new("notepad.exe").arg(path).spawn()?;
            Ok(())
        }
        #[cfg(not(windows))]
        {
            Command::new("xdg-open").arg(path).spawn()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_shell_helpers() {
        let tmp = std::env::temp_dir().join("4forge_test_shell");
        let res = NativeShell::open_folder(&tmp);
        let _ = std::fs::remove_dir_all(&tmp);
        assert!(res.is_ok());
    }

    #[test]
    fn test_native_shell_open_file() {
        let tmp = std::env::temp_dir().join("4forge_test_shell_config.ini");
        let res = NativeShell::open_file(&tmp);
        let _ = std::fs::remove_file(&tmp);
        assert!(res.is_ok());
    }
}
