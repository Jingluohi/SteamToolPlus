// 窗口数据模型
// 定义窗口管理相关的数据结构

use serde::{Deserialize, Serialize};

/// 窗口操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowOperationResult {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}

/// 设置窗口大小请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetWindowSizeRequest {
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
}
