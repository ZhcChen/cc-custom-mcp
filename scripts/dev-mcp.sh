#!/bin/bash

# 开发模式 MCP 服务器启动脚本

echo "🔧 Starting MCP Server in DEVELOPMENT mode..."

# 加载开发环境变量
if [ -f ".env.development" ]; then
    export $(cat .env.development | grep -v '^#' | xargs)
    echo "✅ Loaded development environment variables"
else
    echo "⚠️  .env.development not found, using defaults"
    export MCP_DEV_MODE=true
fi

# 确保开发数据目录存在
mkdir -p ./dev-data/feedback

# 启动开发版本
echo "🚀 Starting development MCP server..."
if [ -f "./src-tauri/target/debug/cc-custom-mcp" ]; then
    ./src-tauri/target/debug/cc-custom-mcp --mcp-mode
else
    echo "❌ Debug binary not found. Please run 'cargo build' first."
    echo "💡 Run: cd src-tauri && cargo build"
    exit 1
fi
