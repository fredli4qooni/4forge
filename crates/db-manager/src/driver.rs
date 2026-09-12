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

use crate::{DatabaseEngine, DatabaseError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseCommand {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub current_dir: Option<PathBuf>,
    pub envs: HashMap<String, String>,
}

pub trait DatabaseDriver: Send + Sync {
    fn engine(&self) -> DatabaseEngine;
    fn port(&self) -> Option<u16>;
    fn data_dir(&self) -> &Path;
    fn is_initialized(&self) -> bool;
    fn prepare_environment(&self) -> Result<(), DatabaseError>;
    fn build_init_command(&self) -> Option<DatabaseCommand>;
    fn build_start_command(&self) -> Result<DatabaseCommand, DatabaseError>;
    fn build_stop_command(&self) -> Option<DatabaseCommand>;
}
