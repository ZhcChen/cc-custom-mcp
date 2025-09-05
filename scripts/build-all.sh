#!/bin/bash

# MCP Manager - 全平台编译脚本
# 支持 macOS (arm64/x64), Windows (arm64/x64), Linux (arm64/x64)

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目信息
PROJECT_NAME="ccMcp"
VERSION=$(grep '"version"' package.json | sed 's/.*"version": "\(.*\)".*/\1/')
BUILD_DIR="dist"

echo -e "${BLUE}🚀 MCP Manager 全平台编译脚本${NC}"
echo -e "${BLUE}版本: ${VERSION}${NC}"
echo -e "${BLUE}项目: ${PROJECT_NAME}${NC}"
echo ""

# 检查依赖
check_dependencies() {
    echo -e "${YELLOW}📋 检查编译依赖...${NC}"
    
    # 检查 Rust
    if ! command -v rustc &> /dev/null; then
        echo -e "${RED}❌ Rust 未安装，请先安装 Rust${NC}"
        exit 1
    fi
    
    # 检查 Tauri CLI
    if ! command -v cargo-tauri &> /dev/null; then
        echo -e "${YELLOW}⚠️  Tauri CLI 未安装，正在安装...${NC}"
        cargo install tauri-cli
    fi
    
    # 检查 Node.js
    if ! command -v node &> /dev/null; then
        echo -e "${RED}❌ Node.js 未安装，请先安装 Node.js${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ 依赖检查完成${NC}"
}

# 安装 Rust 目标平台
install_targets() {
    echo -e "${YELLOW}🎯 安装 Rust 目标平台...${NC}"
    
    # macOS 目标
    rustup target add aarch64-apple-darwin
    rustup target add x86_64-apple-darwin
    
    # Windows 目标
    rustup target add aarch64-pc-windows-msvc
    rustup target add x86_64-pc-windows-msvc
    
    # Linux 目标
    rustup target add aarch64-unknown-linux-gnu
    rustup target add x86_64-unknown-linux-gnu
    
    echo -e "${GREEN}✅ 目标平台安装完成${NC}"
}

# 清理构建目录
clean_build() {
    echo -e "${YELLOW}🧹 清理构建目录...${NC}"
    rm -rf ${BUILD_DIR}
    mkdir -p ${BUILD_DIR}
    echo -e "${GREEN}✅ 构建目录清理完成${NC}"
}

# 安装前端依赖
install_deps() {
    echo -e "${YELLOW}📦 安装前端依赖...${NC}"
    npm install
    echo -e "${GREEN}✅ 前端依赖安装完成${NC}"
}

# 编译单个目标
build_target() {
    local target=$1
    local platform=$2
    local arch=$3
    
    echo -e "${BLUE}🔨 编译 ${platform} (${arch})...${NC}"
    
    # 设置输出目录
    local output_dir="${BUILD_DIR}/${platform}-${arch}"
    mkdir -p ${output_dir}
    
    # 编译
    if cargo tauri build --target ${target}; then
        echo -e "${GREEN}✅ ${platform} (${arch}) 编译成功${NC}"
        
        # 复制编译结果
        case ${platform} in
            "macos")
                if [ -d "src-tauri/target/${target}/release/bundle/macos" ]; then
                    cp -r src-tauri/target/${target}/release/bundle/macos/* ${output_dir}/
                fi
                ;;
            "windows")
                if [ -f "src-tauri/target/${target}/release/${PROJECT_NAME}.exe" ]; then
                    cp src-tauri/target/${target}/release/${PROJECT_NAME}.exe ${output_dir}/
                fi
                if [ -d "src-tauri/target/${target}/release/bundle/msi" ]; then
                    cp -r src-tauri/target/${target}/release/bundle/msi/* ${output_dir}/
                fi
                ;;
            "linux")
                if [ -f "src-tauri/target/${target}/release/${PROJECT_NAME}" ]; then
                    cp src-tauri/target/${target}/release/${PROJECT_NAME} ${output_dir}/
                fi
                if [ -d "src-tauri/target/${target}/release/bundle/deb" ]; then
                    cp -r src-tauri/target/${target}/release/bundle/deb/* ${output_dir}/
                fi
                if [ -d "src-tauri/target/${target}/release/bundle/appimage" ]; then
                    cp -r src-tauri/target/${target}/release/bundle/appimage/* ${output_dir}/
                fi
                ;;
        esac
        
        return 0
    else
        echo -e "${RED}❌ ${platform} (${arch}) 编译失败${NC}"
        return 1
    fi
}

# 主编译函数
build_all() {
    echo -e "${YELLOW}🏗️  开始全平台编译...${NC}"
    
    local success_count=0
    local total_count=6
    
    # macOS 编译
    if build_target "aarch64-apple-darwin" "macos" "arm64"; then
        ((success_count++))
    fi
    
    if build_target "x86_64-apple-darwin" "macos" "x64"; then
        ((success_count++))
    fi
    
    # Windows 编译
    if build_target "aarch64-pc-windows-msvc" "windows" "arm64"; then
        ((success_count++))
    fi
    
    if build_target "x86_64-pc-windows-msvc" "windows" "x64"; then
        ((success_count++))
    fi
    
    # Linux 编译
    if build_target "aarch64-unknown-linux-gnu" "linux" "arm64"; then
        ((success_count++))
    fi
    
    if build_target "x86_64-unknown-linux-gnu" "linux" "x64"; then
        ((success_count++))
    fi
    
    echo ""
    echo -e "${BLUE}📊 编译统计:${NC}"
    echo -e "${GREEN}✅ 成功: ${success_count}/${total_count}${NC}"
    
    if [ ${success_count} -eq ${total_count} ]; then
        echo -e "${GREEN}🎉 所有平台编译成功！${NC}"
    else
        echo -e "${YELLOW}⚠️  部分平台编译失败，请检查错误信息${NC}"
    fi
}

# 创建发布包
create_release() {
    echo -e "${YELLOW}📦 创建发布包...${NC}"
    
    cd ${BUILD_DIR}
    
    # 为每个平台创建压缩包
    for dir in */; do
        if [ -d "$dir" ]; then
            platform_name=${dir%/}
            echo -e "${BLUE}📦 打包 ${platform_name}...${NC}"
            
            if [[ "$platform_name" == *"windows"* ]]; then
                # Windows 使用 zip
                zip -r "${PROJECT_NAME}-${VERSION}-${platform_name}.zip" "$dir"
            else
                # macOS 和 Linux 使用 tar.gz
                tar -czf "${PROJECT_NAME}-${VERSION}-${platform_name}.tar.gz" "$dir"
            fi
        fi
    done
    
    cd ..
    echo -e "${GREEN}✅ 发布包创建完成${NC}"
}

# 显示结果
show_results() {
    echo ""
    echo -e "${BLUE}📁 编译结果:${NC}"
    echo -e "${BLUE}构建目录: ${BUILD_DIR}${NC}"
    echo ""
    
    if [ -d "${BUILD_DIR}" ]; then
        ls -la ${BUILD_DIR}/
    fi
    
    echo ""
    echo -e "${GREEN}🎉 编译完成！${NC}"
    echo -e "${BLUE}版本: ${VERSION}${NC}"
    echo -e "${BLUE}时间: $(date)${NC}"
}

# 主函数
main() {
    echo -e "${BLUE}开始编译流程...${NC}"
    
    check_dependencies
    install_targets
    clean_build
    install_deps
    build_all
    create_release
    show_results
}

# 执行主函数
main "$@"
