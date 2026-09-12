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
pub mod job_object;
pub mod log;
pub mod manager;
pub mod ports;
pub mod process;

pub use log::{LogHub, LogMessage, LogStream};
pub use manager::SupervisorManager;
pub use ports::{PortCheckResult, PortInspector};
pub use process::{ActiveProcess, ProcessConfig, ProcessError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Crashed,
    Restarting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub status: ServiceStatus,
    pub pid: Option<u32>,
    pub port: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_status_serialization() {
        let status = ServiceStatus::Running;
        let json = serde_json::to_string(&status).expect("serialization failed");
        assert_eq!(json, "\"Running\"");
    }
}
