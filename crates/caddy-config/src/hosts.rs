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

use std::path::PathBuf;
use std::process::Command;

pub struct WindowsHostsManager;

impl WindowsHostsManager {
    pub const START_MARKER: &'static str = "# === 4FORGE VIRTUAL HOSTS START ===";
    pub const END_MARKER: &'static str = "# === 4FORGE VIRTUAL HOSTS END ===";

    pub fn hosts_path() -> PathBuf {
        #[cfg(windows)]
        {
            if let Ok(sysroot) = std::env::var("SystemRoot") {
                PathBuf::from(sysroot)
                    .join("System32")
                    .join("drivers")
                    .join("etc")
                    .join("hosts")
            } else {
                PathBuf::from("C:\\Windows\\System32\\drivers\\etc\\hosts")
            }
        }
        #[cfg(not(windows))]
        {
            PathBuf::from("/etc/hosts")
        }
    }

    pub fn check_missing_domains(domains: &[String]) -> Vec<String> {
        let path = Self::hosts_path();
        if !path.exists() {
            return domains.to_vec();
        }

        let content = std::fs::read_to_string(&path).unwrap_or_default();
        Self::find_missing_in_content(&content, domains)
    }

    pub fn find_missing_in_content(content: &str, domains: &[String]) -> Vec<String> {
        let mut missing = Vec::new();
        for domain in domains {
            let d = domain.trim().to_lowercase();
            if d.is_empty() || d == "localhost" || d == "127.0.0.1" {
                continue;
            }

            let found = content.lines().any(|line| {
                let clean = line.trim();
                if clean.starts_with('#') {
                    return false;
                }
                let parts: Vec<&str> = clean.split_whitespace().collect();
                if parts.len() >= 2 {
                    let ip = parts[0];
                    let host = parts[1].to_lowercase();
                    (ip == "127.0.0.1" || ip == "::1") && host == d
                } else {
                    false
                }
            });

            if !found {
                missing.push(domain.clone());
            }
        }
        missing
    }

    pub fn build_synced_content(existing_content: &str, all_domains: &[String]) -> String {
        let lines: Vec<&str> = existing_content.lines().collect();
        let mut start_idx = None;
        let mut end_idx = None;

        for (idx, line) in lines.iter().enumerate() {
            let t = line.trim();
            if t == Self::START_MARKER {
                start_idx = Some(idx);
            } else if t == Self::END_MARKER {
                end_idx = Some(idx);
            }
        }

        let mut block_lines = Vec::new();
        block_lines.push(Self::START_MARKER.to_string());
        for domain in all_domains {
            let d = domain.trim().to_lowercase();
            if !d.is_empty() && d != "localhost" && d != "127.0.0.1" {
                block_lines.push(format!("127.0.0.1\t{}", d));
                block_lines.push(format!("::1\t\t{}", d));
            }
        }
        block_lines.push(Self::END_MARKER.to_string());
        let block_string = block_lines.join("\r\n");

        if let (Some(s), Some(e)) = (start_idx, end_idx) {
            if s <= e {
                let mut prefix = lines[..s].join("\r\n");
                let suffix = lines[e + 1..].join("\r\n");
                if !prefix.is_empty() && !prefix.ends_with("\r\n") {
                    prefix.push_str("\r\n");
                }
                let mut out = format!("{}{}", prefix, block_string);
                if !suffix.is_empty() {
                    out.push_str("\r\n");
                    out.push_str(&suffix);
                }
                return out;
            }
        }

        let mut out = existing_content.trim_end().to_string();
        if !out.is_empty() {
            out.push_str("\r\n\r\n");
        }
        out.push_str(&block_string);
        out.push_str("\r\n");
        out
    }

    pub fn sync_domains(domains: &[String]) -> std::io::Result<bool> {
        let path = Self::hosts_path();
        let current_content = if path.exists() {
            std::fs::read_to_string(&path).unwrap_or_default()
        } else {
            String::new()
        };

        let new_content = Self::build_synced_content(&current_content, domains);

        if std::fs::write(&path, &new_content).is_ok() {
            return Ok(true);
        }

        #[cfg(windows)]
        {
            let temp_hosts = std::env::temp_dir().join("4forge_hosts.tmp");
            std::fs::write(&temp_hosts, &new_content)?;

            let target = path.to_string_lossy().to_string();
            let src = temp_hosts.to_string_lossy().to_string();
            let elevated_script = format!(
                "Start-Process powershell -Verb RunAs -Wait -ArgumentList '-NoProfile -Command Copy-Item -Force \"{}\" \"{}\" ; Remove-Item -Force \"{}\"'",
                src, target, src
            );

            let status = Command::new("powershell.exe")
                .arg("-NoProfile")
                .arg("-Command")
                .arg(&elevated_script)
                .status()?;

            Ok(status.success())
        }
        #[cfg(not(windows))]
        {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Permission denied modifying hosts file",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_domains_detection() {
        let sample = "127.0.0.1 localhost\n127.0.0.1 laravel-app.test\n";
        let domains = vec![
            "laravel-app.test".to_string(),
            "missing-app.test".to_string(),
        ];
        let missing = WindowsHostsManager::find_missing_in_content(sample, &domains);
        assert_eq!(missing, vec!["missing-app.test".to_string()]);
    }

    #[test]
    fn test_build_synced_content_insertion_and_replacement() {
        let sample = "127.0.0.1 localhost\r\n";
        let domains = vec!["site-a.test".to_string(), "site-b.test".to_string()];
        let out = WindowsHostsManager::build_synced_content(sample, &domains);
        assert!(out.contains(WindowsHostsManager::START_MARKER));
        assert!(out.contains("127.0.0.1\tsite-a.test"));
        assert!(out.contains("::1\t\tsite-b.test"));
        assert!(out.contains(WindowsHostsManager::END_MARKER));

        let updated_domains = vec!["site-c.test".to_string()];
        let out2 = WindowsHostsManager::build_synced_content(&out, &updated_domains);
        assert!(!out2.contains("site-a.test"));
        assert!(out2.contains("site-c.test"));
    }
}
