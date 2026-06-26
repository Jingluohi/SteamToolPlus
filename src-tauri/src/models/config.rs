// 配置数据模型
// 定义应用程序配置相关的数据结构

use serde::{Deserialize, Serialize};

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct LaunchConfig {
    /// 程序启动后最小化到托盘
    pub start_minimized_to_tray: bool,
    /// 关闭程序后隐藏在托盘（默认开启）
    pub hide_to_tray_on_close: bool,
    /// 启动前检查游戏文件
    pub verify_before_launch: bool,
    /// 清单入库功能是否已完成首次初始化
    pub manifest_import_initialized: bool,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            start_minimized_to_tray: false,
            hide_to_tray_on_close: true,
            verify_before_launch: false,
            manifest_import_initialized: false,
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
}

impl Default for OpenSteamToolConfig {
    fn default() -> Self {
        Self {
            kernel_installed: false,
            advanced_mode: false,
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
}

/// 部分更新游戏目录配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PartialGameDirConfig {
    /// Steam安装路径
    pub steam_path: Option<String>,
    /// 游戏默认下载路径
    pub default_download_path: Option<String>,
    /// 封面图存储路径
    pub covers_path: Option<String>,
}

/// 部分更新启动配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PartialLaunchConfig {
    /// 程序启动后最小化到托盘
    pub start_minimized_to_tray: Option<bool>,
    /// 关闭程序后隐藏在托盘（默认开启）
    pub hide_to_tray_on_close: Option<bool>,
    /// 启动前检查游戏文件
    pub verify_before_launch: Option<bool>,
    /// 清单入库功能是否已完成首次初始化
    pub manifest_import_initialized: Option<bool>,
}
