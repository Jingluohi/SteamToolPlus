// 日志工具
// 提供日志记录功能，支持JSON格式

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use serde_json::json;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// 获取日志目录路径（程序根目录下的logs文件夹）
fn get_log_dir() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    
    exe_dir.join("logs")
}

/// 写入日志到JSON文件
pub fn log_to_json(level: LogLevel, message: &str, source: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    
    let log_entry = json!({
        "timestamp": timestamp,
        "level": level.as_str(),
        "source": source,
        "message": message
    });

    // 确保日志目录存在
    let log_dir = get_log_dir();
    let _ = std::fs::create_dir_all(&log_dir);

    // 按日期分文件
    let date = chrono::Local::now().format("%Y-%m-%d");
    let log_file = log_dir.join(format!("{}.json", date));

    // 追加写入JSON文件
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        Ok(f) => f,
        Err(_) => return,
    };

    // 如果是新文件，写入JSON数组开头
    let metadata = match std::fs::metadata(&log_file) {
        Ok(m) => m,
        Err(_) => return,
    };

    if metadata.len() == 0 {
        let _ = file.write_all(b"[\n");
    } else {
        // 替换最后的 ] 为 ,
        // 简单处理：直接追加，不处理格式
    }

    let log_line = format!("  {},\n", log_entry.to_string());
    let _ = file.write_all(log_line.as_bytes());
}

/// 写入日志到文本文件（兼容旧格式）
pub fn log_to_file(level: LogLevel, message: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_line = format!("[{}] [{}] {}\n", timestamp, level.as_str(), message);

    // 确保日志目录存在
    let log_dir = get_log_dir();
    let _ = std::fs::create_dir_all(&log_dir);

    // 按日期分文件
    let date = chrono::Local::now().format("%Y-%m-%d");
    let log_file = log_dir.join(format!("{}.log", date));

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        let _ = file.write_all(log_line.as_bytes());
    }
}

/// 记录调试日志
#[allow(dead_code)]
pub fn debug(message: &str) {
    log_to_file(LogLevel::Debug, message);
    log_to_json(LogLevel::Debug, message, "rust");
}

/// 记录信息日志
#[allow(dead_code)]
pub fn info(message: &str) {
    log_to_file(LogLevel::Info, message);
    log_to_json(LogLevel::Info, message, "rust");
}

/// 记录警告日志
#[allow(dead_code)]
pub fn warn(message: &str) {
    log_to_file(LogLevel::Warn, message);
    log_to_json(LogLevel::Warn, message, "rust");
}

/// 记录错误日志
#[allow(dead_code)]
pub fn error(message: &str) {
    log_to_file(LogLevel::Error, message);
    log_to_json(LogLevel::Error, message, "rust");
}
