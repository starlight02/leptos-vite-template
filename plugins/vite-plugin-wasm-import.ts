import { Plugin, ResolvedConfig } from 'vite';
import { resolve } from 'path';

interface WasmImportOptions {
  packageName?: string;
  pkgDir?: string;
}

export function wasmImport(options: WasmImportOptions = {}): Plugin {
  const {
    packageName = 'leptos_vite_template', // 使用下划线格式
    pkgDir = 'pkg',
  } = options;

  const virtualModuleId = 'virtual:wasm-init';
  const resolvedVirtualModuleId = '\0' + virtualModuleId;
  let config: ResolvedConfig;

  return {
    name: 'wasm-import',
    configResolved(resolvedConfig) {
      config = resolvedConfig;
    },
    resolveId(id, importer) {
      if (id === virtualModuleId) {
        return resolvedVirtualModuleId;
      }
      // 在虚拟模块中解析 WASM 文件
      if (importer === resolvedVirtualModuleId && id.startsWith(`/${pkgDir}/`)) {
        return resolve(config.root, id.slice(1));
      }
    },
    load(id) {
      if (id === resolvedVirtualModuleId) {
        return `
import init from '/${pkgDir}/${packageName}.js';

let wasmInitialized = false;

export async function initWasm() {
  if (wasmInitialized) {
    return;
  }
  
  try {
    await init();
    wasmInitialized = true;
    console.log('✅ ${packageName} WASM loaded successfully');
  } catch (error) {
    console.error('❌ Failed to load WASM:', error);
    throw error;
  }
}

// 自动初始化
initWasm();

// 热重载支持
if (import.meta.hot) {
  import.meta.hot.accept('/${pkgDir}/${packageName}.js', () => {
    console.log('🔄 WASM module updated, reloading...');
    window.location.reload();
  });
}
`;
      }
    },
    handleHotUpdate(ctx) {
      // 监听 pkg 目录变化
      if (ctx.file.includes(pkgDir)) {
        console.log('🔄 WASM files changed, triggering reload...');
        ctx.server.ws.send({
          type: 'full-reload',
        });
        return [];
      }
    },
  };
}
