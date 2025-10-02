# JavaScript 脚本引擎集成说明

## 📋 当前状态

### ✅ 已完成
1. **脚本引擎设计** - 完整的 API 设计和架构
2. **示例脚本更新** - scripts/ 目录下的脚本已更新为可用格式
3. **Tauri Commands** - 脚本执行、停止、状态查询命令已定义
4. **代码框架** - script_engine.rs 已创建（但被注释）

### ⏸️ 暂停状态
虽然 Git for Windows 已安装，`patch` 工具可用，但 `rquickjs` 库在 Tauri 的多线程环境中存在以下**技术限制**：

1. **Runtime 不支持 Send + Sync**
   - QuickJS 的 Runtime 无法在线程间安全共享
   - Tauri 的 `.manage()` 要求状态必须是 `Send + Sync + 'static`
   
2. **JavaScript 函数无法直接调用异步 Rust 函数**
   - rquickjs 的 Function::new() 不支持捕获 tokio Runtime
   - 无法在 JS 中直接调用需要 `async/await` 的充电桩管理 API
   
3. **设计复杂度过高**
   - 尝试使用独立线程 + 消息传递解决 Send/Sync 问题
   - 闭包捕获和生命周期管理变得极其复杂
   - 代码可维护性很低

### 🔄 替代方案

#### 方案 1: 使用 deno_core（推荐）
- **优点**: V8 引擎，性能强，支持异步操作，社区活跃
- **缺点**: 依赖较大（~20MB），编译时间长
- **适用场景**: 需要完整 JavaScript 生态和高性能

#### 方案 2: 使用 boa_engine
- **优点**: 纯 Rust 实现，无需外部依赖，支持 async
- **缺点**: 性能较 V8 低，ES 标准支持不完整
- **适用场景**: 简单脚本，不需要复杂 JS 特性

#### 方案 3: 简化脚本引擎设计
- 使用**JSON-based DSL**代替 JavaScript
- 定义充电流程为配置文件，而非可执行脚本
- 例如:
  ```json
  {
    "steps": [
      {"action": "startCharger", "chargerId": "CP001"},
      {"action": "sleep", "duration": 2000},
      {"action": "startCharging", "chargerId": "CP001"},
      {"action": "setPower", "chargerId": "CP001", "power": 30.0}
    ]
  }
  ```
  
#### 方案 4: 前端脚本（推荐短期方案）
- 在 Vue 前端编写 TypeScript/JavaScript 脚本
- 通过 Tauri IPC 调用后端命令
- 无需在 Rust 中集成 JS 引擎

## 📁 脚本引擎实现

### API 设计

#### Console API
```javascript
console.log(message)  // 打印日志到 Rust 日志系统
```

#### Utility API
```javascript
sleep(milliseconds)   // 异步睡眠
shouldStop()          // 检查脚本是否应该停止
```

#### Charger Control API
```javascript
// 充电桩控制
charger.startCharger(chargerId)     // 启动充电桩
charger.stopCharger(chargerId)      // 停止充电桩
charger.startCharging(chargerId)    // 开始充电
charger.stopCharging(chargerId)     // 停止充电
charger.setPower(chargerId, power)  // 设置功率 (kW)
charger.getChargerState(chargerId)  // 获取状态 (返回 JSON)
```

### 示例脚本

#### 正常充电流程 (scripts/normal_charging.js)
```javascript
const CHARGER_ID = "CP000001";
const TARGET_POWER = 30.0;

// 1. 启动充电桩
console.log("启动充电桩...");
charger.startCharger(CHARGER_ID);
sleep(2000);

// 2. 开始充电
console.log("开始充电...");
charger.startCharging(CHARGER_ID);
sleep(2000);

// 3. 功率爬坡
let currentPower = 0;
while (currentPower < TARGET_POWER && !shouldStop()) {
    currentPower += 5.0;
    if (currentPower > TARGET_POWER) {
        currentPower = TARGET_POWER;
    }
    charger.setPower(CHARGER_ID, currentPower);
    sleep(2000);
}

// 4. 持续充电...
// 5. 停止充电...
```

### Tauri Commands

```rust
// 执行脚本
#[tauri::command]
pub async fn execute_script(
    script_id: String,
    script_code: String,
    engine: State<'_, ScriptEngine>,
) -> Result<ScriptResult, String>

// 停止脚本
#[tauri::command]
pub async fn stop_script(
    script_id: String,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String>

// 检查脚本运行状态
#[tauri::command]
pub async fn is_script_running(
    script_id: String,
    engine: State<'_, ScriptEngine>,
) -> Result<bool, String>
```

### 前端调用示例

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 读取脚本文件
const scriptCode = await readTextFile('scripts/normal_charging.js');

// 执行脚本
const result = await invoke('execute_script', {
  scriptId: 'charging_001',
  scriptCode: scriptCode
});

console.log('脚本执行结果:', result);

// 停止脚本
await invoke('stop_script', {
  scriptId: 'charging_001'
});

// 检查状态
const isRunning = await invoke('is_script_running', {
  scriptId: 'charging_001'
});
```

## 🔧 启用脚本引擎的步骤

### 1. 安装 Git for Windows
下载并安装: https://git-scm.com/download/win

### 2. 添加 patch 到 PATH
```powershell
$env:PATH += ";C:\Program Files\Git\usr\bin"
```

### 3. 取消代码注释

**Cargo.toml:**
```toml
rquickjs = { version = "0.6", features = ["loader", "array-buffer", "classes"] }
```

**main.rs:**
```rust
mod script_engine;
use std::sync::Arc;

let manager = Arc::new(manager::ChargerManager::new());
let script_engine = script_engine::ScriptEngine::new(manager.clone())
    .expect("Failed to initialize script engine");

// 在 invoke_handler 中添加:
commands::execute_script,
commands::stop_script,
commands::is_script_running,

// 在 manage 中添加:
.manage(script_engine)
```

**commands.rs:**
```rust
use crate::script_engine::{ScriptEngine, ScriptResult};

// 取消注释三个脚本命令函数
```

### 4. 重新编译
```powershell
cd src-tauri
cargo build
```

## 📝 注意事项

1. **线程安全**: 脚本引擎使用 `Arc<RwLock<HashMap>>` 管理运行中的脚本
2. **异步支持**: 脚本中的 `sleep()` 和 API 调用都是异步的
3. **错误处理**: 所有 API 调用都有 try-catch 保护
4. **脚本停止**: 通过 `shouldStop()` 检查来实现优雅停止
5. **资源管理**: 脚本执行完成后自动清理资源

## 🚀 未来改进

1. **脚本沙箱**: 限制脚本访问权限
2. **脚本调试**: 支持断点和单步执行
3. **脚本库**: 内置常用函数库
4. **脚本市场**: 分享和下载脚本
5. **脚本调度**: 定时执行和任务队列

## 📚 相关文档

- [QuickJS 官方文档](https://bellard.org/quickjs/)
- [rquickjs GitHub](https://github.com/DelSkayn/rquickjs)
- [docs/SCRIPT_API.md](docs/SCRIPT_API.md) - 详细 API 文档
