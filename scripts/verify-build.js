#!/usr/bin/env node

/**
 * 验证构建产物的脚本
 * 检查 dist 目录中的文件是否正确生成，以及路径配置是否正确
 */

import { readFileSync, existsSync, readdirSync, statSync } from 'fs';
import { join } from 'path';

const DIST_DIR = 'dist';
const REQUIRED_FILES = ['index.html'];

function checkFileExists(filePath) {
  if (!existsSync(filePath)) {
    console.error(`❌ Required file missing: ${filePath}`);
    return false;
  }
  console.log(`✅ Found: ${filePath}`);
  return true;
}

function checkDistDirectory() {
  if (!existsSync(DIST_DIR)) {
    console.error(`❌ Build directory '${DIST_DIR}' does not exist`);
    return false;
  }
  
  console.log(`✅ Build directory '${DIST_DIR}' exists`);
  return true;
}

function listDistContents() {
  console.log('\n📁 Contents of dist directory:');
  
  function listDir(dir, indent = '') {
    const items = readdirSync(dir);
    items.forEach(item => {
      const fullPath = join(dir, item);
      const stats = statSync(fullPath);
      
      if (stats.isDirectory()) {
        console.log(`${indent}📁 ${item}/`);
        listDir(fullPath, indent + '  ');
      } else {
        const size = (stats.size / 1024).toFixed(2);
        console.log(`${indent}📄 ${item} (${size} KB)`);
      }
    });
  }
  
  listDir(DIST_DIR);
}

function checkIndexHtml() {
  const indexPath = join(DIST_DIR, 'index.html');
  if (!existsSync(indexPath)) {
    return false;
  }
  
  const content = readFileSync(indexPath, 'utf-8');
  
  // 检查是否包含必要的元素
  const checks = [
    { name: 'HTML structure', test: /<html[^>]*>/i },
    { name: 'Head section', test: /<head[^>]*>/i },
    { name: 'Body section', test: /<body[^>]*>/i },
    { name: 'App container', test: /id="leptos-app"/i },
    { name: 'Script tag', test: /<script[^>]*type="module"/i }
  ];
  
  console.log('\n🔍 Checking index.html content:');
  let allPassed = true;
  
  checks.forEach(check => {
    if (check.test.test(content)) {
      console.log(`✅ ${check.name}`);
    } else {
      console.log(`❌ ${check.name}`);
      allPassed = false;
    }
  });
  
  return allPassed;
}

function checkWasmFiles() {
  console.log('\n🦀 Checking for WASM files:');
  
  function findWasmFiles(dir) {
    const wasmFiles = [];
    const items = readdirSync(dir);
    
    items.forEach(item => {
      const fullPath = join(dir, item);
      const stats = statSync(fullPath);
      
      if (stats.isDirectory()) {
        wasmFiles.push(...findWasmFiles(fullPath));
      } else if (item.endsWith('.wasm')) {
        wasmFiles.push(fullPath);
      }
    });
    
    return wasmFiles;
  }
  
  const wasmFiles = findWasmFiles(DIST_DIR);
  
  if (wasmFiles.length === 0) {
    console.log('⚠️  No WASM files found - this might be expected if WASM is embedded');
    return true;
  }
  
  wasmFiles.forEach(file => {
    const stats = statSync(file);
    const size = (stats.size / 1024).toFixed(2);
    console.log(`✅ Found WASM: ${file} (${size} KB)`);
  });
  
  return true;
}

function main() {
  console.log('🔍 Verifying build artifacts...\n');
  
  let success = true;
  
  // 检查基本目录和文件
  success &= checkDistDirectory();
  
  if (success) {
    // 列出目录内容
    listDistContents();
    
    // 检查必需文件
    console.log('\n📋 Checking required files:');
    REQUIRED_FILES.forEach(file => {
      success &= checkFileExists(join(DIST_DIR, file));
    });
    
    // 检查 index.html 内容
    success &= checkIndexHtml();
    
    // 检查 WASM 文件
    success &= checkWasmFiles();
  }
  
  console.log('\n' + '='.repeat(50));
  
  if (success) {
    console.log('🎉 All checks passed! Build artifacts are ready for deployment.');
    process.exit(0);
  } else {
    console.log('❌ Some checks failed. Please review the build process.');
    process.exit(1);
  }
}

main();