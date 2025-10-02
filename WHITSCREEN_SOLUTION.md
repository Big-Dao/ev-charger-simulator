# 白屏问题 - 最终解决方案

## 🎯 问题状态

- ✅ WebView2 已安装（版本 140.0.3485.94）
- ✅ 开发模式正常 (`npm run tauri:dev`)
- ✅ 构建产物完整（44.62 MB）
- ❌ 生产版本白屏

## 🔬 测试方案

我们准备了两个配置供您测试：

### 方案 A：优化版配置（当前使用）

**文件**：`vite.config.ts`

**特点**：
- 代码分割优化（Ant Design Vue 分组加载）
- 构建目标：es2020, edge88, firefox78, chrome87, safari13
- 适合生产环境

**测试命令**：
```powershell
# 清理并构建
Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
npm run build
npm run tauri:build

# 测试
.\src-tauri\target\release\ev-charger-simulator.exe
```

### 方案 B：简化版配置（备用）

**文件**：`vite.config.simple.ts`

**特点**：
- 完全禁用代码分割
- 所有代码打包成一个文件
- 更简单但文件更大

**测试命令**：
```powershell
# 1. 备份当前配置
Copy-Item vite.config.ts vite.config.backup.ts

# 2. 使用简化配置
Copy-Item vite.config.simple.ts vite.config.ts

# 3. 清理并构建
Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
npm run build
npm run tauri:build

# 4. 测试
.\src-tauri\target\release\ev-charger-simulator.exe

# 5. 如果不行，恢复原配置
Copy-Item vite.config.backup.ts vite.config.ts
```

## 🔍 深度调试

如果两个方案都白屏，请执行以下调试：

### 1. 启用详细日志

```powershell
$env:RUST_LOG="tauri=debug,wry=debug,tao=debug"
.\src-tauri\target\release\ev-charger-simulator.exe
```

查看输出中是否有：
- 资源加载失败
- JavaScript 错误
- WebView 初始化错误

### 2. 检查 dist/index.html

```powershell
Get-Content dist\index.html
```

确认：
- 所有路径都是相对路径（`./assets/...`）
- script 标签有 `type="module"`
- 没有绝对路径（`/assets/...`）

### 3. 手动测试 dist 目录

使用 Web 服务器测试 dist 目录：

```powershell
# 安装简单的 HTTP 服务器
npm install -g http-server

# 在 dist 目录启动服务
cd dist
http-server -p 8080

# 浏览器访问 http://localhost:8080
```

如果浏览器能正常显示，说明前端代码没问题，是 Tauri 的资源加载问题。

### 4. 检查 Tauri 资源配置

编辑 `src-tauri/tauri.conf.json`，尝试添加：

```json
{
  "tauri": {
    "bundle": {
      "resources": []
    }
  }
}
```

## 💡 其他可能的解决方案

### 解决方案 1：使用 hash 路由（如果有路由）

如果项目使用了 Vue Router，确保使用 hash 模式：

```typescript
// router/index.ts
import { createRouter, createWebHashHistory } from 'vue-router';

const router = createRouter({
  history: createWebHashHistory(),  // 使用 hash 模式
  routes: [...]
});
```

### 解决方案 2：添加错误边界

在 `src/App.vue` 中添加：

```vue
<script setup lang="ts">
import { onErrorCaptured } from 'vue';

onErrorCaptured((err) => {
  console.error('Vue Error:', err);
  return false;
});

// 全局错误处理
window.onerror = function(message, source, lineno, colno, error) {
  console.error('Global Error:', { message, source, lineno, colno, error });
};
</script>
```

### 解决方案 3：检查 Ant Design Vue 版本兼容性

尝试降级到稳定版本：

```powershell
npm install ant-design-vue@4.0.0
npm run build
npm run tauri:build
```

### 解决方案 4：完全禁用 CSP（已完成 ✅）

`src-tauri/tauri.conf.json`：
```json
{
  "security": {
    "csp": null  // ✅ 已设置
  }
}
```

### 解决方案 5：使用 Tauri v2（重大升级）

如果以上都无效，考虑升级到 Tauri v2：

```powershell
# 这是重大升级，需要修改代码
npm install @tauri-apps/api@next @tauri-apps/cli@next
```

## 📊 调试检查清单

请按顺序执行并记录结果：

| # | 检查项 | 命令 | 结果 |
|---|--------|------|------|
| 1 | 方案 A 构建测试 | 见上文 | ⬜ 正常 / ⬜ 白屏 |
| 2 | 方案 B 简化配置 | 见上文 | ⬜ 正常 / ⬜ 白屏 |
| 3 | 启用详细日志 | 见上文 | ⬜ 有错误信息 |
| 4 | HTTP 服务器测试 | 见上文 | ⬜ 浏览器正常 |
| 5 | 检查 index.html | `cat dist/index.html` | ⬜ 路径正确 |

## 🆘 如果仍然白屏

请提供以下信息：

1. **执行方案 A 和 B 的结果**
2. **详细日志输出**（步骤 1）
3. **HTTP 服务器测试结果**（步骤 3）
4. **dist/index.html 内容**
5. **系统信息**：
```powershell
Get-ComputerInfo | Select-Object WindowsProductName, WindowsVersion, OsBuildNumber
```

## 🎯 当前建议

**立即执行**：
1. 等待当前构建完成
2. 测试新构建的 exe
3. 如果仍然白屏，尝试方案 B（简化配置）
4. 记录所有错误信息

**下一步**：
根据测试结果决定：
- 如果方案 B 成功 → 逐步优化代码分割
- 如果都失败 → 深度调试（启用日志）
- 如果有错误信息 → 针对性修复

---

**更新时间**: 2025-10-02 20:45
**状态**: 等待测试结果
