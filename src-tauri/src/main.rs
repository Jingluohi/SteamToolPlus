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
use std::thread;
use std::time::Duration;
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

/// 显示主窗口
fn show_main_window(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        log::info!("主窗口已显示");
    }
}

/// 重启应用程序
#[tauri::command]
async fn restart_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 使用 tauri-plugin-process 的 restart 功能
    app_handle.restart();
    Ok(())
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
            let _app_handle = app.handle().clone();
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
                                "#);
                                log::info!("图片缓存已释放");
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
            commands::tool_commands::get_lua_files_in_folder,
            commands::tool_commands::convert_lua_to_vdf,
            commands::tool_commands::get_vdf_files_in_folder,
            commands::tool_commands::convert_vdf_to_lua,
            commands::tool_commands::download_steam_cover,
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
            patch_commands::apply_steam_patch_basic,
            patch_commands::unpack_game_exe,
            // 配置保存命令
            patch_commands::save_main_config,
            patch_commands::save_user_config,
            patch_commands::save_overlay_config,
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
        ])
        .run(tauri::generate_context!())
        .expect("应用程序启动失败");
}
