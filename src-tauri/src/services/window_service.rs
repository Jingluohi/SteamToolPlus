// 窗口服务
// 实现窗口管理的业务逻辑

use crate::models::{SetWindowSizeRequest, WindowOperationResult, WindowState};
use tauri::Window;

/// 窗口服务接口
pub trait WindowServiceTrait: Send + Sync {
    /// 最小化窗口
    fn minimize(&self, window: &Window) -> WindowOperationResult;
    /// 最大化/还原窗口
    fn maximize(&self, window: &Window) -> WindowOperationResult;
    /// 关闭窗口
    fn close(&self, window: &Window) -> WindowOperationResult;
    /// 切换全屏模式
    fn toggle_fullscreen(&self, window: &Window) -> WindowOperationResult;
    /// 设置窗口大小
    fn set_size(&self, window: &Window, request: SetWindowSizeRequest) -> WindowOperationResult;
    /// 获取窗口状态
    #[allow(dead_code)]
    fn get_state(&self, window: &Window) -> WindowState;
}

/// 窗口服务实现
pub struct WindowService;

impl WindowService {
    /// 创建新的窗口服务实例
    pub fn new() -> Self {
        Self
    }
}

impl WindowServiceTrait for WindowService {
    /// 最小化窗口
    fn minimize(&self, window: &Window) -> WindowOperationResult {
        match window.minimize() {
            Ok(_) => WindowOperationResult {
                success: true,
                error: None,
            },
            Err(e) => WindowOperationResult {
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// 最大化/还原窗口
    fn maximize(&self, window: &Window) -> WindowOperationResult {
        match window.is_maximized() {
            Ok(is_maximized) => {
                let result = if is_maximized {
                    window.unmaximize()
                } else {
                    window.maximize()
                };

                match result {
                    Ok(_) => WindowOperationResult {
                        success: true,
                        error: None,
                    },
                    Err(e) => WindowOperationResult {
                        success: false,
                        error: Some(e.to_string()),
                    },
                }
            }
            Err(e) => WindowOperationResult {
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// 关闭窗口
    fn close(&self, window: &Window) -> WindowOperationResult {
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
    }

    /// 切换全屏模式
    fn toggle_fullscreen(&self, window: &Window) -> WindowOperationResult {
        match window.is_fullscreen() {
            Ok(is_fullscreen) => {
                let result = window.set_fullscreen(!is_fullscreen);

                match result {
                    Ok(_) => WindowOperationResult {
                        success: true,
                        error: None,
                    },
                    Err(e) => WindowOperationResult {
                        success: false,
                        error: Some(e.to_string()),
                    },
                }
            }
            Err(e) => WindowOperationResult {
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// 设置窗口大小
    fn set_size(&self, window: &Window, request: SetWindowSizeRequest) -> WindowOperationResult {
        match window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: request.width,
            height: request.height,
        })) {
            Ok(_) => WindowOperationResult {
                success: true,
                error: None,
            },
            Err(e) => WindowOperationResult {
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// 获取窗口状态
    #[allow(dead_code)]
    fn get_state(&self, window: &Window) -> WindowState {
        let inner_size = window.inner_size().unwrap_or(tauri::PhysicalSize {
            width: 1600,
            height: 900,
        });
        let outer_position = window.outer_position().unwrap_or(tauri::PhysicalPosition { x: 0, y: 0 });
        let maximized = window.is_maximized().unwrap_or(false);
        let fullscreen = window.is_fullscreen().unwrap_or(false);

        WindowState {
            width: inner_size.width,
            height: inner_size.height,
            x: outer_position.x,
            y: outer_position.y,
            maximized,
            fullscreen,
        }
    }
}

impl Default for WindowService {
    fn default() -> Self {
        Self::new()
    }
}
