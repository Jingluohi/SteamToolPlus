// Denuvo 授权数据模型
// 定义 D 加密授权提取、备份、写回以及状态展示所需的数据结构

use serde::{Deserialize, Serialize};

/// D 加密授权状态枚举
/// 用于描述某个 AppID 在当前注册表中的授权情况
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DenuvoAuthStatus {
    /// 注册表中已存在 SteamID，视为已授权
    Authorized,
    /// 注册表中不存在 SteamID，视为未授权
    NotAuthorized,
    /// 注册表项本身不存在，需要至少启动一次游戏完成首次授权
    NeedLaunch,
}

/// 单个 D 加密授权条目
/// 对应注册表 HKEY_CURRENT_USER\Software\Valve\Steam\Apps\{app_id}\ 下的三个值
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DenuvoAuthEntry {
    /// Steam AppID（32 位无符号整数）
    pub app_id: u32,
    /// 用户自定义的游戏显示名称
    pub game_name: String,
    /// 授权 SteamID（64 位十进制字符串），例如 76561198xxxxxxxx
    pub steam_id: Option<String>,
    /// AppTicket 二进制数据的十六进制字符串表示
    pub app_ticket_hex: Option<String>,
    /// ETicket 二进制数据的十六进制字符串表示
    pub e_ticket_hex: Option<String>,
    /// 备份创建时间（RFC3339 格式）
    pub backup_time: Option<String>,
}

/// D 加密授权备份列表项
/// 用于前端列表展示，不包含完整的 ticket 数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DenuvoAuthListItem {
    /// Steam AppID
    pub app_id: u32,
    /// 用户自定义的游戏显示名称
    pub game_name: String,
    /// 是否存在 SteamID 备份
    pub has_steam_id: bool,
    /// 是否存在 AppTicket 备份
    pub has_app_ticket: bool,
    /// 是否存在 ETicket 备份
    pub has_e_ticket: bool,
    /// 备份创建时间（RFC3339 格式）
    pub backup_time: Option<String>,
}

/// 当前 Steam 活动用户信息
/// 读取自 HKEY_CURRENT_USER\Software\Valve\Steam\ActiveProcess
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveSteamUserInfo {
    /// 当前登录账号的 AccountID
    pub account_id: u32,
    /// 当前登录账号所在 Universe（如 Public、Beta）
    pub universe: String,
    /// 由 account_id 与 universe 组合出的 64 位 SteamID
    pub steam_id_64: String,
}

/// 假入库游戏的 D 加密授权状态（返回给前端）
/// 结合 fake_imported_service 的游戏信息与注册表授权状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DenuvoGameAuthStatus {
    /// 游戏 AppID
    pub app_id: u32,
    /// 游戏显示名称
    pub display_name: String,
    /// 封面图片路径（在配置中存在时提供）
    pub cover_path: Option<String>,
    /// 当前授权状态
    pub status: DenuvoAuthStatus,
    /// 注册表中记录的授权 SteamID（如果有）
    pub authorized_steam_id: Option<String>,
}
