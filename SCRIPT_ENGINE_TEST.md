# 脚本引擎测试指南

## 🎉 成功集成 deno_core

经过多次调试和版本升级，我们成功将 deno_core 0.320 集成到了 Tauri 应用中！

## ✅ 已完成

1. **编译成功** - 所有代码编译通过，只有少量警告（未使用的方法）
2. **应用启动** - Tauri 应用成功运行在 <http://localhost:1420/>
3. **脚本引擎初始化** - V8 引擎已在独立线程中运行
4. **测试脚本** - 创建了 `scripts/basic_test.js` 来测试所有 9 个 ops
5. **UI 集成** - 在主界面添加了"测试脚本引擎"按钮

## 🧪 如何测试

### 方法一：使用 UI 按钮（推荐）

1. 通过 `npm run tauri dev` 启动应用，等待 **Tauri 桌面窗口** 打开。
  Vite 仍监听 <http://localhost:1420/> 用于热更新。
2. 在桌面窗口中点击主界面的 **"测试脚本引擎"** 按钮。
3. 观察以下输出：
   - 界面上会提示当前测试进度
   - 按 `Ctrl+Shift+I`（macOS 使用 `Cmd+Option+I`）打开 Tauri 窗口的开发者工具，查看 Console 标签页
   - 脚本的所有 `console.log` 输出会显示在该 Console 内

### 方法二：使用浏览器控制台

> ⚠ **注意**：此方式必须在 **Tauri 窗口** 的开发者工具 Console 中执行，无法在普通浏览器标签页运行。

打开 Tauri 窗口的开发者工具（`Ctrl+Shift+I` / `Cmd+Option+I`），在 Console 中运行：

```javascript
// 简单测试
await window.__TAURI__.invoke('execute_script', {
  scriptId: 'test1',
  scriptCode: `
    console.log("Hello from script engine!");
    await sleep(1000);
    console.log("Sleep works!");
  `
});

// 测试充电桩控制
await window.__TAURI__.invoke('execute_script', {
  scriptId: 'test2',
  scriptCode: `
    await charger.startCharger("test-001", "ws://localhost:8080", "OCPP1.6J");
    console.log("Charger started");
    const state = await charger.getChargerState("test-001");
    console.log("State:", JSON.stringify(state));
  `
});

// 停止脚本
await window.__TAURI__.invoke('stop_script', {
  scriptId: 'test1'
});

// 检查脚本是否运行
await window.__TAURI__.invoke('is_script_running', {
  scriptId: 'test1'
});
```

## 📋 测试脚本说明

`scripts/basic_test.js` 测试以下功能：

### 1. 基础 API
- ✅ `console.log()` - 日志输出
- ✅ `sleep(ms)` - 异步延迟
- ✅ `shouldStop()` - 检查是否应停止

### 2. 充电桩控制 API
- ✅ `charger.startCharger(id, url, protocol)` - 启动充电桩
- ✅ `charger.stopCharger(id)` - 停止充电桩
- ✅ `charger.getChargerState(id)` - 获取状态

### 3. 充电会话 API
- ✅ `charger.startCharging(id)` - 开始充电
- ✅ `charger.stopCharging(id)` - 停止充电
- ✅ `charger.setPower(id, power)` - 设置功率

## 📝 预期输出

如果一切正常，你应该看到类似以下的输出：

```
=== 脚本引擎测试开始 ===
Script engine test started
✓ Test 1: console.log works!
Test 2: Testing sleep...
✓ Test 2: sleep(1000ms) completed!
Test 3: Checking shouldStop...
✓ Test 3: shouldStop() returns false (script running)
Test 4: Testing startCharger...
⚠ Test 4: startCharger failed (expected if no OCPP server): ...
Test 5: Testing getChargerState...
✓ Test 5: getChargerState succeeded!
Charger state: { ... }
... (更多测试输出)
=== 测试完成 ===
All tests completed!
Script engine is working properly! 🎉
```

## 🔍 故障排除

### 问题：点击按钮没有反应
- 打开开发者工具查看是否有错误信息
- 检查文件路径是否正确（`scripts/basic_test.js`）

### 问题：脚本执行错误
- 查看 Rust 端日志（终端输出）
- 检查脚本语法是否正确

### 问题：脚本没有任何输出
- 确认测试按钮位于 Tauri 窗口中执行，而非普通浏览器
- 检查 `App.vue` 是否仍然导入 `../scripts/basic_test.js?raw`
- 确认热更新完成：终端中应看到 `VITE ready` 与 `Starting EV Charger Simulator...`

## 🎯 下一步计划

1. **完善 UI**
   - 创建脚本编辑器（Monaco Editor）
   - 实时日志显示面板
   - 脚本管理（保存、加载、删除）
   - 多脚本并发执行

2. **功能增强**
   - 更多 API：获取充电桩列表、批量操作等
   - 脚本调试功能
   - 性能监控
   - 错误处理和恢复

3. **文档完善**
   - JavaScript API 完整文档
   - 示例脚本库
   - 最佳实践指南

## 🚀 技术栈

- **前端**: Vue 3 + TypeScript + Vite + Ant Design Vue
- **后端**: Rust + Tauri 1.5 + Tokio 1.40
- **脚本引擎**: deno_core 0.320 (V8 JavaScript Engine)
- **协议**: OCPP 1.6J

## 📚 相关文档

- `DENO_CORE_INTEGRATION.md` - deno_core 集成详细文档
- `SCRIPT_ENGINE_LEARNINGS.md` - rquickjs 失败经验总结
- `SCRIPT_API.md` - JavaScript API 文档（如果存在）
- `scripts/normal_charging.js` - 正常充电示例脚本

---

**状态**: ✅ 编译成功，应用正常运行，等待功能测试
**最后更新**: 2025-01-02
