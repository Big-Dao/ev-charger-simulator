# 图标生成指南

## 问题

构建项目时需要 `icons/icon.ico` 文件，但默认项目不包含此文件。

## 解决方案

### 方法 1：使用 Tauri CLI 生成图标（推荐）

1. 准备一个 1024x1024 的 PNG 图标文件

2. 安装 Tauri CLI（如果尚未安装）:
```bash
npm install -g @tauri-apps/cli
```

3. 生成所有平台图标:
```bash
cd src-tauri
npm run tauri icon path/to/your/icon.png
```

这将自动生成:
- `icons/icon.ico` (Windows)
- `icons/icon.png` (Linux)
- `icons/*.icns` (macOS)
- 其他所需的图标尺寸

### 方法 2：使用在线图标生成器

1. 访问 [IconGenerator](https://icon.kitchen/) 或 [RealFaviconGenerator](https://realfavicongenerator.net/)

2. 上传一个 PNG 图标

3. 下载生成的 ICO 文件

4. 将文件重命名为 `icon.ico` 并放入 `src-tauri/icons/` 目录

### 方法 3：使用默认图标（快速测试）

1. 下载 Tauri 默认图标:
```bash
cd src-tauri/icons
Invoke-WebRequest -Uri "https://github.com/tauri-apps/tauri/raw/dev/tooling/cli/templates/app/app-icon.png" -OutFile "app-icon.png"
```

2. 使用在线转换器将 PNG 转换为 ICO 格式

3. 将生成的 ICO 文件保存为 `icon.ico`

### 方法 4：临时禁用图标要求（开发调试）

修改 `src-tauri/tauri.conf.json`：

```json
{
  "tauri": {
    "bundle": {
      "icon": [],  // 暂时清空图标列表
      "identifier": "com.ev-charger-simulator.app",
      // ...其他配置
    }
  }
}
```

**注意**：此方法仅用于开发调试，生产构建时必须提供图标。

## 推荐的图标设计

### 尺寸要求
- **源文件**: 至少 1024x1024 px
- **格式**: PNG (透明背景)
- **内容**: 简洁、可识别的图标

### 设计建议
对于充电桩模拟器:
- 使用充电桩图标 ⚡
- 使用绿色/蓝色主色调
- 简洁的线条设计
- 避免过多细节

## 验证

构建成功后，你应该能在以下位置看到图标:
- Windows: `src-tauri/target/release/bundle/msi/`
- Linux: `src-tauri/target/release/bundle/deb/` 或 `appimage/`
- macOS: `src-tauri/target/release/bundle/macos/`

## 常见问题

**Q: 为什么需要多种尺寸的图标？**  
A: 不同操作系统和显示场景需要不同尺寸：任务栏、开始菜单、桌面快捷方式等。

**Q: 可以使用 SVG 格式吗？**  
A: 不可以，Tauri 需要光栅图像格式（PNG, ICO, ICNS）。

**Q: 图标必须是正方形吗？**  
A: 是的，建议使用 1:1 的正方形图标以获得最佳效果。

## 相关链接

- [Tauri Icons 文档](https://tauri.app/v1/guides/features/icons/)
- [Icon Kitchen](https://icon.kitchen/)
- [Tauri CLI 文档](https://tauri.app/v1/api/cli/)
