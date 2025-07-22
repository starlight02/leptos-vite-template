# Leptos + Vite Template

[![Deploy to GitHub Pages](https://github.com/starlight02/leptos-vite-template/actions/workflows/deploy.yml/badge.svg)](https://github.com/starlight02/leptos-vite-template/actions/workflows/deploy.yml)

A modern template for building web applications with Leptos (Rust) and Vite with automated GitHub Pages deployment.

## Features

- 🦀 **Leptos** - Reactive web framework for Rust
- ⚡ **Vite** - Fast build tool and dev server
- 🔥 **Hot Reload** - Instant reload when Rust code changes
- 📦 **pnpm** - Fast, disk space efficient package manager
- 🚀 **WASM** - WebAssembly for high performance
- 🛠️ **TypeScript** - Type-safe JavaScript
- 🤖 **Auto WASM Import** - Automatically imports and initializes WASM modules
- 🚀 **GitHub Actions** - Automated build and deployment to GitHub Pages

## Prerequisites

Make sure you have the following tools installed:

```bash
# Add wasm target
rustup target add wasm32-unknown-unknown

# Install some develop and build tools
cargo install wasm-pack wasm-bindgen-cli wasm-opt cargo-watch leptosfmt

# Install pnpm
npm install -g pnpm
```

## Getting Started

1. Clone the repository
2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Start development server:
   ```bash
   pnpm dev
   ```

4. Open your browser at `http://localhost:3000`

## Scripts

- `pnpm dev` - Start development server with hot reload
- `pnpm build` - Build for production
- `pnpm preview` - Preview production build
- `pnpm clean` - Clean build artifacts
- `pnpm wasm:build` - Build WASM only
- `pnpm wasm:watch` - Watch and rebuild WASM

## Project Structure

```
├── src/           # Rust source code
│   ├── lib.rs     # Main Leptos app
│   ├── components/ # Leptos components
│   └── pages/     # Page components
├── pkg/           # Generated WASM files
├── dist/          # Built files
├── main.ts        # TypeScript entry point
├── index.html     # HTML template
├── vite.config.ts # Vite configuration
├── Cargo.toml     # Rust dependencies
└── package.json   # Node.js dependencies
```

## How It Works

1. **Rust/WASM**: `cargo-watch` monitors Rust source files and rebuilds WASM using `wasm-pack`
2. **Auto Import**: Our custom Vite plugin automatically injects WASM initialization code into the HTML
3. **Hot Reload**: Vite watches the generated WASM files and hot-reloads the browser
4. **TypeScript**: Provides type safety for the frontend code

## Auto WASM Import

This template includes a custom Vite plugin that automatically:

- 🔍 **Detects** WASM modules in the `pkg/` directory
- 💫 **Injects** initialization code using virtual modules
- 🔄 **Reloads** the browser when WASM files change
- ⚙️ **Configures** hot module replacement for seamless development

**No manual imports needed!** Just import `'virtual:wasm-init'` and the plugin handles everything automatically.

### How It Works

1. The plugin creates a virtual module `virtual:wasm-init`
2. This module automatically imports the correct WASM file (using underscore naming: `leptos_vite_template.js`)
3. WASM is initialized automatically when the module is imported
4. Hot reload is configured to refresh on WASM changes

## Deployment

This template includes automated deployment to GitHub Pages using GitHub Actions.

### Automatic Deployment

- **Trigger**: Push a tag to trigger deployment (e.g., `git tag v1.0.0 && git push origin v1.0.0`)
- **Manual**: Use the "Actions" tab in GitHub to manually trigger deployment
- **URL**: Your app will be available at `https://[username].github.io/[repository-name]/`

### Deployment Process

1. 🔧 **Setup**: Installs Rust nightly, Node.js, pnpm, and WASM tools
2. 📦 **Cache**: Uses intelligent caching for faster builds
3. 🦀 **Build**: Compiles Rust to WASM and builds frontend assets
4. ✅ **Verify**: Validates build artifacts
5. 🚀 **Deploy**: Publishes to GitHub Pages

### Requirements

Make sure to:
1. Enable GitHub Pages in your repository settings
2. Set the source to "GitHub Actions"
3. Push a tag to trigger the first deployment

## License

MIT
