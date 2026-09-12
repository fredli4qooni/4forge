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
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogStream {
    Stdout,
    Stderr,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    pub service: String,
    pub stream: LogStream,
    pub message: String,
    pub timestamp_millis: u64,
}

#[derive(Clone)]
pub struct LogHub {
    buffer: Arc<RwLock<VecDeque<LogMessage>>>,
    sender: broadcast::Sender<LogMessage>,
    capacity: usize,
}

impl LogHub {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(1024);
        Self {
            buffer: Arc::new(RwLock::new(VecDeque::with_capacity(capacity))),
            sender,
            capacity,
        }
    }

    pub async fn append(&self, service: &str, stream: LogStream, message: String) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let log = LogMessage {
            service: service.to_string(),
            stream,
            message,
            timestamp_millis: now,
        };

        {
            let mut buf = self.buffer.write().await;
            if buf.len() >= self.capacity {
                buf.pop_front();
            }
            buf.push_back(log.clone());
        }

        let _ = self.sender.send(log);
    }

    pub async fn get_recent(&self, service: Option<&str>, limit: usize) -> Vec<LogMessage> {
        let buf = self.buffer.read().await;
        buf.iter()
            .filter(|item| {
                if let Some(svc) = service {
                    item.service == svc
                } else {
                    true
                }
            })
            .rev()
            .take(limit)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<LogMessage> {
        self.sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_log_hub_append_and_retrieve() {
        let hub = LogHub::new(5);
        hub.append("caddy", LogStream::Stdout, "server started".to_string())
            .await;
        hub.append("caddy", LogStream::Stderr, "warning test".to_string())
            .await;
        hub.append(
            "mariadb",
            LogStream::Stdout,
            "ready for connections".to_string(),
        )
        .await;

        let all_logs = hub.get_recent(None, 10).await;
        assert_eq!(all_logs.len(), 3);

        let caddy_logs = hub.get_recent(Some("caddy"), 10).await;
        assert_eq!(caddy_logs.len(), 2);
    }
}
