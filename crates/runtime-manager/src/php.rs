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
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhpConfig {
    pub version: String,
    pub base_dir: PathBuf,
    pub port: u16,
    pub extensions: Vec<String>,
    pub memory_limit: String,
    pub upload_max_filesize: String,
    pub post_max_size: String,
    pub max_execution_time: u32,
}

impl PhpConfig {
    pub fn new(version: &str, base_dir: PathBuf, port: u16) -> Self {
        Self {
            version: version.to_string(),
            base_dir,
            port,
            extensions: vec![
                "curl".to_string(),
                "fileinfo".to_string(),
                "mbstring".to_string(),
                "openssl".to_string(),
                "pdo_mysql".to_string(),
            ],
            memory_limit: "512M".to_string(),
            upload_max_filesize: "64M".to_string(),
            post_max_size: "64M".to_string(),
            max_execution_time: 300,
        }
    }

    pub fn generate_ini(&self) -> String {
        let ext_dir = self
            .base_dir
            .join("ext")
            .to_string_lossy()
            .replace('\\', "/");
        let mut extensions_block = String::new();
        for ext in &self.extensions {
            extensions_block.push_str(&format!("extension={}\r\n", ext));
        }

        format!(
            "[PHP]\r\n\
             engine = On\r\n\
             short_open_tag = Off\r\n\
             precision = 14\r\n\
             output_buffering = 4096\r\n\
             max_execution_time = {}\r\n\
             memory_limit = {}\r\n\
             error_reporting = E_ALL\r\n\
             display_errors = On\r\n\
             display_startup_errors = On\r\n\
             log_errors = On\r\n\
             post_max_size = {}\r\n\
             default_charset = \"UTF-8\"\r\n\
             file_uploads = On\r\n\
             upload_max_filesize = {}\r\n\
             max_file_uploads = 20\r\n\
             extension_dir = \"{}\"\r\n\
             {}\r\n\
             [Date]\r\n\
             date.timezone = \"UTC\"\r\n",
            self.max_execution_time,
            self.memory_limit,
            self.post_max_size,
            self.upload_max_filesize,
            ext_dir,
            extensions_block
        )
    }

    pub fn ini_path(&self) -> PathBuf {
        self.base_dir.join("php.ini")
    }

    pub fn write_ini(&self) -> Result<PathBuf, std::io::Error> {
        let path = self.ini_path();
        std::fs::write(&path, self.generate_ini())?;
        Ok(path)
    }

    pub fn build_fastcgi_command(&self) -> (PathBuf, Vec<String>) {
        let cgi_bin = self.base_dir.join("php-cgi.exe");
        let args = vec![
            "-b".to_string(),
            format!("127.0.0.1:{}", self.port),
            "-c".to_string(),
            self.ini_path().display().to_string(),
        ];
        (cgi_bin, args)
    }
}

pub struct PhpRuntime;

impl PhpRuntime {
    pub fn build_fastcgi_command(base_dir: &Path, port: u16) -> (PathBuf, Vec<String>) {
        let cgi_bin = base_dir.join("php-cgi.exe");
        let ini_file = base_dir.join("php.ini");
        let args = vec![
            "-b".to_string(),
            format!("127.0.0.1:{}", port),
            "-c".to_string(),
            ini_file.display().to_string(),
        ];
        (cgi_bin, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_php_ini_generation() {
        let base = PathBuf::from("C:\\4forge\\runtimes\\php\\8.3.16");
        let config = PhpConfig::new("8.3.16", base, 9000);
        let ini = config.generate_ini();

        assert!(ini.contains("memory_limit = 512M"));
        assert!(ini.contains("extension=curl"));
        assert!(ini.contains("extension=pdo_mysql"));
        assert!(ini.contains("extension_dir = \"C:/4forge/runtimes/php/8.3.16/ext\""));

        let (cmd, args) = config.build_fastcgi_command();
        assert!(cmd.ends_with("php-cgi.exe"));
        assert!(args.contains(&"127.0.0.1:9000".to_string()));
    }
}
