// dlc_network.rs - dlc_config、lan_multiplayer_config、other_config 配置

use super::common::*;
use std::path::Path;

// ============================================
// DLC 配置
// ============================================

/// 保存 DLC 配置
/// 接收前端简化格式: { unlockAll, dlcList, depotIds, dlcPaths }
#[tauri::command]
pub async fn save_dlc_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use crate::models::steam_config::*;
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    // 读取现有的 app 配置
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let mut app_config = if app_config_path.exists() {
        let content = fs::read_to_string(&app_config_path)
            .await
            .map_err(|e| format!("读取 configs.app.ini 失败: {}", e))?;
        super::config_core::parse_app_ini(&content)?
    } else {
        SteamAppConfig::default_config()
    };

    // 更新 DLC 配置 - 处理 unlockAll
    if let Some(unlock_all) = config.get("unlockAll").and_then(|v| v.as_bool()) {
        app_config.dlcs.unlock_all = unlock_all;
    }

    // 处理 dlcList (前端简化格式: 每行一个 ID 的字符串)
    if let Some(dlc_list_str) = config.get("dlcList").and_then(|v| v.as_str()) {
        // 清空之前的 individual_dlcs，用 dlcList 中的 ID 重新构建
        app_config.dlcs.individual_dlcs.clear();
        app_config.dlcs.dlc_list = dlc_list_str
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // 将 dlcList 转换为 individual_dlcs 格式（用于 to_ini 输出）
        for dlc_id in &app_config.dlcs.dlc_list {
            app_config.dlcs.individual_dlcs.push(IndividualDlc {
                app_id: dlc_id.clone(),
                name: format!("DLC {}", dlc_id),
                enabled: true,
            });
        }
    }

    // 处理 depotIds (每行一个 ID 的字符串)
    if let Some(depot_ids_str) = config.get("depotIds").and_then(|v| v.as_str()) {
        app_config.dlcs.depot_ids = depot_ids_str
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    // 处理 dlcPaths (格式: appid=相对路径，每行一个)
    if let Some(dlc_paths_str) = config.get("dlcPaths").and_then(|v| v.as_str()) {
        app_config.dlcs.dlc_paths.clear();
        for line in dlc_paths_str.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(eq_pos) = line.find('=') {
                let app_id = line[..eq_pos].trim().to_string();
                let path = line[eq_pos + 1..].trim().to_string();
                if !app_id.is_empty() && !path.is_empty() {
                    app_config.dlcs.dlc_paths.insert(app_id, path);
                }
            }
        }
    }

    // 保存配置
    let app_config_content = app_config.to_ini();
    let mut file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "DLC 配置已保存".to_string(),
    })
}

/// 局域网联机配置保存
#[tauri::command]
pub async fn save_lan_multiplayer_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    // 保存 configs.lan.ini（enabled 和 listen_port）
    let enabled = config.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let listen_port = config
        .get("listenPort")
        .and_then(|v| v.as_u64())
        .unwrap_or(47584) as u16;
    let lan_ini_path = steam_settings_dir.join("configs.lan.ini");
    let lan_ini_content = format!(
        "[lan::general]\nenabled = {}\nlisten_port = {}\n",
        enabled as i32, listen_port
    );
    let mut lan_file = fs::File::create(&lan_ini_path)
        .await
        .map_err(|e| format!("创建 configs.lan.ini 失败: {}", e))?;
    lan_file
        .write_all(lan_ini_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.lan.ini 失败: {}", e))?;

    // 保存 custom_broadcasts.txt
    if let Some(broadcasts) = config.get("customBroadcasts").and_then(|v| v.as_array()) {
        let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
        let content: Vec<String> = broadcasts
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .filter(|s| !s.is_empty())
            .collect();

        if !content.is_empty() {
            let mut file = fs::File::create(&broadcasts_path)
                .await
                .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
            file.write_all(content.join("\n").as_bytes())
                .await
                .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;
        }
    }

    // 保存 auto_accept_invite.txt
    if let Some(auto_accept) = config.get("autoAcceptInvite").and_then(|v| v.as_str()) {
        let auto_accept_path = steam_settings_dir.join("auto_accept_invite.txt");
        let content = match auto_accept {
            "all" => "*".to_string(),
            "whitelist" => {
                if let Some(whitelist) = config.get("whitelist").and_then(|v| v.as_array()) {
                    whitelist
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        };

        if !content.is_empty() {
            let mut file = fs::File::create(&auto_accept_path)
                .await
                .map_err(|e| format!("创建 auto_accept_invite.txt 失败: {}", e))?;
            file.write_all(content.as_bytes())
                .await
                .map_err(|e| format!("写入 auto_accept_invite.txt 失败: {}", e))?;
        }
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "配置已保存".to_string(),
    })
}

/// 加载局域网联机配置
#[tauri::command]
pub async fn load_lan_multiplayer_config(
    game_path: String,
) -> Result<serde_json::Value, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");

    // 读取 configs.lan.ini（enabled / listen_port）
    let lan_ini_path = steam_settings_dir.join("configs.lan.ini");
    let (mut enabled, mut listen_port) = (false, 47584u16);
    if lan_ini_path.exists() {
        let content = fs::read_to_string(&lan_ini_path)
            .await
            .map_err(|e| format!("读取 configs.lan.ini 失败: {}", e))?;
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('[') || line.is_empty() {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "enabled" => enabled = value == "1" || value.to_lowercase() == "true",
                    "listen_port" => listen_port = value.parse().unwrap_or(47584),
                    _ => {}
                }
            }
        }
    }

    // 读取 custom_broadcasts.txt
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let custom_broadcasts = if broadcasts_path.exists() {
        let content = fs::read_to_string(&broadcasts_path)
            .await
            .map_err(|e| format!("读取 custom_broadcasts.txt 失败: {}", e))?;
        content.lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    // 读取 auto_accept_invite.txt
    let auto_accept_path = steam_settings_dir.join("auto_accept_invite.txt");
    let (auto_accept, whitelist) = if auto_accept_path.exists() {
        let content = fs::read_to_string(&auto_accept_path)
            .await
            .map_err(|e| format!("读取 auto_accept_invite.txt 失败: {}", e))?;
        let lines: Vec<String> = content.lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if lines.is_empty() {
            ("none".to_string(), vec![])
        } else if lines.len() == 1 && lines[0] == "*" {
            ("all".to_string(), vec![])
        } else {
            ("whitelist".to_string(), lines)
        }
    } else {
        ("none".to_string(), vec![])
    };

    Ok(serde_json::json!({
        "exists": broadcasts_path.exists() || auto_accept_path.exists() || lan_ini_path.exists(),
        "enabled": enabled,
        "customBroadcasts": custom_broadcasts,
        "autoAcceptInvite": auto_accept,
        "whitelist": whitelist,
        "listenPort": listen_port
    }))
}

// ============================================
// 其他配置综合检查
// ============================================

/// 加载其他配置（仅检查存在性）
/// 检查 installed_app_ids.txt / subscribed_groups.txt / purchased_keys.txt / supported_languages.txt
#[tauri::command]
pub async fn load_other_config(game_path: String) -> Result<ConfigLoadResult<()>, String> {
    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    let exists = [
        steam_settings_dir.join("installed_app_ids.txt"),
        steam_settings_dir.join("subscribed_groups.txt"),
        steam_settings_dir.join("purchased_keys.txt"),
        steam_settings_dir.join("supported_languages.txt"),
    ]
    .iter()
    .any(|p| p.exists());

    Ok(ConfigLoadResult { exists, config: None })
}