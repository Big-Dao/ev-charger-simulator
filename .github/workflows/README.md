# 🤖 GitHub Actions 自动化发布

本目录包含 GitHub Actions 工作流配置。

## 📁 文件说明

### `release.yml` - 自动发布工作流

**触发条件**：推送版本标签（如 `v0.8.4`）

**功能**：
- 自动构建 Windows 安装包（MSI + NSIS）
- 创建 GitHub Release
- 上传安装包到 Release
- 生成发布说明

**构建时间**：
- 首次：15-20 分钟
- 后续（有缓存）：5-10 分钟

---

### `ci.yml` - 持续集成工作流

**触发条件**：
- 推送到 `master` 分支
- Pull Request 到 `master` 分支

**功能**：
- 前端 Lint 检查
- Rust 格式检查
- Rust Clippy 检查
- 构建测试

**目的**：及早发现问题，确保代码质量

---

## 🚀 快速发布

### 使用发布脚本（推荐）

```powershell
# 在项目根目录运行
.\release.ps1 -Version v0.8.4
```

### 手动发布

```bash
# 1. 创建 tag
git tag -a v0.8.4 -m "Release v0.8.4"

# 2. 推送 tag
git push origin v0.8.4

# 3. 等待自动构建
# 访问: https://github.com/Big-Dao/ev-charger-simulator/actions
```

---

## 📚 详细文档

完整的自动化发布指南，请查看：

**[GitHub Actions 自动化发布指南](../docs/guides/GITHUB_ACTIONS_GUIDE.md)**

包含内容：
- ✅ 详细的发布流程
- ✅ 版本号规范
- ✅ 常见问题解决
- ✅ 高级用法
- ✅ 最佳实践

---

## 🔍 监控构建

- **Actions 页面**: https://github.com/Big-Dao/ev-charger-simulator/actions
- **Releases 页面**: https://github.com/Big-Dao/ev-charger-simulator/releases

---

## ⚙️ 技术栈

- **CI/CD**: GitHub Actions
- **构建工具**: Tauri Action
- **缓存**: Rust Cache, NPM Cache
- **发布**: 自动创建 GitHub Release

---

## 🎯 优势

- ✅ **全自动** - 推送 tag 即可发布
- ✅ **一致性** - 每次使用相同的构建环境
- ✅ **可追溯** - 完整的构建日志
- ✅ **省时** - 无需手动构建和上传
- ✅ **可靠** - 减少人为错误

---

**最后更新**: 2025-10-02  
**维护者**: @Big-Dao
