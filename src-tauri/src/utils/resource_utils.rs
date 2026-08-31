// 资源目录工具
// 提供获取资源目录的统一方法，避免重复代码

use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};

/// 资源目录解析结果缓存
/// 进程内只解析一次，后续调用直接返回缓存结果，避免每次 IO 探测与日志开销
static RESOURCE_DIR_CACHE: OnceLock<PathBuf> = OnceLock::new();

/// 获取资源目录路径
/// 首次调用执行探测并缓存，后续调用直接返回缓存结果
/// 按以下优先级查找：
/// 1. 程序所在目录的 resources/Resources 文件夹（打包后）
/// 2. 程序所在目录的父目录的 resources 文件夹（开发模式）
/// 3. 向上查找项目根目录的 resources 文件夹（开发模式）
/// 4. Tauri 的 resource_dir
pub fn get_resource_dir(app: &AppHandle) -> Result<PathBuf, String> {
    if let Some(dir) = RESOURCE_DIR_CACHE.get() {
        return Ok(dir.clone());
    }

    let dir = resolve_resource_dir(app)?;
    // 首次解析后写入缓存；并发下 set 可能被拒绝，但 dir 本身即为正确结果
    let _ = RESOURCE_DIR_CACHE.set(dir.clone());
    Ok(dir)
}

/// 执行实际的资源目录探测（仅在首次调用时执行一次）
fn resolve_resource_dir(app: &AppHandle) -> Result<PathBuf, String> {
    // 获取程序所在目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();

    log::info!("程序所在目录: {}", exe_dir.display());

    // 1. 尝试程序所在目录的 resources（打包后的标准结构）
    let resources_path = exe_dir.join("resources");
    if resources_path.exists() && resources_path.is_dir() {
        log::info!("找到 resources 目录: {}", resources_path.display());
        return Ok(resources_path);
    }

    // 2. 尝试程序所在目录的 Resources（兼容大小写）
    let resources_path_capital = exe_dir.join("Resources");
    if resources_path_capital.exists() && resources_path_capital.is_dir() {
        log::info!("找到 Resources 目录: {}", resources_path_capital.display());
        return Ok(resources_path_capital);
    }

    // 3. 如果在 target/debug 或 target/release 目录下，向上查找项目根目录
    let mut check_dir = exe_dir.clone();
    for depth in 0..5 {
        if let Some(parent) = check_dir.parent() {
            check_dir = parent.to_path_buf();
            let project_resources = check_dir.join("resources");
            log::info!("检查项目 resources 路径 (深度{}): {}", depth, project_resources.display());
            if project_resources.exists() && project_resources.is_dir() {
                log::info!("找到项目 resources 目录: {}", project_resources.display());
                return Ok(project_resources);
            }
        } else {
            break;
        }
    }

    // 4. 最后尝试使用 Tauri 的 resource_dir
    if let Ok(resource_dir) = app.path().resource_dir() {
        log::info!("Tauri resource_dir: {}", resource_dir.display());
        if resource_dir.exists() && resource_dir.is_dir() {
            return Ok(resource_dir);
        }
    }

    // 5. 如果都找不到，尝试当前工作目录
    if let Ok(current_dir) = std::env::current_dir() {
        let cwd_resources = current_dir.join("resources");
        if cwd_resources.exists() && cwd_resources.is_dir() {
            log::info!("找到当前工作目录 resources: {}", cwd_resources.display());
            return Ok(cwd_resources);
        }
    }

    log::error!("无法找到资源目录，程序目录: {}", exe_dir.display());
    Err("无法找到资源目录".to_string())
}