#[cfg(target_os = "linux")]
use std::process::Command;

/// 播放系统提示音的跨平台实现
pub fn play_notification_sound() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        play_macos_sound()
    }
    
    #[cfg(target_os = "windows")]
    {
        play_windows_sound()
    }
    
    #[cfg(target_os = "linux")]
    {
        play_linux_sound()
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("Unsupported operating system".to_string())
    }
}

#[cfg(target_os = "macos")]
fn play_macos_sound() -> Result<(), String> {
    use cocoa::base::nil;
    use cocoa::foundation::{NSString, NSAutoreleasePool};
    use objc::runtime::Object;
    use objc::{msg_send, sel, sel_impl};
    
    unsafe {
        let pool = NSAutoreleasePool::new(nil);
        
        // 获取 NSSound 类
        let sound_class = objc::runtime::Class::get("NSSound").ok_or("Failed to get NSSound class")?;
        
        // 创建系统提示音 "Glass" (macOS 默认通知音)
        let sound_name = NSString::alloc(nil).init_str("Glass");
        let sound: *mut Object = msg_send![sound_class, soundNamed: sound_name];
        
        if sound.is_null() {
            // 如果 Glass 不可用，尝试使用 Ping
            let ping_name = NSString::alloc(nil).init_str("Ping");
            let ping_sound: *mut Object = msg_send![sound_class, soundNamed: ping_name];
            
            if !ping_sound.is_null() {
                let _: () = msg_send![ping_sound, play];
            } else {
                // 最后尝试使用系统默认音效
                let _: () = msg_send![sound_class, beep];
            }
        } else {
            let _: () = msg_send![sound, play];
        }
        
        pool.drain();
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
fn play_windows_sound() -> Result<(), String> {
    use winapi::um::winuser::{MessageBeep, MB_ICONINFORMATION};
    
    unsafe {
        // 播放 Windows 信息提示音
        MessageBeep(MB_ICONINFORMATION);
    }
    
    Ok(())
}

#[cfg(target_os = "linux")]
fn play_linux_sound() -> Result<(), String> {
    // 尝试多种 Linux 音频播放方法
    
    // 方法1: 尝试使用 paplay (PulseAudio)
    if let Ok(_) = Command::new("paplay")
        .arg("/usr/share/sounds/alsa/Front_Left.wav")
        .output()
    {
        return Ok(());
    }
    
    // 方法2: 尝试使用 aplay (ALSA)
    if let Ok(_) = Command::new("aplay")
        .arg("/usr/share/sounds/alsa/Front_Left.wav")
        .output()
    {
        return Ok(());
    }
    
    // 方法3: 尝试使用 speaker-test 生成简单的提示音
    if let Ok(_) = Command::new("speaker-test")
        .args(&["-t", "sine", "-f", "1000", "-l", "1", "-s", "1"])
        .output()
    {
        return Ok(());
    }
    
    // 方法4: 尝试使用 beep 命令
    if let Ok(_) = Command::new("beep").output() {
        return Ok(());
    }
    
    // 方法5: 尝试使用 printf 发送 bell 字符到终端
    if let Ok(_) = Command::new("printf")
        .arg("\x07")
        .output()
    {
        return Ok(());
    }
    
    Err("No available sound playback method found on this Linux system".to_string())
}

/// 异步播放系统提示音，不阻塞主线程
pub async fn play_notification_sound_async() -> Result<(), String> {
    tokio::task::spawn_blocking(|| {
        play_notification_sound()
    }).await.map_err(|e| format!("Failed to spawn sound task: {}", e))?
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_play_sound() {
        // 这个测试只是确保函数不会 panic
        // 实际的音频播放可能需要音频设备
        let result = play_notification_sound();
        println!("Sound play result: {:?}", result);
    }
    
    #[tokio::test]
    async fn test_play_sound_async() {
        let result = play_notification_sound_async().await;
        println!("Async sound play result: {:?}", result);
    }
}
