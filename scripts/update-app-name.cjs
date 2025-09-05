#!/usr/bin/env node

/**
 * 动态更新应用名称的脚本
 * 用于在开发和生产环境中使用不同的应用名称
 */

const fs = require('fs');
const path = require('path');

// 配置文件路径
const configPath = path.join(__dirname, '../src-tauri/tauri.conf.json');

// 应用名称配置
const APP_NAMES = {
  development: 'cc-custom-mcp',
  production: 'ccMCP'
};

// 获取当前模式
const mode = process.env.NODE_ENV === 'production' ? 'production' : 'development';
const targetAppName = APP_NAMES[mode];

console.log(`🔧 Updating app name for ${mode} mode...`);
console.log(`📝 Target app name: ${targetAppName}`);

try {
  // 读取配置文件
  const configContent = fs.readFileSync(configPath, 'utf8');
  const config = JSON.parse(configContent);
  
  // 备份原始配置（如果需要的话）
  const backupPath = configPath + '.backup';
  if (!fs.existsSync(backupPath)) {
    fs.writeFileSync(backupPath, configContent);
    console.log(`💾 Created backup at: ${backupPath}`);
  }
  
  // 更新应用名称
  const oldName = config.productName;
  config.productName = targetAppName;
  
  // 同时更新窗口标题（如果需要的话）
  if (config.app && config.app.windows && config.app.windows[0]) {
    const windowTitle = mode === 'production' ? 'ccMCP' : 'MCP 管理器';
    config.app.windows[0].title = windowTitle;
  }
  
  // 写回配置文件
  const updatedContent = JSON.stringify(config, null, 2);
  fs.writeFileSync(configPath, updatedContent);
  
  console.log(`✅ App name updated: ${oldName} → ${targetAppName}`);
  console.log(`📁 Config file updated: ${configPath}`);
  
} catch (error) {
  console.error('❌ Error updating app name:', error);
  process.exit(1);
}
