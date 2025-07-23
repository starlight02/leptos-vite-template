#!/usr/bin/env node

/**
 * 完整构建脚本
 * 按顺序执行 WASM 构建和 Vite 构建
 */

import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { autoDetectEnvironment, loadAndSetEnv, displayEnvInfo } from './env-loader.js';

/**
 * 清理旧的构建产物
 */
function cleanupOldBuilds() {
  console.log('🧹 Cleaning up old build artifacts...');
  
  const dirsToClean = ['dist', 'pkg'];
  
  for (const dir of dirsToClean) {
    if (fs.existsSync(dir)) {
      try {
        fs.rmSync(dir, { recursive: true, force: true });
        console.log(`   ✅ Removed ${dir} directory`);
      } catch (error) {
        console.warn(`   ⚠️ Failed to remove ${dir}: ${error.message}`);
      }
    }
  }
}

/**
 * 执行构建步骤
 * @param {string} environment - 环境类型
 * @param {boolean} release - 是否为 release 构建
 */
async function buildAll(environment, release = false) {
  console.log('🚀 Starting complete build process...');
  console.log(`Environment: ${environment}, Release: ${release}`);
  
  try {
    // 加载环境变量
    console.log('🔧 Loading environment variables...');
    const env = loadAndSetEnv(environment);
    displayEnvInfo(env);
    
    // 清理旧构建
    cleanupOldBuilds();
    
    const profile = release ? '--release' : '--dev';
    
    // 步骤 1: 构建 WASM
    console.log('📦 Step 1: Building WASM...');
    const wasmCommand = `wasm-pack build --target web --no-typescript ${profile}`;
    console.log(`   Command: ${wasmCommand}`);
    
    execSync(wasmCommand, {
      stdio: 'inherit',
      env: process.env
    });
    
    console.log('   ✅ WASM build completed');
    
    // 步骤 2: 构建 Vite
    console.log('⚡ Step 2: Building with Vite...');
    const viteCommand = 'pnpm vite build';
    console.log(`   Command: ${viteCommand}`);
    
    execSync(viteCommand, {
      stdio: 'inherit',
      env: process.env
    });
    
    console.log('   ✅ Vite build completed');
    
    // 显示构建结果
    await displayBuildResults();
    
    console.log('🎉 Complete build process finished successfully!');
    
  } catch (error) {
    console.error('\n❌ Build process failed!');
    console.error(error.message);
    process.exit(1);
  }
}



/**
 * 显示构建结果信息
 */
async function displayBuildResults() {
  console.log('📊 Build Results:');
  
  // WASM 文件信息
  const wasmFile = 'pkg/leptos_vite_template_bg.wasm';
  if (fs.existsSync(wasmFile)) {
    const wasmStats = fs.statSync(wasmFile);
    const wasmSizeKB = Math.round(wasmStats.size / 1024);
    console.log(`   📦 WASM: ${wasmSizeKB}KB`);
  }
  
  // Dist 目录信息
  const distDir = 'dist';
  if (fs.existsSync(distDir)) {
    const files = fs.readdirSync(distDir, { recursive: true });
    let totalSize = 0;
    
    files.forEach(file => {
      const filePath = path.join(distDir, file);
      if (fs.statSync(filePath).isFile()) {
        totalSize += fs.statSync(filePath).size;
      }
    });
    
    console.log(`   📁 Dist: ${files.length} files, ${Math.round(totalSize / 1024)}KB total`);
  }
  
  console.log('');
}

/**
 * 主函数
 */
function main() {
  const args = process.argv.slice(2);
  
  // 检测环境
  const environment = autoDetectEnvironment();
  
  // 检查是否为 release 构建
  const isRelease = args.includes('--release') || args.includes('-r');
  
  buildAll(environment, isRelease);
}

// 执行主函数
main();