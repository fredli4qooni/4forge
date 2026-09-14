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
use crate::log::{LogHub, LogStream};
use crate::ServiceStatus;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Process '{0}' is already running")]
    AlreadyRunning(String),
    #[error("Process '{0}' is not running")]
    NotRunning(String),
}

#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub current_dir: Option<PathBuf>,
    pub envs: HashMap<String, String>,
    pub port: Option<u16>,
    pub auto_restart: bool,
}

pub struct ActiveProcess {
    pub config: ProcessConfig,
    pub status: Arc<RwLock<ServiceStatus>>,
    pub pid: Arc<RwLock<Option<u32>>>,
    child: Arc<RwLock<Option<Child>>>,
    job_object: Arc<JobObject>,
    log_hub: LogHub,
}

impl ActiveProcess {
    pub fn new(config: ProcessConfig, job_object: Arc<JobObject>, log_hub: LogHub) -> Self {
        Self {
            config,
            status: Arc::new(RwLock::new(ServiceStatus::Stopped)),
            pid: Arc::new(RwLock::new(None)),
            child: Arc::new(RwLock::new(None)),
            job_object,
            log_hub,
        }
    }

    pub async fn start(&self) -> Result<(), ProcessError> {
        let mut status_lock = self.status.write().await;
        if *status_lock == ServiceStatus::Running || *status_lock == ServiceStatus::Starting {
            let is_actually_running = {
                let mut child_guard = self.child.write().await;
                if let Some(ref mut child) = *child_guard {
                    match child.try_wait() {
                        Ok(Some(_)) => {
                            *child_guard = None;
                            *self.pid.write().await = None;
                            false
                        }
                        Ok(None) => true,
                        Err(_) => false,
                    }
                } else {
                    false
                }
            };

            if is_actually_running {
                return Err(ProcessError::AlreadyRunning(self.config.name.clone()));
            }
        }

        *status_lock = ServiceStatus::Starting;
        self.log_hub
            .append(
                &self.config.name,
                LogStream::System,
                format!("Starting service: {}", self.config.name),
            )
            .await;

        let mut cmd = Command::new(&self.config.program);
        cmd.args(&self.config.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(ref dir) = self.config.current_dir {
            cmd.current_dir(dir);
        }

        for (k, v) in &self.config.envs {
            cmd.env(k, v);
        }

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                *status_lock = ServiceStatus::Crashed;
                self.log_hub
                    .append(
                        &self.config.name,
                        LogStream::System,
                        format!("Failed to spawn process: {}", e),
                    )
                    .await;
                return Err(ProcessError::Io(e));
            }
        };

        #[cfg(windows)]
        {
            if let Some(raw_handle) = child.raw_handle() {
                let res = self.job_object.assign_process(raw_handle as _);
                if let Err(e) = res {
                    self.log_hub
                        .append(
                            &self.config.name,
                            LogStream::System,
                            format!("Warning: failed to assign to job object: {}", e),
                        )
                        .await;
                }
            }
        }

        let child_pid = child.id();
        *self.pid.write().await = child_pid;
        *status_lock = ServiceStatus::Running;

        self.log_hub
            .append(
                &self.config.name,
                LogStream::System,
                format!("Service started with PID: {:?}", child_pid),
            )
            .await;

        if let Some(stdout) = child.stdout.take() {
            let log_hub = self.log_hub.clone();
            let name = self.config.name.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_hub.append(&name, LogStream::Stdout, line).await;
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let log_hub = self.log_hub.clone();
            let name = self.config.name.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    log_hub.append(&name, LogStream::Stderr, line).await;
                }
            });
        }

        *self.child.write().await = Some(child);
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), ProcessError> {
        let mut child_guard = self.child.write().await;
        if let Some(mut child) = child_guard.take() {
            let mut status_lock = self.status.write().await;
            *status_lock = ServiceStatus::Stopped;

            let _ = child.kill().await;
            let _ = child.wait().await;

            *self.pid.write().await = None;

            self.log_hub
                .append(
                    &self.config.name,
                    LogStream::System,
                    format!("Service '{}' stopped successfully", self.config.name),
                )
                .await;
            Ok(())
        } else {
            let mut status_lock = self.status.write().await;
            *status_lock = ServiceStatus::Stopped;
            *self.pid.write().await = None;
            Ok(())
        }
    }

    pub async fn is_alive(&self) -> bool {
        let mut child_guard = self.child.write().await;
        if let Some(ref mut child) = *child_guard {
            match child.try_wait() {
                Ok(None) => true,
                _ => {
                    *child_guard = None;
                    *self.pid.write().await = None;
                    *self.status.write().await = ServiceStatus::Stopped;
                    false
                }
            }
        } else {
            *self.status.write().await = ServiceStatus::Stopped;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_lifecycle() {
        let job = Arc::new(JobObject::create().expect("failed to create job object"));
        let hub = LogHub::new(10);

        let config = ProcessConfig {
            name: "test_cmd".to_string(),
            program: PathBuf::from("cmd.exe"),
            args: vec![
                "/c".to_string(),
                "echo".to_string(),
                "hello 4forge".to_string(),
            ],
            current_dir: None,
            envs: HashMap::new(),
            port: None,
            auto_restart: false,
        };

        let proc = ActiveProcess::new(config, job, hub.clone());
        let res = proc.start().await;
        assert!(res.is_ok());

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let logs = hub.get_recent(Some("test_cmd"), 10).await;
        assert!(logs.iter().any(|l| l.message.contains("hello 4forge")));
    }
}
