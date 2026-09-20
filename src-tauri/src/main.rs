// Steam Tool Plus - 程序入口
// 负责Tauri应用初始化、命令注册、服务初始化

// 在Windows上禁用控制台窗口（仅在需要时通过代码显式创建）
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod engine;
mod models;
mod services;
mod update_manager;
mod utils;

use commands::{background_commands, cache_commands, config_commands, denuvo_commands, download_commands, extract_play_commands, fake_imported_commands, game_commands, game_data_commands, help_commands, log_commands, manifest_commands, opensteamtool_commands, patch_commands, window_commands};
use once_cell::sync::Lazy;
use services::{CacheService, CacheServiceTrait, ConfigService, ConfigServiceTrait, GameService, GameServiceTrait, opensteamtool_service};
use std::sync::{Arc, Mutex};
use tauri::{Manager, Emitter};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
use tauri::menu::{Menu, MenuItem};
use tauri::WindowEvent;
use update_manager::UpdateCheckResult;

/// 启动时更新检查结果
/// 在 setup 中异步执行检查，前端通过 get_pending_startup_update 命令获取结果
static PENDING_STARTUP_UPDATE: Lazy<Mutex<Option<UpdateCheckResult>>> =
    Lazy::new(|| Mutex::new(None));

/// 检查关键资源文件是否存在
/// 用于检测用户是否从压缩包中直接运行 exe 而未完整解压
/// 返回 Ok(()) 表示资源正常，Err(msg) 表示资源缺失
fn check_critical_resources() -> Result<(), String> {
    let exe_dir = utils::config_path_utils::get_exe_dir()
        .map_err(|e| format!("无法获取程序路径: {}", e))?;

    // 检查 resources 目录是否存在
    let resources_dir = exe_dir.join("resources");
    if !resources_dir.exists() || !resources_dir.is_dir() {
        return Err("未找到 resources 目录".to_string());
    }

    // 检查 games_config.json 是否存在
    let games_config_path = resources_dir.join("games_config.json");
    if !games_config_path.exists() {
        return Err("未找到 resources/games_config.json 配置文件".to_string());
    }

    Ok(())
}

/// 应用程序状态结构体
/// 包含所有全局服务的引用
pub struct AppState {
    pub game_service: Arc<dyn GameServiceTrait>,
    pub config_service: Arc<dyn ConfigServiceTrait>,
    pub cache_service: Arc<dyn CacheServiceTrait>,
}

/// 初始化应用程序状态
/// 创建所有服务的单例实例
fn initialize_app_state() -> AppState {
    let config_service: Arc<dyn ConfigServiceTrait> = Arc::new(ConfigService::new());
    let game_service = GameService::new(Arc::clone(&config_service));
    let cache_service: Arc<dyn CacheServiceTrait> = Arc::new(CacheService::new());

    AppState {
        game_service: Arc::new(game_service) as Arc<dyn GameServiceTrait>,
        config_service: config_service as Arc<dyn ConfigServiceTrait>,
        cache_service: cache_service as Arc<dyn CacheServiceTrait>,
    }
}

/// 显示主窗口并刷新图片
/// 统一处理窗口显示逻辑，确保图片正确加载
fn show_main_window(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        // 若渲染进程之前被 TrySuspend 暂停（隐藏到托盘时），先恢复
        crate::utils::webview2_memory::resume_renderer(app_handle);
        let _ = window.show();
        let _ = window.set_focus();
        // 直接向前端发送刷新事件，通知恢复背景与封面图片
        // 使用 Emitter 的 emit（前端 listen 可接收），替代原先无效的 DOM CustomEvent 注入
        let _ = window.emit("window-focused", ());
    }
}

/// 触发一次 GPU discardable 缓存回收（前端翻页时调用）
/// 短暂降低 WebView2 内存目标等级，让 Chromium 丢弃旧页纹理后恢复正常
#[tauri::command]
fn flush_gpu_cache(app: tauri::AppHandle) {
    crate::utils::webview2_memory::flush_gpu_cache(&app);
}

/// 结束 WebView2 GPU 进程，强制释放全部 GPU 显存（前端空闲超时/手动触发时调用）
/// GPU 进程退出后 Chromium 会自动重启，图片由前端自动重载，渲染进程状态不受影响
#[tauri::command]
fn kill_gpu_process() {
    crate::utils::gpu_process::kill_gpu_process();
}

/// 获取启动时预检查的更新结果
/// 前端在挂载时调用此命令，获取 setup 阶段已完成的更新检查结果
/// 这样更新弹窗可以在窗口刚显示时就弹出，无需等待网络请求
#[tauri::command]
async fn get_pending_startup_update() -> Result<Option<UpdateCheckResult>, String> {
    let result = PENDING_STARTUP_UPDATE
        .lock()
        .map_err(|e| format!("获取更新状态失败: {}", e))?
        .clone();
    Ok(result)
}

/// 重置启动时更新检查的缓存结果
/// 资源更新成功且磁盘 version.json 已写入新版本后调用：
/// 将 PENDING_STARTUP_UPDATE 置为「已完成、无更新」状态，并重新从磁盘读取本地资源版本。
///
/// 为什么需要：前端在资源更新完成后会执行 window.location.reload()，但 Rust 进程并未重启，
/// PENDING_STARTUP_UPDATE 仍是启动时基于旧 version.json 计算的「有更新」结果。
/// 若不重置，刷新页面后前端轮询又会拿到该旧缓存，反复弹出显示旧版本号的更新弹窗。
fn reset_pending_startup_update() {
    if let Ok(mut pending) = PENDING_STARTUP_UPDATE.lock() {
        // 一次性读取磁盘 version.json，同时取版本号与「公告」说明，避免重复读盘
        let local = update_manager::read_local_resource_version();
        *pending = Some(UpdateCheckResult {
            has_update: false,
            local_app_version: env!("CARGO_PKG_VERSION").to_string(),
            local_resource_version: local
                .as_ref()
                .map(|v| v.version.clone())
                .unwrap_or_default(),
            remote_resource_version: String::new(),
            remote_description: None,
            has_exe_update: false,
            patch_url: String::new(),
            // 磁盘最新说明即本次更新写入的说明，作为后续启动「公告板」的展示内容
            local_description: local.as_ref().and_then(|v| v.description.clone()),
        });
    }
}

/// 重启应用程序
#[tauri::command]
async fn restart_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 使用 tauri-plugin-process 的 restart 功能
    app_handle.restart();
}

/// 退出应用程序
#[tauri::command]
async fn exit_app(app_handle: tauri::AppHandle, exit_code: i32) {
    // 退出前强制将内存缓存中的游戏数据落盘，避免最后一次变更因后台线程延迟而丢失
    crate::services::game_data_service::flush_now();
    app_handle.exit(exit_code);
}

/// 重置所有正在下载的游戏状态为「暂停」
/// - 退出（托盘关闭）前调用：把 downloading → paused，记录"下载中被打断"；
/// - 启动时调用：把上次异常退出（含任务管理器强杀）遗留的 downloading 一并转
///   paused，前端据此在下载记录页展示"已暂停"并可断点续传。
fn reset_downloading_games() -> Result<(), String> {
    // 先将内存缓存中的最新游戏数据强制落盘，确保读取到的是最新状态
    crate::services::game_data_service::flush_now();

    use crate::services::game_data_service::GameDataCollection;
    use crate::utils::config_path_utils;
    use std::fs;
    
    // 游戏数据文件名
    const GAME_DATA_FILENAME: &str = "game.json";
    
    // 获取游戏数据文件路径
    let data_path = config_path_utils::get_runtime_config_path(GAME_DATA_FILENAME)?;
    
    // 如果文件不存在，直接返回
    if !data_path.exists() {
        return Ok(());
    }
    
    // 读取文件内容
    let content = fs::read_to_string(&data_path)
        .map_err(|e| format!("读取游戏数据文件失败: {}", e))?;
    
    // 解析JSON
    let mut collection: GameDataCollection = serde_json::from_str(&content)
        .map_err(|e| format!("解析游戏数据失败: {}", e))?;
    
    // 找到所有正在下载的游戏并标记为「暂停」（保留进度，供断点续传）
    let mut has_changes = false;
    for (game_id, game) in collection.games.iter_mut() {
        if game.download_status == "downloading" {
            log::info!("标记游戏 {} 的下载状态为暂停", game_id);
            game.download_status = "paused".to_string();
            // 保留 download_progress：暂停后可如上继续下载
            has_changes = true;
        }
    }
    
    // 如果有修改，保存文件
    if has_changes {
        collection.last_updated = chrono::Local::now().to_rfc3339();
        
        // 确保目录存在
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
        
        // 序列化为JSON
        let content = serde_json::to_string_pretty(&collection)
            .map_err(|e| format!("序列化游戏数据失败: {}", e))?;
        
        // 写入文件
        fs::write(&data_path, content)
            .map_err(|e| format!("写入游戏数据文件失败: {}", e))?;
        
        log::info!("已将异常中断的下载标记为暂停");
    }
    
    Ok(())
}

/// 初始化文件日志写入 `%APPDATA%/SteamToolPlus/download.log`。
///
/// 程序打包后无终端，env_logger 默认输出会全部丢失（此前引擎的"换服务器/超时/
/// 使用 CDN 主机"等排查信息无处可看）。改为写文件后：
/// - 所有 log::*（info 及以上；含引擎「开始使用 CDN 服务器」）进入该文件；
/// - 滚动保留：启动时把旧 download.log 重命名为 download.log.1（覆盖更旧），
///   避免单文件无限增长；
/// - 目录/文件不可用（如权限异常）时降级 stderr，不阻塞程序启动。
fn init_file_logger() {
    use env_logger::{Builder, Env, Target};

    // 目标目录 %APPDATA%/SteamToolPlus（与程序配置目录一致）
    let appdata = match utils::config_path_utils::get_appdata_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("日志目录不可用（{e}），回退 stderr");
            Builder::from_env(Env::default().default_filter_or("info")).init();
            return;
        }
    };
    if let Err(e) = std::fs::create_dir_all(&appdata) {
        eprintln!("创建日志目录失败（{e}），回退 stderr");
        Builder::from_env(Env::default().default_filter_or("info")).init();
        return;
    }

    // 滚动：上次文件 → download.log.1（先删更旧的一份）
    let path = appdata.join("download.log");
    let prev = appdata.join("download.log.1");
    if path.exists() {
        let _ = std::fs::remove_file(&prev);
        let _ = std::fs::rename(&path, &prev);
    }

    let file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("打开日志文件失败（{e}），回退 stderr");
            Builder::from_env(Env::default().default_filter_or("info")).init();
            return;
        }
    };

    Builder::from_env(Env::default().default_filter_or("info"))
        .target(Target::Pipe(Box::new(file)))
        // 自定义格式：时间用本地时区（用户所在时区，如北京时间 UTC+8），
        // 不使用默认的 UTC（Z 后缀）时间戳，方便按本地时间对照下载过程
        .format(|buf, record| {
            use std::io::Write;
            let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(
                buf,
                "[{ts} {:<5} {}] {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    // 初始化日志：写文件到 %APPDATA%/SteamToolPlus/download.log（无终端时也能排查，
    // 引擎进度/换服务器/超时等 log::* 均入此文件；保留最近 2 份滚动防无限增长）
    init_file_logger();

    // 在 Tauri 启动前处理实例冲突
    // 如果检测到同名进程但路径不同，请求旧进程退出并等待其结束
    update_manager::handle_instance_conflicts();

    // 清理 exe 自更新遗留的 .old 备份
    // 启动时旧进程已退出，此时可以安全删除 .old 文件
    if let Ok(exe_dir) = utils::config_path_utils::get_exe_dir() {
        if let Ok(entries) = std::fs::read_dir(&exe_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.to_lowercase().ends_with(".exe.old") {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }
    }

    // 初始化应用程序状态
    let app_state = initialize_app_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 单实例回调：当尝试启动第二个实例时，显示已存在的窗口
            log::info!("检测到重复启动请求，显示已存在的窗口");
            show_main_window(app);
        }))
        .manage(app_state)
        .setup(|app| {
            // 应用程序启动时的初始化逻辑
            log::info!("Steam Tool Plus 应用程序启动");

            // === 第一步：检查关键资源文件是否存在 ===
            // 防止用户从压缩包中直接运行 exe 而未完整解压
            if let Err(msg) = check_critical_resources() {
                log::error!("关键资源检查失败: {}", msg);
                use tauri_plugin_dialog::DialogExt;
                app.dialog()
                    .message(format!(
                        "无法找到必要的资源文件：{}\n\n\
                        请确保将压缩包内的所有文件完整解压到同一目录后再运行程序。\n\
                        不要直接从压缩包中运行 exe。",
                        msg
                    ))
                    .title("Steam Tool Plus - 资源文件缺失")
                    .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                    .blocking_show();
                app.handle().exit(1);
                return Ok(());
            }

            // 启动实例冲突监听器
            update_manager::spawn_instance_control_watcher(app.handle().clone());

            // 启动恢复：把上次异常退出（含任务管理器强杀）遗留的 downloading 标记为暂停，
            // 前端据此在下载记录页展示"已暂停"并可断点续传
            if let Err(e) = reset_downloading_games() {
                log::warn!("启动时标记异常下载为暂停失败: {}", e);
            }

            // 获取主窗口并设置窗口属性
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_decorations(false);

            // 获取配置服务检查是否需要启动时最小化到托盘
            let app_state = app.state::<AppState>();
            let config = app_state.config_service.get_config();
            let start_minimized = config.launch.start_minimized_to_tray;

            // === 第二步：异步执行更新检查（不阻塞窗口显示） ===
            // 结果存到全局变量，前端通过 get_pending_startup_update 命令轮询获取
            tauri::async_runtime::spawn(async move {
                log::info!("开始异步检查更新...");
                match update_manager::check_for_update().await {
                    Ok(result) if result.has_update => {
                        if let Ok(mut pending) = PENDING_STARTUP_UPDATE.lock() {
                            *pending = Some(result);
                        }
                        log::info!("检测到可用更新，前端轮询后会自动弹出提示");
                    }
                    Ok(result) => {
                        // 没有更新：直接存入完整检查结果（has_update=false 且包含本地更新说明），
                        // 前端据此判断是否需要展示「公告板」（无更新时展示上次更新的说明），
                        // 同时非 None 表示检查已完成，避免前端无限轮询
                        if let Ok(mut pending) = PENDING_STARTUP_UPDATE.lock() {
                            *pending = Some(result);
                        }
                        log::info!("当前已是最新版本");
                    }
                    Err(e) => {
                        log::warn!("更新检查失败: {}", e);
                        // 检查失败也存入空结果，防止前端一直轮询
                        if let Ok(mut pending) = PENDING_STARTUP_UPDATE.lock() {
                            *pending = Some(update_manager::UpdateCheckResult {
                                has_update: false,
                                local_app_version: String::new(),
                                local_resource_version: String::new(),
                                remote_resource_version: String::new(),
                                remote_description: None,
                                has_exe_update: false,
                                patch_url: String::new(),
                                local_description: None,
                            });
                        }
                    }
                }
            });

            // 创建托盘菜单 - 包含常用功能快捷入口
            let open_item = MenuItem::with_id(app, "open", "打开主窗口", true, None::<&str>)?;
            let browse_item = MenuItem::with_id(app, "browse", "浏览游戏", true, None::<&str>)?;
            let library_item = MenuItem::with_id(app, "library", "游戏库", true, None::<&str>)?;
            let download_item = MenuItem::with_id(app, "download", "游戏本体下载", true, None::<&str>)?;
            let patch_item = MenuItem::with_id(app, "patch", "免Steam补丁", true, None::<&str>)?;
            let manifest_item = MenuItem::with_id(app, "manifest", "清单入库", true, None::<&str>)?;
            let restart_steam_item = MenuItem::with_id(app, "restart_steam", "重启Steam", true, None::<&str>)?;
            let exit_item = MenuItem::with_id(app, "exit", "退出", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &open_item,
                    &browse_item,
                    &library_item,
                    &download_item,
                    &patch_item,
                    &manifest_item,
                    &restart_steam_item,
                    &exit_item,
                ],
            )?;

            // 创建托盘图标 - 使用应用默认图标（从编译嵌入的资源中加载）
            let tray_icon = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Steam Tool Plus")
                .menu(&menu)
                .show_menu_on_left_click(false)  // 左键单击不显示菜单，双击显示窗口
                .build(app)?;

            // 如果配置为启动时最小化到托盘，则隐藏窗口
            if start_minimized {
                let _ = window.hide();
            }

            // 处理托盘图标事件（双击）
            let app_handle_clone = app.handle().clone();
            tray_icon.on_tray_icon_event(move |_tray, event| {
                if let TrayIconEvent::DoubleClick { button: MouseButton::Left, .. } = event {
                    // 双击托盘图标显示窗口
                    show_main_window(&app_handle_clone);
                }
            });

            // 处理托盘菜单事件
            tray_icon.on_menu_event(move |tray, event| {
                let app_handle = tray.app_handle();
                match event.id().as_ref() {
                    "open" => {
                        // 点击"打开主窗口"，显示窗口
                        show_main_window(app_handle);
                    }
                    "browse" => {
                        // 跳转到浏览页面
                        show_main_window(app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/'; }, 200);");
                        }
                    }
                    "library" => {
                        // 跳转到游戏库页面
                        show_main_window(app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/library'; }, 200);");
                        }
                    }
                    "download" => {
                        // 跳转到下载页面
                        show_main_window(app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/download'; }, 200);");
                        }
                    }
                    "patch" => {
                        // 跳转到免Steam补丁页面
                        show_main_window(app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/patch'; }, 200);");
                        }
                    }
                    "manifest" => {
                        // 跳转到清单入库页面
                        show_main_window(app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/manifest-import'; }, 200);");
                        }
                    }
                    "restart_steam" => {
                        // 重启Steam客户端
                        #[cfg(target_os = "windows")]
                        {
                            use std::os::windows::process::CommandExt;
                            use std::process::Command;
                            const CREATE_NO_WINDOW: u32 = 0x08000000;

                            // 先关闭Steam
                            let _ = Command::new("taskkill")
                                .args(["/F", "/IM", "steam.exe"])
                                .creation_flags(CREATE_NO_WINDOW)
                                .output();

                            std::thread::sleep(std::time::Duration::from_secs(2));

                            // 启动Steam：优先使用配置/注册表检测到的路径，不再硬编码
                            match opensteamtool_service::detect_steam_path(None) {
                                Ok(steam_path) => {
                                    let steam_exe = std::path::Path::new(&steam_path).join("steam.exe");
                                    if steam_exe.exists() {
                                        let _ = Command::new(&steam_exe)
                                            .creation_flags(CREATE_NO_WINDOW)
                                            .spawn();
                                    } else {
                                        log::warn!("检测到 Steam 目录但找不到 steam.exe: {}", steam_exe.display());
                                    }
                                }
                                Err(e) => {
                                    log::warn!("托盘重启 Steam 失败，无法检测到 Steam 路径: {}", e);
                                }
                            }
                        }
                    }
                    "exit" => {
                        // 点击"退出"菜单项，彻底关闭程序
                        log::info!("用户通过托盘菜单退出程序");
                        
                        // 重置所有正在下载的游戏状态
                        if let Err(e) = reset_downloading_games() {
                            log::error!("重置下载状态失败: {}", e);
                        }
                        
                        app_handle.exit(0);
                    }
                    _ => {}
                }
            });

            // 处理窗口关闭事件
            let app_handle = app.handle().clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    // 获取配置
                    let app_state = app_handle.state::<AppState>();
                    let config = app_state.config_service.get_config();
                    let hide_to_tray = config.launch.hide_to_tray_on_close;

                    if hide_to_tray {
                        // 阻止默认关闭行为
                        api.prevent_close();

                        // 隐藏窗口
                        if let Some(window) = app_handle.get_webview_window("main") {
                            // 通知前端释放图片缓存与背景资源（前端已注册 window-blurred 处理）
                            let _ = window.emit("window-blurred", ());

                            // 隐藏窗口
                            let _ = window.hide();
                            log::info!("窗口已隐藏到托盘");

                            // 暂停渲染进程，彻底释放 GPU/渲染内存（恢复时在 show_main_window 中 Resume）
                            crate::utils::webview2_memory::suspend_renderer(&app_handle);

                            // 结束 WebView2 GPU 进程，强制归零 GPU 显存
                            // （Chromium 会在下次显示时自动重启 GPU 进程）
                            crate::utils::gpu_process::kill_gpu_process();
                        }
                    }
                }
            });

            // 处理文件拖放事件
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event {
                    // 过滤出Lua文件
                    let lua_files: Vec<String> = paths
                        .iter()
                        .filter(|p| {
                            p.extension()
                                .map(|e| e.to_string_lossy().to_lowercase() == "lua")
                                .unwrap_or(false)
                        })
                        .map(|p| p.to_string_lossy().to_string())
                        .collect();

                    if !lua_files.is_empty() {
                        // 发送事件给前端
                        let _ = window_clone.emit("lua-files-dropped", lua_files);
                    }

                    // 过滤出VDF文件
                    let vdf_files: Vec<String> = paths
                        .iter()
                        .filter(|p| {
                            p.extension()
                                .map(|e| e.to_string_lossy().to_lowercase() == "vdf")
                                .unwrap_or(false)
                        })
                        .map(|p| p.to_string_lossy().to_string())
                        .collect();

                    if !vdf_files.is_empty() {
                        // 发送事件给前端
                        let _ = window_clone.emit("vdf-files-dropped", vdf_files);
                    }
                }
            });

            Ok(())
        })
        // 注册游戏相关命令
        .invoke_handler(tauri::generate_handler![
            // 游戏库命令
            game_commands::get_games,
            game_commands::get_game_by_id,
            game_commands::add_game,
            game_commands::update_game,
            game_commands::delete_game,
            game_commands::launch_game,
            game_commands::scan_games_directory,
            game_commands::import_steam_games,
            // 游戏配置加载命令（从JSON文件）
            game_commands::load_games_config_from_file,
            game_commands::load_tested_games_config,
            game_commands::get_game_cover_image,
            game_commands::get_game_cover_bytes,
            game_commands::check_game_installed,
            game_commands::check_patch_file_exists,
            game_commands::get_game_cover_path,
            game_commands::get_game_library_image,
            // 日志命令
            log_commands::log_to_file_command,
            // 配置命令
            config_commands::get_config,
            config_commands::update_config,
            config_commands::reset_config,
            // 缓存命令
            cache_commands::add_imported_manifest_game_id,
            cache_commands::get_cache,
            cache_commands::clear_imported_manifest_cache,
            cache_commands::get_system_cache_size,
            cache_commands::clear_system_cache,
            cache_commands::get_download_log_cache_size,
            cache_commands::clear_download_log_cache,
            // 窗口命令
            window_commands::minimize_window,
            window_commands::maximize_window,
            window_commands::close_window,
            window_commands::toggle_fullscreen,
            window_commands::set_window_size,
            // 工具命令
            commands::tool_commands::read_file_content,
            commands::tool_commands::check_file_exists,
            commands::tool_commands::check_directory_exists,
            commands::tool_commands::get_lua_files_in_folder,
            commands::tool_commands::convert_lua_to_vdf,
            commands::tool_commands::get_vdf_files_in_folder,
            commands::tool_commands::convert_vdf_to_lua,
            commands::tool_commands::download_steam_cover,
            commands::tool_commands::scan_and_convert_manifest_for_download,
            // 下载命令
            download_commands::read_manifest_folder,
            download_commands::read_text_file,
            download_commands::read_json_file,
            download_commands::get_available_drive,
            download_commands::get_manifest_path,
            download_commands::get_download_progress_files,
            download_commands::start_game_download,
            download_commands::get_download_status,
            download_commands::read_directory,
            download_commands::delete_file,
            download_commands::delete_game_manifest_folder,
            download_commands::shutdown_system,
            download_commands::get_game_depots,
            download_commands::check_and_cleanup_completed_downloads,
            download_commands::stop_download,
            // 补丁命令
            patch_commands::get_resource_dir,
            patch_commands::path_exists,
            patch_commands::backup_game_exe,
            patch_commands::close_application,
            patch_commands::open_external_link,
            patch_commands::write_text_file,
            // 免Steam补丁注入新命令 - 100% 实现 gbe_fork 所有功能
            patch_commands::check_steam_dll_exists,
            patch_commands::apply_steam_patch_basic,
            patch_commands::restore_game_files,
            patch_commands::unpack_game_exe,
            // 配置保存命令
            patch_commands::save_main_config,
            patch_commands::save_user_config,
            patch_commands::save_overlay_config,
            patch_commands::save_app_config,
            patch_commands::save_achievements_config,
            patch_commands::save_stats_config,
            patch_commands::save_items_config,
            patch_commands::save_mods_config,
            patch_commands::save_leaderboards_config,
            patch_commands::save_controller_config,
            patch_commands::save_dlc_config,
            patch_commands::save_lan_multiplayer_config,
            // Patch 页面状态命令
            patch_commands::load_patch_state,
            patch_commands::save_patch_state,
            // 配置加载命令
            patch_commands::load_main_config,
            patch_commands::load_user_config,
            patch_commands::load_overlay_config,
            patch_commands::load_app_config,
            patch_commands::load_achievements_config,
            patch_commands::load_stats_config,
            patch_commands::load_items_config,
            patch_commands::load_mods_config,
            patch_commands::load_leaderboards_config,
            patch_commands::load_controller_config,
            patch_commands::load_lan_multiplayer_config,
            // 导入导出命令
            patch_commands::import_achievements_from_file,
            patch_commands::export_achievements_config,
            // 7z补丁应用命令
            patch_commands::apply_patch,
            patch_commands::apply_patch_from_file,
            // 补丁说明读取命令
            patch_commands::get_patch_readme,
            // 虚拟化环境配置教程
            patch_commands::open_virtualization_tutorial,
            // gbe_fork 额外配置文件命令
            patch_commands::save_installed_app_ids,
            patch_commands::load_installed_app_ids,
            patch_commands::save_subscribed_groups,
            patch_commands::load_subscribed_groups,
            patch_commands::save_subscribed_groups_clans,
            patch_commands::load_subscribed_groups_clans,
            patch_commands::save_purchased_keys,
            patch_commands::load_purchased_keys,
            patch_commands::save_supported_languages,
            patch_commands::load_supported_languages,
            patch_commands::save_depots,
            patch_commands::load_depots,
            patch_commands::save_branches,
            patch_commands::load_branches,
            patch_commands::save_game_coordinator,
            patch_commands::load_game_coordinator,
            patch_commands::save_default_items,
            patch_commands::load_default_items,
            patch_commands::save_sound_file,
            patch_commands::load_sound_file,
            patch_commands::check_sound_file_exists,
            patch_commands::save_avatar,
            patch_commands::load_avatar,
            patch_commands::check_avatar_exists,
            patch_commands::save_font_file,
            patch_commands::load_font_file,
            patch_commands::list_font_files,
            patch_commands::save_steam_http_response,
            patch_commands::load_steam_http_response,
            patch_commands::list_steam_http_configs,
            patch_commands::load_other_config,
            // ColdClientLoader 和 lobby_connect 配置命令
            patch_commands::save_coldclient_config,
            patch_commands::load_coldclient_config,
            patch_commands::save_lobby_connect_config,
            patch_commands::load_lobby_connect_config,
            // 游戏数据管理命令
            game_data_commands::get_all_games_data,
            game_data_commands::get_game_data,
            game_data_commands::get_download_history,
            game_data_commands::append_download_history,
            game_data_commands::remove_download_history,
            game_data_commands::upsert_game_data,
            game_data_commands::remove_game_data,
            game_data_commands::update_game_download_status,
            game_data_commands::update_game_play_time,
            game_data_commands::check_game_exists,
            game_data_commands::reset_missing_installed_games,
            game_data_commands::import_custom_game,
            game_data_commands::launch_game_with_tracking,
            game_data_commands::update_game_data,
            game_data_commands::close_game_process,
            game_data_commands::toggle_game_favorite,
            game_data_commands::finalize_game_download,
            game_data_commands::check_game_process_status,
            game_data_commands::delete_game_directory,
            game_data_commands::delete_game_log_cache,
            game_data_commands::open_game_directory,
            // 解压即玩图片命令
            extract_play_commands::get_extract_play_image,
            // 帮助命令
            help_commands::get_weixin_image_base64,
            help_commands::get_alipay_image_base64,
            help_commands::get_qingdan_image_base64,
            // 背景图片命令
            background_commands::get_background_config,
            background_commands::save_background_config,
            background_commands::add_background_file,
            background_commands::add_background_image,
            background_commands::remove_background_file,
            background_commands::remove_background_image,
            background_commands::scan_background_files,
            background_commands::scan_background_images,
            background_commands::reset_background_config,
            // 应用重启命令
            restart_app,
            exit_app,
            flush_gpu_cache,
            kill_gpu_process,
            update_manager::check_instance_restart_request,
            update_manager::get_app_versions,
            update_manager::check_for_update,
            update_manager::apply_update,
            // 启动时公告板命令（无更新时读取 resources/announcement/announcement.md）
            update_manager::get_announcement,
            // 启动时更新检查命令
            get_pending_startup_update,
            // 清单入库命令
            manifest_commands::scan_manifest_folder,
            manifest_commands::extract_archive,
            manifest_commands::extract_manifest_archive,
            manifest_commands::copy_folder_to_manifest,
            manifest_commands::restart_steam,
            manifest_commands::check_game_manifest_exists,
            // OpenSteamTool内核命令
            opensteamtool_commands::get_steam_path,
            opensteamtool_commands::detect_steam_path_auto,
            opensteamtool_commands::install_opensteamtool_kernel,
            opensteamtool_commands::uninstall_opensteamtool_kernel,
            opensteamtool_commands::check_opensteamtool_kernel_installed,
            opensteamtool_commands::get_opensteamtool_dll_info,
            opensteamtool_commands::import_with_opensteamtool_command,
            opensteamtool_commands::import_game_with_opensteamtool,
            opensteamtool_commands::import_manifest_with_opensteamtool,
            opensteamtool_commands::check_steam_running,
            opensteamtool_commands::generate_opensteamtool_config,
            opensteamtool_commands::clean_steamtools_residuals_command,
            opensteamtool_commands::validate_steam_path_command,
            // 假入库游戏管理命令
            fake_imported_commands::get_fake_imported_games_command,
            fake_imported_commands::add_fake_imported_game_command,
            fake_imported_commands::remove_fake_imported_game_command,
            // Denuvo 授权管理命令
            denuvo_commands::get_active_steam_user,
            denuvo_commands::read_denuvo_auth_from_registry,
            denuvo_commands::write_denuvo_auth_to_registry,
            denuvo_commands::check_denuvo_auth_status,
            denuvo_commands::get_fake_imported_games_denuvo_status,
            denuvo_commands::backup_denuvo_auth,
            denuvo_commands::list_denuvo_auth_backups,
            denuvo_commands::load_denuvo_auth_backup,
            denuvo_commands::save_denuvo_auth_entry,
            denuvo_commands::delete_denuvo_auth_backup,
            denuvo_commands::apply_denuvo_auth_backup,
        ])
        .build(tauri::generate_context!())
        .expect("应用程序启动失败")
        .run(|_app_handle, event| {
            // 程序事件循环结束（真正退出）时清理解压即玩图片的本地临时缓存
            if let tauri::RunEvent::Exit = event {
                if let Err(e) = extract_play_commands::clear_extract_play_cache() {
                    log::warn!("清理解压即玩图片缓存失败: {}", e);
                }
            }
        });
}
