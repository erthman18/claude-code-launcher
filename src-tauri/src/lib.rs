mod services;
mod commands;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::check_nodejs,
            commands::check_claude,
            commands::check_gitbash,
            commands::check_nodejs_with_update,
            commands::check_claude_with_update,
            commands::check_gitbash_with_update,
            commands::refresh_system_path,
            commands::install_nodejs,
            commands::update_nodejs,
            commands::install_claude,
            commands::update_claude,
            commands::install_gitbash,
            commands::update_gitbash,
            commands::launch_claude_code,
            commands::generate_powershell_command,
            commands::generate_cmd_command,
            commands::generate_bash_command,
            commands::get_platform,
            commands::save_to_settings,
            commands::reset_settings,
            commands::open_settings_file,
            commands::save_app_config,
            commands::load_app_config,
            // Project management commands
            commands::get_projects,
            commands::get_project,
            commands::create_project,
            commands::update_project,
            commands::delete_project,
            commands::launch_project,
            commands::select_directory,
            commands::generate_project_powershell_command,
            commands::generate_project_cmd_command,
            commands::generate_project_bash_command,
            commands::get_home_directory,
            commands::update_projects_order,
            commands::update_pinned_order,
            commands::toggle_project_pinned,
            // Onboarding commands
            commands::get_onboarding_status,
            commands::set_onboarding_completed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
