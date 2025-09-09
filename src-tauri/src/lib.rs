mod mcp_server;
mod system_sound;

use mcp_server::LocalMcpServer;
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

// --- 文件系统 IPC 辅助函数 ---

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

// --- 文件监听器 ---

fn start_file_watcher(app: AppHandle, stop_signal: Arc<AtomicBool>) {
    thread::spawn(move || {
        let requests_dir = get_feedback_request_path("").parent().unwrap().to_path_buf();

        loop {
            if stop_signal.load(Ordering::Relaxed) {
                break;
            }

            thread::sleep(Duration::from_millis(500));

            if let Ok(entries) = fs::read_dir(&requests_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(mut request_data) = serde_json::from_str::<Value>(&content) {
                                if request_data.get("processed").and_then(|v| v.as_bool()).unwrap_or(false) {
                                    continue;
                                }

                                let feedback_data = json!({
                                    "sessionId": request_data["sessionId"],
                                    "aiResponse": request_data["aiResponse"],
                                    "context": request_data["context"],
                                    "timestamp": request_data["timestamp"],
                                    "aiSource": request_data.get("aiSource").and_then(|v| v.as_str()).unwrap_or("unknown"),
                                    "aiSourceDisplay": request_data.get("aiSourceDisplay").and_then(|v| v.as_str()).unwrap_or("Unknown AI Tool")
                                });

                                if app.emit("feedback-request", &feedback_data).is_ok() {
                                    thread::spawn(|| {
                                        if let Ok(rt) = tokio::runtime::Runtime::new() {
                                            rt.block_on(async {
                                                let _ = play_notification_sound_async().await;
                                            });
                                        }
                                    });

                                    request_data["processed"] = json!(true);
                                    request_data["processed_at"] = json!(chrono::Utc::now().to_rfc3339());
                                    let _ = fs::write(&path, serde_json::to_string_pretty(&request_data).unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

// --- Tauri 命令 ---

#[tauri::command]
async fn start_mcp_server(app: AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let mut server_guard = state.mcp_server.lock().unwrap();
    let mut running_guard = state.server_running.lock().unwrap();

    if *running_guard {
        return Ok("MCP server is already running".to_string());
    }

    let mut server = LocalMcpServer::new();
    server.set_app_handle(app.clone());
    *server_guard = Some(server);
    *running_guard = true;

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
    if *state.server_running.lock().unwrap() { "running".to_string() } else { "stopped".to_string() }
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
async fn submit_feedback(session_id: String, feedback_content: String) -> Result<(), String> {
    let response_path = get_feedback_response_path(&session_id);
    let response_data = json!({
        "feedback": feedback_content,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    fs::write(&response_path, serde_json::to_string_pretty(&response_data).unwrap())
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn cancel_feedback(session_id: String) -> Result<(), String> {
    let request_path = get_feedback_request_path(&session_id);
    
    if request_path.exists() {
        fs::remove_file(&request_path).map_err(|e| e.to_string())?;
    }
    Ok(())
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
    let _is_dev = env == "development";

    // 修复：无论开发还是生产环境，current_exe() 都能正确获取路径
    // 移除了之前错误的开发环境路径拼接逻辑
    let exe_path = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "!!! FAILED TO GET EXECUTABLE PATH !!!".to_string());

    let config = json!({
        "mcpServers": {
            "cc-mcp": {
                "command": exe_path,
                "args": ["--mcp-mode"],
                "autoApprove": ["file_read", "system_info", "feedback"]
            }
        }
    });
    serde_json::to_string_pretty(&config).unwrap_or_default()
}

#[tauri::command]
async fn bring_window_to_front(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
async fn play_notification_sound() -> Result<(), String> {
    play_notification_sound_async().await.map_err(|e| e.to_string())
}

#[tauri::command]
fn get_cursor_config() -> String {
    get_config_for_source("cursor")
}

#[tauri::command]
fn get_augment_config() -> String {
    get_config_for_source("augment")
}

#[tauri::command]
fn get_claude_desktop_config() -> String {
    get_config_for_source("claude-desktop")
}

#[tauri::command]
fn get_chatgpt_config() -> String {
    get_config_for_source("chatgpt")
}

#[tauri::command]
fn get_custom_config(source_name: String) -> String {
    get_config_for_source(&source_name.to_lowercase().replace(" ", "-"))
}

fn get_config_for_source(source: &str) -> String {
    let exe_path = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "!!! FAILED TO GET EXECUTABLE PATH !!!".to_string());

    let config = json!({
        "mcpServers": {
            "cc-mcp": {
                "command": exe_path,
                "args": ["--mcp-mode"],
                "env": {
                    "MCP_SOURCE": source
                },
                "autoApprove": ["file_read", "system_info", "feedback"]
            }
        }
    });
    serde_json::to_string_pretty(&config).unwrap_or_default()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if std::env::args().any(|arg| arg == "--mcp-mode") {
        // 优先使用环境变量，如果没有设置则根据构建模式自动判断
        let is_dev_mode = match std::env::var("MCP_DEV_MODE") {
            Ok(val) => val == "true",
            Err(_) => {
                // 如果没有设置环境变量，在 debug 构建时默认为开发模式
                #[cfg(debug_assertions)]
                {
                    true
                }
                #[cfg(not(debug_assertions))]
                {
                    false
                }
            }
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut server = LocalMcpServer::new();
            server.set_dev_mode(is_dev_mode);
            if let Err(e) = server.start_stdio_server().await {
                eprintln!("MCP Server failed to start: {}", e);
                std::process::exit(1);
            }
        });
        return;
    }

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
            get_claude_desktop_config,
            get_chatgpt_config,
            get_custom_config,
            submit_feedback,
            cancel_feedback,
            bring_window_to_front,
            play_notification_sound
        ])
        .setup(|app| {
            let state: State<AppState> = app.state();
            start_file_watcher(app.handle().clone(), state.file_watcher_stop.clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
