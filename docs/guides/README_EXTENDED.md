# EV Charger Simulator - 扩展文档

## 📁 项目结构

```
ev-charger-simulator/
├── src/                          # 前端源代码
│   ├── App.vue                   # 主应用组件（2000+行）
│   ├── main.ts                   # 入口文件
│   └── styles.css                # 全局样式
├── src-tauri/                    # Rust 后端代码
│   ├── src/
│   │   ├── main.rs              # 入口文件
│   │   ├── manager.rs           # 充电桩管理器
│   │   ├── charger.rs           # 充电桩实现
│   │   ├── ocpp_client.rs       # OCPP 客户端
│   │   ├── protocol.rs          # 协议定义
│   │   ├── script_engine.rs     # 脚本引擎（Deno Core）
│   │   ├── commands.rs          # Tauri 命令
│   │   ├── state.rs             # 状态定义
│   │   └── config_loader.rs     # 配置加载器
│   ├── Cargo.toml               # Rust 依赖
│   └── tauri.conf.json          # Tauri 配置
├── config/
│   └── chargers.json            # 充电桩配置文件
├── scripts/                      # JavaScript 脚本
│   ├── basic_test.js
│   ├── normal_charging.js
│   ├── fast_charging.js
│   └── fault_test.js
├── docs/                         # 文档
│   ├── SCRIPT_API.md            # 脚本 API 文档
│   ├── OCPP_IMPLEMENTATION.md   # OCPP 实现说明
│   └── UI_NEXT_STEPS.md         # UI 开发计划
├── package.json                  # Node.js 依赖
├── tsconfig.json                 # TypeScript 配置
├── vite.config.ts                # Vite 配置
└── README.md                     # 项目总览
```

---

## 📚 完整文档导航

### 核心文档
- [README.md](README.md) - 项目总览和快速开始
- [FEATURES.md](FEATURES.md) - 完整功能特性列表
- [requirements.md](requirements.md) - 项目需求和进度

### 功能文档
- [PERSISTENCE.md](PERSISTENCE.md) - 配置持久化功能说明
- [THEME_SWITCHING.md](THEME_SWITCHING.md) - 主题切换完整指南
- [THEME_TEST.md](THEME_TEST.md) - 主题切换测试清单
- [TIME_FORMAT_FIX.md](TIME_FORMAT_FIX.md) - 时间格式化修复

### 技术文档
- [TECH_STACK.md](TECH_STACK.md) - 技术栈详细说明
- [SCRIPT_ENGINE_STATUS.md](SCRIPT_ENGINE_STATUS.md) - 脚本引擎当前状态
- [SCRIPT_ENGINE_LEARNINGS.md](SCRIPT_ENGINE_LEARNINGS.md) - 脚本引擎开发笔记
- [DENO_CORE_INTEGRATION.md](DENO_CORE_INTEGRATION.md) - Deno Core 集成指南
- [OCPP_PROGRESS.md](OCPP_PROGRESS.md) - OCPP 实现进度

### API 参考
- [docs/SCRIPT_API.md](docs/SCRIPT_API.md) - JavaScript 脚本 API
- [docs/OCPP_IMPLEMENTATION.md](docs/OCPP_IMPLEMENTATION.md) - OCPP 协议实现

### 开发文档
- [SETUP.md](SETUP.md) - 开发环境搭建
- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - 项目总结
- [PROBLEMS_FIXED.md](PROBLEMS_FIXED.md) - 问题修复记录
- [SCRIPT_ENGINE_TEST.md](SCRIPT_ENGINE_TEST.md) - 脚本引擎测试

---

## 🎮 详细的脚本开发指南

### 预设脚本说明

#### 1. basic_test.js
基础功能测试脚本，验证充电桩基本操作。

```javascript
const charger = Deno.core.ops.op_get_charger();
await charger.start_charging();
await charger.set_power(30.0);
// 等待5秒
await new Promise(resolve => setTimeout(resolve, 5000));
await charger.stop_charging();
```

#### 2. normal_charging.js
模拟正常充电流程（慢充）。

**流程**：
1. 开始充电会话
2. 逐步提升功率（0 → 30kW）
3. 保持稳定充电
4. 逐步降低功率
5. 结束会话

#### 3. fast_charging.js
模拟快速充电流程（快充）。

**特点**：
- 高功率充电（最高60kW）
- 快速达到最大功率
- 充电时间短

#### 4. fault_test.js
模拟故障测试场景。

**测试内容**：
- 连接中断
- 功率异常
- 状态切换异常

### 脚本 API 参考

详见 [`docs/SCRIPT_API.md`](docs/SCRIPT_API.md)

**主要 API**：

```javascript
// 充电桩对象
const charger = Deno.core.ops.op_get_charger();

// 基础操作
await charger.start_charging();     // 开始充电
await charger.stop_charging();      // 停止充电
await charger.set_power(power);     // 设置功率

// 状态查询
const status = await charger.get_status();
const power = await charger.get_power();
const energy = await charger.get_energy();

// 日志输出
console.log("消息");
console.error("错误");
```

---

## ⚙️ 详细配置说明

### 配置文件结构

`config/chargers.json` 包含两部分配置：

#### 1. chargers 数组（逐个定义）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | string | ✅ | 唯一标识符，如 CP000001 |
| `name` | string | ✅ | 显示名称，如"1号充电桩" |
| `protocol_type` | string | ✅ | OCPP 或 YunKuaiChong |
| `server_url` | string | ✅ | WebSocket 服务器地址 |
| `max_power` | number | ✅ | 最大充电功率（kW） |
| `script_path` | string \| null | ❌ | 脚本文件路径 |
| `enabled` | boolean | ✅ | 是否启用此充电桩 |

#### 2. batch_config 对象（批量生成）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `enabled` | boolean | ✅ | 是否启用批量创建 |
| `count` | number | ✅ | 要创建的充电桩数量 |
| `protocol_type` | string | ✅ | 协议类型 |
| `server_url` | string | ✅ | 服务器地址 |
| `max_power` | number | ✅ | 最大功率 |
| `script_path` | string \| null | ❌ | 默认脚本路径 |
| `id_prefix` | string | ✅ | ID 前缀（如 "CP"） |
| `name_prefix` | string | ✅ | 名称前缀（如 "充电桩"） |
| `start_index` | number | ✅ | 起始编号 |

### 配置示例

#### 示例 1：少量充电桩（逐个定义）

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
    },
    {
      "id": "CP000002",
      "name": "2号充电桩",
      "protocol_type": "OCPP",
      "server_url": "ws://localhost:8080/ocpp",
      "max_power": 120.0,
      "script_path": "scripts/fast_charging.js",
      "enabled": true
    }
  ],
  "batch_config": {
    "enabled": false,
    "count": 0
  }
}
```

#### 示例 2：大量充电桩（批量生成）

```json
{
  "chargers": [],
  "batch_config": {
    "enabled": true,
    "count": 100,
    "protocol_type": "OCPP",
    "server_url": "ws://localhost:8080/ocpp",
    "max_power": 60.0,
    "script_path": null,
    "id_prefix": "CP",
    "name_prefix": "充电桩",
    "start_index": 1
  }
}
```

**结果**：自动创建 CP000001 到 CP000100，名称为"充电桩 #1"到"充电桩 #100"

#### 示例 3：混合模式

```json
{
  "chargers": [
    {
      "id": "SPECIAL001",
      "name": "特殊充电桩1",
      "protocol_type": "YunKuaiChong",
      "server_url": "https://api.yunkuaichong.com",
      "max_power": 90.0,
      "script_path": "scripts/custom_test.js",
      "enabled": true
    }
  ],
  "batch_config": {
    "enabled": true,
    "count": 50,
    "protocol_type": "OCPP",
    "server_url": "ws://localhost:8080/ocpp",
    "max_power": 60.0,
    "script_path": "scripts/normal_charging.js",
    "id_prefix": "CP",
    "name_prefix": "充电桩",
    "start_index": 2
  }
}
```

**结果**：
- 1个特殊配置的充电桩（SPECIAL001）
- 50个批量生成的充电桩（CP000002 到 CP000051）

### 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `CHARGER_CONFIG_PATH` | 配置文件路径 | `config/chargers.json` |

**设置方法**：

```bash
# Windows (PowerShell)
$env:CHARGER_CONFIG_PATH="D:\my-config\chargers.json"

# Linux/Mac
export CHARGER_CONFIG_PATH="/path/to/my/chargers.json"
```

---

## 🔧 故障排查

### 常见问题

#### 1. 充电桩无法连接

**症状**：充电桩状态显示"未连接"

**可能原因**：
- 服务器地址配置错误
- 服务器未启动
- 网络连接问题
- 防火墙阻止

**解决方法**：
```bash
# 检查服务器是否可访问
ping your-server.com

# 检查端口是否开放
telnet your-server.com 8080
```

#### 2. 脚本执行失败

**症状**：脚本状态显示"失败"

**可能原因**：
- 脚本语法错误
- API 调用错误
- 权限问题

**解决方法**：
- 查看脚本执行结果消息
- 检查脚本语法
- 参考预设脚本示例

#### 3. 配置未保存

**症状**：重启后配置丢失

**可能原因**：
- 配置文件权限问题
- 磁盘空间不足
- 配置目录不存在

**解决方法**：
```bash
# 检查配置目录
ls -la config/

# 创建配置目录
mkdir -p config

# 检查权限
chmod 755 config/
```

#### 4. 主题未保持

**症状**：重启后主题恢复默认

**可能原因**：
- localStorage 被清除
- 浏览器隐私模式
- 存储权限问题

**解决方法**：
- 检查浏览器设置
- 允许本地存储
- 退出隐私模式

---

## 🚀 性能优化建议

### 1. 大量充电桩场景

当模拟超过100个充电桩时：

- 使用批量配置而非逐个定义
- 关闭不必要的自动刷新
- 增加刷新间隔
- 使用分页显示

### 2. 脚本优化

- 避免密集循环
- 合理设置延时
- 及时释放资源
- 使用异步操作

### 3. UI 性能

- 启用虚拟滚动（大数据）
- 减少实时刷新频率
- 优化表格渲染
- 使用防抖/节流

---

## 📈 开发路线图

### 当前版本（v0.1.0）

- ✅ 基础充电桩管理
- ✅ OCPP 1.6J 支持
- ✅ 脚本引擎集成
- ✅ 主题切换
- ✅ 配置持久化

### 计划中（v0.2.0）

- ⏳ OCPP 2.0.1 支持
- ⏳ 更多预设脚本
- ⏳ 实时日志查看
- ⏳ 性能监控面板
- ⏳ 导入导出配置

### 未来版本

- 📋 图表可视化
- 📋 多语言支持
- 📋 插件系统
- 📋 云端同步

---

## 🤝 贡献指南

### 如何贡献

1. **Fork 本仓库**
2. **创建特性分支**
   ```bash
   git checkout -b feature/AmazingFeature
   ```
3. **提交更改**
   ```bash
   git commit -m '添加某个新功能'
   ```
4. **推送到分支**
   ```bash
   git push origin feature/AmazingFeature
   ```
5. **开启 Pull Request**

### 代码规范

#### 前端

- 使用 TypeScript
- 遵循 Vue 3 Composition API 风格
- 使用 ESLint 和 Prettier
- 组件使用单文件组件（SFC）

#### 后端

- 遵循 Rust 标准代码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码
- 添加适当的注释和文档

### 提交信息规范

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type**:
- `feat`: 新功能
- `fix`: 修复
- `docs`: 文档
- `style`: 格式
- `refactor`: 重构
- `test`: 测试
- `chore`: 构建/工具

**示例**:
```
feat(ui): 添加充电桩批量删除功能

- 添加批量选择功能
- 添加批量删除确认对话框
- 更新文档

Closes #123
```

---

## 📝 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

---

## 📞 联系方式

- **问题反馈**: [GitHub Issues](https://github.com/yourusername/ev-charger-simulator/issues)
- **功能建议**: [GitHub Discussions](https://github.com/yourusername/ev-charger-simulator/discussions)
- **邮件**: your-email@example.com

---

Made with ❤️ by the EV Charger Simulator Team
