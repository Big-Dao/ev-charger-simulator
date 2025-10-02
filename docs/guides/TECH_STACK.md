# 技术栈说明

> **最后更新**: 2025-10-02

---

## 📐 架构设计

```
┌──────────────────────────────────────────┐
│         桌面应用层 (Tauri)                │
├──────────────────────────────────────────┤
│                                          │
│  ┌────────────────────────────────────┐ │
│  │   前端 UI (Vue 3 + TypeScript)    │ │
│  │   - Ant Design Vue 组件库          │ │
│  │   - 响应式布局                     │ │
│  │   - 主题切换                       │ │
│  └──────────┬─────────────────────────┘ │
│             │ IPC通信                   │
│  ┌──────────▼─────────────────────────┐ │
│  │   后端服务 (Rust + Tokio)         │ │
│  │   - 充电桩管理器                   │ │
│  │   - OCPP协议客户端                 │ │
│  │   - Deno Core脚本引擎              │ │
│  └────────────────────────────────────┘ │
│                                          │
└──────────────────────────────────────────┘
```

---

## 🎨 前端技术栈

### Vue 3.4

**选择理由**:
- ✅ **Composition API**: 更好的代码组织和类型推导
- ✅ **响应式系统**: 基于 Proxy 的响应式，性能优秀
- ✅ **TypeScript支持**: 官方一流的 TypeScript 集成
- ✅ **生态成熟**: 丰富的社区插件和工具链
- ✅ **体积小**: 运行时仅 34KB (gzipped)

**核心特性**:
- `<script setup>` 语法糖
- 响应式引用 (ref, reactive)
- 计算属性 (computed)
- 侦听器 (watch, watchEffect)
- 生命周期钩子 (onMounted, onUnmounted)

### Ant Design Vue 4.1

**选择理由**:
- ✅ **企业级设计**: 专业、大气的商业风格
- ✅ **组件丰富**: 60+ 高质量组件，覆盖所有场景
- ✅ **TypeScript支持**: 完整的类型定义
- ✅ **主题定制**: 支持暗色模式，可自定义主题色
- ✅ **高分辨率优化**: 完美支持 2K/4K 显示器
- ✅ **性能优秀**: 虚拟滚动、按需加载等优化
- ✅ **文档完善**: 中英文文档齐全，示例丰富

**核心组件**:
- Layout - 布局系统（响应式栅格）
- Table - 表格（虚拟滚动，大数据量）
- Form - 表单（强大的验证）
- Modal/Drawer - 弹窗
- Card - 卡片容器
- Button - 按钮（多种类型）
- Tag - 标签（状态显示）
- Message/Notification - 消息提示

**替代方案对比**:

| 特性 | Ant Design Vue | Element Plus | Naive UI |
|------|----------------|--------------|----------|
| 设计风格 | 商业专业 ⭐⭐⭐ | 中性简洁 ⭐⭐ | 现代轻量 ⭐⭐ |
| 组件数量 | 60+ ⭐⭐⭐ | 50+ ⭐⭐ | 80+ ⭐⭐⭐ |
| TypeScript | 完善 ⭐⭐⭐ | 完善 ⭐⭐⭐ | 完善 ⭐⭐⭐ |
| 暗色模式 | 支持 ⭐⭐⭐ | 支持 ⭐⭐ | 支持 ⭐⭐⭐ |
| 文档质量 | 优秀 ⭐⭐⭐ | 良好 ⭐⭐ | 良好 ⭐⭐ |
| 企业案例 | 丰富 ⭐⭐⭐ | 一般 ⭐⭐ | 较少 ⭐ |

### TypeScript 5.3

**选择理由**:
- ✅ **类型安全**: 编译期捕获错误
- ✅ **智能提示**: IDE 自动补全和重构
- ✅ **可维护性**: 代码结构清晰，易于重构
- ✅ **工具链**: ESLint、Prettier 完善支持

**配置**:
```json
{
  "strict": true,  // 严格模式
  "noImplicitAny": true,  // 禁止隐式 any
  "strictNullChecks": true,  // 严格空值检查
}
```

### Vite 7.1

**选择理由**:
- ✅ **极速启动**: 基于 ESM 的开发服务器
- ✅ **热更新快**: HMR 秒级响应
- ✅ **开箱即用**: 零配置支持 TypeScript
- ✅ **构建优化**: 基于 Rollup 的生产构建

---

## ⚙️ 后端技术栈

### Tauri 1.5

**选择理由**:
- ✅ **体积小**: 安装包仅 3-5 MB（vs Electron 100+ MB）
- ✅ **性能高**: Rust 原生性能，内存占用低
- ✅ **安全性**: Rust 内存安全保证
- ✅ **跨平台**: Windows/macOS/Linux 统一 API
- ✅ **Web 技术**: 使用熟悉的前端技术栈
- ✅ **系统集成**: 原生 API 访问能力

**vs Electron**:

| 特性 | Tauri | Electron |
|------|-------|----------|
| 安装包大小 | 3-5 MB ⭐⭐⭐ | 100+ MB ⭐ |
| 内存占用 | 50-100 MB ⭐⭐⭐ | 200-300 MB ⭐ |
| 启动速度 | 快 ⭐⭐⭐ | 慢 ⭐⭐ |
| 安全性 | 高 ⭐⭐⭐ | 中 ⭐⭐ |
| 生态成熟度 | 成长中 ⭐⭐ | 成熟 ⭐⭐⭐ |

### Rust 1.70+

**选择理由**:
- ✅ **内存安全**: 所有权系统，编译期保证
- ✅ **高性能**: 零成本抽象，接近C的性能
- ✅ **并发安全**: 编译期防止数据竞争
- ✅ **类型系统**: 强大的类型系统和模式匹配
- ✅ **工具链**: Cargo 包管理和构建工具

**核心特性**:
- 所有权 (Ownership)
- 借用检查 (Borrow Checker)
- 生命周期 (Lifetimes)
- 模式匹配 (Pattern Matching)
- 特征 (Traits)

### Tokio 1.40

**选择理由**:
- ✅ **异步运行时**: 高效的异步 I/O
- ✅ **任务调度**: 轻量级绿色线程
- ✅ **生态丰富**: 大量异步库支持
- ✅ **性能优秀**: 接近裸金属性能

**核心组件**:
- `tokio::task` - 异步任务
- `tokio::net` - 异步网络
- `tokio::time` - 异步定时器
- `tokio::sync` - 异步同步原语

### Deno Core 0.320

**选择理由**:
- ✅ **轻量高效**: Rust 实现的 JavaScript 引擎
- ✅ **现代语法**: ES6+ 全支持
- ✅ **异步支持**: 原生 async/await
- ✅ **安全沙箱**: 可控的运行时权限
- ✅ **Rust 集成**: 与 Rust 无缝集成

**vs 其他脚本引擎**:

| 特性 | Deno Core | QuickJS | Lua |
|------|-----------|---------|-----|
| 语法熟悉度 | JavaScript ⭐⭐⭐ | JavaScript ⭐⭐⭐ | Lua ⭐⭐ |
| 性能 | 高 ⭐⭐⭐ | 中 ⭐⭐ | 高 ⭐⭐⭐ |
| 异步支持 | 原生 ⭐⭐⭐ | 需扩展 ⭐ | 需扩展 ⭐ |
| 生态系统 | 丰富 ⭐⭐⭐ | 一般 ⭐⭐ | 丰富 ⭐⭐⭐ |
| Rust集成 | 优秀 ⭐⭐⭐ | 一般 ⭐⭐ | 一般 ⭐⭐ |
| 体积大小 | 中 ⭐⭐ | 小 ⭐⭐⭐ | 小 ⭐⭐⭐ |

**为什么选择 Deno Core 而非 QuickJS**:
1. **更好的异步支持**: 原生 async/await，与 Rust 的 Tokio 无缝集成
2. **现代 JavaScript**: ES6+ 全部特性支持，无需转译
3. **Rust 集成**: deno_core crate 提供完善的 Rust API
4. **官方支持**: Deno 官方维护，质量有保证
5. **调试体验**: 更好的错误信息和堆栈跟踪

### tokio-tungstenite

**选择理由**:
- ✅ **WebSocket客户端**: 标准 WebSocket 协议
- ✅ **异步非阻塞**: 基于 Tokio 的异步实现
- ✅ **性能优秀**: 零拷贝优化
- ✅ **易于使用**: 简洁的 API

### Serde

**选择理由**:
- ✅ **序列化框架**: Rust 生态标准
- ✅ **高性能**: 零拷贝，编译期优化
- ✅ **格式丰富**: JSON、YAML、TOML 等
- ✅ **类型安全**: 强类型序列化/反序列化

---

## 📦 核心依赖

### Rust (Cargo.toml)

```toml
[dependencies]
# 应用框架
tauri = { version = "1.5", features = ["window-all"] }

# 异步运行时
tokio = { version = "1.40", features = ["full"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# WebSocket客户端
tokio-tungstenite = "0.23"

# 脚本引擎
deno_core = "0.320"

# 时间处理
chrono = "0.4"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

# 高性能锁
parking_lot = "0.12"

# 异步trait
async-trait = "0.1"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"
```

### Node.js (package.json)

```json
{
  "dependencies": {
    "vue": "^3.4.0",
    "ant-design-vue": "^4.1.0",
    "pinia": "^2.1.0",
    "@vueuse/core": "^10.7.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.0.0",
    "typescript": "^5.3.0",
    "vite": "^7.1.0",
    "@tauri-apps/cli": "^1.5.0",
    "eslint": "^8.56.0",
    "prettier": "^3.1.0"
  }
}
```

---

## 🛠️ 开发工具

### IDE
- **VS Code** - 推荐
  - rust-analyzer（Rust语言服务器）
  - Volar（Vue3支持）
  - ESLint（代码检查）
  - Prettier（代码格式化）

### 构建工具
- **Cargo** - Rust 包管理和构建
- **npm/pnpm** - Node.js 包管理
- **Vite** - 前端构建工具

### 版本控制
- **Git** - 代码版本管理
- **.gitignore** - 忽略规则配置完善

---

## 🎯 技术选型原则

### 1. 性能优先
- Rust 后端保证高性能
- 异步 I/O 降低延迟
- 零拷贝优化内存使用

### 2. 类型安全
- Rust 编译期类型检查
- TypeScript 静态类型
- Serde 序列化保证

### 3. 开发体验
- 熟悉的 Web 技术栈
- 完善的工具链支持
- 丰富的文档和社区

### 4. 可维护性
- 模块化设计
- 清晰的代码结构
- 完善的文档注释

### 5. 跨平台
- Tauri 统一 API
- 避免平台特定代码
- 充分测试各平台

---

## 📊 性能指标

### 应用体积
- **安装包**: 约 5-8 MB（未压缩）
- **运行时**: 约 50-100 MB 内存
- **启动时间**: < 2 秒

### 运行性能（目标）
- **并发充电桩**: 500+
- **CPU占用**: < 80%（500桩）
- **内存占用**: < 2GB（500桩）
- **消息响应**: < 100ms
- **UI响应**: < 200ms

---

## 🔄 技术演进

### 已废弃
- ❌ QuickJS - 改用 Deno Core（更好的异步支持）
- ❌ Lua - 改用 JavaScript（开发者更熟悉）

### 考虑中
- 🤔 OCPP 2.0.1 支持
- 🤔 插件系统（WASM）
- 🤔 云端版本（Web技术栈）

---

## 📚 学习资源

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Tauri
- [Tauri Documentation](https://tauri.app/)
- [Tauri Examples](https://github.com/tauri-apps/tauri/tree/dev/examples)

### Vue 3
- [Vue 3 Documentation](https://vuejs.org/)
- [Vue 3 Composition API](https://vuejs.org/guide/extras/composition-api-faq.html)

### Ant Design Vue
- [Ant Design Vue Documentation](https://antdv.com/)
- [Ant Design Guidelines](https://ant.design/)

---

**最后更新**: 2025-10-02  
**维护者**: EV Charger Simulator Team
