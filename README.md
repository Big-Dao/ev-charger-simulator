# ⚡ EV Charger Simulator (充电桩模拟器)

<div align="center">

**基于 Tauri 技术栈的电动车充电桩模拟器桌面应用**

[![Tauri](https://img.shields.io/badge/Tauri-1.5-blue.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.4-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[功能特性](#功能特性) • [快速开始](#快速开始) • [使用指南](#使用指南) • [文档](#文档)

</div>

---

## 📋 目录

- [项目简介](#项目简介)
- [核心特性](#核心特性)
- [技术栈](#技术栈)
- [快速开始](#快速开始)
- [使用指南](#使用指南)
- [配置说明](#配置说明)
- [脚本开发](#脚本开发)
- [项目结构](#项目结构)
- [文档导航](#文档导航)

---

## 🎯 项目简介

EV Charger Simulator 是一个功能强大的电动车充电桩模拟器，支持同时模拟数百个虚拟充电桩，用于：

- 🧪 **充电站系统测试** - 压力测试、功能验证
- 🔧 **开发调试** - 快速验证充电站后端逻辑
- 📊 **性能评估** - 评估系统在高负载下的表现
- 🎓 **学习研究** - 了解充电桩通信协议

### 核心优势

- ✅ **支持多协议** - OCPP 1.6J 和云快充协议
- ✅ **完整的 CRUD** - 增删改查虚拟充电桩
- ✅ **脚本自动化** - JavaScript 脚本模拟充电场景
- ✅ **数据持久化** - 配置自动保存，重启恢复
- ✅ **现代化 UI** - 亮色/暗色主题，响应式设计
- ✅ **高性能** - Rust 后端，低资源占用

---

## 🚀 核心特性

### 1. 充电桩管理

- **增删改查** - 完整的充电桩生命周期管理
- **批量创建** - 通过配置文件快速创建多个充电桩
- **实时状态** - 连接状态、充电状态、功率、电量
- **参数配置** - 协议类型、服务器地址、最大功率
- **自动持久化** - 所有操作自动保存到配置文件

### 2. 脚本引擎

- **JavaScript 支持** - 使用 Deno Core 引擎
- **预设模板** - 基础测试、正常充电、快充、故障测试
- **自动执行** - 脚本可随充电桩启动自动运行
- **状态反馈** - 实时显示脚本执行状态和结果

### 3. 协议支持

- **OCPP 1.6J** - 标准开放充电点协议
- **云快充协议** - 国内云快充平台协议
- **WebSocket** - 稳定的实时通信
- **消息追踪** - 完整的消息日志记录

### 4. 用户界面

- **🎨 主题切换** - 亮色/暗色模式，配置自动保存
- **📊 统计面板** - 总桩数、在线数、充电中、总功率
- **📋 数据表格** - 分页、排序、展开详情
- **💾 窗口记忆** - 窗口位置和大小自动保存
- **⚡ 响应式** - 适配各种屏幕尺寸

---

## 🛠️ 技术栈

### 前端

- **框架**: Vue 3.4 (Composition API)
- **语言**: TypeScript 5.3
- **UI 组件**: Ant Design Vue 4.1
- **图标**: @ant-design/icons-vue
- **构建工具**: Vite 7.1
- **路由**: Vue Router 4.2
- **状态管理**: Pinia 2.1

### 后端

- **框架**: Tauri 1.5
- **语言**: Rust 1.70+
- **异步运行时**: Tokio 1.40
- **WebSocket**: tokio-tungstenite
- **序列化**: Serde
- **脚本引擎**: Deno Core 0.320
- **时间处理**: Chrono

---

## 📦 快速开始

### 环境要求

- **Node.js** >= 16.0
- **Rust** >= 1.70
- **Tauri CLI** >= 1.5

### 安装步骤

```bash
# 1. 克隆仓库
git clone https://github.com/yourusername/ev-charger-simulator.git
cd ev-charger-simulator

# 2. 安装依赖
npm install

# 3. 开发模式运行
npm run tauri:dev

# 4. 构建应用（可选）
npm run tauri:build
```

### 首次运行

1. 应用启动后会自动加载 `config/chargers.json` 中的充电桩配置
2. 如果没有配置文件，可以通过 UI 手动添加充电桩
3. 点击"添加充电桩"按钮开始创建虚拟充电桩

---

## 📖 使用指南

### 1. 添加充电桩

**方式一：通过 UI 添加**

1. 点击"快速操作"区域的"➕ 添加充电桩"按钮
2. 填写充电桩信息：
   - ID：唯一标识符（如 CP000001）
   - 名称：显示名称（如 1号充电桩）
   - 协议：OCPP 1.6J 或云快充
   - 服务器地址：WebSocket URL
   - 最大功率：充电功率（kW）
3. 点击"确定"完成创建

**方式二：配置文件添加**

编辑 `config/chargers.json`：

```json
{
  "chargers": [
    {
      "id": "CP000001",
      "name": "1号充电桩",
      "protocol_type": "OCPP",
      "server_url": "ws://localhost:8080/ocpp",
      "max_power": 60.0,
      "script_path": null,
      "enabled": true
    }
  ]
}
```

### 2. 配置脚本

1. 点击充电桩操作栏的 <SettingOutlined /> 脚本按钮
2. 选择预设脚本或自定义：
   - **basic_test.js** - 基础功能测试
   - **normal_charging.js** - 正常充电流程
   - **fast_charging.js** - 快速充电
   - **fault_test.js** - 故障测试
3. 设置是否自动启动
4. 保存配置

### 3. 启动模拟

**单个启动**
- 点击充电桩行的 <PlayCircleOutlined /> 启动按钮

**批量启动**
- 点击"快速操作"区域的"启动所有模拟"按钮

### 4. 控制充电

**手动控制**
- <ThunderboltOutlined /> 开始充电
- <PoweroffOutlined /> 停止充电
- <ReloadOutlined /> 重置充电桩

**脚本控制**
- <CodeOutlined /> 启动脚本
- <PauseCircleOutlined /> 停止脚本

### 5. 查看状态

- **统计面板** - 实时显示总览数据
- **表格列表** - 查看所有充电桩状态
- **展开详情** - 点击行展开查看完整信息

---

## ⚙️ 配置说明

## 配置虚拟充电桩数量

应用启动时会自动读取 `config/chargers.json`（或通过环境变量 `CHARGER_CONFIG_PATH` 指向的配置文件）来初始化虚拟充电桩。文件包含两个部分：

- `chargers`：逐个定义的充电桩；
- `batch_config`：批量生成充电桩的参数，可快速创建大量虚拟站点。

示例：

```json
{
	"chargers": [
		{
			"id": "CP000001",
			"name": "1号充电桩",
			"protocol_type": "OCPP",
			"server_url": "ws://localhost:8080/ocpp",
			"max_power": 60.0,
			"script_path": "scripts/normal_charging.js",
			"enabled": true
		}
	],
	"batch_config": {
		"enabled": true,
		"count": 100,
		"protocol_type": "OCPP",
		"server_url": "ws://localhost:8080/ocpp",
		"max_power": 60.0,
		"script_path": null,
		"id_prefix": "CP",
		"name_prefix": "充电桩",
		"start_index": 4
	}
}
```

关键字段说明：

| 字段 | 说明 |
|------|------|
| `enabled` | 控制批量创建逻辑是否生效。 |
| `count` | 需要创建的虚拟充电桩数量。 |
| `id_prefix` / `start_index` | 用于生成类似 `CP000123` 的唯一编号，避免与手动配置冲突。 |
| `name_prefix` | 生成显示名称，例如 “充电桩 #10”。 |
| `script_path` | 可选，给批量创建的充电桩预设默认脚本。 |

修改配置后重新启动应用即可按新数量初始化虚拟充电桩；无需重新编译。

## 项目进度

请查看 `requirements.md` 了解详细需求和开发进度。

---

## 📂 项目结构

```
ev-charger-simulator/
├── src/                        # Vue 前端源代码
│   ├── App.vue                # 主应用组件
│   ├── main.ts                # 应用入口
│   └── styles.css             # 全局样式
├── src-tauri/                 # Tauri 后端源代码
│   ├── src/
│   │   ├── main.rs            # 主程序入口
│   │   ├── charger.rs         # 充电桩核心逻辑
│   │   ├── manager.rs         # 充电桩管理器
│   │   ├── ocpp_client.rs     # OCPP 协议客户端
│   │   ├── script_engine.rs   # Deno 脚本引擎
│   │   └── commands.rs        # Tauri 命令
│   ├── Cargo.toml             # Rust 依赖配置
│   └── tauri.conf.json        # Tauri 应用配置
├── config/                    # 配置文件目录
│   └── chargers.json          # 充电桩配置
├── scripts/                   # 预设脚本
│   ├── basic_test.js          # 基础测试
│   ├── normal_charging.js     # 正常充电
│   ├── fast_charging.js       # 快速充电
│   └── fault_test.js          # 故障测试
└── docs/                      # 文档目录
    ├── OCPP_IMPLEMENTATION.md # OCPP 实现文档
    ├── SCRIPT_API.md          # 脚本 API 文档
    └── UI_NEXT_STEPS.md       # UI 开发计划
```

---

## 📚 文档导航

我们提供了完善的文档体系：

### 核心文档
- [README.md](README.md) - 项目总览（本文档）
- [FEATURES.md](FEATURES.md) - 功能特性索引
- [DOCS_INDEX.md](DOCS_INDEX.md) - 完整文档导航
- [README_EXTENDED.md](README_EXTENDED.md) - 扩展文档

### 功能文档
- [PERSISTENCE.md](PERSISTENCE.md) - 数据持久化
- [THEME_SWITCHING.md](THEME_SWITCHING.md) - 主题切换
- [TIME_FORMAT_FIX.md](TIME_FORMAT_FIX.md) - 时间格式

### 技术文档
- [docs/OCPP_IMPLEMENTATION.md](docs/OCPP_IMPLEMENTATION.md) - OCPP 协议
- [docs/SCRIPT_API.md](docs/SCRIPT_API.md) - 脚本 API
- [DENO_CORE_INTEGRATION.md](DENO_CORE_INTEGRATION.md) - Deno Core
- [SCRIPT_ENGINE_STATUS.md](SCRIPT_ENGINE_STATUS.md) - 脚本引擎状态

**详细导航请参考**: [DOCS_INDEX.md](DOCS_INDEX.md)

---

## 🤝 贡献指南

我们欢迎各种形式的贡献！

### 贡献方式
1. 🐛 报告 Bug
2. 💡 提出新功能建议
3. 📝 改进文档
4. 💻 提交代码

### 开发流程
1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

### 代码规范
- **Rust**: 遵循 Rust 官方规范，使用 `cargo fmt` 格式化
- **TypeScript**: 遵循 ESLint 配置
- **Vue**: 遵循 Vue 3 组合式 API 最佳实践
- **Commit**: 使用语义化提交信息

详细指南请参考: [README_EXTENDED.md#贡献指南](README_EXTENDED.md#贡献指南)

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

## 🙏 致谢

感谢以下开源项目：

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Ant Design Vue](https://antdv.com/) - 企业级 UI 组件库
- [Deno Core](https://github.com/denoland/deno_core) - JavaScript 运行时
- [Tokio](https://tokio.rs/) - Rust 异步运行时
- [Open Charge Alliance](https://www.openchargealliance.org/) - OCPP 协议标准

---

## 📞 联系方式

- **Issues**: [GitHub Issues](https://github.com/yourusername/ev-charger-simulator/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/ev-charger-simulator/discussions)

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给我们一个 Star！**

Made with ❤️ by EV Charger Simulator Team

</div>
