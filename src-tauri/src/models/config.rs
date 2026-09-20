// 配置数据模型
// 定义应用程序配置相关的数据结构

use serde::{Deserialize, Serialize};

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)] // 旧版 config.json 缺少新字段时用 Default 补齐，避免整体反序列化失败丢配置
#[derive(Default)]
pub struct AppConfig {
    /// 窗口配置
    pub window: WindowConfig,
    /// 主题配置
    pub theme: ThemeConfig,
    /// 游戏目录配置
    pub game_dirs: GameDirConfig,
    /// 启动配置
    pub launch: LaunchConfig,
    /// OpenSteamTool内核配置
    pub opensteamtool: OpenSteamToolConfig,
    /// 下载记录按钮配置（无下载任务时右下角的悬浮入口位置）
    pub record_button: RecordButtonConfig,
}

/// 下载记录按钮配置
/// 存储无下载任务时「下载记录」悬浮按钮距离窗口右边缘/底部边缘的距离（px），
/// 用户可自由拖动按钮，拖动后的位置持久化到 config.json。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordButtonConfig {
    /// 距离窗口右边缘的距离（px）
    pub right: i32,
    /// 距离窗口底部边缘的距离（px）
    pub bottom: i32,
}

impl Default for RecordButtonConfig {
    fn default() -> Self {
        Self {
            right: 16,
            bottom: 16,
        }
    }
}


/// 窗口配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowConfig {
    /// 窗口宽度
    pub width: u32,
    /// 窗口高度
    pub height: u32,
    /// 是否最大化
    pub maximized: bool,
    /// 是否全屏
    pub fullscreen: bool,
    /// 窗口位置X
    pub pos_x: Option<i32>,
    /// 窗口位置Y
    pub pos_y: Option<i32>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1500,
            height: 1000,
            maximized: false,
            fullscreen: false,
            pos_x: None,
            pos_y: None,
        }
    }
}

/// 主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeConfig {
    /// 主题模式：dark/light/black/white/auto/auto-solid
    pub mode: String,
    /// 是否使用系统主题
    pub follow_system: bool,
    /// 自定义主题变量
    pub custom_vars: Option<serde_json::Value>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            mode: "dark".to_string(),
            follow_system: false,
            custom_vars: None,
        }
    }
}

/// 游戏目录配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDirConfig {
    /// Steam安装路径
    pub steam_path: Option<String>,
    /// 游戏默认下载路径
    pub default_download_path: Option<String>,
    /// 封面图存储路径
    pub covers_path: String,
}

impl Default for GameDirConfig {
    fn default() -> Self {
        Self {
            steam_path: None,
            default_download_path: None,
            covers_path: "data/covers".to_string(),
        }
    }
}

/// 启动配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)] // 旧版 config.json 的 launch 段没有新字段时用 Default 补齐，避免反序列化失败
pub struct LaunchConfig {
    /// 程序启动后最小化到托盘
    pub start_minimized_to_tray: bool,
    /// 关闭程序后隐藏在托盘（默认开启）
    pub hide_to_tray_on_close: bool,
    /// 启动前检查游戏文件
    pub verify_before_launch: bool,
    /// 清单入库功能是否已完成首次初始化
    pub manifest_import_initialized: bool,
    /// 是否使用进程内 Rust 新下载引擎。
    /// false（默认）= 使用外部 ddv20.exe 下载（稳定，验证过）；true = 使用
    /// 新引擎（不稳定、容易出错，仅限愿意尝鲜的用户手动开启）。
    pub use_new_engine: bool,

    // ==================== 新下载引擎可调参数 ====================
    // 仅当 use_new_engine = true（使用新引擎）时生效，修改后下次下载生效
    // （不热切换）。四项参数联动约束：分块并发数 × 并发 depot 数 ≤ 全局并发上限，
    // 前端保存时校验并提示。

    /// 分块并发数：单个下载任务同时从 CDN 拉取的数据块数量，直接决定带宽与 CPU。
    /// 调大 → 速度更快；调小 → 更稳定、更省资源。默认 32（推荐）。
    pub chunk_concurrency: usize,
    /// 全局并发上限：所有下载任务（多游戏/多 depot 叠加）在途分块总数的封顶值。
    /// 调大 → 多任务并行更宽裕；调小 → 更保守、连接数更少。默认 96（推荐）。
    pub global_concurrency: usize,
    /// 并发 depot 数：同一游戏内同时下载的 depot 数量。
    /// 调大 → 大文件/多 depot 更快；调小 → 小文件写盘更集中。默认 2（推荐）。
    pub concurrent_depots: usize,
    /// 文件句柄缓存上限：写盘时 LRU 保留的已打开文件句柄数量。
    /// 调大 → 小文件场景更顺畅；调小 → 更省系统句柄。默认 512（推荐）。
    pub file_handle_cache: usize,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            start_minimized_to_tray: false,
            hide_to_tray_on_close: true,
            verify_before_launch: false,
            manifest_import_initialized: false,
            use_new_engine: false,
            chunk_concurrency: 32,
            global_concurrency: 96,
            concurrent_depots: 2,
            file_handle_cache: 512,
        }
    }
}

/// OpenSteamTool内核配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSteamToolConfig {
    /// 内核DLL是否已安装到Steam目录
    pub kernel_installed: bool,
    /// 是否启用高级模式（写注册表等）
    pub advanced_mode: bool,
    /// 是否使用热加载入库（Steam运行时不重启，依赖OpenSteamTool文件监视自动加载）
    pub hot_reload: bool,
}

impl Default for OpenSteamToolConfig {
    fn default() -> Self {
        Self {
            kernel_installed: false,
            advanced_mode: false,
            hot_reload: true,
        }
    }
}

/// 更新配置请求
/// 使用 Option 包装各个字段，允许部分更新
/// 如果字段为 Some，则更新对应配置；如果为 None，则保持原值不变
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigRequest {
    /// 窗口配置更新
    pub window: Option<WindowConfig>,
    /// 主题配置更新
    pub theme: Option<ThemeConfig>,
    /// 游戏目录配置更新
    pub game_dirs: Option<GameDirConfig>,
    /// 启动配置更新
    pub launch: Option<LaunchConfig>,
    /// OpenSteamTool内核配置更新
    pub opensteamtool: Option<OpenSteamToolConfig>,
    /// 下载记录按钮配置更新
    pub record_button: Option<RecordButtonConfig>,
}

