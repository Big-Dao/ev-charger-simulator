# Release v0.8.2 - 白屏和配置持久化修复

## 🔥 关键修复

这是一个**关键修复版本**，解决了 v0.8.0 中的两个严重问题：

### ✅ 修复 1：安装版本白屏问题

**问题**：使用安装包安装后，应用窗口显示白屏，无法使用。

**原因**：
- `index.html` 引用了不存在的 `/vite.svg` 图标
- 代码分割过多（13 个文件）导致资源加载复杂化
- Tauri 资源协议无法正确处理缺失资源

**修复**：
- ✅ 移除导致资源加载失败的图标引用
- ✅ 禁用代码分割，所有代码打包成单个 JS 文件（~1MB）
- ✅ 使用相对路径 `base: './'` 确保资源正确加载
- ✅ 禁用 CSP 限制避免资源被阻止

**验证**：✅ 测试通过 - 窗口正常显示，UI 完整加载

---

### ✅ 修复 2：配置持久化失败问题

**问题**：在安装版本中，充电桩列表为空，添加充电桩后重启应用配置丢失。

**原因**：
- 配置文件使用相对路径 `config/chargers.json`
- 生产环境当前工作目录不是应用安装目录
- 没有权限写入应用安装目录

**修复**：
- ✅ 配置文件现在保存到用户应用数据目录
  - **Windows**: `%APPDATA%\com.evcharger.simulator\config\chargers.json`
  - **macOS**: `~/Library/Application Support/com.evcharger.simulator/config/chargers.json`
  - **Linux**: `~/.local/share/com.evcharger.simulator/config/chargers.json`
- ✅ 实现配置文件查找优先级机制
- ✅ 添加、修改、删除充电桩后立即自动保存
- ✅ 自动创建配置目录

**验证**：✅ 测试通过 - 配置正确保存和加载，重启后数据保留

---

## 📦 下载

### Windows 安装包

- **MSI 安装包**（推荐）: `ev-charger-simulator_0.8.0_x64_en-US.msi`
  - 标准 Windows 安装程序
  - 自动创建开始菜单快捷方式
  - 支持卸载程序

- **NSIS 安装包**: `ev-charger-simulator_0.8.0_x64-setup.exe`
  - 轻量级安装程序
  - 安装选项可定制

**系统要求**:
- Windows 10/11 (64位)
- WebView2 Runtime（通常系统已包含）
- 磁盘空间: 约 50 MB

---

## 🆕 配置文件位置变更

**重要说明**：配置文件位置已更改，以确保正确的权限和持久化。

### Windows 用户

- **新位置**（v0.8.2+）:
  ```
  %APPDATA%\com.evcharger.simulator\config\chargers.json
  ```
  完整路径示例：
  ```
  C:\Users\YourName\AppData\Roaming\com.evcharger.simulator\config\chargers.json
  ```

- **快速访问**：
  打开文件资源管理器，地址栏输入：
  ```
  %APPDATA%\com.evcharger.simulator\config
  ```

### macOS 用户

```
~/Library/Application Support/com.evcharger.simulator/config/chargers.json
```

### Linux 用户

```
~/.local/share/com.evcharger.simulator/config/chargers.json
```

### 配置查找优先级

应用会按以下顺序查找配置文件：

1. **环境变量** `CHARGER_CONFIG_PATH`（自定义路径）
2. **应用数据目录**（生产环境默认，推荐）
3. **可执行文件目录**（便携模式）
4. **当前工作目录**（开发模式）

---

## 🚀 快速开始

### 安装步骤

1. 下载 `ev-charger-simulator_0.8.0_x64_en-US.msi`
2. 双击运行安装程序
3. 按提示完成安装
4. 从开始菜单或桌面快捷方式启动应用

### 首次使用

1. 应用启动后会自动创建配置目录
2. 点击"添加充电桩"按钮
3. 填写充电桩信息：
   - 充电桩 ID（如：CP000001）
   - 服务器 URL（如：ws://localhost:8080/ocpp）
   - 其他配置（可选）
4. 点击"确定"保存
5. 配置会自动保存，重启后保留

---

## ✅ 已验证功能

本版本经过以下测试：

- ✅ 应用窗口正常显示（不再白屏）
- ✅ UI 界面完整加载
- ✅ 添加充电桩功能正常
- ✅ 充电桩配置正确保存
- ✅ 重启后配置正确加载
- ✅ 修改/删除充电桩后配置更新
- ✅ 主题切换正常工作
- ✅ OCPP 1.6J 连接正常

---

## 📚 文档

- **用户指南**: [README.md](README.md)
- **功能列表**: [FEATURES.md](FEATURES.md)
- **配置位置说明**: [docs/CONFIG_LOCATION.md](docs/CONFIG_LOCATION.md)
- **完整变更日志**: [CHANGELOG.md](CHANGELOG.md)
- **发布总结**: [RELEASE_v0.8.2_SUMMARY.md](RELEASE_v0.8.2_SUMMARY.md)

---

## 🐛 已知问题

无关键问题。

如果遇到问题，请：
1. 检查配置文件位置是否正确
2. 查看应用日志（设置 `RUST_LOG=info`）
3. 在 [Issues](https://github.com/Big-Dao/ev-charger-simulator/issues) 提交问题报告

---

## 📝 完整更新日志

### Fixed - 修复

**安装版本白屏问题** (Critical)
- 移除 `index.html` 中的 `vite.svg` 引用
- 简化构建配置，禁用代码分割
- 所有前端代码打包成单个文件（1014 KB）
- 设置相对路径和禁用 CSP

**配置持久化失败** (Critical)
- 配置保存到应用数据目录
- 实现配置文件查找优先级
- 自动创建配置目录
- 添加配置路径日志

### Technical - 技术细节

**修改的文件**:
- `vite.config.ts` - 构建配置简化
- `index.html` - 移除资源引用
- `src-tauri/src/config_loader.rs` - 配置路径重构
- `src-tauri/tauri.conf.json` - CSP 配置

**构建变化**:
- 之前：13 个 JS 文件（代码分割）
- 现在：1 个 JS 文件（1014 KB）
- 应用大小：44.62 MB

---

## 🙏 致谢

感谢所有测试和反馈的用户！

如果这个项目对你有帮助，欢迎：
- ⭐ Star 本项目
- 🐛 提交问题报告
- 💡 提出功能建议
- 🔧 贡献代码

---

**发布时间**: 2025-10-02  
**版本号**: v0.8.2  
**标签**: v0.8.2  
**提交**: 4196bfe
