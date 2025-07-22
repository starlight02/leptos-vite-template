# GitHub Pages 部署指南

本文档详细说明如何使用 GitHub Actions 自动部署 Leptos + Vite 项目到 GitHub Pages。

## 快速开始

### 1. 启用 GitHub Pages

1. 进入你的 GitHub 仓库
2. 点击 **Settings** 标签
3. 在左侧菜单中找到 **Pages**
4. 在 **Source** 部分选择 **GitHub Actions**

### 2. 触发部署

有两种方式触发部署：

#### 方式一：推送标签（推荐）
```bash
# 创建并推送标签
git tag v1.0.0
git push origin v1.0.0
```

#### 方式二：手动触发
1. 进入仓库的 **Actions** 标签
2. 选择 **Build and Deploy to GitHub Pages** 工作流
3. 点击 **Run workflow** 按钮

### 3. 监控部署状态

- 在 **Actions** 标签中查看工作流运行状态
- 部署成功后，应用将在 `https://[用户名].github.io/[仓库名]/` 可用

## 工作流详解

### 触发条件

```yaml
on:
  push:
    tags:
      - '*'          # 任何标签推送都会触发
  workflow_dispatch:   # 支持手动触发
```

### 构建环境

- **操作系统**: Ubuntu Latest
- **Node.js**: 20.x LTS
- **Rust**: Nightly 版本
- **包管理器**: pnpm

### 构建步骤

1. **环境准备**
   - 检出代码
   - 设置 Node.js 和 pnpm
   - 安装 Rust 工具链

2. **工具安装**
   - wasm-pack
   - wasm-bindgen-cli
   - wasm-opt

3. **缓存优化**
   - Rust 依赖缓存
   - pnpm 依赖缓存

4. **构建项目**
   - 安装依赖
   - 编译 WASM
   - 构建前端资源

5. **验证和部署**
   - 验证构建产物
   - 部署到 GitHub Pages

## 缓存策略

为了提高构建速度，工作流使用了多层缓存：

### Rust 缓存
```yaml
- name: Cache Rust dependencies
  uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

### pnpm 缓存
```yaml
- name: Cache pnpm dependencies
  uses: actions/cache@v4
  with:
    path: ${{ env.STORE_PATH }}
    key: ${{ runner.os }}-pnpm-store-${{ hashFiles('**/pnpm-lock.yaml') }}
```

## 路径配置

项目自动配置 GitHub Pages 的正确路径：

### Vite 配置
```typescript
base: process.env.NODE_ENV === 'production' && process.env.GITHUB_REPOSITORY 
  ? `/${process.env.GITHUB_REPOSITORY.split('/')[1]}/`
  : '/',
```

### 环境变量
在构建时设置：
```yaml
env:
  NODE_ENV: production
  GITHUB_REPOSITORY: ${{ github.repository }}
```

## 故障排除

### 常见问题

#### 1. 部署失败：权限错误
**解决方案**: 确保在仓库设置中启用了 GitHub Pages 并选择 "GitHub Actions" 作为源。

#### 2. 页面显示 404
**解决方案**: 检查 `vite.config.ts` 中的 `base` 配置是否正确。

#### 3. WASM 文件加载失败
**解决方案**: 确保 WASM 文件包含在构建产物中，检查 `assetsInclude` 配置。

#### 4. 构建时间过长
**解决方案**: 检查缓存是否正常工作，确保 `Cargo.lock` 和 `pnpm-lock.yaml` 文件已提交。

### 调试步骤

1. **检查工作流日志**
   - 进入 Actions 标签
   - 点击失败的工作流运行
   - 查看详细的步骤日志

2. **本地验证构建**
   ```bash
   # 本地构建测试
   pnpm run build
   
   # 验证构建产物
   node scripts/verify-build.js
   ```

3. **检查环境变量**
   ```bash
   # 在工作流中添加调试输出
   echo "Repository: $GITHUB_REPOSITORY"
   echo "Base path: /${GITHUB_REPOSITORY#*/}/"
   ```

## 自定义配置

### 修改触发条件

如果你想要在每次推送到 main 分支时都部署：

```yaml
on:
  push:
    branches: [ main ]
  workflow_dispatch:
```

### 添加环境变量

在工作流中添加自定义环境变量：

```yaml
env:
  CUSTOM_VAR: "value"
  BUILD_ENV: "production"
```

### 修改 Node.js 版本

```yaml
- name: Setup Node.js
  uses: actions/setup-node@v4
  with:
    node-version: '18'  # 或其他版本
```

## 性能优化

### 构建时间优化

1. **使用缓存**: 已默认启用 Rust 和 pnpm 缓存
2. **并行构建**: Rust 编译自动使用多核
3. **增量构建**: 缓存未变更的依赖

### 资源优化

1. **WASM 优化**: 使用 wasm-opt 进行代码优化
2. **资源压缩**: 启用 Brotli 和 Gzip 压缩
3. **代码分割**: 合理的代码分割策略

## 安全考虑

- 使用官方 GitHub Actions
- 最小权限原则
- 自动提供的 GITHUB_TOKEN
- 只从官方源安装工具

## 监控和通知

### 状态徽章

在 README 中添加状态徽章：

```markdown
[![Deploy](https://github.com/用户名/仓库名/actions/workflows/deploy.yml/badge.svg)](https://github.com/用户名/仓库名/actions/workflows/deploy.yml)
```

### 部署通知

工作流会在部署完成后输出访问 URL：

```yaml
- name: Output deployment URL
  run: |
    echo "🚀 Deployment completed successfully!"
    echo "📱 Your app is now available at: ${{ steps.deployment.outputs.page_url }}"
```