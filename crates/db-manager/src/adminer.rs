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
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseLaunchResult {
    pub launched_type: String,
    pub client_name: String,
    pub url_or_path: String,
}

pub struct AdminerManager;

impl AdminerManager {
    pub const ADMINER_PHP_CONTENT: &'static str = r#"<?php
error_reporting(E_ALL & ~E_NOTICE);
ini_set('display_errors', '0');

$host = isset($_GET['host']) ? $_GET['host'] : '127.0.0.1';
$port = isset($_GET['port']) ? intval($_GET['port']) : 3306;
$user = isset($_GET['user']) ? $_GET['user'] : 'root';
$pass = isset($_GET['pass']) ? $_GET['pass'] : '';
$db   = isset($_GET['db']) ? $_GET['db'] : '';

$conn = @new mysqli($host, $user, $pass, $db, $port);
$connected = !$conn->connect_error;

$action = isset($_GET['action']) ? $_GET['action'] : 'dashboard';
$query_result = null;
$query_error = null;
$sql_input = isset($_POST['sql']) ? trim($_POST['sql']) : '';

if ($connected && !empty($sql_input)) {
    $res = $conn->query($sql_input);
    if ($res === false) {
        $query_error = $conn->error;
    } else {
        $query_result = $res;
    }
}

$databases = [];
if ($connected) {
    $db_res = $conn->query("SHOW DATABASES");
    if ($db_res) {
        while ($row = $db_res->fetch_array()) {
            $databases[] = $row[0];
        }
    }
}

$tables = [];
if ($connected && !empty($db)) {
    $conn->select_db($db);
    $tb_res = $conn->query("SHOW TABLES");
    if ($tb_res) {
        while ($row = $tb_res->fetch_array()) {
            $tables[] = $row[0];
        }
    }
}
?>
<!DOCTYPE html>
<html lang="en" class="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>4Forge Database Manager (Adminer)</title>
    <style>
        :root {
            --bg-main: #0B0F17;
            --bg-card: #121929;
            --bg-card-header: #192237;
            --text-main: #F1F5F9;
            --text-muted: #94A3B8;
            --border-color: #1E293B;
            --accent-cyan: #06B6D4;
            --accent-green: #10B981;
            --accent-rose: #F43F5E;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: var(--bg-main);
            color: var(--text-main);
            display: flex;
            height: 100vh;
            overflow: hidden;
        }
        aside {
            width: 260px;
            background: #0E1524;
            border-right: 1px solid var(--border-color);
            display: flex;
            flex-direction: column;
        }
        .aside-header {
            padding: 18px 20px;
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .aside-header h1 { font-size: 15px; font-weight: 700; color: #fff; }
        .aside-header p { font-size: 11px; color: var(--text-muted); }
        .db-list { flex: 1; overflow-y: auto; padding: 12px; }
        .db-list h2 { font-size: 11px; text-transform: uppercase; color: var(--text-muted); margin: 8px 8px 6px; letter-spacing: 0.05em; }
        .db-item, .table-item {
            display: block;
            padding: 7px 10px;
            border-radius: 8px;
            font-size: 12px;
            color: #cbd5e1;
            text-decoration: none;
            margin-bottom: 2px;
            font-family: ui-monospace, monospace;
        }
        .db-item:hover, .table-item:hover { background: rgba(255,255,255,0.06); color: #fff; }
        .db-item.active { background: rgba(6,182,212,0.15); color: var(--accent-cyan); font-weight: 600; }
        main {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
        header {
            height: 60px;
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 24px;
            background: rgba(14,21,36,0.6);
        }
        .status-pill {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            font-size: 11px;
            padding: 4px 10px;
            border-radius: 9999px;
            border: 1px solid rgba(16,185,129,0.3);
            background: rgba(16,185,129,0.1);
            color: var(--accent-green);
            font-weight: 600;
        }
        .status-pill.error {
            border-color: rgba(244,63,94,0.3);
            background: rgba(244,63,94,0.1);
            color: var(--accent-rose);
        }
        .content {
            flex: 1;
            padding: 24px;
            overflow-y: auto;
        }
        .card {
            background: var(--bg-card);
            border: 1px solid var(--border-color);
            border-radius: 12px;
            padding: 20px;
            margin-bottom: 20px;
        }
        textarea.sql-input {
            width: 100%;
            height: 90px;
            background: #090D14;
            border: 1px solid var(--border-color);
            border-radius: 8px;
            color: #E2E8F0;
            font-family: ui-monospace, monospace;
            padding: 10px;
            font-size: 12px;
            resize: vertical;
        }
        .btn {
            background: linear-gradient(to right, #06B6D4, #2563EB);
            color: #fff;
            border: none;
            padding: 8px 16px;
            border-radius: 8px;
            font-size: 12px;
            font-weight: 600;
            cursor: pointer;
            margin-top: 8px;
        }
        table.data-table {
            width: 100%;
            border-collapse: collapse;
            font-size: 12px;
            margin-top: 10px;
        }
        table.data-table th, table.data-table td {
            padding: 8px 12px;
            text-align: left;
            border-bottom: 1px solid rgba(255,255,255,0.06);
        }
        table.data-table th { background: #0B101C; color: var(--text-muted); font-size: 11px; text-transform: uppercase; }
        .meta-tag { font-family: monospace; font-size: 11px; background: #1E293B; padding: 2px 6px; border-radius: 4px; color: var(--accent-cyan); }
    </style>
</head>
<body>
    <aside>
        <div class="aside-header">
            <div>
                <h1>4Forge Adminer</h1>
                <p>MariaDB / MySQL Zero-Config</p>
            </div>
        </div>
        <div class="db-list">
            <h2>Databases</h2>
            <?php foreach ($databases as $d): ?>
                <a href="?host=<?= urlencode($host) ?>&port=<?= $port ?>&user=<?= urlencode($user) ?>&db=<?= urlencode($d) ?>" class="db-item <?= $db === $d ? 'active' : '' ?>">
                    📁 <?= htmlspecialchars($d) ?>
                </a>
            <?php endforeach; ?>
            <?php if (!empty($tables)): ?>
                <h2 style="margin-top: 16px;">Tables (<?= htmlspecialchars($db) ?>)</h2>
                <?php foreach ($tables as $t): ?>
                    <a href="?host=<?= urlencode($host) ?>&port=<?= $port ?>&user=<?= urlencode($user) ?>&db=<?= urlencode($db) ?>&table=<?= urlencode($t) ?>" class="table-item">
                        📊 <?= htmlspecialchars($t) ?>
                    </a>
                <?php endforeach; ?>
            <?php endif; ?>
        </div>
    </aside>

    <main>
        <header>
            <div style="display:flex; align-items:center; gap:12px;">
                <span class="meta-tag"><?= htmlspecialchars($user) ?>@<?= htmlspecialchars($host) ?>:<?= $port ?></span>
                <?php if (!empty($db)): ?>
                    <span class="meta-tag">db: <?= htmlspecialchars($db) ?></span>
                <?php endif; ?>
            </div>
            <div>
                <?php if ($connected): ?>
                    <span class="status-pill">● Database Connected</span>
                <?php else: ?>
                    <span class="status-pill error">✕ <?= htmlspecialchars($conn->connect_error) ?></span>
                <?php endif; ?>
            </div>
        </header>

        <div class="content">
            <div class="card">
                <h3 style="font-size:14px; margin-bottom:10px;">SQL Query Console</h3>
                <form method="POST">
                    <textarea name="sql" class="sql-input" placeholder="SELECT * FROM ...;"><?= htmlspecialchars($sql_input) ?></textarea>
                    <button type="submit" class="btn">Execute Query</button>
                </form>
                <?php if ($query_error): ?>
                    <div style="margin-top:10px; color:var(--accent-rose); font-size:12px; font-family:monospace;">
                        Error: <?= htmlspecialchars($query_error) ?>
                    </div>
                <?php elseif ($query_result instanceof mysqli_result): ?>
                    <div style="margin-top:14px; overflow-x:auto;">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <?php $fields = $query_result->fetch_fields(); foreach ($fields as $f): ?>
                                        <th><?= htmlspecialchars($f->name) ?></th>
                                    <?php endforeach; ?>
                                </tr>
                            </thead>
                            <tbody>
                                <?php while ($row = $query_result->fetch_assoc()): ?>
                                    <tr>
                                        <?php foreach ($row as $val): ?>
                                            <td><?= htmlspecialchars($val !== null ? $val : 'NULL') ?></td>
                                        <?php endforeach; ?>
                                    </tr>
                                <?php endwhile; ?>
                            </tbody>
                        </table>
                    </div>
                <?php elseif ($query_result === true): ?>
                    <div style="margin-top:10px; color:var(--accent-green); font-size:12px;">
                        Query executed successfully. Affected rows: <?= $conn->affected_rows ?>
                    </div>
                <?php endif; ?>
            </div>

            <?php if (!empty($_GET['table']) && $connected): 
                $table_name = preg_replace('/[^a-zA-Z0-9_]/', '', $_GET['table']);
                $rows = $conn->query("SELECT * FROM `$table_name` LIMIT 50");
            ?>
                <div class="card">
                    <h3 style="font-size:14px; margin-bottom:10px;">Data: <?= htmlspecialchars($table_name) ?> (Limit 50)</h3>
                    <?php if ($rows && $rows->num_rows > 0): ?>
                        <div style="overflow-x:auto;">
                            <table class="data-table">
                                <thead>
                                    <tr>
                                        <?php $fields = $rows->fetch_fields(); foreach ($fields as $f): ?>
                                            <th><?= htmlspecialchars($f->name) ?></th>
                                        <?php endforeach; ?>
                                    </tr>
                                </thead>
                                <tbody>
                                    <?php while ($r = $rows->fetch_assoc()): ?>
                                        <tr>
                                            <?php foreach ($r as $v): ?>
                                                <td><?= htmlspecialchars($v !== null ? $v : 'NULL') ?></td>
                                            <?php endforeach; ?>
                                        </tr>
                                    <?php endwhile; ?>
                                </tbody>
                            </table>
                        </div>
                    <?php else: ?>
                        <p style="font-size:12px; color:var(--text-muted);">Table is empty.</p>
                    <?php endif; ?>
                </div>
            <?php endif; ?>
        </div>
    </main>
</body>
</html>
"#;

    pub fn ensure_adminer_script(target_dir: &Path) -> std::io::Result<PathBuf> {
        let adminer_dir = target_dir.join("adminer");
        if !adminer_dir.exists() {
            std::fs::create_dir_all(&adminer_dir)?;
        }
        let index_php = adminer_dir.join("index.php");
        if !index_php.exists() {
            std::fs::write(&index_php, Self::ADMINER_PHP_CONTENT)?;
        }
        Ok(index_php)
    }

    pub fn get_adminer_url(web_port: u16) -> String {
        if web_port == 80 {
            "http://localhost/__4forge/db".to_string()
        } else {
            format!("http://localhost:{}/__4forge/db", web_port)
        }
    }

    pub fn find_native_client() -> Option<PathBuf> {
        let candidates = [
            "heidisql.exe",
            "dbeaver.exe",
            "tableplus.exe",
            "mysqlworkbench.exe",
            "C:\\Program Files\\HeidiSQL\\heidisql.exe",
            "D:\\laragon\\bin\\heidisql\\heidisql.exe",
            "C:\\Program Files\\DBeaver\\dbeaver.exe",
            "C:\\Program Files\\TablePlus\\TablePlus.exe",
        ];

        for c in candidates {
            let p = PathBuf::from(c);
            if p.is_file() {
                return Some(p);
            }
            if let Ok(paths) = std::env::var("PATH") {
                for dir in std::env::split_paths(&paths) {
                    let full = dir.join(c);
                    if full.is_file() {
                        return Some(full);
                    }
                }
            }
        }

        None
    }

    pub fn find_mongodb_client() -> Option<PathBuf> {
        let mut candidates = vec![
            PathBuf::from("MongoDBCompass.exe"),
            PathBuf::from("mongosh.exe"),
            PathBuf::from("C:\\Program Files\\MongoDB Compass\\MongoDBCompass.exe"),
        ];

        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            candidates.push(
                PathBuf::from(local_app_data)
                    .join("Programs")
                    .join("MongoDB Compass")
                    .join("MongoDBCompass.exe"),
            );
        }

        for c in candidates {
            if c.is_file() {
                return Some(c);
            }
            if let Some(file_name) = c.file_name() {
                if let Ok(paths) = std::env::var("PATH") {
                    for dir in std::env::split_paths(&paths) {
                        let full = dir.join(file_name);
                        if full.is_file() {
                            return Some(full);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn launch_mongodb_ui(uri: &str) -> std::io::Result<DatabaseLaunchResult> {
        if let Some(client) = Self::find_mongodb_client() {
            let name = client
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("MongoDB Client")
                .to_string();

            let mut cmd = Command::new(&client);
            cmd.arg(uri);
            cmd.spawn()?;

            Ok(DatabaseLaunchResult {
                launched_type: "native".to_string(),
                client_name: name,
                url_or_path: client.to_string_lossy().to_string(),
            })
        } else {
            let data_dir = PathBuf::from("C:\\4forge\\data\\mongodb");
            let _ = std::fs::create_dir_all(&data_dir);
            #[cfg(windows)]
            {
                let _ = Command::new("explorer.exe").arg(&data_dir).spawn();
            }
            Ok(DatabaseLaunchResult {
                launched_type: "folder".to_string(),
                client_name: "MongoDB Data Directory".to_string(),
                url_or_path: uri.to_string(),
            })
        }
    }

    pub fn launch_database_ui(
        tools_root: &Path,
        web_port: u16,
        host: &str,
        port: u16,
        user: &str,
    ) -> std::io::Result<DatabaseLaunchResult> {
        if let Some(client) = Self::find_native_client() {
            let name = client
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Desktop Database Client")
                .to_string();

            let mut cmd = Command::new(&client);
            if name.to_lowercase().contains("heidisql") {
                let net_type = if port == 5432 { "2" } else { "0" };
                cmd.arg(format!("--net-type={}", net_type))
                    .arg(format!("--host={}", host))
                    .arg(format!("--port={}", port))
                    .arg(format!("--user={}", user));
            }
            cmd.spawn()?;

            Ok(DatabaseLaunchResult {
                launched_type: "native".to_string(),
                client_name: name,
                url_or_path: client.to_string_lossy().to_string(),
            })
        } else {
            let _ = Self::ensure_adminer_script(tools_root);
            let url = Self::get_adminer_url(web_port);

            #[cfg(windows)]
            {
                let _ = Command::new("rundll32.exe")
                    .args(["url.dll,FileProtocolHandler", &url])
                    .spawn();
            }
            #[cfg(not(windows))]
            {
                let _ = Command::new("xdg-open").arg(&url).spawn();
            }

            Ok(DatabaseLaunchResult {
                launched_type: "web_adminer".to_string(),
                client_name: "Adminer (Built-in)".to_string(),
                url_or_path: url,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adminer_script_generation() {
        let tmp = std::env::temp_dir().join("4forge_test_adminer_dir");
        let script = AdminerManager::ensure_adminer_script(&tmp);
        assert!(script.is_ok());
        let path = script.unwrap();
        assert!(path.is_file());
        assert!(path.ends_with("index.php"));
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("4Forge Database Manager"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_get_adminer_url() {
        assert_eq!(
            AdminerManager::get_adminer_url(80),
            "http://localhost/__4forge/db"
        );
        assert_eq!(
            AdminerManager::get_adminer_url(8080),
            "http://localhost:8080/__4forge/db"
        );
    }
}
