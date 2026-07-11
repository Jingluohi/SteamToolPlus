// 公共类型定义和工具函数
// 包含所有模块共享的结果类型和基础命令

use crate::utils::resource_utils::get_resource_dir as get_resource_dir_util;
use std::path::Path;
use tauri::AppHandle;

// ============================================
// 结果类型定义
// ============================================

/// 备份结果
#[derive(serde::Serialize)]
pub struct BackupResult {
    pub success: bool,
    pub message: String,
}

/// 脱壳结果
#[derive(serde::Serialize)]
pub struct UnpackResult {
    pub success: bool,
    pub message: String,
    pub unpacked_path: Option<String>,
}

/// 基础配置结果
#[derive(serde::Serialize)]
pub struct BasicConfigResult {
    pub success: bool,
    pub message: String,
}

/// 配置保存结果
#[derive(serde::Serialize)]
pub struct ConfigSaveResult {
    pub success: bool,
    pub message: String,
}

/// 配置加载结果（泛型）
#[derive(serde::Serialize)]
pub struct ConfigLoadResult<T> {
    pub exists: bool,
    pub config: Option<T>,
}

/// 导入结果（泛型）
#[derive(serde::Serialize)]
pub struct ImportResult<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub message: String,
}

/// 导出结果
#[derive(serde::Serialize)]
pub struct ExportResult {
    pub success: bool,
    pub data: Option<String>,
    pub message: String,
}

/// 补丁应用结果
#[derive(serde::Serialize)]
pub struct ApplyPatchResult {
    pub success: bool,
    pub backed_up_files: Vec<String>,
    pub copied_files: Vec<String>,
    pub errors: Vec<String>,
}

// ============================================
// 通用配置读写辅助函数
// ============================================

use crate::utils::file_utils;
use serde::{de::DeserializeOwned, Serialize};

/// 保存 steam_settings 目录下的 JSON 配置文件
pub async fn save_steam_settings_json<T: Serialize>(
    game_path: &str,
    file_name: &str,
    config: &T,
) -> Result<(), String> {
    let path = Path::new(game_path).join("steam_settings").join(file_name);
    file_utils::write_json_file_async(&path, config).await
}

/// 加载 steam_settings 目录下的 JSON 配置文件
pub async fn load_steam_settings_json<T: DeserializeOwned>(
    game_path: &str,
    file_name: &str,
) -> Result<Option<T>, String> {
    let path = Path::new(game_path).join("steam_settings").join(file_name);

    if !path.exists() {
        return Ok(None);
    }

    let config = file_utils::read_json_file_async(&path).await?;
    Ok(Some(config))
}

// ============================================
// 基础命令
// ============================================

/// 获取资源目录（IPC命令版本）
#[tauri::command]
pub fn get_resource_dir(app: AppHandle) -> Result<String, String> {
    let path = get_resource_dir_util(&app)?;
    Ok(path.to_string_lossy().to_string())
}

/// 规范化资源相对路径
/// 移除用户传入路径中可能包含的 "resources/" 或 "Resources\\" 前缀，使其统一为相对于资源目录的路径
pub fn normalize_resource_relative_path(path: &str) -> &str {
    let path_lower = path.to_lowercase();
    if path_lower.starts_with("resources/") {
        &path[10..]
    } else if path_lower.starts_with("resources\\") {
        &path[11..]
    } else {
        path
    }
}

/// 检查路径是否存在
#[tauri::command]
pub fn path_exists(path: String) -> Result<bool, String> {
    Ok(Path::new(&path).exists())
}

/// 获取补丁说明
/// 根据游戏ID和补丁类型读取对应的 Readme 文本
#[tauri::command]
pub async fn get_patch_readme(
    app: AppHandle,
    game_id: String,
    patch_type: i32,
) -> Result<String, String> {
    let resource_dir = get_resource_dir(app.clone())?;

    let readme_subdir = match patch_type {
        0 => "免_steam",
        1 => "局域网联机",
        2 => "Steam联机",
        3 => "D_加密虚拟机",
        4 => "epic_联机",
        _ => "免_steam",
    };

    let readme_path = Path::new(&resource_dir)
        .join("Readme")
        .join(readme_subdir)
        .join(&format!("{}.txt", game_id));

    if !readme_path.exists() {
        return Ok(String::new());
    }

    match tokio::fs::read_to_string(&readme_path).await {
        Ok(content) => Ok(content),
        Err(_) => Ok(String::new()),
    }
}

// ============================================
// 通用命令
// ============================================

/// 关闭应用程序
#[tauri::command]
pub fn close_application() -> Result<(), String> {
    std::process::exit(0);
}

/// 打开外部链接
/// 使用 CREATE_NO_WINDOW 标志防止闪烁终端窗口
#[tauri::command]
pub fn open_external_link(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use std::process::Command;

        const CREATE_NO_WINDOW: u32 = 0x08000000;

        Command::new("cmd")
            .args(["/c", "start", "", &url])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("打开链接失败: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开链接失败: {}", e))?;
    }

    Ok(())
}

/// 写入文本文件
#[tauri::command]
pub async fn write_text_file(path: String, content: String) -> Result<(), String> {
    use tokio::io::AsyncWriteExt;

    // 确保父目录存在
    if let Some(parent) = Path::new(&path).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let mut file = tokio::fs::File::create(&path)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;

    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

/// 打开虚拟化环境配置教程视频
/// 使用系统默认应用打开 resources/D_加密虚拟化（虚拟机）环境搭建教程.mp4
#[tauri::command]
pub async fn open_virtualization_tutorial(_app: AppHandle) -> Result<(), String> {
    use std::process::Command;

    // 获取程序根目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();

    // 构建视频文件路径
    let video_path = exe_dir.join("resources").join("D_加密虚拟化（虚拟机）环境搭建教程.mp4");

    // 检查文件是否存在
    if !video_path.exists() {
        return Err("视频文件不存在".to_string());
    }

    // 使用系统默认应用打开视频
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", video_path.to_string_lossy().as_ref()])
            .spawn()
            .map_err(|e| format!("打开视频失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&video_path)
            .spawn()
            .map_err(|e| format!("打开视频失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&video_path)
            .spawn()
            .map_err(|e| format!("打开视频失败: {}", e))?;
    }

    Ok(())
}

// ============================================
// 补丁应用命令
// ============================================

/// 应用补丁 - 解压7z文件并复制到游戏目录
#[tauri::command]
pub async fn apply_patch(
    app: AppHandle,
    patch_source_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    let resource_dir = get_resource_dir_util(&app)?;

    let patch_relative_path = normalize_resource_relative_path(&patch_source_path);

    let patch_file_name = if patch_relative_path.ends_with(".7z") {
        patch_relative_path.to_string()
    } else {
        format!("{}.7z", patch_relative_path)
    };

    let patch_file_path = Path::new(&resource_dir).join(&patch_file_name);

    if !patch_file_path.exists() {
        return Err(format!("补丁源路径不存在: {}", patch_file_path.display()));
    }

    apply_patch_internal(&patch_file_path, &game_path).await
}

/// 从用户选择的压缩包文件应用补丁
#[tauri::command]
pub async fn apply_patch_from_file(
    archive_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    let archive_path = Path::new(&archive_path);

    if !archive_path.exists() {
        return Err(format!("补丁文件不存在: {}", archive_path.display()));
    }

    apply_patch_internal(archive_path, &game_path).await
}

/// 应用补丁的内部通用实现
/// 解压 7z 压缩包到临时目录，再递归复制到游戏目录，并备份已存在的文件
async fn apply_patch_internal(
    archive_path: &Path,
    game_path: &str,
) -> Result<ApplyPatchResult, String> {
    let target_path = Path::new(game_path);

    if !target_path.exists() {
        return Err(format!("游戏目标路径不存在: {}", game_path));
    }

    let temp_dir = std::env::temp_dir().join(format!("steam_tool_patch_{}", std::process::id()));
    let temp_path = temp_dir.clone();

    if temp_path.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
    }
    tokio::fs::create_dir_all(&temp_path)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    match extract_7z(archive_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    let mut backed_up_files: Vec<String> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    match copy_dir_with_backup(&temp_path, target_path, &mut backed_up_files, &mut copied_files, &mut errors).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }

    let _ = tokio::fs::remove_dir_all(&temp_path).await;

    Ok(ApplyPatchResult {
        success: errors.is_empty(),
        backed_up_files,
        copied_files,
        errors,
    })
}

/// 使用 sevenz-rust 解压7z文件
async fn extract_7z(
    archive_path: &Path,
    output_dir: &Path,
) -> Result<(), String> {
    use tokio::time::{timeout, Duration};

    let archive_str = archive_path
        .to_str()
        .ok_or("补丁文件路径包含非法字符")?;
    let output_str = output_dir
        .to_str()
        .ok_or("临时目录路径包含非法字符")?;

    let result = timeout(
        Duration::from_secs(300),
        async {
            sevenz_rust::decompress_file(archive_str, output_str)
                .map_err(|e| format!("解压7z文件失败: {:?}", e))
        }
    ).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("解压操作超时（超过5分钟）".to_string()),
    }
}

/// 递归复制目录，并备份已存在的文件
async fn copy_dir_with_backup(
    src: &Path,
    dst: &Path,
    backed_up_files: &mut Vec<String>,
    copied_files: &mut Vec<String>,
    errors: &mut Vec<String>,
) -> Result<(), String> {
    use tokio::fs;

    let mut entries = fs::read_dir(src).await
        .map_err(|e| format!("无法读取源目录: {}", e))?;

    while let Some(entry) = entries.next_entry().await
        .map_err(|e| format!("读取目录条目失败: {}", e))? {

        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if src_path.is_dir() {
            if !dst_path.exists() {
                fs::create_dir_all(&dst_path).await
                    .map_err(|e| format!("创建目录失败: {}", e))?;
            }

            Box::pin(copy_dir_with_backup(
                &src_path,
                &dst_path,
                backed_up_files,
                copied_files,
                errors,
            )).await?;
        } else {
            if dst_path.exists() {
                match backup_file(&dst_path).await {
                    Ok(backup_path) => {
                        backed_up_files.push(backup_path.to_string_lossy().to_string());
                    }
                    Err(e) => {
                        errors.push(format!("备份文件失败 {}: {}", dst_path.display(), e));
                        continue;
                    }
                }
            }

            match fs::copy(&src_path, &dst_path).await {
                Ok(_) => {
                    copied_files.push(dst_path.to_string_lossy().to_string());
                }
                Err(e) => {
                    errors.push(format!("复制文件失败 {} -> {}: {}",
                        src_path.display(), dst_path.display(), e));
                }
            }
        }
    }

    Ok(())
}

/// 备份文件（自动递增 .bak 后缀）
async fn backup_file(file_path: &Path) -> Result<std::path::PathBuf, String> {
    use tokio::fs;

    let file_name = file_path.file_stem()
        .ok_or("无法获取文件名")?
        .to_string_lossy();
    let extension = file_path.extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let parent = file_path.parent()
        .ok_or("无法获取父目录")?;

    let mut backup_number = 0;
    let backup_path = loop {
        let suffix = if backup_number == 0 {
            ".bak".to_string()
        } else {
            format!(".bak{}", backup_number)
        };

        let backup_name = format!("{}{}{}", file_name, suffix, extension);
        let backup_path = parent.join(&backup_name);

        if !backup_path.exists() {
            break backup_path;
        }

        backup_number += 1;

        if backup_number > 100 {
            return Err("无法找到可用的备份文件名".to_string());
        }
    };

    fs::rename(file_path, &backup_path).await
        .map_err(|e| format!("重命名文件失败: {}", e))?;

    Ok(backup_path)
}
