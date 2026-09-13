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

pub mod commands;
pub mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = state::AppState::new().expect("failed to initialize 4Forge app state");
    let supervisor = app_state.supervisor.clone();

    tauri::Builder::default()
        .manage(app_state)
        .on_window_event(move |_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let sup = supervisor.clone();
                tauri::async_runtime::spawn(async move {
                    sup.stop_all().await;
                });
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_services,
            commands::start_service,
            commands::stop_service,
            commands::restart_service,
            commands::toggle_all_services,
            commands::get_logs,
            commands::list_sites,
            commands::add_site,
            commands::delete_site,
            commands::list_runtimes,
            commands::set_active_runtime,
            commands::get_system_overview,
            commands::check_port_conflicts,
            commands::update_service_port,
            commands::suggest_alternative_port,
            commands::check_for_updates,
            commands::open_browser,
            commands::open_projects_folder,
            commands::open_system_terminal,
            commands::open_database_gui,
            commands::launch_database_manager,
            commands::open_adminer_in_browser,
            commands::open_service_config,
            commands::detect_project_framework,
            commands::scan_projects_directory,
            commands::open_path_in_vscode,
            commands::auto_register_detected_project,
            commands::check_hosts_sync,
            commands::sync_windows_hosts,
            commands::create_database,
            commands::open_project_terminal,
            commands::set_window_compact_mode,
            commands::terminal_spawn,
            commands::terminal_write,
            commands::terminal_resize,
            commands::terminal_kill,
        ])
        .run(tauri::generate_context!())
        .expect("error while running 4Forge application");
}
