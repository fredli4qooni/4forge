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

use crate::vhost::{BackendType, VirtualHostConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetectedProject {
    pub name: String,
    pub path: String,
    pub domain: String,
    pub framework: String,
    pub runtime: String,
    pub web_root: String,
    pub backend_type: String,
    pub target: String,
}

impl DetectedProject {
    pub fn to_virtual_host_config(&self) -> VirtualHostConfig {
        let backend = match self.backend_type.as_str() {
            "fastcgi" => BackendType::PhpFastCgi {
                fastcgi_addr: self.target.clone(),
            },
            "proxy" => BackendType::ReverseProxy {
                upstream_addr: self.target.clone(),
            },
            _ => BackendType::Static,
        };

        VirtualHostConfig {
            domain: self.domain.clone(),
            root_dir: self.web_root.clone(),
            backend,
            enable_ssl: true,
        }
    }
}

pub struct ProjectSignatureDetector;

impl ProjectSignatureDetector {
    pub fn detect(dir: &Path, domain_suffix: Option<&str>) -> DetectedProject {
        let folder_name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string();

        let clean_slug = folder_name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>();

        let suffix = domain_suffix.unwrap_or("test");
        let domain = format!("{}.{}", clean_slug, suffix);
        let path_str = dir.to_string_lossy().to_string();

        if dir.join("artisan").is_file() {
            let public_dir = dir.join("public");
            let web_root = if public_dir.is_dir() {
                public_dir.to_string_lossy().to_string()
            } else {
                path_str.clone()
            };
            return DetectedProject {
                name: folder_name,
                path: path_str,
                domain,
                framework: "Laravel".to_string(),
                runtime: "PHP 8.3".to_string(),
                web_root,
                backend_type: "fastcgi".to_string(),
                target: "127.0.0.1:9000".to_string(),
            };
        }

        if dir.join("wp-config.php").is_file()
            || dir.join("wp-load.php").is_file()
            || dir.join("wp-content").is_dir()
        {
            return DetectedProject {
                name: folder_name,
                path: path_str.clone(),
                domain,
                framework: "WordPress".to_string(),
                runtime: "PHP 8.3".to_string(),
                web_root: path_str,
                backend_type: "fastcgi".to_string(),
                target: "127.0.0.1:9000".to_string(),
            };
        }

        if dir.join("bin").join("console").is_file() {
            let public_dir = dir.join("public");
            let web_root = if public_dir.is_dir() {
                public_dir.to_string_lossy().to_string()
            } else {
                path_str.clone()
            };
            return DetectedProject {
                name: folder_name,
                path: path_str,
                domain,
                framework: "Symfony".to_string(),
                runtime: "PHP 8.3".to_string(),
                web_root,
                backend_type: "fastcgi".to_string(),
                target: "127.0.0.1:9000".to_string(),
            };
        }

        if dir.join("package.json").is_file() {
            let is_next = dir.join("next.config.js").is_file()
                || dir.join("next.config.mjs").is_file()
                || dir.join("next.config.ts").is_file();
            let is_vite = dir.join("vite.config.js").is_file()
                || dir.join("vite.config.ts").is_file()
                || dir.join("vite.config.mjs").is_file();

            let (framework, target) = if is_next {
                ("Next.js", "127.0.0.1:3000")
            } else if is_vite {
                ("Vite", "127.0.0.1:5173")
            } else {
                ("Node.js", "127.0.0.1:3000")
            };

            return DetectedProject {
                name: folder_name,
                path: path_str.clone(),
                domain,
                framework: framework.to_string(),
                runtime: "Node 22".to_string(),
                web_root: path_str,
                backend_type: "proxy".to_string(),
                target: target.to_string(),
            };
        }

        if dir.join("manage.py").is_file() {
            return DetectedProject {
                name: folder_name,
                path: path_str.clone(),
                domain,
                framework: "Django".to_string(),
                runtime: "Python 3.12".to_string(),
                web_root: path_str,
                backend_type: "proxy".to_string(),
                target: "127.0.0.1:8000".to_string(),
            };
        }

        if dir.join("index.php").is_file() || dir.join("composer.json").is_file() {
            let public_dir = dir.join("public");
            let web_root = if public_dir.is_dir() {
                public_dir.to_string_lossy().to_string()
            } else {
                path_str.clone()
            };
            return DetectedProject {
                name: folder_name,
                path: path_str,
                domain,
                framework: "PHP Web App".to_string(),
                runtime: "PHP 8.3".to_string(),
                web_root,
                backend_type: "fastcgi".to_string(),
                target: "127.0.0.1:9000".to_string(),
            };
        }

        if dir.join("index.html").is_file() {
            return DetectedProject {
                name: folder_name,
                path: path_str.clone(),
                domain,
                framework: "Static HTML".to_string(),
                runtime: "Static".to_string(),
                web_root: path_str,
                backend_type: "static".to_string(),
                target: "-".to_string(),
            };
        }

        DetectedProject {
            name: folder_name,
            path: path_str.clone(),
            domain,
            framework: "Generic Web".to_string(),
            runtime: "PHP 8.3".to_string(),
            web_root: path_str,
            backend_type: "fastcgi".to_string(),
            target: "127.0.0.1:9000".to_string(),
        }
    }

    pub fn scan_directory(parent: &Path, domain_suffix: Option<&str>) -> Vec<DetectedProject> {
        let mut results = Vec::new();
        if !parent.is_dir() {
            return results;
        }

        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let name = p
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                    if !name.starts_with('.') && !name.starts_with('_') {
                        results.push(Self::detect(&p, domain_suffix));
                    }
                }
            }
        }

        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_laravel() {
        let dir = std::env::temp_dir().join("4forge_test_laravel_proj");
        let _ = std::fs::create_dir_all(dir.join("public"));
        let _ = std::fs::write(dir.join("artisan"), "");

        let detected = ProjectSignatureDetector::detect(&dir, Some("test"));
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(detected.framework, "Laravel");
        assert_eq!(detected.backend_type, "fastcgi");
        assert!(detected.domain.ends_with(".test"));
        assert!(detected.web_root.ends_with("public"));
    }

    #[test]
    fn test_detect_node_next() {
        let dir = std::env::temp_dir().join("4forge_test_next_proj");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(dir.join("package.json"), "{}");
        let _ = std::fs::write(dir.join("next.config.js"), "");

        let detected = ProjectSignatureDetector::detect(&dir, None);
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(detected.framework, "Next.js");
        assert_eq!(detected.backend_type, "proxy");
        assert_eq!(detected.target, "127.0.0.1:3000");
    }

    #[test]
    fn test_scan_workspace() {
        let parent = std::env::temp_dir().join("4forge_test_workspace_root");
        let sub1 = parent.join("blog");
        let sub2 = parent.join("frontend");
        let _ = std::fs::create_dir_all(&sub1);
        let _ = std::fs::create_dir_all(&sub2);
        let _ = std::fs::write(sub1.join("wp-config.php"), "");
        let _ = std::fs::write(sub2.join("index.html"), "");

        let list = ProjectSignatureDetector::scan_directory(&parent, Some("local"));
        let _ = std::fs::remove_dir_all(&parent);

        assert_eq!(list.len(), 2);
        assert!(list
            .iter()
            .any(|p| p.framework == "WordPress" && p.domain == "blog.local"));
        assert!(list
            .iter()
            .any(|p| p.framework == "Static HTML" && p.domain == "frontend.local"));
    }
}
