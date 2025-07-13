# Leptos + Vite 自动 WASM 导入设置

这个项目展示了如何配置 Leptos + Vite 实现完全自动化的 WASM 模块导入和热重载。

## 核心特性

- ✅ **零手动导入**：使用 `import 'virtual:wasm-init'` 自动处理 WASM
- ✅ **智能热重载**：Rust 代码变化时自动刷新浏览器
- ✅ **TypeScript 支持**：完整的类型安全
- ✅ **生产构建**：开发和生产环境都完美工作

## 关键文件

### 1. `plugins/vite-plugin-wasm-import.ts`
自定义 Vite 插件，创建虚拟模块 `virtual:wasm-init`：
- 自动导入正确的 WASM 文件（`leptos_vite_template.js`）
- 处理初始化逻辑
- 配置热重载

### 2. `main.ts`
超简洁的入口文件：
```typescript
import 'virtual:wasm-init';
console.log('🚀 Main.ts loaded - WASM auto-import configured!');
```

### 3. `vite.config.ts`
配置使用自定义插件：
```typescript
import { wasmImport } from "./plugins/vite-plugin-wasm-import";

export default defineConfig({
  plugins: [
    wasmImport(), // 自动处理 WASM 模块导入
    wasm(),
    topLevelAwait(),
  ],
  // ... 其他配置
});
```

### 4. `vite-env.d.ts`
TypeScript 声明：
```typescript
declare module 'virtual:wasm-init' {
  export function initWasm(): Promise<void>;
}
```

## 工作流程

1. **开发时**：
   ```bash
   pnpm dev
   ```
   - `cargo-watch` 监视 Rust 文件变化
   - 自动运行 `wasm-pack build`
   - Vite 检测到 `pkg/` 目录变化
   - 浏览器自动刷新

2. **构建时**：
   ```bash
   pnpm build
   ```
   - 先构建 WASM：`pnpm wasm:build`
   - 然后构建前端：`vite build`
   - 输出到 `dist/` 目录

## 文件名约定

⚠️ **重要**：`wasm-pack` 生成的文件使用下划线命名：
- ✅ `leptos_vite_template.js`（正确）
- ❌ `leptos-vite-template.js`（错误）

插件会自动处理这个命名约定。

## 故障排除

1. **构建失败**：确保先运行 `pnpm wasm:build`
2. **热重载不工作**：检查 `pkg/` 目录是否存在且包含正确的文件
3. **TypeScript 错误**：确保 `tsconfig.node.json` 包含 `plugins/**/*`

## 扩展

如果需要手动控制 WASM 初始化：

```typescript
import { initWasm } from 'virtual:wasm-init';

// 手动初始化
await initWasm();
```

这个设置提供了最现代化、最自动化的 Leptos + Vite 开发体验！
