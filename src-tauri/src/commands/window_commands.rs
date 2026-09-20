// 窗口命令
// 处理窗口管理相关的IPC调用

use crate::models::{SetWindowSizeRequest, WindowOperationResult};
use crate::services::{WindowService, WindowServiceTrait};

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
