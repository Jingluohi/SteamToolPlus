// 杂项配置模块
// 处理各种辅助配置：应用ID、群组、密钥、语言、Depot、分支、游戏协调器、
// 自定义广播、自动接受邀请、音效、头像、字体、HTTP响应、默认物品等

use std::path::Path;
use super::common::ConfigSaveResult;

// ============================================
// 已安装应用ID
// ============================================

/// 保存 installed_app_ids.txt
#[tauri::command]
pub async fn save_installed_app_ids(
    game_path: String,
    app_ids: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("installed_app_ids.txt");
    let content = app_ids.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 installed_app_ids.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 installed_app_ids.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "已安装应用ID已保存".to_string(),
    })
}

/// 加载 installed_app_ids.txt
#[tauri::command]
pub async fn load_installed_app_ids(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("installed_app_ids.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 installed_app_ids.txt 失败: {}", e))?;

    let app_ids: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(app_ids)
}

// ============================================
// 订阅群组
// ============================================

/// 保存 subscribed_groups.txt
#[tauri::command]
pub async fn save_subscribed_groups(
    game_path: String,
    group_ids: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("subscribed_groups.txt");
    let content = group_ids.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 subscribed_groups.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 subscribed_groups.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "订阅群组已保存".to_string(),
    })
}

/// 加载 subscribed_groups.txt
#[tauri::command]
pub async fn load_subscribed_groups(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("subscribed_groups.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 subscribed_groups.txt 失败: {}", e))?;

    let group_ids: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(group_ids)
}

// ============================================
// 订阅群组（公会）配置
// ============================================

/// 保存 subscribed_groups_clans.txt
#[tauri::command]
pub async fn save_subscribed_groups_clans(
    game_path: String,
    groups: Vec<serde_json::Value>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("subscribed_groups_clans.txt");
    let mut lines = vec![];

    for group in groups {
        let group_id = group.get("groupId").and_then(|v| v.as_str()).unwrap_or("");
        let name = group.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let tag = group.get("tag").and_then(|v| v.as_str()).unwrap_or("");

        if !group_id.is_empty() {
            lines.push(format!("{}\t\t{}\t\t{}", group_id, name, tag));
        }
    }

    let content = lines.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 subscribed_groups_clans.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 subscribed_groups_clans.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "订阅群组（公会）配置已保存".to_string(),
    })
}

/// 加载 subscribed_groups_clans.txt
#[tauri::command]
pub async fn load_subscribed_groups_clans(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("subscribed_groups_clans.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 subscribed_groups_clans.txt 失败: {}", e))?;

    let mut groups = vec![];
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').filter(|s| !s.is_empty()).collect();
        if parts.len() >= 3 {
            groups.push(serde_json::json!({
                "groupId": parts[0].trim(),
                "name": parts[1].trim(),
                "tag": parts[2].trim()
            }));
        } else if parts.len() == 1 {
            groups.push(serde_json::json!({
                "groupId": parts[0].trim(),
                "name": "",
                "tag": ""
            }));
        }
    }

    Ok(groups)
}

// ============================================
// 购买密钥
// ============================================

/// 保存 purchased_keys.txt
#[tauri::command]
pub async fn save_purchased_keys(
    game_path: String,
    keys: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("purchased_keys.txt");
    let content = keys.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 purchased_keys.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 purchased_keys.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "购买密钥已保存".to_string(),
    })
}

/// 加载 purchased_keys.txt
#[tauri::command]
pub async fn load_purchased_keys(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("purchased_keys.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 purchased_keys.txt 失败: {}", e))?;

    let keys: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(keys)
}

// ============================================
// 支持语言
// ============================================

/// 保存 supported_languages.txt
#[tauri::command]
pub async fn save_supported_languages(
    game_path: String,
    languages: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("supported_languages.txt");
    let content = languages.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 supported_languages.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 supported_languages.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "支持语言已保存".to_string(),
    })
}

/// 加载 supported_languages.txt
#[tauri::command]
pub async fn load_supported_languages(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("supported_languages.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 supported_languages.txt 失败: {}", e))?;

    let languages: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(languages)
}

// ============================================
// Depot 配置
// ============================================

/// 保存 depots.txt
#[tauri::command]
pub async fn save_depots(
    game_path: String,
    depots: Vec<serde_json::Value>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("depots.txt");
    let mut lines = vec![];

    for depot in depots {
        if let (Some(id), Some(manifest)) = (
            depot.get("depotId").and_then(|v| v.as_str()),
            depot.get("manifestId").and_then(|v| v.as_str())
        ) {
            lines.push(format!("{} {}", id, manifest));
        }
    }

    let content = lines.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 depots.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 depots.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "Depot 配置已保存".to_string(),
    })
}

/// 加载 depots.txt
#[tauri::command]
pub async fn load_depots(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("depots.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 depots.txt 失败: {}", e))?;

    let mut depots = vec![];
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            depots.push(serde_json::json!({
                "depotId": parts[0],
                "manifestId": parts[1]
            }));
        }
    }

    Ok(depots)
}

// ============================================
// 分支配置
// ============================================

/// 保存 branches.json
#[tauri::command]
pub async fn save_branches(
    game_path: String,
    branches: Vec<serde_json::Value>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("branches.json");
    let json = serde_json::to_string_pretty(&branches)
        .map_err(|e| format!("序列化 branches 失败: {}", e))?;

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 branches.json 失败: {}", e))?;
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| format!("写入 branches.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "分支配置已保存".to_string(),
    })
}

/// 加载 branches.json
#[tauri::command]
pub async fn load_branches(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("branches.json");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 branches.json 失败: {}", e))?;

    let branches: Vec<serde_json::Value> = serde_json::from_str(&content)
        .map_err(|e| format!("解析 branches.json 失败: {}", e))?;

    Ok(branches)
}

// ============================================
// 游戏协调器配置
// ============================================

/// 保存 gc.json (游戏协调器)
#[tauri::command]
pub async fn save_game_coordinator(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("gc.json");
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化 gc 配置失败: {}", e))?;

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 gc.json 失败: {}", e))?;
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| format!("写入 gc.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "游戏协调器配置已保存".to_string(),
    })
}

/// 加载 gc.json
#[tauri::command]
pub async fn load_game_coordinator(
    game_path: String,
) -> Result<serde_json::Value, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("gc.json");

    if !file_path.exists() {
        return Ok(serde_json::json!({}));
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 gc.json 失败: {}", e))?;

    let config: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 gc.json 失败: {}", e))?;

    Ok(config)
}

// ============================================
// 自定义广播
// ============================================

/// 保存自定义广播IP列表
#[allow(dead_code)]
#[tauri::command]
pub async fn save_custom_broadcasts(
    game_path: String,
    broadcasts: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("custom_broadcasts.txt");
    let content = broadcasts.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "自定义广播已保存".to_string(),
    })
}

/// 加载自定义广播IP列表
#[allow(dead_code)]
#[tauri::command]
pub async fn load_custom_broadcasts(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("custom_broadcasts.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 custom_broadcasts.txt 失败: {}", e))?;

    Ok(content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

// ============================================
// 自动接受邀请
// ============================================

/// 保存自动接受邀请列表
#[allow(dead_code)]
#[tauri::command]
pub async fn save_auto_accept_invite(
    game_path: String,
    invite_list: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("auto_accept_invite.txt");
    let content = if invite_list.contains(&"*".to_string()) {
        "*".to_string()
    } else {
        invite_list.join("\n")
    };

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 auto_accept_invite.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 auto_accept_invite.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "自动接受邀请配置已保存".to_string(),
    })
}

/// 加载自动接受邀请列表
#[allow(dead_code)]
#[tauri::command]
pub async fn load_auto_accept_invite(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("auto_accept_invite.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 auto_accept_invite.txt 失败: {}", e))?;

    Ok(content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

// ============================================
// 音效配置
// ============================================

/// 保存音效文件
#[tauri::command]
pub async fn save_sound_file(
    game_path: String,
    sound_type: String,
    file_data: Vec<u8>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let sounds_dir = Path::new(&game_path).join("steam_settings").join("sounds");
    fs::create_dir_all(&sounds_dir)
        .await
        .map_err(|e| format!("创建 sounds 目录失败: {}", e))?;

    let file_name = match sound_type.as_str() {
        "achievement" => "overlay_achievement_notification.wav",
        "friend" => "overlay_friend_notification.wav",
        "message" => "overlay_message_notification.wav",
        _ => return Err(format!("未知的音效类型: {}", sound_type)),
    };

    let file_path = sounds_dir.join(file_name);

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建音效文件失败: {}", e))?;
    file.write_all(&file_data)
        .await
        .map_err(|e| format!("写入音效文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: format!("{} 音效已保存", sound_type),
    })
}

/// 加载音效文件
#[tauri::command]
pub async fn load_sound_file(
    game_path: String,
    sound_type: String,
) -> Result<Option<Vec<u8>>, String> {
    use tokio::fs;

    let file_name = match sound_type.as_str() {
        "achievement" => "overlay_achievement_notification.wav",
        "friend" => "overlay_friend_notification.wav",
        "message" => "overlay_message_notification.wav",
        _ => return Err(format!("未知的音效类型: {}", sound_type)),
    };

    let file_path = Path::new(&game_path).join("steam_settings").join("sounds").join(file_name);

    if !file_path.exists() {
        return Ok(None);
    }

    let data = fs::read(&file_path)
        .await
        .map_err(|e| format!("读取音效文件失败: {}", e))?;

    Ok(Some(data))
}

/// 检查音效文件是否存在
#[tauri::command]
pub async fn check_sound_file_exists(
    game_path: String,
    sound_type: String,
) -> Result<bool, String> {
    let file_name = match sound_type.as_str() {
        "achievement" => "overlay_achievement_notification.wav",
        "friend" => "overlay_friend_notification.wav",
        "message" => "overlay_message_notification.wav",
        _ => return Err(format!("未知的音效类型: {}", sound_type)),
    };

    let file_path = Path::new(&game_path).join("steam_settings").join("sounds").join(file_name);
    Ok(file_path.exists())
}

// ============================================
// 头像配置
// ============================================

/// 保存头像文件
#[tauri::command]
pub async fn save_avatar(
    game_path: String,
    avatar_type: String,
    file_data: Vec<u8>,
    extension: String,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_name = match avatar_type.as_str() {
        "account" => format!("account_avatar.{}", extension),
        "default" => format!("account_avatar_default.{}", extension),
        _ => return Err(format!("未知的头像类型: {}", avatar_type)),
    };

    let file_path = steam_settings_dir.join(&file_name);

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建头像文件失败: {}", e))?;
    file.write_all(&file_data)
        .await
        .map_err(|e| format!("写入头像文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "头像已保存".to_string(),
    })
}

/// 加载头像文件
#[tauri::command]
pub async fn load_avatar(
    game_path: String,
    avatar_type: String,
) -> Result<Option<(Vec<u8>, String)>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");

    let base_name = match avatar_type.as_str() {
        "account" => "account_avatar",
        "default" => "account_avatar_default",
        _ => return Err(format!("未知的头像类型: {}", avatar_type)),
    };

    for ext in ["png", "jpg", "jpeg"] {
        let file_path = steam_settings_dir.join(format!("{}.{}", base_name, ext));
        if file_path.exists() {
            let data = fs::read(&file_path)
                .await
                .map_err(|e| format!("读取头像文件失败: {}", e))?;
            return Ok(Some((data, ext.to_string())));
        }
    }

    Ok(None)
}

/// 检查头像文件是否存在
#[tauri::command]
pub async fn check_avatar_exists(
    game_path: String,
    avatar_type: String,
) -> Result<bool, String> {
    let steam_settings_dir = Path::new(&game_path).join("steam_settings");

    let base_name = match avatar_type.as_str() {
        "account" => "account_avatar",
        "default" => "account_avatar_default",
        _ => return Err(format!("未知的头像类型: {}", avatar_type)),
    };

    for ext in ["png", "jpg", "jpeg"] {
        let file_path = steam_settings_dir.join(format!("{}.{}", base_name, ext));
        if file_path.exists() {
            return Ok(true);
        }
    }

    Ok(false)
}

// ============================================
// 字体配置
// ============================================

/// 保存字体文件
#[tauri::command]
pub async fn save_font_file(
    game_path: String,
    font_name: String,
    file_data: Vec<u8>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let fonts_dir = Path::new(&game_path).join("steam_settings").join("fonts");
    fs::create_dir_all(&fonts_dir)
        .await
        .map_err(|e| format!("创建 fonts 目录失败: {}", e))?;

    let file_path = fonts_dir.join(&font_name);

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建字体文件失败: {}", e))?;
    file.write_all(&file_data)
        .await
        .map_err(|e| format!("写入字体文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "字体文件已保存".to_string(),
    })
}

/// 加载字体文件
#[tauri::command]
pub async fn load_font_file(
    game_path: String,
    font_name: String,
) -> Result<Option<Vec<u8>>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("fonts").join(&font_name);

    if !file_path.exists() {
        return Ok(None);
    }

    let data = fs::read(&file_path)
        .await
        .map_err(|e| format!("读取字体文件失败: {}", e))?;

    Ok(Some(data))
}

/// 列出所有字体文件
#[tauri::command]
pub async fn list_font_files(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let fonts_dir = Path::new(&game_path).join("steam_settings").join("fonts");

    if !fonts_dir.exists() {
        return Ok(vec![]);
    }

    let mut files = vec![];
    let mut entries = fs::read_dir(&fonts_dir)
        .await
        .map_err(|e| format!("读取 fonts 目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("读取条目失败: {}", e))? {
        let path = entry.path();
        if path.is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".ttf") || name.ends_with(".otf") {
                files.push(name);
            }
        }
    }

    Ok(files)
}

// ============================================
// SteamHTTP 配置
// ============================================

/// 保存 SteamHTTP 响应文件
#[tauri::command]
pub async fn save_steam_http_response(
    game_path: String,
    domain: String,
    path: String,
    content: String,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let http_dir = Path::new(&game_path).join("steam_settings").join("http").join(&domain);
    fs::create_dir_all(&http_dir)
        .await
        .map_err(|e| format!("创建 http 目录失败: {}", e))?;

    let file_path = http_dir.join(&path);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("创建父目录失败: {}", e))?;
    }

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 HTTP 响应文件失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 HTTP 响应文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "HTTP 响应已保存".to_string(),
    })
}

/// 加载 SteamHTTP 响应文件
#[tauri::command]
pub async fn load_steam_http_response(
    game_path: String,
    domain: String,
    path: String,
) -> Result<Option<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path)
        .join("steam_settings")
        .join("http")
        .join(&domain)
        .join(&path);

    if !file_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 HTTP 响应文件失败: {}", e))?;

    Ok(Some(content))
}

/// 列出所有 SteamHTTP 配置
#[tauri::command]
pub async fn list_steam_http_configs(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let http_dir = Path::new(&game_path).join("steam_settings").join("http");

    if !http_dir.exists() {
        return Ok(vec![]);
    }

    let mut configs = vec![];
    let mut entries = fs::read_dir(&http_dir)
        .await
        .map_err(|e| format!("读取 http 目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("读取条目失败: {}", e))? {
        let domain = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        if path.is_dir() {
            // 使用 spawn_blocking 来执行同步的递归函数
            let paths = tokio::task::spawn_blocking(move || list_http_paths(&path))
                .await
                .map_err(|e| format!("列出 HTTP 路径失败: {:?}", e))??;
            configs.push(serde_json::json!({
                "domain": domain,
                "paths": paths
            }));
        }
    }

    Ok(configs)
}

/// 递归列出 HTTP 目录下的所有文件路径（同步版本，用于 spawn_blocking）
fn list_http_paths(dir: &Path) -> Result<Vec<String>, String> {
    use std::fs;

    let mut paths = vec![];

    let entries = fs::read_dir(dir)
        .map_err(|e| format!("读取目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
        let path = entry.path();
        if path.is_file() {
            paths.push(entry.file_name().to_string_lossy().to_string());
        } else if path.is_dir() {
            let sub_paths = list_http_paths(&path)?;
            for sub_path in sub_paths {
                paths.push(format!("{}/{}", entry.file_name().to_string_lossy(), sub_path));
            }
        }
    }

    Ok(paths)
}

// ============================================
// 默认物品配置
// ============================================

/// 保存 default_items.json
#[tauri::command]
pub async fn save_default_items(
    game_path: String,
    items: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("default_items.json");
    let json = serde_json::to_string_pretty(&items)
        .map_err(|e| format!("序列化 default_items 失败: {}", e))?;

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 default_items.json 失败: {}", e))?;
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| format!("写入 default_items.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "默认物品配置已保存".to_string(),
    })
}

/// 加载 default_items.json
#[tauri::command]
pub async fn load_default_items(
    game_path: String,
) -> Result<serde_json::Value, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("default_items.json");

    if !file_path.exists() {
        return Ok(serde_json::json!({}));
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 default_items.json 失败: {}", e))?;

    let items: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 default_items.json 失败: {}", e))?;

    Ok(items)
}

// ============================================
// DLC 配置
// ============================================

/// 保存 DLC 配置
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

    // 更新 DLC 配置
    if let Some(unlock_all) = config.get("unlockAll").and_then(|v| v.as_bool()) {
        app_config.dlcs.unlock_all = unlock_all;
    }

    if let Some(dlcs) = config.get("individualDlcs").and_then(|v| v.as_array()) {
        app_config.dlcs.individual_dlcs = dlcs
            .iter()
            .filter_map(|v| {
                Some(IndividualDlc {
                    app_id: v.get("appId")?.as_str()?.to_string(),
                    name: v.get("name")?.as_str()?.to_string(),
                    enabled: v.get("enabled")?.as_bool()?,
                })
            })
            .collect();
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
        "exists": broadcasts_path.exists() || auto_accept_path.exists(),
        "enabled": true,
        "customBroadcasts": custom_broadcasts,
        "autoAcceptInvite": auto_accept,
        "whitelist": whitelist,
        "listenPort": 47584
    }))
}
