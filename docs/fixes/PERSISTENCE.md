# 充电桩配置持久化功能

## 概述

充电桩模拟器现已支持配置自动持久化，所有通过UI进行的增删改操作都会自动保存到配置文件中。

## 功能特性

### ✅ 自动保存

以下操作会自动触发配置保存：

1. **添加充电桩** (`add_charger`)
   - 在UI中点击"添加充电桩"按钮
   - 填写表单并提交
   - 系统自动保存到 `config/chargers.json`

2. **删除充电桩** (`remove_charger`)
   - 在充电桩列表中点击"删除"按钮
   - 确认删除后自动保存配置

3. **修改充电桩配置** (`update_charger_config`)
   - 点击"充电桩参数"按钮
   - 修改参数（名称、协议、服务器地址、最大功率等）
   - 提交后自动保存

### 📁 配置文件位置

配置文件存储在：`config/chargers.json`

文件结构：
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
    "enabled": false,
    "count": 0,
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

### 🔄 配置加载

应用启动时会自动：
1. 读取 `config/chargers.json`
2. 解析配置文件
3. 创建所有 `enabled: true` 的充电桩
4. 如果配置了 `batch_config.enabled: true`，还会批量创建充电桩

### 🛡️ 错误处理

- 保存失败时会记录警告日志，但不会影响UI操作
- 如果配置目录不存在，会自动创建
- 保留原有的 `batch_config` 配置（不会被覆盖）

## 技术实现

### 核心函数

#### `config_loader::save_to_file()`
```rust
pub async fn save_to_file(manager: Arc<ChargerManager>) -> Result<(), String>
```
- 获取所有充电桩配置
- 序列化为JSON格式
- 写入配置文件
- 自动创建目录（如果不存在）

#### `manager::get_all_charger_configs()`
```rust
pub async fn get_all_charger_configs(&self) -> Vec<ChargerConfig>
```
- 遍历所有充电桩
- 收集配置信息
- 返回配置列表

### 集成点

在以下 Tauri 命令中集成了自动保存：
- `commands::add_charger` - 添加后保存
- `commands::remove_charger` - 删除后保存
- `commands::update_charger_config` - 更新后保存

## 使用示例

### 添加充电桩
```typescript
// 前端调用
await invoke('add_charger', {
  config: {
    id: 'CP000010',
    name: '10号充电桩',
    protocol_type: 'OCPP',
    server_url: 'ws://localhost:8080/ocpp',
    max_power: 60.0,
    script_path: null,
    enabled: true
  }
});
// 配置自动保存到文件 ✅
```

### 修改充电桩
```typescript
// 前端调用
await invoke('update_charger_config', {
  chargerId: 'CP000001',
  config: {
    id: 'CP000001',
    name: '1号充电桩(已修改)',
    protocol_type: 'OCPP',
    server_url: 'ws://localhost:9000/ocpp',
    max_power: 120.0,
    script_path: 'scripts/fast_charging.js',
    enabled: true
  }
});
// 配置自动保存到文件 ✅
```

### 删除充电桩
```typescript
// 前端调用
await invoke('remove_charger', {
  chargerId: 'CP000005'
});
// 配置自动保存到文件 ✅
```

## 日志

成功保存时会输出：
```
[INFO] Saved charger configuration to C:\...\config\chargers.json
```

保存失败时会输出警告：
```
[WARN] Failed to save config after adding charger: <error message>
```

## 注意事项

1. **脚本路径不持久化运行状态**：只保存 `script_path` 配置路径，不保存脚本是否正在运行
2. **batch_config 保留**：自动保存时会保留原有的批量创建配置
3. **异步保存**：保存操作不会阻塞UI响应
4. **向后兼容**：如果配置文件不存在，会自动创建

## 开发建议

- 修改充电桩相关代码时，确保调用 `save_to_file()` 保持持久化
- 添加新的充电桩操作命令时，记得集成自动保存
- 测试时可以查看 `config/chargers.json` 验证保存结果
