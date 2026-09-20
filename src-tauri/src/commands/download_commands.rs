// 下载命令（双模式）
//
// 模式分流：
//   1. ddv20 模式（默认，稳定）——启动外部 ddv20.exe 进程下载：
//      - 启动：创建控制台窗口运行 ddv20.exe（-lu China --use-http -o 目标 app -p 清单目录）；
//      - 监控：后台线程轮询 %APPDATA%/SteamToolPlus/log/{game_id}/ 下的
//        "{百分比}% - {depotId}.json" 进度文件，把变化转换成
//        depot-download-progress 事件推送给前端（ddv20 只有百分比，
//        无字节/速度/当前文件，前端在 ddv20 模式下隐藏速度与当前文件）；
//      - 结束：ddv20 退出后按 check_all_depots_completed 判定完成 → 落库 + 入库收尾
//        + depot-download-finished 事件；未完成则自动续传（最多 3 次）；
//      - 停止：stop_download 置停止信号 → 监控线程 taskkill /F /T 终结进程树。
//   2. engine 模式（全局设置开启「新下载引擎」，标注不稳定）——进程内 Rust 引擎：
//      进度经 depot-download-progress / depot-download-finished 事件驱动，保留完整
//      字节/速度/当前文件/断点续传能力。
//
// 两种模式共用前端「下载任务模块」：事件结构（gameId + payload）完全一致。

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use once_cell::sync::Lazy;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;

use crate::engine::engine::{
    cancel_download, get_status, start_download_with_progress, DownloadRequest, EngineParams,
    ProgressFn, CANCELLED,
};
use crate::engine::progress::{DepotProgress, STATUS_COMPLETED, STATUS_DOWNLOADING};
use crate::services::game_data_service;
use crate::services::{DownloadResult, DownloadService, DownloadServiceTrait};
use crate::utils::resource_utils::get_resource_dir;

/// 引擎结束事件 payload（两种模式共用）
#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DownloadFinishedEvent {
    game_id: String,
    success: bool,
    message: String,
}

/// 进度事件 payload（两种模式共用）
#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DownloadProgressEvent {
    game_id: String,
    progress: DepotProgress,
}

/// 全局下载进程管理表（ddv20 模式）
/// key: game_id, value: ddv20.exe 子进程句柄
static DOWNLOAD_PROCESSES: Lazy<Mutex<HashMap<String, std::process::Child>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 全局下载停止信号量表（ddv20 模式）
/// stop_download 设置信号 → 监控线程检测后自行 kill 进程并清理
static DOWNLOAD_STOP_SIGNALS: Lazy<Mutex<HashMap<String, Arc<AtomicBool>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 每游戏当前使用的下载模式：game_id → true=engine / false=ddv20
/// 供 stop_download / get_download_status 分流（start_game_download 时登记）
static DOWNLOAD_MODES: Lazy<Mutex<HashMap<String, bool>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// ddv20 进度缓存：game_id → depot_id → 上次上报的百分比。
/// 只在上报值发生变化时才发进度事件，避免每轮轮询都刷屏
static DDV20_PROGRESS_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, u32>>>> =
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

/// 获取下载进度文件列表（ddv20 模式；供调试/外部调用，前端主链路走事件）
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

/// 检查并清理已完成的下载（ddv20 模式）
/// 当游戏的所有 depot 都下载完成后，自动静默删除对应的进度 JSON 文件
#[tauri::command]
pub fn check_and_cleanup_completed_downloads(app: AppHandle, game_id: Option<String>) -> Result<(), String> {
    let service = DownloadService::new();
    service.check_and_cleanup_completed_downloads(&app, game_id.as_deref())
}

/// 启动游戏下载（按模式分流）
///
/// `use_engine`：true=进程内 Rust 新引擎；false/缺省=外部 ddv20.exe（默认，稳定）。
/// `verify`：engine 模式为校验完整性（verify=true 强制文件级哈希检查）；
///           ddv20 模式忽略（ddv20 重跑清单即校验缺失/损坏文件补下）。
#[tauri::command]
pub async fn start_game_download(
    app: AppHandle,
    manifest_path: String,
    download_path: String,
    game_id: String,
    verify: Option<bool>,
    use_engine: Option<bool>,
) -> Result<DownloadResult, String> {
    let use_engine = use_engine.unwrap_or(false);

    // 登记模式，供 stop_download / get_download_status 分流
    if let Ok(mut modes) = DOWNLOAD_MODES.lock() {
        modes.insert(game_id.clone(), use_engine);
    }

    if use_engine {
        start_engine_download(app, manifest_path, download_path, game_id, verify).await
    } else {
        start_ddv20_download(app, manifest_path, download_path, game_id)
    }
}

/// ==================== engine 模式（进程内 Rust 引擎） ====================

/// 启动引擎下载：直接交给进程内引擎（无外部进程、无控制台窗口）。
/// 进度经 depot-download-progress 实时推送；整体结束（成功/取消/失败）后
/// 在后台上执行状态落库与入库收尾，并发出 depot-download-finished。
async fn start_engine_download(
    app: AppHandle,
    manifest_path: String,
    download_path: String,
    game_id: String,
    verify: Option<bool>,
) -> Result<DownloadResult, String> {
    // 1. 先取消同游戏的旧任务：让旧引擎尽快让位（引擎内部同时会替换任务登记）
    let _ = cancel_download(&game_id).await;

    // 2. 从全局设置读取引擎可调参数（仅新引擎生效；修改后下次下载生效，不热切换）。
    //    缺省/异常值回落到推荐默认值（EngineParams::default），保证下载始终可跑。
    let params = {
        let cfg = app.state::<crate::AppState>().config_service.get_config();
        let launch = &cfg.launch;
        let default_p = EngineParams::default();
        EngineParams {
            chunk_concurrency: if launch.chunk_concurrency > 0 {
                launch.chunk_concurrency
            } else {
                default_p.chunk_concurrency
            },
            global_concurrency: if launch.global_concurrency > 0 {
                launch.global_concurrency
            } else {
                default_p.global_concurrency
            },
            concurrent_depots: if launch.concurrent_depots > 0 {
                launch.concurrent_depots
            } else {
                default_p.concurrent_depots
            },
            file_handle_cache: if launch.file_handle_cache > 0 {
                launch.file_handle_cache
            } else {
                default_p.file_handle_cache
            },
        }
    };

    // 3. 构造引擎请求（manifest/target 为绝对路径时引擎原样使用，
    //    相对路径按程序运行目录解析）verify=true（校验完整性）
    let request = DownloadRequest {
        game_id: game_id.clone(),
        manifest_dir: manifest_path,
        target_dir: download_path,
        verify: verify.unwrap_or(false),
        params,
    };

    // 4. 进度回调 → Tauri.emit 推送前端（事件驱动）
    let emit_app = app.clone();
    let emit_game_id = game_id.clone();
    let progress: ProgressFn = Arc::new(move |p: DepotProgress| {
        let _ = emit_app.emit(
            "depot-download-progress",
            DownloadProgressEvent {
                game_id: emit_game_id.clone(),
                progress: p,
            },
        );
    });

    // 5. 后台原生任务跑引擎（不阻塞 invoke）
    let engine_app = app.clone();
    let engine_game_id = game_id.clone();
    let finished_app = app.clone();
    let finished_game_id = engine_game_id.clone();
    tauri::async_runtime::spawn(async move {
        let result = start_download_with_progress(request, progress, None).await;
        let outcome: (bool, String) = match result {
            Ok(()) => {
                let _ = game_data_service::update_download_status(
                    engine_app.clone(),
                    engine_game_id.clone(),
                    "completed".to_string(),
                    100,
                )
                .await;
                let _ = game_data_service::finalize_download(
                    engine_app.clone(),
                    engine_game_id.clone(),
                )
                .await;
                log::info!("游戏 {} 下载完成", engine_game_id);
                (true, "下载完成".to_string())
            }
            Err(e) if e.contains(CANCELLED) => {
                log::info!("游戏 {} 下载被用户暂停", engine_game_id);
                if let Ok(Some(game)) = game_data_service::get_game(
                    engine_app.clone(),
                    engine_game_id.clone(),
                )
                .await
                {
                    let _ = game_data_service::update_download_status(
                        engine_app.clone(),
                        engine_game_id.clone(),
                        "paused".to_string(),
                        game.download_progress,
                    )
                    .await;
                }
                (false, "下载已暂停".to_string())
            }
            Err(e) => {
                log::error!("游戏 {} 下载失败: {}", engine_game_id, e);
                let _ = game_data_service::update_download_status(
                    engine_app.clone(),
                    engine_game_id.clone(),
                    "error".to_string(),
                    0,
                )
                .await;
                (false, e)
            }
        };

        let _ = finished_app.emit(
            "depot-download-finished",
            DownloadFinishedEvent {
                game_id: finished_game_id.clone(),
                success: outcome.0,
                message: outcome.1,
            },
        );
    });

    Ok(DownloadResult {
        success: true,
        message: "下载已启动（新引擎）".to_string(),
        process_id: None,
    })
}

/// ==================== ddv20 模式（外部进程，默认稳定） ====================

/// 启动 ddv20.exe 外部下载并挂载监控线程。
/// 若该游戏已有旧进程，先发停止信号让旧监控线程自清理（避免竞争进程句柄）。
fn start_ddv20_download(
    app: AppHandle,
    manifest_path: String,
    download_path: String,
    game_id: String,
) -> Result<DownloadResult, String> {
    let service = DownloadService::new();

    // 如果该游戏已有正在运行的下载进程，通过停止信号量通知旧监控线程
    {
        let mut old_signals = DOWNLOAD_STOP_SIGNALS
            .lock()
            .map_err(|e| format!("获取停止信号量表锁失败: {}", e))?;
        if let Some(signal) = old_signals.get(&game_id) {
            signal.store(true, Ordering::SeqCst);
            log::info!("游戏 {} 存在旧下载进程，已发送停止信号", game_id);
        }
        old_signals.remove(&game_id);
    }

    let start_result =
        service.start_game_download(&app, &manifest_path, &download_path, &game_id);

    match start_result {
        Ok((result, child)) => {
            let process_id = result.process_id;

            // 创建新的停止信号量（初始 false），监控线程持有该信号的克隆
            let stop_signal = Arc::new(AtomicBool::new(false));

            {
                let mut processes = DOWNLOAD_PROCESSES
                    .lock()
                    .map_err(|e| format!("锁获取失败: {}", e))?;
                let mut signals = DOWNLOAD_STOP_SIGNALS
                    .lock()
                    .map_err(|e| format!("信号量表锁获取失败: {}", e))?;
                processes.insert(game_id.clone(), child);
                signals.insert(game_id.clone(), stop_signal.clone());
            }

            // 更新游戏状态为 downloading
            let app_clone = app.clone();
            let game_id_clone = game_id.clone();
            tauri::async_runtime::spawn(async move {
                let _ = game_data_service::update_download_status(
                    app_clone,
                    game_id_clone,
                    "downloading".to_string(),
                    0,
                )
                .await;
            });

            // 启动后台监控任务
            let app_handle = app.clone();
            let manifest_path_clone = manifest_path.clone();
            let download_path_clone = download_path.clone();
            let game_id_clone = game_id.clone();

            thread::spawn(move || {
                log::info!(
                    "启动 ddv20 下载监控任务，游戏ID: {}, PID: {:?}",
                    game_id_clone,
                    process_id
                );
                monitor_ddv20_process(
                    app_handle,
                    game_id_clone,
                    manifest_path_clone,
                    download_path_clone,
                    stop_signal,
                );
            });

            Ok(result)
        }
        Err(e) => Err(e),
    }
}

/// 轮询 ddv20 进度 JSON 并推送变化事件
/// 只读 log/{game_id}/ 下的 "{百分比}% - {depotId}.json"；
/// 仅百分比发生变化（或首次出现）时 emit，避免每轮刷屏。
fn poll_and_emit_ddv20_progress(app: &AppHandle, game_id: &str) {
    let service = DownloadService::new();
    let Ok(files) = service.get_download_progress_files(Some(game_id)) else {
        return;
    };
    if files.is_empty() {
        return;
    }

    let mut cache = match DDV20_PROGRESS_CACHE.lock() {
        Ok(c) => c,
        Err(_) => return,
    };
    let entry = cache.entry(game_id.to_string()).or_default();

    for f in &files {
        let last = entry.get(&f.depot_id).copied();
        if last == Some(f.progress) {
            continue; // 无变化，跳过
        }
        entry.insert(f.depot_id.clone(), f.progress);

        // ddv20 只有百分比：字节/当前文件均不提供（置 0/空，前端在该模式隐藏速度与文件）
        let completed = f.progress >= 100;
        let p = DepotProgress {
            depot_id: f.depot_id.parse::<u32>().unwrap_or(0),
            done_bytes: 0,
            total_bytes: 0,
            percent: f.progress as f32,
            current_file: String::new(),
            status: if completed {
                STATUS_COMPLETED.to_string()
            } else {
                STATUS_DOWNLOADING.to_string()
            },
            message: if completed {
                "depot 下载完成".to_string()
            } else {
                String::new()
            },
        };
        let _ = app.emit(
            "depot-download-progress",
            DownloadProgressEvent {
                game_id: game_id.to_string(),
                progress: p,
            },
        );
    }
}

/// 监控 ddv20 下载进程：轮询进度 → 进程退出判定 → 完成/失败/自动续传
fn monitor_ddv20_process(
    app_handle: AppHandle,
    game_id: String,
    manifest_path: String,
    download_path: String,
    stop_signal: Arc<AtomicBool>,
) {
    let service = DownloadService::new();
    let retry_count = Arc::new(std::sync::atomic::AtomicU32::new(0));
    const MAX_RETRIES: u32 = 3;

    // 创建一个 tokio 运行时，整个监控期间复用（async 落库/入库收尾）
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

        // 等待该游戏对应的 ddv20.exe 进程退出或被停止信号终止。
        // 等待期间反复轮询 log 目录进度文件并推送变化事件。
        let stopped_by_user = wait_process_exit_with_progress(&app_handle, &game_id, &stop_signal);

        if stopped_by_user {
            log::info!("游戏 {} 被用户主动停止，不再自动续传", game_id);
            remove_stop_signal(&game_id);
            break;
        }

        log::info!("检测到游戏 {} 的 ddv20.exe 进程已退出", game_id);

        // 再次检查停止信号（防止在 wait 刚返回时被设置）
        if stop_signal.load(Ordering::SeqCst) {
            log::info!("游戏 {} 在进程退出后收到停止信号", game_id);
            remove_stop_signal(&game_id);
            break;
        }

        // 检查游戏是否全部下载完成
        let is_completed = rt
            .block_on(async { service.check_all_depots_completed(&app_handle, &game_id).unwrap_or(false) });

        if is_completed {
            rt.block_on(async {
                let _ = game_data_service::update_download_status(
                    app_handle.clone(),
                    game_id.clone(),
                    "completed".to_string(),
                    100,
                )
                .await;
                let _ = game_data_service::finalize_download(app_handle.clone(), game_id.clone())
                    .await;
            });
            log::info!("游戏 {} 所有 depot 下载完成", game_id);
            // 完成：清理进度 JSON（静默删除 100% 的进度文件，与旧版一致）
            let _ = service.check_and_cleanup_completed_downloads(&app_handle, Some(&game_id));
            // 结束事件（成功）：前端据此收尾（完成任务、弹完成弹窗）
            let _ = app_handle.emit(
                "depot-download-finished",
                DownloadFinishedEvent {
                    game_id: game_id.clone(),
                    success: true,
                    message: "下载完成".to_string(),
                },
            );
            break;
        }

        // 下载未完成，检查是否需要重试
        let current_retry = retry_count.fetch_add(1, Ordering::SeqCst);
        if current_retry >= MAX_RETRIES {
            rt.block_on(async {
                if let Ok(Some(game)) =
                    game_data_service::get_game(app_handle.clone(), game_id.clone()).await
                {
                    if game.download_status == "downloading" {
                        let _ = game_data_service::update_download_status(
                            app_handle.clone(),
                            game_id.clone(),
                            "error".to_string(),
                            game.download_progress,
                        )
                        .await;
                    }
                }
            });
            log::warn!("游戏 {} 达到最大重试次数，标记为错误状态", game_id);
            let _ = app_handle.emit(
                "depot-download-finished",
                DownloadFinishedEvent {
                    game_id: game_id.clone(),
                    success: false,
                    message: "下载失败（已达最大重试次数）".to_string(),
                },
            );
            break;
        }

        // 等待3秒后重新启动下载
        log::info!(
            "游戏 {} 将在3秒后自动续传 (重试 {}/{})",
            game_id,
            current_retry + 1,
            MAX_RETRIES
        );
        thread::sleep(Duration::from_secs(3));

        // 续传前检查停止信号
        if stop_signal.load(Ordering::SeqCst) {
            log::info!("游戏 {} 续传前收到停止信号", game_id);
            remove_stop_signal(&game_id);
            break;
        }

        // 重新启动 ddv20.exe 进行续传
        match service.start_game_download(&app_handle, &manifest_path, &download_path, &game_id) {
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
                    kill_process(&game_id);
                    log::info!("游戏 {} 续传后收到停止信号，已终止新进程", game_id);
                    remove_stop_signal(&game_id);
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

    // 清理
    remove_stop_signal(&game_id);
    let _ = DOWNLOAD_MODES.lock().map(|mut m| m.remove(&game_id));
    let _ = DDV20_PROGRESS_CACHE.lock().map(|mut c| c.remove(&game_id));
    log::info!("ddv20 下载监控任务已结束，游戏ID: {}", game_id);
}

/// 等待进程退出；等待期间每 1 秒轮询一次 ddv20 进度文件并推送变化事件。
/// 返回值：true = 被停止信号终止，false = 进程自然退出
fn wait_process_exit_with_progress(
    app: &AppHandle,
    game_id: &str,
    stop_signal: &AtomicBool,
) -> bool {
    loop {
        // 每次循环先检查停止信号
        if stop_signal.load(Ordering::SeqCst) {
            kill_process(game_id);
            return true;
        }

        // 轮询并推送 ddv20 进度（百分比变化才 emit）
        poll_and_emit_ddv20_progress(app, game_id);

        thread::sleep(Duration::from_secs(1));

        let mut exited = false;
        {
            let mut processes = match DOWNLOAD_PROCESSES.lock() {
                Ok(p) => p,
                Err(_) => continue,
            };
            match processes.get_mut(game_id) {
                Some(child) => match child.try_wait() {
                    Ok(Some(_s)) => {
                        processes.remove(game_id);
                        exited = true;
                    }
                    Ok(None) => {}
                    Err(_) => {
                        processes.remove(game_id);
                        exited = true;
                    }
                },
                None => {
                    // 进程表中不存在该游戏进程，视为已退出
                    exited = true;
                }
            }
        }
        if exited {
            return false;
        }
    }
}

/// 移除该游戏的停止信号量
fn remove_stop_signal(game_id: &str) {
    if let Ok(mut signals) = DOWNLOAD_STOP_SIGNALS.lock() {
        signals.remove(game_id);
    }
}

/// 在 DOWNLOAD_PROCESSES 中查找并终止指定游戏的进程
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
            game_id,
            pid,
            taskkill_ok,
            child_result
        );
    }
}

/// 递归终止指定 PID 的进程树（Windows taskkill /F /T）
#[cfg(target_os = "windows")]
fn kill_process_tree(pid: u32) -> bool {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    match std::process::Command::new("taskkill")
        .args(["/F", "/T", "/PID", &pid.to_string()])
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

/// ==================== 状态 / 停止（双模式分流） ====================

/// 获取下载引擎当前状态（快照）
/// - engine 模式：引擎实时快照；
/// - ddv20 模式：从 log/{game_id}/ 进度 JSON 组装（running 表示 ddv20 进程仍在跑）。
#[tauri::command]
pub async fn get_download_status(game_id: String) -> Result<crate::engine::progress::EngineStatus, String> {
    // 模式登记表优先；未登记（如程序重启后无任务）时按 ddv20 处理（无进程 → 未运行）
    let is_engine = DOWNLOAD_MODES
        .lock()
        .map(|m| m.get(&game_id).copied().unwrap_or(false))
        .unwrap_or(false);

    if is_engine {
        return get_status(&game_id).await;
    }

    // ddv20 模式：组装快照
    let service = DownloadService::new();
    let mut depots = Vec::new();
    if let Ok(files) = service.get_download_progress_files(Some(&game_id)) {
        for f in files {
            let completed = f.progress >= 100;
            depots.push(DepotProgress {
                depot_id: f.depot_id.parse::<u32>().unwrap_or(0),
                done_bytes: 0,
                total_bytes: 0,
                percent: f.progress as f32,
                current_file: String::new(),
                status: if completed {
                    STATUS_COMPLETED.to_string()
                } else {
                    STATUS_DOWNLOADING.to_string()
                },
                message: String::new(),
            });
        }
    }
    let running = DOWNLOAD_PROCESSES
        .lock()
        .map(|p| p.contains_key(&game_id))
        .unwrap_or(false);
    Ok(crate::engine::progress::EngineStatus {
        game_id,
        depots,
        running,
    })
}

/// 停止下载（按模式分流）
/// - engine 模式：触发引擎 CancellationToken；
/// - ddv20 模式：设置停止信号，监控线程 taskkill 进程树。
/// 两种模式都把游戏状态置为 paused（暂停，不视为失败；继续下载重新发起即可）。
#[tauri::command]
pub async fn stop_download(app: AppHandle, game_id: String) -> Result<(), String> {
    let is_engine = DOWNLOAD_MODES
        .lock()
        .map(|m| m.get(&game_id).copied().unwrap_or(false))
        .unwrap_or(false);

    if is_engine {
        match cancel_download(&game_id).await {
            Ok(()) => log::info!("已请求取消游戏 {} 的下载", game_id),
            Err(e) => log::info!("游戏 {} 停止时无运行任务: {}", game_id, e),
        }
    } else {
        // ddv20：只设置停止信号量，由监控线程自行 kill 进程并清理
        {
            let signals = DOWNLOAD_STOP_SIGNALS
                .lock()
                .map_err(|e| format!("获取停止信号量表锁失败: {}", e))?;
            if let Some(signal) = signals.get(&game_id) {
                signal.store(true, Ordering::SeqCst);
                log::info!("已设置游戏 {} 的停止信号", game_id);
            } else {
                log::warn!("游戏 {} 没有找到对应的停止信号量，可能已经停止", game_id);
            }
        }
    }

    // 将游戏状态设置为 paused（与前端"已暂停"语义一致）
    if let Ok(Some(game)) = game_data_service::get_game(app.clone(), game_id.clone()).await {
        let _ = game_data_service::update_download_status(
            app,
            game_id,
            "paused".to_string(),
            game.download_progress,
        )
        .await;
    }

    Ok(())
}