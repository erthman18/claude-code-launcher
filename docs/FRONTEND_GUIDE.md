# 前端开发指南

> **技术栈**: React 19 + TypeScript + Vite + Tailwind CSS
> **最后更新**: 2026-02-03

---

## 📋 目录

- [1. 技术栈概览](#1-技术栈概览)
- [2. 项目结构](#2-项目结构)
- [3. 组件详解](#3-组件详解)
- [4. 状态管理](#4-状态管理)
- [5. API 调用](#5-api-调用)
- [6. 类型系统](#6-类型系统)
- [7. 样式设计](#7-样式设计)
- [8. 开发实践](#8-开发实践)

---

## 1. 技术栈概览

### 1.1 核心依赖

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2",           // Tauri 前端 API
    "@tauri-apps/plugin-opener": "^2", // 打开文件/URL 插件
    "react": "^19.1.0",                // React 框架
    "react-dom": "^19.1.0"             // React DOM 渲染
  }
}
```

### 1.2 开发依赖

```json
{
  "devDependencies": {
    "@types/react": "^19.1.8",         // React 类型定义
    "@types/react-dom": "^19.1.6",     // React DOM 类型
    "@vitejs/plugin-react": "^4.6.0",  // Vite React 插件
    "autoprefixer": "^10.4.22",        // CSS 前缀自动化
    "postcss": "^8.5.6",               // CSS 处理器
    "tailwindcss": "^3.4.0",           // 实用优先的 CSS 框架
    "typescript": "~5.8.3",            // TypeScript 编译器
    "vite": "^7.0.4"                   // 快速构建工具
  }
}
```

---

## 2. 项目结构

```
src/
├── main.tsx                 # 应用入口，ReactDOM 渲染
├── App.tsx                  # 主应用组件，状态管理和布局
├── index.css                # 全局样式，Tailwind 基础
├── types.ts                 # TypeScript 类型定义
├── api.ts                   # Tauri API 调用封装
└── components/
    ├── DependencyFrame.tsx  # 依赖检测面板组件
    └── ConfigPanel.tsx      # 配置参数面板组件
```

### 2.1 文件职责

| 文件 | 职责 | 代码行数 |
|------|------|----------|
| `main.tsx` | 应用挂载，React 根节点 | ~10 行 |
| `App.tsx` | 主逻辑、状态管理、布局 | ~250 行 |
| `DependencyFrame.tsx` | 依赖检测 UI 和交互 | ~200 行 |
| `ConfigPanel.tsx` | 配置表单 UI 和验证 | ~300 行 |
| `api.ts` | Tauri Commands 封装 | ~80 行 |
| `types.ts` | 类型定义和常量 | ~40 行 |
| `index.css` | 全局样式和主题 | ~150 行 |

---

## 3. 组件详解

### 3.1 App.tsx - 主应用组件

#### 3.1.1 组件结构

```tsx
import { useEffect, useState } from "react";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import DependencyFrame from "./components/DependencyFrame";
import ConfigPanel from "./components/ConfigPanel";
import { api } from "./api";
import { AppConfig, DEFAULT_CONFIG } from "./types";

function App() {
  // 状态管理
  const [mode, setMode] = useState<'claude' | 'custom'>('claude');
  const [proxy, setProxy] = useState('');
  const [model, setModel] = useState('qwen3-coder-480b-a35b');
  const [baseUrl, setBaseUrl] = useState('http://litellm.uattest.weoa.com');
  const [token, setToken] = useState('');
  const [copySuccess, setCopySuccess] = useState(false);

  // 生命周期和事件处理
  useEffect(() => { /* ... */ }, []);

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 p-6">
      {/* 组件布局 */}
    </div>
  );
}
```

#### 3.1.2 状态管理

**状态列表**:

| 状态 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `mode` | `'claude' \| 'custom'` | `'claude'` | 工作模式 |
| `proxy` | `string` | `''` | 代理地址 |
| `model` | `string` | `'qwen3-coder-480b-a35b'` | 自定义模型名称 |
| `baseUrl` | `string` | `'http://...'` | 自定义 API 地址 |
| `token` | `string` | `''` | 认证令牌 |
| `copySuccess` | `boolean` | `false` | 复制成功提示 |

**状态同步**:

```tsx
// 启动时加载配置
useEffect(() => {
  loadConfig();
}, []);

// 关闭前保存配置
useEffect(() => {
  const handleBeforeUnload = () => {
    saveConfig();
  };
  window.addEventListener('beforeunload', handleBeforeUnload);
  return () => {
    window.removeEventListener('beforeunload', handleBeforeUnload);
    saveConfig(); // 组件卸载时也保存
  };
}, [mode, proxy, model, baseUrl, token]);
```

#### 3.1.3 核心功能函数

**1. 加载配置**

```tsx
const loadConfig = async () => {
  try {
    const config: AppConfig = await api.loadAppConfig();
    setMode(config.mode);
    setProxy(config.proxy);
    setModel(config.model);
    setBaseUrl(config.base_url);
    setToken(config.token);
  } catch (error) {
    console.error('加载配置失败:', error);
    // 使用默认配置
  }
};
```

**2. 保存配置**

```tsx
const saveConfig = async () => {
  try {
    const config: AppConfig = {
      mode,
      proxy,
      model,
      base_url: baseUrl,
      token,
    };
    await api.saveAppConfig(config);
  } catch (error) {
    console.error('保存配置失败:', error);
  }
};
```

**3. 配置验证**

```tsx
const validateConfig = (): string | null => {
  if (mode === 'claude') {
    // Claude 模式：验证代理地址
    if (proxy && !proxy.startsWith('http://') && !proxy.startsWith('https://')) {
      return '代理地址必须以 http:// 或 https:// 开头';
    }
  } else {
    // 自定义模式：验证必填字段
    if (!model.trim()) {
      return '请输入模型名称';
    }
    if (!baseUrl.trim()) {
      return '请输入 Base URL';
    }
    if (!baseUrl.startsWith('http://') && !baseUrl.startsWith('https://')) {
      return 'Base URL 必须以 http:// 或 https:// 开头';
    }
  }
  return null;
};
```

**4. 获取环境变量配置**

```tsx
const getConfig = (): Record<string, string> => {
  const config: Record<string, string> = {};

  if (mode === 'claude') {
    // Claude 原版模式
    if (proxy) {
      config['HTTP_PROXY'] = proxy;
      config['HTTPS_PROXY'] = proxy;
    }
  } else {
    // 自定义模型模式
    config['ANTHROPIC_MODEL'] = model;
    config['ANTHROPIC_BASE_URL'] = baseUrl;
    if (token) {
      config['ANTHROPIC_AUTH_TOKEN'] = token;
    }
  }

  return config;
};
```

**5. 启动 Claude Code**

```tsx
const handleLaunch = async () => {
  const error = validateConfig();
  if (error) {
    alert(error);
    return;
  }

  try {
    const config = getConfig();
    await api.launchClaudeCode(config);
    alert('Claude Code 已启动！');
  } catch (error) {
    alert(`启动失败: ${error}`);
  }
};
```

**6. 生成和复制命令**

```tsx
const handleCopyCommand = async (type: 'powershell' | 'cmd') => {
  const error = validateConfig();
  if (error) {
    alert(error);
    return;
  }

  try {
    const config = getConfig();
    const command = type === 'powershell'
      ? await api.generatePowershellCommand(config)
      : await api.generateCmdCommand(config);

    await writeText(command);
    setCopySuccess(true);
    setTimeout(() => setCopySuccess(false), 2000);
  } catch (error) {
    alert(`生成命令失败: ${error}`);
  }
};
```

**7. 保存到 Claude 设置**

```tsx
const handleSaveToSettings = async () => {
  const error = validateConfig();
  if (error) {
    alert(error);
    return;
  }

  try {
    const config = getConfig();
    await api.saveToSettings(config);
    alert('配置已保存到 Claude 设置！');
  } catch (error) {
    alert(`保存失败: ${error}`);
  }
};
```

**8. 重置设置**

```tsx
const handleResetSettings = async () => {
  if (!confirm('确定要重置 Claude 设置中的环境变量配置吗？')) {
    return;
  }

  try {
    await api.resetSettings();
    alert('设置已重置！');
  } catch (error) {
    alert(`重置失败: ${error}`);
  }
};
```

**9. 打开设置文件**

```tsx
const handleOpenSettingsFile = async () => {
  try {
    await api.openSettingsFile();
  } catch (error) {
    alert(`打开设置文件失败: ${error}`);
  }
};
```

#### 3.1.4 布局结构

```tsx
<div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 p-6">
  <div className="max-w-4xl mx-auto space-y-6">
    {/* 标题 */}
    <h1 className="text-4xl font-bold text-center text-white mb-8">
      Claude Code 启动器
    </h1>

    {/* 依赖检测面板 */}
    <DependencyFrame />

    {/* 配置面板 */}
    <ConfigPanel
      mode={mode}
      setMode={setMode}
      proxy={proxy}
      setProxy={setProxy}
      model={model}
      setModel={setModel}
      baseUrl={baseUrl}
      setBaseUrl={setBaseUrl}
      token={token}
      setToken={setToken}
    />

    {/* 操作按钮组 */}
    <div className="card space-y-4">
      {/* 启动按钮 */}
      <button onClick={handleLaunch} className="btn-launch">
        🚀 启动 Claude Code
      </button>

      {/* 命令生成按钮 */}
      <div className="grid grid-cols-2 gap-4">
        <button onClick={() => handleCopyCommand('powershell')}>
          📋 复制 PowerShell 命令
        </button>
        <button onClick={() => handleCopyCommand('cmd')}>
          📋 复制 CMD 命令
        </button>
      </div>

      {/* 设置管理按钮 */}
      <div className="grid grid-cols-3 gap-4">
        <button onClick={handleSaveToSettings}>💾 保存到 Claude 设置</button>
        <button onClick={handleResetSettings}>🔄 重置设置</button>
        <button onClick={handleOpenSettingsFile}>📂 打开设置文件</button>
      </div>

      {/* 复制成功提示 */}
      {copySuccess && (
        <div className="text-center text-green-400">✓ 已复制到剪贴板</div>
      )}
    </div>
  </div>
</div>
```

---

### 3.2 DependencyFrame.tsx - 依赖检测组件

#### 3.2.1 组件结构

```tsx
import { useEffect, useState } from "react";
import { api } from "../api";
import { DependencyStatus } from "../types";

export default function DependencyFrame() {
  // 状态管理
  const [nodejsStatus, setNodejsStatus] = useState<DependencyStatus | null>(null);
  const [claudeStatus, setClaudeStatus] = useState<DependencyStatus | null>(null);
  const [loading, setLoading] = useState<string | null>(null);

  // 自动检测依赖
  useEffect(() => {
    const timer = setTimeout(() => {
      checkDependencies();
    }, 100);
    return () => clearTimeout(timer);
  }, []);

  // 功能函数
  const checkDependencies = async () => { /* ... */ };
  const checkUpdates = async () => { /* ... */ };
  const handleInstallOrUpdate = async (dep: 'nodejs' | 'claude') => { /* ... */ };

  return (
    <div className="card">
      {/* 组件内容 */}
    </div>
  );
}
```

#### 3.2.2 状态管理

| 状态 | 类型 | 说明 |
|------|------|------|
| `nodejsStatus` | `DependencyStatus \| null` | Node.js 依赖状态 |
| `claudeStatus` | `DependencyStatus \| null` | Claude Code 依赖状态 |
| `loading` | `string \| null` | 加载状态标识 |

**DependencyStatus 结构**:
```typescript
interface DependencyStatus {
  installed: boolean;           // 是否已安装
  version: string | null;        // 当前版本
  meets_requirement: boolean;    // 是否满足版本要求
  latest_version: string | null; // 最新版本
  update_available: boolean;     // 是否有更新
  error: string | null;          // 错误信息
}
```

#### 3.2.3 核心功能

**1. 检测依赖**

```tsx
const checkDependencies = async () => {
  setLoading('checking');
  try {
    const [nodejs, claude] = await Promise.all([
      api.checkNodejs(),
      api.checkClaude(),
    ]);
    setNodejsStatus(nodejs);
    setClaudeStatus(claude);
  } catch (error) {
    console.error('检测依赖失败:', error);
  } finally {
    setLoading(null);
  }
};
```

**2. 检查更新**

```tsx
const checkUpdates = async () => {
  setLoading('checking-updates');
  try {
    const [nodejs, claude] = await Promise.all([
      api.checkNodejsWithUpdate(),
      api.checkClaudeWithUpdate(),
    ]);
    setNodejsStatus(nodejs);
    setClaudeStatus(claude);
  } catch (error) {
    console.error('检查更新失败:', error);
  } finally {
    setLoading(null);
  }
};
```

**3. 安装/更新依赖**

```tsx
const handleInstallOrUpdate = async (dep: 'nodejs' | 'claude') => {
  const status = dep === 'nodejs' ? nodejsStatus : claudeStatus;
  if (!status) return;

  setLoading(`${dep}-install`);
  try {
    if (status.installed && status.update_available) {
      // 更新
      if (dep === 'nodejs') {
        await api.updateNodejs();
      } else {
        await api.updateClaude();
      }
      alert(`${dep === 'nodejs' ? 'Node.js' : 'Claude Code'} 更新完成！`);
    } else {
      // 安装
      if (dep === 'nodejs') {
        await api.installNodejs();
      } else {
        await api.installClaude();
      }
      alert(`${dep === 'nodejs' ? 'Node.js' : 'Claude Code'} 安装完成！`);
    }

    // 刷新系统 PATH
    await api.refreshSystemPath();

    // 重新检测
    await checkDependencies();
  } catch (error) {
    alert(`操作失败: ${error}`);
  } finally {
    setLoading(null);
  }
};
```

#### 3.2.4 状态渲染逻辑

```tsx
const renderStatus = (status: DependencyStatus | null) => {
  if (!status) {
    return <span className="text-gray-400">⏳ 检测中...</span>;
  }

  if (status.error) {
    return <span className="text-error">✗ {status.error}</span>;
  }

  if (!status.installed) {
    return <span className="text-error">✗ 未安装</span>;
  }

  if (!status.meets_requirement) {
    return (
      <span className="text-warning">
        ⚠ 版本过低 (当前: {status.version}, 需要: ≥18.0.0)
      </span>
    );
  }

  if (status.update_available) {
    return (
      <span className="text-warning">
        ⚠ 已安装 {status.version} (有更新: {status.latest_version})
      </span>
    );
  }

  return (
    <span className="text-success">
      ✓ 已安装 {status.version} (最新版本)
    </span>
  );
};
```

#### 3.2.5 UI 布局

```tsx
<div className="card">
  <h2 className="text-2xl font-bold text-white mb-4">📦 依赖检测</h2>

  <div className="space-y-4">
    {/* Node.js 状态 */}
    <div className="flex items-center justify-between">
      <div>
        <span className="text-lg font-semibold text-white">Node.js:</span>
        <div className="mt-1">{renderStatus(nodejsStatus)}</div>
      </div>
      {nodejsStatus && (!nodejsStatus.installed || nodejsStatus.update_available) && (
        <button
          onClick={() => handleInstallOrUpdate('nodejs')}
          disabled={loading === 'nodejs-install'}
          className="btn-primary"
        >
          {loading === 'nodejs-install'
            ? '处理中...'
            : nodejsStatus.installed ? '更新' : '安装'}
        </button>
      )}
    </div>

    {/* Claude Code 状态 */}
    <div className="flex items-center justify-between">
      <div>
        <span className="text-lg font-semibold text-white">Claude Code:</span>
        <div className="mt-1">{renderStatus(claudeStatus)}</div>
      </div>
      {claudeStatus && (!claudeStatus.installed || claudeStatus.update_available) && (
        <button
          onClick={() => handleInstallOrUpdate('claude')}
          disabled={loading === 'claude-install'}
          className="btn-primary"
        >
          {loading === 'claude-install'
            ? '处理中...'
            : claudeStatus.installed ? '更新' : '安装'}
        </button>
      )}
    </div>
  </div>

  {/* 操作按钮 */}
  <div className="mt-6 flex gap-4">
    <button
      onClick={checkDependencies}
      disabled={loading === 'checking'}
      className="btn-secondary flex-1"
    >
      {loading === 'checking' ? '检测中...' : '🔄 重新检测'}
    </button>
    <button
      onClick={checkUpdates}
      disabled={loading === 'checking-updates'}
      className="btn-secondary flex-1"
    >
      {loading === 'checking-updates' ? '检查中...' : '🔍 检查更新'}
    </button>
  </div>
</div>
```

---

### 3.3 ConfigPanel.tsx - 配置面板组件

#### 3.3.1 组件 Props

```tsx
interface ConfigPanelProps {
  mode: 'claude' | 'custom';
  onModeChange: (mode: 'claude' | 'custom') => void;
  proxy: string;
  onProxyChange: (value: string) => void;
  model: string;
  onModelChange: (value: string) => void;
  baseUrl: string;
  onBaseUrlChange: (value: string) => void;
  token: string;
  onTokenChange: (value: string) => void;
  skipPermissions: boolean;
  onSkipPermissionsChange: (value: boolean) => void;
  onLaunch: () => void;
  onCopyPowershell: () => void;
  onCopyCmd: () => void;
  onCopyBash: () => void;
  copySuccess: boolean;
  platform: 'windows' | 'macos' | 'linux' | 'unknown';
}
```

**新增 Props 说明**:

| Prop | 类型 | 说明 |
|------|------|------|
| `skipPermissions` | `boolean` | 是否跳过权限确认 |
| `onSkipPermissionsChange` | `(value: boolean) => void` | 切换跳过权限回调 |
| `onCopyBash` | `() => void` | 复制 Bash 命令回调 |
| `platform` | `string` | 当前操作系统平台 |

**平台适配**: 根据 `platform` 值显示不同的命令复制按钮:
- Windows: 显示 PowerShell 和 CMD 按钮
- macOS/Linux: 显示 Bash 按钮

#### 3.3.2 内部状态

```tsx
const [showToken, setShowToken] = useState(false);
const [isCustomModel, setIsCustomModel] = useState(false);
```

| 状态 | 类型 | 说明 |
|------|------|------|
| `showToken` | `boolean` | 是否显示 Token 明文 |
| `isCustomModel` | `boolean` | 是否使用自定义模型名称 |

#### 3.3.3 模型选项

```tsx
const MODEL_OPTIONS = [
  'deepseek-v3',
  'qwen3-235b-a22b',
  'qwen3-coder-480b-a35b',
];

// 检查当前模型是否在预设列表中
useEffect(() => {
  setIsCustomModel(!MODEL_OPTIONS.includes(model));
}, [model]);
```

#### 3.3.4 UI 布局

**模式切换**:

```tsx
<div className="card">
  <h2 className="text-2xl font-bold text-white mb-4">⚙️ 配置参数</h2>

  {/* 模式选择 */}
  <div className="mb-6">
    <label className="text-white font-semibold mb-2 block">工作模式</label>
    <div className="flex gap-4">
      <button
        onClick={() => setMode('claude')}
        className={`flex-1 py-3 px-4 rounded-lg font-semibold transition-all ${
          mode === 'claude'
            ? 'bg-primary text-white shadow-lg'
            : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
        }`}
      >
        🌐 Claude 原版
      </button>
      <button
        onClick={() => setMode('custom')}
        className={`flex-1 py-3 px-4 rounded-lg font-semibold transition-all ${
          mode === 'custom'
            ? 'bg-primary text-white shadow-lg'
            : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
        }`}
      >
        🔧 自定义模型
      </button>
    </div>
  </div>

  {/* 根据模式显示不同表单 */}
  {mode === 'claude' ? renderClaudeMode() : renderCustomMode()}
</div>
```

**Claude 原版模式表单**:

```tsx
const renderClaudeMode = () => (
  <div className="space-y-4">
    <div>
      <label className="text-white font-semibold mb-2 block">
        代理地址 (可选)
      </label>
      <input
        type="text"
        value={proxy}
        onChange={(e) => setProxy(e.target.value)}
        placeholder="http://127.0.0.1:7890"
        className="input-field"
      />
      <p className="text-sm text-gray-400 mt-1">
        用于访问 Claude 官方服务，留空则不使用代理
      </p>
    </div>
  </div>
);
```

**自定义模型模式表单**:

```tsx
const renderCustomMode = () => (
  <div className="space-y-4">
    {/* Model Name */}
    <div>
      <label className="text-white font-semibold mb-2 block">
        Model Name <span className="text-error">*</span>
      </label>

      {/* 预设模型选择 */}
      {!isCustomModel && (
        <select
          value={model}
          onChange={(e) => setModel(e.target.value)}
          className="input-field mb-2"
        >
          {MODEL_OPTIONS.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
      )}

      {/* 自定义模型输入 */}
      {isCustomModel && (
        <input
          type="text"
          value={model}
          onChange={(e) => setModel(e.target.value)}
          placeholder="输入模型名称"
          className="input-field mb-2"
        />
      )}

      {/* 切换按钮 */}
      <button
        onClick={() => {
          if (isCustomModel) {
            setModel(MODEL_OPTIONS[0]);
          } else {
            setModel('');
          }
          setIsCustomModel(!isCustomModel);
        }}
        className="text-sm text-primary hover:underline"
      >
        {isCustomModel ? '← 使用预设模型' : '→ 使用自定义模型'}
      </button>
    </div>

    {/* Base URL */}
    <div>
      <label className="text-white font-semibold mb-2 block">
        Base URL <span className="text-error">*</span>
      </label>
      <input
        type="text"
        value={baseUrl}
        onChange={(e) => setBaseUrl(e.target.value)}
        placeholder="http://api.example.com"
        className="input-field"
      />
    </div>

    {/* Auth Token */}
    <div>
      <label className="text-white font-semibold mb-2 block">
        Auth Token (可选)
      </label>
      <div className="relative">
        <input
          type={showToken ? "text" : "password"}
          value={token}
          onChange={(e) => setToken(e.target.value)}
          placeholder="输入认证令牌"
          className="input-field pr-12"
        />
        <button
          onClick={() => setShowToken(!showToken)}
          className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-white"
        >
          {showToken ? '🙈' : '👁️'}
        </button>
      </div>
    </div>
  </div>
);
```

---

## 4. 状态管理

### 4.1 状态架构

```
App (根组件)
├── mode (工作模式)
├── proxy (代理地址)
├── model (模型名称)
├── baseUrl (API 地址)
├── token (认证令牌)
└── copySuccess (复制提示)

DependencyFrame (依赖检测)
├── nodejsStatus (Node.js 状态)
├── claudeStatus (Claude Code 状态)
└── loading (加载状态)

ConfigPanel (配置面板)
├── showToken (显示密码)
└── isCustomModel (自定义模型)
```

### 4.2 状态提升模式

**ConfigPanel** 组件不持有配置状态，而是通过 Props 接收状态和更新函数：

```tsx
// App.tsx
<ConfigPanel
  mode={mode}
  setMode={setMode}
  proxy={proxy}
  setProxy={setProxy}
  // ... 其他 props
/>

// ConfigPanel.tsx
export default function ConfigPanel({
  mode,
  setMode,
  proxy,
  setProxy,
  // ... 其他 props
}: ConfigPanelProps) {
  // 组件不持有这些状态，直接使用 props
  return (
    <input value={proxy} onChange={(e) => setProxy(e.target.value)} />
  );
}
```

**优势**:
- ✅ 单一数据源（Single Source of Truth）
- ✅ 配置状态在 App 组件统一管理
- ✅ 便于实现配置持久化
- ✅ 组件间数据共享简单

### 4.3 副作用管理

**配置持久化副作用**:

```tsx
useEffect(() => {
  const handleBeforeUnload = () => {
    saveConfig();
  };

  window.addEventListener('beforeunload', handleBeforeUnload);

  return () => {
    window.removeEventListener('beforeunload', handleBeforeUnload);
    saveConfig(); // 清理时也保存
  };
}, [mode, proxy, model, baseUrl, token]); // 依赖所有配置状态
```

**自动检测副作用**:

```tsx
useEffect(() => {
  const timer = setTimeout(() => {
    checkDependencies();
  }, 100); // 延迟 100ms 避免闪烁

  return () => clearTimeout(timer); // 清理定时器
}, []); // 仅挂载时执行
```

---

## 5. API 调用

### 5.1 API 封装 (api.ts)

```typescript
import { invoke } from "@tauri-apps/api/core";
import { AppConfig, DependencyStatus } from "./types";

export const api = {
  // 依赖检测
  checkNodejs: () => invoke<DependencyStatus>('check_nodejs'),
  checkClaude: () => invoke<DependencyStatus>('check_claude'),
  checkGitbash: () => invoke<DependencyStatus>('check_gitbash'),
  checkNodejsWithUpdate: () =>
    invoke<DependencyStatus>('check_nodejs_with_update'),
  checkClaudeWithUpdate: () =>
    invoke<DependencyStatus>('check_claude_with_update'),
  checkGitbashWithUpdate: () =>
    invoke<DependencyStatus>('check_gitbash_with_update'),
  refreshSystemPath: () => invoke('refresh_system_path'),

  // 安装/更新
  installNodejs: () => invoke('install_nodejs'),
  updateNodejs: () => invoke('update_nodejs'),
  installClaude: () => invoke('install_claude'),
  updateClaude: () => invoke('update_claude'),
  installGitbash: () => invoke('install_gitbash'),
  updateGitbash: () => invoke('update_gitbash'),

  // 启动
  launchClaudeCode: (config: Record<string, string>) =>
    invoke('launch_claude_code', { config }),

  // 命令生成
  generatePowershellCommand: (config: Record<string, string>) =>
    invoke<string>('generate_powershell_command', { config }),
  generateCmdCommand: (config: Record<string, string>) =>
    invoke<string>('generate_cmd_command', { config }),
  generateBashCommand: (config: Record<string, string>) =>
    invoke<string>('generate_bash_command', { config }),

  // 平台检测
  getPlatform: () => invoke<string>('get_platform'),

  // 设置管理
  saveToSettings: (config: Record<string, string>) =>
    invoke('save_to_settings', { config }),
  resetSettings: () => invoke('reset_settings'),
  openSettingsFile: () => invoke('open_settings_file'),

  // 应用配置
  saveAppConfig: (config: AppConfig) =>
    invoke('save_app_config', { config }),
  loadAppConfig: () => invoke<AppConfig>('load_app_config'),
};
```

**新增 API 说明**:

| API | 说明 |
|-----|------|
| `checkGitbash()` | 检测 Git Bash 安装状态 |
| `checkGitbashWithUpdate()` | 检测 Git Bash 并获取最新版本 |
| `installGitbash()` | 安装 Git Bash |
| `updateGitbash()` | 更新 Git Bash |
| `generateBashCommand()` | 生成 Bash 格式命令 |
| `getPlatform()` | 获取当前操作系统平台 |

### 5.2 错误处理模式

**基础错误处理**:

```tsx
try {
  const result = await api.someCommand();
  // 成功处理
} catch (error) {
  console.error('操作失败:', error);
  alert(`操作失败: ${error}`);
}
```

**带加载状态的错误处理**:

```tsx
setLoading('some-operation');
try {
  const result = await api.someCommand();
  // 成功处理
} catch (error) {
  alert(`操作失败: ${error}`);
} finally {
  setLoading(null); // 确保加载状态被清除
}
```

**并发请求错误处理**:

```tsx
try {
  const [result1, result2] = await Promise.all([
    api.command1(),
    api.command2(),
  ]);
  // 两个请求都成功
} catch (error) {
  // 任一请求失败都会进入这里
  console.error('操作失败:', error);
}
```

### 5.3 类型安全

**类型推断**:

```tsx
// ✅ 正确：TypeScript 推断返回类型
const status: DependencyStatus = await api.checkNodejs();

// ❌ 错误：类型不匹配会编译报错
const status: string = await api.checkNodejs(); // 编译错误
```

**泛型支持**:

```tsx
// invoke 函数支持泛型指定返回类型
invoke<DependencyStatus>('check_nodejs');  // 返回 Promise<DependencyStatus>
invoke<string>('generate_powershell_command', { config });  // 返回 Promise<string>
invoke('refresh_system_path');  // 返回 Promise<void>
```

---

## 6. 类型系统

### 6.1 类型定义 (types.ts)

```typescript
// 依赖状态
export interface DependencyStatus {
  installed: boolean;           // 是否已安装
  version: string | null;        // 当前版本号
  meets_requirement: boolean;    // 是否满足最低版本要求
  latest_version: string | null; // 最新可用版本
  update_available: boolean;     // 是否有可用更新
  error: string | null;          // 错误信息
}

// 应用配置
export interface AppConfig {
  mode: 'claude' | 'custom';  // 工作模式
  proxy: string;              // 代理地址
  model: string;              // 模型名称
  base_url: string;           // API Base URL
  token: string;              // 认证令牌
  skip_permissions: boolean;  // 是否跳过权限确认
}

// 默认配置
export const DEFAULT_CONFIG: AppConfig = {
  mode: 'claude',
  proxy: '',
  model: 'qwen3-coder-480b-a35b',
  base_url: 'http://litellm.uattest.weoa.com',
  token: '',
  skip_permissions: true,     // 默认启用跳过权限
};

// 模型选项
export const MODEL_OPTIONS = [
  'deepseek-v3',
  'qwen3-235b-a22b',
  'qwen3-coder-480b-a35b',
];
```

**`skip_permissions` 说明**:
- `true`: 启动时添加 `--dangerously-skip-permissions` 参数
- `false`: 普通模式，需要权限确认
- 配合 UI 中的启动模式选择使用

### 6.2 类型使用示例

**组件 Props 类型**:

```tsx
interface ConfigPanelProps {
  mode: 'claude' | 'custom';
  setMode: (mode: 'claude' | 'custom') => void;
  proxy: string;
  setProxy: (proxy: string) => void;
  // ...
}

export default function ConfigPanel(props: ConfigPanelProps) {
  // TypeScript 确保 props 类型正确
}
```

**状态类型注解**:

```tsx
const [mode, setMode] = useState<'claude' | 'custom'>('claude');
const [status, setStatus] = useState<DependencyStatus | null>(null);
```

**函数返回类型**:

```tsx
const getConfig = (): Record<string, string> => {
  // 返回类型明确，增强代码可读性
  const config: Record<string, string> = {};
  // ...
  return config;
};
```

---

## 7. 样式设计

### 7.1 Tailwind CSS 主题

**自定义颜色**:

```javascript
// tailwind.config.js
export default {
  theme: {
    extend: {
      colors: {
        primary: '#007ACC',    // 主色（蓝色）
        success: '#5a7c5c',    // 成功（绿色）
        error: '#8b5a5a',      // 错误（红色）
        warning: '#FF9800',    // 警告（橙色）
      },
    },
  },
}
```

**自定义字体**:

```javascript
fontFamily: {
  sans: ['Microsoft YaHei', 'sans-serif'],  // 微软雅黑
}
```

### 7.2 全局样式 (index.css)

**Tailwind 指令**:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

**自定义组件类**:

```css
/* 卡片容器 */
.card {
  @apply bg-gray-800 rounded-xl p-6 shadow-lg;
  @apply border border-gray-700;
  @apply hover:shadow-xl transition-all duration-300;
}

/* 输入框 */
.input-field {
  @apply w-full px-4 py-3 rounded-lg;
  @apply bg-gray-700 text-white;
  @apply border border-gray-600;
  @apply focus:border-primary focus:ring-2 focus:ring-primary/50;
  @apply transition-all duration-200;
}

/* 主按钮 */
.btn-primary {
  @apply px-6 py-3 rounded-lg font-semibold;
  @apply bg-primary text-white;
  @apply hover:bg-blue-600 active:scale-95;
  @apply disabled:opacity-50 disabled:cursor-not-allowed;
  @apply transition-all duration-200;
}

/* 次要按钮 */
.btn-secondary {
  @apply px-6 py-3 rounded-lg font-semibold;
  @apply bg-gray-700 text-gray-300;
  @apply hover:bg-gray-600 active:scale-95;
  @apply disabled:opacity-50 disabled:cursor-not-allowed;
  @apply transition-all duration-200;
}

/* 启动按钮 */
.btn-launch {
  @apply w-full py-4 rounded-xl text-xl font-bold;
  @apply bg-gradient-to-r from-primary to-blue-600;
  @apply text-white shadow-lg;
  @apply hover:shadow-2xl hover:scale-105;
  @apply active:scale-95;
  @apply transition-all duration-300;
}
```

**自定义滚动条**:

```css
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  @apply bg-gray-800;
}

::-webkit-scrollbar-thumb {
  @apply bg-gray-600 rounded-full;
  @apply hover:bg-gray-500;
}
```

### 7.3 响应式设计

**容器最大宽度**:

```tsx
<div className="max-w-4xl mx-auto">
  {/* 内容居中，最大宽度 4xl */}
</div>
```

**网格布局**:

```tsx
<div className="grid grid-cols-2 gap-4">
  {/* 两列等宽布局 */}
</div>

<div className="grid grid-cols-3 gap-4">
  {/* 三列等宽布局 */}
</div>
```

**间距控制**:

```tsx
<div className="space-y-4">
  {/* 垂直方向子元素间距 1rem */}
</div>

<div className="space-y-6">
  {/* 垂直方向子元素间距 1.5rem */}
</div>
```

### 7.4 动画效果

**过渡动画**:

```css
/* 通用过渡 */
.transition-all {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}

/* 自定义持续时间 */
.duration-200 { transition-duration: 200ms; }
.duration-300 { transition-duration: 300ms; }
```

**缩放动画**:

```tsx
<button className="hover:scale-105 active:scale-95">
  {/* 鼠标悬停时放大 5%，点击时缩小 5% */}
</button>
```

**阴影动画**:

```tsx
<div className="shadow-lg hover:shadow-2xl">
  {/* 鼠标悬停时阴影增强 */}
</div>
```

---

## 8. 开发实践

### 8.1 最佳实践

**1. 组件拆分原则**:
- ✅ 单一职责：每个组件只负责一个功能
- ✅ 可复用：通过 Props 传递配置，避免硬编码
- ✅ 小而精：组件代码控制在 300 行以内

**2. 状态管理原则**:
- ✅ 状态提升：共享状态放在最近的公共父组件
- ✅ 最小化状态：能通过计算得出的数据不要存为状态
- ✅ 不可变更新：使用新对象替代，不要直接修改状态

**3. 类型安全原则**:
- ✅ 显式类型注解：复杂类型要明确声明
- ✅ 避免 `any`：使用具体类型或 `unknown`
- ✅ 接口优先：定义清晰的接口契约

**4. 性能优化原则**:
- ✅ 合理使用 `useEffect` 依赖
- ✅ 避免不必要的重新渲染
- ✅ 大型列表使用虚拟化

### 8.2 常见模式

**条件渲染**:

```tsx
{mode === 'claude' ? (
  <ClaudeModeForm />
) : (
  <CustomModeForm />
)}

{status && status.update_available && (
  <button>更新</button>
)}
```

**列表渲染**:

```tsx
{MODEL_OPTIONS.map((option) => (
  <option key={option} value={option}>
    {option}
  </option>
))}
```

**事件处理**:

```tsx
// 内联箭头函数
<button onClick={() => handleClick(param)}>

// 直接传递函数引用
<button onClick={handleClick}>

// 事件对象
<input onChange={(e) => setValue(e.target.value)} />
```

### 8.3 调试技巧

**1. 控制台日志**:

```tsx
console.log('状态:', { mode, proxy, model });
console.error('错误:', error);
```

**2. React DevTools**:
- 查看组件树
- 检查 Props 和 State
- 分析渲染性能

**3. Tauri DevTools**:
```bash
npm run tauri dev
# 打开应用后按 F12 或 Ctrl+Shift+I
```

**4. TypeScript 类型检查**:

```bash
# 编译检查
npm run build

# 或使用 VSCode 实时检查
```

### 8.4 代码规范

**命名约定**:
- 组件：PascalCase (`DependencyFrame`)
- 函数：camelCase (`handleClick`)
- 常量：UPPER_SNAKE_CASE (`MODEL_OPTIONS`)
- 文件：kebab-case 或 PascalCase

**导入顺序**:
```tsx
// 1. React 相关
import { useEffect, useState } from "react";

// 2. 第三方库
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

// 3. 本地组件
import DependencyFrame from "./components/DependencyFrame";

// 4. 本地模块
import { api } from "./api";
import { AppConfig } from "./types";
```

**注释规范**:
```tsx
// 单行注释：说明代码意图

/**
 * 多行注释：函数文档
 * @param config 配置对象
 * @returns 命令字符串
 */
```

---

## 9. 总结

### 9.1 前端架构特点

- 🎯 **组件化**: 清晰的组件职责划分
- 🔄 **单向数据流**: Props Down, Events Up
- 🛡️ **类型安全**: 全面的 TypeScript 类型
- 🎨 **现代 UI**: Tailwind CSS 实用优先
- ⚡ **高性能**: Vite 快速开发和构建
- 🖥️ **跨平台**: 根据平台动态调整 UI

### 9.2 技术亮点

- ✨ React 19 最新特性
- ✨ Tauri IPC 高效通信 (34 个 API)
- ✨ 响应式设计和动画
- ✨ 完善的错误处理
- ✨ 优雅的状态管理
- ✨ 平台检测和 UI 适配
- ✨ 多 Shell 命令生成 (PowerShell/CMD/Bash)
- ✨ 多项目管理支持
- ✨ 跳过权限确认模式

### 9.3 API 统计

| 分类 | 函数数量 |
|------|----------|
| 依赖检测 | 7 |
| 安装/更新 | 6 |
| 启动器 | 4 |
| 平台/工具 | 2 |
| 设置管理 | 3 |
| 应用配置 | 2 |
| 项目管理 | 10 |
| **总计** | **34** |

### 9.4 后续优化方向

- 🔮 添加更多自定义主题
- 🔮 支持配置导入/导出
- 🔮 添加更多模型预设
- 🔮 优化加载状态显示
- 🔮 添加配置验证提示
- 🔮 支持 Linux 平台

---

**相关文档**:
- [项目总览](./PROJECT_DOCUMENTATION.md)
- [后端开发指南](./BACKEND_GUIDE.md)
- [API 参考](./API_REFERENCE.md)
