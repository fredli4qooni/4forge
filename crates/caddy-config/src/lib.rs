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
pub mod client;
pub mod detector;
pub mod generator;
pub mod hosts;
pub mod trust;
pub mod vhost;

pub use client::{CaddyClient, CaddyClientError};
pub use detector::{DetectedProject, ProjectSignatureDetector};
pub use generator::CaddyConfigGenerator;
pub use hosts::WindowsHostsManager;
use serde::{Deserialize, Serialize};
pub use trust::CaddyTrustManager;
pub use vhost::{BackendType, VirtualHostConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualHost {
    pub domain: String,
    pub root_dir: String,
    pub backend_port: Option<u16>,
    pub enable_ssl: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_host_creation() {
        let vhost = VirtualHost {
            domain: "myproject.test".to_string(),
            root_dir: "C:\\projects\\myproject".to_string(),
            backend_port: Some(8000),
            enable_ssl: true,
        };
        assert_eq!(vhost.domain, "myproject.test");
        assert!(vhost.enable_ssl);
    }
}
