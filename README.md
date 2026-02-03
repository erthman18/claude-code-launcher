# Claude Code 启动器

一个用于管理和启动 Claude Code 项目的桌面应用程序，基于 Tauri + React + TypeScript 构建。

## 功能特性

### 项目管理
- **多项目支持**：创建和管理多个项目配置
- **默认项目**：系统自带一个默认项目，使用用户主目录作为工作目录
- **项目配置**：每个项目可独立配置启动模式、代理、模型等参数

### 拖拽排序与置顶
- **拖拽排序**：通过拖拽调整项目在列表中的顺序
- **置顶功能**：在编辑页面可将项目设为置顶，置顶项目显示在默认项目之后、普通项目之前
- **排序优先级**：
  1. 默认项目 - 固定第一位，不可拖拽
  2. 置顶项目 - 按置顶时间倒序排列，可在置顶区域内拖拽互换
  3. 普通项目 - 按自定义顺序排列，可在普通区域内拖拽互换

### 启动模式
- **Claude 原版模式**：使用 Anthropic 官方服务，支持配置代理
- **自定义模型模式**：支持配置自定义 API 端点、模型名称和认证令牌
- **dangerously-skip 模式**：跳过权限确认提示，适合自动化场景

### 新手引导
- **首次使用引导**：首次打开应用时自动显示分步引导
- **功能高亮**：逐步高亮关键功能区域，介绍应用功能
- **随时查看**：右下角帮助按钮可随时重新查看引导

### 其他功能
- **依赖检测**：自动检测 Node.js、Claude CLI、Git Bash 等依赖
- **一键安装**：支持一键安装/更新缺失的依赖
- **命令复制**：生成并复制 PowerShell/CMD/Bash 启动命令
- **文件夹拖拽**：拖拽文件夹到窗口快速创建项目

## 技术栈

- **前端**：React 19 + TypeScript + Tailwind CSS
- **后端**：Rust + Tauri 2
- **拖拽库**：@dnd-kit
- **剪贴板**：@tauri-apps/plugin-clipboard-manager (macOS 必需)

## 开发

### 环境要求
- Node.js 18+
- Rust 1.70+
- pnpm 或 npm

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri:dev
```

### 构建

```bash
npm run tauri:build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录。

## 配置文件

配置文件存储在系统配置目录：
- Windows: `%APPDATA%\ClaudeCodeLauncher\config.json`
- macOS: `~/Library/Application Support/ClaudeCodeLauncher/config.json`
- Linux: `~/.config/ClaudeCodeLauncher/config.json`

## 平台支持

| 功能 | Windows | macOS |
|------|---------|-------|
| 依赖检测 | ✅ | ✅ (扩展 PATH) |
| 启动 Claude | ✅ PowerShell | ✅ Terminal.app |
| 复制命令 | ✅ | ✅ (Tauri 剪贴板 API) |
| 安装/更新 | ✅ winget | ✅ brew/npm |

### macOS 特殊处理

macOS GUI 应用不继承 shell 的 PATH 环境变量，因此：
- **依赖检测**：自动扫描常见安装路径（Homebrew、nvm、pnpm、Volta 等）
- **启动功能**：通过 Terminal.app 启动，Terminal 会加载完整 PATH
- **剪贴板**：使用 Tauri 剪贴板插件，而非浏览器 API

## 许可证

MIT
