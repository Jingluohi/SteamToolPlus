// 日志命令模块
// 接收前端日志并写入日志文件

use crate::utils::log_utils::{LogLevel, log_to_file, log_to_json};

/// 接收前端日志并记录
/// 
/// # 参数
/// - `level`: 日志级别字符串 (DEBUG, INFO, WARN, ERROR)
/// - `message`: 日志消息
/// - `source`: 日志来源 (默认 'frontend')
#[tauri::command]
pub fn log_to_file_command(level: String, message: String, source: Option<String>) -> Result<(), String> {
    let log_level = match level.as_str() {
        "DEBUG" => LogLevel::Debug,
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warn,
        "ERROR" => LogLevel::Error,
        _ => LogLevel::Info,
    };

    let log_source = source.unwrap_or_else(|| "frontend".to_string());
    
    // 写入文本日志
    log_to_file(log_level, &format!("[{}] {}", log_source, message));
    
    // 写入JSON日志
    log_to_json(log_level, &message, &log_source);
    
    Ok(())
}