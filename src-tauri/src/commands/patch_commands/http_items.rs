// http_items.rs - steam_http_response、default_items 配置

use super::common::*;
use std::path::Path;

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