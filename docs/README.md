# 📚 文档目录

本目录包含项目的所有详细文档，按类型分类组织。

## 📁 目录结构

```
docs/
├── README.md                    # 本文件 - 文档索引
├── OCPP_IMPLEMENTATION.md       # OCPP 协议实现文档
├── SCRIPT_API.md                # 脚本 API 文档
├── UI_NEXT_STEPS.md             # UI 开发计划
├── releases/                    # 版本发布文档
├── fixes/                       # 问题修复文档
└── guides/                      # 开发指南文档
```

---

## 🚀 版本发布文档 (`releases/`)

存放每个版本的发布相关文档：

| 文档 | 说明 | 适用人群 |
|------|------|---------|
| `GITHUB_RELEASE_v0.8.3.md` | v0.8.3 完整发布说明 | 用户、运营 |
| `RELEASE_SUMMARY_v0.8.3.md` | v0.8.3 发布总结 | 开发团队 |
| `RELEASE_CHECKLIST_v0.8.3.md` | v0.8.3 发布检查清单 | 发布负责人 |
| `GITHUB_RELEASE_GUIDE.md` | GitHub 发布操作指南 | 所有发布者 |
| `QUICK_RELEASE_GUIDE.md` | 快速发布参考卡片 | 所有发布者 |

**命名规范**：
- 版本特定文档：`<TYPE>_v<VERSION>.md`
- 通用指南：`<TYPE>_GUIDE.md`

---

## 🔧 问题修复文档 (`fixes/`)

存放重要问题的修复记录：

| 文档 | 问题 | 版本 | 优先级 |
|------|------|------|--------|
| `PRESET_SCRIPT_FIX.md` | 预设脚本加载失败 | v0.8.3 | 🔴 高 |
| `PRESET_SCRIPT_FIX_SUMMARY.md` | 预设脚本修复总结 | v0.8.3 | 🔴 高 |
| `TEST_PRESET_SCRIPT.md` | 预设脚本测试指南 | v0.8.3 | 🟡 中 |

**命名规范**：
- 技术文档：`<FEATURE>_FIX.md`
- 总结文档：`<FEATURE>_FIX_SUMMARY.md`
- 测试文档：`TEST_<FEATURE>.md`

**每个修复文档应包含**：
1. 问题描述
2. 根因分析
3. 解决方案
4. 测试方法
5. 相关版本

---

## 📖 开发指南文档 (`guides/`)

存放技术实现和开发相关的指南：

### 架构与技术栈
- `TECH_STACK.md` - 技术栈说明
- `DENO_CORE_INTEGRATION.md` - Deno Core 集成说明

### 功能模块
- `OCPP_PROGRESS.md` - OCPP 实现进度
- `SCRIPT_ENGINE_STATUS.md` - 脚本引擎状态
- `SCRIPT_ENGINE_LEARNINGS.md` - 脚本引擎经验总结
- `SCRIPT_ENGINE_TEST.md` - 脚本引擎测试

### 开发环境
- `SETUP.md` - 开发环境搭建指南
- `PROBLEMS_FIXED.md` - 已修复问题汇总

---

## 📋 根目录保留文档

以下文档保留在项目根目录，因为它们是标准项目文档：

| 文档 | 说明 | 原因 |
|------|------|------|
| `README.md` | 项目主文档 | GitHub 默认入口 |
| `CHANGELOG.md` | 版本变更日志 | 标准实践 |
| `PROJECT_SUMMARY.md` | 项目概览 | 快速了解项目 |
| `requirements.md` | 需求文档 | 产品规划 |

---

## 🎯 文档使用指南

### 对于新开发者
1. 先读 `README.md` - 了解项目
2. 再读 `PROJECT_SUMMARY.md` - 理解架构
3. 然后读 `docs/guides/SETUP.md` - 搭建环境
4. 最后读 `docs/guides/TECH_STACK.md` - 熟悉技术栈

### 对于维护者
- 修复问题后 → 在 `docs/fixes/` 创建文档
- 发布版本前 → 查看 `docs/releases/GITHUB_RELEASE_GUIDE.md`
- 需要参考 → 查看 `docs/fixes/` 的历史修复

### 对于用户
- 查看版本说明 → `CHANGELOG.md`
- 详细发布信息 → `docs/releases/GITHUB_RELEASE_v<VERSION>.md`

---

## 📝 文档创建规范

### 何时创建文档？
- ✅ 重要功能实现
- ✅ 复杂问题修复（调查超过 1 小时）
- ✅ 架构级别变更
- ✅ 用户可见的重大更新
- ❌ 简单 bug 修复（仅更新 CHANGELOG）
- ❌ 代码格式调整

### 文档分类决策树
```
是否与版本发布相关？
├─ 是 → docs/releases/
└─ 否 → 是否是问题修复？
        ├─ 是 → docs/fixes/
        └─ 否 → docs/guides/
```

### 文档模板

#### 修复文档模板 (`docs/fixes/`)
```markdown
# <功能名称> 修复

## 问题描述
[用户体验到的问题]

## 根因分析
[技术原因]

## 解决方案
[实现方案]

## 测试验证
[如何测试]

## 相关版本
- 问题版本: v<X.Y.Z>
- 修复版本: v<X.Y.Z>
```

#### 发布文档模板 (`docs/releases/`)
```markdown
# v<X.Y.Z> Release Notes

## 主要更新
[核心功能]

## 修复问题
[Bug 列表]

## 下载链接
[安装包]

## 升级指南
[升级步骤]
```

---

## 🔍 快速查找

### 按问题类型查找
- 预设脚本问题 → `docs/fixes/PRESET_SCRIPT_FIX.md`
- 发布流程 → `docs/releases/GITHUB_RELEASE_GUIDE.md`
- 环境搭建 → `docs/guides/SETUP.md`

### 按版本查找
- v0.8.3 → `docs/releases/*_v0.8.3.md` + `docs/fixes/PRESET_SCRIPT_FIX.md`

---

## 🎓 文档维护

### 定期清理（每季度）
1. 归档 6 个月前的发布文档
2. 合并重复的指南文档
3. 更新过时的技术文档
4. 删除不再适用的修复文档

### 归档规则
```
docs/
└── archive/
    └── 2025-Q1/
        ├── releases/
        └── fixes/
```

---

**最后更新**: 2025-10-02  
**维护者**: @Big-Dao  
**文档版本**: 1.0
