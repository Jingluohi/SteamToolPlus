// 文件工具
// 提供文件读写相关的通用函数

use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::Path;

// ============================================
// 路径安全验证
// ============================================

/// 验证游戏ID是否安全（仅允许纯数字）
pub fn is_game_id_safe(game_id: &str) -> bool {
    !game_id.is_empty() && game_id.chars().all(|c| c.is_ascii_digit())
}

// ============================================
// 文件操作函数
// ============================================

/// 读取JSON文件并反序列化
pub fn read_json_file<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("解析JSON失败: {}", e))
}

/// 序列化并写入JSON文件
pub fn write_json_file<T: Serialize + ?Sized>(path: &str, data: &T) -> Result<(), String> {
    // 确保父目录存在
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    let content = serde_json::to_string_pretty(data).map_err(|e| format!("序列化JSON失败: {}", e))?;
    fs::write(path, content).map_err(|e| format!("写入文件失败: {}", e))
}

/// 检查文件是否存在
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// 读取文本文件
pub fn read_text_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 异步读取JSON文件并反序列化
pub async fn read_json_file_async<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    use tokio::fs;

    let content = fs::read_to_string(path)
        .await
        .map_err(|e| format!("读取文件失败: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("解析JSON失败: {}", e))
}

/// 异步序列化并写入JSON文件
pub async fn write_json_file_async<T: Serialize>(path: &Path, data: &T) -> Result<(), String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    // 确保父目录存在
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    let content = serde_json::to_string_pretty(data).map_err(|e| format!("序列化JSON失败: {}", e))?;
    let mut file = fs::File::create(path)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}
