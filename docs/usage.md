# 使用指南

本文档介绍如何使用这个 Leptos + Vite 模板进行开发和部署。

## 开发流程

### 1. 本地开发

```bash
# 启动开发服务器
pnpm dev
```

这将启动：
- Rust WASM 监视和自动编译
- Vite 开发服务器（端口 3000）
- 热重载功能

### 2. 构建测试

```bash
# 构建项目
pnpm build

# 构建并验证
pnpm run build:verify

# 预览构建结果
pnpm preview
```

### 3. 工作流测试

```bash
# 测试 GitHub Actions 工作流配置
pnpm run test:workflow
```

## 部署流程

### 自动部署

1. **创建标签并推送**：
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **监控部署**：
   - 访问 GitHub 仓库的 Actions 标签
   - 查看工作流运行状态

3. **访问应用**：
   - 部署成功后访问 `https://[用户名].github.io/[仓库名]/`

### 手动部署

1. 进入 GitHub 仓库的 Actions 标签
2. 选择 "Build and Deploy to GitHub Pages" 工作流
3. 点击 "Run workflow" 按钮

## 项目结构说明

```
leptos-vite-template/
├── .github/
│   └── workflows/
│       └── deploy.yml          # GitHub Actions 工作流
├── docs/
│   ├── deployment.md           # 部署指南
│   └── usage.md               # 使用指南
├── scripts/
│   ├── verify-build.js        # 构建验证脚本
│   └── test-workflow.js       # 工作流测试脚本
├── src/                       # Rust 源代码
├── pkg/                       # 生成的 WASM 文件
├── dist/                      # 构建输出
├── main.ts                    # TypeScript 入口
├── index.html                 # HTML 模板
├── vite.config.ts            # Vite 配置
├── Cargo.toml                # Rust 配置
└── package.json              # Node.js 配置
```

## 配置说明

### Vite 配置 (vite.config.ts)

关键配置项：

```typescript
// GitHub Pages 路径配置
base: process.env.NODE_ENV === 'production' && process.env.GITHUB_REPOSITORY 
  ? `/${process.env.GITHUB_REPOSITORY.split('/')[1]}/`
  : '/',

// WASM 文件处理
assetsInclude: ["**/*.wasm"],

// 依赖优化
optimizeDeps: {
  exclude: ["./pkg/*.js", "./pkg/*.wasm"],
},
```

### Cargo 配置 (Cargo.toml)

生产优化配置：

```toml
[profile.release]
lto = true
opt-level = "z"
debug = false
codegen-units = 1
panic = "abort"
strip = "symbols"
```

### GitHub Actions 配置

工作流特性：
- 🏷️ 标签推送触发
- 🔧 手动触发支持
- 📦 智能缓存
- 🦀 Rust nightly 工具链
- ⚡ 快速构建和部署

## 常用命令

### 开发命令

```bash
# 开发服务器
pnpm dev

# 只构建 WASM
pnpm run wasm:build

# 监视 WASM 变化
pnpm run wasm:watch

# 清理构建文件
pnpm clean
```

### 构建命令

```bash
# 生产构建
pnpm build

# 构建分析
pnpm run build:analyze

# 构建验证
pnpm run build:verify
```

### 测试命令

```bash
# 工作流配置测试
pnpm run test:workflow

# 预览构建结果
pnpm preview
```

## 自定义配置

### 修改应用标题

编辑 `index.html`：

```html
<title>你的应用名称</title>
```

### 修改仓库信息

编辑 `Cargo.toml`：

```toml
[package]
name = "your-app-name"
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/username/repo-name"
```

### 添加自定义样式

创建 CSS 文件并在 `main.ts` 中导入：

```typescript
import './styles/main.css';
```

### 配置环境变量

在 `vite.config.ts` 中添加：

```typescript
define: {
  __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
  __CUSTOM_VAR__: JSON.stringify(process.env.CUSTOM_VAR || 'default'),
},
```

## 故障排除

### 常见问题

1. **WASM 编译失败**
   - 检查 Rust 工具链是否正确安装
   - 确保 `wasm32-unknown-unknown` 目标已添加

2. **热重载不工作**
   - 检查 `pkg/` 目录是否在 Vite 监视列表中
   - 重启开发服务器

3. **部署后页面空白**
   - 检查浏览器控制台错误
   - 验证 base 路径配置是否正确

4. **GitHub Actions 失败**
   - 检查工作流日志
   - 运行 `pnpm run test:workflow` 验证配置

### 调试技巧

1. **本地调试**：
   ```bash
   # 启用详细日志
   RUST_LOG=debug pnpm dev
   ```

2. **构建调试**：
   ```bash
   # 生成 source map
   pnpm build --sourcemap
   ```

3. **WASM 调试**：
   ```bash
   # 使用开发模式构建 WASM
   wasm-pack build --dev --target web
   ```

## 最佳实践

### 开发实践

1. **频繁提交**：小步快跑，频繁提交代码
2. **标签管理**：使用语义化版本标签
3. **测试先行**：在部署前本地测试构建
4. **文档更新**：及时更新 README 和文档

### 性能优化

1. **代码分割**：合理使用动态导入
2. **资源优化**：压缩图片和其他静态资源
3. **缓存策略**：利用浏览器缓存
4. **WASM 优化**：使用 release 模式构建

### 安全实践

1. **依赖更新**：定期更新依赖包
2. **权限最小化**：只授予必要的权限
3. **敏感信息**：不要在代码中硬编码敏感信息
4. **HTTPS**：始终使用 HTTPS 部署