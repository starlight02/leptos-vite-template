import { defineConfig } from "vite";
import { resolve } from "path";
import { wasmImport } from "./plugins/vite-plugin-wasm-import";
import { visualizer } from "rollup-plugin-visualizer";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import viteCompression from "vite-plugin-compression";

export default defineConfig({
  plugins: [
    wasmImport(), // 自动处理 WASM 模块导入
    wasm(),
    topLevelAwait(),
    // Brotli 压缩配置 - 最高级别，包含 WASM 文件
    viteCompression({
      algorithm: "brotliCompress",
      ext: ".br",
      compressionOptions: {
        level: 11,
        chunkSize: 32 * 1024,
        windowBits: 22,
      },
      threshold: 1024,
      deleteOriginFile: false,
      verbose: true,
      filter: /\.(js|mjs|json|css|html|wasm)$/i,
    }),
    // 可选：同时生成 gzip 版本用于兼容，也包含 WASM
    viteCompression({
      algorithm: "gzip",
      ext: ".gz",
      compressionOptions: {
        level: 9,
      },
      threshold: 1024,
      deleteOriginFile: false,
      filter: /\.(js|mjs|json|css|html|wasm)$/i,
    }),
  ],

  // 开发服务器配置
  server: {
    port: 3000,
    open: true,
    host: "localhost",
    cors: true,
    watch: {
      // 监视 pkg 目录变化，触发热重载
      ignored: ["!**/pkg/**"],
    },
  },

  // 预览服务器配置
  preview: {
    port: 4173,
    host: "localhost",
    cors: true,
  },

  // 构建配置 - 重点优化部分
  build: {
    target: ["esnext", "chrome91", "firefox90", "safari15"],
    outDir: "dist",
    assetsDir: "assets",
    emptyOutDir: true,
    // 启用源码映射（生产环境可选）
    sourcemap: false, // 设为 true 可启用源码映射
    // 压缩配置
    minify: true, // 或者 'terser' 获得更好的压缩
    // 代码分割阈值
    chunkSizeWarningLimit: 1000,
    // Rollup 特定配置
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
      },
      output: {
        // 手动代码分割
        manualChunks: {
          // 将 WASM 相关代码分离到单独的 chunk
          wasm: ["virtual:wasm-init"],
          // MDUI 工具库（本地打包版本）
          mdui: ["./src/plugins/mdui.ts"],
        },
        // 文件命名策略
        entryFileNames: "assets/[name]-[hash].js",
        chunkFileNames: "assets/[name]-[hash].js",
        // 资源文件命名 - 特别处理 WASM
        assetFileNames: (assetInfo) => {
          const info = assetInfo.name?.split(".") || [];
          const ext = info[info.length - 1];
          // WASM 文件使用特殊命名，方便 CDN 缓存
          if (ext === "wasm") {
            return "assets/wasm/[name]-[hash][extname]";
          }
          // 图片文件
          if (/png|jpe?g|svg|gif|tiff|bmp|ico/i.test(ext)) {
            return "assets/images/[name]-[hash][extname]";
          }
          // 字体文件
          if (/woff2?|eot|ttf|otf/i.test(ext)) {
            return "assets/fonts/[name]-[hash][extname]";
          }
          // CSS 文件
          if (ext === "css") {
            return "assets/css/[name]-[hash][extname]";
          }
          // 其他文件
          return "assets/[name]-[hash][extname]";
        },
        // 优化输出格式
        format: "es",
        // 压缩配置
        compact: true,
      },
      // 外部依赖处理（如果需要）
      // external: ['some-external-dependency'],
      // 插件配置 - 包含 Bundle Analyzer
      plugins: [
        visualizer({
          filename: ".vite/stats.html",
          open: true,
          gzipSize: true,
          brotliSize: true,
        }),
      ],
    },
    // 实验性功能
    reportCompressedSize: true,
    // CSS 代码分割
    cssCodeSplit: true,
    // 库模式配置（如果需要）
    // lib: {
    //   entry: resolve(__dirname, 'src/lib.rs'),
    //   name: 'LeptosApp',
    //   fileName: 'leptos-app',
    // },
  },
  // CSS 处理
  css: {
    // CSS 模块化
    modules: {
      localsConvention: "camelCase",
    },
    // PostCSS 配置
    postcss: {
      plugins: [
        // 可以添加 autoprefixer, cssnano 等插件
      ],
    },
    // 开发时的源码映射
    devSourcemap: true,
  },
  // 依赖优化
  optimizeDeps: {
    exclude: ["./pkg/*.js", "./pkg/*.wasm"],
    // esbuild 选项 - 启用自动 Tree Shaking
    esbuildOptions: {
      target: "esnext",
      treeShaking: true,
    },
  },
  // 确保 WASM 文件被正确识别
  assetsInclude: ["**/*.wasm"],
  // 环境变量
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
    __BUILD_TIME__: JSON.stringify(new Date().toISOString()),
  },
  // 解析配置
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
      "@assets": resolve(__dirname, "src/assets"),
      "@components": resolve(__dirname, "src/components"),
      "@pages": resolve(__dirname, "src/pages"),
    },
  },
});
