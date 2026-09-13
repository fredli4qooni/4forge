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

use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct PtySession {
    pub id: String,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
}

#[derive(Clone, Default)]
pub struct PtySessionManager {
    sessions: Arc<Mutex<HashMap<String, PtySession>>>,
}

impl PtySessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn spawn_session<F>(
        &self,
        cwd: Option<&Path>,
        extra_paths: &[PathBuf],
        cols: u16,
        rows: u16,
        on_output: F,
    ) -> Result<String, String>
    where
        F: Fn(Vec<u8>) + Send + 'static,
    {
        let session_id = format!(
            "pty-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0)
        );

        let pty_system = native_pty_system();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: rows.max(10),
                cols: cols.max(20),
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| format!("Failed to open PTY: {e}"))?;

        let workdir = cwd
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("C:\\4forge\\projects"));

        if !workdir.exists() {
            let _ = std::fs::create_dir_all(&workdir);
        }

        let mut path_prefix = String::new();
        for p in extra_paths {
            if p.is_dir() {
                path_prefix.push_str(&format!("{};", p.display()));
            } else if let Some(parent) = p.parent() {
                path_prefix.push_str(&format!("{};", parent.display()));
            }
        }

        #[cfg(windows)]
        let cmd = {
            let mut c = CommandBuilder::new("powershell.exe");
            c.cwd(&workdir);
            c.arg("-NoLogo");
            if let Ok(current_path) = std::env::var("PATH") {
                c.env("PATH", format!("{path_prefix}{current_path}"));
            } else {
                c.env("PATH", &path_prefix);
            }
            c.env("TERM", "xterm-256color");
            c
        };

        #[cfg(not(windows))]
        let mut cmd = {
            let mut c = CommandBuilder::new("bash");
            c.cwd(&workdir);
            if let Ok(current_path) = std::env::var("PATH") {
                c.env("PATH", format!("{path_prefix}:{current_path}"));
            }
            c.env("TERM", "xterm-256color");
            c
        };

        let _child = pty_pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| format!("Failed to spawn shell in PTY: {e}"))?;

        drop(pty_pair.slave);

        let master = pty_pair.master;
        let mut reader = master
            .try_clone_reader()
            .map_err(|e| format!("Failed to clone PTY reader: {e}"))?;
        let writer = master
            .take_writer()
            .map_err(|e| format!("Failed to take PTY writer: {e}"))?;

        let writer_arc = Arc::new(Mutex::new(writer));
        let master_arc = Arc::new(Mutex::new(master));

        let session = PtySession {
            id: session_id.clone(),
            writer: writer_arc,
            master: master_arc,
        };

        {
            let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
            sessions.insert(session_id.clone(), session);
        }

        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 {
                    break;
                }
                on_output(buf[..n].to_vec());
            }
        });

        Ok(session_id)
    }

    pub fn write_session(&self, session_id: &str, data: &[u8]) -> Result<(), String> {
        let sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session) = sessions.get(session_id) {
            let mut writer = session.writer.lock().map_err(|e| e.to_string())?;
            writer
                .write_all(data)
                .map_err(|e| format!("Failed to write to PTY: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("Failed to flush PTY writer: {e}"))?;
            Ok(())
        } else {
            Err(format!("PTY session not found: {session_id}"))
        }
    }

    pub fn resize_session(&self, session_id: &str, cols: u16, rows: u16) -> Result<(), String> {
        let sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session) = sessions.get(session_id) {
            let master = session.master.lock().map_err(|e| e.to_string())?;
            master
                .resize(PtySize {
                    rows: rows.max(1),
                    cols: cols.max(1),
                    pixel_width: 0,
                    pixel_height: 0,
                })
                .map_err(|e| format!("Failed to resize PTY: {e}"))?;
            Ok(())
        } else {
            Err(format!("PTY session not found: {session_id}"))
        }
    }

    pub fn kill_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if sessions.remove(session_id).is_some() {
            Ok(())
        } else {
            Err(format!("PTY session not found: {session_id}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pty_session_manager_lifecycle() {
        let manager = PtySessionManager::new();
        let res = manager.spawn_session(None, &[], 80, 24, |_| {});
        if let Ok(id) = res {
            assert!(manager.write_session(&id, b"echo hello\r\n").is_ok());
            assert!(manager.resize_session(&id, 100, 30).is_ok());
            assert!(manager.kill_session(&id).is_ok());
        }
    }
}
