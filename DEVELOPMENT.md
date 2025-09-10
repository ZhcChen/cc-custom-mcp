# 开发指南

## 🚀 快速开始

这个项目包含两个主要组件：

1. **GUI应用** - Tauri + Vue.js 桌面应用
2. **MCP服务器** - 独立的MCP协议服务器

## 📋 开发模式

### 方法1：分别启动 (推荐)

#### 1. 启动GUI应用 (开发模式)
```bash
# 启动前端开发服务器 + Tauri应用
bun run tauri dev
# 或者
npm run tauri dev
```

#### 2. 构建并测试MCP服务器
```bash
# 构建MCP服务器 (debug模式，编译更快)
npm run build:mcp
# 或者
bash scripts/dev-mcp-server.sh
```

这个脚本会：
- 使用debug模式构建MCP服务器（编译更快，便于开发调试）
- 显示配置信息
- 可选择启动服务器进行测试

### 方法2：仅构建MCP服务器

```bash
cd src-tauri
cargo build
./target/debug/cc-custom-mcp --mcp-mode
```

## 🏗️ 生产构建

### 构建所有平台
```bash
npm run build:all
```

### 构建特定平台
```bash
npm run build:macos    # macOS (arm64 + x64)
npm run build:windows  # Windows (arm64 + x64)  
npm run build:linux    # Linux (arm64 + x64)
```

## 🔧 MCP配置

开发过程中，MCP服务器的配置信息会显示在终端中。

### Cursor配置示例
```json
{
  "mcpServers": {
    "cc-mcp": {
      "command": "/path/to/cc-custom-mcp",
      "args": ["--mcp-mode"],
      "autoApprove": ["file_read", "system_info", "feedback"]
    }
  }
}
```

## 📁 项目结构

```
├── src/                    # Vue.js 前端代码
├── src-tauri/              # Rust 后端代码
│   ├── src/
│   │   ├── main.rs         # 应用入口
│   │   ├── lib.rs          # 主要逻辑
│   │   ├── mcp_server.rs   # MCP服务器实现
│   │   └── system_sound.rs # 系统声音
│   └── Cargo.toml          # Rust依赖配置
├── scripts/                # 构建脚本
│   ├── dev-mcp-server.sh   # 开发MCP服务器脚本
│   └── build-all.sh        # 全平台构建脚本
└── package.json            # Node.js配置
```

## 🐛 常见问题

### Q: 为什么`bun run tauri dev`不会构建MCP服务器？
A: Tauri开发模式只启动GUI应用，MCP服务器需要单独构建。使用`npm run build:mcp`来构建MCP服务器。

### Q: 如何测试MCP功能？
A: 
1. 运行`npm run build:mcp`构建MCP服务器
2. 将配置添加到你的AI工具（如Cursor）
3. 在AI工具中测试MCP功能

### Q: 开发时需要同时运行GUI和MCP服务器吗？
A: 通常不需要。GUI应用可以独立测试，MCP服务器在需要时单独构建和测试。
