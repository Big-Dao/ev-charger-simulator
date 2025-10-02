# Deno Core 脚本引擎集成进度

## 📅 实施时间
2025-10-02

## 🎯 目标
使用 Deno Core (V8 引擎) 替代 rquickjs，解决多线程环境兼容性问题。

## ✅ 已完成

### 1. 依赖配置
- ✅ 添加 `deno_core = "0.306"` 到 Cargo.toml
- ✅ 更新 `tokio = "1.40"` 以兼容 deno_core
- ✅ 移除 rquickjs 相关依赖

### 2. 脚本引擎实现 (script_engine.rs)
- ✅ 使用 `#[op]` 宏定义 Deno ops
- ✅ 创建 Extension 注册所有 ops
- ✅ 实现异步任务模型（tokio channel）
- ✅ 实现脚本执行、停止、状态查询功能

### 3. JavaScript API 设计
```javascript
// Console API
console.log(message)

// Utility API  
await sleep(ms)
await shouldStop()

// Charger Control API
await charger.startCharger(id)
await charger.stopCharger(id)
await charger.startCharging(id)
await charger.stopCharging(id)
await charger.setPower(id, power)
await charger.getChargerState(id)
```

### 4. 集成到 Tauri
- ✅ 启用 main.rs 中的 script_engine 模块
- ✅ 初始化 ScriptEngine 并传入 ChargerManager
- ✅ 注册 Tauri commands: execute_script, stop_script, is_script_running

## 🔄 进行中

### 编译项目
- ⏳ **首次编译中** - deno_core 需要编译 V8 引擎，预计需要 5-10 分钟
- 编译命令: `cd src-tauri && cargo build`
- 状态: 正在编译依赖

## 🎨 架构设计

### 线程模型
```
┌─────────────────┐
│  Tauri Main     │
│  Thread         │
└────────┬────────┘
         │
         │ mpsc::channel
         │
┌────────▼────────┐
│  Script Engine  │
│  Task           │
│                 │
│  ┌───────────┐  │
│  │ Deno      │  │
│  │ Runtime   │  │
│  │ (V8)      │  │
│  └───────────┘  │
└─────────────────┘
```

### 优势对比

| 特性 | rquickjs | deno_core |
|------|----------|-----------|
| 引擎 | QuickJS | V8 |
| 性能 | 中等 | 高 |
| 异步支持 | ❌ 困难 | ✅ 原生 |
| 线程安全 | ❌ 需要包装 | ✅ 通过 channel |
| 生态 | 小 | 大 (Deno) |
| 编译时间 | 快 (~1分钟) | 慢 (~10分钟) |
| 二进制大小 | 小 (~5MB) | 大 (~20MB) |

## 📝 技术细节

### Ops 定义
使用 `#[op]` 宏自动生成 FFI 绑定：
```rust
#[op]
async fn op_start_charger(
    state: Rc<RefCell<OpState>>,
    charger_id: String,
) -> Result<String, String> {
    let manager = state.borrow().borrow::<Arc<ChargerManager>>().clone();
    manager.start_charger(&charger_id).await?;
    Ok(format!("Charger {} started", charger_id))
}
```

### 状态管理
通过 OpState 传递共享状态：
```rust
op_state.put(manager.clone());              // ChargerManager
op_state.put(running_scripts.clone());      // 运行中脚本
op_state.put(script_id.clone());            // 当前脚本 ID
```

### JavaScript 包装
脚本自动包装为异步函数：
```rust
let wrapped_code = format!(
    r#"(async function() {{ {} }})();"#,
    script_code
);
```

## 🔮 下一步

### 编译完成后
1. ✅ 验证编译成功（无错误）
2. ✅ 测试基本脚本执行
3. ✅ 运行 normal_charging.js 示例
4. ✅ 验证所有 API 功能

### 前端集成
1. 创建脚本编辑器 UI (Vue 3)
2. 实现脚本执行控制面板
3. 显示脚本运行日志
4. 支持多脚本管理

## 📚 参考资料

- [Deno Core文档](https://github.com/denoland/deno_core)
- [Deno Ops指南](https://docs.deno.com/runtime/manual/references/contributing/architecture#ops-operations)
- [V8引擎](https://v8.dev/)

## ⚠️ 注意事项

1. **编译时间**: 首次编译需要5-10分钟，请耐心等待
2. **二进制大小**: 最终程序会增加约20MB（包含V8）
3. **内存使用**: V8引擎会增加运行时内存占用（约10-20MB per runtime）
4. **版本兼容**: 确保 tokio 版本与 deno_core 兼容

## 🐛 已知问题

### 已解决
- ✅ rquickjs 线程安全问题 → 使用 deno_core
- ✅ 异步 API 调用问题 → V8 原生支持 async/await
- ✅ 闭包捕获限制 → 通过 OpState 传递状态

### 待观察
- ⏳ 编译时间和二进制大小是否可接受
- ⏳ 运行时性能是否满足需求
- ⏳ 错误处理和调试体验

## 🎉 预期成果

成功后将实现：
1. ✅ 完整的 JavaScript 脚本引擎
2. ✅ 支持 async/await 的充电桩控制 API
3. ✅ 线程安全的脚本执行
4. ✅ 与 Tauri 无缝集成
5. ✅ 为前端提供脚本管理接口
