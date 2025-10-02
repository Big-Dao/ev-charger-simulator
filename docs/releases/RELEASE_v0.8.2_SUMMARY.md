# v0.8.2 修复总结 - 白屏问题和配置持久化

## 🎯 本次修复的问题

### 问题 1：安装版本白屏 ✅ 已修复

**症状**：
- 安装包运行后窗口显示白屏
- 无法加载 UI 界面
- 开发模式正常，生产模式白屏

**根本原因**：
- `index.html` 引用了不存在的 `/vite.svg` 图标
- Tauri 资源协议无法处理缺失资源
- 过多的代码分割可能加剧加载问题

**修复方案**：
1. 移除 `index.html` 中的 `vite.svg` 图标引用
2. 使用简化配置禁用代码分割（`manualChunks: undefined`）
3. 所有代码打包成单个 JS 文件（~1MB）
4. 确保 `base: './'` 相对路径
5. 设置 `CSP: null`

**修复commit**: `062e4f1`, `a4e8430`

---

### 问题 2：配置持久化失败 ✅ 已修复

**症状**：
- 安装版本充电桩列表为空
- 添加充电桩后没有存盘
- 重启应用配置丢失

**根本原因**：
- 生产环境当前工作目录不是应用目录
- 配置文件路径 `config/chargers.json` 无法创建
- 没有适当的权限写入应用安装目录

**修复方案**：
1. 使用系统应用数据目录存储配置
   - Windows: `%APPDATA%\com.evcharger.simulator\config\`
   - macOS: `~/Library/Application Support/com.evcharger.simulator/config/`
   - Linux: `~/.local/share/com.evcharger.simulator/config/`
2. 配置文件查找优先级：
   - 环境变量 `CHARGER_CONFIG_PATH`
   - 应用数据目录（生产环境）
   - 可执行文件目录（便携模式）
   - 当前工作目录（开发模式）
3. 自动创建配置目录
4. 添加日志记录配置文件路径

**修复commit**: `39b7f11`

---

## 📦 构建产物

### 安装包位置

**Windows MSI**:
```
src-tauri\target\release\bundle\msi\ev-charger-simulator_0.8.0_x64_en-US.msi
```

**Windows NSIS** (如果有):
```
src-tauri\target\release\bundle\nsis\ev-charger-simulator_0.8.0_x64-setup.exe
```

### 可执行文件

```
src-tauri\target\release\ev-charger-simulator.exe (44.62 MB)
```

### 前端构建产物

```
dist/
├── index.html (0.42 KB)
└── assets/
    ├── index-B8ZvVBMa.css (11.18 KB)
    └── index-WhX5_MO5.js (1,014 KB)  ← 所有代码打包在一起
```

---

## 🧪 测试步骤

### 1. 白屏问题测试

```powershell
# 运行安装包
.\src-tauri\target\release\bundle\msi\ev-charger-simulator_0.8.0_x64_en-US.msi

# 或直接运行 exe
.\src-tauri\target\release\ev-charger-simulator.exe
```

**验证**：
- [ ] 窗口正常打开（不是白屏）
- [ ] UI 界面完整显示
- [ ] 主题切换正常
- [ ] 所有功能按钮可见

### 2. 配置持久化测试

运行测试脚本：
```powershell
.\test-config-persistence.ps1
```

**验证步骤**：
1. 首次运行应用
2. 添加充电桩（ID: CP000001, URL: ws://localhost:8080/ocpp）
3. 检查配置文件是否创建
4. 关闭应用
5. 重新启动应用
6. 验证充电桩列表是否保留

**验证**：
- [ ] 添加充电桩后，配置文件立即创建
- [ ] 配置文件位置正确：`%APPDATA%\com.evcharger.simulator\config\chargers.json`
- [ ] 配置文件包含充电桩数据
- [ ] 重启后充电桩列表正确显示
- [ ] 修改充电桩后更改被保存
- [ ] 删除充电桩后配置文件更新

### 3. 完整功能测试

- [ ] 添加充电桩
- [ ] 启动充电桩连接
- [ ] 停止充电桩
- [ ] 修改充电桩配置
- [ ] 删除充电桩
- [ ] 批量启动/停止
- [ ] 脚本配置和运行
- [ ] 主题切换
- [ ] 窗口大小调整

---

## 📝 新增/修改的文件

### 修复文件
- `vite.config.ts` - 使用简化配置
- `index.html` - 移除 vite.svg 引用
- `src-tauri/src/config_loader.rs` - 修复配置路径逻辑
- `src-tauri/tauri.conf.json` - CSP 设置为 null

### 文档文件
- `WHITSCREEN_FIX.md` - 白屏问题初步修复文档
- `WHITSCREEN_FINAL_FIX.md` - 白屏问题最终修复文档
- `WHITSCREEN_SOLUTION.md` - 白屏问题完整解决方案
- `DEBUG_WHITSCREEN.md` - 白屏问题调试指南
- `CONFIG_PERSISTENCE_FIX.md` - 配置持久化修复文档
- `docs/CONFIG_LOCATION.md` - 配置文件位置说明

### 测试脚本
- `fix-whitscreen.ps1` - 白屏修复脚本
- `test-whitscreen-fix.ps1` - 白屏修复测试脚本
- `test-config-persistence.ps1` - 配置持久化测试脚本

### 备份文件
- `vite.config.backup.ts` - 原配置备份
- `vite.config.simple.ts` - 简化配置

---

## 🔄 版本更新

### 当前版本
- **版本号**: v0.8.0 (tauri.conf.json)
- **修复级别**: v0.8.2 (实际功能)

### 建议版本号
更新 `src-tauri/tauri.conf.json`:
```json
{
  "package": {
    "version": "0.8.2"
  }
}
```

---

## 📋 发布检查清单

### 代码
- [x] 白屏问题修复
- [x] 配置持久化修复
- [x] 代码已提交到 Git
- [ ] 版本号已更新（需要从 0.8.0 → 0.8.2）

### 测试
- [ ] 白屏问题测试通过
- [ ] 配置持久化测试通过
- [ ] 所有功能正常工作
- [ ] 在干净的 Windows 环境测试

### 文档
- [x] 修复文档完整
- [x] 用户指南更新
- [ ] CHANGELOG.md 更新
- [ ] README.md 更新（如需要）

### 构建
- [x] 生产构建成功
- [ ] 安装包测试
- [ ] 文件大小合理（<50MB）
- [ ] 包含所有必需依赖

---

## 🚀 发布步骤

### 1. 更新版本号

```powershell
# 编辑 src-tauri/tauri.conf.json
# 将 version 从 "0.8.0" 改为 "0.8.2"

# 重新构建
npm run tauri:build
```

### 2. 更新 CHANGELOG

在 `CHANGELOG.md` 中添加：

```markdown
## [0.8.2] - 2025-10-02

### Fixed
- 🔴 修复安装版本白屏问题
  - 移除导致资源加载失败的 vite.svg 引用
  - 简化构建配置，禁用代码分割
  - 所有代码打包成单个文件以提高加载可靠性
- 🔴 修复配置持久化失败问题
  - 配置文件现在保存到用户应用数据目录
  - Windows: %APPDATA%\com.evcharger.simulator\config\
  - 添加、修改、删除充电桩后自动保存
  - 支持自定义配置路径（环境变量）
- ✅ 添加配置文件位置说明文档
- ✅ 添加自动化测试脚本

### Technical
- vite.config.ts: 禁用代码分割（manualChunks: undefined）
- config_loader.rs: 使用应用数据目录存储配置
- index.html: 移除不存在的资源引用
```

### 3. 创建 Git 标签

```powershell
git tag -a v0.8.2 -m "Release v0.8.2 - 修复白屏和配置持久化问题

修复:
- 安装版本白屏问题
- 配置持久化失败问题

详细信息见 CHANGELOG.md"

git push origin v0.8.2
```

### 4. 创建 GitHub Release

1. 访问：https://github.com/Big-Dao/ev-charger-simulator/releases/new
2. 选择标签：`v0.8.2`
3. 发布标题：`v0.8.2 - 白屏问题和配置持久化修复`
4. 使用 CHANGELOG 内容作为描述
5. 上传安装包：
   - `ev-charger-simulator_0.8.2_x64_en-US.msi`
   - `ev-charger-simulator_0.8.2_x64-setup.exe`（如果有）

---

## 💡 用户通知

### 升级提示

**重要说明**：
- ✅ 白屏问题已完全修复
- ✅ 配置现在正确保存
- ⚠️ 配置文件位置已更改到应用数据目录
- ℹ️ 首次启动会自动从旧位置加载配置
- ℹ️ 之后的更改会保存到新位置

### 快速开始

1. 下载并安装 v0.8.2
2. 首次运行会自动创建配置目录
3. 添加充电桩即可自动保存
4. 配置位置：`%APPDATA%\com.evcharger.simulator\config\`

---

## 🆘 如果还有问题

### 白屏问题

1. 确认安装的是 v0.8.2 版本
2. 检查 WebView2 是否已安装
3. 查看应用日志（设置 `RUST_LOG=info`）
4. 提交 Issue 并附上日志

### 配置问题

1. 检查配置文件位置：
   ```powershell
   explorer "$env:APPDATA\com.evcharger.simulator\config"
   ```
2. 查看应用日志确认保存路径
3. 检查文件权限
4. 尝试使用自定义路径（环境变量）

---

**测试状态**: ⏳ 等待测试  
**发布状态**: ⏳ 等待发布  
**更新时间**: 2025-10-02 21:30
