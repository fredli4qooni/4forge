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

//! Polyglot Runtime Version Manager for 4Forge.
//!
//! Manages downloading, extracting, verifying checksums, and switching runtime
//! versions (PHP, Node.js, Python, Ruby) per project.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeKind {
    Php,
    Node,
    Python,
    Ruby,
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
