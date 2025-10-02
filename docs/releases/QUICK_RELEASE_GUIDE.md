# 🚀 v0.8.3 发布快速参考

## ⚡ 5 步完成发布

### 1️⃣ 填写标题
```
v0.8.3 - 预设脚本修复版本
```

### 2️⃣ 复制发布说明
- 打开文件: `GITHUB_RELEASE_v0.8.3.md`
- 全选 (Ctrl+A) → 复制 (Ctrl+C)
- 粘贴到 GitHub 描述框 (Ctrl+V)

### 3️⃣ 上传文件
拖放这两个文件到"Attach binaries"区域：
- ✅ `ev-charger-simulator_0.8.0_x64_en-US.msi` (~17.5 MB)
- ✅ `ev-charger-simulator_0.8.0_x64-setup.exe` (~12.3 MB)

### 4️⃣ 勾选选项
- ✅ **勾选** "Set as the latest release"
- ❌ 不勾选其他选项

### 5️⃣ 点击发布
点击绿色按钮：**"Publish release"**

---

## 📦 文件位置

安装包在这个目录（已打开文件管理器）：
```
src-tauri\target\release\bundle\msi\
src-tauri\target\release\bundle\nsis\
```

---

## 🔗 已打开的链接

✅ GitHub Release 创建页面（浏览器已打开）  
✅ 安装包目录（文件管理器已打开）

---

## ✅ 验证发布成功

发布后访问：https://github.com/Big-Dao/ev-charger-simulator/releases

检查：
- ✅ v0.8.3 在最上方
- ✅ 有 "Latest" 绿色标签
- ✅ 有 2 个安装包可下载
- ✅ 点击可以下载

---

## 📝 版本信息

- **版本**: v0.8.3
- **主要更新**: 预设脚本修复
- **包含**: v0.8.2 所有修复（白屏、配置持久化）
- **日期**: 2025-10-02

---

## 💡 简化版发布说明（备选）

如果不想复制完整文档，可以用这个：

```markdown
## 🔧 修复

✅ 预设脚本在生产环境可正常使用  
✅ 包含白屏和配置持久化修复

## 📦 下载

- MSI 安装包（推荐）  
- NSIS 安装包

**系统要求**: Windows 10/11 (64位)

详见: [CHANGELOG.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/CHANGELOG.md)
```

---

**预计耗时**: 5 分钟  
**准备状态**: ✅ 完全就绪  
**下一步**: 在浏览器中完成发布

🎉 祝发布顺利！
