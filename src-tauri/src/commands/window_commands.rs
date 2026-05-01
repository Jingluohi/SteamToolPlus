// 窗口命令
// 处理窗口管理相关的IPC调用

use crate::models::{SetWindowSizeRequest, WindowOperationResult};
use crate::services::{WindowService, WindowServiceTrait};
use tauri::Manager;

/// 最小化窗口
#[tauri::command]
pub fn minimize_window(window: tauri::Window) -> WindowOperationResult {
    let service = WindowService::new();
    service.minimize(&window)
}

/// 最大化/还原窗口
#[tauri::command]
pub fn maximize_window(window: tauri::Window) -> WindowOperationResult {
    let service = WindowService::new();
    service.maximize(&window)
}

/// 关闭窗口
#[tauri::command]
pub fn close_window(window: tauri::Window) -> WindowOperationResult {
    let service = WindowService::new();
    service.close(&window)
}

/// 切换全屏模式
#[tauri::command]
pub fn toggle_fullscreen(window: tauri::Window) -> WindowOperationResult {
    let service = WindowService::new();
    service.toggle_fullscreen(&window)
}

/// 设置窗口大小
#[tauri::command]
pub fn set_window_size(
    window: tauri::Window,
    request: SetWindowSizeRequest,
) -> WindowOperationResult {
    let service = WindowService::new();
    service.set_size(&window, request)
}

/// 打开帮助窗口
/// 创建一个独立的窗口显示使用说明
#[tauri::command]
pub async fn open_help_window(app: tauri::AppHandle) -> WindowOperationResult {
    // 检查帮助窗口是否已存在
    if let Some(existing_window) = app.get_webview_window("help") {
        // 如果窗口已存在，将其置于前台
        match existing_window.set_focus() {
            Ok(_) => return WindowOperationResult {
                success: true,
                error: None,
            },
            Err(e) => return WindowOperationResult {
                success: false,
                error: Some(format!("无法聚焦帮助窗口: {}", e)),
            },
        }
    }

    // 创建新的帮助窗口
    let window_result = tauri::WebviewWindowBuilder::new(
        &app,
        "help",
        tauri::WebviewUrl::App("/help-window".into())
    )
    .title("使用说明 - Steam Tool Plus")
    .inner_size(1080.0, 700.0)  // 宽度增加20% (900 -> 1080)
    .min_inner_size(600.0, 400.0)
    .center()
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .closable(true)
    .decorations(false)  // 无边框窗口
    .transparent(true)
    .shadow(true)
    .visible(true)
    .focused(true)
    .build();

    match window_result {
        Ok(_) => WindowOperationResult {
            success: true,
            error: None,
        },
        Err(e) => WindowOperationResult {
            success: false,
            error: Some(format!("创建帮助窗口失败: {}", e)),
        },
    }
}

/// 关闭帮助窗口
#[tauri::command]
pub fn close_help_window(app: tauri::AppHandle) -> WindowOperationResult {
    if let Some(window) = app.get_webview_window("help") {
        match window.close() {
            Ok(_) => WindowOperationResult {
                success: true,
                error: None,
            },
            Err(e) => WindowOperationResult {
                success: false,
                error: Some(e.to_string()),
            },
        }
    } else {
        WindowOperationResult {
            success: true,
            error: None,
        }
    }
}
