#!/usr/bin/env node

/**
 * 环境变量加载工具模块
 * 提供简单的 API 来加载和设置环境变量
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

/**
 * 解析单个 .env 文件
 * @param {string} filePath - .env 文件路径
 * @returns {Object} 解析后的环境变量对象
 */
export function parseEnvFile(filePath) {
  if (!fs.existsSync(filePath)) {
    return {};
  }

  const content = fs.readFileSync(filePath, 'utf8');
  const env = {};

  content.split('\n').forEach(line => {
    line = line.trim();
    if (!line || line.startsWith('#')) {
      return;
    }

    const match = line.match(/^([^=]+)=(.*)$/);
    if (match) {
      const key = match[1].trim();
      let value = match[2].trim();
      
      // 移除引号
      if ((value.startsWith('"') && value.endsWith('"')) ||
          (value.startsWith("'") && value.endsWith("'"))) {
        value = value.slice(1, -1);
      }
      
      env[key] = value;
    }
  });

  return env;
}

/**
 * 加载指定环境的环境变量
 * @param {string} environment - 环境名称 ('development', 'production', 'github-pages')
 * @returns {Object} 合并后的环境变量对象
 */
export function loadEnvForEnvironment(environment) {
  const envFiles = [];
  
  // 根据环境类型确定要加载的文件
  switch (environment) {
    case 'development':
      envFiles.push('.env.development');
      break;
    case 'production':
      envFiles.push('.env.production');
      break;
    case 'github-pages':
      envFiles.push('.env.github');
      break;
    default:
      console.warn(`Unknown environment: ${environment}, using development`);
      envFiles.push('.env.development');
  }
  
  // 添加默认 .env 文件作为后备
  envFiles.push('.env');
  
  let mergedEnv = {};
  
  // 按优先级加载（后加载的覆盖先加载的）
  envFiles.reverse().forEach(fileName => {
    const filePath = path.join(rootDir, fileName);
    const env = parseEnvFile(filePath);
    mergedEnv = { ...mergedEnv, ...env };
  });
  
  return mergedEnv;
}

/**
 * 设置环境变量到 process.env
 * @param {Object} env - 环境变量对象
 * @param {boolean} override - 是否覆盖已存在的环境变量
 */
export function setEnvironmentVariables(env, override = true) {
  Object.entries(env).forEach(([key, value]) => {
    if (override || !process.env[key]) {
      process.env[key] = value;
    }
  });
}

/**
 * 一键加载并设置环境变量
 * @param {string} environment - 环境名称
 * @param {boolean} override - 是否覆盖已存在的环境变量
 * @returns {Object} 加载的环境变量对象
 */
export function loadAndSetEnv(environment, override = true) {
  const env = loadEnvForEnvironment(environment);
  setEnvironmentVariables(env, override);
  return env;
}

/**
 * 从命令行参数或环境变量自动检测环境类型
 * @returns {string} 检测到的环境类型
 */
export function autoDetectEnvironment() {
  const args = process.argv.slice(2);
  
  // 从命令行参数检测
  for (const arg of args) {
    if (arg === '--env=development' || arg === '--dev') {
      return 'development';
    }
    if (arg === '--env=production' || arg === '--prod') {
      return 'production';
    }
    if (arg === '--env=github-pages' || arg === '--github') {
      return 'github-pages';
    }
  }

  // 从环境变量检测
  if (process.env.NODE_ENV === 'production') {
    return 'production';
  }
  if (process.env.GITHUB_ACTIONS === 'true') {
    return 'github-pages';
  }

  // 默认开发环境
  return 'development';
}

/**
 * 显示当前加载的环境变量信息
 * @param {Object} env - 环境变量对象
 */
export function displayEnvInfo(env) {
  console.log('📋 Environment Variables:');
  const keyVars = ['VITE_BASE_URL', 'VITE_ENV', 'VITE_APP_TITLE'];
  
  keyVars.forEach(key => {
    const value = env[key] || process.env[key] || 'undefined';
    console.log(`   ${key}: ${value}`);
  });
  
  console.log(`   Total variables: ${Object.keys(env).length}`);
}