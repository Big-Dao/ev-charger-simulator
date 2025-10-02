# 配置持久化测试脚本

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "配置持久化修复测试" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# 等待构建完成
Write-Host "等待构建完成..." -ForegroundColor Yellow
while (-not (Test-Path "src-tauri\target\release\ev-charger-simulator.exe")) {
    Start-Sleep -Seconds 2
}
Start-Sleep -Seconds 3
Write-Host "✓ 构建完成`n" -ForegroundColor Green

# 配置文件路径
$configPath = "$env:APPDATA\com.evcharger.simulator\config\chargers.json"
$configDir = Split-Path $configPath

Write-Host "配置文件路径: $configPath`n" -ForegroundColor Cyan

# 清理旧配置（可选）
$cleanOld = Read-Host "是否清理旧配置进行全新测试？(Y/N)"
if ($cleanOld -eq "Y" -or $cleanOld -eq "y") {
    if (Test-Path $configDir) {
        Remove-Item -Recurse -Force $configDir
        Write-Host "✓ 已清理旧配置`n" -ForegroundColor Green
    }
}

# 测试 1: 首次运行
Write-Host "[测试 1] 首次运行应用..." -ForegroundColor Yellow
$env:RUST_LOG="info"
$process = Start-Process "src-tauri\target\release\ev-charger-simulator.exe" -PassThru

Start-Sleep -Seconds 5

if ($process.HasExited) {
    Write-Host "✗ 应用程序已退出" -ForegroundColor Red
    exit 1
}

Write-Host "✓ 应用程序运行中 (PID: $($process.Id))`n" -ForegroundColor Green

# 等待用户添加充电桩
Write-Host "请在应用中:" -ForegroundColor Yellow
Write-Host "  1. 点击'添加充电桩'按钮" -ForegroundColor Gray
Write-Host "  2. 填写充电桩信息（ID、服务器 URL 等）" -ForegroundColor Gray
Write-Host "  3. 点击'确定'保存`n" -ForegroundColor Gray

Read-Host "添加完成后按回车继续"

# 检查配置文件是否创建
Write-Host "`n[测试 2] 检查配置文件..." -ForegroundColor Yellow

if (Test-Path $configPath) {
    Write-Host "✓ 配置文件已创建" -ForegroundColor Green
    
    # 读取并显示配置
    $config = Get-Content $configPath | ConvertFrom-Json
    $chargerCount = $config.chargers.Count
    
    Write-Host "  - 路径: $configPath" -ForegroundColor Gray
    Write-Host "  - 大小: $((Get-Item $configPath).Length) 字节" -ForegroundColor Gray
    Write-Host "  - 充电桩数量: $chargerCount`n" -ForegroundColor Gray
    
    if ($chargerCount -gt 0) {
        Write-Host "✓ 配置包含充电桩数据" -ForegroundColor Green
        Write-Host "`n充电桩列表:" -ForegroundColor Cyan
        foreach ($charger in $config.chargers) {
            Write-Host "  - ID: $($charger.id), 名称: $($charger.name)" -ForegroundColor Gray
        }
    } else {
        Write-Host "✗ 配置中没有充电桩数据" -ForegroundColor Red
    }
} else {
    Write-Host "✗ 配置文件未创建" -ForegroundColor Red
    Write-Host "  预期路径: $configPath" -ForegroundColor Gray
}

# 关闭应用
Write-Host "`n[测试 3] 关闭应用..." -ForegroundColor Yellow
Stop-Process -Id $process.Id -Force
Start-Sleep -Seconds 2
Write-Host "✓ 应用已关闭`n" -ForegroundColor Green

# 测试 4: 重新启动验证持久化
Write-Host "[测试 4] 重新启动应用验证配置..." -ForegroundColor Yellow
$process2 = Start-Process "src-tauri\target\release\ev-charger-simulator.exe" -PassThru
Start-Sleep -Seconds 5

if ($process2.HasExited) {
    Write-Host "✗ 应用程序已退出" -ForegroundColor Red
    exit 1
}

Write-Host "✓ 应用程序已重新启动 (PID: $($process2.Id))`n" -ForegroundColor Green

Write-Host "请检查:" -ForegroundColor Yellow
Write-Host "  1. 充电桩列表是否显示？" -ForegroundColor Gray
Write-Host "  2. 之前添加的充电桩是否还在？" -ForegroundColor Gray
Write-Host "  3. 充电桩信息是否完整？`n" -ForegroundColor Gray

$persistent = Read-Host "配置是否成功持久化？(Y/N)"

# 清理
Stop-Process -Id $process2.Id -Force -ErrorAction SilentlyContinue

# 结果
Write-Host "`n========================================" -ForegroundColor Cyan
if ($persistent -eq "Y" -or $persistent -eq "y") {
    Write-Host "✅ 配置持久化测试通过！" -ForegroundColor Green
    Write-Host "`n修复验证成功:" -ForegroundColor Cyan
    Write-Host "  ✓ 配置文件正确保存到应用数据目录" -ForegroundColor Gray
    Write-Host "  ✓ 添加充电桩后立即保存" -ForegroundColor Gray
    Write-Host "  ✓ 重启后配置正确加载" -ForegroundColor Gray
    Write-Host "  ✓ 充电桩数据完整保留`n" -ForegroundColor Gray
    
    Write-Host "配置文件位置: $configPath" -ForegroundColor Cyan
} else {
    Write-Host "❌ 配置持久化测试失败" -ForegroundColor Red
    Write-Host "`n请检查:" -ForegroundColor Yellow
    Write-Host "  1. 配置文件是否存在: $configPath" -ForegroundColor Gray
    Write-Host "  2. 文件权限是否正常" -ForegroundColor Gray
    Write-Host "  3. 应用日志中是否有错误" -ForegroundColor Gray
    Write-Host "`n查看日志命令:" -ForegroundColor Yellow
    Write-Host "  `$env:RUST_LOG='debug'; .\src-tauri\target\release\ev-charger-simulator.exe" -ForegroundColor DarkGray
}

Write-Host "========================================" -ForegroundColor Cyan
