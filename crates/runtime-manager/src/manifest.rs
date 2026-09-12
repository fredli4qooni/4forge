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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuntimeKind {
    Php,
    Node,
    Python,
    Ruby,
}

impl std::fmt::Display for RuntimeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Php => write!(f, "php"),
            Self::Node => write!(f, "node"),
            Self::Python => write!(f, "python"),
            Self::Ruby => write!(f, "ruby"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimePackage {
    pub kind: RuntimeKind,
    pub version: String,
    pub arch: String,
    pub download_url: String,
    pub sha256: String,
    pub is_thread_safe: Option<bool>,
}

pub struct RuntimeManifest;

impl RuntimeManifest {
    pub fn get_available_packages(kind: RuntimeKind) -> Vec<RuntimePackage> {
        match kind {
            RuntimeKind::Php => vec![
                RuntimePackage {
                    kind: RuntimeKind::Php,
                    version: "8.4.3".to_string(),
                    arch: "x64".to_string(),
                    download_url: "https://windows.php.net/downloads/releases/php-8.4.3-Win32-vs17-x64.zip".to_string(),
                    sha256: "913adfcb145a8e32906b3bc57b29388147d3d2c69d80d199dcfa2c23a7cf5e85".to_string(),
                    is_thread_safe: Some(false),
                },
                RuntimePackage {
                    kind: RuntimeKind::Php,
                    version: "8.3.16".to_string(),
                    arch: "x64".to_string(),
                    download_url: "https://windows.php.net/downloads/releases/php-8.3.16-Win32-vs16-x64.zip".to_string(),
                    sha256: "bcae52ea351b8cce6097561845bb02b9347d43ef19d6d37667cf7097564d6732".to_string(),
                    is_thread_safe: Some(false),
                },
                RuntimePackage {
                    kind: RuntimeKind::Php,
                    version: "8.2.27".to_string(),
                    arch: "x64".to_string(),
                    download_url: "https://windows.php.net/downloads/releases/php-8.2.27-Win32-vs16-x64.zip".to_string(),
                    sha256: "7ff04fdbfa5f694602f37e49221191ebc5a9fc2faeb5fa600a944fc139f40fa4".to_string(),
                    is_thread_safe: Some(false),
                },
            ],
            RuntimeKind::Node => vec![
                RuntimePackage {
                    kind: RuntimeKind::Node,
                    version: "22.14.0".to_string(),
                    arch: "x64".to_string(),
                    download_url: "https://nodejs.org/dist/v22.14.0/node-v22.14.0-win-x64.zip".to_string(),
                    sha256: "df8a5a415aefb7fe073f1ff23081e22709e99e28df95213d297d21b0ff41df12".to_string(),
                    is_thread_safe: None,
                },
                RuntimePackage {
                    kind: RuntimeKind::Node,
                    version: "20.18.3".to_string(),
                    arch: "x64".to_string(),
                    download_url: "https://nodejs.org/dist/v20.18.3/node-v20.18.3-win-x64.zip".to_string(),
                    sha256: "cfa53b499fb46299b8772bf2ebc3497d3e42ea2d6bafe8d8fb0299f01abef5bc".to_string(),
                    is_thread_safe: None,
                },
            ],
            RuntimeKind::Python => vec![
                RuntimePackage {
                    kind: RuntimeKind::Python,
                    version: "3.12.9".to_string(),
                    arch: "x64".to_string(),
                    download_url: "https://www.python.org/ftp/python/3.12.9/python-3.12.9-embed-amd64.zip".to_string(),
                    sha256: "b0f7454f7a77d488950f146fc77b7381cf3a66e04d49d95f08b3c65c6c061aa6".to_string(),
                    is_thread_safe: None,
                },
            ],
            RuntimeKind::Ruby => vec![
                RuntimePackage {
                    kind: RuntimeKind::Ruby,
                    version: "3.3.7".to_string(),
                    arch: "x64".to_string(),
                    download_url: "https://github.com/oneclick/rubyinstaller2/releases/download/RubyInstaller-3.3.7-1/rubyinstaller-3.3.7-1-x64.7z".to_string(),
                    sha256: "67e1a3bcfe895c10ad7bf23d7a8ce0b0942095f9c5e53e414c519808bfd32115".to_string(),
                    is_thread_safe: None,
                },
            ],
        }
    }

    pub fn find_package(kind: RuntimeKind, version: &str) -> Option<RuntimePackage> {
        Self::get_available_packages(kind)
            .into_iter()
            .find(|pkg| pkg.version == version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_available_packages() {
        let php_packages = RuntimeManifest::get_available_packages(RuntimeKind::Php);
        assert_eq!(php_packages.len(), 3);
        assert_eq!(php_packages[0].version, "8.4.3");

        let node_packages = RuntimeManifest::get_available_packages(RuntimeKind::Node);
        assert_eq!(node_packages.len(), 2);
        assert_eq!(node_packages[0].version, "22.14.0");
    }

    #[test]
    fn test_manifest_find_package() {
        let pkg = RuntimeManifest::find_package(RuntimeKind::Php, "8.3.16");
        assert!(pkg.is_some());
        assert_eq!(pkg.unwrap().arch, "x64");

        let missing = RuntimeManifest::find_package(RuntimeKind::Php, "5.6.0");
        assert!(missing.is_none());
    }
}
