// 游戏功能配置模块
// 处理成就、统计、物品、模组、排行榜、控制器等游戏功能配置

use crate::models::steam_config::*;
use std::path::Path;
use super::common::{ConfigSaveResult, ConfigLoadResult, ImportResult, ExportResult, load_steam_settings_json, save_steam_settings_json};

// ============================================
// 成就配置
// ============================================

/// 保存成就配置
#[tauri::command]
pub async fn save_achievements_config(
    game_path: String,
    config: AchievementsConfig,
) -> Result<ConfigSaveResult, String> {
    save_steam_settings_json(&game_path, "achievements.json", &config).await?;

    Ok(ConfigSaveResult {
        success: true,
        message: "成就配置已保存".to_string(),
    })
}

/// 加载成就配置
#[tauri::command]
pub async fn load_achievements_config(
    game_path: String,
) -> Result<ConfigLoadResult<AchievementsConfig>, String> {
    let config = load_steam_settings_json(&game_path, "achievements.json").await?;

    Ok(ConfigLoadResult {
        exists: config.is_some(),
        config,
    })
}

/// 从文件导入成就配置
#[tauri::command]
pub async fn import_achievements_from_file(
    file_path: String,
) -> Result<ImportResult<Achievement>, String> {
    use tokio::fs;

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取文件失败: {}", e))?;

    // 尝试解析为成就配置
    let achievements: Vec<Achievement> = serde_json::from_str(&content)
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let count = achievements.len();

    Ok(ImportResult {
        success: true,
        data: achievements,
        message: format!("成功导入 {} 个成就", count),
    })
}

/// 导出成就配置
#[tauri::command]
pub async fn export_achievements_config(
    game_path: String,
) -> Result<ExportResult, String> {
    use tokio::fs;

    let achievements_path = Path::new(&game_path).join("steam_settings").join("achievements.json");

    if !achievements_path.exists() {
        return Ok(ExportResult {
            success: false,
            data: None,
            message: "成就配置文件不存在".to_string(),
        });
    }

    let content = fs::read_to_string(&achievements_path)
        .await
        .map_err(|e| format!("读取 achievements.json 失败: {}", e))?;

    Ok(ExportResult {
        success: true,
        data: Some(content),
        message: "导出成功".to_string(),
    })
}

// ============================================
// 统计配置
// ============================================

/// 保存统计配置
#[tauri::command]
pub async fn save_stats_config(
    game_path: String,
    config: StatsConfig,
) -> Result<ConfigSaveResult, String> {
    save_steam_settings_json(&game_path, "stats.json", &config).await?;

    Ok(ConfigSaveResult {
        success: true,
        message: "统计配置已保存".to_string(),
    })
}

/// 加载统计配置
#[tauri::command]
pub async fn load_stats_config(
    game_path: String,
) -> Result<ConfigLoadResult<StatsConfig>, String> {
    let config = load_steam_settings_json(&game_path, "stats.json").await?;

    Ok(ConfigLoadResult {
        exists: config.is_some(),
        config,
    })
}

// ============================================
// 物品配置
// ============================================

/// 保存物品配置
#[tauri::command]
pub async fn save_items_config(
    game_path: String,
    config: ItemsConfig,
) -> Result<ConfigSaveResult, String> {
    save_steam_settings_json(&game_path, "items.json", &config).await?;

    Ok(ConfigSaveResult {
        success: true,
        message: "物品配置已保存".to_string(),
    })
}

/// 加载物品配置
#[tauri::command]
pub async fn load_items_config(
    game_path: String,
) -> Result<ConfigLoadResult<ItemsConfig>, String> {
    let config = load_steam_settings_json(&game_path, "items.json").await?;

    Ok(ConfigLoadResult {
        exists: config.is_some(),
        config,
    })
}

// ============================================
// 模组配置
// ============================================

/// 保存模组配置
#[tauri::command]
pub async fn save_mods_config(
    game_path: String,
    config: ModsConfig,
) -> Result<ConfigSaveResult, String> {
    save_steam_settings_json(&game_path, "mods.json", &config).await?;

    Ok(ConfigSaveResult {
        success: true,
        message: "模组配置已保存".to_string(),
    })
}

/// 加载模组配置
#[tauri::command]
pub async fn load_mods_config(
    game_path: String,
) -> Result<ConfigLoadResult<ModsConfig>, String> {
    let config = load_steam_settings_json(&game_path, "mods.json").await?;

    Ok(ConfigLoadResult {
        exists: config.is_some(),
        config,
    })
}

// ============================================
// 排行榜配置
// ============================================

/// 保存排行榜配置
#[tauri::command]
pub async fn save_leaderboards_config(
    game_path: String,
    config: LeaderboardsConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let leaderboards_path = steam_settings_dir.join("leaderboards.txt");
    let leaderboards_content = config.to_txt();

    let mut file = fs::File::create(&leaderboards_path)
        .await
        .map_err(|e| format!("创建 leaderboards.txt 失败: {}", e))?;
    file.write_all(leaderboards_content.as_bytes())
        .await
        .map_err(|e| format!("写入 leaderboards.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "排行榜配置已保存".to_string(),
    })
}

/// 加载排行榜配置
#[tauri::command]
pub async fn load_leaderboards_config(
    game_path: String,
) -> Result<ConfigLoadResult<LeaderboardsConfig>, String> {
    use tokio::fs;

    let leaderboards_path = Path::new(&game_path).join("steam_settings").join("leaderboards.txt");

    if !leaderboards_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&leaderboards_path)
        .await
        .map_err(|e| format!("读取 leaderboards.txt 失败: {}", e))?;

    let config = parse_leaderboards_txt(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析 gbe_fork leaderboards.txt 内容
/// 格式: LEADERBOARD_NAME=sort method=display type
fn parse_leaderboards_txt(content: &str) -> Result<LeaderboardsConfig, String> {
    let mut config = LeaderboardsConfig::default_config();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // gbe_fork 格式: NAME=sort=display
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() >= 3 {
            config.leaderboards.push(Leaderboard {
                name: parts[0].trim().to_string(),
                display_name: parts[0].trim().to_string(), // gbe_fork 格式没有 display_name，使用 name 代替
                sort_method: parts[1].trim().to_string(),
                display_type: parts[2].trim().to_string(),
                entries: vec![],
            });
        }
    }

    Ok(config)
}

// ============================================
// 控制器配置
// ============================================

/// 保存控制器配置
#[tauri::command]
pub async fn save_controller_config(
    game_path: String,
    config: ControllerConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let controller_dir = Path::new(&game_path).join("steam_settings").join("controller");
    fs::create_dir_all(&controller_dir)
        .await
        .map_err(|e| format!("创建 controller 目录失败: {}", e))?;

    // 保存配置文件
    let config_path = Path::new(&game_path).join("steam_settings").join("configs.controller.ini");
    let config_content = format!(
        r#"[controller]
enabled = {}
type = {}

[controller::deadzone]
left_stick = {:.2}
right_stick = {:.2}
left_trigger = {:.2}
right_trigger = {:.2}

[controller::rumble]
enabled = {}
intensity = {:.2}

[controller::custom_glyphs]
enabled = {}
"#,
        config.enabled as i32,
        config.controller_type,
        config.deadzone.left_stick,
        config.deadzone.right_stick,
        config.deadzone.left_trigger,
        config.deadzone.right_trigger,
        config.rumble.enabled as i32,
        config.rumble.intensity,
        config.custom_glyphs.enabled as i32
    );

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 configs.controller.ini 失败: {}", e))?;
    file.write_all(config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.controller.ini 失败: {}", e))?;

    // 保存按键绑定
    let bindings_path = controller_dir.join("bindings.txt");
    let mut bindings_content = String::new();
    for binding in &config.bindings {
        bindings_content.push_str(&format!("{} = {} # {}\n",
            binding.action,
            binding.button,
            binding.description.as_deref().unwrap_or("")
        ));
    }

    let mut bindings_file = fs::File::create(&bindings_path)
        .await
        .map_err(|e| format!("创建 bindings.txt 失败: {}", e))?;
    bindings_file.write_all(bindings_content.as_bytes())
        .await
        .map_err(|e| format!("写入 bindings.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "控制器配置已保存".to_string(),
    })
}

/// 加载控制器配置
#[tauri::command]
pub async fn load_controller_config(
    game_path: String,
) -> Result<ConfigLoadResult<ControllerConfig>, String> {
    use tokio::fs;

    let config_path = Path::new(&game_path).join("steam_settings").join("configs.controller.ini");

    if !config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取 configs.controller.ini 失败: {}", e))?;

    let config = parse_controller_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析控制器 INI 内容
fn parse_controller_ini(content: &str) -> Result<ControllerConfig, String> {
    let mut config = ControllerConfig::default();
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
                "controller" => {
                    match key {
                        "enabled" => config.enabled = value == "1",
                        "type" => config.controller_type = value.to_string(),
                        _ => {}
                    }
                }
                "controller::deadzone" => {
                    match key {
                        "left_stick" => config.deadzone.left_stick = value.parse().unwrap_or(0.1),
                        "right_stick" => config.deadzone.right_stick = value.parse().unwrap_or(0.1),
                        "left_trigger" => config.deadzone.left_trigger = value.parse().unwrap_or(0.1),
                        "right_trigger" => config.deadzone.right_trigger = value.parse().unwrap_or(0.1),
                        _ => {}
                    }
                }
                "controller::rumble" => {
                    match key {
                        "enabled" => config.rumble.enabled = value == "1",
                        "intensity" => config.rumble.intensity = value.parse().unwrap_or(0.8),
                        _ => {}
                    }
                }
                "controller::custom_glyphs" => {
                    match key {
                        "enabled" => config.custom_glyphs.enabled = value == "1",
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}


