// 下载命令
// 处理游戏本体下载相关的IPC调用

use crate::services::{
    DownloadService, DownloadServiceTrait,
};
use tauri::AppHandle;
use crate::services::game_data_service;
use std::thread;
use std::time::Duration;

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
    let result = service.start_game_download(&app, &manifest_path, &download_path);

    // 如果下载启动成功，启动监控任务
    if result.is_ok() {
        let app_handle = app.clone();
        let game_id_clone = game_id.clone();

        // 更新游戏状态为 downloading
        let _ = game_data_service::update_download_status(
            app.clone(),
            game_id.clone(),
            "downloading".to_string(),
            0,
        ).await;

        // 启动后台监控任务
        thread::spawn(move || {
            log::info!("启动下载监控任务，游戏ID: {}", game_id_clone);

            // 持续监控直到 ddv20.exe 进程退出
            loop {
                thread::sleep(Duration::from_secs(5));

                // 检查 ddv20.exe 是否还在运行
                let is_running = crate::check_ddv20_running();

                if !is_running {
                    // ddv20.exe 已退出，检查游戏状态
                    let app_handle_clone = app_handle.clone();
                    let game_id_check = game_id_clone.clone();

                    // 使用 tokio 运行时执行异步操作
                    let rt = tokio::runtime::Runtime::new();
                    if let Ok(rt) = rt {
                        let _ = rt.block_on(async {
                            // 检查游戏当前状态
                            if let Ok(Some(game)) = game_data_service::get_game(app_handle_clone.clone(), game_id_check.clone()).await {
                                // 如果状态是 downloading 且进度未到 100%，标记为 idle
                                if game.download_status == "downloading" && game.download_progress < 100 {
                                    let _ = game_data_service::update_download_status(
                                        app_handle_clone,
                                        game_id_check,
                                        "idle".to_string(),
                                        game.download_progress,
                                    ).await;
                                    log::info!("检测到 ddv20.exe 进程已退出，游戏 {} 下载状态已重置为 idle", game_id_clone);
                                }
                            }
                        });
                    }

                    // 退出监控循环
                    break;
                }
            }

            log::info!("下载监控任务已结束，游戏ID: {}", game_id_clone);
        });
    }

    result
}

/// 获取下载进度文件
#[tauri::command]
pub fn get_download_progress_files() -> Result<Vec<crate::services::ProgressFileInfo>, String> {
    let service = DownloadService::new();
    service.get_download_progress_files()
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
#[tauri::command]
pub fn check_and_cleanup_completed_downloads(app: AppHandle) -> Result<(), String> {
    let service = DownloadService::new();
    service.check_and_cleanup_completed_downloads(&app)
}

/// 停止下载进程
/// 终止 ddv20.exe 进程，并将游戏状态设置为 idle（未下载）
#[tauri::command]
pub async fn stop_download(app: AppHandle, game_id: String) -> Result<(), String> {
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
