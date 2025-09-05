# Local MCP Server Manager

一个基于 Tauri + Vue 3 的本地 MCP (Model Context Protocol) 服务器管理工具，为 AI 助手提供本地工具支持。

## 功能特性

- 🔧 **可视化管理**: 通过友好的 GUI 界面管理 MCP 服务器
- 🚀 **即插即用**: 自动生成配置，可直接用于 Cursor 等 AI 客户端
- 🛠️ **内置工具**: 提供文件读取、系统信息等常用工具
- 📦 **易于扩展**: 基于特征的工具系统，轻松添加新功能
- ⚡ **高性能**: Rust 后端确保高效的工具执行

## 内置工具

### 1. 文件读取工具
- **功能**: 读取文件内容
- **用途**: 让 AI 助手访问本地文件
- **参数**: `path` (文件路径)

### 2. 系统信息工具
- **功能**: 获取系统信息
- **用途**: 提供操作系统、架构等信息
- **参数**: 无

### 3. 反馈工具
- **功能**: 交互式反馈收集
- **用途**: 让 AI 助手收集用户反馈
- **参数**: `ai_response` (AI回复), `context` (可选上下文)

## 快速开始

### 1. 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 或使用 bun
bun install
```

### 2. 开发模式运行

```bash
npm run tauri dev
```

### 3. 构建应用

```bash
npm run tauri build
```

## 使用方法

### 1. 启动 GUI 管理器

运行应用后，你将看到一个管理界面，可以：

- 启动/停止 MCP 服务器
- 查看可用工具列表
- 复制配置到 AI 客户端

### 2. 配置 AI 客户端

1. 在管理界面中点击"Start Server"启动服务器
2. 复制生成的配置 JSON
3. 将配置添加到你的 AI 客户端（如 Cursor）的 MCP 设置中
4. 重启 AI 客户端以加载工具

### 3. 直接使用 MCP 模式

你也可以直接以 MCP 模式运行：

```bash
./target/debug/cc-custom-mcp --mcp-mode
```

然后通过 stdio 发送 JSON-RPC 请求：

```bash
# 初始化
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | ./cc-custom-mcp --mcp-mode

# 列出工具
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}' | ./cc-custom-mcp --mcp-mode

# 调用工具
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "echo", "arguments": {"message": "Hello!"}}}' | ./cc-custom-mcp --mcp-mode
```

## 配置示例

以下是用于 Cursor 的配置示例：

```json
{
  "mcpServers": {
    "local-tools": {
      "command": "/path/to/your/cc-custom-mcp",
      "args": ["--mcp-mode"],
      "timeout": 600,
      "autoApprove": ["echo", "file_read", "system_info"]
    }
  }
}
```

## 技术架构

- **前端**: Vue 3 + TypeScript
- **后端**: Rust + Tauri
- **MCP 实现**: 基于官方 MCP 协议规范
- **通信**: JSON-RPC over stdio

## 扩展开发

### 添加新工具

1. 在 `src-tauri/src/mcp_server.rs` 中实现 `McpTool` 特征：

```rust
pub struct MyCustomTool;

impl McpTool for MyCustomTool {
    fn name(&self) -> &str {
        "my_tool"
    }

    fn description(&self) -> &str {
        "My custom tool description"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "param": {
                    "type": "string",
                    "description": "Parameter description"
                }
            },
            "required": ["param"]
        })
    }

    fn execute(&self, params: Value) -> Result<Value> {
        // 实现工具逻辑
        Ok(json!({"result": "success"}))
    }
}
```

2. 在 `LocalMcpServer::new()` 中注册工具：

```rust
server.register_tool(Box::new(MyCustomTool));
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
