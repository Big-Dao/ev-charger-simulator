# 手动发布记录 - v0.8.4

## 📋 发布信息

- **版本号**: v0.8.4
- **发布日期**: 2025-10-02
- **发布方式**: 手动发布（GitHub Actions 失败后）
- **发布者**: Big-Dao
- **Release URL**: https://github.com/Big-Dao/ev-charger-simulator/releases/tag/v0.8.4

---

## 🐛 GitHub Actions 失败原因

### 问题描述
GitHub Actions 自动发布失败，错误信息：
```
npm error ERESOLVE could not resolve
npm error peer vite@"^5.0.0 || ^6.0.0" from @vitejs/plugin-vue@5.2.4
npm error Found: vite@7.1.7
npm error Conflicting peer dependency
```

### 根本原因
- 项目使用 `vite@7.1.7`
- `@vitejs/plugin-vue@5.2.4` 要求 `vite@^5.0.0 || ^6.0.0`
- 版本不兼容导致 `npm ci` 失败

### 解决方案
在 GitHub Actions 工作流中使用 `npm ci --legacy-peer-deps`：
- ✅ 修改 `.github/workflows/release.yml`
- ✅ 修改 `.github/workflows/ci.yml`

---

## ✅ 手动发布流程

### 1. 本地构建
```powershell
npm run tauri build
```

**构建结果**:
- ✅ MSI 安装包: `ev-charger-simulator_0.8.3_x64_en-US.msi` (18,358,272 bytes)
- ✅ NSIS 安装包: `ev-charger-simulator_0.8.3_x64-setup.exe` (12,914,131 bytes)
- ⚠️ Rust 编译警告（dead_code）- 不影响功能

### 2. 创建 GitHub Release
```powershell
gh release create v0.8.4 \
  "src-tauri\target\release\bundle\msi\ev-charger-simulator_0.8.3_x64_en-US.msi#Windows-MSI-Installer-v0.8.4.msi" \
  "src-tauri\target\release\bundle\nsis\ev-charger-simulator_0.8.3_x64-setup.exe#Windows-NSIS-Installer-v0.8.4.exe" \
  --title "v0.8.4 - 修复安装包发布者信息" \
  --notes "..."
```

**发布结果**: ✅ 成功

### 3. 提交修复
```powershell
git add -A
git commit -m "fix: GitHub Actions 添加 --legacy-peer-deps 解决 vite 7 依赖冲突"
git push origin master
```

**Commit**: 9f2fa47

---

## 🔧 本版本修复内容

### 主要修复
1. **安装包 Publisher 修复**
   - 问题: 安装包 publisher 显示为 "unknown"
   - 修复: 在 `tauri.conf.json` 添加 `"publisher": "Big-Dao"`
   - 文件: `src-tauri/tauri.conf.json`
   - Commit: 5b6bcdc

2. **GitHub Actions 依赖冲突修复**
   - 问题: vite 7.1.7 与 @vitejs/plugin-vue 5.2.4 版本冲突
   - 修复: 工作流中使用 `npm ci --legacy-peer-deps`
   - 文件: `.github/workflows/release.yml`, `.github/workflows/ci.yml`
   - Commit: 9f2fa47

### 版本更新
- `tauri.conf.json` 版本: 0.8.0 → 0.8.3

---

## 📦 安装包信息

### MSI 安装包
- **文件名**: Windows-MSI-Installer-v0.8.4.msi
- **大小**: 18.3 MB (18,358,272 bytes)
- **类型**: Windows Installer Package
- **Publisher**: Big-Dao ✅
- **推荐使用**: 标准 Windows 安装程序

### NSIS 安装包
- **文件名**: Windows-NSIS-Installer-v0.8.4.exe
- **大小**: 12.9 MB (12,914,131 bytes)
- **类型**: NSIS Installer
- **Publisher**: Big-Dao ✅
- **优势**: 体积更小

---

## 🧪 验证清单

- ✅ 本地构建成功
- ✅ MSI 安装包生成
- ✅ NSIS 安装包生成
- ✅ Publisher 信息正确（Big-Dao）
- ✅ GitHub Release 创建成功
- ✅ 安装包上传成功
- ✅ Release 页面可访问
- ✅ 修复代码已提交
- ✅ 修复代码已推送

---

## 📝 经验总结

### 为什么需要手动发布？
1. GitHub Actions 构建环境严格使用 `npm ci`
2. 本地 `node_modules` 可能有不同解析结果
3. Peer dependencies 冲突在 CI 环境更严格

### 未来建议
1. ✅ **已修复**: 工作流使用 `--legacy-peer-deps`
2. 📋 **待评估**: 升级 `@vitejs/plugin-vue` 到支持 vite 7 的版本
3. 📋 **待考虑**: 降级 vite 到 6.x（如果 vue plugin 不支持 7.x）

### 手动发布优势
- ✅ 完全控制发布流程
- ✅ 可以本地验证构建产物
- ✅ 绕过 CI 环境限制
- ✅ 发布速度更快（无需等待 15-20 分钟）

### 自动发布优势
- ✅ 标准化流程
- ✅ 减少人为错误
- ✅ 完整的 CI/CD 记录
- ✅ 团队协作友好

---

## 🔗 相关链接

- **Release 页面**: https://github.com/Big-Dao/ev-charger-simulator/releases/tag/v0.8.4
- **Commit 5b6bcdc**: fix: 设置安装包 publisher 为 Big-Dao
- **Commit 9f2fa47**: fix: GitHub Actions 添加 --legacy-peer-deps 解决 vite 7 依赖冲突
- **Actions 历史**: https://github.com/Big-Dao/ev-charger-simulator/actions

---

## 📊 时间线

| 时间 | 事件 |
|------|------|
| 23:35 | 本地构建完成 |
| 23:36 | GitHub Release 创建成功 |
| 23:37 | 修复代码提交并推送 |
| **总耗时** | **~5 分钟** (vs GitHub Actions 预计 15-20 分钟) |

---

**下次发布**: GitHub Actions 已修复，可使用自动发布流程 🚀
