// audio_visual.rs - sound_file、avatar、font_file 配置

use super::common::*;
use std::path::Path;

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