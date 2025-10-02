# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.8.3] - 2025-10-02

### 🔧 Fixed - 功能修复

**预设脚本加载失败问题** (Medium)
- 修复生产环境中"加载预设脚本失败"的错误
- 根本原因：前端使用动态导入 `import()` 无法在 Tauri 生产环境中访问项目外文件
- 解决方案：
  - 后端添加 Tauri 命令 `get_preset_scripts()` 和 `read_preset_script()`
  - 使用 `include_str!()` 宏在编译时将脚本嵌入可执行文件
  - 前端使用 `invoke()` 调用后端命令加载脚本
  - 动态加载预设脚本列表（支持扩展）
- **影响**: 用户现可在生产环境正常使用预设脚本功能（basic_test.js, normal_charging.js, fast_charging.js, fault_test.js）

### 🎯 Improved - 改进

**预设脚本系统**
- ✅ 脚本编译时嵌入，无需外部文件
- ✅ 加载速度更快（无 I/O 操作）
- ✅ 更安全（脚本内容不可修改）
- ✅ 易于维护（添加新脚本只需修改后端代码）

### 📚 Documentation - 文档更新

- 添加 `PRESET_SCRIPT_FIX.md` - 预设脚本修复详细说明（200+ 行）
- 添加 `TEST_PRESET_SCRIPT.md` - 预设脚本测试指南（150+ 行）
- 添加 `PRESET_SCRIPT_FIX_SUMMARY.md` - 完整修复总结（400+ 行）

### 🛠️ Technical - 技术细节

**新增文件**:
- `src-tauri/src/commands.rs` - 添加 `PresetScript` 结构体和相关命令（58 行）
- 修改 `src-tauri/src/main.rs` - 注册新命令（2 行）
- 修改 `src/App.vue` - 使用 Tauri 命令加载脚本（35 行）

---

## [0.8.2] - 2025-10-02

### 🔥 Fixed - 关键问题修复

**🔴 安装版本白屏问题** (Critical)
- 移除 `index.html` 中导致资源加载失败的 `vite.svg` 引用
- 简化 Vite 构建配置，禁用代码分割（`manualChunks: undefined`）
- 所有前端代码打包成单个 JS 文件（~1MB）以提高加载可靠性
- 设置 `base: './'` 确保资源使用相对路径
- 禁用 CSP 限制避免资源加载被阻止
- **影响**: 修复后应用窗口正常显示，UI 完整加载

**🔴 配置持久化失败问题** (Critical)
- 配置文件现在保存到用户应用数据目录（而非应用安装目录）
  - Windows: `%APPDATA%\com.evcharger.simulator\config\chargers.json`
  - macOS: `~/Library/Application Support/com.evcharger.simulator/config/chargers.json`
  - Linux: `~/.local/share/com.evcharger.simulator/config/chargers.json`
- 实现配置文件查找优先级机制
  1. 环境变量 `CHARGER_CONFIG_PATH`（自定义路径）
  2. 应用数据目录（生产环境默认）
  3. 可执行文件目录（便携模式）
  4. 当前工作目录（开发模式）
- 添加、修改、删除充电桩后立即保存配置
- 自动创建配置目录和父目录
- 添加配置文件路径日志记录
- **影响**: 充电桩配置在重启后正确保留，不再丢失数据

### 📚 Documentation - 文档更新

- 添加 `CONFIG_PERSISTENCE_FIX.md` - 配置持久化修复详细说明
- 添加 `WHITSCREEN_FINAL_FIX.md` - 白屏问题最终修复方案
- 添加 `docs/CONFIG_LOCATION.md` - 配置文件位置用户指南
- 添加 `RELEASE_v0.8.2_SUMMARY.md` - 发布总结和测试报告

### 🧪 Testing - 测试改进

- 添加 `test-whitscreen-fix.ps1` - 白屏修复自动化测试脚本
- 添加 `test-config-persistence.ps1` - 配置持久化自动化测试脚本
- 测试结果：✅ 白屏问题已解决，✅ 配置持久化正常工作

### 🛠️ Technical - 技术细节

**修改的文件**:
- `vite.config.ts` - 禁用代码分割，使用相对路径
- `index.html` - 移除不存在的资源引用
- `src-tauri/src/config_loader.rs` - 重构配置路径逻辑，使用应用数据目录
- `src-tauri/tauri.conf.json` - 禁用 CSP 限制

**构建产物变化**:
- 之前：13 个 JS 文件（代码分割）
- 现在：1 个 JS 文件（1014 KB，单文件加载）
- 应用大小：44.62 MB（不变）

---

## [0.8.0] - 2025-10-02

🎉 **首个正式发布版本**

### ✨ Added - 新增功能

#### 核心功能 (80% 完成)

**OCPP 1.6J 协议 (100%)**
- WebSocket 异步客户端实现
- 完整的 OCPP 1.6J 消息支持
  - BootNotification（启动通知）
  - Heartbeat（心跳保活）
  - StatusNotification（状态通知）
  - StartTransaction（启动充电）
  - StopTransaction（停止充电）
  - MeterValues（电表数据上报）
  - Authorize（用户授权）
  - DataTransfer（数据传输）
- 服务器请求处理
  - RemoteStartTransaction
  - RemoteStopTransaction
  - Reset
  - ChangeConfiguration
  - GetConfiguration
  - UnlockConnector
  - TriggerMessage
- 自动重连机制（指数退避）
- 心跳保活机制
- 消息序列化/反序列化（JSON-RPC 2.0）

**充电桩管理 (95%)**
- 充电桩 CRUD 操作
  - 添加/删除/修改充电桩
  - 批量创建充电桩（支持数百个）
  - 查看充电桩详细信息
- 充电桩状态机
  - 9 种状态支持（Available, Preparing, Charging, SuspendedEV, SuspendedEVSE, Finishing, Reserved, Unavailable, Faulted）
  - 完整的状态转换逻辑
- 充电会话管理
  - 启动/停止充电会话
  - 会话 ID 生成和管理
  - 累计电量计算
  - 充电功率动态调整

**Deno Core 脚本引擎 (90%)**
- Deno Core 0.320 集成
- JavaScript ES6+ 语法支持
- 异步操作支持（async/await）
- 完整的脚本 API
  - `op_get_charger()` - 获取充电桩对象
  - `charger.start_charging()` - 启动充电
  - `charger.stop_charging()` - 停止充电
  - `charger.set_power(kw)` - 设置充电功率
  - `charger.get_status()` - 获取状态
  - `charger.get_power()` - 获取功率
  - `charger.get_energy()` - 获取电量
  - `console.log()` - 日志输出
  - `setTimeout/setInterval` - 定时器
- 预设脚本库
  - basic_test.js - 基础功能测试
  - normal_charging.js - 正常充电流程
  - fast_charging.js - 快速充电流程
  - fault_test.js - 故障测试场景
- 脚本启动/停止控制
- 自动启动脚本功能

**数据持久化 (100%)**
- 配置文件持久化（config/chargers.json）
  - 单个充电桩配置
  - 批量充电桩配置
  - 启动时自动加载
  - 修改后自动保存
- 环境变量支持（CHARGER_CONFIG_PATH）
- 主题选择持久化（localStorage）
- 窗口状态持久化（位置、大小）

#### 用户界面 (85%)

**布局和主题**
- Vue 3.4 + Composition API
- Ant Design Vue 4.1 企业级组件库
- 响应式布局（Header + Content + Footer）
- 暗色/亮色主题切换
- 主题持久化
- 高分辨率屏幕适配（2K/4K）

**功能界面**
- 统计面板
  - 总桩数、在线数、充电中数量、总功率
  - 实时数据更新（1秒刷新）
- 充电桩列表
  - 表格视图展示
  - 分页功能（10/20/50/100条）
  - 展开查看详细信息
  - 时间格式化（YYYY-MM-DD HH:mm:ss）
  - 状态标签（Tag 组件）
- 操作功能
  - 添加/修改/删除充电桩
  - 启动/停止充电桩
  - 脚本配置对话框
  - 启动/停止脚本
  - 批量操作（启动/停止所有）
- 使用说明
  - 内置快速上手指南（5步）
  - 图标化操作说明
  - 功能提示和注意事项

#### 文档体系 (100%)

**核心文档（4份）**
- README.md - 项目总览（200+ 行）
- FEATURES.md - 功能特性索引（570+ 行）
- DOCS_INDEX.md - 文档导航（300+ 行）
- README_EXTENDED.md - 扩展文档（490+ 行）

**工程文档（4份）**
- requirements.md - 需求与实现状态（320 行）
- PROJECT_SUMMARY.md - 项目实现总结（350 行）
- TECH_STACK.md - 技术栈说明（400 行）
- SETUP.md - 安装配置指南（360 行）

**功能文档（4份）**
- PERSISTENCE.md - 持久化功能
- THEME_SWITCHING.md - 主题切换
- THEME_TEST.md - 主题测试
- TIME_FORMAT_FIX.md - 时间格式

**技术文档（6份）**
- docs/OCPP_IMPLEMENTATION.md - OCPP 实现详解
- docs/SCRIPT_API.md - 脚本 API 参考
- DENO_CORE_INTEGRATION.md - Deno Core 集成
- SCRIPT_ENGINE_STATUS.md - 脚本引擎状态
- SCRIPT_ENGINE_LEARNINGS.md - 脚本引擎经验
- SCRIPT_ENGINE_TEST.md - 脚本引擎测试
- OCPP_PROGRESS.md - OCPP 开发进度
- PROBLEMS_FIXED.md - 已解决问题

### 🛠️ Technical - 技术实现

**后端技术栈**
- Tauri 1.5 - 桌面应用框架
- Rust 1.70+ - 系统编程语言
- Tokio 1.40 - 异步运行时
- Deno Core 0.320 - JavaScript 引擎
- tokio-tungstenite - WebSocket 客户端
- serde/serde_json - 序列化框架
- chrono - 时间处理
- tracing - 日志系统
- parking_lot - 高性能锁
- async-trait - 异步 trait

**前端技术栈**
- Vue 3.4 - 渐进式框架
- TypeScript 5.3 - 类型支持
- Ant Design Vue 4.1 - UI 组件库
- Vite 7.1 - 构建工具
- Pinia - 状态管理

**代码结构**
- Rust 后端: 10 个模块，~3,500 行
- Vue 前端: 1 个主组件，~2,000 行
- JavaScript 脚本: 4 个预设，~400 行
- 文档: 18 份文档，~5,000 行
- **总计**: ~10,900 行代码

### 📊 Metrics - 项目指标

- **完成度**: 80%
- **代码量**: ~10,900 行
- **文件数**: 105+
- **文档数**: 18 份
- **测试覆盖**: 待完善
- **性能**: 支持数百个充电桩并发（目标 500+）

---

## [Unreleased] - 未来计划

### 🚀 v0.9 (2025-Q4) - 性能优化和测试

#### Planned - 计划中

**UI 增强 (70%)**
- [ ] 搜索和筛选功能
- [ ] 表格排序功能
- [ ] 批量选择操作
- [ ] 数据可视化图表（ECharts）
  - [ ] 状态分布饼图
  - [ ] 功率趋势图
  - [ ] 电量统计图
- [ ] 日志查看器界面
- [ ] 脚本在线编辑器（Monaco Editor）

**性能优化 (60%)**
- [ ] 虚拟滚动优化（大数据量）
- [ ] 数据更新节流/防抖
- [ ] 增量更新机制
- [ ] 内存占用优化
- [ ] 并发性能测试（目标 500+）

**云快充协议 (40%)**
- [x] 协议类型定义（YunKuaiChong）
- [x] 协议抽象接口
- [ ] HTTP/HTTPS 客户端封装
- [ ] 设备注册和心跳
- [ ] 充电启动/停止
- [ ] 订单管理

### 🎯 v1.0 (2026-Q1) - 正式版本发布

#### Planned - 计划中

**日志系统**
- [ ] 系统日志记录
- [ ] 通信日志（OCPP 消息）
- [ ] 事件日志
- [ ] 日志过滤和搜索
- [ ] 日志导出（CSV/JSON）
- [ ] 实时日志流展示

**监控仪表盘**
- [ ] 性能监控图表（CPU、内存、网络）
- [ ] 消息统计（发送/接收、类型分布、响应时间）
- [ ] 实时数据可视化

**高级特性**
- [ ] TLS/SSL 加密通信
- [ ] 证书管理
- [ ] 多语言支持（中英文）
- [ ] 插件系统
- [ ] 数据库支持（SQLite）
- [ ] 历史数据查询
- [ ] 统计报表生成

**性能目标**
- [ ] 支持 500+ 充电桩并发
- [ ] CPU < 80%（500 桩）
- [ ] 内存 < 2GB（500 桩）
- [ ] 消息响应 < 100ms
- [ ] UI 响应 < 200ms

**发布**
- [ ] Windows 安装包
- [ ] macOS 应用包
- [ ] Linux 安装包
- [ ] 完整用户手册
- [ ] 视频教程

### 🔮 v2.0 (未来) - 云端版本

#### Future - 未来展望

- [ ] 云端版本（远程控制和管理）
- [ ] 插件系统（支持自定义扩展）
- [ ] GB/T 国标协议支持
- [ ] CharIN 测试场景库
- [ ] 分布式部署支持
- [ ] Web 版本（浏览器访问）
- [ ] OCPP 2.0.1 支持

---

## 📝 Notes - 说明

### Version Naming - 版本命名

- **v0.x.x** - 开发版本
- **v1.x.x** - 稳定版本
- **v2.x.x** - 重大更新版本

### Release Cycle - 发布周期

- **Minor Release**: 每季度一次（v0.8, v0.9, v1.0）
- **Patch Release**: 按需发布（v0.8.1, v0.8.2）
- **Major Release**: 年度发布（v1.0, v2.0）

### Support - 支持

- **Current Version**: v0.8.0
- **Minimum Supported**: v0.8.0
- **End of Life**: TBD

---

**维护者**: EV Charger Simulator Team  
**最后更新**: 2025-10-02

[0.8.0]: https://github.com/yourusername/ev-charger-simulator/releases/tag/v0.8.0
[Unreleased]: https://github.com/yourusername/ev-charger-simulator/compare/v0.8.0...HEAD
