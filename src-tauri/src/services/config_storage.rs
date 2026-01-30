use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use base64::{Engine as _, engine::general_purpose};

fn default_skip_permissions() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub mode: String,
    pub proxy: String,
    pub model: String,
    pub base_url: String,
    pub token: String,
    #[serde(default = "default_skip_permissions")]
    pub skip_permissions: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mode: "claude".to_string(),
            proxy: String::new(),
            model: "qwen3-coder-480b-a35b".to_string(),
            base_url: "http://litellm.uattest.weoa.com".to_string(),
            token: String::new(),
            skip_permissions: true,
        }
    }
}

pub struct ConfigStorage;

impl ConfigStorage {
    fn get_config_path() -> Result<PathBuf, String> {
        // 跨平台配置目录:
        // Windows: C:\Users\<user>\AppData\Roaming\ClaudeCodeLauncher
        // macOS: ~/Library/Application Support/ClaudeCodeLauncher
        // Linux: ~/.config/ClaudeCodeLauncher
        let config_dir = dirs::config_dir()
            .ok_or("无法获取配置目录")?
            .join("ClaudeCodeLauncher");

        // 确保目录存在
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("无法创建配置目录: {}", e))?;
        }

        Ok(config_dir.join("config.json"))
    }

    pub fn save_config(config: &AppConfig) -> Result<(), String> {
        let config_path = Self::get_config_path()?;

        // 创建副本用于保存
        let mut config_to_save = config.clone();

        // Token Base64编码(简单混淆)
        if !config_to_save.token.is_empty() {
            config_to_save.token = general_purpose::STANDARD.encode(&config_to_save.token);
        }

        // 写入JSON文件
        let json_string = serde_json::to_string_pretty(&config_to_save)
            .map_err(|e| format!("无法序列化配置: {}", e))?;

        fs::write(&config_path, json_string)
            .map_err(|e| format!("无法写入配置文件: {}", e))?;

        Ok(())
    }

    pub fn load_config() -> Result<AppConfig, String> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Ok(AppConfig::default());
        }

        // 读取文件
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("无法读取配置文件: {}", e))?;

        let mut config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| format!("无法解析配置文件: {}", e))?;

        // Token Base64解码
        if !config.token.is_empty() {
            match general_purpose::STANDARD.decode(&config.token) {
                Ok(decoded) => {
                    if let Ok(token_str) = String::from_utf8(decoded) {
                        config.token = token_str;
                    }
                }
                Err(_) => {
                    // 解码失败,保持原值
                }
            }
        }

        Ok(config)
    }
}
