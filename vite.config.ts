import { defineConfig } from "vite";
import { resolve } from "path";
import { wasmImport } from "./plugins/vite-plugin-wasm-import";
import { visualizer } from "rollup-plugin-visualizer";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import viteCompression from "vite-plugin-compression";
import oxlintPlugin from 'vite-plugin-oxlint';

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
    oxlintPlugin(),
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
  // 构建配置
  build: {
    target: "esnext", // Rolldown 推荐使用单一目标
    sourcemap: false,
    minify: true,
    chunkSizeWarningLimit: 1000,
    rollupOptions: {
      output: {
        // 使用 Rolldown 的 advancedChunks 进行精细化代码分割
        advancedChunks: {
          groups: [
            {
              name: "wasm",
              test: /virtual:wasm-init/,
            },
            {
              name: "mdui-vendor",
              test: /node_modules.*mdui/,
            },
            {
              name: "vendor",
              test: /node_modules/,
            },
            {
              name: "shared",
              test: /\/src\/(utils|shared)\//,
            },
            {
              name: "wasm-bindings",
              test: /pkg\/.*\.js/,
            },
          ],
        },
        // 文件命名策略
        entryFileNames: "assets/[name].[hash].js",
        chunkFileNames: "assets/[name].[hash].js",
        // 资源文件命名 - 特别处理 WASM
        assetFileNames: (assetInfo) => {
          // 使用 names[0] 获取第一个文件名，兼容新的 PreRenderedAsset 类型
          const fileName = assetInfo.names?.[0] || "asset";
          const info = fileName.split(".");
          const ext = info[info.length - 1];
          // WASM 文件使用特殊命名，方便 CDN 缓存
          if (ext === "wasm") {
            return "assets/wasm/[name].[hash][extname]";
          }
          // 图片文件
          if (/png|jpe?g|svg|gif|tiff|bmp|ico/i.test(ext)) {
            return "assets/images/[name].[hash][extname]";
          }
          // 字体文件
          if (/woff2?|eot|ttf|otf/i.test(ext)) {
            return "assets/fonts/[name].[hash][extname]";
          }
          // CSS 文件
          if (ext === "css") {
            return "assets/css/[name].[hash][extname]";
          }
          // 其他文件
          return "assets/[name].[hash][extname]";
        },
      },
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
  },
  // CSS 处理
  css: {
    // 如果不使用 CSS 模块，可以移除此配置
    // modules: {
    //   localsConvention: "camelCase",
    // },
    devSourcemap: true,
  },
  // 依赖优化
  optimizeDeps: {
    exclude: ["./pkg/*.js", "./pkg/*.wasm"],
  },
  // 确保 WASM 文件被正确识别
  assetsInclude: ["**/*.wasm"],
  // 环境变量
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
    __BUILD_TIME__: JSON.stringify(new Date().toISOString()),
  },
  // 解析配置 - 只保留实际使用的别名
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },
  experimental: {
    enableNativePlugin: true,
  },
});
