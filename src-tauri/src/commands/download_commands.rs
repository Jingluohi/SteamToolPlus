// 下载命令
// 处理游戏本体下载相关的IPC调用

use crate::services::{
    DownloadService, DownloadServiceTrait,
};
use tauri::AppHandle;
use crate::services::game_data_service;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

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
    let result = service.start_game_download(&app, &manifest_path, &download_path, &game_id);

    // 如果下载启动成功，启动监控任务
    if result.is_ok() {
        let app_handle = app.clone();
        let manifest_path_clone = manifest_path.clone();
        let download_path_clone = download_path.clone();
        let game_id_clone = game_id.clone();

        // 更新游戏状态为 downloading
        let _ = game_data_service::update_download_status(
            app.clone(),
            game_id.clone(),
            "downloading".to_string(),
            0,
        ).await;

        // 启动后台监控任务，支持自动续传
        thread::spawn(move || {
            log::info!("启动下载监控任务，游戏ID: {}", game_id_clone);
            
            // 重试计数器，最多重试3次
            let retry_count = Arc::new(AtomicU32::new(0));
            const MAX_RETRIES: u32 = 3;

            // 持续监控直到下载完成或达到最大重试次数
            loop {
                // 等待 ddv20.exe 启动
                thread::sleep(Duration::from_secs(3));

                // 持续监控直到 ddv20.exe 进程退出
                let mut ddv20_running = false;
                loop {
                    thread::sleep(Duration::from_secs(5));
                    let is_running = crate::check_ddv20_running();
                    if is_running {
                        ddv20_running = true;
                    } else if ddv20_running {
                        // ddv20.exe 从运行状态变为退出状态
                        break;
                    }
                }

                log::info!("检测到 ddv20.exe 进程已退出，游戏ID: {}", game_id_clone);

                // 检查是否是用户主动停止的下载
                if crate::take_download_stopped(&game_id_clone) {
                    log::info!("游戏 {} 被用户主动停止，不再自动续传", game_id_clone);
                    break;
                }

                // 检查游戏是否全部下载完成
                let app_check = app_handle.clone();
                let game_id_check = game_id_clone.clone();
                let is_completed = {
                    let rt = tokio::runtime::Runtime::new();
                    if let Ok(rt) = rt {
                        rt.block_on(async {
                            service.check_all_depots_completed(&app_check, &game_id_check).unwrap_or(false)
                        })
                    } else {
                        false
                    }
                };

                if is_completed {
                    // 下载已完成，更新状态并退出监控
                    let app_complete = app_handle.clone();
                    let game_id_complete = game_id_clone.clone();
                    let rt = tokio::runtime::Runtime::new();
                    if let Ok(rt) = rt {
                        rt.block_on(async {
                            let _ = game_data_service::update_download_status(
                                app_complete,
                                game_id_complete,
                                "completed".to_string(),
                                100,
                            ).await;
                        });
                    }
                    log::info!("游戏 {} 所有 depot 下载完成", game_id_clone);
                    break;
                }

                // 下载未完成，检查是否需要重试
                let current_retry = retry_count.fetch_add(1, Ordering::SeqCst);
                if current_retry >= MAX_RETRIES {
                    // 达到最大重试次数，标记为错误状态
                    let app_error = app_handle.clone();
                    let game_id_error = game_id_clone.clone();
                    let rt = tokio::runtime::Runtime::new();
                    if let Ok(rt) = rt {
                        rt.block_on(async {
                            if let Ok(Some(game)) = game_data_service::get_game(app_error.clone(), game_id_error.clone()).await {
                                if game.download_status == "downloading" {
                                    let _ = game_data_service::update_download_status(
                                        app_error,
                                        game_id_error,
                                        "error".to_string(),
                                        game.download_progress,
                                    ).await;
                                }
                            }
                        });
                    }
                    log::warn!("游戏 {} 达到最大重试次数，标记为错误状态", game_id_clone);
                    break;
                }

                // 等待3秒后重新启动下载
                log::info!("游戏 {} 将在3秒后自动续传 (重试 {}/{})", game_id_clone, current_retry + 1, MAX_RETRIES);
                thread::sleep(Duration::from_secs(3));

                // 重新启动 ddv20.exe 进行续传
                let restart_result = service.start_game_download(
                    &app_handle,
                    &manifest_path_clone,
                    &download_path_clone,
                    &game_id_clone,
                );

                if let Err(e) = restart_result {
                    log::error!("游戏 {} 续传启动失败: {}", game_id_clone, e);
                    // 续传失败，继续循环会再次重试
                } else {
                    log::info!("游戏 {} 续传已启动", game_id_clone);
                }
            }

            log::info!("下载监控任务已结束，游戏ID: {}", game_id_clone);
        });
    }

    result
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
/// 终止 ddv20.exe 进程，并将游戏状态设置为 idle（未下载）
#[tauri::command]
pub async fn stop_download(app: AppHandle, game_id: String) -> Result<(), String> {
    // 标记该游戏ID为用户主动停止，防止监控线程自动续传
    crate::mark_download_stopped(&game_id);

    // 尝试终止 ddv20.exe 进程（如果存在）
    let service = DownloadService::new();
    let _ = service.stop_download(); // 忽略错误，因为进程可能不存在

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
