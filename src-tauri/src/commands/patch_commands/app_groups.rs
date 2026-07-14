// 应用ID、群组、密钥、语言配置模块
// 处理 installed_app_ids、subscribed_groups、purchased_keys、supported_languages

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