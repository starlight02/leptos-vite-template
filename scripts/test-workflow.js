#!/usr/bin/env node

/**
 * GitHub Actions 工作流测试脚本
 * 验证工作流配置的正确性
 */

import { readFileSync, existsSync } from 'fs';
import { parse } from 'yaml';

const WORKFLOW_FILE = '.github/workflows/deploy.yml';

function loadWorkflow() {
  if (!existsSync(WORKFLOW_FILE)) {
    console.error(`❌ Workflow file not found: ${WORKFLOW_FILE}`);
    return null;
  }
  
  try {
    const content = readFileSync(WORKFLOW_FILE, 'utf-8');
    return parse(content);
  } catch (error) {
    console.error(`❌ Failed to parse workflow file: ${error.message}`);
    return null;
  }
}

function validateWorkflow(workflow) {
  const checks = [];
  
  // 检查基本结构
  checks.push({
    name: 'Workflow name',
    test: () => workflow.name && typeof workflow.name === 'string',
    message: 'Workflow should have a name'
  });
  
  // 检查触发条件
  checks.push({
    name: 'Tag trigger',
    test: () => workflow.on?.push?.tags?.includes('*'),
    message: 'Should trigger on tag push'
  });
  
  checks.push({
    name: 'Manual trigger',
    test: () => workflow.on?.workflow_dispatch !== undefined,
    message: 'Should support manual trigger'
  });
  
  // 检查权限
  checks.push({
    name: 'Permissions',
    test: () => workflow.permissions?.contents === 'read' && 
                workflow.permissions?.pages === 'write',
    message: 'Should have correct permissions'
  });
  
  // 检查作业
  checks.push({
    name: 'Job exists',
    test: () => workflow.jobs && Object.keys(workflow.jobs).length > 0,
    message: 'Should have at least one job'
  });
  
  const job = Object.values(workflow.jobs)[0];
  
  // 检查运行环境
  checks.push({
    name: 'Ubuntu runner',
    test: () => job['runs-on'] === 'ubuntu-latest',
    message: 'Should run on ubuntu-latest'
  });
  
  // 检查步骤
  const steps = job.steps || [];
  const stepNames = steps.map(step => step.name || '');
  
  const requiredSteps = [
    'Checkout code',
    'Setup Node.js',
    'Setup pnpm',
    'Install Rust',
    'Install wasm-pack',
    'Install dependencies',
    'Build project',
    'Deploy to GitHub Pages'
  ];
  
  requiredSteps.forEach(stepName => {
    checks.push({
      name: `Step: ${stepName}`,
      test: () => stepNames.some(name => name.includes(stepName.split(':')[1]?.trim() || stepName)),
      message: `Should include step: ${stepName}`
    });
  });
  
  return checks;
}

function runChecks(checks) {
  console.log('🔍 Validating GitHub Actions workflow...\n');
  
  let passed = 0;
  let failed = 0;
  
  checks.forEach(check => {
    try {
      if (check.test()) {
        console.log(`✅ ${check.name}`);
        passed++;
      } else {
        console.log(`❌ ${check.name}: ${check.message}`);
        failed++;
      }
    } catch (error) {
      console.log(`❌ ${check.name}: Error - ${error.message}`);
      failed++;
    }
  });
  
  console.log('\n' + '='.repeat(50));
  console.log(`📊 Results: ${passed} passed, ${failed} failed`);
  
  return failed === 0;
}

function checkEnvironmentVariables(workflow) {
  console.log('\n🌍 Checking environment variables...');
  
  const job = Object.values(workflow.jobs)[0];
  const buildStep = job.steps?.find(step => 
    step.name?.includes('Build project') || step.run?.includes('pnpm run build')
  );
  
  if (buildStep?.env) {
    const env = buildStep.env;
    
    if (env.NODE_ENV === 'production') {
      console.log('✅ NODE_ENV set to production');
    } else {
      console.log('❌ NODE_ENV should be set to production');
    }
    
    if (env.GITHUB_REPOSITORY) {
      console.log('✅ GITHUB_REPOSITORY environment variable configured');
    } else {
      console.log('❌ GITHUB_REPOSITORY should be configured');
    }
  } else {
    console.log('⚠️  No environment variables found in build step');
  }
}

function main() {
  const workflow = loadWorkflow();
  if (!workflow) {
    process.exit(1);
  }
  
  console.log(`✅ Workflow file loaded: ${WORKFLOW_FILE}`);
  
  const checks = validateWorkflow(workflow);
  const success = runChecks(checks);
  
  checkEnvironmentVariables(workflow);
  
  if (success) {
    console.log('\n🎉 Workflow validation passed!');
    process.exit(0);
  } else {
    console.log('\n❌ Workflow validation failed. Please fix the issues above.');
    process.exit(1);
  }
}

// 动态导入 yaml 解析器
async function importYaml() {
  try {
    const yaml = await import('yaml');
    global.parse = yaml.parse;
    main();
  } catch (error) {
    console.error('❌ yaml package not found. Installing...');
    console.log('Please run: pnpm add -D yaml');
    process.exit(1);
  }
}

importYaml();