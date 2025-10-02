# 🎉 GitHub Release 发布指南 - v0.8.3

## ✅ 准备工作已完成

- ✅ 代码已提交并推送到 GitHub
- ✅ Git 标签 v0.8.3 已创建并推送
- ✅ CHANGELOG.md 已更新
- ✅ 发布说明文档已创建（GITHUB_RELEASE_v0.8.3.md）
- ✅ 安装包已构建完成

---

## 📦 构建产物

### Windows 安装包

位置：`src-tauri\target\release\bundle\`

1. **MSI 安装包**（推荐上传）
   ```
   src-tauri\target\release\bundle\msi\ev-charger-simulator_0.8.0_x64_en-US.msi
   ```
   - 大小: ~17.5 MB (18,358,272 字节)
   - 最后修改: 2025-10-02 21:31:31

2. **NSIS 安装包**（可选上传）
   ```
   src-tauri\target\release\bundle\nsis\ev-charger-simulator_0.8.0_x64-setup.exe
   ```
   - 大小: ~12.3 MB (12,920,079 字节)
   - 最后修改: 2025-10-02 21:32:09

---

## 🚀 在 GitHub 上创建 Release

### 步骤 1: 访问 Release 页面

在浏览器中打开：
```
https://github.com/Big-Dao/ev-charger-simulator/releases/new?tag=v0.8.3
```

或手动操作：
1. 访问 https://github.com/Big-Dao/ev-charger-simulator
2. 点击右侧的 "Releases"
3. 点击 "Draft a new release"
4. 在 "Choose a tag" 下拉菜单中选择 `v0.8.3`

---

### 步骤 2: 填写 Release 信息

#### 标签 (Tag)
- 选择: `v0.8.3` （已存在）

#### 发布标题 (Release Title)
```
v0.8.3 - 预设脚本修复版本
```

#### 描述 (Description)

**方式一：使用完整文档**（推荐）

复制 `GITHUB_RELEASE_v0.8.3.md` 文件的全部内容。

**方式二：使用简化版本**

```markdown
## 🔧 本次修复

修复生产环境预设脚本加载失败问题。

### ✅ 修复内容

**预设脚本加载**
- ✅ 修复"加载预设脚本失败"错误
- ✅ 所有 4 个预设脚本现可正常使用
- ✅ 脚本编译时嵌入可执行文件
- ✅ 加载速度更快，更安全

**可用脚本**
- basic_test.js - 基础测试
- normal_charging.js - 正常充电
- fast_charging.js - 快速充电
- fault_test.js - 故障测试

## 📦 下载

- **MSI 安装包**（推荐）: ev-charger-simulator_0.8.0_x64_en-US.msi
- **NSIS 安装包**: ev-charger-simulator_0.8.0_x64-setup.exe

**系统要求**: Windows 10/11 (64位)

## 📚 包含 v0.8.2 所有修复

✅ 白屏问题修复  
✅ 配置持久化修复  
✅ 预设脚本修复

详见 [CHANGELOG.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/CHANGELOG.md)
```

---

### 步骤 3: 上传安装包

在 "Attach binaries" 区域：

1. **拖放或点击上传 MSI 文件**
   - 文件: `ev-charger-simulator_0.8.0_x64_en-US.msi`
   - 等待上传完成（显示绿色勾号）

2. **（可选）上传 NSIS 文件**
   - 文件: `ev-charger-simulator_0.8.0_x64-setup.exe`
   - 等待上传完成

---

### 步骤 4: 发布设置

#### 选项配置

- ✅ **勾选** "Set as the latest release"
  - 将此版本标记为最新版本
  - 用户访问 Releases 页面时首先看到

- ❌ **不要勾选** "Set as a pre-release"
  - 这是正式发布版本，不是预发布

- ❌ **不要勾选** "Create a discussion for this release"
  - 可选，如果想创建讨论可以勾选

---

### 步骤 5: 发布

点击 **"Publish release"** 按钮完成发布。

---

## ✅ 发布后验证

### 1. 访问 Release 页面

```
https://github.com/Big-Dao/ev-charger-simulator/releases
```

验证：
- ✅ v0.8.3 显示在列表顶部
- ✅ 标记为 "Latest"
- ✅ 显示正确的发布标题
- ✅ 描述内容完整
- ✅ 安装包可以下载

### 2. 测试下载链接

点击安装包下载链接：
- ✅ MSI 文件可以正常下载
- ✅ NSIS 文件可以正常下载（如果上传了）
- ✅ 文件大小正确
- ✅ 文件可以正常安装

### 3. 检查标签

访问：
```
https://github.com/Big-Dao/ev-charger-simulator/tags
```

验证：
- ✅ v0.8.3 标签存在
- ✅ 标签关联到正确的提交
- ✅ 标签有 Release 链接

---

## 📋 快速操作清单

### 在浏览器中执行

```
☐ 1. 打开 https://github.com/Big-Dao/ev-charger-simulator/releases/new?tag=v0.8.3
☐ 2. 确认标签为 v0.8.3
☐ 3. 输入标题：v0.8.3 - 预设脚本修复版本
☐ 4. 粘贴 GITHUB_RELEASE_v0.8.3.md 内容到描述框
☐ 5. 上传 ev-charger-simulator_0.8.0_x64_en-US.msi
☐ 6. 上传 ev-charger-simulator_0.8.0_x64-setup.exe（可选）
☐ 7. 勾选 "Set as the latest release"
☐ 8. 点击 "Publish release"
☐ 9. 验证 Release 页面显示正常
☐ 10. 测试下载链接
```

---

## 📢 发布后宣传（可选）

### 更新 README.md

在 README.md 开头添加最新版本通知：

```markdown
> **最新版本**: v0.8.3 (2025-10-02) - [下载](https://github.com/Big-Dao/ev-charger-simulator/releases/tag/v0.8.3)  
> ✅ 修复预设脚本加载问题  
> ✅ 包含 v0.8.2 所有修复（白屏、配置持久化）
```

### GitHub Discussions

如果启用了 Discussions，可以发布公告：

```markdown
标题：🎉 v0.8.3 发布 - 预设脚本修复

正文：
v0.8.3 版本已发布！

本次修复：
✅ 预设脚本在生产环境可正常使用
✅ 包含 v0.8.2 的白屏和配置持久化修复

下载：https://github.com/Big-Dao/ev-charger-simulator/releases/tag/v0.8.3

强烈建议所有用户升级！
```

---

## 🎯 命令行快速操作

如果需要在 PowerShell 中打开 Release 页面：

```powershell
# 打开浏览器到 Release 创建页面
Start-Process "https://github.com/Big-Dao/ev-charger-simulator/releases/new?tag=v0.8.3"

# 打开文件管理器到安装包目录
explorer "src-tauri\target\release\bundle\msi"
```

---

## 📊 发布统计

### 代码统计

```
提交总数: 3 次 (v0.8.2 → v0.8.3)
- 2bd10cc: fix: 修复预设脚本加载失败问题
- 476cc94: docs: 添加预设脚本修复总结文档
- 26aff1e: docs: 更新 CHANGELOG.md 添加 v0.8.3 版本信息

修改文件: 5 个
新增代码: ~570 行（包括文档）
新增文档: 3 个
```

### 发布时间线

```
2025-10-02 21:31 - 构建完成
2025-10-02 21:35 - 代码修复提交
2025-10-02 21:40 - 文档完成
2025-10-02 21:45 - 标签创建和推送
2025-10-02 21:50 - 准备 GitHub Release
```

---

## 🆘 常见问题

### Q: 上传文件失败？

**A**: 检查网络连接和文件大小。如果文件太大，可以：
- 压缩成 ZIP 后上传
- 使用 GitHub CLI 上传
- 分多次上传

### Q: 标签已存在错误？

**A**: 标签已经推送到 GitHub，直接在下拉菜单选择即可。

### Q: 无法编辑发布内容？

**A**: 发布后可以点击 "Edit release" 按钮编辑。

### Q: 文件名显示 0.8.0 但版本是 0.8.3？

**A**: 正常，tauri.conf.json 中的版本号未更新。下次构建时更新即可。

---

**准备完成时间**: 2025-10-02 21:50  
**状态**: ✅ 就绪，可以发布  
**下一步**: 访问 GitHub Release 页面创建发布
