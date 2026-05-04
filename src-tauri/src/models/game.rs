// 游戏数据模型
// 定义游戏相关的所有数据结构

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 游戏数据结构
/// 存储单个游戏的完整信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// 唯一ID，UUID生成
    pub id: String,
    /// 游戏名称
    pub name: String,
    /// 游戏exe文件路径
    pub exe_path: String,
    /// 封面图路径
    pub cover_path: Option<String>,
    /// 启动参数
    pub launch_params: String,
    /// 发行商
    pub publisher: String,
    /// 发行日期
    pub release_date: String,
    /// 标签/分类
    pub tags: Vec<String>,
    /// 是否安装
    pub is_installed: bool,
    /// 是否收藏
    pub is_favorite: bool,
    /// 总游玩时长，单位秒
    pub total_play_time: u64,
    /// 最近游玩时间
    pub last_play_time: Option<String>,
    /// 添加时间
    pub create_time: String,
}

impl Game {
    /// 创建新游戏实例
    pub fn new(name: String, exe_path: String) -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            exe_path,
            cover_path: None,
            launch_params: String::new(),
            publisher: String::new(),
            release_date: String::new(),
            tags: Vec::new(),
            is_installed: true,
            is_favorite: false,
            total_play_time: 0,
            last_play_time: None,
            create_time: now,
        }
    }
}

/// 游戏筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFilter {
    /// 搜索关键词
    pub search: Option<String>,
    /// 是否已安装
    pub installed: Option<bool>,
    /// 是否收藏
    pub favorite: Option<bool>,
    /// 标签筛选
    pub tags: Option<Vec<String>>,
    /// 发行商筛选
    pub publisher: Option<String>,
}

/// 游戏排序方式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameSortBy {
    /// 按名称
    Name,
    /// 按最近游玩
    LastPlayed,
    /// 按总游玩时长
    PlayTime,
    /// 按安装时间
    InstallDate,
    /// 按发行时间
    ReleaseDate,
}

/// 游戏列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameListResponse {
    /// 游戏列表
    pub games: Vec<Game>,
    /// 总数
    pub total: usize,
}

/// 添加游戏请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddGameRequest {
    /// 游戏名称
    pub name: String,
    /// 游戏exe路径
    pub exe_path: String,
    /// 封面图路径（可选）
    pub cover_path: Option<String>,
    /// 启动参数
    pub launch_params: Option<String>,
    /// 发行商
    pub publisher: Option<String>,
    /// 发行日期
    pub release_date: Option<String>,
    /// 标签
    pub tags: Option<Vec<String>>,
}

/// 更新游戏请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGameRequest {
    /// 游戏ID
    pub id: String,
    /// 游戏名称
    pub name: Option<String>,
    /// 封面图路径
    pub cover_path: Option<String>,
    /// 启动参数
    pub launch_params: Option<String>,
    /// 发行商
    pub publisher: Option<String>,
    /// 发行日期
    pub release_date: Option<String>,
    /// 标签
    pub tags: Option<Vec<String>>,
    /// 是否收藏
    pub is_favorite: Option<bool>,
}

/// 扫描游戏目录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanGamesRequest {
    /// 要扫描的目录路径
    pub directory: String,
    /// 是否递归扫描子目录
    pub recursive: bool,
}

/// 游戏启动结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchGameResult {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
    /// 进程ID
    pub pid: Option<u32>,
}

/// 游戏标签配置（来自games_config.json）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTagConfig {
    /// 补丁类型：0=免Steam，1=局域网联机，2=Steam联机，3=D加密虚拟机，4=Epic联机
    pub patch_type: u8,
    /// 补丁源路径（可选，如果不提供则自动生成）
    #[serde(default)]
    pub patch_source_path: Option<String>,
    /// 下载链接
    pub download_url: Option<String>,
}

/// 游戏配置数据（来自games_config.json）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfigData {
    /// 游戏ID（Steam App ID）
    pub game_id: String,
    /// 英文游戏名称
    pub game_name: String,
    /// 中文游戏名称
    pub chinese_name: String,
    /// 是否可下载
    pub downloadable: bool,
    /// 补丁标签列表
    pub tags: Vec<GameTagConfig>,
}
