use std::collections::HashMap;
use std::process::Command;

pub struct Launcher;

impl Launcher {
    pub fn launch_with_config(config: HashMap<String, String>) -> Result<(), String> {
        Self::launch_with_temp_env(config, None)
    }

    pub fn launch_with_config_and_dir(config: HashMap<String, String>, working_dir: Option<String>) -> Result<(), String> {
        Self::launch_with_temp_env(config, working_dir)
    }

    pub fn launch_simple() -> Result<(), String> {
        #[cfg(windows)]
        {
            Self::execute_windows("claude")
        }
        #[cfg(target_os = "macos")]
        {
            Self::execute_macos("claude", None)
        }
        #[cfg(all(not(windows), not(target_os = "macos")))]
        {
            Err("不支持的操作系统".to_string())
        }
    }

    fn launch_with_temp_env(config: HashMap<String, String>, working_dir: Option<String>) -> Result<(), String> {
        let ordered_keys = [
            "ANTHROPIC_MODEL",
            "ANTHROPIC_BASE_URL",
            "ANTHROPIC_AUTH_TOKEN",
            "HTTP_PROXY",
            "HTTPS_PROXY",
        ];

        let skip_permissions = config.get("SKIP_PERMISSIONS").map(|v| v == "true").unwrap_or(false);
        let claude_cmd = if skip_permissions {
            "claude --dangerously-skip-permissions"
        } else {
            "claude"
        };

        #[cfg(windows)]
        {
            let mut commands = Vec::new();
            for key in ordered_keys.iter() {
                if let Some(value) = config.get(*key) {
                    if !value.is_empty() {
                        let escaped_value = value.replace("\"", "`\"");
                        commands.push(format!("$env:{}=\"{}\"", key, escaped_value));
                    }
                }
            }
            commands.push(claude_cmd.to_string());
            let full_command = format!("& {{ {} }}", commands.join("; "));
            let _ = working_dir; // Windows uses existing logic
            Self::execute_windows(&full_command)
        }

        #[cfg(target_os = "macos")]
        {
            let mut env_exports = Vec::new();
            for key in ordered_keys.iter() {
                if let Some(value) = config.get(*key) {
                    if !value.is_empty() {
                        let escaped_value = value.replace("\"", "\\\"");
                        env_exports.push(format!("export {}=\"{}\"", key, escaped_value));
                    }
                }
            }
            env_exports.push(claude_cmd.to_string());
            let full_command = env_exports.join(" && ");
            Self::execute_macos(&full_command, working_dir)
        }

        #[cfg(all(not(windows), not(target_os = "macos")))]
        {
            Err("不支持的操作系统".to_string())
        }
    }

    #[cfg(windows)]
    fn execute_windows(command: &str) -> Result<(), String> {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_CONSOLE: u32 = 0x00000010;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        // First check if claude command exists
        let check = Command::new("powershell")
            .args(&["-Command", "where.exe claude 2>$null"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| format!("无法检查Claude命令: {}", e))?;

        if !check.status.success() || check.stdout.is_empty() {
            let npm_check = Command::new("powershell")
                .args(&["-Command", "npm list -g @anthropic-ai/claude-code --depth=0 2>$null"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .map_err(|e| format!("无法检查npm包: {}", e))?;

            if !npm_check.status.success() {
                return Err("Claude Code未安装或未在PATH中。请先安装Claude Code。".to_string());
            }
        }

        let home_dir = dirs::home_dir()
            .ok_or("无法获取用户主目录".to_string())?;

        let enhanced_command = format!(
            "Write-Host 'Starting Claude Code...' -ForegroundColor Green; {}; if ($LASTEXITCODE -ne 0) {{ Write-Host 'Claude Code exited with error' -ForegroundColor Red; Read-Host 'Press Enter to exit' }}",
            command
        );

        Command::new("powershell")
            .args(&[
                "-NoExit",
                "-ExecutionPolicy", "Bypass",
                "-Command", &enhanced_command
            ])
            .current_dir(home_dir)
            .creation_flags(CREATE_NEW_CONSOLE)
            .spawn()
            .map_err(|e| format!("无法启动PowerShell: {}", e))?;

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn execute_macos(command: &str, working_dir: Option<String>) -> Result<(), String> {
        // Note: We don't check for claude existence here because:
        // 1. Terminal.app will launch with a login shell that loads .zshrc/.bash_profile
        // 2. This means PATH will include npm global bin, homebrew, nvm, etc.
        // 3. The GUI app doesn't have this PATH, so `which claude` would fail even when claude is installed
        // 4. If claude is not installed, the user will see the error directly in Terminal

        let target_dir = working_dir.unwrap_or_else(|| {
            dirs::home_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "~".to_string())
        });

        // Use osascript to open Terminal.app with the command
        // Terminal.app runs as a login shell by default, so PATH will be correct
        let script = format!(
            r#"tell application "Terminal"
                activate
                do script "cd '{}' && echo 'Starting Claude Code...' && {}"
            end tell"#,
            target_dir.replace("'", "'\\''"),
            command.replace("\"", "\\\"")
        );

        Command::new("osascript")
            .args(&["-e", &script])
            .spawn()
            .map_err(|e| format!("无法启动Terminal: {}", e))?;

        Ok(())
    }

    // Windows: PowerShell command
    pub fn generate_powershell_command(config: &HashMap<String, String>) -> String {
        let mut commands = Vec::new();
        let ordered_keys = [
            "ANTHROPIC_MODEL",
            "ANTHROPIC_BASE_URL",
            "ANTHROPIC_AUTH_TOKEN",
            "HTTP_PROXY",
            "HTTPS_PROXY",
        ];

        for key in ordered_keys.iter() {
            if let Some(value) = config.get(*key) {
                if !value.is_empty() {
                    let escaped_value = value.replace("\"", "\\\"");
                    commands.push(format!("$env:{}=\"{}\"", key, escaped_value));
                }
            }
        }

        let claude_cmd = if config.get("SKIP_PERMISSIONS").map(|v| v == "true").unwrap_or(false) {
            "claude --dangerously-skip-permissions".to_string()
        } else {
            "claude".to_string()
        };
        commands.push(claude_cmd);
        commands.join(";")
    }

    // Windows: CMD command
    pub fn generate_cmd_command(config: &HashMap<String, String>) -> String {
        let mut commands = Vec::new();
        let ordered_keys = [
            "ANTHROPIC_MODEL",
            "ANTHROPIC_BASE_URL",
            "ANTHROPIC_AUTH_TOKEN",
            "HTTP_PROXY",
            "HTTPS_PROXY",
        ];

        for key in ordered_keys.iter() {
            if let Some(value) = config.get(*key) {
                if !value.is_empty() {
                    let escaped = if value.contains(' ') || value.contains('&') || value.contains('|') || value.contains('"') {
                        value.replace("\"", "\"\"")
                    } else {
                        value.clone()
                    };
                    commands.push(format!("set {}={}", key, escaped));
                }
            }
        }

        let claude_cmd = if config.get("SKIP_PERMISSIONS").map(|v| v == "true").unwrap_or(false) {
            "claude --dangerously-skip-permissions".to_string()
        } else {
            "claude".to_string()
        };
        commands.push(claude_cmd);
        commands.join(" & ")
    }

    // macOS/Linux: Bash command
    pub fn generate_bash_command(config: &HashMap<String, String>) -> String {
        let mut commands = Vec::new();
        let ordered_keys = [
            "ANTHROPIC_MODEL",
            "ANTHROPIC_BASE_URL",
            "ANTHROPIC_AUTH_TOKEN",
            "HTTP_PROXY",
            "HTTPS_PROXY",
        ];

        for key in ordered_keys.iter() {
            if let Some(value) = config.get(*key) {
                if !value.is_empty() {
                    let escaped_value = value.replace("\"", "\\\"");
                    commands.push(format!("export {}=\"{}\"", key, escaped_value));
                }
            }
        }

        let claude_cmd = if config.get("SKIP_PERMISSIONS").map(|v| v == "true").unwrap_or(false) {
            "claude --dangerously-skip-permissions".to_string()
        } else {
            "claude".to_string()
        };
        commands.push(claude_cmd);
        commands.join(" && ")
    }

    // With working directory variants
    pub fn generate_powershell_command_with_dir(config: &HashMap<String, String>, working_dir: Option<String>) -> String {
        let base = Self::generate_powershell_command(config);
        if let Some(dir) = working_dir {
            format!("cd \"{}\"; {}", dir, base)
        } else {
            base
        }
    }

    pub fn generate_cmd_command_with_dir(config: &HashMap<String, String>, working_dir: Option<String>) -> String {
        let base = Self::generate_cmd_command(config);
        if let Some(dir) = working_dir {
            format!("cd /d \"{}\" & {}", dir, base)
        } else {
            base
        }
    }

    pub fn generate_bash_command_with_dir(config: &HashMap<String, String>, working_dir: Option<String>) -> String {
        let base = Self::generate_bash_command(config);
        if let Some(dir) = working_dir {
            format!("cd '{}' && {}", dir.replace("'", "'\\''"), base)
        } else {
            base
        }
    }
}
