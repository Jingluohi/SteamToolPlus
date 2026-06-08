// Steam Tool Plus - 程序入口
// 负责Tauri应用初始化、命令注册、服务初始化

// 在Windows上禁用控制台窗口（仅在需要时通过代码显式创建）
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;
mod utils;

use commands::{background_commands, config_commands, download_commands, game_commands, game_data_commands, help_commands, log_commands, manifest_commands, patch_commands, window_commands};
use services::{ConfigService, ConfigServiceTrait, GameService, GameServiceTrait};
use std::sync::Arc;
use tauri::{Manager, Emitter};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
use tauri::menu::{Menu, MenuItem};
use tauri::WindowEvent;

/// 应用程序状态结构体
/// 包含所有全局服务的引用
pub struct AppState {
    pub game_service: Arc<dyn GameServiceTrait>,
    pub config_service: Arc<dyn ConfigServiceTrait>,
}

/// 初始化应用程序状态
/// 创建所有服务的单例实例
fn initialize_app_state() -> AppState {
    let config_service: Arc<dyn ConfigServiceTrait> = Arc::new(ConfigService::new());
    let game_service = GameService::new(Arc::clone(&config_service));

    AppState {
        game_service: Arc::new(game_service) as Arc<dyn GameServiceTrait>,
        config_service: config_service as Arc<dyn ConfigServiceTrait>,
    }
}

/// 显示主窗口并刷新图片
/// 统一处理窗口显示逻辑，确保图片正确加载
fn show_main_window(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        // 延迟注入刷新脚本，确保窗口已完全显示
        let window_clone = window.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            // 注入刷新脚本：通知前端刷新所有图片
            let _ = window_clone.eval(r#"
                window.dispatchEvent(new CustomEvent('window-focused'));
            "#);
            log::info!("主窗口已显示并触发图片刷新");
        });
    }
}

/// 重启应用程序
#[tauri::command]
async fn restart_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 使用 tauri-plugin-process 的 restart 功能
    app_handle.restart();
}

/// 检查 ddv20.exe 进程是否正在运行（不显示终端窗口）
#[cfg(target_os = "windows")]
pub fn check_ddv20_running() -> bool {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("tasklist")
        .args(&["/FI", "IMAGENAME eq ddv20.exe", "/NH"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.contains("ddv20.exe");
    }

    false
}

#[cfg(not(target_os = "windows"))]
pub fn check_ddv20_running() -> bool {
    false
}

/// 重置所有正在下载的游戏状态
/// 在程序关闭时调用，将 download_status 从 "downloading" 重置为 "idle"
fn reset_downloading_games() -> Result<(), String> {
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
    
    // 找到所有正在下载的游戏并重置状态
    let mut has_changes = false;
    for (game_id, game) in collection.games.iter_mut() {
        if game.download_status == "downloading" {
            log::info!("重置游戏 {} 的下载状态", game_id);
            game.download_status = "idle".to_string();
            game.download_progress = 0;
            game.last_played = Some(chrono::Local::now().to_rfc3339());
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
        
        log::info!("已重置所有正在下载的游戏状态");
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    // 初始化日志
    env_logger::init();

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

            // 获取主窗口并设置窗口属性
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_decorations(false);

            // 获取配置服务检查是否需要启动时最小化到托盘
            let app_state = app.state::<AppState>();
            let config = app_state.config_service.get_config();
            let start_minimized = config.launch.start_minimized_to_tray;

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
                match event {
                    TrayIconEvent::DoubleClick { button: MouseButton::Left, .. } => {
                        // 双击托盘图标显示窗口
                        show_main_window(&app_handle_clone);
                    }
                    _ => {}
                }
            });

            // 处理托盘菜单事件
            tray_icon.on_menu_event(move |tray, event| {
                let app_handle = tray.app_handle();
                match event.id().as_ref() {
                    "open" => {
                        // 点击"打开主窗口"，显示窗口
                        show_main_window(&app_handle);
                    }
                    "browse" => {
                        // 跳转到浏览页面
                        show_main_window(&app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/'; }, 200);");
                        }
                    }
                    "library" => {
                        // 跳转到游戏库页面
                        show_main_window(&app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/library'; }, 200);");
                        }
                    }
                    "download" => {
                        // 跳转到下载页面
                        show_main_window(&app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/download'; }, 200);");
                        }
                    }
                    "patch" => {
                        // 跳转到免Steam补丁页面
                        show_main_window(&app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.eval("setTimeout(() => { window.location.href = '/patch'; }, 200);");
                        }
                    }
                    "manifest" => {
                        // 跳转到清单入库页面
                        show_main_window(&app_handle);
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

                            // 启动Steam
                            let steam_paths = [
                                "C:\\Program Files (x86)\\Steam\\steam.exe",
                                "C:\\Program Files\\Steam\\steam.exe",
                            ];
                            for path in &steam_paths {
                                if std::path::Path::new(path).exists() {
                                    let _ = Command::new(path)
                                        .creation_flags(CREATE_NO_WINDOW)
                                        .spawn();
                                    break;
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
                            // 先通知前端释放 Vue 数据（避免内存泄漏）
                            let _ = window.emit("window-blurred", ());

                            // 隐藏窗口
                            let _ = window.hide();
                            log::info!("窗口已隐藏到托盘");

                            // 立即注入 JS 清理 DOM 图片，释放 WebView2 GPU 进程显存
                            let window_clone = window.clone();
                            std::thread::spawn(move || {
                                // 等待窗口完全隐藏后再清理 DOM
                                std::thread::sleep(std::time::Duration::from_millis(300));
                                let _ = window_clone.eval(r#"
                                    (function() {
                                        // 清除所有 img 标签的图片，释放 GPU 显存
                                        document.querySelectorAll('img').forEach(function(img) {
                                            img.removeAttribute('src');
                                            img.removeAttribute('srcset');
                                            img.removeAttribute('data-src');
                                            img.removeAttribute('data-srcset');
                                        });
                                        // 清除所有背景图片
                                        document.querySelectorAll('[style*="background-image"]').forEach(function(el) {
                                            el.style.backgroundImage = '';
                                        });
                                        // 清除所有 CSS 变量中的图片 URL
                                        document.documentElement.style.setProperty('--bg-image-url', '');
                                        // 强制回流，确保浏览器释放资源
                                        document.body.offsetHeight;
                                    })();
                                "#);
                                log::info!("DOM 图片已清理，GPU 显存已释放");
                            });
                        }
                    }
                }
            });

            // 处理文件拖放事件
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::DragDrop(drag_event) = event {
                    match drag_event {
                        tauri::DragDropEvent::Drop { paths, .. } => {
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
                        _ => {}
                    }
                }
            });

            // 处理窗口焦点事件：窗口获得焦点时通知前端
            // 只在窗口获得焦点时通知前端刷新图片（从托盘恢复、Alt+Tab 切换回来等）
            // 不监听失去焦点，避免切换到其他应用时误触发图片释放
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::Focused(focused) = event {
                    if *focused {
                        // 窗口获得焦点（从托盘恢复、Alt+Tab 切换回来等）
                        let _ = window_clone.emit("window-focused", ());
                        log::info!("窗口获得焦点，已通知前端刷新背景");
                    }
                    // 不再监听失去焦点事件，避免误触发
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
            // 窗口命令
            window_commands::minimize_window,
            window_commands::maximize_window,
            window_commands::close_window,
            window_commands::toggle_fullscreen,
            window_commands::set_window_size,
            window_commands::open_help_window,
            window_commands::close_help_window,
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
            download_commands::start_game_download,
            download_commands::get_download_progress_files,
            download_commands::read_directory,
            download_commands::delete_file,
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
            patch_commands::save_custom_broadcasts,
            patch_commands::load_custom_broadcasts,
            patch_commands::save_auto_accept_invite,
            patch_commands::load_auto_accept_invite,
            // ColdClientLoader 和 lobby_connect 配置命令
            patch_commands::save_coldclient_config,
            patch_commands::load_coldclient_config,
            patch_commands::save_lobby_connect_config,
            patch_commands::load_lobby_connect_config,
            // 游戏数据管理命令
            game_data_commands::get_all_games_data,
            game_data_commands::get_game_data,
            game_data_commands::upsert_game_data,
            game_data_commands::remove_game_data,
            game_data_commands::update_game_download_status,
            game_data_commands::update_game_play_time,
            game_data_commands::check_game_exists,
            game_data_commands::import_custom_game,
            game_data_commands::launch_game_with_tracking,
            game_data_commands::update_game_data,
            game_data_commands::close_game_process,
            game_data_commands::toggle_game_favorite,
            game_data_commands::check_game_process_status,
            game_data_commands::delete_game_directory,
            game_data_commands::open_game_directory,
            // 帮助命令
            help_commands::read_readme_file,
            help_commands::check_readme_exists,
            help_commands::get_sponsor_image_base64,
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
            // 清单入库命令
            manifest_commands::scan_manifest_folder,
            manifest_commands::extract_archive,
            manifest_commands::import_manifest_to_steam,
            manifest_commands::restart_steam,
            manifest_commands::check_game_manifest_exists,
            manifest_commands::import_game_manifest_to_steam,
            manifest_commands::setup_manifest_import_first_time,
            manifest_commands::open_steamtools,
            manifest_commands::open_example_folder,
        ])
        .run(tauri::generate_context!())
        .expect("应用程序启动失败");
}
