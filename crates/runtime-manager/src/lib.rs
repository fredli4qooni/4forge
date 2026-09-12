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

pub mod manager;
pub mod manifest;
pub mod node;
pub mod php;
pub mod shim;
pub mod store;

pub use manager::RuntimeManager;
pub use manifest::{RuntimeKind, RuntimeManifest, RuntimePackage};
pub use node::NodeRuntime;
pub use php::{PhpConfig, PhpRuntime};
pub use shim::EnvironmentShim;
pub use store::{InstalledRuntime, RuntimeStore};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Runtime '{0}' version '{1}' is not installed")]
    NotInstalled(RuntimeKind, String),
    #[error("No active version configured for runtime '{0}'")]
    NoActiveVersion(RuntimeKind),
    #[error("Checksum mismatch for package: expected {0}, got {1}")]
    ChecksumMismatch(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeVersion {
    pub kind: RuntimeKind,
    pub version: String,
    pub path: String,
    pub is_active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_kind_serialization() {
        let kind = RuntimeKind::Php;
        let json = serde_json::to_string(&kind).expect("failed to serialize");
        assert_eq!(json, "\"Php\"");
    }
}
