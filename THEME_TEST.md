# 主题切换功能测试指南

## 快速测试步骤

### 1. 启动应用

```bash
npm run tauri:dev
```

### 2. 测试主题切换

#### ✅ 在UI中测试

1. 找到右上角的主题切换开关
2. 点击切换按钮
3. 观察变化：

**切换到暗色模式时**：
- ✅ Header 背景变为深灰色渐变
- ✅ Content 背景变为深色渐变
- ✅ 所有卡片变为深灰色背景
- ✅ 文字变为浅色
- ✅ 表格行背景变深
- ✅ 悬停效果变深色
- ✅ Footer 变为深色

**切换到亮色模式时**：
- ✅ Header 背景变为蓝色渐变
- ✅ Content 背景变为浅色渐变
- ✅ 所有卡片变为白色背景
- ✅ 文字变为深色
- ✅ 表格行背景变白
- ✅ 悬停效果变浅色
- ✅ Footer 变为浅色

### 3. 测试持久化

#### ✅ 配置保存测试

1. 切换到暗色模式
2. 关闭应用窗口
3. 重新打开应用
4. **预期结果**：应用自动以暗色模式打开

#### ✅ 配置切换测试

1. 切换到亮色模式
2. 关闭应用
3. 重新打开应用
4. **预期结果**：应用自动以亮色模式打开

### 4. 测试 localStorage

#### 查看存储的配置

打开浏览器开发者工具（如果在 Tauri 中，按 F12）：

```javascript
// 在 Console 中执行
localStorage.getItem('theme-mode')
// 应该返回 'dark' 或 'light'
```

#### 手动设置主题

```javascript
// 设置为暗色模式
localStorage.setItem('theme-mode', 'dark')
// 刷新页面查看效果

// 设置为亮色模式
localStorage.setItem('theme-mode', 'light')
// 刷新页面查看效果
```

### 5. 视觉检查清单

#### Header（头部）
- [ ] Logo 图标可见且清晰
- [ ] Logo 文字可见且清晰
- [ ] 统计数字可见（在线数、充电中）
- [ ] 主题切换开关可见且可用

#### Content（内容区）
- [ ] 背景颜色协调
- [ ] 统计卡片可见且清晰
- [ ] 卡片阴影效果适当
- [ ] 文字对比度足够

#### 充电桩列表
- [ ] 表头颜色正确（蓝色渐变）
- [ ] 表头文字白色清晰
- [ ] 表格行背景适配主题
- [ ] 悬停效果明显
- [ ] 边框颜色协调

#### 操作按钮
- [ ] 按钮可见且可点击
- [ ] 图标清晰
- [ ] 悬停效果正常

#### 展开行
- [ ] 展开行背景适配主题
- [ ] 描述列表样式正确
- [ ] 文字清晰可读

#### Footer（页脚）
- [ ] 背景颜色适配主题
- [ ] 文字可见且清晰
- [ ] 边框颜色协调

### 6. 过渡效果测试

- [ ] 切换主题时有平滑的过渡动画（0.3s）
- [ ] 所有元素同步变化
- [ ] 没有闪烁或跳动

### 7. 响应式测试

#### 调整窗口大小
- [ ] 缩小窗口时主题正常
- [ ] 放大窗口时主题正常
- [ ] 所有元素在不同尺寸下都适配主题

### 8. 功能测试

#### 在暗色模式下
- [ ] 可以添加充电桩
- [ ] 可以删除充电桩
- [ ] 可以修改充电桩配置
- [ ] 可以启动/停止充电桩
- [ ] 可以配置脚本
- [ ] 所有弹窗适配暗色主题

#### 在亮色模式下
- [ ] 可以添加充电桩
- [ ] 可以删除充电桩
- [ ] 可以修改充电桩配置
- [ ] 可以启动/停止充电桩
- [ ] 可以配置脚本
- [ ] 所有弹窗适配亮色主题

## 常见问题

### Q: 切换主题后某些元素没有变化？
A: 检查是否正确添加了 `.dark-mode` 或 `.light-mode` 类名前缀。

### Q: 刷新后主题没有保持？
A: 检查浏览器是否允许 localStorage 存储。

### Q: 主题切换有延迟或卡顿？
A: 检查 CSS 过渡是否正确设置为 `transition: all 0.3s ease;`

### Q: 暗色模式下文字看不清？
A: 检查文字颜色是否设置为 `rgba(255, 255, 255, 0.85)` 或更亮的颜色。

### Q: 卡片在暗色模式下边框不明显？
A: 确认边框颜色设置为 `#303030` 或更亮的颜色。

## 调试技巧

### 1. 检查当前主题类名

```javascript
document.querySelector('.app-container').className
// 应该包含 'dark-mode' 或 'light-mode'
```

### 2. 检查 localStorage

```javascript
console.log('Theme:', localStorage.getItem('theme-mode'))
```

### 3. 强制切换主题

在开发者工具 Console 中：

```javascript
// 切换到暗色
document.querySelector('.app-container').classList.remove('light-mode')
document.querySelector('.app-container').classList.add('dark-mode')

// 切换到亮色
document.querySelector('.app-container').classList.remove('dark-mode')
document.querySelector('.app-container').classList.add('light-mode')
```

### 4. 检查 CSS 是否加载

```javascript
// 查找主题相关的样式规则
Array.from(document.styleSheets)
  .flatMap(sheet => Array.from(sheet.cssRules))
  .filter(rule => rule.selectorText?.includes('dark-mode'))
```

## 性能测试

### 主题切换速度
- 预期：< 300ms
- 测量方法：使用浏览器 Performance 工具录制切换过程

### 内存占用
- 预期：主题切换前后内存占用基本相同
- 测量方法：使用浏览器 Memory 工具对比切换前后的内存快照

## 已知限制

1. 不同标签页/窗口的主题不会自动同步
2. 首次加载时默认为亮色模式（除非有保存的配置）
3. 某些第三方组件可能需要额外的样式适配

## 下一步优化

- [ ] 添加自动主题模式（跟随系统）
- [ ] 添加更多主题色选项
- [ ] 支持跨标签页主题同步
- [ ] 添加主题切换快捷键
- [ ] 添加主题预览功能
