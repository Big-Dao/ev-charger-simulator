# 白屏问题 - 最终解决方案

## 🎯 问题根源

通过详细的日志分析，发现了白屏的真正原因：

```
Asset `vite.svg` not found; fallback to vite.svg.html
```

**关键问题**：
1. ❌ `index.html` 引用了不存在的 `/vite.svg` 图标
2. ❌ Tauri 无法处理这个缺失的资源，导致页面加载失败
3. ❌ 代码分割可能加剧了资源加载问题

## ✅ 实施的修复

### 修复 1：移除问题图标引用

**文件**: `index.html`

```html
<!-- 修复前 -->
<link rel="icon" type="image/svg+xml" href="/vite.svg" />

<!-- 修复后 -->
<!-- 移除了这一行 -->
```

**原因**: Tauri 的资源协议无法正确处理这个引用。

### 修复 2：使用简化构建配置

**文件**: `vite.config.ts` (使用 `vite.config.simple.ts`)

```typescript
// 关键配置
export default defineConfig({
  base: './',  // 相对路径
  build: {
    target: ['es2020', 'edge88'],
    minify: 'esbuild',
    sourcemap: false,
    rollupOptions: {
      output: {
        manualChunks: undefined,  // 禁用代码分割
      },
    },
  },
});
```

**效果**：
- ✅ 所有代码打包成单个文件（~1MB）
- ✅ 避免复杂的代码分割导致的加载问题
- ✅ 简化资源加载路径

### 修复 3：禁用 CSP 策略

**文件**: `src-tauri/tauri.conf.json`

```json
{
  "security": {
    "csp": null
  }
}
```

## 📊 构建结果对比

| 指标 | 修复前 | 修复后 |
|------|--------|--------|
| **文件数量** | 13 个 | 3 个 |
| **主 JS 大小** | ~35KB (分割) | ~1MB (单文件) |
| **总大小** | ~0.99 MB | ~1.01 MB |
| **加载方式** | 多个 chunk | 单文件加载 |
| **白屏问题** | ❌ 存在 | ✅ 应该解决 |

## 🧪 测试步骤

### 1. 运行新构建的应用

```powershell
.\src-tauri\target\release\ev-charger-simulator.exe
```

**预期结果**：
- ✅ 窗口正常打开
- ✅ UI 界面完整显示
- ✅ 功能正常工作

### 2. 如果仍然有问题，查看日志

```powershell
$env:RUST_LOG="info"
.\src-tauri\target\release\ev-charger-simulator.exe
```

**检查输出**：
- ❌ 不应该有 "Asset not found" 错误
- ❌ 不应该有 JavaScript 加载失败
- ✅ 应该只看到正常的启动日志

### 3. 验证功能

- [ ] 添加充电桩
- [ ] 启动充电桩
- [ ] 配置脚本
- [ ] 主题切换
- [ ] 数据持久化

## 🔧 如果还是白屏

### 选项 A：检查 WebView2 控制台

虽然生产版本无法直接打开开发者工具，但可以通过以下方式调试：

1. **临时启用 devtools**

编辑 `src-tauri/src/main.rs`，在窗口创建前添加：

```rust
// 在 tauri::Builder::default() 前添加
#[cfg(debug_assertions)]
use tauri::Manager;

tauri::Builder::default()
    .setup(|app| {
        #[cfg(debug_assertions)]
        {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
        }
        Ok(())
    })
    // ...
```

然后重新构建开发版本：
```powershell
npm run tauri:dev
```

### 选项 B：使用浏览器测试前端

```powershell
# 安装 http-server
npm install -g http-server

# 测试 dist 目录
cd dist
http-server -p 8080

# 浏览器访问 http://localhost:8080
```

如果浏览器能正常显示，说明是 Tauri 集成问题，不是前端代码问题。

### 选项 C：完全重置构建

```powershell
# 1. 清理所有构建产物
Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force src-tauri\target -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force node_modules -ErrorAction SilentlyContinue

# 2. 重新安装依赖
npm install

# 3. 重新构建
npm run build
npm run tauri:build

# 4. 测试
.\src-tauri\target\release\ev-charger-simulator.exe
```

## 📝 配置文件说明

### 当前使用的配置

```
vite.config.ts (简化版)
├── base: './'
├── target: ['es2020', 'edge88']
├── minify: 'esbuild'
├── sourcemap: false
└── manualChunks: undefined (禁用代码分割)
```

### 如果需要恢复优化版配置

```powershell
# 在确认简化版本正常后，可以尝试恢复
Copy-Item vite.config.backup.ts vite.config.ts

# 但建议先确保简化版本完全正常
```

## 🎓 经验教训

### 问题分析

1. **资源引用必须存在**
   - Tauri 的资源协议对缺失资源的容错性较差
   - 任何引用的文件都必须实际存在

2. **代码分割需谨慎**
   - 过多的 chunk 可能导致 Tauri 加载问题
   - 简单项目建议禁用代码分割

3. **路径必须相对**
   - `base: './'` 是必需的
   - 所有资源路径必须使用相对路径

4. **CSP 策略要宽松**
   - 开发阶段建议设置为 `null`
   - 生产环境可以逐步加强

### 最佳实践

```typescript
// Tauri + Vite 推荐配置
export default defineConfig({
  base: './',  // 必须
  clearScreen: false,  // Tauri 推荐
  
  server: {
    port: 1420,
    strictPort: true,
  },
  
  build: {
    target: ['es2020', 'edge88'],  // 兼容 WebView2
    sourcemap: false,  // 减小体积
    rollupOptions: {
      output: {
        manualChunks: undefined,  // 简单项目禁用
      },
    },
  },
});
```

## 🚀 发布检查清单

在发布前确认：

- [ ] `index.html` 没有引用不存在的资源
- [ ] `vite.config.ts` 设置了 `base: './'`
- [ ] `dist/index.html` 中所有路径都是相对的（`./assets/...`）
- [ ] `tauri.conf.json` 中 CSP 设置合理
- [ ] 在干净的环境测试安装包
- [ ] 确认 WebView2 已预安装或包含在安装包中

## 📦 构建安装包

```powershell
# 清理并构建
Remove-Item -Recurse -Force dist, src-tauri\target\release\bundle -ErrorAction SilentlyContinue
npm run build
npm run tauri:build

# 查找安装包
Get-ChildItem src-tauri\target\release\bundle -Recurse -Include *.msi, *.exe | Select-Object FullName, Length
```

## ✅ 成功标志

如果看到以下输出，说明修复成功：

```
2025-10-02T12:47:19.192767Z  INFO ev_charger_simulator: Starting EV Charger Simulator...
2025-10-02T12:47:20.306051Z  INFO ev_charger_simulator::config_loader: Loading charger configuration...
2025-10-02T12:47:20.306520Z  INFO ev_charger_simulator::manager: Added charger: CP000004
```

**不应该有**：
- ❌ "Asset not found" 错误
- ❌ "fallback to" 警告
- ❌ JavaScript 错误

---

**修复日期**: 2025-10-02 21:00  
**修复版本**: v0.8.1  
**状态**: 等待测试确认 ✅
