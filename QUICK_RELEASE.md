# 🚀 快速发布参考卡片

## ⚡ 一行命令发布

```powershell
.\release.ps1 -Version v0.8.4
```

就这么简单！脚本会自动完成：
- ✅ 检查工作区状态
- ✅ 拉取最新代码
- ✅ 创建版本 tag
- ✅ 推送到 GitHub
- ✅ 触发自动构建

---

## 📋 完整流程（3 步）

### 1️⃣ 更新文档
```powershell
code CHANGELOG.md
git add CHANGELOG.md
git commit -m "docs: 更新 CHANGELOG v0.8.4"
git push
```

### 2️⃣ 运行发布
```powershell
.\release.ps1 -Version v0.8.4
```

### 3️⃣ 等待完成
- 访问: https://github.com/Big-Dao/ev-charger-simulator/actions
- 等待: 10 分钟
- 完成: https://github.com/Big-Dao/ev-charger-simulator/releases

---

## 🎯 版本号选择

| 变更 | 版本号 | 示例 |
|------|--------|------|
| 🐛 Bug 修复 | v0.8.3 → v0.8.**4** | 补丁 +1 |
| ✨ 新功能 | v0.8.4 → v0.**9**.0 | 次版本 +1 |
| 💥 破坏性变更 | v0.9.0 → v**1**.0.0 | 主版本 +1 |

---

## 📊 监控链接

- **Actions**: https://github.com/Big-Dao/ev-charger-simulator/actions
- **Releases**: https://github.com/Big-Dao/ev-charger-simulator/releases

---

## 🐛 如果 tag 已存在

```powershell
# 删除并重新创建
git push --delete origin v0.8.4
git tag -d v0.8.4
.\release.ps1 -Version v0.8.4
```

---

## 💡 提示

- ⏱️ 首次构建：15-20 分钟
- ⚡ 后续构建：5-10 分钟
- 📚 详细文档：`docs/guides/GITHUB_ACTIONS_GUIDE.md`

---

**就是这么简单！** 🎉
