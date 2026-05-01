// 配置数据模型
// 定义应用程序配置相关的数据结构

use serde::{Deserialize, Serialize};

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// 窗口配置
    pub window: WindowConfig,
    /// 主题配置
    pub theme: ThemeConfig,
    /// 游戏目录配置
    pub game_dirs: GameDirConfig,
    /// 启动配置
    pub launch: LaunchConfig,
    /// 扩展配置
    pub extension: ExtensionConfig,
    /// 安全配置
    pub security: SecurityConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            theme: ThemeConfig::default(),
            game_dirs: GameDirConfig::default(),
            launch: LaunchConfig::default(),
            extension: ExtensionConfig::default(),
            security: SecurityConfig::default(),
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
    /// 主题模式：dark/light/auto
    pub mode: String,
    /// 是否使用系统主题
    pub follow_system: bool,
    /// 自定义主题变量
    pub custom_vars: Option<serde_json::Value>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            mode: "auto".to_string(),
            follow_system: true,
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
    /// 封面图存储路径
    pub covers_path: String,
}

impl Default for GameDirConfig {
    fn default() -> Self {
        Self {
            steam_path: None,
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
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            start_minimized_to_tray: false,
            hide_to_tray_on_close: true,
            verify_before_launch: false,
        }
    }
}

/// 扩展配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionConfig {
    /// 扩展目录路径
    pub extensions_path: String,
    /// 是否启用扩展
    pub enabled: bool,
    /// 已启用扩展列表
    pub enabled_extensions: Vec<String>,
    /// 扩展权限配置
    pub permissions: serde_json::Value,
}

impl Default for ExtensionConfig {
    fn default() -> Self {
        Self {
            extensions_path: "extensions".to_string(),
            enabled: true,
            enabled_extensions: Vec::new(),
            permissions: serde_json::json!({}),
        }
    }
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityConfig {
    /// 是否启用沙箱
    pub sandbox_enabled: bool,
    /// 允许扩展访问的文件路径
    pub allowed_paths: Vec<String>,
    /// 禁止扩展访问的路径
    pub blocked_paths: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            sandbox_enabled: true,
            allowed_paths: vec!["data".to_string(), "config".to_string()],
            blocked_paths: vec![
                "C:\\Windows".to_string(),
                "C:\\Program Files".to_string(),
            ],
        }
    }
}

/// 更新配置请求
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
    /// 扩展配置更新
    pub extension: Option<ExtensionConfig>,
    /// 安全配置更新
    pub security: Option<SecurityConfig>,
}
