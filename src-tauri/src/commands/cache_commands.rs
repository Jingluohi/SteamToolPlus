// 缓存命令
// 处理缓存相关的IPC调用

use crate::AppState;
use std::path::Path;
use tauri::State;

/// 系统缓存清理结果
#[derive(serde::Serialize)]
pub struct SystemCacheClearResult {
    /// 释放的字节数
    pub freed_size: u64,
    /// 删除的文件/目录数量
    pub deleted_count: usize,
}

/// 添加手动导入过清单的游戏ID到缓存
#[tauri::command]
pub fn add_imported_manifest_game_id(
    state: State<AppState>,
    game_id: String,
) -> Result<(), String> {
    state.cache_service.add_imported_manifest_game_id(&game_id)
}

/// 获取缓存数据
#[tauri::command]
pub fn get_cache(state: State<AppState>) -> Result<crate::models::CacheData, String> {
    state.cache_service.get_cache()
}

/// 清除导入过的清单缓存
/// 返回成功删除的文件夹数量
#[tauri::command]
pub fn clear_imported_manifest_cache(
    state: State<AppState>,
    app: tauri::AppHandle,
) -> Result<usize, String> {
    state.cache_service.clear_imported_manifest_cache(&app)
}

/// 获取程序在系统临时目录(%TEMP%)中生成的缓存大小
/// 只统计以 steam_tool_ 开头的文件/目录
#[tauri::command]
pub fn get_system_cache_size() -> Result<u64, String> {
    let temp_dir = std::env::temp_dir();
    calculate_steam_tool_cache_size(&temp_dir)
}

/// 清理程序在系统临时目录(%TEMP%)中生成的缓存
/// 删除所有以 steam_tool_ 开头的文件/目录
#[tauri::command]
pub fn clear_system_cache() -> Result<SystemCacheClearResult, String> {
    let temp_dir = std::env::temp_dir();
    let mut freed_size: u64 = 0;
    let mut deleted_count: usize = 0;

    let entries = std::fs::read_dir(&temp_dir)
        .map_err(|e| format!("读取临时目录失败: {}", e))?;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        // 只处理本程序创建的 steam_tool_ 前缀临时文件
        if name_str.starts_with("steam_tool_") {
            let path = entry.path();
            let size = calculate_path_size(&path).unwrap_or(0);
            // 根据类型选择删除方式，避免对文件调用 remove_dir_all 报错
            let remove_result = if path.is_file() {
                std::fs::remove_file(&path)
            } else {
                std::fs::remove_dir_all(&path)
            };
            match remove_result {
                Ok(_) => {
                    freed_size += size;
                    deleted_count += 1;
                }
                Err(e) => {
                    log::warn!("清理系统缓存失败 {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(SystemCacheClearResult {
        freed_size,
        deleted_count,
    })
}

/// 计算系统临时目录中 steam_tool_ 前缀缓存的总大小
fn calculate_steam_tool_cache_size(dir: &Path) -> Result<u64, String> {
    let mut total: u64 = 0;
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("读取临时目录失败: {}", e))?;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("steam_tool_") {
            total += calculate_path_size(&entry.path())?;
        }
    }

    Ok(total)
}

/// 递归计算单个文件或目录的大小
fn calculate_path_size(path: &Path) -> Result<u64, String> {
    let meta = std::fs::metadata(path)
        .map_err(|e| format!("获取文件元数据失败 {}: {}", path.display(), e))?;

    if meta.is_file() {
        Ok(meta.len())
    } else {
        let mut total = 0u64;
        let entries = std::fs::read_dir(path)
            .map_err(|e| format!("读取目录失败 {}: {}", path.display(), e))?;
        for entry in entries.flatten() {
            total += calculate_path_size(&entry.path())?;
        }
        Ok(total)
    }
}
