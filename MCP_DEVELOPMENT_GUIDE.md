# MCP 开发与正式版分离指南

本项目支持开发环境和正式环境的 MCP 服务分离运行，让你可以在开发这个 MCP 的同时使用它。

## 🏗️ 架构概述

项目包含两种运行模式：

1. **GUI 模式** - Tauri 图形界面应用
2. **MCP 模式** - 通过 `--mcp-mode` 参数启动的 stdio 服务器

每种模式都支持开发和正式环境配置。

## 📁 文件结构

```
├── .env.development          # 开发环境配置
├── .env.production          # 正式环境配置
├── scripts/
│   ├── dev-mcp.sh          # 开发模式启动脚本
│   └── prod-mcp.sh         # 正式模式启动脚本
└── src-tauri/target/
    ├── debug/              # 开发版构建输出
    └── release/            # 正式版构建输出
```

## 🚀 快速开始

### 1. 构建项目

```bash
# 构建开发版本
npm run mcp:build:dev

# 构建正式版本  
npm run mcp:build:prod
```

### 2. 启动 MCP 服务

```bash
# 开发模式
npm run mcp:dev

# 正式模式
npm run mcp:prod
```

## 🔧 环境配置

### 开发环境 (.env.development)
- `MCP_DEV_MODE=true` - 启用开发模式
- `MCP_LOG_LEVEL=debug` - 详细日志
- `MCP_ENABLE_DEBUG_TOOLS=true` - 启用调试工具
- 数据存储在 `./dev-data/` 目录

### 正式环境 (.env.production)
- `MCP_DEV_MODE=false` - 正式模式
- `MCP_LOG_LEVEL=info` - 标准日志
- `MCP_ENABLE_DEBUG_TOOLS=false` - 禁用调试工具
- 数据存储在 `~/.mcp-manager/` 目录

## 🔌 AI 客户端配置

### 开发环境配置
在 AI 客户端中使用开发版配置：

```json
{
  "mcpServers": {
    "local-tools-development": {
      "command": "./src-tauri/target/debug/cc-custom-mcp",
      "args": ["--mcp-mode"],
      "env": {
        "MCP_DEV_MODE": "true",
        "MCP_LOG_LEVEL": "debug"
      },
      "timeout": 300,
      "autoApprove": ["file_read", "system_info", "feedback"]
    }
  }
}
```

### 正式环境配置
在 AI 客户端中使用正式版配置：

```json
{
  "mcpServers": {
    "local-tools-production": {
      "command": "/path/to/cc-custom-mcp",
      "args": ["--mcp-mode"],
      "env": {
        "MCP_DEV_MODE": "false",
        "MCP_LOG_LEVEL": "info"
      },
      "timeout": 600,
      "autoApprove": ["file_read", "system_info"]
    }
  }
}
```

## 🛠️ 开发工作流

1. **开发时**：
   ```bash
   # 启动开发版 MCP 服务
   npm run mcp:dev
   
   # 在另一个终端开发 GUI
   npm run tauri:dev
   ```

2. **测试时**：
   ```bash
   # 构建并测试正式版
   npm run mcp:build:prod
   npm run mcp:prod
   ```

3. **发布时**：
   ```bash
   # 构建所有平台的正式版
   npm run build:all
   ```

## 📊 日志和调试

### 开发模式
- 详细的调试日志输出到 stderr
- 启用额外的调试工具
- 更宽松的超时设置

### 正式模式  
- 简洁的信息日志
- 禁用调试工具
- 更严格的安全设置

## 🔄 同时运行两个版本

你可以同时运行开发版和正式版：

```bash
# 终端 1: 开发版
npm run mcp:dev

# 终端 2: 正式版  
npm run mcp:prod
```

它们使用不同的：
- 数据目录
- 配置名称
- 端口（如果适用）

## 🚨 注意事项

1. **数据隔离**：开发版和正式版使用不同的数据目录
2. **配置分离**：确保 AI 客户端使用正确的配置
3. **版本管理**：开发时修改代码后需要重新构建
4. **环境变量**：确保正确加载对应环境的配置文件
