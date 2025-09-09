#!/bin/bash

# 自动生成 MCP 配置的脚本

# 检查是否为开发模式
IS_DEV=${MCP_DEV_MODE:-false}

# 获取当前项目路径
PROJECT_DIR=$(pwd)

if [ "$IS_DEV" = "true" ]; then
    echo "🔧 Generating DEVELOPMENT MCP configuration..."

    # 开发模式配置
    cat > mcp-config-dev.json << EOF
{
  "mcpServers": {
    "local-tools-dev": {
      "command": "$PROJECT_DIR/src-tauri/target/debug/cc-custom-mcp",
      "args": ["--mcp-mode"],
      "env": {
        "MCP_LOG_LEVEL": "debug",
        "MCP_DATA_DIR": "$PROJECT_DIR/dev-data"
      },
      "timeout": 600000,
      "autoApprove": ["file_read", "system_info", "feedback"],
      "description": "Local MCP Tools (Development)"
    }
  }
}
EOF

    echo "✅ Development configuration saved to mcp-config-dev.json"
    echo "📁 Data will be stored in: $PROJECT_DIR/dev-data/"
    echo "🔌 Use this config when developing and testing"

else
    echo "🚀 Generating PRODUCTION MCP configuration..."

    # 检查是否有已构建的应用
    if [ -f "$PROJECT_DIR/src-tauri/target/aarch64-apple-darwin/release/cc-custom-mcp" ]; then
        PROD_COMMAND="$PROJECT_DIR/src-tauri/target/aarch64-apple-darwin/release/cc-custom-mcp"
    elif [ -f "$PROJECT_DIR/src-tauri/target/x86_64-apple-darwin/release/cc-custom-mcp" ]; then
        PROD_COMMAND="$PROJECT_DIR/src-tauri/target/x86_64-apple-darwin/release/cc-custom-mcp"
    elif [ -f "$PROJECT_DIR/src-tauri/target/release/cc-custom-mcp" ]; then
        PROD_COMMAND="$PROJECT_DIR/src-tauri/target/release/cc-custom-mcp"
    else
        PROD_COMMAND="/Applications/cc-custom-mcp.app/Contents/MacOS/cc-custom-mcp"
    fi

    # 正式模式配置
    cat > mcp-config-prod.json << EOF
{
  "mcpServers": {
    "local-tools-prod": {
      "command": "$PROD_COMMAND",
      "args": ["--mcp-mode"],
      "env": {
        "MCP_LOG_LEVEL": "info"
      },
      "timeout": 600000,
      "autoApprove": ["file_read", "system_info"],
      "description": "Local MCP Tools (Production)"
    }
  }
}
EOF

    echo "✅ Production configuration saved to mcp-config-prod.json"
    echo "📁 Data will be stored in: ~/.mcp-manager/"
    echo "🔌 Use this config for the installed application"
fi

echo ""
echo "💡 Copy the content of mcp-config.json to your AI client configuration"
