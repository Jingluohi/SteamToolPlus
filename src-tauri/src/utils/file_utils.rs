// 文件工具
// 提供文件读写相关的通用函数

use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::Path;

/// 读取JSON文件并反序列化
pub fn read_json_file<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("解析JSON失败: {}", e))
}

/// 序列化并写入JSON文件
pub fn write_json_file<T: Serialize>(path: &str, data: &T) -> Result<(), String> {
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

/// 确保目录存在
pub fn ensure_dir(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| format!("创建目录失败: {}", e))
    } else {
        Ok(())
    }
}

/// 复制文件
pub fn copy_file(src: &str, dst: &str) -> Result<(), String> {
    // 确保目标目录存在
    if let Some(parent) = Path::new(dst).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    fs::copy(src, dst).map_err(|e| format!("复制文件失败: {}", e))?;
    Ok(())
}

/// 删除文件
pub fn delete_file(path: &str) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| format!("删除文件失败: {}", e))
}

/// 删除目录及其内容
pub fn delete_dir(path: &str) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| format!("删除目录失败: {}", e))
}

/// 读取文本文件
pub fn read_text_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 写入文本文件
pub fn write_text_file(path: &str, content: &str) -> Result<(), String> {
    // 确保父目录存在
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    fs::write(path, content).map_err(|e| format!("写入文件失败: {}", e))
}
