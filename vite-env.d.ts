/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_TITLE: string
  // 可以在这里添加更多环境变量类型
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}

// 虚拟模块声明
declare module 'virtual:wasm-init' {
  export function initWasm(): Promise<void>;
}
