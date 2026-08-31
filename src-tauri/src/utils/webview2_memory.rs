// WebView2 内存管理模块（Windows 专用）
//
// 封装 WebView2 内核级内存 API，解决两个问题：
// 1. 隐藏到托盘后 GPU/渲染内存不释放 -> TrySuspend（暂停渲染进程）彻底释放，恢复时 Resume
// 2. 翻页浏览时 GPU discardable 纹理累积 -> MemoryUsageTargetLevel 短暂设 Low 触发回收
//
// 说明：这两个能力 Tauri 2 未直接暴露，这里通过 PlatformWebview 的 controller()
// 拿到 WebView2 COM 接口直接调用。

use tauri::Manager;

// ---------------------------------------------------------------------------
// Windows 实现
// ---------------------------------------------------------------------------

/// 暂停主窗口渲染进程（隐藏到托盘时调用）
/// TrySuspend 暂停渲染进程释放 working set，同时降低内存目标让 GPU 进程丢弃 discardable 纹理
#[cfg(target_os = "windows")]
pub fn suspend_renderer(app: &tauri::AppHandle) {
    use windows::core::Interface;
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2_3, ICoreWebView2_19, COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW,
    };

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.with_webview(|webview| {
            let controller = webview.controller();
            if let Ok(core) = unsafe { controller.CoreWebView2() } {
                // 暂停渲染进程（需要 WebView2 Runtime 88+，失败时静默忽略）
                if let Ok(suspendable) = core.cast::<ICoreWebView2_3>() {
                    let _ = unsafe { suspendable.TrySuspend(None) };
                }
                // 降低内存目标，促使 GPU 进程丢弃 discardable 纹理
                if let Ok(memory) = core.cast::<ICoreWebView2_19>() {
                    let _ = unsafe {
                        memory.SetMemoryUsageTargetLevel(COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW)
                    };
                }
            }
        });
    }
}

/// 恢复主窗口渲染进程（从托盘恢复显示时调用）
#[cfg(target_os = "windows")]
pub fn resume_renderer(app: &tauri::AppHandle) {
    use windows::core::Interface;
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2_3, ICoreWebView2_19, COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL,
    };

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.with_webview(|webview| {
            let controller = webview.controller();
            if let Ok(core) = unsafe { controller.CoreWebView2() } {
                // 恢复渲染进程
                if let Ok(suspendable) = core.cast::<ICoreWebView2_3>() {
                    let _ = unsafe { suspendable.Resume() };
                }
                // 恢复内存目标等级
                if let Ok(memory) = core.cast::<ICoreWebView2_19>() {
                    let _ = unsafe {
                        memory.SetMemoryUsageTargetLevel(
                            COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL,
                        )
                    };
                }
            }
        });
    }
}

/// 设置内存目标等级
/// low=true 降低内存目标（要求 Chromium 丢弃 discardable 缓存），low=false 恢复正常
#[cfg(target_os = "windows")]
fn set_memory_level(app: &tauri::AppHandle, low: bool) {
    use windows::core::Interface;
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2_19, COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW,
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL,
    };

    if let Some(window) = app.get_webview_window("main") {
        // 闭包外先确定目标等级，避免闭包借用局部变量（with_webview 要求 'static）
        let level = if low {
            COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW
        } else {
            COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL
        };

        let _ = window.with_webview(move |webview| {
            let controller = webview.controller();
            if let Ok(core) = unsafe { controller.CoreWebView2() } {
                if let Ok(memory) = core.cast::<ICoreWebView2_19>() {
                    let _ = unsafe { memory.SetMemoryUsageTargetLevel(level) };
                }
            }
        });
    }
}

/// 触发一次 GPU discardable 缓存回收（翻页时调用）
/// 短暂降低内存目标，让 Chromium 丢弃旧页纹理后恢复正常
#[cfg(target_os = "windows")]
pub fn flush_gpu_cache(app: &tauri::AppHandle) {
    set_memory_level(app, true);

    let app_handle = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        set_memory_level(&app_handle, false);
    });
}

// ---------------------------------------------------------------------------
// 非 Windows 平台的空实现，保证跨平台可编译
// ---------------------------------------------------------------------------

#[cfg(not(target_os = "windows"))]
pub fn suspend_renderer(_app: &tauri::AppHandle) {}
#[cfg(not(target_os = "windows"))]
pub fn resume_renderer(_app: &tauri::AppHandle) {}
#[cfg(not(target_os = "windows"))]
pub fn flush_gpu_cache(_app: &tauri::AppHandle) {}