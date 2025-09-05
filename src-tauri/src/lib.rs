mod mcp_server;
mod system_sound;

use mcp_server::{LocalMcpServer, FEEDBACK_STORAGE};
use system_sound::play_notification_sound_async;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, State, Emitter, Manager};
use serde_json::{json, Value};

struct AppState {
    mcp_server: Arc<Mutex<Option<LocalMcpServer>>>,
    server_running: Arc<Mutex<bool>>,
    file_watcher_stop: Arc<AtomicBool>,
}

// 获取共享存储目录
fn get_shared_storage_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("mcp_manager");
    path
}

// 启动文件监听器
fn start_file_watcher(app: AppHandle, stop_signal: Arc<AtomicBool>) {
    thread::spawn(move || {
        eprintln!("🔍 Starting file watcher for feedback requests...");
        let requests_dir = {
            let mut path = get_shared_storage_dir();
            path.push("feedback_requests");
            fs::create_dir_all(&path).ok();
            eprintln!("📂 File watcher monitoring directory: {:?}", path);
            path
        };

        loop {
            // 检查停止信号
            if stop_signal.load(Ordering::Relaxed) {
                eprintln!("🛑 File watcher received stop signal, shutting down");
                break;
            }

            thread::sleep(Duration::from_millis(500)); // 每 500ms 检查一次

            if let Ok(entries) = fs::read_dir(&requests_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            eprintln!("📁 File watcher: Found JSON file: {:?}", path);
                            if let Ok(content) = fs::read_to_string(&path) {
                                eprintln!("📄 File content: {}", content);
                                if let Ok(mut request_data) = serde_json::from_str::<Value>(&content) {
                                    // 检查是否已经处理过
                                    if request_data.get("processed").and_then(|v| v.as_bool()).unwrap_or(false) {
                                        eprintln!("📋 File already processed, skipping: {:?}", path);
                                        continue; // 跳过已处理的文件
                                    }

                                    // 发送事件到前端
                                    let feedback_data = json!({
                                        "sessionId": request_data["sessionId"],
                                        "aiResponse": request_data["aiResponse"],
                                        "context": request_data["context"],
                                        "timestamp": request_data["timestamp"]
                                    });

                                    eprintln!("📁 File watcher: Found feedback request, sending to frontend");
                                    eprintln!("📦 Event payload: {}", serde_json::to_string_pretty(&feedback_data).unwrap_or_default());

                                    // 重试机制：尝试发送事件，最多重试 3 次
                                    let mut success = false;
                                    for attempt in 1..=3 {
                                        thread::sleep(Duration::from_millis(100 * attempt as u64));

                                        match app.emit("feedback-request", &feedback_data) {
                                            Ok(_) => {
                                                eprintln!("✅ Feedback request event sent successfully (attempt {})", attempt);
                                                success = true;
                                                break;
                                            }
                                            Err(e) => {
                                                eprintln!("❌ Failed to emit feedback-request event (attempt {}): {}", attempt, e);
                                                if attempt == 3 {
                                                    eprintln!("❌ All attempts failed, giving up");
                                                }
                                            }
                                        }
                                    }

                                    if success {
                                        // 标记文件为已处理，而不是删除
                                        request_data["processed"] = json!(true);
                                        request_data["processed_at"] = json!(chrono::Utc::now().to_rfc3339());

                                        match fs::write(&path, serde_json::to_string_pretty(&request_data).unwrap()) {
                                            Ok(_) => {
                                                eprintln!("✅ Request file marked as processed: {:?}", path);
                                            }
                                            Err(e) => {
                                                eprintln!("⚠️  Failed to mark file as processed: {}", e);
                                            }
                                        }
                                    }
                                } else {
                                    eprintln!("❌ Failed to parse JSON content");
                                }
                            } else {
                                eprintln!("❌ Failed to read file content");
                            }
                        }
                    }
                }
            }
        }
    });
}



#[tauri::command]
async fn start_mcp_server(app: AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    eprintln!("🚀 Starting MCP server in GUI mode...");

    let mut server_guard = state.mcp_server.lock().unwrap();
    let mut running_guard = state.server_running.lock().unwrap();

    if *running_guard {
        eprintln!("⚠️  MCP server is already running");
        return Ok("MCP server is already running".to_string());
    }

    let mut server = LocalMcpServer::new();
    eprintln!("🔧 Setting app handle for event communication...");
    server.set_app_handle(app.clone());
    *server_guard = Some(server);
    *running_guard = true;

    eprintln!("✅ MCP server started in GUI mode - ready to accept connections");

    // 注意：文件监听器已经在应用启动时启动了，这里不需要重复启动

    Ok("MCP server started successfully".to_string())
}

#[tauri::command]
fn stop_mcp_server(state: State<'_, AppState>) -> Result<String, String> {
    let mut server_guard = state.mcp_server.lock().unwrap();
    let mut running_guard = state.server_running.lock().unwrap();

    *server_guard = None;
    *running_guard = false;

    Ok("MCP server stopped".to_string())
}

#[tauri::command]
fn get_server_status(state: State<'_, AppState>) -> String {
    let running = *state.server_running.lock().unwrap();
    if running {
        "running".to_string()
    } else {
        "stopped".to_string()
    }
}

#[tauri::command]
fn list_available_tools(state: State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let server_guard = state.mcp_server.lock().unwrap();

    if let Some(server) = server_guard.as_ref() {
        Ok(server.list_tools())
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
fn get_mcp_config() -> String {
    get_mcp_config_for_env("production")
}

#[tauri::command]
fn get_mcp_config_dev() -> String {
    get_mcp_config_for_env("development")
}

fn get_mcp_config_for_env(env: &str) -> String {
    let is_dev = env == "development";
    let exe_path = if is_dev {
        // 开发模式使用 debug 构建
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                parent.join("target/debug/cc-custom-mcp")
                    .to_string_lossy()
                    .to_string()
            } else {
                "./src-tauri/target/debug/cc-custom-mcp".to_string()
            }
        } else {
            "./src-tauri/target/debug/cc-custom-mcp".to_string()
        }
    } else {
        // 正式版使用当前可执行文件
        std::env::current_exe()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    };

    let mut env_vars = std::collections::HashMap::new();
    if is_dev {
        env_vars.insert("MCP_DEV_MODE".to_string(), "true".to_string());
        env_vars.insert("MCP_LOG_LEVEL".to_string(), "debug".to_string());
    } else {
        env_vars.insert("MCP_DEV_MODE".to_string(), "false".to_string());
        env_vars.insert("MCP_LOG_LEVEL".to_string(), "info".to_string());
    }

    let config = serde_json::json!({
        "mcpServers": {
            format!("cc-mcp-{}", env): {
                "command": exe_path,
                "args": ["--mcp-mode"],
                "env": env_vars,
                "timeout": if is_dev { 300 } else { 600 },
                "autoApprove": if is_dev {
                    vec!["echo", "file_read", "system_info", "feedback"]
                } else {
                    vec!["echo", "file_read", "system_info"]
                },
                "description": format!("CC MCP Tools ({})", if is_dev { "Development" } else { "Production" })
            }
        }
    });

    serde_json::to_string_pretty(&config).unwrap_or_default()
}

#[tauri::command]
fn get_cursor_config() -> String {
    let exe_path = std::env::current_exe()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let config = serde_json::json!({
        "mcpServers": {
            "cc-mcp": {
                "command": exe_path,
                "args": ["--mcp-mode"],
                "autoApprove": ["echo", "file_read", "system_info", "feedback"]
            }
        }
    });

    serde_json::to_string_pretty(&config).unwrap_or_default()
}

#[tauri::command]
fn get_augment_config() -> String {
    let exe_path = std::env::current_exe()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let config = serde_json::json!({
        "mcpServers": {
            "cc-mcp": {
                "command": exe_path,
                "args": ["--mcp-mode"]
            }
        }
    });

    serde_json::to_string_pretty(&config).unwrap_or_default()
}

#[tauri::command]
async fn start_external_mcp_server(state: State<'_, AppState>) -> Result<String, String> {
    let server_guard = state.mcp_server.lock().unwrap();
    let running_guard = state.server_running.lock().unwrap();

    if !*running_guard {
        return Err("Please start the MCP server first".to_string());
    }

    if let Some(_server) = server_guard.as_ref() {
        // 在后台启动一个新的进程来处理外部连接
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        tokio::spawn(async move {
            let mut cmd = tokio::process::Command::new(exe_path);
            cmd.arg("--mcp-mode");

            match cmd.spawn() {
                Ok(mut child) => {
                    eprintln!("External MCP server process started");
                    let _ = child.wait().await;
                    eprintln!("External MCP server process ended");
                }
                Err(e) => {
                    eprintln!("Failed to start external MCP server: {}", e);
                }
            }
        });

        Ok("External MCP server started".to_string())
    } else {
        Err("MCP server not initialized".to_string())
    }
}

#[tauri::command]
fn submit_feedback(session_id: String, feedback_content: String) -> Result<String, String> {
    eprintln!("📝 Submitting feedback for session: {}", session_id);

    // 尝试更新内存存储（GUI 模式）
    let mut storage = FEEDBACK_STORAGE.lock().unwrap();
    let found_in_memory = storage.contains_key(&session_id);

    if found_in_memory {
        storage.insert(session_id.clone(), Some(feedback_content.clone()));
        eprintln!("✅ Feedback stored in memory (GUI mode)");
    }

    // 同时写入文件系统（支持 stdio 模式）
    let response_data = json!({
        "sessionId": session_id,
        "feedback": feedback_content,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let mut response_path = get_shared_storage_dir();
    response_path.push("feedback_responses");
    fs::create_dir_all(&response_path).ok();
    response_path.push(format!("{}.json", session_id));

    match fs::write(&response_path, serde_json::to_string_pretty(&response_data).unwrap()) {
        Ok(_) => {
            eprintln!("✅ Feedback written to file system: {:?}", response_path);
            Ok("Feedback submitted successfully".to_string())
        }
        Err(e) => {
            eprintln!("❌ Failed to write feedback to file system: {}", e);
            if found_in_memory {
                Ok("Feedback submitted successfully (memory only)".to_string())
            } else {
                Err(format!("Failed to submit feedback: {}", e))
            }
        }
    }
}

#[tauri::command]
async fn bring_window_to_front(app: AppHandle) -> Result<String, String> {
    eprintln!("🔝 Bringing window to front with gentle activation...");

    // 获取主窗口
    if let Some(window) = app.get_webview_window("main") {
        // 1. 显示窗口（如果被最小化）
        if let Err(e) = window.show() {
            eprintln!("❌ Failed to show window: {}", e);
        } else {
            eprintln!("✅ Window shown successfully");
        }

        // 2. 请求用户注意（不强制置顶，会在任务栏闪烁或其他系统提示）
        if let Err(e) = window.request_user_attention(Some(tauri::UserAttentionType::Informational)) {
            eprintln!("❌ Failed to request user attention: {}", e);
        } else {
            eprintln!("✅ User attention requested successfully");
        }

        // 3. 温和地设置焦点（不阻塞系统切换）
        if let Err(e) = window.set_focus() {
            eprintln!("❌ Failed to focus window: {}", e);
        } else {
            eprintln!("✅ Window focus set successfully");
        }

        eprintln!("✅ Window brought to front with gentle activation - Alt+Tab should work normally");
        Ok("Window brought to front".to_string())
    } else {
        eprintln!("❌ Main window not found");
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
fn cancel_feedback(session_id: String) -> Result<String, String> {
    eprintln!("❌ Cancelling feedback session: {}", session_id);

    // 从内存存储中移除（GUI 模式）
    let mut storage = FEEDBACK_STORAGE.lock().unwrap();
    let found_in_memory = storage.contains_key(&session_id);
    if found_in_memory {
        storage.remove(&session_id);
        eprintln!("✅ Session removed from memory (GUI mode)");
    }

    // 标记文件系统中的请求为取消状态（stdio 模式）
    let mut request_path = get_shared_storage_dir();
    request_path.push("feedback_requests");
    request_path.push(format!("{}.json", session_id));

    if request_path.exists() {
        // 读取现有文件内容并标记为取消
        match fs::read_to_string(&request_path) {
            Ok(content) => {
                if let Ok(mut data) = serde_json::from_str::<Value>(&content) {
                    data["status"] = json!("cancelled");
                    data["cancelled_at"] = json!(chrono::Utc::now().to_rfc3339());

                    match fs::write(&request_path, serde_json::to_string_pretty(&data).unwrap()) {
                        Ok(_) => {
                            eprintln!("✅ Request file marked as cancelled: {:?}", request_path);
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to update request file: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to read request file: {}", e);
            }
        }
    }

    Ok("Feedback session cancelled".to_string())
}

#[tauri::command]
async fn play_notification_sound() -> Result<String, String> {
    eprintln!("🔊 Playing notification sound...");

    match play_notification_sound_async().await {
        Ok(_) => {
            eprintln!("✅ Notification sound played successfully");
            Ok("Notification sound played".to_string())
        }
        Err(e) => {
            eprintln!("❌ Failed to play notification sound: {}", e);
            Err(format!("Failed to play notification sound: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 检查是否以 MCP 模式启动
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--mcp-mode".to_string()) {
        // 检查环境变量确定是开发模式还是正式模式
        let is_dev_mode = std::env::var("MCP_DEV_MODE").unwrap_or_default() == "true";

        if is_dev_mode {
            eprintln!("🔧 Starting MCP server in DEVELOPMENT mode...");
        } else {
            eprintln!("🚀 Starting MCP server in PRODUCTION mode...");
        }

        // MCP 模式：启动 stdio 服务器
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let mut server = LocalMcpServer::new();

                // 根据模式调整服务器配置
                if is_dev_mode {
                    server.set_dev_mode(true);
                }

                if let Err(e) = server.start_stdio_server().await {
                    eprintln!("MCP server failed: {}", e);
                    std::process::exit(1);
                }
            });
        return;
    }

    // 正常 GUI 模式
    let file_watcher_stop = Arc::new(AtomicBool::new(false));
    let app_state = AppState {
        mcp_server: Arc::new(Mutex::new(None)),
        server_running: Arc::new(Mutex::new(false)),
        file_watcher_stop: file_watcher_stop.clone(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            start_mcp_server,
            stop_mcp_server,
            get_server_status,
            list_available_tools,
            get_mcp_config,
            get_mcp_config_dev,
            get_cursor_config,
            get_augment_config,
            start_external_mcp_server,
            submit_feedback,
            cancel_feedback,
            bring_window_to_front,
            play_notification_sound
        ])
        .setup(|app| {
            // 启动文件监听器来处理来自 stdio 模式的反馈请求
            // 这样即使用户没有点击"启动服务器"，也能处理反馈请求
            eprintln!("🔧 Starting file watcher on app startup...");
            let state: State<AppState> = app.state();
            start_file_watcher(app.handle().clone(), state.file_watcher_stop.clone());
            eprintln!("✅ File watcher started successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
