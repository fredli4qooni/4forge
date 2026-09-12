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

use serde_json::Value;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum CaddyClientError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Caddy returned HTTP {0}: {1}")]
    ApiError(u16, String),
    #[error("Failed to parse JSON response: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct CaddyClient {
    admin_url: String,
    http: reqwest::Client,
}

impl Default for CaddyClient {
    fn default() -> Self {
        Self::new("http://localhost:2019")
    }
}

impl CaddyClient {
    pub fn new(admin_url: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            admin_url: admin_url.trim_end_matches('/').to_string(),
            http,
        }
    }

    pub async fn is_alive(&self) -> bool {
        let url = format!("{}/config/", self.admin_url);
        self.http
            .get(&url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub async fn load_config(&self, config: &Value) -> Result<(), CaddyClientError> {
        let url = format!("{}/load", self.admin_url);
        let resp = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .json(config)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(CaddyClientError::ApiError(status, body));
        }

        Ok(())
    }

    pub async fn get_config(&self) -> Result<Value, CaddyClientError> {
        let url = format!("{}/config/", self.admin_url);
        let resp = self.http.get(&url).send().await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(CaddyClientError::ApiError(status, body));
        }

        let json = resp.json::<Value>().await?;
        Ok(json)
    }

    pub async fn stop_server(&self) -> Result<(), CaddyClientError> {
        let url = format!("{}/stop", self.admin_url);
        let resp = self.http.post(&url).send().await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(CaddyClientError::ApiError(status, body));
        }

        Ok(())
    }
}
