# 预设脚本加载失败问题修复

## 问题描述

在安装版本（生产环境）中，点击"选择预设脚本"下拉菜单时，提示"加载预设脚本失败"。

## 根本原因

前端使用动态导入（`import()`）从相对路径加载脚本文件：

```javascript
const module = await import(/* @vite-ignore */ preset.path + '?raw');
```

这种方式在生产环境中失败，因为：
1. 脚本文件位于 `scripts/` 目录，不在前端打包的 `dist/` 目录中
2. Tauri 应用的资源协议无法访问项目目录外的文件
3. 打包后文件路径结构与开发环境不同

## 解决方案

### 1. 后端添加 Tauri 命令

在 `src-tauri/src/commands.rs` 中添加两个新命令：

```rust
/// 获取预设脚本列表
#[tauri::command]
pub fn get_preset_scripts() -> Result<Vec<PresetScript>, String>

/// 读取预设脚本内容
#[tauri::command]
pub fn read_preset_script(script_key: String) -> Result<String, String>
```

使用 `include_str!()` 宏在编译时将脚本内容嵌入到可执行文件中：

```rust
let script_content = match script_key.as_str() {
    "basic_test" => include_str!("../../scripts/basic_test.js"),
    "normal_charging" => include_str!("../../scripts/normal_charging.js"),
    "fast_charging" => include_str!("../../scripts/fast_charging.js"),
    "fault_test" => include_str!("../../scripts/fault_test.js"),
    _ => return Err(format!("Unknown preset script: {}", script_key)),
};
```

### 2. 前端调用 Tauri 命令

修改 `src/App.vue` 中的 `loadPresetScript` 函数：

```typescript
// 之前：使用动态导入（失败）
const module = await import(/* @vite-ignore */ preset.path + '?raw');
scriptForm.value.code = module.default;

// 现在：使用 Tauri 命令（成功）
const scriptCode = await invoke<string>('read_preset_script', { scriptKey: presetKey });
scriptForm.value.code = scriptCode;
```

### 3. 动态加载预设脚本列表

```typescript
interface PresetScript {
  key: string;
  name: string;
  description: string;
}

const presetScripts = ref<PresetScript[]>([]);

const loadPresetScriptList = async () => {
  try {
    presetScripts.value = await invoke<PresetScript[]>('get_preset_scripts');
  } catch (error) {
    console.error('加载预设脚本列表失败:', error);
  }
};

onMounted(() => {
  loadPresetScriptList();
});
```

### 4. 更新模板使用动态列表

```vue
<a-select-option 
  v-for="preset in presetScripts" 
  :key="preset.key" 
  :value="preset.key"
>
  <FileTextOutlined /> {{ preset.name }} - {{ preset.description }}
</a-select-option>
```

## 修复优点

### ✅ 生产环境可用
- 脚本内容编译时嵌入可执行文件
- 不依赖外部文件或路径

### ✅ 更好的用户体验
- 启动后立即可用，无需加载文件
- 加载速度更快（无 I/O 操作）
- 不会因文件缺失而失败

### ✅ 易于维护
- 脚本列表集中管理（后端）
- 前端动态加载，无需硬编码
- 添加新脚本只需修改后端代码

### ✅ 安全性
- 脚本内容不可被用户修改
- 避免路径遍历攻击
- 编译时验证脚本存在

## 文件修改

### 修改的文件

1. **src-tauri/src/commands.rs**
   - 添加 `PresetScript` 结构体
   - 添加 `get_preset_scripts()` 命令
   - 添加 `read_preset_script()` 命令

2. **src-tauri/src/main.rs**
   - 注册 `get_preset_scripts` 命令
   - 注册 `read_preset_script` 命令

3. **src/App.vue**
   - 添加 `PresetScript` 接口
   - 修改 `loadPresetScript()` 函数使用 Tauri 命令
   - 添加 `loadPresetScriptList()` 函数
   - 更新模板使用动态列表

## 测试方法

### 开发环境测试

```powershell
npm run tauri:dev
```

1. 点击"添加充电桩"
2. 填写充电桩 ID 和 URL
3. 切换到"脚本配置"标签页
4. 点击"选择预设脚本"下拉菜单
5. 选择任意预设脚本（如 `basic_test.js`）
6. 验证脚本内容正确加载

### 生产环境测试

```powershell
npm run tauri:build
.\src-tauri\target\release\ev-charger-simulator.exe
```

执行同样的测试步骤，确认预设脚本正确加载。

## 预设脚本列表

当前支持的预设脚本：

1. **basic_test.js** - 基础测试脚本
   - 验证脚本引擎的所有功能
   - 测试 console.log, sleep, shouldStop 等 API

2. **normal_charging.js** - 正常充电流程
   - 模拟完整的充电流程
   - 功率爬坡、持续充电、停止充电

3. **fast_charging.js** - 快速充电流程
   - 模拟快充场景
   - 大功率充电测试

4. **fault_test.js** - 故障测试
   - 模拟充电桩故障情况
   - 异常场景测试

## 扩展方法

### 添加新的预设脚本

1. 在 `scripts/` 目录创建新脚本文件（如 `my_script.js`）

2. 在 `commands.rs` 的 `get_preset_scripts()` 中添加：
   ```rust
   PresetScript {
       key: "my_script".to_string(),
       name: "my_script.js".to_string(),
       description: "我的自定义脚本".to_string(),
   },
   ```

3. 在 `read_preset_script()` 中添加：
   ```rust
   "my_script" => include_str!("../../scripts/my_script.js"),
   ```

4. 重新编译即可

---

**修复状态**: ✅ 已完成  
**测试状态**: ⏳ 等待构建完成  
**影响范围**: 生产环境预设脚本加载  
**修复日期**: 2025-10-02
