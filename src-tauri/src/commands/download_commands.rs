// 下载命令
// 处理游戏本体下载相关的IPC调用

use crate::services::{
    DownloadService, DownloadServiceTrait,
};
use tauri::AppHandle;
use crate::services::game_data_service;
use crate::utils::resource_utils::get_resource_dir;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// 全局下载进程管理表
/// key: game_id, value: ddv20.exe 子进程句柄
static DOWNLOAD_PROCESSES: Lazy<Mutex<HashMap<String, std::process::Child>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 读取清单文件夹
#[tauri::command]
pub fn read_manifest_folder(folder_path: String) -> Result<crate::services::ManifestFolderResult, String> {
    let service = DownloadService::new();
    service.read_manifest_folder(&folder_path)
}

/// 读取文本文件
#[tauri::command]
pub fn read_text_file(file_path: String) -> Result<String, String> {
    let service = DownloadService::new();
    service.read_text_file(&file_path)
}

/// 读取JSON文件
#[tauri::command]
pub fn read_json_file(file_path: String) -> Result<serde_json::Value, String> {
    let service = DownloadService::new();
    service.read_json_file(&file_path)
}

/// 获取可用盘符
#[tauri::command]
pub fn get_available_drive() -> Result<String, String> {
    let service = DownloadService::new();
    service.get_available_drive()
}

/// 获取清单路径
#[tauri::command]
pub fn get_manifest_path(app: AppHandle, game_id: String) -> Result<String, String> {
    let service = DownloadService::new();
    service.get_manifest_path(&app, &game_id)
}

/// 启动游戏下载
#[tauri::command]
pub async fn start_game_download(
    app: AppHandle,
    manifest_path: String,
    download_path: String,
    game_id: String,
) -> Result<crate::services::DownloadResult, String> {
    let service = DownloadService::new();

    // 如果该游戏已有正在运行的下载进程，先终止旧进程，避免同一游戏多实例冲突
    {
        let mut processes = DOWNLOAD_PROCESSES.lock().map_err(|e| format!("锁获取失败: {}", e))?;
        if let Some(mut old_child) = processes.remove(&game_id) {
            let _ = old_child.kill();
            log::info!("游戏 {} 存在旧下载进程，已终止", game_id);
        }
    }

    let start_result = service.start_game_download(&app, &manifest_path, &download_path, &game_id);

    match start_result {
        Ok((result, child)) => {
            let process_id = result.process_id;

            // 保存子进程句柄到全局管理表
            {
                let mut processes = DOWNLOAD_PROCESSES.lock().map_err(|e| format!("锁获取失败: {}", e))?;
                processes.insert(game_id.clone(), child);
            }

            // 更新游戏状态为 downloading
            let _ = game_data_service::update_download_status(
                app.clone(),
                game_id.clone(),
                "downloading".to_string(),
                0,
            ).await;

            // 启动后台监控任务，支持自动续传
            let app_handle = app.clone();
            let manifest_path_clone = manifest_path.clone();
            let download_path_clone = download_path.clone();
            let game_id_clone = game_id.clone();

            thread::spawn(move || {
                log::info!("启动下载监控任务，游戏ID: {}, PID: {:?}", game_id_clone, process_id);
                monitor_download_process(
                    app_handle,
                    game_id_clone,
                    manifest_path_clone,
                    download_path_clone,
                );
            });

            Ok(result)
        }
        Err(e) => Err(e),
    }
}

/// 监控单个游戏的下载进程
/// 通过全局进程表中的 Child 句柄进行精准监控，支持多实例并行下载
fn monitor_download_process(
    app_handle: AppHandle,
    game_id: String,
    manifest_path: String,
    download_path: String,
) {
    let service = DownloadService::new();
    let retry_count = Arc::new(AtomicU32::new(0));
    const MAX_RETRIES: u32 = 3;

    // 创建一个 tokio 运行时，整个监控期间复用，避免反复创建 Runtime 导致内存泄漏
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            log::error!("创建 tokio 运行时失败: {}", e);
            return;
        }
    };

    loop {
        // 等待进程启动稳定
        thread::sleep(Duration::from_secs(3));

        // 等待该游戏对应的 ddv20.exe 进程退出
        wait_for_process_exit(&game_id);

        log::info!("检测到游戏 {} 的 ddv20.exe 进程已退出", game_id);

        // 检查是否是用户主动停止的下载
        if crate::take_download_stopped(&game_id) {
            log::info!("游戏 {} 被用户主动停止，不再自动续传", game_id);
            let _ = DOWNLOAD_PROCESSES.lock()
                .map(|mut processes| { processes.remove(&game_id); });
            break;
        }

        // 检查游戏是否全部下载完成
        let is_completed = rt.block_on(async {
            service.check_all_depots_completed(&app_handle, &game_id).unwrap_or(false)
        });

        if is_completed {
            rt.block_on(async {
                let _ = game_data_service::update_download_status(
                    app_handle.clone(),
                    game_id.clone(),
                    "completed".to_string(),
                    100,
                ).await;
                let _ = game_data_service::finalize_download(
                    app_handle.clone(),
                    game_id.clone(),
                ).await;
            });
            log::info!("游戏 {} 所有 depot 下载完成", game_id);
            break;
        }

        // 下载未完成，检查是否需要重试
        let current_retry = retry_count.fetch_add(1, Ordering::SeqCst);
        if current_retry >= MAX_RETRIES {
            rt.block_on(async {
                if let Ok(Some(game)) = game_data_service::get_game(app_handle.clone(), game_id.clone()).await {
                    if game.download_status == "downloading" {
                        let _ = game_data_service::update_download_status(
                            app_handle.clone(),
                            game_id.clone(),
                            "error".to_string(),
                            game.download_progress,
                        ).await;
                    }
                }
            });
            log::warn!("游戏 {} 达到最大重试次数，标记为错误状态", game_id);
            break;
        }

        // 等待3秒后重新启动下载
        log::info!("游戏 {} 将在3秒后自动续传 (重试 {}/{})", game_id, current_retry + 1, MAX_RETRIES);
        thread::sleep(Duration::from_secs(3));

        // 重新启动 ddv20.exe 进行续传
        match service.start_game_download(
            &app_handle,
            &manifest_path,
            &download_path,
            &game_id,
        ) {
            Ok((_, child)) => {
                let mut processes = match DOWNLOAD_PROCESSES.lock() {
                    Ok(p) => p,
                    Err(e) => {
                        log::error!("游戏 {} 续传时锁获取失败: {}", game_id, e);
                        continue;
                    }
                };
                processes.insert(game_id.clone(), child);
                log::info!("游戏 {} 续传已启动", game_id);
            }
            Err(e) => {
                log::error!("游戏 {} 续传启动失败: {}", game_id, e);
            }
        }
    }

    log::info!("下载监控任务已结束，游戏ID: {}", game_id);
}

/// 等待指定游戏的 ddv20.exe 进程退出
/// 通过全局进程表中的 Child 句柄精准判断，不影响其他游戏的下载进程
/// 检查完成后立即释放锁，避免阻塞 stop_download 等操作
fn wait_for_process_exit(game_id: &str) {
    loop {
        thread::sleep(Duration::from_secs(5));

        let is_running = {
            let mut processes = match DOWNLOAD_PROCESSES.lock() {
                Ok(p) => p,
                Err(e) => {
                    log::error!("获取进程表锁失败: {}", e);
                    continue;
                }
            };

            match processes.get_mut(game_id) {
                Some(child) => {
                    match child.try_wait() {
                        Ok(Some(_status)) => {
                            // 进程已退出，从表中移除
                            processes.remove(game_id);
                            false
                        }
                        Ok(None) => {
                            // 进程仍在运行
                            true
                        }
                        Err(e) => {
                            log::error!("检查游戏 {} 进程状态时出错: {}", game_id, e);
                            processes.remove(game_id);
                            false
                        }
                    }
                }
                None => {
                    // 进程表中不存在该游戏进程，视为已退出
                    false
                }
            }
        };

        if !is_running {
            return;
        }
    }
}

/// 获取下载进度文件
/// 可选传入 game_id，只获取该游戏的进度文件
#[tauri::command]
pub fn get_download_progress_files(game_id: Option<String>) -> Result<Vec<crate::services::ProgressFileInfo>, String> {
    let service = DownloadService::new();
    service.get_download_progress_files(game_id.as_deref())
}

/// 读取目录
#[tauri::command]
pub fn read_directory(path: String) -> Result<Vec<crate::services::DirEntry>, String> {
    let service = DownloadService::new();
    service.read_directory(&path)
}

/// 删除文件
#[tauri::command]
pub fn delete_file(file_path: String) -> Result<(), String> {
    let service = DownloadService::new();
    service.delete_file(&file_path)
}

/// 删除游戏的清单文件夹
/// 用于"替换清单"功能，清空 resources/manifest/{game_id} 目录
#[tauri::command]
pub fn delete_game_manifest_folder(app: AppHandle, game_id: String) -> Result<(), String> {
    let resource_dir = get_resource_dir(&app)?;
    let manifest_dir = resource_dir.join("manifest").join(&game_id);

    if manifest_dir.exists() {
        std::fs::remove_dir_all(&manifest_dir)
            .map_err(|e| format!("删除清单文件夹失败: {}", e))?;
    }

    Ok(())
}

/// 关闭系统
#[tauri::command]
pub fn shutdown_system() -> Result<(), String> {
    let service = DownloadService::new();
    service.shutdown_system()
}

/// 获取游戏的 depot 列表
#[tauri::command]
pub fn get_game_depots(app: AppHandle, game_id: String) -> Result<Vec<String>, String> {
    let service = DownloadService::new();
    service.get_game_depots(&app, &game_id)
}

/// 检查并清理已完成的下载
/// 当游戏的所有 depot 都下载完成后，自动静默删除对应的进度 JSON 文件
/// 可选传入 game_id，只清理该游戏的进度文件
#[tauri::command]
pub fn check_and_cleanup_completed_downloads(app: AppHandle, game_id: Option<String>) -> Result<(), String> {
    let service = DownloadService::new();
    service.check_and_cleanup_completed_downloads(&app, game_id.as_deref())
}

/// 停止下载进程
/// 终止指定游戏对应的 ddv20.exe 进程，不影响其他正在下载的游戏
/// 并将游戏状态设置为 idle（未下载）
#[tauri::command]
pub async fn stop_download(app: AppHandle, game_id: String) -> Result<(), String> {
    // 标记该游戏ID为用户主动停止，防止监控线程自动续传
    crate::mark_download_stopped(&game_id);

    // 从全局进程表中取出该游戏的子进程句柄并终止
    {
        let mut processes = DOWNLOAD_PROCESSES.lock()
            .map_err(|e| format!("获取进程表锁失败: {}", e))?;

        if let Some(mut child) = processes.remove(&game_id) {
            if let Err(e) = child.kill() {
                log::warn!("终止游戏 {} 的 ddv20.exe 进程失败: {}", game_id, e);
            } else {
                log::info!("已终止游戏 {} 的 ddv20.exe 进程", game_id);
            }
        }
    }

    // 将游戏状态设置为 idle（未下载）
    if let Ok(Some(game)) = game_data_service::get_game(app.clone(), game_id.clone()).await {
        let _ = game_data_service::update_download_status(
            app,
            game_id,
            "idle".to_string(),
            game.download_progress,
        ).await;
    }

    Ok(())
}
