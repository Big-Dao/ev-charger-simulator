# 前端 UI 下一步计划

## 🎯 总体目标

在现有仪表盘基础上，完善充电桩运维体验，提供脚本执行、实时监控与批量控制能力。计划拆分为三个迭代，每个迭代可以独立上线。

---

## Iteration 1 · Charger Overview

**目标**：提供可排序、可搜索的充电桩概览列表，并支持基础控制操作。

### 必做 · Overview
- [ ] 充电桩列表视图
  - 使用 `a-table` 或自定义卡片展示 `chargerId / 状态 / 当前功率 / 会话信息`
  - 支持搜索、过滤（状态、站点、功率范围）
- [ ] 批量操作入口
  - 顶部工具栏：启动全部、停止全部、刷新、导出 CSV
  - 行内操作：启动、停止、查看详情
- [ ] 详情抽屉
  - 展示最近 10 条状态变化、功率曲线占位图
  - 集成 `get_charger_state`、`start_charger`、`stop_charger` 命令

### 可选 · Overview
- [ ] 使用 `Pinia` 建立 `chargerStore`，缓存状态并定期刷新
- [ ] 在 `App.vue` 中拆分成 `ChargerOverview` 子组件

---

## Iteration 2 · Script Workspace

**目标**：让运维工程师在应用内编写、调试、运行脚本。

### 必做 · Script Workspace
- [ ] 脚本管理抽屉
  - 列出 `/scripts` 目录下的示例脚本（通过 Tauri FS API）
  - 支持新建、重命名、删除
- [ ] Monaco Editor 集成
  - TypeScript 提示（基于脚本 API 定义）
  - 自动保存、格式化
- [ ] 执行控制区
  - 运行、停止、查看日志按钮
  - 展示 `is_script_running` 状态
- [ ] 输出面板
  - 实时显示脚本 `console.log`、异常信息
  - 支持关键词过滤、导出日志

### 可选 · Script Workspace
- [ ] 预置脚本模板（正常充电、快充、故障注入）
- [ ] `diff` 视图：对比修改前后脚本

---

## Iteration 3 · Monitoring & Analytics

**目标**：提供实时监控和历史数据分析能力。

### 必做 · Monitoring
- [ ] 实时功率图表
  - 选中一个或多个桩，展示功率/电量曲线（ECharts）
  - 支持实时刷新与回放模式
- [ ] 告警中心
  - 展示故障列表、告警级别、处理状态
  - 支持筛选、批量确认
- [ ] 运行报表
  - 显示充电次数、平均充电时长、能量统计
  - 支持导出 PDF / Excel

### 可选 · Monitoring
- [ ] Ops Dashboard 多屏模式（电视墙）
- [ ] 自定义 KPI 卡片（可配置指标）

---

## 依赖与技术建议

- 组件库：继续使用 Ant Design Vue，图表推荐 ECharts（`vue-echarts` 封装）
- 状态管理：Pinia + 持久化（可选 `pinia-plugin-persistedstate`）
- 布局：依据迭代拆分生成单独路由页面，便于维护
- 调试工具：在开发环境启用 `vite-plugin-inspect` 以分析组件依赖

---

## 下一步行动清单

1. 定义 `chargerStore` 状态结构（列表、筛选参数、活跃选中项）
2. 设计 `ChargerOverview` 组件原型图（Figma/Excalidraw）
3. 拆分 `App.vue` 为布局、概览模块、脚本控制模块
4. 验证 Tauri FS `readDir` `readTextFile` 等权限配置，为脚本管理做准备
