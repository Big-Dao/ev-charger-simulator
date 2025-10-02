# 白屏修复测试脚本

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "白屏问题修复 - 测试脚本" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# 等待构建完成
Write-Host "等待构建完成..." -ForegroundColor Yellow
while (-not (Test-Path "src-tauri\target\release\ev-charger-simulator.exe")) {
    Start-Sleep -Seconds 2
}
Start-Sleep -Seconds 3

Write-Host "✓ 构建完成`n" -ForegroundColor Green

# 显示文件信息
$exePath = "src-tauri\target\release\ev-charger-simulator.exe"
$fileInfo = Get-Item $exePath
Write-Host "可执行文件信息:" -ForegroundColor Cyan
Write-Host "  - 路径: $($fileInfo.FullName)" -ForegroundColor Gray
Write-Host "  - 大小: $([math]::Round($fileInfo.Length/1MB,2)) MB" -ForegroundColor Gray
Write-Host "  - 构建时间: $($fileInfo.LastWriteTime)`n" -ForegroundColor Gray

# 检查 dist 文件
Write-Host "前端构建产物:" -ForegroundColor Cyan
Get-ChildItem dist -File | ForEach-Object {
    Write-Host "  - $($_.Name): $([math]::Round($_.Length/1KB,2)) KB" -ForegroundColor Gray
}
Get-ChildItem dist\assets -File | ForEach-Object {
    Write-Host "  - assets\$($_.Name): $([math]::Round($_.Length/1KB,2)) KB" -ForegroundColor Gray
}

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "开始测试..." -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# 测试 1: 检查日志
Write-Host "[测试 1] 启动应用并检查日志..." -ForegroundColor Yellow
$env:RUST_LOG="info"

$process = Start-Process $exePath -PassThru -WindowStyle Normal

Start-Sleep -Seconds 5

if ($process.HasExited) {
    Write-Host "✗ 应用程序已退出 (Exit Code: $($process.ExitCode))" -ForegroundColor Red
} else {
    Write-Host "✓ 应用程序正在运行 (PID: $($process.Id))" -ForegroundColor Green
    
    Write-Host "`n请手动检查:" -ForegroundColor Yellow
    Write-Host "  1. 窗口是否正常显示（不是白屏）？" -ForegroundColor Gray
    Write-Host "  2. UI 界面是否完整加载？" -ForegroundColor Gray
    Write-Host "  3. 主题切换是否正常？" -ForegroundColor Gray
    Write-Host "  4. 添加充电桩功能是否正常？`n" -ForegroundColor Gray
    
    $answer = Read-Host "应用是否正常显示？(Y/N)"
    
    if ($answer -eq "Y" -or $answer -eq "y") {
        Write-Host "`n✅ 白屏问题已解决！" -ForegroundColor Green
        Write-Host "`n修复关键点:" -ForegroundColor Cyan
        Write-Host "  1. 移除了 index.html 中的 vite.svg 引用" -ForegroundColor Gray
        Write-Host "  2. 使用简化配置禁用代码分割" -ForegroundColor Gray
        Write-Host "  3. 设置 CSP 为 null" -ForegroundColor Gray
        Write-Host "  4. 使用相对路径 base: './'" -ForegroundColor Gray
    } else {
        Write-Host "`n❌ 问题仍然存在" -ForegroundColor Red
        Write-Host "`n请执行深度调试:" -ForegroundColor Yellow
        Write-Host "  1. 查看应用日志:" -ForegroundColor Gray
        Write-Host "     $env:RUST_LOG='trace'; .\src-tauri\target\release\ev-charger-simulator.exe" -ForegroundColor DarkGray
        Write-Host "`n  2. 在浏览器中测试 dist 目录:" -ForegroundColor Gray
        Write-Host "     cd dist; http-server -p 8080" -ForegroundColor DarkGray
        Write-Host "`n  3. 检查是否有安全软件阻止" -ForegroundColor Gray
    }
    
    Write-Host "`n按任意键关闭应用..." -ForegroundColor Yellow
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
}

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "测试完成" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
