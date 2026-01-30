pub struct Environment;

impl Environment {
    #[cfg(windows)]
    pub fn set_permanent(key: &str, value: &str) -> Result<(), String> {
        use winreg::RegKey;
        use winreg::enums::*;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env = hkcu
            .open_subkey_with_flags("Environment", KEY_SET_VALUE)
            .map_err(|e| format!("无法打开注册表: {}", e))?;

        if !value.is_empty() {
            env.set_value(key, &value)
                .map_err(|e| format!("无法设置环境变量: {}", e))?;
        } else {
            // 值为空则删除
            let _ = env.delete_value(key);
        }

        // 广播环境变量变更消息
        Self::broadcast_environment_change();

        Ok(())
    }

    #[cfg(windows)]
    fn broadcast_environment_change() {
        use windows::Win32::Foundation::*;
        use windows::Win32::UI::WindowsAndMessaging::*;

        unsafe {
            let hwnd_broadcast = HWND(0xFFFF as *mut _);
            let wm_settingchange: u32 = 0x001A;

            let env_str = "Environment\0";
            let env_wide: Vec<u16> = env_str.encode_utf16().collect();

            let _ = SendMessageTimeoutW(
                hwnd_broadcast,
                wm_settingchange,
                WPARAM(0),
                LPARAM(env_wide.as_ptr() as isize),
                SMTO_ABORTIFHUNG,
                5000,
                None,
            );
        }
    }

    #[cfg(not(windows))]
    pub fn set_permanent(_key: &str, _value: &str) -> Result<(), String> {
        Err("此功能仅在Windows上可用".to_string())
    }

    pub fn get_env_keys() -> Vec<String> {
        vec![
            "ANTHROPIC_MODEL".to_string(),
            "ANTHROPIC_BASE_URL".to_string(),
            "ANTHROPIC_AUTH_TOKEN".to_string(),
            "HTTP_PROXY".to_string(),
            "HTTPS_PROXY".to_string(),
        ]
    }
}
