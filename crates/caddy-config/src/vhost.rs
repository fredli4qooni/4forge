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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendType {
    Static,
    PhpFastCgi { fastcgi_addr: String },
    ReverseProxy { upstream_addr: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualHostConfig {
    pub domain: String,
    pub root_dir: String,
    pub backend: BackendType,
    pub enable_ssl: bool,
}

impl VirtualHostConfig {
    pub fn new_php(domain: &str, root_dir: &str, fastcgi_addr: &str) -> Self {
        Self {
            domain: domain.to_string(),
            root_dir: root_dir.to_string(),
            backend: BackendType::PhpFastCgi {
                fastcgi_addr: fastcgi_addr.to_string(),
            },
            enable_ssl: true,
        }
    }

    pub fn new_proxy(domain: &str, root_dir: &str, upstream_addr: &str) -> Self {
        Self {
            domain: domain.to_string(),
            root_dir: root_dir.to_string(),
            backend: BackendType::ReverseProxy {
                upstream_addr: upstream_addr.to_string(),
            },
            enable_ssl: true,
        }
    }

    pub fn new_static(domain: &str, root_dir: &str) -> Self {
        Self {
            domain: domain.to_string(),
            root_dir: root_dir.to_string(),
            backend: BackendType::Static,
            enable_ssl: true,
        }
    }
}
