#!/bin/bash

# MCP Server 开发模式脚本
# 用于在开发过程中快速构建和测试 MCP 服务器

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 MCP Server 开发模式${NC}"
echo ""

# 检查是否在项目根目录
if [ ! -f "src-tauri/Cargo.toml" ]; then
    echo -e "${RED}❌ 请在项目根目录运行此脚本${NC}"
    exit 1
fi

# 进入 Rust 项目目录
cd src-tauri

echo -e "${YELLOW}🔨 构建 MCP 服务器 (debug 模式)...${NC}"

# 使用 debug 模式构建，速度更快
if cargo build; then
    echo -e "${GREEN}✅ MCP 服务器构建成功${NC}"
    
    # 获取可执行文件路径
    EXECUTABLE_PATH="$(pwd)/target/debug/cc-custom-mcp"
    
    echo -e "${BLUE}📍 可执行文件路径: ${EXECUTABLE_PATH}${NC}"
    echo ""
    
    # 显示配置信息
    echo -e "${YELLOW}📋 MCP 配置信息:${NC}"
    echo "{"
    echo "  \"mcpServers\": {"
    echo "    \"cc-mcp\": {"
    echo "      \"command\": \"${EXECUTABLE_PATH}\","
    echo "      \"args\": [\"--mcp-mode\"],"
    echo "      \"autoApprove\": [\"file_read\", \"system_info\", \"feedback\"]"
    echo "    }"
    echo "  }"
    echo "}"
    echo ""
    
    echo -e "${GREEN}✅ MCP 服务器构建完成！${NC}"
    echo -e "${BLUE}💡 如需测试，可运行: ${EXECUTABLE_PATH} --mcp-mode${NC}"
else
    echo -e "${RED}❌ MCP 服务器构建失败${NC}"
    exit 1
fi
