#!/bin/bash

# 正式版 MCP 服务器启动脚本

echo "🚀 Starting MCP Server in PRODUCTION mode..."

# 加载正式环境变量
if [ -f ".env.production" ]; then
    export $(cat .env.production | grep -v '^#' | xargs)
    echo "✅ Loaded production environment variables"
else
    echo "⚠️  .env.production not found, using defaults"
    export MCP_DEV_MODE=false
fi

# 确保正式版数据目录存在
mkdir -p ~/.mcp-manager/feedback

# 启动正式版本
echo "🚀 Starting production MCP server..."
if [ -f "./src-tauri/target/release/cc-custom-mcp" ]; then
    ./src-tauri/target/release/cc-custom-mcp --mcp-mode
else
    echo "❌ Release binary not found. Please run 'cargo build --release' first."
    echo "💡 Run: cd src-tauri && cargo build --release"
    exit 1
fi
