use crate::services::*;
use crate::models::{Project, ProjectConfig, CreateProjectInput, UpdateProjectInput, ProjectOrderItem, PinnedOrderItem};
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

// ============ New Project Management Commands ============

#[tauri::command]
pub fn get_projects() -> Result<Vec<Project>, String> {
    ConfigStorage::get_projects()
}

#[tauri::command]
pub fn get_project(id: String) -> Result<Project, String> {
    ConfigStorage::get_project(&id)
}

#[tauri::command]
pub fn create_project(name: String, working_directory: String, config: ProjectConfig) -> Result<Project, String> {
    let input = CreateProjectInput {
        name,
        working_directory,
        config,
    };
    ConfigStorage::create_project(input)
}

#[tauri::command]
pub fn update_project(id: String, name: Option<String>, working_directory: Option<String>, config: Option<ProjectConfig>, is_pinned: Option<bool>) -> Result<Project, String> {
    let updates = UpdateProjectInput {
        name,
        working_directory,
        config,
        is_pinned,
    };
    ConfigStorage::update_project(&id, updates)
}

#[tauri::command]
pub fn delete_project(id: String) -> Result<(), String> {
    ConfigStorage::delete_project(&id)
}

#[tauri::command]
pub fn launch_project(id: String) -> Result<(), String> {
    let project = ConfigStorage::get_project(&id)?;

    // Build config from project
    let mut config: HashMap<String, String> = HashMap::new();

    if project.config.mode == "claude" {
        if !project.config.proxy.is_empty() {
            config.insert("HTTP_PROXY".to_string(), project.config.proxy.clone());
            config.insert("HTTPS_PROXY".to_string(), project.config.proxy.clone());
        }
    } else {
        if !project.config.model.is_empty() {
            config.insert("ANTHROPIC_MODEL".to_string(), project.config.model.clone());
        }
        if !project.config.base_url.is_empty() {
            config.insert("ANTHROPIC_BASE_URL".to_string(), project.config.base_url.clone());
        }
        if !project.config.token.is_empty() {
            config.insert("ANTHROPIC_AUTH_TOKEN".to_string(), project.config.token.clone());
        }
    }

    if project.config.skip_permissions {
        config.insert("SKIP_PERMISSIONS".to_string(), "true".to_string());
    }

    // Launch with working directory
    Launcher::launch_with_config_and_dir(config, Some(project.working_directory.clone()))?;

    // Update last launched timestamp
    let _ = ConfigStorage::update_project_launched(&id);

    Ok(())
}

#[tauri::command]
pub async fn select_directory(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let result = app_handle
        .dialog()
        .file()
        .set_title("选择项目目录")
        .blocking_pick_folder();

    Ok(result.map(|p| p.to_string()))
}

#[tauri::command]
pub fn generate_project_powershell_command(id: String) -> Result<String, String> {
    let project = ConfigStorage::get_project(&id)?;
    let config = build_config_map(&project);
    Ok(Launcher::generate_powershell_command_with_dir(&config, Some(project.working_directory)))
}

#[tauri::command]
pub fn generate_project_cmd_command(id: String) -> Result<String, String> {
    let project = ConfigStorage::get_project(&id)?;
    let config = build_config_map(&project);
    Ok(Launcher::generate_cmd_command_with_dir(&config, Some(project.working_directory)))
}

#[tauri::command]
pub fn generate_project_bash_command(id: String) -> Result<String, String> {
    let project = ConfigStorage::get_project(&id)?;
    let config = build_config_map(&project);
    Ok(Launcher::generate_bash_command_with_dir(&config, Some(project.working_directory)))
}

fn build_config_map(project: &Project) -> HashMap<String, String> {
    let mut config: HashMap<String, String> = HashMap::new();

    if project.config.mode == "claude" {
        if !project.config.proxy.is_empty() {
            config.insert("HTTP_PROXY".to_string(), project.config.proxy.clone());
            config.insert("HTTPS_PROXY".to_string(), project.config.proxy.clone());
        }
    } else {
        if !project.config.model.is_empty() {
            config.insert("ANTHROPIC_MODEL".to_string(), project.config.model.clone());
        }
        if !project.config.base_url.is_empty() {
            config.insert("ANTHROPIC_BASE_URL".to_string(), project.config.base_url.clone());
        }
        if !project.config.token.is_empty() {
            config.insert("ANTHROPIC_AUTH_TOKEN".to_string(), project.config.token.clone());
        }
    }

    if project.config.skip_permissions {
        config.insert("SKIP_PERMISSIONS".to_string(), "true".to_string());
    }

    config
}

#[tauri::command]
pub fn get_home_directory() -> Result<String, String> {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "无法获取用户主目录".to_string())
}

#[tauri::command]
pub fn update_projects_order(orders: Vec<ProjectOrderItem>) -> Result<(), String> {
    ConfigStorage::update_projects_order(orders)
}

#[tauri::command]
pub fn update_pinned_order(orders: Vec<PinnedOrderItem>) -> Result<(), String> {
    ConfigStorage::update_pinned_order(orders)
}

#[tauri::command]
pub fn toggle_project_pinned(id: String, is_pinned: bool) -> Result<Project, String> {
    ConfigStorage::toggle_project_pinned(&id, is_pinned)
}
