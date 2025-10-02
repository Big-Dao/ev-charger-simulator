# Release v0.8.3 - 预设脚本修复版本

## 🔧 本次修复

这是一个**功能修复版本**，解决了预设脚本在生产环境中无法加载的问题。

### ✅ 修复：预设脚本加载失败

**问题**：在安装版本中，选择预设脚本时提示"加载预设脚本失败"。

**原因**：
- 前端使用动态导入 `import()` 加载脚本文件
- 生产环境无法访问项目目录外的文件
- Tauri 资源协议路径解析失败

**修复**：
- ✅ 后端添加 Tauri 命令获取和读取脚本
- ✅ 使用 `include_str!()` 编译时嵌入脚本到可执行文件
- ✅ 前端使用 `invoke()` 动态加载脚本列表和内容
- ✅ 支持 4 个预设脚本：
  - `basic_test.js` - 基础测试脚本
  - `normal_charging.js` - 正常充电流程
  - `fast_charging.js` - 快速充电流程
  - `fault_test.js` - 故障测试脚本

---

## 🎯 改进优势

### 更可靠
- ✅ 脚本编译时嵌入，无需外部文件
- ✅ 不依赖文件系统路径
- ✅ 生产和开发环境行为一致

### 更快速
- ✅ 无 I/O 操作，直接从内存加载
- ✅ 启动即可用
- ✅ 选择脚本立即加载

### 更安全
- ✅ 脚本内容不可被修改
- ✅ 无路径遍历风险
- ✅ 编译时验证脚本存在

### 更易维护
- ✅ 脚本列表集中管理
- ✅ 添加新脚本只需修改后端
- ✅ 类型安全的 API

---

## 📦 下载

### Windows 安装包

**注意**：虽然版本号标记为 v0.8.3，但安装包文件名仍为 0.8.0（构建配置未更新）。功能已包含所有修复。

- **MSI 安装包**（推荐）: `ev-charger-simulator_0.8.0_x64_en-US.msi`
  - 标准 Windows 安装程序
  - 自动创建开始菜单快捷方式
  - 支持卸载程序
  - 大小: ~17.5 MB

- **NSIS 安装包**: `ev-charger-simulator_0.8.0_x64-setup.exe`
  - 轻量级安装程序
  - 安装选项可定制
  - 大小: ~12.3 MB

**系统要求**:
- Windows 10/11 (64位)
- WebView2 Runtime（通常系统已包含）
- 磁盘空间: 约 50 MB

---

## 🆕 v0.8.3 新增内容

### 功能修复

**预设脚本加载**
- 修复生产环境"加载预设脚本失败"错误
- 所有 4 个预设脚本现可正常使用
- 脚本选择器显示脚本名称和描述

### 技术改进

**后端 (Rust)**
```rust
// 新增命令
get_preset_scripts()  // 获取预设脚本列表
read_preset_script()  // 读取脚本内容
```

**前端 (Vue)**
```typescript
// 使用 Tauri 命令加载
const scripts = await invoke('get_preset_scripts');
const code = await invoke('read_preset_script', { scriptKey });
```

---

## 📚 包含 v0.8.2 的所有修复

v0.8.3 包含 v0.8.2 的所有关键修复：

### ✅ 白屏问题修复
- 移除导致加载失败的 vite.svg 引用
- 简化构建配置，禁用代码分割
- 所有代码打包成单个 JS 文件
- 窗口正常显示，UI 完整加载

### ✅ 配置持久化修复
- 配置保存到应用数据目录
  - Windows: `%APPDATA%\com.evcharger.simulator\config\chargers.json`
  - macOS: `~/Library/Application Support/com.evcharger.simulator/config/chargers.json`
  - Linux: `~/.local/share/com.evcharger.simulator/config/chargers.json`
- 重启后配置正确保留
- 支持自定义配置路径（环境变量）

---

## 🚀 快速开始

### 安装步骤

1. 下载 `ev-charger-simulator_0.8.0_x64_en-US.msi`
2. 双击运行安装程序
3. 按提示完成安装
4. 从开始菜单或桌面快捷方式启动

### 使用预设脚本

1. 点击"添加充电桩"或编辑现有充电桩
2. 切换到"脚本配置"标签页
3. 点击"选择预设脚本"下拉菜单
4. 选择一个预设脚本（如 `basic_test.js`）
5. 脚本内容自动填充到编辑器
6. 可选择"自动随充电桩启动脚本"
7. 点击"确定"保存

### 首次运行

1. 添加充电桩：
   - ID: `CP000001`
   - URL: `ws://localhost:8080/ocpp`
   - 协议: `OCPP 1.6J`
2. 选择预设脚本: `normal_charging.js`
3. 勾选"自动启动"
4. 点击"启动"按钮
5. 查看脚本运行状态

---

## ✅ 已验证功能

本版本经过以下测试：

### 核心功能
- ✅ 应用窗口正常显示（不白屏）
- ✅ UI 界面完整加载
- ✅ 主题切换正常

### 预设脚本功能（新）
- ✅ 预设脚本列表正确显示
- ✅ 选择 basic_test.js 正常加载
- ✅ 选择 normal_charging.js 正常加载
- ✅ 选择 fast_charging.js 正常加载
- ✅ 选择 fault_test.js 正常加载
- ✅ 脚本内容正确填充
- ✅ 脚本名称自动填充

### 充电桩功能
- ✅ 添加充电桩
- ✅ 启动充电桩连接
- ✅ 停止充电桩
- ✅ 修改充电桩配置
- ✅ 删除充电桩
- ✅ 批量启动/停止

### 配置持久化
- ✅ 充电桩配置正确保存
- ✅ 重启后配置正确加载
- ✅ 配置文件位置正确
- ✅ 脚本配置正确保存

### 脚本引擎
- ✅ 脚本正常运行
- ✅ 自动启动功能正常
- ✅ 停止脚本正常
- ✅ 脚本状态正确显示

---

## 📝 完整更新日志

### Fixed - 修复

**预设脚本加载失败** (Medium)
- 修复生产环境"加载预设脚本失败"错误
- 后端添加 `get_preset_scripts()` 和 `read_preset_script()` 命令
- 使用 `include_str!()` 编译时嵌入脚本
- 前端使用 `invoke()` 动态加载脚本

### Improved - 改进

**预设脚本系统**
- 脚本编译时嵌入（无需外部文件）
- 加载速度更快（无 I/O）
- 更安全（内容不可修改）
- 易于维护（集中管理）

### Documentation - 文档

- 添加 `PRESET_SCRIPT_FIX.md` - 修复详细说明
- 添加 `TEST_PRESET_SCRIPT.md` - 测试指南
- 添加 `PRESET_SCRIPT_FIX_SUMMARY.md` - 完整总结
- 更新 `CHANGELOG.md` - v0.8.3 版本信息

### Technical - 技术细节

**新增/修改文件**:
- `src-tauri/src/commands.rs` - 添加预设脚本命令（+58 行）
- `src-tauri/src/main.rs` - 注册命令（+2 行）
- `src/App.vue` - 使用 Tauri 命令加载（+35 行）

---

## 📚 文档

- **用户指南**: [README.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/README.md)
- **功能列表**: [FEATURES.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/FEATURES.md)
- **完整变更日志**: [CHANGELOG.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/CHANGELOG.md)
- **配置位置说明**: [docs/CONFIG_LOCATION.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/docs/CONFIG_LOCATION.md)
- **预设脚本修复**: [PRESET_SCRIPT_FIX.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/PRESET_SCRIPT_FIX.md)
- **测试指南**: [TEST_PRESET_SCRIPT.md](https://github.com/Big-Dao/ev-charger-simulator/blob/master/TEST_PRESET_SCRIPT.md)

---

## 🐛 已知问题

无关键问题。

如遇到问题：
1. 查看配置文件位置：`%APPDATA%\com.evcharger.simulator\config\`
2. 检查 WebView2 是否已安装
3. 在 [Issues](https://github.com/Big-Dao/ev-charger-simulator/issues) 提交问题报告

---

## 🔄 从旧版本升级

### 从 v0.8.0/v0.8.1 升级

1. 卸载旧版本（可选，配置会保留）
2. 安装 v0.8.3
3. 首次启动会自动迁移配置
4. 验证充电桩列表正确显示
5. 测试预设脚本功能

### 配置迁移

- ✅ 配置文件自动迁移到应用数据目录
- ✅ 旧配置自动加载（如果存在）
- ✅ 充电桩列表和脚本配置保留

---

## 🙏 致谢

感谢所有测试和反馈的用户！

如果这个项目对你有帮助，欢迎：
- ⭐ Star 本项目
- 🐛 提交问题报告
- 💡 提出功能建议
- 🔧 贡献代码
- 📣 分享给朋友

---

## 📊 版本对比

| 版本 | 白屏问题 | 配置持久化 | 预设脚本 | 发布日期 |
|------|---------|-----------|---------|---------|
| v0.8.0 | ❌ | ❌ | ❌ | 2025-10-02 |
| v0.8.2 | ✅ | ✅ | ❌ | 2025-10-02 |
| v0.8.3 | ✅ | ✅ | ✅ | 2025-10-02 |

---

**发布时间**: 2025-10-02  
**版本号**: v0.8.3  
**标签**: v0.8.3  
**提交**: 26aff1e  
**包含**: v0.8.2 所有修复 + 预设脚本修复
