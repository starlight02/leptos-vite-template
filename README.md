# Leptos + Vite Template

[English](README.md) | [简体中文](README.zh-Hans.md)

[![Deploy to GitHub Pages](https://github.com/starlight02/leptos-vite-template/actions/workflows/deploy.yml/badge.svg)](https://github.com/starlight02/leptos-vite-template/actions/workflows/deploy.yml)
[![Create Release](https://github.com/starlight02/leptos-vite-template/actions/workflows/release.yml/badge.svg)](https://github.com/starlight02/leptos-vite-template/actions/workflows/release.yml)

A modern, production-ready template for building web applications with Leptos (Rust) and Vite, featuring automated GitHub Pages deployment and release management.

## ✨ Features

### 🦀 Core Technologies
- **Leptos 0.8** - Reactive web framework for Rust with nightly features
- **Vite** - Fast build tool with rolldown integration
- **WebAssembly** - High-performance WASM with size optimization
- **TypeScript** - Type-safe JavaScript development
- **MDUI** - Material Design UI components

### 🔥 Development Experience
- **Hot Reload** - Instant reload for both Rust and frontend changes
- **Smart Build Scripts** - Environment-aware build system
- **Advanced Caching** - Optimized dependency and build caching
- **Auto WASM Import** - Seamless WASM module integration
- **Development Tools** - Integrated linting, formatting, and debugging

### 🚀 Production & Deployment
- **Automated Releases** - Tag-based release creation with checksums
- **GitHub Pages** - Automated deployment with optimized builds
- **Multi-Environment** - Development, production, and GitHub Pages configs
- **Security** - SHA256/SHA1 checksums for release integrity
- **Documentation** - Auto-generated changelogs and installation guides

## 📋 Prerequisites

Ensure you have the following tools installed:

```bash
# Install Rust with nightly toolchain
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown

# Install essential WASM tools
cargo install wasm-pack wasm-bindgen-cli cargo-watch

# Install Node.js package manager
npm install -g pnpm

# Optional: Install additional development tools
cargo install leptosfmt  # Leptos code formatter
```

## 🚀 Quick Start

1. **Clone and setup**:
   ```bash
   git clone https://github.com/starlight02/leptos-vite-template.git
   cd leptos-vite-template
   pnpm install
   ```

2. **Start development**:
   ```bash
   pnpm dev
   ```

3. **Open browser**: Navigate to `http://localhost:5173`

## 📜 Available Scripts

### Development
- `pnpm dev` - Start development server with hot reload
- `pnpm wasm:watch` - Watch and rebuild WASM only
- `pnpm wasm:build:dev` - Build WASM for development

### Production
- `pnpm build` - Build for production deployment
- `pnpm build:dev` - Build for development testing
- `pnpm build:github` - Build optimized for GitHub Pages
- `pnpm preview` - Preview production build locally

### WASM Specific
- `pnpm wasm:build` - Build WASM for production
- `pnpm wasm:build:github` - Build WASM for GitHub Pages

### Maintenance
- `pnpm clean` - Clean build artifacts (pkg, dist, target/wasm32-unknown-unknown)

## 📁 Project Structure

```
leptos-vite-template/
├── src/                    # Rust source code
│   ├── lib.rs             # Main Leptos application
│   ├── components/        # Reusable Leptos components
│   ├── pages/            # Page components and routing
│   ├── assets/           # Static assets
│   ├── bindings/         # WASM bindings
│   ├── plugins/          # Custom plugins
│   └── env.rs            # Environment configuration
├── scripts/              # Build and utility scripts
│   ├── build-all.js      # Complete build orchestration
│   ├── build-wasm.js     # WASM-specific build logic
│   └── env-loader.js     # Environment configuration loader
├── plugins/              # Custom Vite plugins
│   └── vite-plugin-wasm-import.ts
├── .github/workflows/    # CI/CD automation
│   ├── deploy.yml        # GitHub Pages deployment
│   └── release.yml       # Automated release creation
├── public/               # Static public assets
│   └── icons/           # Application icons
├── pkg/                  # Generated WASM files
├── dist/                 # Production build output
├── main.ts              # TypeScript entry point
├── index.html           # HTML template
├── vite.config.ts       # Vite configuration
├── Cargo.toml           # Rust project configuration
└── package.json         # Node.js dependencies
```

## 🔧 How It Works

### Smart Build System
1. **Environment Detection**: Automatically configures builds for development, production, or GitHub Pages
2. **WASM Optimization**: Uses `wasm-opt` with size optimization (`-Oz`) for production
3. **Caching Strategy**: Intelligent caching for Rust dependencies, WASM tools, and Node modules
4. **Hot Reload**: Monitors Rust source changes and rebuilds WASM automatically

### Auto WASM Integration
- **Virtual Modules**: Custom Vite plugin creates `virtual:wasm-init` for seamless imports
- **Automatic Detection**: Discovers WASM modules in `pkg/` directory
- **HMR Support**: Hot module replacement for WASM changes
- **Error Handling**: Graceful fallbacks and error reporting

### Multi-Environment Support
- **Development**: Fast builds with debugging symbols
- **Production**: Optimized builds with LTO and size optimization  
- **GitHub Pages**: Special builds with correct base paths and routing

## 🚀 Deployment

### Automated GitHub Pages Deployment

**Trigger Methods**:
- Push any tag starting with `v` (e.g., `v1.0.0`)
- Manual trigger from GitHub Actions tab

**Process**:
1. 🔧 **Environment Setup** - Installs Rust nightly, Node.js, pnpm, WASM tools
2. 📦 **Smart Caching** - Caches dependencies for faster subsequent builds
3. 🦀 **WASM Build** - Compiles Rust to optimized WebAssembly
4. ⚡ **Frontend Build** - Builds and optimizes frontend assets
5. ✅ **Verification** - Validates build artifacts and structure
6. 🚀 **Deployment** - Publishes to GitHub Pages

**Requirements**:
1. Enable GitHub Pages in repository settings
2. Set source to "GitHub Actions"
3. Ensure repository has proper permissions

### Automated Release Management

**Creating Releases**:
```bash
# Create and push a signed tag
git tag -s v1.0.0 -m "Release v1.0.0 - Description"
git push origin v1.0.0
```

**What You Get**:
- 📦 **Optimized ZIP Package** - Production build with standard naming
- 🔐 **Security Checksums** - SHA256 and SHA1 files for integrity verification
- 📝 **Detailed Changelog** - Auto-generated with commit categorization
- 📖 **Installation Guide** - Complete setup and verification instructions
- 🔗 **GitHub Integration** - Proper release pages with download links

**Verification**:
```bash
# Verify downloaded files
sha256sum -c leptos-vite-template-v1.0.0-wasm32-unknown-unknown.zip.sha256
sha1sum -c leptos-vite-template-v1.0.0-wasm32-unknown-unknown.zip.sha1
```

## 🛠️ Development Tips

### Performance Optimization
- Use `cargo watch` for fast incremental WASM builds
- Leverage Vite's HMR for instant frontend updates
- Enable Rust analyzer for better IDE support

### Debugging
- Check browser console for WASM initialization errors
- Use development builds for better error messages
- Monitor GitHub Actions logs for deployment issues

### Customization
- Modify `vite.config.ts` for build customization
- Update `.env.*` files for environment-specific settings
- Extend `scripts/` for custom build logic

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with `pnpm build` and `pnpm dev`
5. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Live Demo**: [https://starlight02.github.io/leptos-vite-template/](https://starlight02.github.io/leptos-vite-template/)