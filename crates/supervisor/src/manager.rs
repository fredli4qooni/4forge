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

use crate::job_object::windows::JobObject;
use crate::log::{LogHub, LogMessage};
use crate::process::{ActiveProcess, ProcessConfig, ProcessError};
use crate::ServiceInfo;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

#[derive(Clone)]
pub struct SupervisorManager {
    services: Arc<RwLock<HashMap<String, Arc<ActiveProcess>>>>,
    job_object: Arc<JobObject>,
    log_hub: LogHub,
}

impl SupervisorManager {
    pub fn new() -> Result<Self, std::io::Error> {
        let job_object = Arc::new(JobObject::create()?);
        let log_hub = LogHub::new(1000);
        Ok(Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            job_object,
            log_hub,
        })
    }

    pub fn new_with_services(services: Vec<ProcessConfig>) -> Result<Self, std::io::Error> {
        let job_object = Arc::new(JobObject::create()?);
        let log_hub = LogHub::new(1000);
        let mut map = HashMap::new();
        for cfg in services {
            let proc = Arc::new(ActiveProcess::new(
                cfg.clone(),
                job_object.clone(),
                log_hub.clone(),
            ));
            map.insert(cfg.name.clone(), proc);
        }
        Ok(Self {
            services: Arc::new(RwLock::new(map)),
            job_object,
            log_hub,
        })
    }

    pub async fn register_service(&self, config: ProcessConfig) {
        let mut map = self.services.write().await;
        let proc = Arc::new(ActiveProcess::new(
            config.clone(),
            self.job_object.clone(),
            self.log_hub.clone(),
        ));
        map.insert(config.name, proc);
    }

    pub async fn start_service(&self, name: &str) -> Result<(), ProcessError> {
        let map = self.services.read().await;
        if let Some(proc) = map.get(name) {
            proc.start().await
        } else {
            Err(ProcessError::NotRunning(name.to_string()))
        }
    }

    pub async fn stop_service(&self, name: &str) -> Result<(), ProcessError> {
        let map = self.services.read().await;
        if let Some(proc) = map.get(name) {
            proc.stop().await
        } else {
            Ok(())
        }
    }

    pub async fn restart_service(&self, name: &str) -> Result<(), ProcessError> {
        self.stop_service(name).await?;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        self.start_service(name).await
    }

    pub async fn stop_all(&self) {
        let map = self.services.read().await;
        for proc in map.values() {
            let _ = proc.stop().await;
        }
    }

    pub async fn update_service_port(&self, name: &str, new_port: u16) -> bool {
        let mut map = self.services.write().await;
        if let Some(proc) = map.get_mut(name) {
            let mut cfg = proc.config.clone();
            cfg.port = Some(new_port);
            *proc = Arc::new(ActiveProcess::new(
                cfg,
                self.job_object.clone(),
                self.log_hub.clone(),
            ));
            true
        } else {
            false
        }
    }

    pub async fn get_service_info(&self, name: &str) -> Option<ServiceInfo> {
        let map = self.services.read().await;
        if let Some(proc) = map.get(name) {
            let status = *proc.status.read().await;
            let pid = *proc.pid.read().await;
            Some(ServiceInfo {
                name: proc.config.name.clone(),
                status,
                pid,
                port: proc.config.port,
            })
        } else {
            None
        }
    }

    pub async fn list_services(&self) -> Vec<ServiceInfo> {
        let map = self.services.read().await;
        let mut list = Vec::new();
        for proc in map.values() {
            let status = *proc.status.read().await;
            let pid = *proc.pid.read().await;
            list.push(ServiceInfo {
                name: proc.config.name.clone(),
                status,
                pid,
                port: proc.config.port,
            });
        }
        list
    }

    pub async fn get_logs(&self, service: Option<&str>, limit: usize) -> Vec<LogMessage> {
        self.log_hub.get_recent(service, limit).await
    }

    pub fn subscribe_logs(&self) -> broadcast::Receiver<LogMessage> {
        self.log_hub.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_supervisor_manager_workflow() {
        let manager = SupervisorManager::new().expect("failed to init manager");

        let cfg = ProcessConfig {
            name: "test_echo".to_string(),
            program: PathBuf::from("cmd.exe"),
            args: vec![
                "/c".to_string(),
                "echo".to_string(),
                "supervisor test".to_string(),
            ],
            current_dir: None,
            envs: HashMap::new(),
            port: Some(8080),
            auto_restart: false,
        };

        manager.register_service(cfg).await;
        let info = manager.get_service_info("test_echo").await;
        assert!(info.is_some());
        assert_eq!(info.unwrap().port, Some(8080));

        let res = manager.start_service("test_echo").await;
        assert!(res.is_ok());

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let logs = manager.get_logs(Some("test_echo"), 5).await;
        assert!(logs.iter().any(|l| l.message.contains("supervisor test")));
    }
}
