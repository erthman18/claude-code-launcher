# 开发指南

> **从零开始的完整开发指南**
> **最后更新**: 2026-02-03
> **支持平台**: Windows 10/11, macOS 10.13+

---

## 📋 目录

- [1. 环境准备](#1-环境准备)
- [2. 项目初始化](#2-项目初始化)
- [3. 开发工作流](#3-开发工作流)
- [4. 调试技巧](#4-调试技巧)
- [5. 常见问题](#5-常见问题)
- [6. 最佳实践](#6-最佳实践)
- [7. CI/CD 自动化构建](#7-cicd-自动化构建)
- [8. 发布流程](#8-发布流程)
- [9. 总结](#9-总结)

---

## 1. 环境准备

### 1.1 系统要求

**操作系统**:
- ✅ Windows 10/11 (完全支持)
- ✅ macOS 10.13+ High Sierra (完全支持)
- ⚠️ Linux (需要适配)

**硬件要求**:
- CPU: 双核及以上
- 内存: 8GB 及以上
- 磁盘: 2GB 可用空间

---

### 1.2 必需工具

#### 1.2.1 Node.js

**版本要求**: ≥ 18.0.0

**安装方法**:

**Windows (winget)**:
```bash
winget install OpenJS.NodeJS.LTS
```

**Windows (手动下载)**:
- 下载地址: https://nodejs.org/
- 选择 LTS 版本
- 运行安装程序

**验证安装**:
```bash
node --version
# v20.10.0

npm --version
# 10.2.3
```

---

#### 1.2.2 Rust

**版本要求**: ≥ 1.75.0

**安装方法**:

**Windows**:
- 下载地址: https://rustup.rs/
- 运行 `rustup-init.exe`
- 按提示完成安装

**macOS/Linux**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**验证安装**:
```bash
rustc --version
# rustc 1.75.0

cargo --version
# cargo 1.75.0
```

**更新 Rust**:
```bash
rustup update
```

---

#### 1.2.3 Xcode Command Line Tools (macOS)

**说明**: macOS 上需要 Xcode 命令行工具

**安装方法**:
```bash
xcode-select --install
```

**验证安装**:
```bash
xcode-select -p
# /Library/Developer/CommandLineTools
```

---

#### 1.2.4 Homebrew (macOS 推荐)

**说明**: macOS 包管理器，用于安装依赖

**安装方法**:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

**验证安装**:
```bash
brew --version
# Homebrew 4.2.0
```

---

#### 1.2.5 Visual Studio C++ Build Tools (Windows)

**说明**: Rust 在 Windows 上需要 C++ 编译工具链

**安装方法**:

1. 下载 Visual Studio Build Tools:
   - https://visualstudio.microsoft.com/visual-cpp-build-tools/

2. 安装时选择:
   - ✅ Desktop development with C++
   - ✅ MSVC v142+ (或更高版本)
   - ✅ Windows 10/11 SDK

**或使用 winget**:
```bash
winget install Microsoft.VisualStudio.2022.BuildTools
```

---

#### 1.2.4 Git

**安装方法**:

**Windows (winget)**:
```bash
winget install Git.Git
```

**macOS**:
```bash
brew install git
```

**验证安装**:
```bash
git --version
# git version 2.40.0
```

---

### 1.3 推荐工具

#### 1.3.1 Visual Studio Code

**扩展推荐**:
- **Tauri**: tauri-apps.tauri-vscode
- **rust-analyzer**: rust-lang.rust-analyzer
- **Prettier**: esbenp.prettier-vscode
- **ESLint**: dbaeumer.vscode-eslint
- **Tailwind CSS IntelliSense**: bradlc.vscode-tailwindcss

**安装扩展**:
```bash
code --install-extension tauri-apps.tauri-vscode
code --install-extension rust-lang.rust-analyzer
code --install-extension esbenp.prettier-vscode
code --install-extension dbaeumer.vscode-eslint
code --install-extension bradlc.vscode-tailwindcss
```

---

#### 1.3.2 其他工具

**Windows Terminal** (推荐):
```bash
winget install Microsoft.WindowsTerminal
```

**PowerShell 7**:
```bash
winget install Microsoft.PowerShell
```

---

## 2. 项目初始化

### 2.1 克隆项目

```bash
cd D:\
git clone <repository-url> claude-code-launcher-tauri
cd claude-code-launcher-tauri
```

**或者从现有项目复制**:
```bash
# 假设项目已在 D:\claude-code-launcher-tauri
cd D:\claude-code-launcher-tauri
```

---

### 2.2 安装依赖

#### 2.2.1 安装前端依赖

```bash
npm install
```

**输出示例**:
```
added 123 packages in 15s
```

**依赖说明**:
- `@tauri-apps/api`: Tauri 前端 API
- `react`: UI 框架
- `vite`: 构建工具
- `tailwindcss`: CSS 框架

---

#### 2.2.2 检查 Rust 依赖

```bash
cd src-tauri
cargo check
```

**首次运行会下载并编译所有依赖**:
```
Updating crates.io index
Downloaded tauri v2.0.0
Downloaded serde v1.0.0
...
Compiling 150 crates
Finished dev [unoptimized + debuginfo] target(s) in 3m 45s
```

**返回项目根目录**:
```bash
cd ..
```

---

### 2.3 项目结构验证

```bash
# Windows
dir /s /b

# Linux/macOS
find . -type f
```

**关键文件检查**:
- ✅ `package.json`
- ✅ `vite.config.ts`
- ✅ `tailwind.config.js`
- ✅ `src-tauri/Cargo.toml`
- ✅ `src-tauri/tauri.conf.json`
- ✅ `src/main.tsx`

---

## 3. 开发工作流

### 3.1 启动开发模式

```bash
npm run tauri dev
```

**执行流程**:
1. Vite 启动开发服务器 (http://localhost:1420)
2. 编译 Rust 代码 (Debug 模式)
3. 启动 Tauri 桌面应用
4. 热重载已启用

**输出示例**:
```
> claude-code-launcher-tauri@0.1.0 dev
> vite

  VITE v7.0.4  ready in 500 ms

  ➜  Local:   http://localhost:1420/
  ➜  Network: use --host to expose

   Compiling tauri v2.0.0
   Compiling claude-code-launcher-tauri v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 12.34s
```

**应用窗口**: 自动打开桌面应用窗口

---

### 3.2 开发流程

#### 3.2.1 前端开发

**修改 React 组件**:
```typescript
// src/App.tsx
function App() {
  return (
    <div>
      <h1>修改后自动热重载</h1>
    </div>
  );
}
```

**保存文件后**:
- Vite 自动检测变化
- 浏览器自动刷新
- 无需手动重启

**修改样式**:
```css
/* src/index.css */
.card {
  @apply bg-gray-900;  /* 修改后立即生效 */
}
```

---

#### 3.2.2 后端开发

**修改 Rust 代码**:
```rust
// src-tauri/src/commands/mod.rs
#[tauri::command]
pub fn new_command() -> String {
    "Hello from Rust!".to_string()
}
```

**保存文件后**:
- Cargo 自动重新编译
- 应用自动重启
- 可能需要 10-30 秒

**注册新 Command**:
```rust
// src-tauri/src/lib.rs
.invoke_handler(tauri::generate_handler![
    // ... 现有 commands
    commands::new_command,  // 添加新 command
])
```

**前端调用**:
```typescript
const result = await invoke<string>('new_command');
console.log(result);  // "Hello from Rust!"
```

---

### 3.3 构建生产版本

```bash
npm run tauri build
```

**执行流程**:
1. 执行 `npm run build`
   - TypeScript 编译
   - Vite 构建前端
   - 输出到 `dist/`
2. Cargo 编译 Rust (Release 模式)
3. 生成安装包

**构建输出**:
```
src-tauri/target/release/
├── claude-code-launcher-tauri.exe       # 可执行文件
└── bundle/
    └── nsis/
        ├── claude-code-launcher-tauri_0.1.0_x64-setup.exe
        └── claude-code-launcher-tauri_0.1.0_x64.nsis.zip
```

**构建时间**: 首次约 5-10 分钟，后续 1-3 分钟

---

## 4. 调试技巧

### 4.1 前端调试

#### 4.1.1 浏览器 DevTools

**打开方式**:
- 按 `F12`
- 或 `Ctrl + Shift + I` (Windows)
- 或 `Cmd + Option + I` (macOS)

**功能**:
- **Console**: 查看日志和错误
- **Network**: 查看 Tauri IPC 调用
- **Elements**: 检查 DOM 和样式
- **Sources**: 断点调试

---

#### 4.1.2 控制台日志

```typescript
console.log('调试信息:', variable);
console.error('错误:', error);
console.warn('警告:', warning);
console.table(arrayData);
```

**查看 Tauri IPC 调用**:
```typescript
const result = await invoke('some_command');
console.log('Command result:', result);
```

---

### 4.2 后端调试

#### 4.2.1 打印调试

```rust
// 输出到 stderr (开发模式控制台可见)
eprintln!("调试信息: {:?}", variable);
eprintln!("执行到此处");
```

**查看输出**:
- 开发模式: 在启动 `npm run tauri dev` 的终端查看
- 生产模式: 需要重定向 stderr 到文件

---

#### 4.2.2 Rust 调试器

**使用 VS Code 调试**:

1. 安装 `CodeLLDB` 扩展:
```bash
code --install-extension vadimcn.vscode-lldb
```

2. 创建 `.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Tauri Debug",
      "cargo": {
        "args": [
          "build",
          "--manifest-path=src-tauri/Cargo.toml",
          "--no-default-features"
        ]
      },
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

3. 设置断点并按 `F5` 启动调试

---

#### 4.2.3 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compare() {
        assert_eq!(compare_versions("2.0.0", "1.0.0"), 1);
    }
}
```

**运行测试**:
```bash
cd src-tauri
cargo test
```

---

### 4.3 调试 Tauri IPC

**前端日志**:
```typescript
try {
  console.log('调用 command...');
  const result = await invoke('some_command', { param: value });
  console.log('Command 返回:', result);
} catch (error) {
  console.error('Command 失败:', error);
}
```

**后端日志**:
```rust
#[tauri::command]
pub fn some_command(param: String) -> Result<String, String> {
    eprintln!("收到参数: {}", param);

    let result = do_something(&param)?;

    eprintln!("返回结果: {}", result);
    Ok(result)
}
```

---

## 5. 常见问题

### 5.1 编译错误

#### 问题 1: Node.js 版本过低

**错误信息**:
```
error: package requires node >=18.0.0
```

**解决方法**:
```bash
# 更新 Node.js
winget upgrade OpenJS.NodeJS.LTS
```

---

#### 问题 2: Rust 编译失败

**错误信息**:
```
error: linking with `link.exe` failed
```

**解决方法**:
1. 确认已安装 Visual Studio C++ Build Tools
2. 重启终端
3. 重新编译

---

#### 问题 3: 依赖安装失败

**错误信息**:
```
npm ERR! network timeout
```

**解决方法**:
```bash
# 使用国内镜像
npm config set registry https://registry.npmmirror.com

# 或使用代理
npm config set proxy http://127.0.0.1:7890
```

---

### 5.2 运行时错误

#### 问题 1: Tauri Command 未找到

**错误信息**:
```
Command not found: some_command
```

**解决方法**:
1. 确认 Command 已在 `commands/mod.rs` 定义
2. 确认已在 `lib.rs` 中注册
3. 重新编译后端

---

#### 问题 2: 权限错误

**错误信息**:
```
Access denied
```

**解决方法**:
1. 检查 `capabilities/default.json` 权限配置
2. 确认操作系统权限
3. 使用管理员权限运行（仅调试）

---

#### 问题 3: 端口被占用

**错误信息**:
```
Error: Port 1420 is already in use
```

**解决方法**:
```bash
# 查找占用端口的进程
netstat -ano | findstr :1420

# 结束进程
taskkill /PID <PID> /F

# 或修改 vite.config.ts 中的端口
```

---

### 5.3 打包问题

#### 问题 1: NSIS 错误

**错误信息**:
```
NSIS executable not found
```

**解决方法**:
1. Tauri 会自动下载 NSIS
2. 如果失败，手动下载: https://nsis.sourceforge.io/
3. 设置环境变量: `TAURI_BUNDLER_NSIS_BIN`

---

#### 问题 2: 图标错误

**错误信息**:
```
Invalid icon format
```

**解决方法**:
1. 确认图标尺寸正确 (32x32, 128x128, 256x256)
2. 使用 PNG 格式
3. 使用工具生成 `.ico` 文件

---

## 6. 最佳实践

### 6.1 代码规范

#### 6.1.1 前端代码

**命名规范**:
```typescript
// 组件: PascalCase
function DependencyFrame() {}

// 函数: camelCase
function handleClick() {}

// 常量: UPPER_SNAKE_CASE
const DEFAULT_CONFIG = {};

// 文件: kebab-case 或 PascalCase
// dependency-frame.tsx 或 DependencyFrame.tsx
```

**导入顺序**:
```typescript
// 1. React 相关
import { useState } from 'react';

// 2. 第三方库
import { invoke } from '@tauri-apps/api/core';

// 3. 本地组件
import DependencyFrame from './components/DependencyFrame';

// 4. 类型和常量
import { AppConfig } from './types';
```

---

#### 6.1.2 后端代码

**Rust 命名规范**:
```rust
// 函数和变量: snake_case
fn check_nodejs() {}
let user_name = "test";

// 类型和 Trait: PascalCase
struct DependencyStatus {}
trait Checkable {}

// 常量: SCREAMING_SNAKE_CASE
const DEFAULT_TIMEOUT: u64 = 30;
```

**错误处理**:
```rust
// ✅ 推荐: 返回 Result
pub fn do_something() -> Result<String, String> {
    match operation() {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("操作失败: {}", e)),
    }
}

// ❌ 避免: 使用 unwrap()
let result = operation().unwrap();  // 可能 panic
```

---

### 6.2 性能优化

#### 6.2.1 前端优化

**避免不必要的重新渲染**:
```typescript
// ✅ 使用 memo
const MemoizedComponent = React.memo(ExpensiveComponent);

// ✅ 使用 useMemo
const expensiveValue = useMemo(() => {
  return computeExpensiveValue(a, b);
}, [a, b]);

// ✅ 使用 useCallback
const handleClick = useCallback(() => {
  doSomething(value);
}, [value]);
```

**懒加载组件**:
```typescript
const HeavyComponent = React.lazy(() => import('./HeavyComponent'));

<Suspense fallback={<Loading />}>
  <HeavyComponent />
</Suspense>
```

---

#### 6.2.2 后端优化

**避免不必要的克隆**:
```rust
// ❌ 不必要的克隆
fn process(data: String) -> String {
    data.to_uppercase()
}

// ✅ 使用引用
fn process(data: &str) -> String {
    data.to_uppercase()
}
```

**使用异步 I/O**:
```rust
// ✅ 异步读取文件
#[tauri::command]
pub async fn read_large_file() -> Result<String, String> {
    tokio::fs::read_to_string("large_file.txt")
        .await
        .map_err(|e| e.to_string())
}
```

---

### 6.3 安全性

**前端验证**:
```typescript
// ✅ 验证用户输入
if (!url.startsWith('http://') && !url.startsWith('https://')) {
  return alert('URL 格式不正确');
}

// ✅ 转义特殊字符
const safeValue = escapeHtml(userInput);
```

**后端验证**:
```rust
// ✅ 后端也要验证
#[tauri::command]
pub fn save_url(url: String) -> Result<(), String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL 格式不正确".to_string());
    }
    // ...
    Ok(())
}
```

---

### 6.4 Git 工作流

**提交信息规范**:
```bash
# 功能: feat
git commit -m "feat: 添加自动更新功能"

# 修复: fix
git commit -m "fix: 修复依赖检测失败的问题"

# 文档: docs
git commit -m "docs: 更新开发指南"

# 样式: style
git commit -m "style: 优化按钮样式"

# 重构: refactor
git commit -m "refactor: 重构依赖检测逻辑"

# 测试: test
git commit -m "test: 添加版本比较测试"

# 构建: build
git commit -m "build: 升级 Tauri 到 2.1.0"
```

**分支管理**:
```bash
# 功能分支
git checkout -b feature/auto-update

# 修复分支
git checkout -b fix/dependency-check

# 完成后合并到 main
git checkout main
git merge feature/auto-update
```

---

## 7. CI/CD 自动化构建

### 7.1 GitHub Actions 工作流

**工作流文件**: `.github/workflows/build.yml`

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Setup Rust
        uses: dtolnay/rust-action@stable
      - name: Install dependencies
        run: npm ci
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Claude Code Launcher ${{ github.ref_name }}'
          releaseDraft: true

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Setup Rust
        uses: dtolnay/rust-action@stable
      - name: Install dependencies
        run: npm ci
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Claude Code Launcher ${{ github.ref_name }}'
          releaseDraft: true
```

### 7.2 触发构建

**方式 1: 推送标签**
```bash
git tag v0.2.0
git push origin v0.2.0
```

**方式 2: 手动触发**
1. 打开 GitHub 仓库 → Actions
2. 选择 "Build and Release" 工作流
3. 点击 "Run workflow"

### 7.3 构建产物

| 平台 | 产物 |
|------|------|
| Windows | `.exe` 安装包 (NSIS) |
| macOS | `.app` 应用包 + `.dmg` 磁盘映像 |

### 7.4 重要限制

**跨平台打包限制**:
- ⚠️ Windows 无法直接打包 macOS 应用
- ⚠️ macOS 无法直接打包 Windows 应用
- ✅ 必须通过 CI/CD 在对应平台构建

**解决方案**:
1. **推荐**: 使用 GitHub Actions CI/CD
2. 在 Mac 上本地打包 macOS 版本
3. 在 Windows 上本地打包 Windows 版本

---

## 8. 发布流程

### 8.1 版本管理

**更新版本号**:

1. **package.json**:
```json
{
  "version": "0.2.0"
}
```

2. **src-tauri/Cargo.toml**:
```toml
[package]
version = "0.2.0"
```

3. **src-tauri/tauri.conf.json**:
```json
{
  "version": "0.2.0"
}
```

**版本号规范** (语义化版本):
- **主版本**: 不兼容的 API 变更
- **次版本**: 向下兼容的功能新增
- **修订版本**: 向下兼容的 Bug 修复

---

### 8.2 构建发布版本

```bash
# 1. 清理旧构建
npm run tauri build -- --clean

# 2. 构建新版本
npm run tauri build
```

**检查构建产物**:
```bash
# Windows
cd src-tauri/target/release/bundle/nsis
ls -lh

# macOS
cd src-tauri/target/release/bundle/macos
ls -lh
cd ../dmg
ls -lh
```

---

### 8.3 测试发布版本

**Windows 安装测试**:
1. 运行 `claude-code-launcher-tauri_0.2.0_x64-setup.exe`
2. 测试所有功能
3. 检查是否有错误

**macOS 安装测试**:
1. 双击 `.dmg` 文件挂载
2. 拖拽 `.app` 到 Applications 文件夹
3. 首次运行需要右键 → 打开 (绕过 Gatekeeper)
4. 测试所有功能

**便携版测试**:
1. 解压 `.nsis.zip` 或直接使用 `.app`
2. 直接运行程序
3. 测试功能

---

### 8.4 发布到 GitHub

```bash
# 1. 提交代码
git add .
git commit -m "release: v0.2.0"

# 2. 创建标签
git tag v0.2.0

# 3. 推送到远程
git push origin main
git push origin v0.2.0
```

**创建 Release**:
1. 访问 GitHub Releases
2. 点击 "Draft a new release"
3. 选择标签 `v0.2.0`
4. 填写 Release Notes
5. 上传安装包
6. 发布

---

### 8.5 更新文档

**更新 CHANGELOG.md**:
```markdown
## [0.2.0] - 2025-11-18

### Added
- 添加自动更新功能
- 支持更多自定义模型

### Fixed
- 修复依赖检测失败的问题
- 修复配置保存错误

### Changed
- 优化 UI 设计
- 改进错误提示
```

---

## 9. 总结

### 9.1 开发检查清单

**环境准备**:
- ✅ Node.js ≥ 18.0.0
- ✅ Rust ≥ 1.75.0
- ✅ C++ Build Tools (Windows)
- ✅ Git

**项目初始化**:
- ✅ 克隆/创建项目
- ✅ 安装前端依赖 (`npm install`)
- ✅ 检查 Rust 依赖 (`cargo check`)

**开发工作流**:
- ✅ 启动开发模式 (`npm run tauri dev`)
- ✅ 修改代码 (自动热重载)
- ✅ 调试错误 (DevTools + eprintln!)

**构建发布**:
- ✅ 更新版本号
- ✅ 构建生产版本 (`npm run tauri build`)
- ✅ 测试安装包
- ✅ 发布到 GitHub

---

### 9.2 相关资源

**官方文档**:
- [Tauri 官方文档](https://v2.tauri.app/)
- [React 官方文档](https://react.dev/)
- [Rust 官方文档](https://doc.rust-lang.org/)

**项目文档**:
- [项目总览](./PROJECT_DOCUMENTATION.md)
- [前端开发指南](./FRONTEND_GUIDE.md)
- [后端开发指南](./BACKEND_GUIDE.md)
- [API 参考](./API_REFERENCE.md)

**社区支持**:
- [Tauri Discord](https://discord.com/invite/tauri)
- [Rust 中文社区](https://rust.cc/)
- [React 中文文档](https://react.nodejs.cn/)

---

**祝开发顺利！**
