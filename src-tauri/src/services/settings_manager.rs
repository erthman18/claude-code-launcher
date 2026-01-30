use serde_json::{Value, Map};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

pub struct SettingsManager;

impl SettingsManager {
    fn get_settings_path() -> Result<PathBuf, String> {
        let home = dirs::home_dir().ok_or("无法获取用户主目录")?;
        Ok(home.join(".claude").join("settings.json"))
    }

    fn get_claude_dir() -> Result<PathBuf, String> {
        let home = dirs::home_dir().ok_or("无法获取用户主目录")?;
        Ok(home.join(".claude"))
    }

    pub fn save_config(config: HashMap<String, String>) -> Result<(), String> {
        let claude_dir = Self::get_claude_dir()?;

        // 检查目录是否存在
        if !claude_dir.exists() {
            return Err(format!(
                "未找到 .claude 目录:\n{}\n\n请先安装 Claude Code",
                claude_dir.display()
            ));
        }

        let settings_path = Self::get_settings_path()?;

        // 读取现有配置
        let mut existing_data: Value = if settings_path.exists() {
            match fs::read_to_string(&settings_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(data) => data,
                        Err(_) => {
                            // JSON格式错误,备份原文件
                            let backup_path = settings_path.with_extension("json.bak");
                            let _ = fs::copy(&settings_path, backup_path);
                            Value::Object(Map::new())
                        }
                    }
                }
                Err(_) => Value::Object(Map::new())
            }
        } else {
            Value::Object(Map::new())
        };

        // 确保env字段存在
        if !existing_data.is_object() {
            existing_data = Value::Object(Map::new());
        }

        let obj = existing_data.as_object_mut().unwrap();
        if !obj.contains_key("env") {
            obj.insert("env".to_string(), Value::Object(Map::new()));
        }

        // 合并环境变量
        if let Some(env) = obj.get_mut("env").and_then(|v| v.as_object_mut()) {
            for (key, value) in config.iter() {
                if !value.is_empty() {
                    env.insert(key.clone(), Value::String(value.clone()));
                }
            }
        }

        // 写入文件
        let json_string = serde_json::to_string_pretty(&existing_data)
            .map_err(|e| format!("无法序列化JSON: {}", e))?;

        fs::write(&settings_path, json_string)
            .map_err(|e| format!("无法写入文件: {}", e))?;

        Ok(())
    }

    pub fn reset_config() -> Result<(), String> {
        let claude_dir = Self::get_claude_dir()?;

        // 检查目录
        if !claude_dir.exists() {
            return Err(format!("未找到 .claude 目录:{}", claude_dir.display()));
        }

        let settings_path = Self::get_settings_path()?;

        // 检查文件
        if !settings_path.exists() {
            return Ok(()); // 文件不存在,无需重置
        }

        // 读取配置
        let content = fs::read_to_string(&settings_path)
            .map_err(|e| format!("无法读取文件: {}", e))?;

        let mut data: Value = serde_json::from_str(&content)
            .map_err(|e| format!("无法解析JSON: {}", e))?;

        // 删除相关环境变量
        if let Some(env) = data.get_mut("env").and_then(|v| v.as_object_mut()) {
            let keys_to_remove: Vec<String> = env.keys()
                .filter(|k| k.starts_with("ANTHROPIC_") || k.as_str() == "HTTP_PROXY" || k.as_str() == "HTTPS_PROXY")
                .cloned()
                .collect();

            for key in keys_to_remove {
                env.remove(&key);
            }

            // 如果env为空,删除env字段
            if env.is_empty() {
                if let Some(obj) = data.as_object_mut() {
                    obj.remove("env");
                }
            }
        }

        // 如果整个配置为空,删除文件
        if data.as_object().map_or(true, |o| o.is_empty()) {
            fs::remove_file(&settings_path)
                .map_err(|e| format!("无法删除文件: {}", e))?;
        } else {
            // 写回文件
            let json_string = serde_json::to_string_pretty(&data)
                .map_err(|e| format!("无法序列化JSON: {}", e))?;

            fs::write(&settings_path, json_string)
                .map_err(|e| format!("无法写入文件: {}", e))?;
        }

        Ok(())
    }

    pub fn open_settings_file() -> Result<(), String> {
        let claude_dir = Self::get_claude_dir()?;

        // 检查目录
        if !claude_dir.exists() {
            return Err(format!(
                "未找到 .claude 目录:\n{}\n\n请先安装 Claude Code",
                claude_dir.display()
            ));
        }

        let settings_path = Self::get_settings_path()?;

        // 检查文件
        if !settings_path.exists() {
            return Err(format!(
                "配置文件不存在:\n{}\n\n请先点击\"保存\"按钮创建配置",
                settings_path.display()
            ));
        }

        // 使用默认程序打开文件
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            Command::new("cmd")
                .args(&["/C", "start", "", settings_path.to_str().unwrap()])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .map_err(|e| format!("无法打开文件:{}", e))?;
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(settings_path.to_str().unwrap())
                .spawn()
                .map_err(|e| format!("无法打开文件:{}", e))?;
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(settings_path.to_str().unwrap())
                .spawn()
                .map_err(|e| format!("无法打开文件:{}", e))?;
        }

        Ok(())
    }
}

use std::process::Command;
