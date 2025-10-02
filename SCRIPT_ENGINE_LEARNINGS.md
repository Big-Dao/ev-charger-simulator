# 脚本引擎实现总结

## 问题陈述

在尝试为 EV 充电桩模拟器集成 JavaScript 脚本引擎时，遇到了 **rquickjs** 库在 Tauri 多线程环境中的技术限制。

## 尝试过的解决方案

### 1. 初始方案 - 直接集成
**问题**: Windows 缺少 `patch` 工具导致 rquickjs 编译失败

**解决**: 安装 Git for Windows（自带 patch.exe）

**结果**: ❌ 成功编译 rquickjs，但遇到更深层的架构问题

### 2. 使用消息传递解决线程安全
**问题**: QuickJS Runtime 不实现 `Send + Sync`

**尝试**: 
- 创建独立线程运行 QuickJS Runtime
- 使用 tokio channel 进行消息传递
- ScriptEngine 只持有 channel sender

**结果**: ❌ JavaScript 函数无法捕获 tokio::Runtime 来调用异步 Rust API

### 3. 简化闭包设计
**问题**: rquickjs 的 `Function::new()` 不支持捕获复杂的运行时状态

**尝试**: 使用 `tokio::runtime::Handle::current()` 在闭包中调用异步函数

**结果**: ❌ 生命周期和类型推断错误，代码变得极其复杂

## 根本原因分析

| 问题 | 说明 | 影响 |
|------|------|------|
| **Runtime 非线程安全** | `rquickjs::Runtime` 未实现 `Send + Sync` | 无法在 Tauri 的多线程环境中使用 `.manage()` |
| **同步vs异步冲突** | QuickJS 是同步引擎，Rust API 是异步的 | 无法在 JS 中直接调用 `async fn` |
| **闭包捕获限制** | rquickjs 对闭包有严格的生命周期要求 | 无法传递复杂的状态（如 Arc<Manager>） |

## 技术教训

### ✅ 学到的经验
1. **选型要考虑架构兼容性**: 不是所有 Rust 库都适合 Tauri 的多线程模型
2. **同步异步混用要谨慎**: QuickJS 是同步的，与 tokio 异步生态不兼容
3. **复杂度爆炸点**: 使用消息传递解决线程安全后，闭包捕获变成新瓶颈

### ❌ 不可行的路径
1. 在多线程环境中直接使用 rquickjs
2. 在 QuickJS 函数中调用需要 tokio Runtime 的异步代码
3. 通过包装层解决本质的架构不兼容

## 推荐方案

### 🥇 方案 1: 前端脚本（立即可用）
**实现**: 在 Vue 前端用 TypeScript 编写脚本，通过 Tauri IPC 调用后端

```typescript
// 前端脚本示例
async function normalCharging(chargerId: string) {
  await invoke('start_charger', { chargerId });
  await sleep(2000);
  await invoke('start_charging', { chargerId, connectorId: 1 });
  
  for (let power = 0; power <= 30; power += 5) {
    await invoke('set_power', { chargerId, connectorId: 1, powerKw: power });
    await sleep(2000);
  }
}
```

**优点**:
- ✅ 无需额外依赖
- ✅ TypeScript 类型安全
- ✅ 可以使用 Vue 响应式更新 UI
- ✅ 开发者工具调试

**缺点**:
- ❌ 脚本必须在前端运行，无法独立于UI

### 🥈 方案 2: JSON-based DSL
**实现**: 定义简单的 JSON 格式描述充电流程

```json
{
  "name": "正常充电流程",
  "steps": [
    { "action": "start_charger", "charger_id": "CP001" },
    { "action": "sleep", "duration_ms": 2000 },
    { "action": "start_charging", "charger_id": "CP001" },
    { 
      "action": "ramp_power",
      "charger_id": "CP001",
      "from": 0,
      "to": 30,
      "step": 5,
      "interval_ms": 2000
    }
  ]
}
```

**优点**:
- ✅ 纯 Rust 实现，无外部依赖
- ✅ 简单易懂，易于验证
- ✅ 可以图形化编辑器生成

**缺点**:
- ❌ 表达能力有限
- ❌ 无法实现复杂逻辑（条件、循环）

### 🥉 方案 3: deno_core（完整方案）
**实现**: 使用 Deno 的 V8 引擎绑定，完整支持异步

```rust
use deno_core::{JsRuntime, RuntimeOptions};

// V8 原生支持 async/await
let mut runtime = JsRuntime::new(RuntimeOptions::default());

runtime.execute_script(
    "test.js",
    r#"
    async function main() {
        await Deno.core.ops.start_charger("CP001");
        await Deno.core.ops.sleep(2000);
    }
    main();
    "#,
)?;
```

**优点**:
- ✅ 完整的 JavaScript 引擎（V8）
- ✅ 原生支持异步操作
- ✅ 强大的生态和社区支持

**缺点**:
- ❌ 依赖体积大（~20MB）
- ❌ 编译时间长
- ❌ 集成复杂度较高

## 后续行动建议

1. **短期**（本周）：
   - 采用**方案 1**（前端脚本）
   - 在 Vue 中创建脚本编辑器组件
   - 实现基本的脚本执行和调度

2. **中期**（下个月）：
   - 如果需要后端脚本，实现**方案 2**（JSON DSL）
   - 提供可视化流程编辑器

3. **长期**（有需要时）：
   - 评估是否真的需要完整的 JS 引擎
   - 如果需要，投入时间集成**方案 3**（deno_core）

## 代码保留

所有脚本引擎相关代码已被注释但保留在代码库中：

- `src-tauri/src/script_engine.rs` - 完整实现（已注释）
- `scripts/*.js` - 示例脚本
- `commands.rs` - Tauri 命令（已注释）

如果未来决定使用 deno_core 或其他引擎，可以参考这些设计。

## 结论

虽然 rquickjs 编译问题已解决，但其架构与 Tauri 的多线程模型**根本不兼容**。建议采用前端脚本方案，既简单又实用，满足当前需求。
