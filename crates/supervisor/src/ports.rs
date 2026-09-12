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
use std::net::TcpListener;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortCheckResult {
    pub port: u16,
    pub is_available: bool,
    pub service_name: String,
    pub alternative_port: Option<u16>,
}

pub struct PortInspector;

impl PortInspector {
    pub fn is_port_available(port: u16) -> bool {
        let addr_v4 = format!("127.0.0.1:{}", port);
        match TcpListener::bind(&addr_v4) {
            Ok(listener) => {
                drop(listener);
                true
            }
            Err(_) => false,
        }
    }

    pub fn suggest_alternative(port: u16) -> u16 {
        let initial_candidate = match port {
            80 => 8080,
            443 => 8443,
            3306 => 3307,
            9000 => 9001,
            2019 => 2020,
            p => p + 1,
        };

        let mut candidate = initial_candidate;
        for _ in 0..100 {
            if Self::is_port_available(candidate) {
                return candidate;
            }
            candidate += 1;
        }

        candidate
    }

    pub fn check_port(port: u16, service_name: &str) -> PortCheckResult {
        let available = Self::is_port_available(port);
        let alt = if available {
            None
        } else {
            Some(Self::suggest_alternative(port))
        };

        PortCheckResult {
            port,
            is_available: available,
            service_name: service_name.to_string(),
            alternative_port: alt,
        }
    }

    pub fn check_standard_ports() -> Vec<PortCheckResult> {
        let targets = [
            (80, "Caddy Web Server (HTTP)"),
            (443, "Caddy Web Server (HTTPS)"),
            (2019, "Caddy Admin API"),
            (3306, "MariaDB Database"),
            (9000, "PHP-FPM FastCGI"),
        ];

        targets
            .into_iter()
            .map(|(p, name)| Self::check_port(p, name))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_inspector_check() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind ephemeral port");
        let bound_port = listener.local_addr().unwrap().port();

        assert!(!PortInspector::is_port_available(bound_port));

        let alt = PortInspector::suggest_alternative(bound_port);
        assert_ne!(alt, bound_port);

        drop(listener);
        assert!(PortInspector::is_port_available(bound_port));
    }

    #[test]
    fn test_check_standard_ports() {
        let results = PortInspector::check_standard_ports();
        assert_eq!(results.len(), 5);
        assert!(results.iter().any(|r| r.port == 3306));
    }
}
