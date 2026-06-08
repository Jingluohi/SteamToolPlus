// 核心配置模块
// 处理 Overlay、Main、User、App 配置的保存、加载和解析

use crate::models::steam_config::*;
use std::path::Path;
use super::common::{ConfigSaveResult, ConfigLoadResult};

// ============================================
// Overlay 配置
// ============================================

/// 保存 Overlay 配置
#[tauri::command]
pub async fn save_overlay_config(
    game_path: String,
    config: OverlayConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let overlay_config_path = steam_settings_dir.join("configs.overlay.ini");
    let overlay_config_content = config.to_ini();

    let mut file = fs::File::create(&overlay_config_path)
        .await
        .map_err(|e| format!("创建 configs.overlay.ini 失败: {}", e))?;
    file.write_all(overlay_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.overlay.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "Overlay 配置已保存".to_string(),
    })
}

/// 加载 Overlay 配置
#[tauri::command]
pub async fn load_overlay_config(
    game_path: String,
) -> Result<ConfigLoadResult<OverlayConfig>, String> {
    use tokio::fs;

    let overlay_config_path = Path::new(&game_path).join("steam_settings").join("configs.overlay.ini");

    if !overlay_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&overlay_config_path)
        .await
        .map_err(|e| format!("读取 configs.overlay.ini 失败: {}", e))?;

    // 解析 INI 内容
    let config = parse_overlay_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析 Overlay INI 内容
fn parse_overlay_ini(content: &str) -> Result<OverlayConfig, String> {
    let mut config = OverlayConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "overlay" => {
                    match key {
                        "enable_experimental_overlay" => config.enable_experimental_overlay = value == "1" || value.to_lowercase() == "true",
                        "enabled" => config.enable_experimental_overlay = value == "1" || value.to_lowercase() == "true",
                        "hotkey" => config.hotkey = value.to_string(),
                        _ => {}
                    }
                }
                "overlay::notifications" => {
                    match key {
                        "achievement" => config.notifications.achievement = value == "1",
                        "friend" => config.notifications.friend = value == "1",
                        "message" => config.notifications.message = value == "1",
                        "duration" => config.notifications.duration = value.parse().unwrap_or(5),
                        "position" => config.notifications.position = value.to_string(),
                        _ => {}
                    }
                }
                "overlay::appearance" => {
                    match key {
                        "theme" => config.appearance.theme = value.to_string(),
                        "opacity" => config.appearance.opacity = value.parse().unwrap_or(0.95),
                        "scale" => config.appearance.scale = value.parse().unwrap_or(1.0),
                        "blur" => config.appearance.blur = value == "1",
                        _ => {}
                    }
                }
                "overlay::performance" => {
                    match key {
                        "hardware_acceleration" => config.performance.hardware_acceleration = value == "1",
                        "fps_limit" => config.performance.fps_limit = value.parse().unwrap_or(60),
                        "low_performance_mode" => config.performance.low_performance_mode = value == "1",
                        _ => {}
                    }
                }
                "overlay::features" => {
                    match key {
                        "achievements" => config.features.achievements = value == "1",
                        "friends" => config.features.friends = value == "1",
                        "chat" => config.features.chat = value == "1",
                        "browser" => config.features.browser = value == "1",
                        "settings" => config.features.settings = value == "1",
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 主配置
// ============================================

/// 保存主配置
#[tauri::command]
pub async fn save_main_config(
    game_path: String,
    config: MainConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let main_config_path = steam_settings_dir.join("configs.main.ini");
    let main_config_content = config.to_ini();

    let mut file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    file.write_all(main_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "主配置已保存".to_string(),
    })
}

/// 加载主配置
#[tauri::command]
pub async fn load_main_config(
    game_path: String,
) -> Result<ConfigLoadResult<MainConfig>, String> {
    use tokio::fs;

    let main_config_path = Path::new(&game_path).join("steam_settings").join("configs.main.ini");

    if !main_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&main_config_path)
        .await
        .map_err(|e| format!("读取 configs.main.ini 失败: {}", e))?;

    let config = parse_main_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析主配置 INI 内容
fn parse_main_ini(content: &str) -> Result<MainConfig, String> {
    let mut config = MainConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "main::general" => {
                    match key {
                        "new_app_ticket" => config.new_app_ticket = value == "1",
                        "gc_token" => config.gc_token = value == "1",
                        "block_unknown_clients" => config.block_unknown_clients = value == "1",
                        "steam_deck" => config.steam_deck = value == "1",
                        "enable_account_avatar" => config.enable_account_avatar = value == "1",
                        "enable_voice_chat" => config.enable_voice_chat = value == "1",
                        "immediate_gameserver_stats" => config.immediate_gameserver_stats = value == "1",
                        "matchmaking_server_list_actual_type" => config.matchmaking_server_list_actual_type = value == "1",
                        "matchmaking_server_details_via_source_query" => config.matchmaking_server_details_via_source_query = value == "1",
                        "crash_printer_location" => config.crash_printer_location = Some(value.to_string()),
                        _ => {}
                    }
                }
                "main::stats" => {
                    match key {
                        "disable_leaderboards_create_unknown" => config.disable_leaderboards_create_unknown = value == "1",
                        "allow_unknown_stats" => config.allow_unknown_stats = value == "1",
                        "stat_achievement_progress_functionality" => config.stat_achievement_progress_functionality = value == "1",
                        "save_only_higher_stat_achievement_progress" => config.save_only_higher_stat_achievement_progress = value == "1",
                        "paginated_achievements_icons" => config.paginated_achievements_icons = value.parse().unwrap_or(10),
                        "record_playtime" => config.record_playtime = value == "1",
                        _ => {}
                    }
                }
                "main::connectivity" => {
                    match key {
                        "disable_lan_only" => config.disable_lan_only = value == "1",
                        "disable_networking" => config.disable_networking = value == "1",
                        "listen_port" => config.listen_port = value.parse().unwrap_or(47584),
                        "offline" => config.offline = value == "1",
                        "disable_sharing_stats_with_gameserver" => config.disable_sharing_stats_with_gameserver = value == "1",
                        "disable_source_query" => config.disable_source_query = value == "1",
                        "share_leaderboards_over_network" => config.share_leaderboards_over_network = value == "1",
                        "disable_lobby_creation" => config.disable_lobby_creation = value == "1",
                        "download_steamhttp_requests" => config.download_steamhttp_requests = value == "1",
                        _ => {}
                    }
                }
                "main::misc" => {
                    match key {
                        "achievements_bypass" => config.achievements_bypass = value == "1",
                        "force_steamhttp_success" => config.force_steamhttp_success = value == "1",
                        "disable_steamoverlaygameid_env_var" => config.disable_steamoverlaygameid_env_var = value == "1",
                        "enable_steam_preowned_ids" => config.enable_steam_preowned_ids = value == "1",
                        "steam_game_stats_reports_dir" => config.steam_game_stats_reports_dir = Some(value.to_string()),
                        "free_weekend" => config.free_weekend = value == "1",
                        "use_32bit_inventory_item_ids" => config.use_32bit_inventory_item_ids = value == "1",
                        _ => {}
                    }
                }
                "extra_dlls" => {
                    if key.starts_with("dll") {
                        config.extra_dlls.push(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 用户配置
// ============================================

/// 保存用户配置
#[tauri::command]
pub async fn save_user_config(
    game_path: String,
    config: UserConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config_content = config.to_ini();

    let mut file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "用户配置已保存".to_string(),
    })
}

/// 加载用户配置
#[tauri::command]
pub async fn load_user_config(
    game_path: String,
) -> Result<ConfigLoadResult<UserConfig>, String> {
    use tokio::fs;

    let user_config_path = Path::new(&game_path).join("steam_settings").join("configs.user.ini");

    if !user_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&user_config_path)
        .await
        .map_err(|e| format!("读取 configs.user.ini 失败: {}", e))?;

    let config = parse_user_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析用户配置 INI 内容
fn parse_user_ini(content: &str) -> Result<UserConfig, String> {
    let mut config = UserConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            if current_section == "user" {
                match key {
                    "username" => config.username = value.to_string(),
                    "language" => config.language = value.to_string(),
                    "save_path" => config.save_path = value.to_string(),
                    "avatar_path" => config.avatar_path = Some(value.to_string()),
                    "use_default_avatar" => config.use_default_avatar = value == "1",
                    "saves_folder_name" => config.saves_folder_name = Some(value.to_string()),
                    "local_save_path" => config.local_save_path = Some(value.to_string()),
                    "ticket" => config.ticket = Some(value.to_string()),
                    _ => {}
                }
            }
        }
    }

    Ok(config)
}

// ============================================
// 应用配置
// ============================================

/// 保存应用配置
#[tauri::command]
pub async fn save_app_config(
    game_path: String,
    config: SteamAppConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let app_config_content = config.to_ini();

    let mut file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "应用配置已保存".to_string(),
    })
}

/// 加载应用配置
#[tauri::command]
pub async fn load_app_config(
    game_path: String,
) -> Result<ConfigLoadResult<SteamAppConfig>, String> {
    use tokio::fs;

    let app_config_path = Path::new(&game_path).join("steam_settings").join("configs.app.ini");

    if !app_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&app_config_path)
        .await
        .map_err(|e| format!("读取 configs.app.ini 失败: {}", e))?;

    let config = parse_app_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析应用配置 INI 内容（公开版本，供 DLC 配置模块使用）
pub fn parse_app_ini(content: &str) -> Result<SteamAppConfig, String> {
    parse_app_ini_inner(content)
}

/// 解析应用配置 INI 内容（内部实现）
fn parse_app_ini_inner(content: &str) -> Result<SteamAppConfig, String> {
    let mut config = SteamAppConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "app" => {
                    if key == "branch_name" {
                        config.branch_name = value.to_string();
                    }
                }
                "app::paths" => {
                    if !value.is_empty() && !value.starts_with('#') {
                        config.app_paths.insert(key.to_string(), value.to_string());
                    }
                }
                "app::dlcs" => {
                    if key == "unlock_all" {
                        config.dlcs.unlock_all = value == "1";
                    } else if key != "unlock_all" && !key.starts_with('#') {
                        // 解析单个 DLC 条目: "app_id = 1 # name"
                        let enabled = value.starts_with('1');
                        let name = if let Some(pos) = value.find('#') {
                            value[pos+1..].trim().to_string()
                        } else {
                            String::new()
                        };
                        config.dlcs.individual_dlcs.push(IndividualDlc {
                            app_id: key.to_string(),
                            name,
                            enabled,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}
