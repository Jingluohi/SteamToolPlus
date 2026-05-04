// 窗口数据模型
// 定义窗口管理相关的数据结构

use serde::{Deserialize, Serialize};

/// 窗口状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct WindowState {
    /// 窗口宽度
    pub width: u32,
    /// 窗口高度
    pub height: u32,
    /// 窗口位置X
    pub x: i32,
    /// 窗口位置Y
    pub y: i32,
    /// 是否最大化
    pub maximized: bool,
    /// 是否全屏
    pub fullscreen: bool,
}

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
