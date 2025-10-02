# 配置文件说明

## 📂 配置文件位置

从 v0.8.2 版本开始，配置文件保存位置已更改为系统应用数据目录，以解决权限和持久化问题。

### Windows

```
C:\Users\<您的用户名>\AppData\Roaming\com.evcharger.simulator\config\chargers.json
```

**快速访问**：
```powershell
# 在文件资源管理器中打开
explorer "$env:APPDATA\com.evcharger.simulator\config"

# 或使用 PowerShell 查看
notepad "$env:APPDATA\com.evcharger.simulator\config\chargers.json"
```

### macOS

```
~/Library/Application Support/com.evcharger.simulator/config/chargers.json
```

**快速访问**：
```bash
# 在 Finder 中打开
open ~/Library/Application\ Support/com.evcharger.simulator/config

# 或使用文本编辑器
open -e ~/Library/Application\ Support/com.evcharger.simulator/config/chargers.json
```

### Linux

```
~/.local/share/com.evcharger.simulator/config/chargers.json
```

**快速访问**：
```bash
# 使用文件管理器打开
xdg-open ~/.local/share/com.evcharger.simulator/config

# 或使用文本编辑器
nano ~/.local/share/com.evcharger.simulator/config/chargers.json
```

## 🔧 自定义配置路径

如果您需要使用自定义路径（例如便携模式），可以设置环境变量：

### Windows

```powershell
# 临时设置（当前会话）
$env:CHARGER_CONFIG_PATH = "D:\MyData\chargers.json"

# 永久设置（系统环境变量）
[Environment]::SetEnvironmentVariable("CHARGER_CONFIG_PATH", "D:\MyData\chargers.json", "User")
```

### macOS/Linux

```bash
# 临时设置
export CHARGER_CONFIG_PATH="/path/to/chargers.json"

# 永久设置（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export CHARGER_CONFIG_PATH="/path/to/chargers.json"' >> ~/.bashrc
```

## 📋 配置文件格式

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

## 🔄 迁移旧配置

如果您从旧版本升级，配置文件可能在项目目录的 `config/chargers.json`。

### 自动迁移（推荐）

应用会自动查找旧配置文件并在首次启动时加载。添加或修改充电桩后，配置会自动保存到新位置。

### 手动迁移

#### Windows

```powershell
# 1. 找到旧配置文件
$oldConfig = "C:\path\to\old\config\chargers.json"

# 2. 创建新目录
$newConfigDir = "$env:APPDATA\com.evcharger.simulator\config"
New-Item -ItemType Directory -Force -Path $newConfigDir

# 3. 复制配置
Copy-Item $oldConfig "$newConfigDir\chargers.json"
```

#### macOS/Linux

```bash
# 1. 创建新目录
mkdir -p ~/Library/Application\ Support/com.evcharger.simulator/config

# 2. 复制配置
cp /path/to/old/config/chargers.json ~/Library/Application\ Support/com.evcharger.simulator/config/
```

## 🛠️ 故障排除

### 配置未保存

1. **检查配置文件是否存在**
   ```powershell
   # Windows
   Test-Path "$env:APPDATA\com.evcharger.simulator\config\chargers.json"
   ```

2. **检查目录权限**
   ```powershell
   # Windows - 查看权限
   Get-Acl "$env:APPDATA\com.evcharger.simulator\config" | Format-List
   ```

3. **查看应用日志**
   ```powershell
   # 启用日志运行
   $env:RUST_LOG="info"
   .\ev-charger-simulator.exe
   
   # 查找包含 "Saved charger configuration to" 的行
   ```

### 配置文件损坏

如果配置文件损坏导致应用无法启动：

```powershell
# Windows - 备份并删除配置文件
$configPath = "$env:APPDATA\com.evcharger.simulator\config\chargers.json"
if (Test-Path $configPath) {
    Copy-Item $configPath "$configPath.backup"
    Remove-Item $configPath
}

# 重新启动应用，会创建新的空配置
```

### 找不到配置文件

应用会按以下顺序查找配置：

1. 环境变量 `CHARGER_CONFIG_PATH` 指定的路径
2. 应用数据目录（推荐）
3. 可执行文件所在目录的 `config/chargers.json`
4. 当前工作目录的 `config/chargers.json`

如果所有位置都找不到，应用会创建一个新的配置文件。

## 💡 最佳实践

### 备份配置

定期备份您的配置文件：

```powershell
# Windows - 创建带时间戳的备份
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$configPath = "$env:APPDATA\com.evcharger.simulator\config\chargers.json"
$backupPath = "$env:USERPROFILE\Documents\chargers_backup_$timestamp.json"
Copy-Item $configPath $backupPath
Write-Host "配置已备份到: $backupPath"
```

### 版本控制（高级）

如果您管理多个配置，可以使用 Git：

```bash
cd ~/Library/Application\ Support/com.evcharger.simulator/config
git init
git add chargers.json
git commit -m "Initial config"
```

### 团队共享

使用环境变量指向共享网络位置：

```powershell
# Windows - 使用网络共享
$env:CHARGER_CONFIG_PATH = "\\server\share\configs\chargers.json"
```

## 📞 支持

如果遇到配置相关问题，请：

1. 查看应用日志（设置 `RUST_LOG=debug`）
2. 检查配置文件权限
3. 尝试使用自定义路径（环境变量）
4. 提交 Issue：https://github.com/Big-Dao/ev-charger-simulator/issues

---

**更新日期**: 2025-10-02  
**适用版本**: v0.8.2+
