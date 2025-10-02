# 主题切换功能说明

## 概述

充电桩模拟器支持亮色/暗色主题切换，所有UI元素都会自动适配，并且主题配置会持久化保存。

## 功能特性

### ✅ 完整的主题切换

#### 1. **Header（头部）**
- **亮色模式**：
  - 背景：蓝色渐变 `linear-gradient(135deg, #1890ff 0%, #096dd9 100%)`
  - 文字：白色 `#fff`
  - 阴影：轻微阴影效果
  
- **暗色模式**：
  - 背景：深灰渐变 `linear-gradient(135deg, #141414 0%, #1f1f1f 100%)`
  - 文字：浅白色 `rgba(255, 255, 255, 0.85)`
  - Logo图标：蓝色高亮 `#1890ff`
  - 边框：底部边框 `#303030`

#### 2. **Content（内容区）**
- **亮色模式**：
  - 背景：浅色渐变 `linear-gradient(180deg, #f0f2f5 0%, #ffffff 100%)`
  - 卡片背景：白色
  - 文字：深色
  
- **暗色模式**：
  - 背景：深色渐变 `linear-gradient(180deg, #141414 0%, #1a1a1a 100%)`
  - 卡片背景：深灰 `#1f1f1f`
  - 卡片边框：`#303030`
  - 文字：浅色 `rgba(255, 255, 255, 0.85)`

#### 3. **Table（表格）**
- **亮色模式**：
  - 表头：蓝色渐变，白色文字
  - 行背景：白色
  - 悬停：浅灰色
  - 边框：浅灰色
  
- **暗色模式**：
  - 表头：蓝色渐变，白色文字（保持一致）
  - 行背景：深灰 `#1f1f1f`
  - 悬停：稍深灰色 `#262626`
  - 边框：深灰 `#303030`

#### 4. **Card（卡片）**
- **亮色模式**：
  - 背景：白色
  - 边框：浅色
  - 阴影：轻微
  
- **暗色模式**：
  - 背景：深灰 `#1f1f1f`
  - 边框：深灰 `#303030`
  - 阴影：加深

#### 5. **Footer（页脚）**
- **亮色模式**：
  - 背景：`#f0f2f5`
  - 文字：`rgba(0, 0, 0, 0.65)`
  - 边框：`#d9d9d9`
  
- **暗色模式**：
  - 背景：`#141414`
  - 文字：`rgba(255, 255, 255, 0.65)`
  - 边框：`#303030`

### 💾 持久化保存

#### 技术实现

```typescript
// 读取保存的主题配置
const isDark = ref(localStorage.getItem('theme-mode') === 'dark');

// 监听主题变化并保存
watch(isDark, (value) => {
  localStorage.setItem('theme-mode', value ? 'dark' : 'light');
  message.success(`已切换到${value ? '暗色' : '亮色'}模式`);
});
```

#### 存储位置
- **localStorage** 键名：`theme-mode`
- **存储值**：
  - `'dark'` - 暗色模式
  - `'light'` - 亮色模式

#### 加载逻辑
- 应用启动时自动从 localStorage 读取
- 如果未设置，默认使用亮色模式
- 切换主题时立即保存到 localStorage

### 🎨 动态类名控制

使用动态类名实现主题样式：

```vue
<div class="app-container" :class="{ 'dark-mode': isDark, 'light-mode': !isDark }">
```

- `dark-mode` - 暗色模式样式
- `light-mode` - 亮色模式样式

### ⚡ 平滑过渡

所有主题相关的样式都添加了过渡效果：

```css
transition: all 0.3s ease;
```

包括：
- Header 背景色
- Content 背景色
- 卡片颜色
- 文字颜色
- 边框颜色

## 使用方式

### 在UI中切换

点击右上角的主题切换开关：
- **暗色** - 切换到暗色模式
- **亮色** - 切换到亮色模式

### 通过代码切换

```typescript
// 切换到暗色模式
isDark.value = true;

// 切换到亮色模式
isDark.value = false;
```

## 样式结构

### CSS 类名层级

```
.app-container
  ├── .dark-mode  (暗色模式根类)
  │   ├── .header
  │   ├── .content
  │   ├── .welcome-card
  │   ├── .action-card
  │   ├── .charger-card
  │   ├── .stat-card
  │   └── .footer
  │
  └── .light-mode (亮色模式根类)
      ├── .header
      ├── .content
      ├── .welcome-card
      ├── .action-card
      ├── .charger-card
      ├── .stat-card
      └── .footer
```

### 主题色配置

使用 Ant Design Vue 的主题配置：

```typescript
const themeConfig = computed(() => ({
  algorithm: isDark.value ? theme.darkAlgorithm : theme.defaultAlgorithm,
  token: {
    colorPrimary: '#1890ff',  // 主色调
    borderRadius: 8,          // 圆角
  },
}));
```

## 响应式支持

主题切换支持所有屏幕尺寸：
- 桌面端（≥1920px）
- 笔记本（1366px - 1920px）
- 平板（768px - 1366px）
- 手机（<768px）

## 兼容性

- ✅ Chrome/Edge 90+
- ✅ Firefox 88+
- ✅ Safari 14+
- ✅ 支持所有现代浏览器

## 注意事项

1. **localStorage 权限**：确保浏览器允许使用 localStorage
2. **初次加载**：如果 localStorage 中没有配置，默认为亮色模式
3. **跨标签页同步**：不同标签页的主题不会自动同步，需要刷新
4. **图标颜色**：暗色模式下某些图标会自动调整为高亮色

## 扩展建议

### 添加新的主题色

修改 `themeConfig` 中的 token：

```typescript
const themeConfig = computed(() => ({
  algorithm: isDark.value ? theme.darkAlgorithm : theme.defaultAlgorithm,
  token: {
    colorPrimary: '#1890ff',
    colorSuccess: '#52c41a',
    colorWarning: '#faad14',
    colorError: '#ff4d4f',
    borderRadius: 8,
  },
}));
```

### 添加更多主题模式

可以扩展为多个主题：

```typescript
type ThemeMode = 'light' | 'dark' | 'auto';
const themeMode = ref<ThemeMode>('light');
```

### 自动主题切换

根据系统主题自动切换：

```typescript
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)');
if (themeMode.value === 'auto') {
  isDark.value = prefersDark.matches;
}
```

## 测试清单

- [x] 切换主题后 Header 颜色改变
- [x] 切换主题后 Content 背景改变
- [x] 切换主题后卡片颜色改变
- [x] 切换主题后表格样式改变
- [x] 切换主题后展开行样式改变
- [x] 切换主题后 Footer 颜色改变
- [x] 主题配置保存到 localStorage
- [x] 刷新页面后主题保持
- [x] 所有过渡动画流畅
- [x] 文字在两种模式下都清晰可读
