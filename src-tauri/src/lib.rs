#![allow(unexpected_cfgs)]

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
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WindowSize {
    width: u32,
    height: u32,
    x: Option<i32>,
    y: Option<i32>,
    maximized: bool,
}

impl Default for WindowSize {
    fn default() -> Self {
        WindowSize {
            width: 1200,
            height: 800,
            x: None,
            y: None,
            maximized: true,
        }
    }
}

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

fn get_window_config_path() -> PathBuf {
    let mut path = get_shared_storage_dir();
    fs::create_dir_all(&path).ok();
    path.push("window_config.json");
    path
}

fn save_window_size(window_size: &WindowSize) -> Result<(), String> {
    let config_path = get_window_config_path();
    let json_content = serde_json::to_string_pretty(window_size)
        .map_err(|e| format!("Failed to serialize window size: {}", e))?;
    
    fs::write(&config_path, json_content)
        .map_err(|e| format!("Failed to save window config: {}", e))?;
    
    eprintln!("✅ Window size saved: {}x{}, maximized: {}", 
              window_size.width, window_size.height, window_size.maximized);
    Ok(())
}

fn load_window_size() -> WindowSize {
    let config_path = get_window_config_path();
    
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            match serde_json::from_str::<WindowSize>(&content) {
                Ok(window_size) => {
                    eprintln!("✅ Window size loaded: {}x{}, maximized: {}", 
                              window_size.width, window_size.height, window_size.maximized);
                    window_size
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse window config: {}, using default", e);
                    WindowSize::default()
                }
            }
        }
        Err(_) => {
            eprintln!("📁 No window config found, using default size");
            WindowSize::default()
        }
    }
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

// 处理单个 feedback 请求文件的通用函数
fn process_feedback_request_file(app: &AppHandle, path: &std::path::Path, is_startup_scan: bool) -> bool {
    match fs::read_to_string(path) {
        Ok(content) => {
            match serde_json::from_str::<Value>(&content) {
                Ok(mut request_data) => {
                    // 检查是否已经被用户处理（提交了反馈）
                    if request_data.get("processed").and_then(|v| v.as_bool()).unwrap_or(false) {
                        return false; // 跳过已处理的文件
                    }
                    
                    let session_id = request_data["sessionId"].as_str().unwrap_or("unknown");
                    
                    // 简化逻辑：使用时间戳来避免重复处理
                    let now = chrono::Utc::now().to_rfc3339();
                    let last_processed_time = request_data.get("last_processed_at").and_then(|v| v.as_str());
                    
                    // 对于启动扫描，总是处理
                    // 对于文件监听，检查是否在最近5分钟内处理过
                    let should_process = if is_startup_scan {
                        eprintln!("🔄 Loading pending feedback request on startup: {}", session_id);
                        true
                    } else {
                        // 检查上次处理时间，如果在5分钟内，跳过
                        if let Some(last_time) = last_processed_time {
                            if let Ok(last_datetime) = chrono::DateTime::parse_from_rfc3339(last_time) {
                                let elapsed = chrono::Utc::now().signed_duration_since(last_datetime.with_timezone(&chrono::Utc));
                                if elapsed.num_minutes() < 5 {
                                    return false; // 跳过最近处理过的文件
                                }
                            }
                        }
                        eprintln!("🔄 Processing new feedback request: {}", session_id);
                        true
                    };

                    if !should_process {
                        return false;
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
                        // 只在非启动扫描时播放通知声音
                        if !is_startup_scan {
                            thread::spawn(|| {
                                if let Ok(rt) = tokio::runtime::Runtime::new() {
                                    rt.block_on(async {
                                        if let Err(e) = play_notification_sound_async().await {
                                            eprintln!("🔔 Failed to play notification sound: {}", e);
                                        }
                                    });
                                }
                            });
                        }

                        // 更新处理时间，但不标记为已完成处理
                        request_data["last_processed_at"] = json!(now);
                        if let Err(e) = fs::write(path, serde_json::to_string_pretty(&request_data).unwrap()) {
                            eprintln!("❌ Failed to update processed time: {}", e);
                        }
                        return true;
                    } else {
                        eprintln!("❌ Failed to emit feedback-request event for {:?}", path);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse JSON from {:?}: {}", path, e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to read file {:?}: {}", path, e);
        }
    }
    false
}

// 执行初始扫描，加载所有 pending 的 feedback 请求
fn perform_initial_scan(app: &AppHandle) {
    let requests_dir = get_feedback_request_path("").parent().unwrap().to_path_buf();
    
    // 确保目录存在
    if !requests_dir.exists() {
        eprintln!("📁 Requests directory does not exist, skipping initial scan");
        return;
    }

    eprintln!("🔍 Performing initial scan for pending feedback requests in: {:?}", requests_dir);
    
    let mut loaded_count = 0;
    match fs::read_dir(&requests_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if process_feedback_request_file(app, &path, true) {
                        loaded_count += 1;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to read requests directory during initial scan: {}", e);
            return;
        }
    }
    
    if loaded_count > 0 {
        eprintln!("✅ Initial scan completed: loaded {} pending feedback requests", loaded_count);
    } else {
        eprintln!("📭 Initial scan completed: no pending feedback requests found");
    }
}

fn start_file_watcher(app: AppHandle, stop_signal: Arc<AtomicBool>) {
    thread::spawn(move || {
        let requests_dir = get_feedback_request_path("").parent().unwrap().to_path_buf();
        eprintln!("🔍 File watcher started, monitoring directory: {:?}", requests_dir);

        // 启动时执行初始扫描
        perform_initial_scan(&app);

        loop {
            if stop_signal.load(Ordering::Relaxed) {
                eprintln!("🛑 File watcher stopping due to stop signal");
                break;
            }

            thread::sleep(Duration::from_millis(1000)); // 增加间隔以减少CPU使用

            match fs::read_dir(&requests_dir) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            process_feedback_request_file(&app, &path, false);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to read requests directory {:?}: {}", requests_dir, e);
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
    
    // 标记原始请求文件为已处理
    let request_path = get_feedback_request_path(&session_id);
    if request_path.exists() {
        match fs::read_to_string(&request_path) {
            Ok(content) => {
                match serde_json::from_str::<Value>(&content) {
                    Ok(mut request_data) => {
                        request_data["processed"] = json!(true);
                        request_data["processed_at"] = json!(chrono::Utc::now().to_rfc3339());
                        request_data["feedback_submitted"] = json!(true);
                        if let Err(e) = fs::write(&request_path, serde_json::to_string_pretty(&request_data).unwrap()) {
                            eprintln!("❌ Failed to mark request as processed: {}", e);
                        } else {
                            eprintln!("✅ Marked feedback request as processed: {}", session_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to parse request file for processing: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to read request file for processing: {}", e);
            }
        }
    }
    
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
    let exe_path = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "!!! FAILED TO GET EXECUTABLE PATH !!!".to_string());

    // 通用配置不包含autoApprove，因为这是Cursor独有的功能
    let config = json!({
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
async fn set_window_compact_mode(app: AppHandle, compact: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        // 获取屏幕尺寸
        let current_monitor = window.current_monitor().map_err(|e| e.to_string())?;
        if let Some(monitor) = current_monitor {
            let monitor_size = monitor.size();
            let scale_factor = monitor.scale_factor();
            
            // 计算实际屏幕尺寸（考虑缩放）
            let screen_width = (monitor_size.width as f64 / scale_factor) as u32;
            let screen_height = (monitor_size.height as f64 / scale_factor) as u32;
            
            if compact {
                // 小窗口模式：宽度固定为 800 像素，高度最大化（使用整个屏幕高度）
                let compact_width = 800u32;
                let compact_height = screen_height; // 高度为屏幕的最大高度
                
                eprintln!("🔧 Setting compact mode: screen={}x{}, target={}x{}", screen_width, screen_height, compact_width, compact_height);
                
                // 先取消最大化，这样才能调整窗口大小
                window.unmaximize().map_err(|e| e.to_string())?;
                eprintln!("✅ Window unmaximized");
                
                // 设置窗口大小
                window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: compact_width,
                    height: compact_height,
                })).map_err(|e| e.to_string())?;
                eprintln!("✅ Window size set to {}x{}", compact_width, compact_height);
                
                // 将窗口移动到屏幕右侧
                let x_position = (screen_width - compact_width) as i32;
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: x_position,
                    y: 0, // 窗口顶部与屏幕顶部对齐
                })).map_err(|e| e.to_string())?;
                eprintln!("✅ Window positioned at ({}, 0)", x_position);
                
            } else {
                // 普通模式：最大化窗口
                window.maximize().map_err(|e| e.to_string())?;
            }
            
            Ok(())
        } else {
            Err("No monitor found".to_string())
        }
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
async fn play_notification_sound() -> Result<(), String> {
    play_notification_sound_async().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_current_window_size(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let size = window.inner_size().map_err(|e| e.to_string())?;
        let position = window.outer_position().map_err(|e| e.to_string())?;
        let is_maximized = window.is_maximized().map_err(|e| e.to_string())?;
        
        let window_size = WindowSize {
            width: size.width,
            height: size.height,
            x: Some(position.x),
            y: Some(position.y),
            maximized: is_maximized,
        };
        
        save_window_size(&window_size)?;
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
async fn load_saved_window_size() -> Result<WindowSize, String> {
    Ok(load_window_size())
}

#[tauri::command]
async fn apply_window_size(app: AppHandle, window_size: WindowSize) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window_size.maximized {
            window.maximize().map_err(|e| e.to_string())?;
        } else {
            // 先取消最大化
            window.unmaximize().map_err(|e| e.to_string())?;
            
            // 设置窗口大小
            window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: window_size.width,
                height: window_size.height,
            })).map_err(|e| e.to_string())?;
            
            // 如果有位置信息，设置窗口位置
            if let (Some(x), Some(y)) = (window_size.x, window_size.y) {
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x,
                    y,
                })).map_err(|e| e.to_string())?;
            }
        }
        
        eprintln!("✅ Applied window size: {}x{}, maximized: {}", 
                  window_size.width, window_size.height, window_size.maximized);
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
async fn scan_pending_feedback(app: AppHandle) -> Result<String, String> {
    let requests_dir = get_feedback_request_path("").parent().unwrap().to_path_buf();
    
    // 确保目录存在
    if !requests_dir.exists() {
        return Ok("No pending feedback requests found (directory does not exist)".to_string());
    }

    eprintln!("🔍 Manual scan for pending feedback requests triggered");
    
    let mut loaded_count = 0;
    match fs::read_dir(&requests_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if process_feedback_request_file(&app, &path, true) {
                        loaded_count += 1;
                    }
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Failed to scan pending feedback requests: {}", e);
            eprintln!("❌ {}", error_msg);
            return Err(error_msg);
        }
    }
    
    let result_msg = if loaded_count > 0 {
        format!("Successfully loaded {} pending feedback requests", loaded_count)
    } else {
        "No pending feedback requests found".to_string()
    };
    
    eprintln!("✅ Manual scan completed: {}", result_msg);
    Ok(result_msg)
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

    // autoApprove 是 Cursor 独有的功能，其他 AI 不支持
    let mut server_config = json!({
        "command": exe_path,
        "args": ["--mcp-mode"],
        "env": {
            "MCP_SOURCE": source
        }
    });

    // 只有 Cursor 才添加 autoApprove 配置
    if source == "cursor" {
        server_config["autoApprove"] = json!(["file_read", "system_info", "feedback"]);
    }

    let config = json!({
        "mcpServers": {
            "cc-mcp": server_config
        }
    });
    serde_json::to_string_pretty(&config).unwrap_or_default()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if std::env::args().any(|arg| arg == "--mcp-mode") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let server = LocalMcpServer::new();
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
            get_cursor_config,
            get_augment_config,
            get_claude_desktop_config,
            get_chatgpt_config,
            get_custom_config,
            submit_feedback,
            cancel_feedback,
            bring_window_to_front,
            play_notification_sound,
            scan_pending_feedback,
            set_window_compact_mode,
            save_current_window_size,
            load_saved_window_size,
            apply_window_size
        ])
        .setup(|app| {
            let state: State<AppState> = app.state();
            start_file_watcher(app.handle().clone(), state.file_watcher_stop.clone());
            
            // 在应用启动时加载保存的窗口尺寸
            let app_handle = app.handle().clone();
            thread::spawn(move || {
                // 等待一点时间，确保窗口完全初始化
                thread::sleep(Duration::from_millis(500));
                
                let window_size = load_window_size();
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window_size.maximized {
                        if let Err(e) = window.maximize() {
                            eprintln!("❌ Failed to maximize window: {}", e);
                        }
                    } else {
                        // 先取消最大化
                        if let Err(e) = window.unmaximize() {
                            eprintln!("❌ Failed to unmaximize window: {}", e);
                        }
                        
                        // 设置窗口大小
                        if let Err(e) = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                            width: window_size.width,
                            height: window_size.height,
                        })) {
                            eprintln!("❌ Failed to set window size: {}", e);
                        }
                        
                        // 如果有位置信息，设置窗口位置
                        if let (Some(x), Some(y)) = (window_size.x, window_size.y) {
                            if let Err(e) = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                x,
                                y,
                            })) {
                                eprintln!("❌ Failed to set window position: {}", e);
                            }
                        }
                    }
                    
                    eprintln!("🚀 Window size restored on startup: {}x{}, maximized: {}", 
                              window_size.width, window_size.height, window_size.maximized);
                } else {
                    eprintln!("❌ Main window not found during startup");
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
