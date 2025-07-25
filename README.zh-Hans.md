# Leptos + Vite 模板

[English](README.md) | [简体中文](README.zh-Hans.md)

[![部署到 GitHub Pages](https://github.com/starlight02/leptos-vite-template/actions/workflows/deploy.yml/badge.svg)](https://github.com/starlight02/leptos-vite-template/actions/workflows/deploy.yml)
[![创建发布版本](https://github.com/starlight02/leptos-vite-template/actions/workflows/release.yml/badge.svg)](https://github.com/starlight02/leptos-vite-template/actions/workflows/release.yml)

一个现代化、生产就绪的模板，用于构建基于 Leptos (Rust) 和 Vite 的 Web 应用程序，具有自动化 GitHub Pages 部署和发布管理功能。

## ✨ 特性

### 🦀 核心技术
- **Leptos 0.8** - 支持 nightly 特性的 Rust 响应式 Web 框架
- **Vite** - 集成 rolldown 的快速构建工具
- **WebAssembly** - 具有大小优化的高性能 WASM
- **TypeScript** - 类型安全的 JavaScript 开发
- **MDUI** - Material Design UI 组件

### 🔥 开发体验
- **热重载** - Rust 和前端代码的即时重载
- **智能构建脚本** - 环境感知的构建系统
- **高级缓存** - 优化的依赖和构建缓存
- **自动 WASM 导入** - 无缝的 WASM 模块集成
- **开发工具** - 集成的代码检查、格式化和调试

### 🚀 生产和部署
- **自动化发布** - 基于标签的发布创建和校验和
- **GitHub Pages** - 优化构建的自动化部署
- **多环境支持** - 开发、生产和 GitHub Pages 配置
- **安全性** - SHA256/SHA1 校验和确保发布完整性
- **文档** - 自动生成的变更日志和安装指南

## 📋 前置要求

确保已安装以下工具：

```bash
# 安装 Rust nightly 工具链
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown

# 安装必要的 WASM 工具
cargo install wasm-pack wasm-bindgen-cli cargo-watch

# 安装 Node.js 包管理器
npm install -g pnpm

# 可选：安装额外的开发工具
cargo install leptosfmt  # Leptos 代码格式化工具
```

## 🚀 快速开始

1. **克隆和设置**：
   ```bash
   git clone https://github.com/starlight02/leptos-vite-template.git
   cd leptos-vite-template
   pnpm install
   ```

2. **启动开发**：
   ```bash
   pnpm dev
   ```

3. **打开浏览器**：访问 `http://localhost:5173`

## 📜 可用脚本

### 开发
- `pnpm dev` - 启动带热重载的开发服务器
- `pnpm wasm:watch` - 仅监视和重建 WASM
- `pnpm wasm:build:dev` - 为开发构建 WASM

### 生产
- `pnpm build` - 为生产部署构建
- `pnpm build:dev` - 为开发测试构建
- `pnpm build:github` - 为 GitHub Pages 优化构建
- `pnpm preview` - 本地预览生产构建

### WASM 专用
- `pnpm wasm:build` - 为生产构建 WASM
- `pnpm wasm:build:github` - 为 GitHub Pages 构建 WASM

### 维护
- `pnpm clean` - 清理构建产物 (pkg, dist, target/wasm32-unknown-unknown)

## 📁 项目结构

```
leptos-vite-template/
├── src/                  # Rust 源代码
│   ├── lib.rs            # 主 Leptos 应用程序
│   ├── components/       # 可重用的 Leptos 组件
│   ├── pages/            # 页面组件和路由
│   ├── assets/           # 静态资源
│   ├── bindings/         # WASM 绑定
│   ├── plugins/          # 自定义插件
│   └── env.rs            # 环境配置
├── scripts/              # 构建和实用脚本
│   ├── build-all.js      # 完整构建编排
│   ├── build-wasm.js     # WASM 特定构建逻辑
│   └── env-loader.js     # 环境配置加载器
├── plugins/              # 自定义 Vite 插件
│   └── vite-plugin-wasm-import.ts
├── .github/workflows/    # CI/CD 自动化
│   ├── deploy.yml        # GitHub Pages 部署
│   └── release.yml       # 自动化发布创建
├── public/               # 静态公共资源
│   └── icons/            # 应用程序图标
├── pkg/                  # 生成的 WASM 文件
├── dist/                 # 生产构建输出
├── main.ts               # TypeScript 入口点
├── index.html            # HTML 模板
├── vite.config.ts        # Vite 配置
├── Cargo.toml            # Rust 项目配置
└── package.json          # Node.js 依赖
```

## 🔧 工作原理

### 智能构建系统
1. **环境检测**：自动为开发、生产或 GitHub Pages 配置构建
2. **WASM 优化**：生产环境使用 `wasm-opt` 进行大小优化 (`-Oz`)
3. **缓存策略**：Rust 依赖、WASM 工具和 Node 模块的智能缓存
4. **热重载**：监控 Rust 源代码变化并自动重建 WASM

### 自动 WASM 集成
- **虚拟模块**：自定义 Vite 插件创建 `virtual:wasm-init` 实现无缝导入
- **自动检测**：发现 `pkg/` 目录中的 WASM 模块
- **HMR 支持**：WASM 变化的热模块替换
- **错误处理**：优雅的回退和错误报告

### 多环境支持
- **开发环境**：带调试符号的快速构建
- **生产环境**：使用 LTO 和大小优化的优化构建
- **GitHub Pages**：具有正确基础路径和路由的特殊构建

## 🚀 部署

### 自动化 GitHub Pages 部署

**触发方式**：
- 推送以 `v` 开头的任何标签（例如 `v1.0.0`）
- 从 GitHub Actions 选项卡手动触发

**流程**：
1. 🔧 **环境设置** - 安装 Rust nightly、Node.js、pnpm、WASM 工具
2. 📦 **智能缓存** - 缓存依赖以加快后续构建
3. 🦀 **WASM 构建** - 将 Rust 编译为优化的 WebAssembly
4. ⚡ **前端构建** - 构建和优化前端资源
5. ✅ **验证** - 验证构建产物和结构
6. 🚀 **部署** - 发布到 GitHub Pages

**要求**：
1. 在仓库设置中启用 GitHub Pages
2. 将源设置为 "GitHub Actions"
3. 确保仓库具有适当的权限

### 自动化发布管理

**创建发布**：
```bash
# 创建并推送签名标签
git tag -s v1.0.0 -m "Release v1.0.0 - 描述"
git push origin v1.0.0
```

**您将获得**：
- 📦 **优化的 ZIP 包** - 标准命名的生产构建
- 🔐 **安全校验和** - SHA256 和 SHA1 文件用于完整性验证
- 📝 **详细变更日志** - 自动生成的提交分类
- 📖 **安装指南** - 完整的设置和验证说明
- 🔗 **GitHub 集成** - 带下载链接的适当发布页面

**验证**：
```bash
# 验证下载的文件
sha256sum -c leptos-vite-template-v1.0.0-wasm32-unknown-unknown.zip.sha256
sha1sum -c leptos-vite-template-v1.0.0-wasm32-unknown-unknown.zip.sha1
```

## 🛠️ 开发技巧

### 性能优化
- 使用 `cargo watch` 进行快速增量 WASM 构建
- 利用 Vite 的 HMR 进行即时前端更新
- 启用 Rust analyzer 以获得更好的 IDE 支持

### 调试
- 检查浏览器控制台中的 WASM 初始化错误
- 使用开发构建获得更好的错误消息
- 监控 GitHub Actions 日志以解决部署问题

### 自定义
- 修改 `vite.config.ts` 进行构建自定义
- 更新 `.env.*` 文件进行环境特定设置
- 扩展 `scripts/` 以实现自定义构建逻辑

## 🤝 贡献

1. Fork 仓库
2. 创建功能分支
3. 进行更改
4. 使用 `pnpm build` 和 `pnpm dev` 进行测试
5. 提交 pull request

## 📄 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

**在线演示**：[https://starlight02.github.io/leptos-vite-template/](https://starlight02.github.io/leptos-vite-template/)