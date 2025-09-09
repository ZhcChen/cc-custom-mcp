use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use tauri::Emitter;

use chrono;
use std::time::Duration;
use async_trait::async_trait;

// AI来源信息结构
#[derive(Debug, Clone, PartialEq)]
pub enum AiSource {
    Cursor,
    Augment,
    ClaudeDesktop,
    ChatGpt,
    Custom(String),
    Unknown,
}

impl AiSource {
    pub fn from_string(source: &str) -> Self {
        match source.to_lowercase().as_str() {
            "cursor" => AiSource::Cursor,
            "augment" => AiSource::Augment,
            "claude-desktop" | "claude_desktop" => AiSource::ClaudeDesktop,
            "chatgpt" | "chat-gpt" | "chat_gpt" => AiSource::ChatGpt,
            "unknown" | "" => AiSource::Unknown,
            custom => AiSource::Custom(custom.to_string()),
        }
    }

    pub fn to_display_name(&self) -> &str {
        match self {
            AiSource::Cursor => "Cursor AI",
            AiSource::Augment => "Augment AI",
            AiSource::ClaudeDesktop => "Claude Desktop",
            AiSource::ChatGpt => "ChatGPT",
            AiSource::Custom(name) => name,
            AiSource::Unknown => "Unknown AI Tool",
        }
    }
}

// 共享存储路径 (用于 stdio 模式)
fn get_shared_storage_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("mcp_manager");
    path
}

fn get_feedback_request_path(session_id: &str) -> PathBuf {
    let mut path = get_shared_storage_dir();
    path.push("feedback_requests");
    fs::create_dir_all(&path).ok();
    path.push(format!("{}.json", session_id));
    path
}

fn get_feedback_response_path(session_id: &str) -> PathBuf {
    let mut path = get_shared_storage_dir();
    path.push("feedback_responses");
    fs::create_dir_all(&path).ok();
    path.push(format!("{}.json", session_id));
    path
}

// 写入反馈请求到文件系统
fn write_feedback_request(session_id: &str, ai_response: &str, context: &str, raw_mcp_source: &str, ai_source: &AiSource) -> Result<()> {
    let request_data = json!({
        "sessionId": session_id,
        "aiResponse": ai_response,
        "context": context,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "status": "pending",
        "aiSource": raw_mcp_source, // 使用原始的 MCP_SOURCE 值
        "aiSourceDisplay": ai_source.to_display_name()
    });

    let path = get_feedback_request_path(session_id);
    fs::write(&path, serde_json::to_string_pretty(&request_data)?)?;
    Ok(())
}

// 读取反馈响应
fn read_feedback_response(session_id: &str) -> Option<String> {
    let path = get_feedback_response_path(session_id);
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(response_data) = serde_json::from_str::<Value>(&content) {
            if let Some(feedback) = response_data["feedback"].as_str() {
                fs::remove_file(&path).ok(); // Clean up response file
                return Some(feedback.to_string());
            }
        }
    }
    None
}

// 检查会话是否被取消
fn is_session_cancelled(session_id: &str) -> bool {
    let path = get_feedback_request_path(session_id);
    !path.exists()
}


use tokio::io::{stdin, stdout};

// 获取 MCP 来源的智能函数，支持多种配置方式
fn get_mcp_source_smart() -> String {
    // 优先级：
    // 1. MCP_SOURCE 环境变量（最高优先级）
    // 2. 从命令行参数推断
    // 3. 默认值
    
    if let Ok(source) = std::env::var("MCP_SOURCE") {
        if !source.trim().is_empty() {
            return source;
        }
    }
    
    // 尝试从进程信息推断 AI 工具类型
    if let Ok(current_exe) = std::env::current_exe() {
        let exe_name = current_exe.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // 检查是否在特定的 AI 工具环境中运行
        if exe_name.contains("cursor") {
            return "cursor".to_string();
        }
    }
    
    // 默认值：Qoder AI
    "qoder-ai".to_string()
}

/// MCP 工具特征定义 (异步)
#[async_trait]
pub trait McpTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    async fn execute(&self, params: Value) -> Result<Value>;
    async fn execute_with_app(&self, params: Value, _app: Option<&tauri::AppHandle>) -> Result<Value> {
        self.execute(params).await
    }
}

/// 本地 MCP 服务器
pub struct LocalMcpServer {
    tools: Arc<Mutex<HashMap<String, Box<dyn McpTool>>>>,
    server_info: ServerInfo,
    app_handle: Option<tauri::AppHandle>,
    dev_mode: bool,
}

#[derive(Clone)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl LocalMcpServer {
    pub fn new() -> Self {
        let mut server = Self {
            tools: Arc::new(Mutex::new(HashMap::new())),
            server_info: ServerInfo {
                name: "Local MCP Tools".to_string(),
                version: "1.0.0".to_string(),
                description: "Local tools for AI assistants".to_string(),
            },
            app_handle: None,
            dev_mode: false,
        };

        eprintln!("📋 Registering built-in tools...");
        server.register_tool(Box::new(FileReadTool));
        server.register_tool(Box::new(SystemInfoTool));
        server.register_tool(Box::new(FeedbackTool));
        eprintln!("🎯 All tools registered successfully");

        server
    }

    pub fn set_app_handle(&mut self, app_handle: tauri::AppHandle) {
        self.app_handle = Some(app_handle);
    }

    pub fn register_tool(&mut self, tool: Box<dyn McpTool>) {
        if let Ok(mut tools) = self.tools.lock() {
            tools.insert(tool.name().to_string(), tool);
        }
    }

    pub fn list_tools(&self) -> Vec<Value> {
        match self.tools.lock() {
            Ok(tools) => {
                tools
                    .values()
                    .map(|tool| {
                        json!({
                            "name": tool.name(),
                            "description": tool.description(),
                            "inputSchema": tool.input_schema()
                        })
                    })
                    .collect()
            }
            Err(_) => Vec::new()
        }
    }

    pub fn set_dev_mode(&mut self, dev_mode: bool) {
        self.dev_mode = dev_mode;
        if dev_mode {
            eprintln!("🔧 MCP Server configured for DEVELOPMENT mode");
            self.server_info.description = format!("{} (Development Mode)", self.server_info.description);
        } else {
            eprintln!("🚀 MCP Server configured for PRODUCTION mode");
        }
    }

    pub async fn execute_tool(&self, name: &str, params: Value) -> Result<Value> {
        let tool = {
            let tools = self.tools.lock().map_err(|_| anyhow::anyhow!("Failed to acquire tools lock"))?;
            tools.get(name).map(|t| t.name().to_string())
        };

        if let Some(tool_name) = tool {
            let tools = self.tools.lock().unwrap();
            let tool_instance = tools.get(&tool_name).unwrap();
            tool_instance.execute_with_app(params, self.app_handle.as_ref()).await
        } else {
            Err(anyhow::anyhow!("Tool '{}' not found", name))
        }
    }

    pub async fn start_stdio_server(&self) -> Result<()> {
        eprintln!("Starting MCP server with stdio transport...");
        self.run_simple_server().await
    }

    async fn run_simple_server(&self) -> Result<()> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        eprintln!("MCP Server: Starting stdio transport");

        let stdin = stdin();
        let mut stdout = stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    eprintln!("MCP Server: EOF received, shutting down");
                    break;
                }
                Ok(_) => {
                    if line.trim().is_empty() {
                        continue;
                    }

                    match serde_json::from_str::<Value>(&line) {
                        Ok(request) => {
                            let response = self.handle_request(request).await;
                            if !response.is_null() {
                                if let Ok(response_str) = serde_json::to_string(&response) {
                                    let _ = stdout.write_all(response_str.as_bytes()).await;
                                    let _ = stdout.write_all(b"\n").await;
                                    let _ = stdout.flush().await;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("MCP Server: Error parsing JSON: {}", e);
                            let error_response = json!({
                                "jsonrpc": "2.0",
                                "id": null,
                                "error": { "code": -32700, "message": "Parse error" }
                            });
                            if let Ok(error_str) = serde_json::to_string(&error_response) {
                                let _ = stdout.write_all(error_str.as_bytes()).await;
                                let _ = stdout.write_all(b"\n").await;
                                let _ = stdout.flush().await;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("MCP Server: Error reading from stdin: {}", e);
                    break;
                }
            }
        }

        eprintln!("MCP Server: Shutting down");
        Ok(())
    }

    async fn handle_request(&self, request: Value) -> Value {
        let method = request["method"].as_str().unwrap_or("");
        let id = request["id"].clone();

        match method {
            "initialize" => json!({
                "jsonrpc": "2.0", "id": id, "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {}, "resources": {}, "prompts": {}, "logging": {} },
                    "serverInfo": { "name": self.server_info.name, "version": self.server_info.version },
                    "instructions": self.server_info.description
                }
            }),
            "ping" => json!({"jsonrpc": "2.0", "id": id, "result": {}}),
            "notifications/initialized" => json!(null),
            "resources/list" => json!({"jsonrpc": "2.0", "id": id, "result": {"resources": []}}),
            "prompts/list" => json!({"jsonrpc": "2.0", "id": id, "result": {"prompts": []}}),
            "logging/setLevel" => json!({"jsonrpc": "2.0", "id": id, "result": {}}),
            "tools/list" => json!({
                "jsonrpc": "2.0", "id": id, "result": { "tools": self.list_tools() }
            }),
            "tools/call" => {
                let tool_name = request["params"]["name"].as_str().unwrap_or("");
                let arguments = request["params"]["arguments"].clone();

                match self.execute_tool(tool_name, arguments).await {
                    Ok(result) => json!({
                        "jsonrpc": "2.0", "id": id, "result": {
                            "content": [{"type": "text", "text": result.to_string()}]
                        }
                    }),
                    Err(e) => json!({
                        "jsonrpc": "2.0", "id": id, "error": {
                            "code": -32603, "message": format!("Tool execution failed: {}", e)
                        }
                    }),
                }
            }
            _ => json!({
                "jsonrpc": "2.0", "id": id, "error": {
                    "code": -32601, "message": format!("Method '{}' not found", method)
                }
            }),
        }
    }
}

/// 文件读取工具
pub struct FileReadTool;

#[async_trait]
impl McpTool for FileReadTool {
    fn name(&self) -> &str { "file_read" }
    fn description(&self) -> &str { "Read contents of a file" }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": { "path": { "type": "string", "description": "Path to the file to read" }},
            "required": ["path"]
        })
    }
    async fn execute(&self, params: Value) -> Result<Value> {
        let path = params["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;
        let content = tokio::fs::read_to_string(path).await?;
        Ok(json!({ "path": path, "content": content, "size": content.len() }))
    }
}

/// 系统信息工具
pub struct SystemInfoTool;

#[async_trait]
impl McpTool for SystemInfoTool {
    fn name(&self) -> &str { "system_info" }
    fn description(&self) -> &str { "Get system information" }
    fn input_schema(&self) -> Value { json!({ "type": "object", "properties": {} }) }
    async fn execute(&self, _params: Value) -> Result<Value> {
        Ok(json!({
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "hostname": hostname::get().unwrap_or_default().to_string_lossy().to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

/// 反馈工具 - 文件系统IPC版本
pub struct FeedbackTool;

#[async_trait]
impl McpTool for FeedbackTool {
    fn name(&self) -> &str { "feedback" }
    fn description(&self) -> &str { "Interactive feedback tool - displays AI response and waits for user feedback." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "ai_response": { "type": "string", "description": "The AI's response to display" },
                "context": { "type": "string", "description": "Optional context for the session" },
                "source": { "type": "string", "description": "Optional AI tool source identifier (IGNORED - MCP_SOURCE env var is used instead)" }
            },
            "required": ["ai_response"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        self.execute_with_app(params, None).await
    }

    async fn execute_with_app(&self, params: Value, app: Option<&tauri::AppHandle>) -> Result<Value> {
        let ai_response = params["ai_response"].as_str().unwrap_or("").to_string();
        let context = params["context"].as_str().unwrap_or("Feedback Session").to_string();

        let session_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().to_rfc3339();

        // 强制使用环境变量，完全忽略 AI 传递的任何 source 参数
        let raw_mcp_source = get_mcp_source_smart();

        // 用于生成显示名称的 AiSource
        let ai_source = AiSource::from_string(&raw_mcp_source);

        // 写入请求文件
        write_feedback_request(&session_id, &ai_response, &context, &raw_mcp_source, &ai_source)?;

        if let Some(app_handle) = app {
            let feedback_data = json!({
                "sessionId": session_id,
                "aiResponse": ai_response,
                "context": context,
                "timestamp": timestamp,
                "aiSource": raw_mcp_source,
                "aiSourceDisplay": ai_source.to_display_name()
            });

            app_handle.emit("feedback-request", &feedback_data)?;
            
            tokio::spawn(async {
                let _ = crate::system_sound::play_notification_sound_async().await;
            });
        }
        // 移除超时限制，无限等待用户反馈
        loop {
            // 检查响应
            if let Some(feedback_content) = read_feedback_response(&session_id) {
                // 清理请求文件
                fs::remove_file(get_feedback_request_path(&session_id)).ok();
                return Ok(json!({
                    "type": "feedback_response",
                    "user_feedback": feedback_content
                }));
            }

            // 检查取消
            if is_session_cancelled(&session_id) {
                return Ok(json!({
                    "type": "feedback_cancelled",
                    "message": "Feedback session was cancelled by the user."
                }));
            }

            // 不再检查超时，允许无限等待
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
}
