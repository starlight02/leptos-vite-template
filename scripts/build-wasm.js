#!/usr/bin/env node

/**
 * WASM 构建脚本
 * 加载环境变量后执行 wasm-pack 构建
 */

import { execSync } from "child_process";
import {
  autoDetectEnvironment,
  displayEnvInfo,
  loadAndSetEnv,
} from "./env-loader.js";

/**
 * 构建 WASM
 * @param {string} environment - 环境类型
 * @param {boolean} release - 是否为 release 构建
 */
function buildWasm(environment, release = false) {
  console.log("🦀 Starting WASM build process...\n");

  // 加载环境变量
  console.log(`🔧 Loading environment: ${environment}`);
  const env = loadAndSetEnv(environment);
  displayEnvInfo(env);

  console.log("");

  // 构建命令
  const profile = release ? "--release" : "--dev";
  const command = `wasm-pack build --target web --no-typescript ${profile}`;

  console.log(`🔨 Executing: ${command}`);

  try {
    execSync(command, {
      stdio: "inherit",
      env: process.env,
    });

    console.log("\n✅ WASM build completed successfully!");

    // 显示构建产物信息
    import("fs").then((fs) => {
      const wasmFile = "pkg/leptos_vite_template_bg.wasm";
      if (fs.existsSync(wasmFile)) {
        const stats = fs.statSync(wasmFile);
        console.log(`📦 WASM file size: ${Math.round(stats.size / 1024)}KB`);
      }
    });
  } catch (error) {
    console.error("\n❌ WASM build failed!");
    console.error(error.message);
    process.exit(1);
  }
}

/**
 * 主函数
 */
function main() {
  const args = process.argv.slice(2);

  // 检测环境
  const environment = autoDetectEnvironment();

  // 检查是否为 release 构建
  const isRelease = args.includes("--release") || args.includes("-r");

  buildWasm(environment, isRelease);
}

// 执行主函数
main();
