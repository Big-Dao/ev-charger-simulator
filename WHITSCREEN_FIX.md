# 白屏问题修复指南

## 🐛 问题描述

安装生产版本的安装包后，运行 `.exe` 文件时窗口显示白屏，无法正常加载应用界面。

## 🔍 问题原因

1. **Base 路径配置缺失** - Vite 默认使用 `/` 作为基础路径，但 Tauri 需要相对路径 `./`
2. **CSP 策略过于严格** - 阻止了内联样式和脚本的执行
3. **资源加载路径错误** - 生产环境下无法正确加载静态资源

## ✅ 已修复内容

### 1. Vite 配置修复 (`vite.config.ts`)

```typescript
export default defineConfig({
  // 修复 Tauri 生产构建白屏问题
  base: './',  // ✅ 添加相对路径配置
  
  plugins: [
    // ...
  ],
  // ...
});
```

**作用**：确保所有资源使用相对路径加载，避免路径错误。

### 2. Tauri 配置修复 (`src-tauri/tauri.conf.json`)

#### 版本号更新

```json
{
  "package": {
    "productName": "ev-charger-simulator",
    "version": "0.8.0"  // ✅ 更新到当前版本
  }
}
```

#### CSP 策略配置

```json
{
  "security": {
    "csp": "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: asset: https://asset.localhost; font-src 'self' data:; connect-src 'self' ws: wss:;"
  }
}
```

**CSP 策略说明**：
- `default-src 'self'` - 默认只允许同源资源
- `script-src 'self' 'unsafe-inline' 'unsafe-eval'` - 允许内联脚本和 eval（Vue 运行时需要）
- `style-src 'self' 'unsafe-inline'` - 允许内联样式（Ant Design Vue 需要）
- `img-src 'self' data: asset: https://asset.localhost` - 允许图片资源
- `font-src 'self' data:` - 允许字体资源
- `connect-src 'self' ws: wss:` - 允许 WebSocket 连接（OCPP 需要）

## 🔨 重新构建步骤

修复后需要重新构建生产版本：

### 1. 清理旧构建

```bash
# 清理前端构建产物
rm -rf dist

# 清理 Rust 构建产物
rm -rf src-tauri/target/release
```

### 2. 重新构建

```bash
# 安装依赖（如果需要）
npm install

# 构建生产版本
npm run tauri:build
```

### 3. 查找新的安装包

构建完成后，在以下位置找到新的安装包：

- **Windows**: `src-tauri/target/release/bundle/msi/ev-charger-simulator_0.8.0_x64_en-US.msi`
- **Windows (NSIS)**: `src-tauri/target/release/bundle/nsis/ev-charger-simulator_0.8.0_x64-setup.exe`

## 🧪 测试验证

### 开发模式测试

```bash
npm run tauri:dev
```

如果开发模式正常，说明代码逻辑没问题。

### 生产模式测试

1. 安装新构建的 `.msi` 或 `.exe` 安装包
2. 运行安装后的应用程序
3. 检查是否正常显示界面
4. 测试基本功能：
   - ✅ 添加充电桩
   - ✅ 启动充电桩
   - ✅ 主题切换
   - ✅ WebSocket 连接

## 📝 其他可能的白屏原因

如果修复后仍然白屏，请检查：

### 1. 控制台错误

打开开发者工具（如果可以）：
- Windows: `Ctrl+Shift+I`
- macOS: `Cmd+Option+I`

查看是否有 JavaScript 错误或资源加载失败。

### 2. 防火墙/杀毒软件

某些安全软件可能阻止应用程序运行：
- 临时关闭防火墙测试
- 将应用添加到白名单

### 3. Windows Defender SmartScreen

首次运行可能被拦截：
- 点击"更多信息"
- 选择"仍要运行"

### 4. 依赖缺失

确保系统已安装：
- ✅ Microsoft Visual C++ Redistributable
- ✅ WebView2 Runtime（Tauri 需要）

安装 WebView2：
```
https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

### 5. 日志检查

查看应用日志文件（如果有）：
- Windows: `%APPDATA%\com.evcharger.simulator\logs`
- macOS: `~/Library/Logs/com.evcharger.simulator`
- Linux: `~/.local/share/com.evcharger.simulator/logs`

## 🔧 高级调试

### 启用 Tauri 调试模式

在 `src-tauri/tauri.conf.json` 中临时添加：

```json
{
  "tauri": {
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.evcharger.simulator",
      "icon": [...],
      "windows": {
        "console": true  // ✅ 启用控制台窗口
      }
    }
  }
}
```

重新构建后，运行时会显示一个额外的控制台窗口，可以看到详细的错误信息。

### 检查构建输出

构建时仔细查看输出：
```bash
npm run tauri:build 2>&1 | tee build.log
```

查找：
- ❌ ERROR 行
- ⚠️ WARNING 行
- 文件大小异常（过小可能表示构建不完整）

## 📚 相关文档

- [Tauri 配置文档](https://tauri.app/v1/api/config)
- [Vite 配置文档](https://vitejs.dev/config/)
- [CSP 策略指南](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)

## ✅ 验证清单

重新构建后，请确认：

- [ ] 应用窗口正常打开
- [ ] UI 界面正常显示
- [ ] 主题切换功能正常
- [ ] 添加充电桩功能正常
- [ ] WebSocket 连接正常
- [ ] 脚本功能正常
- [ ] 配置持久化正常

## 🆘 仍然有问题？

如果上述方法都无效，请：

1. 提供详细的错误信息
2. 系统环境信息（Windows 版本等）
3. 构建日志
4. 是否有控制台错误

提交 Issue：https://github.com/Big-Dao/ev-charger-simulator/issues

---

**修复日期**: 2025-10-02  
**影响版本**: v0.8.0  
**修复状态**: ✅ 已修复
