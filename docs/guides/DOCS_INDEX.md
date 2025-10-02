# 文档导航 (DOCS_INDEX.md)

本文档提供了项目所有文档的完整导航和索引。

---

## 📚 文档目录结构

```
ev-charger-simulator/
├── README.md                        # 项目主文档
├── README_EXTENDED.md               # 扩展文档
├── FEATURES.md                      # 功能特性索引
├── DOCS_INDEX.md                    # 本文档
├── requirements.md                  # 项目需求
├── SETUP.md                         # 安装配置指南
├── TECH_STACK.md                    # 技术栈说明
├── PROJECT_SUMMARY.md               # 项目摘要
│
├── docs/                            # 详细文档目录
│   ├── OCPP_IMPLEMENTATION.md       # OCPP协议实现
│   ├── SCRIPT_API.md                # 脚本API参考
│   └── UI_NEXT_STEPS.md             # UI后续开发计划
│
├── [功能文档]                        # 功能说明文档
│   ├── PERSISTENCE.md               # 持久化功能
│   ├── THEME_SWITCHING.md           # 主题切换功能
│   ├── THEME_TEST.md                # 主题测试记录
│   └── TIME_FORMAT_FIX.md           # 时间格式修复
│
├── [技术文档]                        # 技术实现文档
│   ├── OCPP_PROGRESS.md             # OCPP开发进度
│   ├── DENO_CORE_INTEGRATION.md     # Deno Core集成
│   ├── SCRIPT_ENGINE_STATUS.md      # 脚本引擎状态
│   ├── SCRIPT_ENGINE_LEARNINGS.md   # 脚本引擎经验
│   └── SCRIPT_ENGINE_TEST.md        # 脚本引擎测试
│
└── [维护文档]                        # 问题修复文档
    └── PROBLEMS_FIXED.md            # 已解决问题记录
```

---

## 📖 文档分类索引

### 🌟 入门文档

从这里开始了解项目：

| 文档 | 说明 | 适用对象 |
|------|------|----------|
| [README.md](README.md) | 项目总览、快速开始 | 所有用户 |
| [SETUP.md](SETUP.md) | 安装和环境配置 | 开发者 |
| [FEATURES.md](FEATURES.md) | 所有功能详细说明 | 用户、开发者 |
| [requirements.md](requirements.md) | 项目需求和目标 | 产品、开发者 |

### 🔧 核心功能文档

了解各个功能的实现细节：

| 功能 | 文档 | 说明 |
|------|------|------|
| **充电桩管理** | [FEATURES.md#充电桩管理](FEATURES.md#充电桩管理) | CRUD操作、状态管理 |
| **持久化** | [PERSISTENCE.md](PERSISTENCE.md) | 配置保存和加载 |
| **主题切换** | [THEME_SWITCHING.md](THEME_SWITCHING.md) | 亮/暗模式实现 |
| **主题测试** | [THEME_TEST.md](THEME_TEST.md) | 主题功能测试记录 |
| **时间格式** | [TIME_FORMAT_FIX.md](TIME_FORMAT_FIX.md) | 时间显示格式修复 |

### 🎮 脚本引擎文档

脚本引擎相关的所有文档：

| 文档 | 说明 | 更新状态 |
|------|------|----------|
| [docs/SCRIPT_API.md](docs/SCRIPT_API.md) | 脚本API完整参考 | ✅ 最新 |
| [SCRIPT_ENGINE_STATUS.md](SCRIPT_ENGINE_STATUS.md) | 脚本引擎开发状态 | ✅ 最新 |
| [SCRIPT_ENGINE_LEARNINGS.md](SCRIPT_ENGINE_LEARNINGS.md) | 开发过程中的经验总结 | ✅ 最新 |
| [SCRIPT_ENGINE_TEST.md](SCRIPT_ENGINE_TEST.md) | 脚本引擎测试文档 | ✅ 最新 |
| [DENO_CORE_INTEGRATION.md](DENO_CORE_INTEGRATION.md) | Deno Core集成详解 | ✅ 最新 |
| [README_EXTENDED.md#脚本开发](README_EXTENDED.md#脚本开发指南) | 脚本开发指南 | ✅ 最新 |

**快速链接**:
- [预设脚本说明](README_EXTENDED.md#预设脚本说明)
- [脚本API使用示例](docs/SCRIPT_API.md)
- [脚本开发最佳实践](SCRIPT_ENGINE_LEARNINGS.md)

### 🔌 协议支持文档

OCPP和云快充协议的实现：

| 文档 | 说明 | 进度 |
|------|------|------|
| [docs/OCPP_IMPLEMENTATION.md](docs/OCPP_IMPLEMENTATION.md) | OCPP 1.6J协议完整实现 | ✅ 完成 |
| [OCPP_PROGRESS.md](OCPP_PROGRESS.md) | OCPP协议开发进度追踪 | ✅ 完成 |

**支持的OCPP消息**:
- BootNotification
- Heartbeat
- StatusNotification
- StartTransaction
- StopTransaction
- MeterValues
- Authorize
- DataTransfer

### 🎨 用户界面文档

UI相关的实现和计划：

| 文档 | 说明 | 状态 |
|------|------|------|
| [docs/UI_NEXT_STEPS.md](docs/UI_NEXT_STEPS.md) | UI后续开发计划 | 📋 计划中 |
| [THEME_SWITCHING.md](THEME_SWITCHING.md) | 主题切换实现 | ✅ 已实现 |
| [THEME_TEST.md](THEME_TEST.md) | 主题测试记录 | ✅ 已完成 |

**UI特性**:
- ✅ 主题切换（亮/暗模式）
- ✅ 响应式设计
- ✅ 实时数据刷新
- ✅ 表格分页
- ✅ 状态展开/折叠
- ✅ 窗口状态记忆

### 🛠️ 技术栈文档

技术架构和依赖：

| 文档 | 说明 |
|------|------|
| [TECH_STACK.md](TECH_STACK.md) | 完整技术栈说明 |
| [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) | 项目架构摘要 |
| [README.md#技术栈](README.md#技术栈) | 技术栈简介 |

**技术组成**:
- **前端**: Vue 3.4, TypeScript 5.3, Ant Design Vue 4.1, Vite 7.1
- **后端**: Tauri 1.5, Rust 1.70+, Tokio 1.40
- **脚本引擎**: Deno Core 0.320
- **协议**: OCPP 1.6J, YunKuaiChong

### 🔍 问题排查文档

遇到问题时查阅：

| 文档 | 说明 |
|------|------|
| [PROBLEMS_FIXED.md](PROBLEMS_FIXED.md) | 已修复的问题列表 |
| [README_EXTENDED.md#故障排除](README_EXTENDED.md#故障排除) | 常见问题和解决方案 |

**常见问题**:
1. 充电桩无法连接 → 检查服务器地址和网络
2. 脚本执行失败 → 查看脚本语法和API调用
3. 数据不持久化 → 检查文件权限
4. 主题不生效 → 清除浏览器缓存

### 📝 开发文档

开发者参考：

| 文档 | 说明 |
|------|------|
| [README_EXTENDED.md#项目结构](README_EXTENDED.md#项目结构) | 目录结构说明 |
| [README_EXTENDED.md#开发路线图](README_EXTENDED.md#开发路线图) | 开发计划 |
| [README_EXTENDED.md#贡献指南](README_EXTENDED.md#贡献指南) | 代码贡献流程 |
| [SETUP.md](SETUP.md) | 开发环境搭建 |

---

## 🎯 按使用场景查找文档

### 场景1: 首次使用项目

**阅读顺序**:
1. [README.md](README.md) - 了解项目概况
2. [SETUP.md](SETUP.md) - 安装和配置
3. [FEATURES.md](FEATURES.md) - 了解所有功能
4. [docs/SCRIPT_API.md](docs/SCRIPT_API.md) - 学习脚本编写

### 场景2: 开发OCPP功能

**参考文档**:
1. [docs/OCPP_IMPLEMENTATION.md](docs/OCPP_IMPLEMENTATION.md) - OCPP协议实现
2. [OCPP_PROGRESS.md](OCPP_PROGRESS.md) - 开发进度
3. [src-tauri/src/ocpp_client.rs](src-tauri/src/ocpp_client.rs) - 源代码

### 场景3: 编写充电脚本

**参考文档**:
1. [docs/SCRIPT_API.md](docs/SCRIPT_API.md) - API参考
2. [README_EXTENDED.md#预设脚本说明](README_EXTENDED.md#预设脚本说明) - 示例脚本
3. [SCRIPT_ENGINE_LEARNINGS.md](SCRIPT_ENGINE_LEARNINGS.md) - 最佳实践
4. [scripts/](scripts/) - 预设脚本代码

### 场景4: 修改UI界面

**参考文档**:
1. [src/App.vue](src/App.vue) - 主界面代码
2. [THEME_SWITCHING.md](THEME_SWITCHING.md) - 主题实现
3. [docs/UI_NEXT_STEPS.md](docs/UI_NEXT_STEPS.md) - UI计划

### 场景5: 添加新功能

**参考文档**:
1. [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - 项目架构
2. [TECH_STACK.md](TECH_STACK.md) - 技术栈
3. [README_EXTENDED.md#项目结构](README_EXTENDED.md#项目结构) - 代码结构
4. [README_EXTENDED.md#贡献指南](README_EXTENDED.md#贡献指南) - 代码规范

### 场景6: 排查问题

**参考文档**:
1. [PROBLEMS_FIXED.md](PROBLEMS_FIXED.md) - 已知问题
2. [README_EXTENDED.md#故障排除](README_EXTENDED.md#故障排除) - 常见问题
3. 相关功能文档（根据问题查找）

---

## 🔄 文档维护状态

| 文档 | 最后更新 | 状态 | 维护者 |
|------|---------|------|--------|
| README.md | 2025-10-02 | ✅ 最新 | 项目组 |
| README_EXTENDED.md | 2025-10-02 | ✅ 最新 | 项目组 |
| FEATURES.md | 2025-10-02 | ✅ 最新 | 项目组 |
| DOCS_INDEX.md | 2025-10-02 | ✅ 最新 | 项目组 |
| PERSISTENCE.md | 2025-10-01 | ✅ 最新 | 项目组 |
| THEME_SWITCHING.md | 2025-10-01 | ✅ 最新 | 项目组 |
| docs/OCPP_IMPLEMENTATION.md | 2025-09-30 | ✅ 最新 | 项目组 |
| docs/SCRIPT_API.md | 2025-09-30 | ✅ 最新 | 项目组 |
| SCRIPT_ENGINE_STATUS.md | 2025-09-29 | ✅ 最新 | 项目组 |

---

## 📊 文档统计

- **总文档数**: 20+ 份
- **核心文档**: 4 份
- **功能文档**: 4 份
- **技术文档**: 5 份
- **API文档**: 2 份
- **维护文档**: 1 份

---

## 🔗 外部资源

### 官方文档

- [Tauri 官方文档](https://tauri.app/)
- [Vue 3 官方文档](https://vuejs.org/)
- [Ant Design Vue](https://antdv.com/)
- [OCPP 1.6J 规范](https://www.openchargealliance.org/protocols/ocpp-16/)

### 开发工具

- [VS Code](https://code.visualstudio.com/)
- [Rust 官网](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)

---

## 💡 文档使用提示

### 1. 搜索文档

使用 VS Code 全局搜索（Ctrl+Shift+F）快速查找内容：
- 搜索功能名称
- 搜索技术术语
- 搜索错误信息

### 2. 文档链接

所有文档都使用相对路径链接，可以：
- 点击链接直接跳转
- 在编辑器中预览
- 导出为HTML/PDF

### 3. Markdown预览

在 VS Code 中：
- 按 `Ctrl+Shift+V` 预览文档
- 按 `Ctrl+K V` 并排预览

### 4. 更新文档

修改文档后请：
- 更新"最后更新"日期
- 在 PROBLEMS_FIXED.md 记录重要变更
- 更新本索引文档

---

## 📞 获取帮助

如果文档中找不到需要的信息：

1. **查看源代码**: 代码注释可能包含额外信息
2. **搜索Issues**: GitHub Issues 可能有相关讨论
3. **提交Issue**: 描述问题并请求帮助
4. **联系维护者**: 直接联系项目维护团队

---

## 📋 文档改进计划

### 近期计划
- [ ] 添加更多代码示例
- [ ] 添加视频教程
- [ ] 完善API文档
- [ ] 添加性能优化指南

### 长期计划
- [ ] 国际化（英文版本）
- [ ] 交互式文档
- [ ] 在线文档网站
- [ ] 自动生成API文档

---

**文档维护**: 本文档由项目组维护，欢迎贡献改进建议。

**最后更新**: 2025-10-02
