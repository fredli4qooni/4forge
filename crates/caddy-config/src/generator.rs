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
use serde_json::{json, Value};

pub struct CaddyConfigGenerator;

impl CaddyConfigGenerator {
    pub fn generate(hosts: &[VirtualHostConfig]) -> Value {
        let mut routes = Vec::new();
        let mut tls_subjects = Vec::new();

        for host in hosts {
            if host.enable_ssl {
                tls_subjects.push(host.domain.clone());
            }

            let mut handlers = Vec::new();

            match &host.backend {
                BackendType::Static => {
                    handlers.push(json!({
                        "handler": "file_server",
                        "root": host.root_dir
                    }));
                }
                BackendType::PhpFastCgi { fastcgi_addr } => {
                    handlers.push(json!({
                        "handler": "vars",
                        "root": host.root_dir
                    }));
                    handlers.push(json!({
                        "handler": "reverse_proxy",
                        "transport": {
                            "protocol": "fastcgi",
                            "split_path": [".php"]
                        },
                        "upstreams": [{ "dial": fastcgi_addr }]
                    }));
                    handlers.push(json!({
                        "handler": "file_server"
                    }));
                }
                BackendType::ReverseProxy { upstream_addr } => {
                    handlers.push(json!({
                        "handler": "reverse_proxy",
                        "upstreams": [{ "dial": upstream_addr }]
                    }));
                }
            }

            routes.push(json!({
                "match": [{
                    "host": [host.domain]
                }],
                "handle": handlers,
                "terminal": true
            }));
        }

        json!({
            "admin": {
                "listen": "localhost:2019"
            },
            "apps": {
                "http": {
                    "servers": {
                        "forge": {
                            "listen": [":80", ":443"],
                            "routes": routes
                        }
                    }
                },
                "tls": {
                    "automation": {
                        "policies": [{
                            "subjects": tls_subjects,
                            "issuer": {
                                "module": "internal"
                            }
                        }]
                    }
                }
            }
        })
    }

    pub fn generate_caddyfile(hosts: &[VirtualHostConfig]) -> String {
        let mut out = String::new();
        out.push_str("{\n    admin 127.0.0.1:2019\n}\n\n");
        out.push_str(
            "http://localhost {\n    respond \"4Forge Local Web Server Running\" 200\n}\n\n",
        );

        for host in hosts {
            let normalized_root = host.root_dir.replace('\\', "/");
            out.push_str(&format!("http://{} {{\n", host.domain));
            match &host.backend {
                BackendType::Static => {
                    out.push_str(&format!("    root * \"{}\"\n", normalized_root));
                    out.push_str("    file_server\n");
                }
                BackendType::PhpFastCgi { fastcgi_addr } => {
                    out.push_str(&format!("    root * \"{}\"\n", normalized_root));
                    out.push_str(&format!("    php_fastcgi {}\n", fastcgi_addr));
                    out.push_str("    file_server\n");
                }
                BackendType::ReverseProxy { upstream_addr } => {
                    out.push_str(&format!("    reverse_proxy {}\n", upstream_addr));
                }
            }
            out.push_str("}\n\n");
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caddy_config_generation() {
        let hosts = vec![
            VirtualHostConfig::new_php("laravel.test", "C:\\projects\\laravel", "127.0.0.1:9000"),
            VirtualHostConfig::new_proxy("node.test", "C:\\projects\\node", "127.0.0.1:3000"),
        ];

        let config = CaddyConfigGenerator::generate(&hosts);
        assert_eq!(config["admin"]["listen"], "localhost:2019");

        let routes = config["apps"]["http"]["servers"]["forge"]["routes"]
            .as_array()
            .expect("routes should be array");
        assert_eq!(routes.len(), 2);

        let subjects = config["apps"]["tls"]["automation"]["policies"][0]["subjects"]
            .as_array()
            .expect("subjects should be array");
        assert_eq!(subjects.len(), 2);
    }

    #[test]
    fn test_generate_caddyfile() {
        let hosts = vec![
            VirtualHostConfig::new_php(
                "project.test",
                "C:\\projects\\project\\public",
                "127.0.0.1:9000",
            ),
            VirtualHostConfig::new_static("static.test", "C:\\projects\\static"),
        ];

        let caddyfile = CaddyConfigGenerator::generate_caddyfile(&hosts);
        assert!(caddyfile.contains("admin 127.0.0.1:2019"));
        assert!(caddyfile.contains("http://project.test {"));
        assert!(caddyfile.contains("php_fastcgi 127.0.0.1:9000"));
        assert!(caddyfile.contains("http://static.test {"));
        assert!(caddyfile.contains("file_server"));
    }
}
