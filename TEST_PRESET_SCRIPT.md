# 测试预设脚本加载功能

## 测试步骤

### 1. 启动应用

```powershell
# 开发模式
npm run tauri:dev

# 或生产模式
.\src-tauri\target\release\ev-charger-simulator.exe
```

### 2. 测试预设脚本加载

1. **打开脚本配置**
   - 点击"添加充电桩"按钮
   - 填写充电桩信息：
     - ID: `TEST001`
     - 服务器 URL: `ws://localhost:8080/ocpp`
     - 协议: `OCPP 1.6J`
   - 切换到"脚本配置"标签页

2. **测试选择预设脚本**
   - 点击"选择预设脚本"下拉菜单
   - 应该看到 4 个预设脚本：
     - `basic_test.js - 基础测试脚本 - 验证脚本引擎的所有功能`
     - `normal_charging.js - 正常充电流程 - 模拟完整的充电流程`
     - `fast_charging.js - 快速充电流程 - 模拟快充场景`
     - `fault_test.js - 故障测试脚本 - 模拟充电桩故障情况`

3. **选择 basic_test.js**
   - 点击选择 `basic_test.js`
   - 应该看到成功提示：`已加载脚本: basic_test.js`
   - 脚本代码编辑器应该自动填充脚本内容
   - 脚本名称应该自动填充为：`basic_test.js`

4. **验证脚本内容**
   - 检查脚本代码编辑器中的内容
   - 应该包含以下内容：
     ```javascript
     // 基础测试脚本 - 验证脚本引擎的所有功能
     // Basic test script - Verify all script engine features
     
     console.log("=== 脚本引擎测试开始 ===");
     ```

5. **测试其他预设脚本**
   - 依次选择其他预设脚本
   - 验证每个脚本都能正确加载
   - 验证没有"加载预设脚本失败"的错误提示

### 3. 测试保存和运行

1. **保存脚本**
   - 选择一个预设脚本（如 `basic_test.js`）
   - 勾选"自动随充电桩启动脚本"
   - 点击"确定"保存
   - 应该看到成功提示

2. **运行脚本**
   - 点击充电桩操作列的"启动"按钮
   - 脚本应该自动开始运行
   - 检查"脚本状态"列显示为"运行中"
   - 点击"停止"按钮停止脚本

## 预期结果

### ✅ 成功标准

1. **下拉菜单正常**
   - 预设脚本列表正确显示
   - 包含 4 个预设脚本选项
   - 显示脚本名称和描述

2. **脚本加载成功**
   - 选择预设脚本后立即加载
   - 显示成功提示消息
   - 脚本内容正确填充到编辑器
   - 脚本名称自动填充

3. **无错误提示**
   - 不出现"加载预设脚本失败"错误
   - 不出现"加载预设脚本列表失败"错误
   - 控制台无 JavaScript 错误

4. **脚本可正常保存和运行**
   - 保存后配置持久化
   - 启动充电桩时脚本自动运行
   - 脚本状态正确更新

### ❌ 失败情况

如果出现以下情况，说明修复失败：

1. 点击"选择预设脚本"下拉菜单：
   - 列表为空
   - 列表显示但没有选项

2. 选择预设脚本后：
   - 显示"加载预设脚本失败"错误
   - 脚本内容没有填充
   - 脚本名称没有填充

3. 控制台错误：
   - JavaScript 错误
   - Tauri 命令调用失败

## 调试方法

### 开启开发者工具

在应用运行时按 `F12` 打开开发者工具，查看：

1. **Console 标签页**
   - 查看是否有 JavaScript 错误
   - 查看 Tauri 命令调用日志

2. **Network 标签页**
   - 查看是否有资源加载失败

### 查看 Rust 日志

设置环境变量启用详细日志：

```powershell
$env:RUST_LOG="debug"
npm run tauri:dev
```

查看控制台输出的日志信息。

### 手动测试 Tauri 命令

在开发者工具的 Console 中执行：

```javascript
// 测试获取预设脚本列表
const { invoke } = window.__TAURI__.tauri;
invoke('get_preset_scripts').then(scripts => {
  console.log('预设脚本列表:', scripts);
}).catch(err => {
  console.error('获取失败:', err);
});

// 测试读取预设脚本内容
invoke('read_preset_script', { scriptKey: 'basic_test' }).then(code => {
  console.log('脚本内容:', code.substring(0, 100) + '...');
}).catch(err => {
  console.error('读取失败:', err);
});
```

## 修复验证

### 开发环境验证

```powershell
npm run tauri:dev
```

按照上述测试步骤验证。

### 生产环境验证

```powershell
npm run tauri:build
.\src-tauri\target\release\bundle\msi\ev-charger-simulator_0.8.0_x64_en-US.msi
```

安装后运行，按照上述测试步骤验证。

---

**测试日期**: 2025-10-02  
**测试人员**: 开发团队  
**测试状态**: ⏳ 等待测试  
**测试环境**: Windows 11, WebView2 140.0.3485.94
