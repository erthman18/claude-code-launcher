import { useEffect, useState } from 'react';
import { DependencyFrame } from './components/DependencyFrame';
import { ConfigPanel } from './components/ConfigPanel';
import { api } from './api';
import type { AppConfig } from './types';
import './index.css';

function App() {
  const [mode, setMode] = useState<'claude' | 'custom'>('claude');
  const [proxy, setProxy] = useState('');
  const [model, setModel] = useState('qwen3-coder-480b-a35b');
  const [baseUrl, setBaseUrl] = useState('http://litellm.uattest.weoa.com');
  const [token, setToken] = useState('');
  const [skipPermissions, setSkipPermissions] = useState(true);
  const [copySuccess, setCopySuccess] = useState(false);
  const [platform, setPlatform] = useState<'windows' | 'macos' | 'linux' | 'unknown'>('windows');

  // 加载保存的配置和平台信息
  useEffect(() => {
    loadSavedConfig();
    loadPlatform();
  }, []);

  const loadPlatform = async () => {
    try {
      const p = await api.getPlatform();
      setPlatform(p as 'windows' | 'macos' | 'linux' | 'unknown');
    } catch (error) {
      console.log('获取平台信息失败');
    }
  };

  // 配置变化时自动保存（带防抖）
  useEffect(() => {
    const timer = setTimeout(() => {
      const config: AppConfig = {
        mode,
        proxy,
        model,
        base_url: baseUrl,
        token,
        skip_permissions: skipPermissions,
      };
      api.saveAppConfig(config).catch(err => console.error('自动保存配置失败:', err));
    }, 500);

    return () => clearTimeout(timer);
  }, [mode, proxy, model, baseUrl, token, skipPermissions]);

  const loadSavedConfig = async () => {
    try {
      const config = await api.loadAppConfig();
      setMode(config.mode);
      setProxy(config.proxy);
      setModel(config.model);
      setBaseUrl(config.base_url);
      setToken(config.token);
      setSkipPermissions(config.skip_permissions ?? true);
    } catch (error) {
      console.log('加载配置失败,使用默认配置');
    }
  };


  const validateConfig = (): [boolean, string | null] => {
    if (mode === 'claude') {
      if (proxy && !proxy.startsWith('http://') && !proxy.startsWith('https://')) {
        return [false, '代理地址必须以http://或https://开头'];
      }
    } else {
      if (baseUrl && !baseUrl.startsWith('http://') && !baseUrl.startsWith('https://')) {
        return [false, 'Base URL必须以http://或https://开头'];
      }
    }
    return [true, null];
  };

  const getConfig = (): Record<string, string> => {
    const config: Record<string, string> = {};

    if (mode === 'claude') {
      if (proxy) {
        config['HTTP_PROXY'] = proxy;
        config['HTTPS_PROXY'] = proxy;
      }
    } else {
      if (model) config['ANTHROPIC_MODEL'] = model;
      if (baseUrl) config['ANTHROPIC_BASE_URL'] = baseUrl;
      if (token) config['ANTHROPIC_AUTH_TOKEN'] = token;
    }

    if (skipPermissions) {
      config['SKIP_PERMISSIONS'] = 'true';
    }

    return config;
  };

  const handleLaunch = async () => {
    const [valid, error] = validateConfig();
    if (!valid) {
      alert(error);
      return;
    }

    const config = getConfig();

    // 如果没有任何配置,确认是否继续
    if (Object.keys(config).length === 0) {
      if (!confirm('未设置任何配置,将使用默认设置启动Claude Code.\n是否继续?')) {
        return;
      }
    }

    try {
      await api.launchClaudeCode(config);
      alert('Claude Code已启动');
    } catch (error: any) {
      alert(`启动失败: ${error}`);
    }
  };

  const handleCopyPowershell = async () => {
    const [valid, error] = validateConfig();
    if (!valid) {
      alert(error);
      return;
    }

    const config = getConfig();
    try {
      const command = await api.generatePowershellCommand(config);
      await navigator.clipboard.writeText(command);
      setCopySuccess(true);
      setTimeout(() => setCopySuccess(false), 2000);
    } catch (error: any) {
      alert(`复制失败: ${error}`);
    }
  };

  const handleCopyCmd = async () => {
    const [valid, error] = validateConfig();
    if (!valid) {
      alert(error);
      return;
    }

    const config = getConfig();
    try {
      const command = await api.generateCmdCommand(config);
      await navigator.clipboard.writeText(command);
      setCopySuccess(true);
      setTimeout(() => setCopySuccess(false), 2000);
    } catch (error: any) {
      alert(`复制失败: ${error}`);
    }
  };

  const handleCopyBash = async () => {
    const [valid, error] = validateConfig();
    if (!valid) {
      alert(error);
      return;
    }

    const config = getConfig();
    try {
      const command = await api.generateBashCommand(config);
      await navigator.clipboard.writeText(command);
      setCopySuccess(true);
      setTimeout(() => setCopySuccess(false), 2000);
    } catch (error: any) {
      alert(`复制失败: ${error}`);
    }
  };

  const handleSave = async () => {
    const [valid, error] = validateConfig();
    if (!valid) {
      alert(error);
      return;
    }

    const config = getConfig();

    if (Object.keys(config).length === 0) {
      alert('没有配置需要保存');
      return;
    }

    try {
      await api.saveToSettings(config);
      alert('配置已成功保存到 .claude/settings.json');
    } catch (error: any) {
      alert(`保存失败: ${error}`);
    }
  };

  const handleReset = async () => {
    if (!confirm('是否重置配置?\n这将清除 .claude/settings.json 中的 ANTHROPIC_* 环境变量。')) {
      return;
    }

    try {
      await api.resetSettings();
      alert('配置已重置');
    } catch (error: any) {
      alert(`重置失败: ${error}`);
    }
  };

  const handleView = async () => {
    try {
      await api.openSettingsFile();
    } catch (error: any) {
      alert(error);
    }
  };

  return (
    <div className="h-screen bg-[#212121] text-[#DCE4EE] overflow-auto">
      <div className="max-w-full p-4">
        {/* 依赖检测面板 */}
        <DependencyFrame />

        {/* 配置面板 */}
        <ConfigPanel
          mode={mode}
          onModeChange={setMode}
          proxy={proxy}
          onProxyChange={setProxy}
          model={model}
          onModelChange={setModel}
          baseUrl={baseUrl}
          onBaseUrlChange={setBaseUrl}
          token={token}
          onTokenChange={setToken}
          skipPermissions={skipPermissions}
          onSkipPermissionsChange={setSkipPermissions}
          onLaunch={handleLaunch}
          onCopyPowershell={handleCopyPowershell}
          onCopyCmd={handleCopyCmd}
          onCopyBash={handleCopyBash}
          copySuccess={copySuccess}
          platform={platform}
        />

        {/* 配置保存模块 */}
        <div className="px-5 py-3">
          <div className="card-frame">
            <h2 className="text-base font-bold mb-1">配置保存(请阅读后使用)</h2>
            <p className="text-[10px] text-[#999999] px-5 py-1 max-w-[450px]">
              配置将保存到.claude文件的settings.json,后续启动时无需通过启动器,直接终端输入claude,即可
              自定义api和原版存在冲突,不能同时用,如需恢复请点重置
              通过clash等代理,可能覆盖此配置,进入claude后请输入/status检查
            </p>

            <div className="grid grid-cols-3 gap-2 px-5 py-2">
            <button
              onClick={handleSave}
              className="py-2 text-[12px] bg-[#565B5E] hover:bg-[#7A8488] text-white rounded"
            >
              保存
            </button>
            <button
              onClick={handleReset}
              className="py-2 text-[12px] bg-[#565B5E] hover:bg-[#7A8488] text-white rounded"
            >
              重置
            </button>
            <button
              onClick={handleView}
              className="py-2 text-[12px] bg-[#565B5E] hover:bg-[#7A8488] text-white rounded"
            >
              查看
            </button>
          </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
