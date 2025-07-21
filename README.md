# Leptos + Vite Template

A modern template for building web applications with Leptos (Rust) and Vite.

## Features

- 🦀 **Leptos** - Reactive web framework for Rust
- ⚡ **Vite** - Fast build tool and dev server
- 🔥 **Hot Reload** - Instant reload when Rust code changes
- 📦 **pnpm** - Fast, disk space efficient package manager
- 🚀 **WASM** - WebAssembly for high performance
- 🛠️ **TypeScript** - Type-safe JavaScript
- 🤖 **Auto WASM Import** - Automatically imports and initializes WASM modules

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

## License

MIT
