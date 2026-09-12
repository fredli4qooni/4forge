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

use std::path::Path;
use std::process::Command;

pub struct CaddyTrustManager;

impl CaddyTrustManager {
    pub fn install_trust(caddy_binary: &Path) -> Result<(), std::io::Error> {
        let output = Command::new(caddy_binary).arg("trust").output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(std::io::Error::other(format!(
                "Failed to install Caddy Root CA: {}",
                stderr
            )));
        }
        Ok(())
    }

    pub fn uninstall_trust(caddy_binary: &Path) -> Result<(), std::io::Error> {
        let output = Command::new(caddy_binary).arg("untrust").output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(std::io::Error::other(format!(
                "Failed to uninstall Caddy Root CA: {}",
                stderr
            )));
        }
        Ok(())
    }
}
