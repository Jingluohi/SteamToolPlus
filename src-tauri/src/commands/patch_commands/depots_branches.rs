// depots_branches.rs - depots、branches、game_coordinator、custom_broadcasts、auto_accept_invite 配置

use super::common::*;
use std::path::Path;

// ============================================
// Depots 配置
// ============================================

/// 保存 depots.txt
/// gbe_fork 格式: 每行一个 depot ID
#[tauri::command]
pub async fn save_depots(
    game_path: String,
    depot_ids: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let depots_path = steam_settings_dir.join("depots.txt");

    if depot_ids.is_empty() {
        if depots_path.exists() {
            fs::remove_file(&depots_path)
                .await
                .map_err(|e| format!("删除 depots.txt 失败: {}", e))?;
        }
        return Ok(ConfigSaveResult {
            success: true,
            message: "Depots 配置已清空".to_string(),
        });
    }

    let content = depot_ids.join("\n") + "\n";
    let mut file = fs::File::create(&depots_path)
        .await
        .map_err(|e| format!("创建 depots.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 depots.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "Depots 配置已保存".to_string(),
    })
}

/// 加载 depots.txt
/// gbe_fork 格式: 每行一个 depot ID
#[tauri::command]
pub async fn load_depots(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("depots.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 depots.txt 失败: {}", e))?;

    let depot_ids: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect();

    Ok(depot_ids)
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