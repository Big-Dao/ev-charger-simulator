# 快速发布脚本
# 用法: .\release.ps1 -Version v0.8.4

param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

# 验证版本号格式
if ($Version -notmatch '^v\d+\.\d+\.\d+$') {
    Write-Host "❌ 错误: 版本号格式应为 vX.Y.Z (如 v0.8.4)" -ForegroundColor Red
    Write-Host "   示例: .\release.ps1 -Version v0.8.4" -ForegroundColor Yellow
    exit 1
}

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "🚀 EV Charger Simulator - 自动发布" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

Write-Host "版本号: $Version`n" -ForegroundColor Green

# 1. 检查工作区状态
Write-Host "[1/6] 检查工作区状态..." -ForegroundColor Yellow
$status = git status --porcelain
if ($status) {
    Write-Host "⚠️  警告: 工作区有未提交的更改:" -ForegroundColor Yellow
    Write-Host $status -ForegroundColor Gray
    $continue = Read-Host "`n是否继续? (y/N)"
    if ($continue -ne 'y' -and $continue -ne 'Y') {
        Write-Host "❌ 已取消" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✓ 工作区干净`n" -ForegroundColor Green
}

# 2. 拉取最新代码
Write-Host "[2/6] 拉取最新代码..." -ForegroundColor Yellow
git pull origin master
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 拉取失败" -ForegroundColor Red
    exit 1
}
Write-Host "✓ 代码已更新`n" -ForegroundColor Green

# 3. 检查 tag 是否已存在
Write-Host "[3/6] 检查 tag 是否已存在..." -ForegroundColor Yellow
$tagExists = git tag -l $Version
if ($tagExists) {
    Write-Host "❌ 错误: Tag $Version 已存在" -ForegroundColor Red
    Write-Host "   如需重新发布，请先删除旧 tag:" -ForegroundColor Yellow
    Write-Host "   git tag -d $Version" -ForegroundColor Gray
    Write-Host "   git push --delete origin $Version" -ForegroundColor Gray
    exit 1
}
Write-Host "✓ Tag 可用`n" -ForegroundColor Green

# 4. 创建 tag
Write-Host "[4/6] 创建 tag $Version..." -ForegroundColor Yellow
$message = Read-Host "请输入发布说明（可选，直接回车跳过）"
if ($message) {
    git tag -a $Version -m $message
} else {
    git tag -a $Version -m "Release $Version"
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tag 创建失败" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Tag 已创建`n" -ForegroundColor Green

# 5. 推送 tag
Write-Host "[5/6] 推送 tag 到 GitHub..." -ForegroundColor Yellow
git push origin $Version
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 推送失败" -ForegroundColor Red
    Write-Host "   删除本地 tag: git tag -d $Version" -ForegroundColor Yellow
    exit 1
}
Write-Host "✓ Tag 已推送`n" -ForegroundColor Green

# 6. 完成
Write-Host "[6/6] 发布流程启动完成！`n" -ForegroundColor Green

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "✅ 发布流程已启动！" -ForegroundColor Green
Write-Host "========================================`n" -ForegroundColor Cyan

Write-Host "接下来：" -ForegroundColor Cyan
Write-Host "  1. GitHub Actions 将自动构建应用（约 10-15 分钟）" -ForegroundColor White
Write-Host "  2. 构建完成后自动创建 Release" -ForegroundColor White
Write-Host "  3. 安装包将自动上传到 Release" -ForegroundColor White

Write-Host "`n📊 监控进度：" -ForegroundColor Cyan
Write-Host "  https://github.com/Big-Dao/ev-charger-simulator/actions`n" -ForegroundColor Blue

Write-Host "📦 发布页面：" -ForegroundColor Cyan
Write-Host "  https://github.com/Big-Dao/ev-charger-simulator/releases/tag/$Version`n" -ForegroundColor Blue

Write-Host "💡 提示：" -ForegroundColor Yellow
Write-Host "  - 首次构建可能需要 15-20 分钟" -ForegroundColor Gray
Write-Host "  - 后续构建（有缓存）约 5-10 分钟" -ForegroundColor Gray
Write-Host "  - 可以在 Actions 页面查看详细日志`n" -ForegroundColor Gray
