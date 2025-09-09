## 项目概述

这是一个使用 Tauri 和 Vue 3 构建的本地 MCP (Model Context Protocol) 服务器管理器。它为管理 MCP 服务器提供了一个图形用户界面，该服务器反过来又为 AI 助手提供工具。该应用程序被设计为易于扩展，允许开发人员向 MCP 服务器添加新工具。

**关键技术:**

*   **前端:** Vue 3, TypeScript, Pinia, Vue Router, Vue I18n
*   **后端:** Rust, Tauri
*   **构建工具:** Vite, npm

**架构:**

该应用程序由一个 Vue.js 前端和一个 Rust 后端组成，它们使用 Tauri 捆绑成一个单一的桌面应用程序。前端提供用于管理 MCP 服务器的用户界面，而后端处理服务器的逻辑并向前台公开命令。MCP 服务器本身是用 Rust 实现的，并使用 JSON-RPC 协议通过 stdio 与 AI 客户端通信。

## 构建和运行

### 先决条件

*   Node.js 和 npm (或 bun)
*   Rust 和 Cargo

### 安装

```bash
npm install
```

### 开发

要在开发模式下运行应用程序，请使用以下命令：

```bash
npm run tauri:dev
```

这将为前端启动 Vite 开发服务器，为后端启动 Tauri 开发服务器。

### 构建

要为生产构建应用程序，请使用以下命令：

```bash
npm run tauri:build
```

这将构建前端和后端，然后将它们捆绑成一个可执行文件。

### 其他有用的命令

*   `npm run build:all`: 为所有平台构建应用程序。
*   `npm run mcp:dev`: 在开发模式下运行 MCP 服务器。
*   `npm run mcp:prod`: 在生产模式下运行 MCP 服务器。

## 开发约定

### 代码风格

该项目使用 Vue 3 和 Rust 的标准代码风格。前端代码是用 TypeScript 编写的，并使用 Composition API。后端代码是用 Rust 编写的，并遵循标准的 Rust 约定。

### 测试

项目中没有明确的测试实践文档。但是，该项目结构良好，应该易于测试。

### 贡献指南

`README.md` 文件欢迎以 issue 和 pull request 的形式做出贡献。
