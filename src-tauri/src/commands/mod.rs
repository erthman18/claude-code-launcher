use crate::services::*;
use std::collections::HashMap;

#[tauri::command]
pub async fn check_nodejs() -> Result<dependency_checker::DependencyStatus, String> {
    Ok(DependencyChecker::check_nodejs())
}

#[tauri::command]
pub async fn check_claude() -> Result<dependency_checker::DependencyStatus, String> {
    Ok(DependencyChecker::check_claude())
}

#[tauri::command]
pub async fn check_nodejs_with_update() -> Result<dependency_checker::DependencyStatus, String> {
    Ok(DependencyChecker::check_nodejs_with_update().await)
}

#[tauri::command]
pub async fn check_claude_with_update() -> Result<dependency_checker::DependencyStatus, String> {
    Ok(DependencyChecker::check_claude_with_update().await)
}

#[tauri::command]
pub async fn check_gitbash() -> Result<dependency_checker::DependencyStatus, String> {
    Ok(DependencyChecker::check_gitbash())
}

#[tauri::command]
pub async fn check_gitbash_with_update() -> Result<dependency_checker::DependencyStatus, String> {
    Ok(DependencyChecker::check_gitbash_with_update().await)
}

#[tauri::command]
pub fn refresh_system_path() {
    #[cfg(windows)]
    DependencyChecker::refresh_system_path();
}

#[tauri::command]
pub async fn install_nodejs() -> Result<(), String> {
    Installer::install_nodejs()
}

#[tauri::command]
pub async fn update_nodejs() -> Result<(), String> {
    Installer::update_nodejs()
}

#[tauri::command]
pub async fn install_claude() -> Result<(), String> {
    Installer::install_claude()
}

#[tauri::command]
pub async fn update_claude() -> Result<(), String> {
    Installer::update_claude()
}

#[tauri::command]
pub async fn install_gitbash() -> Result<(), String> {
    Installer::install_gitbash()
}

#[tauri::command]
pub async fn update_gitbash() -> Result<(), String> {
    Installer::update_gitbash()
}

#[tauri::command]
pub fn launch_claude_code(config: HashMap<String, String>) -> Result<(), String> {
    Launcher::launch_with_config(config)
}

#[tauri::command]
pub fn generate_powershell_command(config: HashMap<String, String>) -> String {
    Launcher::generate_powershell_command(&config)
}

#[tauri::command]
pub fn generate_cmd_command(config: HashMap<String, String>) -> String {
    Launcher::generate_cmd_command(&config)
}

#[tauri::command]
pub fn generate_bash_command(config: HashMap<String, String>) -> String {
    Launcher::generate_bash_command(&config)
}

#[tauri::command]
pub fn get_platform() -> String {
    #[cfg(windows)]
    return "windows".to_string();
    #[cfg(target_os = "macos")]
    return "macos".to_string();
    #[cfg(target_os = "linux")]
    return "linux".to_string();
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    return "unknown".to_string();
}

#[tauri::command]
pub fn save_to_settings(config: HashMap<String, String>) -> Result<(), String> {
    SettingsManager::save_config(config)
}

#[tauri::command]
pub fn reset_settings() -> Result<(), String> {
    SettingsManager::reset_config()
}

#[tauri::command]
pub fn open_settings_file() -> Result<(), String> {
    SettingsManager::open_settings_file()
}

#[tauri::command]
pub fn save_app_config(config: AppConfig) -> Result<(), String> {
    ConfigStorage::save_config(&config)
}

#[tauri::command]
pub fn load_app_config() -> Result<AppConfig, String> {
    ConfigStorage::load_config()
}
