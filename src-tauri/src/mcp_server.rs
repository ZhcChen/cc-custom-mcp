use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use tauri::Emitter;
use lazy_static::lazy_static;
use chrono;

// 全局反馈存储
lazy_static! {
    pub static ref FEEDBACK_STORAGE: Arc<Mutex<HashMap<String, Option<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

// 共享存储路径
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
fn write_feedback_request(session_id: &str, ai_response: &str, context: &str) -> Result<()> {
    let request_data = json!({
        "sessionId": session_id,
        "aiResponse": ai_response,
        "context": context,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "status": "pending"
    });

    let path = get_feedback_request_path(session_id);
    fs::write(&path, serde_json::to_string_pretty(&request_data)?)?;
    eprintln!("📁 Feedback request written to: {:?}", path);
    Ok(())
}

// 读取反馈响应
fn read_feedback_response(session_id: &str) -> Option<String> {
    let path = get_feedback_response_path(session_id);
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(response_data) = serde_json::from_str::<Value>(&content) {
            if let Some(feedback) = response_data["feedback"].as_str() {
                // 删除响应文件
                fs::remove_file(&path).ok();
                return Some(feedback.to_string());
            }
        }
    }
    None
}

// 检查会话是否被取消
fn is_session_cancelled(session_id: &str) -> bool {
    let path = get_feedback_request_path(session_id);
    if !path.exists() {
        // 请求文件不存在，检查是否有响应文件
        let response_path = get_feedback_response_path(session_id);
        if response_path.exists() {
            return false; // 有响应，不是取消
        }
        // 文件不存在且没有响应，可能是被取消了（但这种情况现在应该很少见）
        eprintln!("⚠️  Request file missing for session {}, assuming cancelled", session_id);
        return true;
    }

    // 请求文件存在，检查文件内容中的状态
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(data) = serde_json::from_str::<Value>(&content) {
            if let Some(status) = data["status"].as_str() {
                if status == "cancelled" {
                    eprintln!("📋 Session {} is marked as cancelled in file", session_id);
                    return true;
                }
            }

            // 检查是否已经被处理（发送到 GUI）
            if data.get("processed").and_then(|v| v.as_bool()).unwrap_or(false) {
                eprintln!("📋 Session {} has been processed by file watcher", session_id);
                // 文件已被处理，说明 GUI 应该已经收到通知，不是取消状态
                return false;
            }
        }
    }

    false // 默认不是取消状态
}
use tokio::io::{stdin, stdout};

/// MCP 工具特征定义
pub trait McpTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    fn execute(&self, params: Value) -> Result<Value>;
    fn execute_with_app(&self, params: Value, _app: Option<&tauri::AppHandle>) -> Result<Value> {
        self.execute(params)
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

        // 注册内置工具
        eprintln!("📋 Registering built-in tools...");
        server.register_tool(Box::new(FileReadTool));
        eprintln!("✅ Registered: file_read");
        server.register_tool(Box::new(SystemInfoTool));
        eprintln!("✅ Registered: system_info");

        server.register_tool(Box::new(FeedbackTool));
        eprintln!("✅ Registered: feedback");
        eprintln!("🎯 All tools registered successfully");

        // 打印所有注册的工具
        let tool_list = server.list_tools();
        eprintln!("📋 Available tools: {:?}", tool_list.iter().map(|t| t["name"].as_str().unwrap_or("unknown")).collect::<Vec<_>>());

        server
    }

    pub fn set_app_handle(&mut self, app_handle: tauri::AppHandle) {
        self.app_handle = Some(app_handle);
    }

    pub fn register_tool(&mut self, tool: Box<dyn McpTool>) {
        if let Ok(mut tools) = self.tools.lock() {
            tools.insert(tool.name().to_string(), tool);
        } else {
            eprintln!("❌ Failed to acquire tools lock for registration");
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
            Err(_) => {
                eprintln!("❌ Failed to acquire tools lock for listing");
                Vec::new()
            }
        }
    }

    pub fn set_dev_mode(&mut self, dev_mode: bool) {
        self.dev_mode = dev_mode;
        if dev_mode {
            eprintln!("🔧 MCP Server configured for DEVELOPMENT mode");
            // 在开发模式下可以启用额外的调试工具或日志
            self.server_info.description = format!("{} (Development Mode)", self.server_info.description);
        } else {
            eprintln!("🚀 MCP Server configured for PRODUCTION mode");
        }
    }



    pub fn execute_tool(&self, name: &str, params: Value) -> Result<Value> {
        eprintln!("🛠️  Executing tool: {}", name);
        eprintln!("📋 Tool params: {}", serde_json::to_string_pretty(&params).unwrap_or_default());

        match self.tools.lock() {
            Ok(tools) => {
                if let Some(tool) = tools.get(name) {
                    eprintln!("✅ Tool found, executing...");
                    let has_app_handle = self.app_handle.is_some();
                    eprintln!("📱 App handle available: {}", has_app_handle);

                    let result = tool.execute_with_app(params, self.app_handle.as_ref());
                    match &result {
                        Ok(value) => eprintln!("✅ Tool execution successful: {}", serde_json::to_string_pretty(value).unwrap_or_default()),
                        Err(e) => eprintln!("❌ Tool execution failed: {}", e),
                    }
                    result
                } else {
                    eprintln!("❌ Tool '{}' not found", name);
                    Err(anyhow::anyhow!("Tool '{}' not found", name))
                }
            }
            Err(_) => {
                eprintln!("❌ Failed to acquire tools lock for execution");
                Err(anyhow::anyhow!("Failed to acquire tools lock"))
            }
        }
    }

    pub async fn start_stdio_server(&self) -> Result<()> {
        eprintln!("Starting MCP server with stdio transport...");

        // 这里将来会集成 rmcp SDK
        // 目前先实现一个简单的 JSON-RPC 处理器
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
                Ok(bytes_read) => {
                    eprintln!("MCP Server: Read {} bytes: {}", bytes_read, line.trim());

                    // 跳过空行
                    if line.trim().is_empty() {
                        continue;
                    }

                    match serde_json::from_str::<Value>(&line) {
                        Ok(request) => {
                            let response = self.handle_request(request).await;

                            // 只有非 null 响应才发送
                            if !response.is_null() {
                                match serde_json::to_string(&response) {
                                    Ok(response_str) => {
                                        eprintln!("MCP Server: Sending response: {}", response_str);
                                        if let Err(e) = stdout.write_all(response_str.as_bytes()).await {
                                            eprintln!("MCP Server: Error writing response: {}", e);
                                            break;
                                        }
                                        if let Err(e) = stdout.write_all(b"\n").await {
                                            eprintln!("MCP Server: Error writing newline: {}", e);
                                            break;
                                        }
                                        if let Err(e) = stdout.flush().await {
                                            eprintln!("MCP Server: Error flushing stdout: {}", e);
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("MCP Server: Error serializing response: {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("MCP Server: Error parsing JSON: {} - Input: {}", e, line.trim());
                            // 发送错误响应
                            let error_response = json!({
                                "jsonrpc": "2.0",
                                "id": null,
                                "error": {
                                    "code": -32700,
                                    "message": "Parse error"
                                }
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

        eprintln!("MCP: Received method: {}", method);

        match method {
            "initialize" => {
                eprintln!("MCP: Handling initialize request");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": {},
                            "resources": {},
                            "prompts": {},
                            "logging": {}
                        },
                        "serverInfo": {
                            "name": self.server_info.name,
                            "version": self.server_info.version
                        },
                        "instructions": self.server_info.description
                    }
                })
            }
            "ping" => {
                eprintln!("MCP: Handling ping request");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {}
                })
            }
            "notifications/initialized" => {
                eprintln!("MCP: Handling initialized notification");
                // 对于通知，不需要返回响应
                return json!(null);
            }
            "resources/list" => {
                eprintln!("MCP: Handling resources/list request");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "resources": []
                    }
                })
            }
            "prompts/list" => {
                eprintln!("MCP: Handling prompts/list request");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "prompts": []
                    }
                })
            }
            "logging/setLevel" => {
                eprintln!("MCP: Handling logging/setLevel request");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {}
                })
            }
            "tools/list" => {
                eprintln!("MCP: Handling tools/list request");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": self.list_tools()
                    }
                })
            }
            "tools/call" => {
                eprintln!("MCP: Handling tools/call request");
                let tool_name = request["params"]["name"].as_str().unwrap_or("");
                let arguments = request["params"]["arguments"].clone();

                match self.execute_tool(tool_name, arguments) {
                    Ok(result) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": result.to_string()
                                    }
                                ]
                            }
                        })
                    }
                    Err(e) => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32603,
                                "message": format!("Tool execution failed: {}", e)
                            }
                        })
                    }
                }
            }
            _ => {
                eprintln!("MCP: Unknown method: {}", method);
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method '{}' not found", method)
                    }
                })
            }
        }
    }
}



/// 文件读取工具
pub struct FileReadTool;

impl McpTool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read contents of a file"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read"
                }
            },
            "required": ["path"]
        })
    }

    fn execute(&self, params: Value) -> Result<Value> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e))?;

        Ok(json!({
            "path": path,
            "content": content,
            "size": content.len()
        }))
    }
}

/// 系统信息工具
pub struct SystemInfoTool;

impl McpTool for SystemInfoTool {
    fn name(&self) -> &str {
        "system_info"
    }

    fn description(&self) -> &str {
        "Get system information"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    fn execute(&self, _params: Value) -> Result<Value> {
        Ok(json!({
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "family": std::env::consts::FAMILY,
            "hostname": hostname::get()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

/// 反馈工具 - 用于交互式反馈
pub struct FeedbackTool;



impl McpTool for FeedbackTool {
    fn name(&self) -> &str {
        "feedback"
    }

    fn description(&self) -> &str {
        "Interactive feedback tool - displays AI response and waits for user feedback. Will block until user responds or cancels."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "ai_response": {
                    "type": "string",
                    "description": "The AI's response to display to the user"
                },
                "context": {
                    "type": "string",
                    "description": "Optional context or title for the feedback session"
                }
            },
            "required": ["ai_response"]
        })
    }

    fn execute(&self, params: Value) -> Result<Value> {
        self.execute_with_app(params, None)
    }

    fn execute_with_app(&self, params: Value, app: Option<&tauri::AppHandle>) -> Result<Value> {
        eprintln!("🔧 FeedbackTool::execute_with_app called");
        eprintln!("📝 Params: {}", serde_json::to_string_pretty(&params).unwrap_or_default());

        let ai_response = params["ai_response"].as_str().unwrap_or("");
        let context = params["context"].as_str().unwrap_or("Feedback Session");
        let session_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().to_rfc3339();

        eprintln!("🆔 Generated session ID: {}", session_id);
        eprintln!("📄 AI response length: {} chars", ai_response.len());
        eprintln!("📋 Context: {}", context);

        // 在全局存储中注册这个会话，等待用户反馈
        {
            match FEEDBACK_STORAGE.lock() {
                Ok(mut storage) => {
                    storage.insert(session_id.clone(), None);
                    eprintln!("💾 Session registered in storage. Total sessions: {}", storage.len());
                }
                Err(_) => {
                    eprintln!("❌ Failed to acquire feedback storage lock");
                    return Err(anyhow::anyhow!("Failed to register feedback session"));
                }
            }
        }

        // 如果有 app handle，发送事件到前端（GUI 模式）
        if let Some(app_handle) = app {
            let feedback_data = json!({
                "sessionId": session_id,
                "aiResponse": ai_response,
                "context": context,
                "timestamp": timestamp
            });

            eprintln!("📡 GUI Mode: Sending feedback-request event to frontend...");
            eprintln!("📦 Event data: {}", serde_json::to_string_pretty(&feedback_data).unwrap_or_default());

            // 发送事件到前端
            match app_handle.emit("feedback-request", &feedback_data) {
                Ok(_) => eprintln!("✅ Event sent successfully"),
                Err(e) => {
                    eprintln!("❌ Failed to emit feedback-request event: {}", e);
                    return Err(anyhow::anyhow!("Failed to send event to frontend: {}", e));
                }
            }
        } else {
            // 没有 app handle，使用文件系统共享（stdio 模式）
            eprintln!("📁 Stdio Mode: Writing feedback request to shared storage...");
            if let Err(e) = write_feedback_request(&session_id, ai_response, context) {
                eprintln!("❌ Failed to write feedback request: {}", e);
                return Err(anyhow::anyhow!("Failed to write feedback request: {}", e));
            }
            eprintln!("✅ Feedback request written to shared storage");
        }

        // 持续等待用户反馈，直到收到回复或连接断开
        eprintln!("✅ Feedback request initiated successfully");
        eprintln!("📋 Session ID: {}", session_id);
        eprintln!("⏰ Waiting for user feedback... (will wait indefinitely until response or disconnection)");

        let is_gui_mode = app.is_some();
        let mut wait_count = 0;
        const MAX_WAIT_TIME: u32 = 300; // 最大等待5分钟 (300秒)

        // 有限等待循环，直到收到反馈、会话被取消或超时
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500)); // 每 500ms 检查一次
            wait_count += 1;

            // 检查超时
            if wait_count >= MAX_WAIT_TIME * 2 { // wait_count每秒增加2次
                eprintln!("⏰ Feedback timeout reached ({} seconds), cancelling session", MAX_WAIT_TIME);
                // 清理存储中的会话
                {
                    if let Ok(mut storage) = FEEDBACK_STORAGE.lock() {
                        storage.remove(&session_id);
                    }
                }
                return Ok(json!({
                    "type": "feedback_timeout",
                    "session_id": session_id,
                    "message": format!("Feedback session timed out after {} seconds", MAX_WAIT_TIME)
                }));
            }

            if wait_count % 20 == 0 { // 每 10 秒打印一次状态
                eprintln!("⏰ Still waiting for feedback... ({} seconds elapsed, {} seconds remaining)",
                    wait_count / 2, MAX_WAIT_TIME - (wait_count / 2));
            }

            if is_gui_mode {
                // GUI 模式：检查内存存储
                match FEEDBACK_STORAGE.lock() {
                    Ok(storage) => {
                        match storage.get(&session_id) {
                            Some(Some(content)) => {
                                eprintln!("✅ User feedback received: {}", content);
                                return Ok(json!({
                                    "type": "feedback_response",
                                    "session_id": session_id,
                                    "user_feedback": content,
                                    "message": format!("User feedback: {}", content)
                                }));
                            }
                            Some(None) => {
                                // 会话仍在等待，继续循环
                                continue;
                            }
                            None => {
                                // 会话被取消
                                eprintln!("❌ Session was cancelled by user");
                                return Ok(json!({
                                    "type": "feedback_cancelled",
                                    "session_id": session_id,
                                    "message": "Feedback session was cancelled by user."
                                }));
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("❌ Failed to acquire feedback storage lock during check");
                        return Err(anyhow::anyhow!("Failed to check feedback status"));
                    }
                }
            } else {
                // stdio 模式：检查文件系统
                if let Some(content) = read_feedback_response(&session_id) {
                    eprintln!("✅ User feedback received: {}", content);
                    return Ok(json!({
                        "type": "feedback_response",
                        "session_id": session_id,
                        "user_feedback": content,
                        "message": format!("User feedback: {}", content)
                    }));
                }

                if is_session_cancelled(&session_id) {
                    eprintln!("❌ Session was cancelled by user");
                    return Ok(json!({
                        "type": "feedback_cancelled",
                        "session_id": session_id,
                        "message": "Feedback session was cancelled by user."
                    }));
                }
            }
        }
    }
}


