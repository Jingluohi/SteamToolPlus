// 配置命令
// 处理配置相关的IPC调用

use crate::models::{AppConfig, UpdateConfigRequest};
use crate::AppState;

/// 获取当前配置
#[tauri::command]
pub fn get_config(state: tauri::State<AppState>) -> AppConfig {
    state.config_service.get_config()
}

/// 更新配置
#[tauri::command]
pub fn update_config(
    state: tauri::State<AppState>,
    request: UpdateConfigRequest,
) -> Result<AppConfig, String> {
    state.config_service.update_config(request)
}

/// 重置配置到默认值
#[tauri::command]
pub fn reset_config(state: tauri::State<AppState>) -> AppConfig {
    state.config_service.reset_config()
}
