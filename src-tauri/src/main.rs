// Steam Tool Plus - 程序入口
// 负责Tauri应用初始化、命令注册、服务初始化

// 在Windows上禁用控制台窗口（仅在需要时通过代码显式创建）
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod extensions;
mod models;
mod security;
mod services;
mod utils;

use commands::{config_commands, download_commands, extension_commands, game_commands, game_data_commands, help_commands, log_commands, patch_commands, window_commands};
use services::{ConfigService, ConfigServiceTrait, GameService, GameServiceTrait};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::Manager;
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

/// 显示主窗口
fn show_main_window(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        log::info!("主窗口已显示");
    }
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

            // 创建托盘菜单
            let open_item = MenuItem::with_id(app, "open", "打开", true, None::<&str>)?;
            let exit_item = MenuItem::with_id(app, "exit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &exit_item])?;

            // 创建托盘图标
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
            tray_icon.on_tray_icon_event(move |tray, event| {
                match event {
                    TrayIconEvent::DoubleClick { button: MouseButton::Left, .. } => {
                        // 双击托盘图标显示窗口
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                }
            });

            // 处理托盘菜单事件
            let app_handle = app.handle().clone();
            tray_icon.on_menu_event(move |tray, event| {
                match event.id().as_ref() {
                    "open" => {
                        // 点击"打开"菜单项，显示窗口
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "exit" => {
                        // 点击"退出"菜单项，彻底关闭程序
                        log::info!("用户通过托盘菜单退出程序");
                        tray.app_handle().exit(0);
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
                            let _ = window.hide();
                            log::info!("窗口已隐藏到托盘");

                            // 2秒后释放图片缓存
                            let window_clone = window.clone();
                            thread::spawn(move || {
                                thread::sleep(Duration::from_secs(2));
                                // 执行JavaScript清除图片缓存
                                let _ = window_clone.eval(r#"
                                    // 清除所有图片元素的src以释放内存
                                    document.querySelectorAll('img').forEach(img => {
                                        img.src = '';
                                    });
                                    // 清除背景图片缓存
                                    document.querySelectorAll('[style*="background-image"]').forEach(el => {
                                        el.style.backgroundImage = '';
                                    });
                                    // 强制垃圾回收提示（如果浏览器支持）
                                    if (window.gc) {
                                        window.gc();
                                    }
                                    console.log('图片缓存已释放');
                                "#);
                                log::info!("图片缓存已释放");
                            });
                        }
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
            game_commands::get_game_cover_image,
            game_commands::check_game_installed,
            game_commands::launch_game,
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
            // 扩展命令
            extension_commands::get_extensions,
            extension_commands::install_extension,
            extension_commands::uninstall_extension,
            extension_commands::toggle_extension,
            extension_commands::reload_extension,
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
            patch_commands::inject_patch,
            patch_commands::close_application,
            patch_commands::select_file,
            patch_commands::select_folder,
            patch_commands::open_external_link,
            // 免Steam补丁注入新命令
            patch_commands::apply_steam_patch_basic,
            patch_commands::unpack_game_exe,
            patch_commands::save_lan_multiplayer_config,
            patch_commands::save_overlay_config,
            patch_commands::save_achievements_config,
            patch_commands::save_items_config,
            patch_commands::save_controller_config,
            patch_commands::save_user_config,
            patch_commands::save_leaderboards_config,
            patch_commands::save_stats_config,
            patch_commands::save_dlc_config,
            patch_commands::save_main_config,
            patch_commands::load_main_config,
            patch_commands::load_lan_multiplayer_config,
            // 7z补丁应用命令
            patch_commands::apply_patch,
            patch_commands::apply_patch_from_file,
            // 补丁说明读取命令
            patch_commands::get_patch_readme,
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
        ])
        .run(tauri::generate_context!())
        .expect("应用程序启动失败");
}
