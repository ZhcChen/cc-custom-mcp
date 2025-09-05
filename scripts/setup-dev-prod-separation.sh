#!/bin/bash

# 设置开发和正式版分离的脚本

echo "🔧 Setting up development and production separation..."

# 创建符号链接或复制文件，使用不同的名称
setup_binaries() {
    echo "📁 Setting up binary separation..."
    
    # 确保目标目录存在
    mkdir -p ./bin
    
    # 开发版本
    if [ -f "./src-tauri/target/debug/cc-custom-mcp" ]; then
        cp "./src-tauri/target/debug/cc-custom-mcp" "./bin/cc-custom-mcp-dev"
        chmod +x "./bin/cc-custom-mcp-dev"
        echo "✅ Created development binary: ./bin/cc-custom-mcp-dev"
    else
        echo "⚠️  Development binary not found. Run 'npm run mcp:build:dev' first."
    fi
    
    # 正式版本
    if [ -f "./src-tauri/target/release/cc-custom-mcp" ]; then
        cp "./src-tauri/target/release/cc-custom-mcp" "./bin/cc-custom-mcp-prod"
        chmod +x "./bin/cc-custom-mcp-prod"
        echo "✅ Created production binary: ./bin/cc-custom-mcp-prod"
    else
        echo "⚠️  Production binary not found. Run 'npm run mcp:build:prod' first."
    fi
}

# 生成配置文件
generate_configs() {
    echo "📝 Generating MCP configuration files..."
    
    mkdir -p ./configs
    
    # 开发环境配置
    cat > ./configs/mcp-config-dev.json << EOF
{
  "mcpServers": {
    "local-tools-dev": {
      "command": "$(pwd)/bin/cc-custom-mcp-dev",
      "args": ["--mcp-mode"],
      "env": {
        "MCP_DEV_MODE": "true",
        "MCP_LOG_LEVEL": "debug",
        "MCP_DATA_DIR": "$(pwd)/dev-data"
      },
      "timeout": 300,
      "autoApprove": ["echo", "file_read", "system_info", "feedback"],
      "description": "Local MCP Tools (Development)"
    }
  }
}
EOF
    
    # 正式环境配置
    cat > ./configs/mcp-config-prod.json << EOF
{
  "mcpServers": {
    "local-tools-prod": {
      "command": "$(pwd)/bin/cc-custom-mcp-prod",
      "args": ["--mcp-mode"],
      "env": {
        "MCP_DEV_MODE": "false",
        "MCP_LOG_LEVEL": "info",
        "MCP_DATA_DIR": "$HOME/.mcp-manager"
      },
      "timeout": 600,
      "autoApprove": ["echo", "file_read", "system_info"],
      "description": "Local MCP Tools (Production)"
    }
  }
}
EOF
    
    echo "✅ Generated ./configs/mcp-config-dev.json"
    echo "✅ Generated ./configs/mcp-config-prod.json"
}

# 创建启动脚本
create_launchers() {
    echo "🚀 Creating launcher scripts..."
    
    # 开发版启动器
    cat > ./bin/start-dev.sh << 'EOF'
#!/bin/bash
export MCP_DEV_MODE=true
export MCP_LOG_LEVEL=debug
export MCP_DATA_DIR="$(dirname "$0")/../dev-data"
mkdir -p "$MCP_DATA_DIR/feedback"
echo "🔧 Starting MCP Server in DEVELOPMENT mode..."
exec "$(dirname "$0")/cc-custom-mcp-dev" --mcp-mode
EOF
    
    # 正式版启动器
    cat > ./bin/start-prod.sh << 'EOF'
#!/bin/bash
export MCP_DEV_MODE=false
export MCP_LOG_LEVEL=info
export MCP_DATA_DIR="$HOME/.mcp-manager"
mkdir -p "$MCP_DATA_DIR/feedback"
echo "🚀 Starting MCP Server in PRODUCTION mode..."
exec "$(dirname "$0")/cc-custom-mcp-prod" --mcp-mode
EOF
    
    chmod +x ./bin/start-dev.sh ./bin/start-prod.sh
    echo "✅ Created ./bin/start-dev.sh"
    echo "✅ Created ./bin/start-prod.sh"
}

# 执行设置
setup_binaries
generate_configs
create_launchers

echo ""
echo "🎉 Setup completed!"
echo ""
echo "📋 Usage:"
echo "  Development: ./bin/start-dev.sh"
echo "  Production:  ./bin/start-prod.sh"
echo ""
echo "📁 Configuration files:"
echo "  Development: ./configs/mcp-config-dev.json"
echo "  Production:  ./configs/mcp-config-prod.json"
echo ""
echo "💡 Add these configurations to your AI client (Claude Desktop, Cursor, etc.)"
