# 安装配置指南# 项目设置指南



> **最后更新**: 2025-10-02## 安装依赖



本指南将帮助你快速搭建 EV Charger Simulator 的开发环境。在项目根目录运行：



---```bash

npm install

## 📋 系统要求```



### 操作系统## 开发模式

- Windows 10/11 (x64)

- macOS 11+ (Intel/Apple Silicon)启动开发服务器：

- Linux (主流发行版)

```bash

### 开发工具npm run tauri:dev

- **Node.js** 18+ ```

- **Rust** 1.70+

- **Git** 2.30+这将同时启动前端开发服务器和 Tauri 应用程序。



---## 构建应用



## 🔧 环境搭建构建生产版本：



### 1. 安装 Rust```bash

npm run tauri:build

**Windows**:```

```powershell

# 下载并运行 rustup-init.exe构建完成后，可执行文件将在 `src-tauri/target/release` 目录中。

# https://rust-lang.org/tools/install

## 项目结构说明

# 或使用 winget

winget install Rustlang.Rustup```

```ev-charger-simulator/

├── src/                    # Vue 前端源代码

**macOS/Linux**:│   ├── App.vue            # 主应用组件

```bash│   ├── main.ts            # Vue 入口文件

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh│   ├── vite-env.d.ts      # TypeScript 类型声明

```│   └── styles.css         # 全局样式

├── src-tauri/             # Rust 后端代码

**验证安装**:│   ├── src/

```bash│   │   └── main.rs        # Tauri 主程序

rustc --version│   ├── Cargo.toml         # Rust 依赖配置

cargo --version│   ├── tauri.conf.json    # Tauri 配置文件

```│   └── build.rs           # 构建脚本

├── index.html             # HTML 入口

### 2. 安装 Node.js├── package.json           # Node.js 依赖配置

├── vite.config.ts         # Vite 配置

**Windows**:├── tsconfig.json          # TypeScript 配置

```powershell├── requirements.md        # 需求文档（请在此填写详细需求）

# 使用 winget└── README.md              # 项目说明

winget install OpenJS.NodeJS.LTS```



# 或从官网下载安装包## 注意事项

# https://nodejs.org/

```1. **图标文件**: 需要在 `src-tauri/icons/` 目录中放置应用图标

2. **需求文档**: 请在 `requirements.md` 中填写详细的功能需求

**macOS**:3. **环境要求**:

```bash   - Node.js >= 16

# 使用 Homebrew   - Rust >= 1.70

brew install node@18   - 安装 Tauri CLI: `npm install -g @tauri-apps/cli`

```

## 技术栈

**Linux**:

```bash- **前端**: Vue 3 + TypeScript + Vite

# Ubuntu/Debian- **后端**: Rust (Tauri)

curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -- **构建工具**: Vite 5.x

sudo apt-get install -y nodejs- **类型检查**: vue-tsc



# Fedora/RHEL## 下一步

sudo dnf install nodejs

```1. 安装依赖: `npm install`

2. 在 `requirements.md` 中填写详细需求

**验证安装**:3. 根据需求设计数据模型和 API

```bash4. 开发 Vue 前端界面组件

node --version5. 实现 Rust 后端逻辑

npm --version6. 测试和优化

```

## 常用命令

### 3. 安装系统依赖

```bash

**Windows**:# 仅启动前端开发服务器

```powershellnpm run dev

# 需要安装 WebView2（Windows 10 已内置）

# 如需手动安装：# 构建前端

# https://developer.microsoft.com/microsoft-edge/webview2/npm run build

```

# 预览构建结果

**macOS**:npm run preview

```bash

# 无需额外依赖# Tauri 开发模式

```npm run tauri:dev



**Linux (Debian/Ubuntu)**:# Tauri 构建

```bashnpm run tauri:build

sudo apt update```

sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Linux (Fedora)**:
```bash
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
sudo dnf group install "C Development Tools and Libraries"
```

---

## 📦 获取代码

### 克隆仓库

```bash
git clone https://github.com/yourusername/ev-charger-simulator.git
cd ev-charger-simulator
```

### 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 或使用 pnpm（更快）
npm install -g pnpm
pnpm install
```

---

## 🚀 运行项目

### 开发模式

```bash
# 启动开发服务器（热重载）
npm run tauri:dev
```

这将：
1. 启动 Vite 开发服务器（前端）
2. 编译 Rust 代码
3. 启动 Tauri 应用程序
4. 支持热重载（前后端）

**首次启动**可能需要 3-5 分钟编译 Rust 依赖，请耐心等待。

### 构建生产版本

```bash
# 构建应用程序
npm run tauri:build
```

构建产物位置：
- **Windows**: `src-tauri/target/release/ev-charger-simulator.exe`
- **macOS**: `src-tauri/target/release/bundle/macos/EV Charger Simulator.app`
- **Linux**: `src-tauri/target/release/ev-charger-simulator`

---

## 🛠️ 开发工具

### VS Code (推荐)

**必装插件**:
- **rust-analyzer** - Rust 语言服务器
- **Volar** - Vue 3 支持
- **ESLint** - JavaScript/TypeScript 代码检查
- **Prettier** - 代码格式化
- **Tauri** - Tauri 开发支持

**推荐插件**:
- **Error Lens** - 行内错误显示
- **GitLens** - Git 增强
- **Todo Tree** - TODO 高亮

**VS Code 设置** (`.vscode/settings.json`):
```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

### 其他 IDE

- **WebStorm** - 前端开发
- **RustRover** - Rust 专用 IDE
- **CLion** - C++/Rust IDE（需要 Rust 插件）

---

## 📁 项目结构

```
ev-charger-simulator/
├── src/                        # Vue 前端源代码
│   ├── App.vue                # 主应用组件（~2000行）
│   ├── main.ts                # Vue 入口文件
│   ├── styles.css             # 全局样式
│   └── vite-env.d.ts          # TypeScript 类型声明
│
├── src-tauri/                  # Rust 后端代码
│   ├── src/
│   │   ├── main.rs            # Tauri 主程序
│   │   ├── state.rs           # 充电桩状态定义
│   │   ├── charger.rs         # 充电桩核心逻辑
│   │   ├── manager.rs         # 充电桩管理器
│   │   ├── protocol.rs        # 协议抽象层
│   │   ├── ocpp_client.rs     # OCPP 客户端
│   │   ├── ocpp_messages.rs   # OCPP 消息定义
│   │   ├── script_engine.rs   # Deno 脚本引擎
│   │   ├── config_loader.rs   # 配置加载器
│   │   └── commands.rs        # Tauri 命令
│   ├── Cargo.toml             # Rust 依赖配置
│   ├── tauri.conf.json        # Tauri 应用配置
│   └── build.rs               # 构建脚本
│
├── config/                     # 配置文件
│   └── chargers.json          # 充电桩配置
│
├── scripts/                    # JavaScript 脚本
│   ├── basic_test.js          # 基础测试
│   ├── normal_charging.js     # 正常充电
│   ├── fast_charging.js       # 快速充电
│   └── fault_test.js          # 故障测试
│
├── docs/                       # 文档目录
│   ├── OCPP_IMPLEMENTATION.md # OCPP 实现文档
│   ├── SCRIPT_API.md          # 脚本 API 文档
│   └── UI_NEXT_STEPS.md       # UI 开发计划
│
├── index.html                  # HTML 入口
├── package.json                # Node.js 依赖配置
├── vite.config.ts              # Vite 配置
├── tsconfig.json               # TypeScript 配置
├── tsconfig.node.json          # Node TypeScript 配置
├── .gitignore                  # Git 忽略规则
├── README.md                   # 项目说明
├── FEATURES.md                 # 功能索引
├── DOCS_INDEX.md               # 文档导航
└── requirements.md             # 需求与实现状态
```

---

## 🔍 常见问题

### 1. Rust 编译失败

**问题**: 首次编译 Rust 代码时间过长或失败

**解决**:
```bash
# 更新 Rust 工具链
rustup update

# 清理构建缓存
cd src-tauri
cargo clean
cargo build
```

### 2. npm install 失败

**问题**: 网络问题导致依赖安装失败

**解决**:
```bash
# 使用国内镜像
npm config set registry https://registry.npmmirror.com

# 或使用 cnpm
npm install -g cnpm --registry=https://registry.npmmirror.com
cnpm install
```

### 3. Tauri 启动失败

**问题**: WebView2 未安装（Windows）

**解决**:
- 从 [Microsoft 官网](https://developer.microsoft.com/microsoft-edge/webview2/) 下载并安装 WebView2 Runtime

### 4. 端口冲突

**问题**: 默认端口 1420 被占用

**解决**:
修改 `vite.config.ts` 中的端口配置：
```typescript
export default defineConfig({
  server: {
    port: 3000,  // 改为其他端口
  },
});
```

### 5. 热重载不工作

**问题**: 修改代码后不自动刷新

**解决**:
```bash
# 重启开发服务器
# Ctrl+C 停止，然后重新运行
npm run tauri:dev
```

---

## 📚 配置文件说明

### package.json

定义 Node.js 依赖和脚本：
- `dependencies` - 运行时依赖
- `devDependencies` - 开发时依赖
- `scripts` - npm 脚本命令

### Cargo.toml

定义 Rust 依赖：
- `[dependencies]` - Rust crate 依赖
- `[features]` - 功能特性开关

### tauri.conf.json

Tauri 应用配置：
- `productName` - 应用名称
- `identifier` - 应用标识符
- `build` - 构建配置
- `tauri.windows` - 窗口配置
- `tauri.bundle` - 打包配置

### vite.config.ts

Vite 构建工具配置：
- `plugins` - Vite 插件
- `server` - 开发服务器配置
- `build` - 构建选项

### tsconfig.json

TypeScript 编译器配置：
- `compilerOptions` - 编译选项
- `include` - 包含的文件
- `exclude` - 排除的文件

---

## 🧪 测试

### 运行测试

```bash
# Rust 单元测试
cd src-tauri
cargo test

# 查看测试覆盖率
cargo tarpaulin --out Html
```

### 代码检查

```bash
# Rust 代码检查
cd src-tauri
cargo clippy

# TypeScript/Vue 代码检查
npm run lint

# 代码格式化
npm run format
```

---

## 📦 打包发布

### Windows

```bash
npm run tauri:build

# 生成文件:
# src-tauri/target/release/bundle/msi/ev-charger-simulator_0.1.0_x64_en-US.msi
# src-tauri/target/release/ev-charger-simulator.exe
```

### macOS

```bash
npm run tauri:build

# 生成文件:
# src-tauri/target/release/bundle/dmg/EV Charger Simulator_0.1.0_aarch64.dmg
# src-tauri/target/release/bundle/macos/EV Charger Simulator.app
```

### Linux

```bash
npm run tauri:build

# 生成文件:
# src-tauri/target/release/bundle/deb/ev-charger-simulator_0.1.0_amd64.deb
# src-tauri/target/release/bundle/appimage/ev-charger-simulator_0.1.0_amd64.AppImage
```

---

## 🔗 相关资源

### 官方文档
- [Tauri 官方文档](https://tauri.app/)
- [Vue 3 官方文档](https://vuejs.org/)
- [Rust 官方文档](https://www.rust-lang.org/)
- [Ant Design Vue](https://antdv.com/)

### 项目文档
- [README.md](README.md) - 项目总览
- [FEATURES.md](FEATURES.md) - 功能索引
- [DOCS_INDEX.md](DOCS_INDEX.md) - 文档导航
- [TECH_STACK.md](TECH_STACK.md) - 技术栈说明

### 学习资源
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Vue 3 教程](https://vuejs.org/tutorial/)
- [Tauri 示例](https://github.com/tauri-apps/tauri/tree/dev/examples)

---

## 💬 获取帮助

遇到问题？
1. 查看[常见问题](#常见问题)
2. 查看[项目文档](DOCS_INDEX.md)
3. 提交 [GitHub Issue](https://github.com/yourusername/ev-charger-simulator/issues)
4. 加入社区讨论

---

**维护者**: EV Charger Simulator Team  
**最后更新**: 2025-10-02
