#!/usr/bin/env node

/**
 * 恢复应用名称的脚本
 * 用于在构建完成后恢复开发环境的应用名称
 */

const fs = require('fs');
const path = require('path');

// 配置文件路径
const configPath = path.join(__dirname, '../src-tauri/tauri.conf.json');
const backupPath = configPath + '.backup';

console.log('🔄 Restoring app name to development settings...');

try {
  // 检查备份文件是否存在
  if (!fs.existsSync(backupPath)) {
    console.log('⚠️  No backup file found, manually setting development name...');
    
    // 手动设置开发环境名称
    const configContent = fs.readFileSync(configPath, 'utf8');
    const config = JSON.parse(configContent);
    
    config.productName = 'cc-custom-mcp';
    if (config.app && config.app.windows && config.app.windows[0]) {
      config.app.windows[0].title = 'MCP 管理器';
    }
    
    const updatedContent = JSON.stringify(config, null, 2);
    fs.writeFileSync(configPath, updatedContent);
    
    console.log('✅ App name restored to: cc-custom-mcp');
    return;
  }
  
  // 从备份恢复
  const backupContent = fs.readFileSync(backupPath, 'utf8');
  fs.writeFileSync(configPath, backupContent);
  
  console.log('✅ App name restored from backup');
  console.log(`📁 Config file restored: ${configPath}`);
  
} catch (error) {
  console.error('❌ Error restoring app name:', error);
  process.exit(1);
}
