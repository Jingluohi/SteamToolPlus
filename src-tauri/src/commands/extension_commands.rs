// 扩展命令
// 处理扩展系统相关的IPC调用

use crate::models::{
    Extension, ExtensionListResponse, InstallExtensionRequest,
};
use crate::services::{ExtensionService, ExtensionServiceTrait};

/// 获取所有扩展
#[tauri::command]
pub fn get_extensions() -> ExtensionListResponse {
    let service = ExtensionService::new();
    service.get_extensions()
}

/// 安装扩展
#[tauri::command]
pub fn install_extension(request: InstallExtensionRequest) -> Result<Extension, String> {
    let service = ExtensionService::new();
    service.install_extension(request)
}

/// 卸载扩展
#[tauri::command]
pub fn uninstall_extension(id: String) -> Result<(), String> {
    let service = ExtensionService::new();
    service.uninstall_extension(&id)
}

/// 启用/禁用扩展
#[tauri::command]
pub fn toggle_extension(id: String, enabled: bool) -> Result<Extension, String> {
    let service = ExtensionService::new();
    service.toggle_extension(&id, enabled)
}

/// 重新加载扩展
#[tauri::command]
pub fn reload_extension(id: String) -> Result<Extension, String> {
    let service = ExtensionService::new();
    service.reload_extension(&id)
}
