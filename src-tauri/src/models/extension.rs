// 扩展数据模型
// 定义扩展系统相关的数据结构

use serde::{Deserialize, Serialize};

/// 扩展清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// 扩展唯一ID
    pub id: String,
    /// 扩展名称
    pub name: String,
    /// 版本号
    pub version: String,
    /// 作者
    pub author: String,
    /// 描述
    pub description: String,
    /// 入口文件
    pub main: String,
    /// 图标文件
    pub icon: Option<String>,
    /// 权限声明
    pub permissions: Vec<String>,
    /// 兼容的程序版本
    pub compatible_versions: Vec<String>,
    /// 扩展配置
    pub config: Option<serde_json::Value>,
}

/// 扩展信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    /// 扩展清单
    pub manifest: ExtensionManifest,
    /// 扩展路径
    pub path: String,
    /// 是否已启用
    pub enabled: bool,
    /// 安装时间
    pub install_time: String,
    /// 最后更新时间
    pub update_time: String,
    /// 加载状态
    pub load_status: ExtensionLoadStatus,
    /// 错误信息
    pub error: Option<String>,
}

/// 扩展加载状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionLoadStatus {
    /// 未加载
    Unloaded,
    /// 加载中
    Loading,
    /// 已加载
    Loaded,
    /// 加载失败
    Failed,
    /// 已禁用
    Disabled,
}

/// 扩展列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionListResponse {
    /// 扩展列表
    pub extensions: Vec<Extension>,
    /// 总数
    pub total: usize,
}

/// 安装扩展请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallExtensionRequest {
    /// 扩展包路径
    pub package_path: String,
}

/// 扩展权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionPermission {
    /// 权限ID
    pub id: String,
    /// 权限名称
    pub name: String,
    /// 权限描述
    pub description: String,
    /// 是否危险权限
    pub dangerous: bool,
    /// 是否已授予
    pub granted: bool,
}

/// 扩展API调用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionApiRequest {
    /// 扩展ID
    pub extension_id: String,
    /// API名称
    pub api: String,
    /// 参数
    pub params: serde_json::Value,
}

/// 扩展API调用响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionApiResponse {
    /// 是否成功
    pub success: bool,
    /// 返回数据
    pub data: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<String>,
}

/// 扩展事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionEvent {
    /// 事件类型
    pub event_type: String,
    /// 事件数据
    pub data: serde_json::Value,
    /// 发送者ID
    pub sender: String,
    /// 时间戳
    pub timestamp: String,
}
