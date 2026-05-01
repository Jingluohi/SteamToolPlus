// 资源目录工具
// 提供获取资源目录的统一方法，避免重复代码

use std::path::PathBuf;
use tauri::AppHandle;

/// 获取资源目录路径
pub fn get_resource_dir(_app: &AppHandle) -> Result<PathBuf, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();

    let resources_path = exe_dir.join("resources");
    if resources_path.exists() && resources_path.is_dir() {
        return Ok(resources_path);
    }

    Err("无法找到资源目录".to_string())
}

/// 获取资源目录路径（同步版本）
pub fn get_resource_dir_sync() -> Result<PathBuf, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();

    let resources_path = exe_dir.join("resources");
    if resources_path.exists() && resources_path.is_dir() {
        return Ok(resources_path);
    }

    Err("无法找到资源目录".to_string())
}

/// 构建资源文件路径
pub fn get_resource_file_path(relative_path: &str) -> Result<PathBuf, String> {
    let resource_dir = get_resource_dir_sync()?;

    let path_lower = relative_path.to_lowercase();
    let clean_path = if path_lower.starts_with("resources/") {
        &relative_path[10..]
    } else if path_lower.starts_with("resources\\") {
        &relative_path[11..]
    } else {
        relative_path
    };

    Ok(resource_dir.join(clean_path))
}
