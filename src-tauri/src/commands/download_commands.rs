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
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// 全局下载进程管理表
/// key: game_id, value: ddv20.exe 子进程句柄
/// 注意：只有监控线程负责从该表移除 child，stop_download 不直接操作此表
static DOWNLOAD_PROCESSES: Lazy<Mutex<HashMap<String, std::process::Child>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 全局下载停止信号量表
/// key: game_id, value: Arc<AtomicBool> 停止信号
/// stop_download 设置信号 → 监控线程检测信号后自行 kill 进程并清理
static DOWNLOAD_STOP_SIGNALS: Lazy<Mutex<HashMap<String, Arc<AtomicBool>>>> =
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

    // 如果该游戏已有正在运行的下载进程，通过停止信号量通知旧监控线程
    // 旧监控线程检测到信号后会自行 kill 进程并退出，避免直接竞争进程句柄
    {
        let mut old_signals = DOWNLOAD_STOP_SIGNALS.lock()
            .map_err(|e| format!("获取停止信号量表锁失败: {}", e))?;
        if let Some(signal) = old_signals.get(&game_id) {
            signal.store(true, Ordering::SeqCst);
            log::info!("游戏 {} 存在旧下载进程，已发送停止信号", game_id);
        }
        // 移除旧信号量，新进程会创建新的
        old_signals.remove(&game_id);
    }

    let start_result = service.start_game_download(&app, &manifest_path, &download_path, &game_id);

    match start_result {
        Ok((result, child)) => {
            let process_id = result.process_id;

            // 创建新的停止信号量（初始 false），监控线程持有该信号的克隆
            let stop_signal = Arc::new(AtomicBool::new(false));

            // 保存子进程句柄和停止信号量到全局管理表
            {
                let mut processes = DOWNLOAD_PROCESSES.lock()
                    .map_err(|e| format!("锁获取失败: {}", e))?;
                let mut signals = DOWNLOAD_STOP_SIGNALS.lock()
                    .map_err(|e| format!("信号量表锁获取失败: {}", e))?;
                processes.insert(game_id.clone(), child);
                signals.insert(game_id.clone(), stop_signal.clone());
            }

            // 更新游戏状态为 downloading
            let _ = game_data_service::update_download_status(
                app.clone(),
                game_id.clone(),
                "downloading".to_string(),
                0,
            ).await;

            // 启动后台监控任务，传递停止信号量的克隆
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
                    stop_signal, // 传递信号量，监控线程独占进程句柄管理权
                );
            });

            Ok(result)
        }
        Err(e) => Err(e),
    }
}

/// 监控单个游戏的下载进程
/// 监控线程独占进程句柄管理权：stop_download 只发信号，由本线程自行 kill 进程
fn monitor_download_process(
    app_handle: AppHandle,
    game_id: String,
    manifest_path: String,
    download_path: String,
    stop_signal: Arc<AtomicBool>, // 停止信号量，stop_download 设置此信号
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
        // 每次循环开始时检查停止信号
        if stop_signal.load(Ordering::SeqCst) {
            log::info!("游戏 {} 收到停止信号，正在终止进程", game_id);
            kill_process(&game_id);
            break;
        }

        // 等待该游戏对应的 ddv20.exe 进程退出或被停止信号终止
        let stopped_by_user = wait_for_process_exit(&game_id, &stop_signal);

        if stopped_by_user {
            log::info!("游戏 {} 被用户主动停止，不再自动续传", game_id);
            // 清理信号量（stop_signal 是 Arc 克隆，不影响其他引用）
            let _ = DOWNLOAD_STOP_SIGNALS.lock()
                .map(|mut signals| { signals.remove(&game_id); });
            break;
        }

        log::info!("检测到游戏 {} 的 ddv20.exe 进程已退出", game_id);

        // 再次检查停止信号（防止在 wait_for_process_exit 刚返回时被设置）
        if stop_signal.load(Ordering::SeqCst) {
            log::info!("游戏 {} 在进程退出后收到停止信号", game_id);
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

        // 续传前检查停止信号
        if stop_signal.load(Ordering::SeqCst) {
            log::info!("游戏 {} 续传前收到停止信号", game_id);
            break;
        }

        // 重新启动 ddv20.exe 进行续传
        match service.start_game_download(
            &app_handle,
            &manifest_path,
            &download_path,
            &game_id,
        ) {
            Ok((result, child)) => {
                let mut processes = match DOWNLOAD_PROCESSES.lock() {
                    Ok(p) => p,
                    Err(e) => {
                        log::error!("游戏 {} 续传时锁获取失败: {}", game_id, e);
                        continue;
                    }
                };
                // 续传前检查停止信号（防止在 start_game_download 期间被设置）
                if stop_signal.load(Ordering::SeqCst) {
                    drop(processes);
                    // 进程已启动，需要 kill 掉
                    kill_process(&game_id);
                    log::info!("游戏 {} 续传后收到停止信号，已终止新进程", game_id);
                    break;
                }
                processes.insert(game_id.clone(), child);
                log::info!("游戏 {} 续传已启动 (PID: {:?})", game_id, result.process_id);
            }
            Err(e) => {
                log::error!("游戏 {} 续传启动失败: {}", game_id, e);
            }
        }
    }

    // 清理：移除该游戏的停止信号量
    let _ = DOWNLOAD_STOP_SIGNALS.lock()
        .map(|mut signals| { signals.remove(&game_id); });
    log::info!("下载监控任务已结束，游戏ID: {}", game_id);
}

/// 在 DOWNLOAD_PROCESSES 中查找并终止指定游戏的进程
/// 被监控线程内部调用，是唯一调用 kill() 的地方
/// 使用 taskkill /F /T 递归终止整个进程树，再调用 child.kill() 二次保障
fn kill_process(game_id: &str) {
    let mut processes = match DOWNLOAD_PROCESSES.lock() {
        Ok(p) => p,
        Err(e) => {
            log::error!("获取进程表锁失败: {}", e);
            return;
        }
    };
    if let Some(mut child) = processes.remove(game_id) {
        let pid = child.id();
        // 第一步：使用 taskkill /F /T 递归终止进程树（包括 ddv20.exe 可能启动的子进程）
        let taskkill_ok = kill_process_tree(pid);
        // 第二步：调用 child.kill() 作为后备
        let child_result = child.kill();
        log::info!(
            "终止游戏 {} 进程: PID={}, taskkill=/F /T, 成功={}, child.kill={:?}",
            game_id, pid, taskkill_ok, child_result
        );
    }
}

/// 递归终止指定 PID 的进程树
/// 使用 Windows taskkill /F /T（/T 表示递归终止子进程）
/// 使用 CREATE_NO_WINDOW 避免 taskkill 自身弹出控制台窗口
#[cfg(target_os = "windows")]
fn kill_process_tree(pid: u32) -> bool {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    match std::process::Command::new("taskkill")
        .args(&["/F", "/T", "/PID", &pid.to_string()])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        Ok(output) => output.status.success(),
        Err(e) => {
            log::warn!("调用 taskkill 失败: {}", e);
            false
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn kill_process_tree(_pid: u32) -> bool {
    false
}

/// 等待指定游戏的 ddv20.exe 进程退出
/// 每1秒轮询一次，同时检查停止信号量
/// 返回值：true = 被停止信号终止，false = 进程自然退出
fn wait_for_process_exit(game_id: &str, stop_signal: &AtomicBool) -> bool {
    loop {
        // 每次循环先检查停止信号
        if stop_signal.load(Ordering::SeqCst) {
            kill_process(game_id);
            return true;
        }

        thread::sleep(Duration::from_secs(1));

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
            return false;
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
/// 只设置停止信号量，由监控线程自行 kill 进程并清理
/// 监控线程每1秒检测信号，响应延迟不超过1秒
#[tauri::command]
pub async fn stop_download(app: AppHandle, game_id: String) -> Result<(), String> {
    // 设置停止信号量，监控线程检测到后会自动 kill 进程并退出
    {
        let signals = DOWNLOAD_STOP_SIGNALS.lock()
            .map_err(|e| format!("获取停止信号量表锁失败: {}", e))?;
        if let Some(signal) = signals.get(&game_id) {
            signal.store(true, Ordering::SeqCst);
            log::info!("已设置游戏 {} 的停止信号", game_id);
        } else {
            log::warn!("游戏 {} 没有找到对应的停止信号量，可能已经停止", game_id);
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
