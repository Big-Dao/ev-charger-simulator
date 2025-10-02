# 白屏问题深度调试指南

## 🔍 当前状态

- ✅ Vite 配置：`base: './'` 已设置
- ✅ 构建产物：dist 目录正常，文件都存在
- ✅ CSP 策略：已设置为 null
- ❌ 问题：安装包运行后依然白屏

## 🛠️ 调试步骤

### 步骤 1：检查是否是 WebView2 问题

运行以下命令检查 WebView2 Runtime：

```powershell
# 检查 WebView2 安装状态
Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -ErrorAction SilentlyContinue

# 或者检查这个路径
Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -ErrorAction SilentlyContinue
```

如果没有安装，请下载安装：
https://developer.microsoft.com/microsoft-edge/webview2/#download-section

### 步骤 2：使用命令行运行查看错误

```powershell
# 直接从命令行运行，会显示所有错误信息
cd src-tauri\target\release
.\ev-charger-simulator.exe
```

**预期输出**：
- 如果有 JavaScript 错误，会在终端显示
- 如果有资源加载失败，会显示 404 或加载错误

### 步骤 3：启用 Rust 日志

在终端中设置环境变量后运行：

```powershell
$env:RUST_LOG="debug"
cd src-tauri\target\release
.\ev-charger-simulator.exe
```

这会显示详细的 Tauri 内部日志。

### 步骤 4：检查文件完整性

```powershell
# 检查 dist 目录是否正确打包到可执行文件中
# 查看可执行文件大小（应该包含 Vue 应用）
Get-Item "src-tauri\target\release\ev-charger-simulator.exe" | Select-Object Name, Length, LastWriteTime

# 如果文件太小（< 10MB），说明可能没有正确打包
```

### 步骤 5：临时构建配置

创建一个调试友好的构建：

```powershell
# 清理旧构建
Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force src-tauri\target\release -ErrorAction SilentlyContinue

# 重新构建
npm run build

# 检查构建产物
Get-ChildItem dist -Recurse | Measure-Object -Property Length -Sum
```

### 步骤 6：使用开发模式对比

```powershell
# 启动开发模式（应该正常工作）
npm run tauri:dev
```

如果开发模式正常，但 release 版本白屏，说明是构建配置问题。

## 🔧 可能的解决方案

### 解决方案 1：修改 Tauri 配置 - 禁用 CSP

已完成 ✅ - CSP 已设置为 null

### 解决方案 2：修改构建目标

编辑 `vite.config.ts`：

```typescript
build: {
  target: ['es2020', 'edge88', 'firefox78', 'chrome87', 'safari13'],  // 更兼容的目标
  minify: 'terser',  // 使用 terser 而不是 esbuild
  // ...
}
```

### 解决方案 3：禁用代码分割（临时测试）

编辑 `vite.config.ts`：

```typescript
build: {
  rollupOptions: {
    output: {
      manualChunks: undefined,  // 禁用代码分割
    },
  },
}
```

### 解决方案 4：使用绝对路径（某些情况）

编辑 `vite.config.ts`：

```typescript
base: process.env.TAURI_PLATFORM ? '/' : './',
```

### 解决方案 5：检查 index.html

确保 `dist/index.html` 中的所有路径都是相对路径（`./assets/...`）。

## 📊 诊断清单

请按顺序检查：

| 检查项 | 命令/方法 | 预期结果 | 状态 |
|--------|-----------|----------|------|
| WebView2 已安装 | 步骤 1 | 有版本号 | ⬜ |
| 命令行运行有输出 | 步骤 2 | 显示错误信息 | ⬜ |
| Rust 日志显示 | 步骤 3 | 有详细日志 | ⬜ |
| 文件大小正常 | 步骤 4 | > 10MB | ⬜ |
| dist 目录完整 | 步骤 5 | 所有文件都在 | ⬜ |
| 开发模式正常 | 步骤 6 | UI 正常显示 | ⬜ |

## 🆘 常见白屏原因及解决

### 原因 1：JavaScript 错误

**症状**：白屏 + 控制台有红色错误
**解决**：检查代码语法、依赖版本

### 原因 2：资源加载失败

**症状**：白屏 + 控制台 404 错误
**解决**：检查路径配置、base 设置

### 原因 3：CSP 策略过严

**症状**：白屏 + CSP 违规警告
**解决**：已设置 CSP 为 null ✅

### 原因 4：WebView2 未安装

**症状**：白屏 + 窗口立即关闭
**解决**：安装 WebView2 Runtime

### 原因 5：构建配置错误

**症状**：开发正常，生产白屏
**解决**：检查 vite.config.ts 和 tauri.conf.json

### 原因 6：防病毒软件阻止

**症状**：白屏 + 无报错
**解决**：临时禁用杀毒软件测试

## 📝 收集错误信息

如果以上都无法解决，请收集以下信息：

1. **系统信息**
```powershell
Get-ComputerInfo | Select-Object WindowsProductName, WindowsVersion, OsArchitecture
```

2. **WebView2 版本**
```powershell
Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
```

3. **应用日志**（如果有）
```powershell
Get-Content "$env:APPDATA\com.evcharger.simulator\logs\*.log" -ErrorAction SilentlyContinue
```

4. **命令行运行输出**
```powershell
.\src-tauri\target\release\ev-charger-simulator.exe 2>&1 | Tee-Object -FilePath "error.log"
```

## 🎯 下一步行动

请按以下顺序执行：

1. ✅ 运行步骤 2（命令行运行）并告诉我输出
2. ✅ 检查 WebView2 是否安装
3. ✅ 运行步骤 3（启用 Rust 日志）
4. ✅ 如果还是白屏，尝试解决方案 2 或 3

---

**创建时间**: 2025-10-02
**目的**: 深度调试白屏问题
