# 🚀 GitHub Actions 自动化发布指南

## 📋 概述

本项目已配置 GitHub Actions 自动化工作流，支持：

1. **自动发布** - 推送 tag 时自动构建并发布
2. **持续集成** - 代码推送时自动构建和测试

---

## 🔧 配置的 Workflows

### 1️⃣ Release Workflow (`release.yml`)

**触发条件**：推送版本标签（如 `v0.8.4`, `v1.0.0`）

**功能**：
- ✅ 自动检出代码
- ✅ 设置 Node.js 和 Rust 环境
- ✅ 安装依赖和缓存
- ✅ 构建前端和后端
- ✅ 打包 Windows 安装包（MSI + NSIS）
- ✅ 创建 GitHub Release
- ✅ 自动上传安装包
- ✅ 生成发布说明

**输出**：
- GitHub Release（Latest Release）
- Windows MSI 安装包
- Windows NSIS 安装包
- 自动生成的发布说明

---

### 2️⃣ CI Workflow (`ci.yml`)

**触发条件**：
- 推送到 `master` 或 `main` 分支
- Pull Request 到 `master` 或 `main` 分支

**功能**：
- ✅ 代码检出
- ✅ 环境设置
- ✅ 前端 Lint 检查
- ✅ 前端构建测试
- ✅ Rust 格式检查
- ✅ Rust Clippy 检查
- ✅ Tauri 应用构建测试

**目的**：确保代码质量，及早发现问题

---

## 🚀 发布新版本

### 方式一：使用脚本自动发布（推荐）

创建一个发布脚本 `release.ps1`：

```powershell
# release.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

# 验证版本号格式
if ($Version -notmatch '^v\d+\.\d+\.\d+$') {
    Write-Host "❌ 错误: 版本号格式应为 vX.Y.Z (如 v0.8.4)" -ForegroundColor Red
    exit 1
}

Write-Host "🚀 准备发布 $Version..." -ForegroundColor Cyan

# 1. 确保工作区干净
$status = git status --porcelain
if ($status) {
    Write-Host "⚠️  警告: 工作区有未提交的更改" -ForegroundColor Yellow
    Write-Host $status
    $continue = Read-Host "是否继续? (y/N)"
    if ($continue -ne 'y') {
        exit 1
    }
}

# 2. 拉取最新代码
Write-Host "`n📥 拉取最新代码..." -ForegroundColor Yellow
git pull origin master

# 3. 创建并推送 tag
Write-Host "`n🏷️  创建 tag $Version..." -ForegroundColor Yellow
git tag -a $Version -m "Release $Version"

Write-Host "`n📤 推送 tag 到 GitHub..." -ForegroundColor Yellow
git push origin $Version

Write-Host "`n✅ Tag 已推送！GitHub Actions 将自动构建和发布。" -ForegroundColor Green
Write-Host "📊 查看构建进度: https://github.com/Big-Dao/ev-charger-simulator/actions" -ForegroundColor Cyan
}
```

**使用方法**：
```powershell
.\release.ps1 -Version v0.8.4
```

---

### 方式二：手动发布

#### 步骤 1: 更新 CHANGELOG.md

```bash
# 编辑 CHANGELOG.md，添加新版本信息
code CHANGELOG.md

# 提交更改
git add CHANGELOG.md
git commit -m "docs: 更新 CHANGELOG.md 添加 v0.8.4 版本信息"
git push origin master
```

#### 步骤 2: 创建并推送 Tag

```bash
# 创建 tag
git tag -a v0.8.4 -m "Release v0.8.4

主要更新：
- 新功能 A
- 修复 Bug B
- 改进性能 C"

# 推送 tag
git push origin v0.8.4
```

#### 步骤 3: 等待自动构建

1. 访问 [GitHub Actions](https://github.com/Big-Dao/ev-charger-simulator/actions)
2. 查看 "Release" workflow 的运行状态
3. 等待构建完成（约 10-15 分钟）

#### 步骤 4: 验证发布

1. 访问 [Releases 页面](https://github.com/Big-Dao/ev-charger-simulator/releases)
2. 检查新版本是否创建成功
3. 确认安装包已上传
4. 测试下载链接

---

## 📝 发布检查清单

### 发布前

- [ ] 所有功能已测试
- [ ] 文档已更新（README, CHANGELOG）
- [ ] 版本号已确定（遵循语义化版本）
- [ ] 工作区干净（无未提交更改）
- [ ] 已合并所有必要的 PR

### 发布时

- [ ] CHANGELOG.md 已更新并提交
- [ ] 创建正确格式的 tag（vX.Y.Z）
- [ ] Tag message 包含简要说明
- [ ] 推送 tag 到 GitHub

### 发布后

- [ ] GitHub Actions 构建成功
- [ ] Release 页面显示正常
- [ ] 安装包可下载
- [ ] 安装包可正常安装
- [ ] 应用功能正常
- [ ] 更新项目文档链接（如需要）

---

## 🔍 监控和调试

### 查看构建日志

1. 访问 [Actions 页面](https://github.com/Big-Dao/ev-charger-simulator/actions)
2. 点击对应的 workflow 运行
3. 查看每个步骤的详细日志

### 常见问题

#### ❌ 构建失败：Node.js 依赖错误

**原因**：`package-lock.json` 不是最新的

**解决**：
```bash
npm install
git add package-lock.json
git commit -m "chore: 更新 package-lock.json"
git push
```

#### ❌ 构建失败：Rust 编译错误

**原因**：Rust 代码有错误

**解决**：
```bash
cd src-tauri
cargo build
# 修复错误后
git add .
git commit -m "fix: 修复 Rust 编译错误"
git push
```

#### ❌ Release 已存在

**原因**：该 tag 的 Release 已经创建

**解决**：
```bash
# 删除远程 tag
git push --delete origin v0.8.4

# 删除本地 tag
git tag -d v0.8.4

# 重新创建和推送
git tag -a v0.8.4 -m "Release v0.8.4"
git push origin v0.8.4
```

#### ⚠️ 构建时间过长

**原因**：没有缓存或首次构建

**说明**：
- 首次构建：15-20 分钟
- 后续构建（有缓存）：5-10 分钟

---

## 🎯 版本号规范

遵循 [语义化版本 2.0.0](https://semver.org/lang/zh-CN/)：

```
vMAJOR.MINOR.PATCH
```

- **MAJOR**：不兼容的 API 修改
- **MINOR**：向下兼容的功能性新增
- **PATCH**：向下兼容的问题修正

### 示例

```
v0.8.3 → v0.8.4  # 修复 bug
v0.8.4 → v0.9.0  # 新增功能
v0.9.0 → v1.0.0  # 重大更新
```

### 版本号选择

| 变更类型 | 版本号变化 | 示例 |
|---------|-----------|------|
| 🐛 Bug 修复 | PATCH +1 | v0.8.3 → v0.8.4 |
| ✨ 新功能 | MINOR +1 | v0.8.4 → v0.9.0 |
| 💥 破坏性变更 | MAJOR +1 | v0.9.0 → v1.0.0 |
| 📚 仅文档更新 | 不变 | v0.8.3 = v0.8.3 |

---

## 🔄 工作流程图

```
开发 → 测试 → 提交 → 推送
                   ↓
            更新 CHANGELOG
                   ↓
              创建 tag
                   ↓
              推送 tag
                   ↓
        ┌──────────┴──────────┐
        ↓                     ↓
  GitHub Actions          本地等待
        ↓                     ↓
    检出代码              查看日志
        ↓                     ↓
    构建前端              监控进度
        ↓                     ↓
    构建后端                 ↓
        ↓                     ↓
    打包应用                 ↓
        ↓                     ↓
  创建 Release               ↓
        ↓                     ↓
  上传安装包                 ↓
        ↓                     ↓
    发布完成   ←──────────────┘
        ↓
    验证测试
```

---

## 🎓 高级用法

### 自定义发布说明

编辑 `.github/workflows/release.yml` 中的 `releaseBody` 部分：

```yaml
releaseBody: |
  ## 🎉 ${{ github.ref_name }} 发布
  
  ### ✨ 新功能
  - 功能 A
  - 功能 B
  
  ### 🐛 Bug 修复
  - 修复问题 X
  - 修复问题 Y
  
  ### 📚 文档
  - [完整变更日志](https://github.com/${{ github.repository }}/blob/master/CHANGELOG.md)
```

### 添加多平台支持

修改 `strategy.matrix`：

```yaml
strategy:
  matrix:
    include:
      - platform: 'windows-latest'
        args: '--target x86_64-pc-windows-msvc'
      - platform: 'macos-latest'
        args: '--target x86_64-apple-darwin'
      - platform: 'ubuntu-latest'
        args: '--target x86_64-unknown-linux-gnu'
```

### 自动更新版本号

可以添加一个步骤自动更新 `src-tauri/tauri.conf.json` 中的版本号：

```yaml
- name: Update version in tauri.conf.json
  run: |
    $version = "${{ github.ref_name }}".TrimStart('v')
    $config = Get-Content src-tauri/tauri.conf.json -Raw | ConvertFrom-Json
    $config.package.version = $version
    $config | ConvertTo-Json -Depth 10 | Set-Content src-tauri/tauri.conf.json
```

---

## 📊 成本和限制

### GitHub Actions 免费额度

- **Public Repository**: 无限制
- **Private Repository**: 
  - 每月 2000 分钟免费
  - Windows runner: 2x 计费（即 1 分钟算 2 分钟）

### 本项目预估

- 每次构建时间: ~10 分钟
- Windows runner: 算 20 分钟
- 每月可发布: ~100 次（免费额度内）

---

## 🎯 最佳实践

### 1. 频繁的小版本发布

✅ **好**：
```
v0.8.3 → v0.8.4 (修复 bug A)
v0.8.4 → v0.8.5 (修复 bug B)
v0.8.5 → v0.9.0 (新功能 C)
```

❌ **不好**：
```
v0.8.3 → v0.9.0 (修复 A、B、C、D + 新功能 E、F、G)
```

### 2. 有意义的提交信息

✅ **好**：
```
feat: 添加预设脚本自动加载功能
fix: 修复配置持久化问题
docs: 更新安装指南
```

❌ **不好**：
```
update
fix bug
改代码
```

### 3. 完整的测试

- 本地构建成功后再推送 tag
- 使用 CI workflow 验证构建
- 手动测试关键功能

### 4. 清晰的文档

- CHANGELOG.md 保持更新
- Release 说明详细清晰
- 文档链接准确有效

---

## 🔗 相关资源

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Tauri Action](https://github.com/tauri-apps/tauri-action)
- [语义化版本](https://semver.org/lang/zh-CN/)
- [本项目 Actions](https://github.com/Big-Dao/ev-charger-simulator/actions)
- [本项目 Releases](https://github.com/Big-Dao/ev-charger-simulator/releases)

---

## 📝 总结

### 优势

- ✅ **自动化** - 无需手动构建和上传
- ✅ **一致性** - 每次发布使用相同流程
- ✅ **可追溯** - 完整的构建日志
- ✅ **省时** - 释放开发者时间
- ✅ **可靠** - 减少人为错误

### 工作流

```bash
# 1. 开发和测试
git add .
git commit -m "feat: 新功能"
git push

# 2. 更新文档
code CHANGELOG.md
git add CHANGELOG.md
git commit -m "docs: 更新 CHANGELOG"
git push

# 3. 创建发布
git tag -a v0.8.4 -m "Release v0.8.4"
git push origin v0.8.4

# 4. 等待自动构建（10 分钟）

# 5. 验证发布
# 访问 GitHub Releases 页面
```

---

**GitHub Actions 已配置完成！下次发布只需推送 tag！** 🚀

**最后更新**: 2025-10-02  
**维护者**: @Big-Dao
