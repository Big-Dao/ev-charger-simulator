# 白屏问题快速修复脚本

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "EV Charger Simulator - 白屏问题修复" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# 步骤 1: 清理旧构建
Write-Host "[1/5] 清理旧构建..." -ForegroundColor Yellow
Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force src-tauri\target\release\bundle -ErrorAction SilentlyContinue
Write-Host "✓ 清理完成`n" -ForegroundColor Green

# 步骤 2: 重新构建前端
Write-Host "[2/5] 构建前端..." -ForegroundColor Yellow
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ 前端构建失败" -ForegroundColor Red
    exit 1
}
Write-Host "✓ 前端构建完成`n" -ForegroundColor Green

# 步骤 3: 验证构建产物
Write-Host "[3/5] 验证构建产物..." -ForegroundColor Yellow
$distFiles = Get-ChildItem dist -Recurse -File
$totalSize = ($distFiles | Measure-Object -Property Length -Sum).Sum
Write-Host "  - 文件数量: $($distFiles.Count)" -ForegroundColor Gray
Write-Host "  - 总大小: $([math]::Round($totalSize/1MB,2)) MB" -ForegroundColor Gray

if (-not (Test-Path "dist\index.html")) {
    Write-Host "✗ 缺少 index.html" -ForegroundColor Red
    exit 1
}
Write-Host "✓ 构建产物验证通过`n" -ForegroundColor Green

# 步骤 4: 构建 Tauri 应用
Write-Host "[4/5] 构建 Tauri 应用..." -ForegroundColor Yellow
npm run tauri:build -- --no-bundle
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Tauri 构建失败" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Tauri 构建完成`n" -ForegroundColor Green

# 步骤 5: 运行测试
Write-Host "[5/5] 启动应用测试..." -ForegroundColor Yellow
$exePath = "src-tauri\target\release\ev-charger-simulator.exe"
if (Test-Path $exePath) {
    $fileInfo = Get-Item $exePath
    Write-Host "  - 文件大小: $([math]::Round($fileInfo.Length/1MB,2)) MB" -ForegroundColor Gray
    Write-Host "  - 构建时间: $($fileInfo.LastWriteTime)" -ForegroundColor Gray
    
    Write-Host "`n正在启动应用程序..." -ForegroundColor Cyan
    Start-Process $exePath
    
    Write-Host "`n✓ 应用程序已启动" -ForegroundColor Green
    Write-Host "`n请检查:" -ForegroundColor Yellow
    Write-Host "  1. 窗口是否正常显示（不是白屏）" -ForegroundColor Gray
    Write-Host "  2. UI 界面是否加载完整" -ForegroundColor Gray
    Write-Host "  3. 功能是否正常工作" -ForegroundColor Gray
} else {
    Write-Host "✗ 找不到可执行文件" -ForegroundColor Red
    exit 1
}

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "修复脚本执行完成" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
