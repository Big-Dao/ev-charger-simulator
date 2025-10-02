# 配置持久化问题修复

## 🐛 问题描述

**症状**：
- 安装版本运行后，充电桩列表为空
- 添加充电桩后，重启应用配置丢失
- 配置文件没有保存

**根本原因**：
在生产环境（安装版本）中，应用的当前工作目录不是应用安装目录，导致配置文件路径 `config/chargers.json` 无法正确定位和创建。

## ✅ 修复方案

### 配置文件路径优先级

修改 `config_loader.rs`，按以下优先级查找和保存配置文件：

1. **环境变量** `CHARGER_CONFIG_PATH`（最高优先级）
2. **应用数据目录**（生产环境推荐）
   - Windows: `%APPDATA%\com.evcharger.simulator\config\chargers.json`
   - macOS: `~/Library/Application Support/com.evcharger.simulator/config/chargers.json`
   - Linux: `~/.local/share/com.evcharger.simulator/config/chargers.json`
3. **可执行文件目录**（便携模式）
   - `<exe目录>\config\chargers.json`
4. **当前工作目录**（开发模式）
   - `config/chargers.json`

### 关键代码修改

```rust
/// 获取应用数据目录
fn get_app_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .ok()
            .map(|appdata| PathBuf::from(appdata).join("com.evcharger.simulator"))
    }
    // ... macOS 和 Linux 类似
}

/// 获取默认配置文件路径
fn get_default_config_path() -> PathBuf {
    // 生产环境：使用应用数据目录
    if let Some(app_data_dir) = get_app_data_dir() {
        return app_data_dir.join("config").join("chargers.json");
    }
    
    // 开发环境：使用当前目录
    PathBuf::from("config/chargers.json")
}

fn locate_config_file() -> Option<PathBuf> {
    // 1. 环境变量
    // 2. 应用数据目录
    // 3. 可执行文件目录
    // 4. 当前工作目录
}
```

## 📂 配置文件位置

### 开发模式 (`npm run tauri:dev`)

```
项目根目录/
└── config/
    └── chargers.json
```

### 生产模式（安装版本）

**Windows**:
```
C:\Users\<用户名>\AppData\Roaming\com.evcharger.simulator\
└── config/
    └── chargers.json
```

**macOS**:
```
~/Library/Application Support/com.evcharger.simulator/
└── config/
    └── chargers.json
```

**Linux**:
```
~/.local/share/com.evcharger.simulator/
└── config/
    └── chargers.json
```

## 🧪 测试方法

### 1. 开发模式测试

```powershell
# 启动开发模式
npm run tauri:dev

# 添加充电桩
# 检查 config/chargers.json 文件是否更新

# 重启应用
# 确认充电桩列表保留
```

### 2. 生产模式测试

```powershell
# 构建并运行
npm run tauri:build
.\src-tauri\target\release\ev-charger-simulator.exe

# 添加充电桩
# 检查配置文件位置:
Get-Content "$env:APPDATA\com.evcharger.simulator\config\chargers.json"

# 重启应用
# 确认充电桩列表保留
```

### 3. 便携模式测试

```powershell
# 设置环境变量使用可执行文件目录
$env:CHARGER_CONFIG_PATH = ".\config\chargers.json"
.\src-tauri\target\release\ev-charger-simulator.exe

# 添加充电桩
# 检查当前目录的 config/chargers.json
```

## 🔍 调试方法

### 查看日志确认配置文件路径

```powershell
# 启用日志
$env:RUST_LOG="info"
.\src-tauri\target\release\ev-charger-simulator.exe

# 查找日志中的以下信息:
# "Loading charger configuration from <路径>"
# "Saved charger configuration to <路径>"
```

### 手动检查配置文件

```powershell
# Windows
Get-Content "$env:APPDATA\com.evcharger.simulator\config\chargers.json" | ConvertFrom-Json | ConvertTo-Json -Depth 10

# 或者
notepad "$env:APPDATA\com.evcharger.simulator\config\chargers.json"
```

## 📋 验证清单

修复后请确认：

- [ ] 开发模式：配置保存到 `config/chargers.json`
- [ ] 生产模式：配置保存到 `%APPDATA%\com.evcharger.simulator\config\chargers.json`
- [ ] 添加充电桩后，配置文件立即更新
- [ ] 重启应用后，充电桩列表保留
- [ ] 修改充电桩配置后，更改被保存
- [ ] 删除充电桩后，配置文件更新
- [ ] 批量创建充电桩后，所有桩都被保存

## 🎓 技术说明

### 为什么使用应用数据目录？

1. **权限问题**：安装目录（如 `C:\Program Files`）通常需要管理员权限写入
2. **多用户支持**：每个用户有独立的配置，互不干扰
3. **Windows 规范**：符合 Windows 应用数据存储最佳实践
4. **卸载友好**：卸载应用时可选择保留或删除用户数据

### 配置文件格式

```json
{
  "chargers": [
    {
      "id": "CP000001",
      "name": "充电桩 #1",
      "protocol_type": "OCPP",
      "server_url": "ws://localhost:8080/ocpp",
      "max_power": 7.0,
      "script_path": null,
      "enabled": true
    }
  ],
  "batch_config": {
    "count": 0,
    "protocol_type": "OCPP",
    "server_url": "ws://localhost:8080/ocpp",
    "max_power": 7.0,
    "script_path": null,
    "enabled": false,
    "name_prefix": "充电桩",
    "id_prefix": "CP",
    "start_index": 1
  }
}
```

## 🚀 发布注意事项

### 更新日志

应在 CHANGELOG.md 中添加：

```markdown
### Fixed
- 修复生产环境配置持久化失败问题
- 配置文件现在保存到用户应用数据目录
- Windows: %APPDATA%\com.evcharger.simulator\config\
- 添加、修改、删除充电桩后配置自动保存
```

### 用户迁移

如果用户之前有配置文件在旧位置，需要手动迁移：

```powershell
# 1. 找到旧配置（如果有）
$oldConfig = ".\config\chargers.json"

# 2. 复制到新位置
$newConfig = "$env:APPDATA\com.evcharger.simulator\config\chargers.json"
New-Item -ItemType Directory -Force -Path (Split-Path $newConfig)
Copy-Item $oldConfig $newConfig
```

## 📝 相关文件

- `src-tauri/src/config_loader.rs` - 配置加载和保存逻辑
- `src-tauri/src/commands.rs` - 调用保存配置的命令
- `config/chargers.json` - 示例配置文件

---

**修复日期**: 2025-10-02  
**影响版本**: v0.8.0  
**修复版本**: v0.8.2  
**严重程度**: 🔴 严重（配置无法保存）
