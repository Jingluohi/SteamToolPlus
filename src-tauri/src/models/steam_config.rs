// Steam 模拟器配置文件数据模型
// 100% 对应 gbe_fork 的所有配置文件

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================
// 1. 核心配置文件
// ============================================

/// configs.main.ini - 主配置文件
/// 100% 实现 gbe_fork 所有配置选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainConfig {
    // [main::general] 通用设置
    #[serde(rename = "new_app_ticket")]
    pub new_app_ticket: bool,
    #[serde(rename = "gc_token")]
    pub gc_token: bool,
    #[serde(rename = "block_unknown_clients")]
    pub block_unknown_clients: bool,
    #[serde(rename = "steam_deck")]
    pub steam_deck: bool,
    #[serde(rename = "enable_account_avatar")]
    pub enable_account_avatar: bool,
    #[serde(rename = "enable_voice_chat")]
    pub enable_voice_chat: bool,
    #[serde(rename = "immediate_gameserver_stats")]
    pub immediate_gameserver_stats: bool,
    #[serde(rename = "matchmaking_server_list_actual_type")]
    pub matchmaking_server_list_actual_type: bool,
    #[serde(rename = "matchmaking_server_details_via_source_query")]
    pub matchmaking_server_details_via_source_query: bool,
    #[serde(rename = "crash_printer_location")]
    pub crash_printer_location: Option<String>,

    // [main::stats] 统计设置
    #[serde(rename = "disable_leaderboards_create_unknown")]
    pub disable_leaderboards_create_unknown: bool,
    #[serde(rename = "allow_unknown_stats")]
    pub allow_unknown_stats: bool,
    #[serde(rename = "stat_achievement_progress_functionality")]
    pub stat_achievement_progress_functionality: bool,
    #[serde(rename = "save_only_higher_stat_achievement_progress")]
    pub save_only_higher_stat_achievement_progress: bool,
    #[serde(rename = "paginated_achievements_icons")]
    pub paginated_achievements_icons: i32,
    #[serde(rename = "record_playtime")]
    pub record_playtime: bool,

    // [main::connectivity] 连接设置
    #[serde(rename = "disable_lan_only")]
    pub disable_lan_only: bool,
    #[serde(rename = "disable_networking")]
    pub disable_networking: bool,
    #[serde(rename = "listen_port")]
    pub listen_port: i32,
    #[serde(rename = "offline")]
    pub offline: bool,
    #[serde(rename = "disable_sharing_stats_with_gameserver")]
    pub disable_sharing_stats_with_gameserver: bool,
    #[serde(rename = "disable_source_query")]
    pub disable_source_query: bool,
    #[serde(rename = "share_leaderboards_over_network")]
    pub share_leaderboards_over_network: bool,
    #[serde(rename = "disable_lobby_creation")]
    pub disable_lobby_creation: bool,
    #[serde(rename = "download_steamhttp_requests")]
    pub download_steamhttp_requests: bool,

    // [main::misc] 其他设置
    #[serde(rename = "achievements_bypass")]
    pub achievements_bypass: bool,
    #[serde(rename = "force_steamhttp_success")]
    pub force_steamhttp_success: bool,
    #[serde(rename = "disable_steamoverlaygameid_env_var")]
    pub disable_steamoverlaygameid_env_var: bool,
    #[serde(rename = "enable_steam_preowned_ids")]
    pub enable_steam_preowned_ids: bool,
    #[serde(rename = "steam_game_stats_reports_dir")]
    pub steam_game_stats_reports_dir: Option<String>,
    #[serde(rename = "free_weekend")]
    pub free_weekend: bool,
    #[serde(rename = "use_32bit_inventory_item_ids")]
    pub use_32bit_inventory_item_ids: bool,

    // 额外DLL列表
    #[serde(rename = "extra_dlls")]
    pub extra_dlls: Vec<String>,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

impl MainConfig {
    pub fn default_config() -> Self {
        Self {
            // [main::general] 默认值
            new_app_ticket: true,
            gc_token: true,
            block_unknown_clients: false,
            steam_deck: false,
            enable_account_avatar: false,
            enable_voice_chat: false,
            immediate_gameserver_stats: false,
            matchmaking_server_list_actual_type: false,
            matchmaking_server_details_via_source_query: false,
            crash_printer_location: None,

            // [main::stats] 默认值
            disable_leaderboards_create_unknown: false,
            allow_unknown_stats: true,
            stat_achievement_progress_functionality: true,
            save_only_higher_stat_achievement_progress: true,
            paginated_achievements_icons: 10,
            record_playtime: false,

            // [main::connectivity] 默认值
            disable_lan_only: false,
            disable_networking: false,
            listen_port: 47584,
            offline: false,
            disable_sharing_stats_with_gameserver: false,
            disable_source_query: false,
            share_leaderboards_over_network: false,
            disable_lobby_creation: false,
            download_steamhttp_requests: false,

            // [main::misc] 默认值
            achievements_bypass: false,
            force_steamhttp_success: false,
            disable_steamoverlaygameid_env_var: false,
            enable_steam_preowned_ids: false,
            steam_game_stats_reports_dir: None,
            free_weekend: false,
            use_32bit_inventory_item_ids: false,

            // 额外DLL
            extra_dlls: vec![],
        }
    }

    /// 序列化为 INI 格式
    pub fn to_ini(&self) -> String {
        let mut result = String::new();

        // [main::general]
        result.push_str("[main::general]\n");
        result.push_str(&format!("new_app_ticket = {}\n", self.new_app_ticket as i32));
        result.push_str(&format!("gc_token = {}\n", self.gc_token as i32));
        result.push_str(&format!("block_unknown_clients = {}\n", self.block_unknown_clients as i32));
        result.push_str(&format!("steam_deck = {}\n", self.steam_deck as i32));
        result.push_str(&format!("enable_account_avatar = {}\n", self.enable_account_avatar as i32));
        result.push_str(&format!("enable_voice_chat = {}\n", self.enable_voice_chat as i32));
        result.push_str(&format!("immediate_gameserver_stats = {}\n", self.immediate_gameserver_stats as i32));
        result.push_str(&format!("matchmaking_server_list_actual_type = {}\n", self.matchmaking_server_list_actual_type as i32));
        result.push_str(&format!("matchmaking_server_details_via_source_query = {}\n", self.matchmaking_server_details_via_source_query as i32));
        if let Some(ref path) = self.crash_printer_location {
            result.push_str(&format!("crash_printer_location = {}\n", path));
        }
        result.push('\n');

        // [main::stats]
        result.push_str("[main::stats]\n");
        result.push_str(&format!("disable_leaderboards_create_unknown = {}\n", self.disable_leaderboards_create_unknown as i32));
        result.push_str(&format!("allow_unknown_stats = {}\n", self.allow_unknown_stats as i32));
        result.push_str(&format!("stat_achievement_progress_functionality = {}\n", self.stat_achievement_progress_functionality as i32));
        result.push_str(&format!("save_only_higher_stat_achievement_progress = {}\n", self.save_only_higher_stat_achievement_progress as i32));
        result.push_str(&format!("paginated_achievements_icons = {}\n", self.paginated_achievements_icons));
        result.push_str(&format!("record_playtime = {}\n", self.record_playtime as i32));
        result.push('\n');

        // [main::connectivity]
        result.push_str("[main::connectivity]\n");
        result.push_str(&format!("disable_lan_only = {}\n", self.disable_lan_only as i32));
        result.push_str(&format!("disable_networking = {}\n", self.disable_networking as i32));
        result.push_str(&format!("listen_port = {}\n", self.listen_port));
        result.push_str(&format!("offline = {}\n", self.offline as i32));
        result.push_str(&format!("disable_sharing_stats_with_gameserver = {}\n", self.disable_sharing_stats_with_gameserver as i32));
        result.push_str(&format!("disable_source_query = {}\n", self.disable_source_query as i32));
        result.push_str(&format!("share_leaderboards_over_network = {}\n", self.share_leaderboards_over_network as i32));
        result.push_str(&format!("disable_lobby_creation = {}\n", self.disable_lobby_creation as i32));
        result.push_str(&format!("download_steamhttp_requests = {}\n", self.download_steamhttp_requests as i32));
        result.push('\n');

        // [main::misc]
        result.push_str("[main::misc]\n");
        result.push_str(&format!("achievements_bypass = {}\n", self.achievements_bypass as i32));
        result.push_str(&format!("force_steamhttp_success = {}\n", self.force_steamhttp_success as i32));
        result.push_str(&format!("disable_steamoverlaygameid_env_var = {}\n", self.disable_steamoverlaygameid_env_var as i32));
        result.push_str(&format!("enable_steam_preowned_ids = {}\n", self.enable_steam_preowned_ids as i32));
        if let Some(ref path) = self.steam_game_stats_reports_dir {
            result.push_str(&format!("steam_game_stats_reports_dir = {}\n", path));
        }
        result.push_str(&format!("free_weekend = {}\n", self.free_weekend as i32));
        result.push_str(&format!("use_32bit_inventory_item_ids = {}\n", self.use_32bit_inventory_item_ids as i32));
        result.push('\n');

        // [extra_dlls]
        result.push_str("[extra_dlls]\n");
        if self.extra_dlls.is_empty() {
            result.push_str("# No extra DLLs\n");
        } else {
            for (i, dll) in self.extra_dlls.iter().enumerate() {
                result.push_str(&format!("dll{} = {}\n", i + 1, dll));
            }
        }

        result
    }
}

/// configs.user.ini - 用户配置文件
/// 100% 实现 gbe_fork 所有配置选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// 用户名 (account_name)
    pub username: String,
    /// Steam64 格式的用户 ID
    #[serde(rename = "account_steamid", default, alias = "accountSteamid")]
    pub account_steamid: Option<String>,
    /// 语言
    pub language: String,
    /// IP 国家代码 (ISO 3166-1-alpha-2)
    #[serde(rename = "ip_country", default, alias = "ipCountry")]
    pub ip_country: Option<String>,
    /// 存档文件夹名称（覆盖默认的 "GSE Saves"）
    #[serde(rename = "saves_folder_name", alias = "savesFolderName")]
    pub saves_folder_name: Option<String>,
    /// 本地存档路径（便携模式）
    #[serde(rename = "local_save_path", alias = "localSavePath")]
    pub local_save_path: Option<String>,
    /// EncryptedAppTicket (Base64编码)
    #[serde(rename = "ticket")]
    pub ticket: Option<String>,
    /// 备用 SteamID（用于加密存档替换）
    #[serde(rename = "alt_steamid", default, alias = "altSteamid")]
    pub alt_steamid: Option<String>,
    /// 备用 SteamID 替换触发次数
    #[serde(rename = "alt_steamid_count", default, alias = "altSteamidCount")]
    pub alt_steamid_count: Option<u64>,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

impl UserConfig {
    pub fn default_config() -> Self {
        Self {
            username: "Player".to_string(),
            account_steamid: None,
            language: "schinese".to_string(),
            ip_country: Some("CN".to_string()),
            saves_folder_name: None,
            local_save_path: None,
            ticket: None,
            alt_steamid: None,
            alt_steamid_count: None,
        }
    }

    pub fn to_ini(&self) -> String {
        let mut result = String::new();
        result.push_str("[user::general]\n");
        result.push_str(&format!("account_name = {}\n", self.username));

        if let Some(ref steamid) = self.account_steamid {
            result.push_str(&format!("account_steamid = {}\n", steamid));
        }

        if let Some(ref ticket) = self.ticket {
            result.push_str(&format!("ticket = {}\n", ticket));
        }

        if let Some(ref alt_id) = self.alt_steamid {
            result.push_str(&format!("alt_steamid = {}\n", alt_id));
        }
        if let Some(ref alt_count) = self.alt_steamid_count {
            result.push_str(&format!("alt_steamid_count = {}\n", alt_count));
        }

        result.push_str(&format!("language = {}\n", self.language));

        if let Some(ref country) = self.ip_country {
            result.push_str(&format!("ip_country = {}\n", country));
        }

        result.push_str("\n[user::saves]\n");

        if let Some(ref path) = self.local_save_path {
            result.push_str(&format!("local_save_path = {}\n", path));
        }

        if let Some(ref name) = self.saves_folder_name {
            result.push_str(&format!("saves_folder_name = {}\n", name));
        }

        result
    }
}

/// configs.app.ini - 应用配置文件 (Steam)
/// 100% 实现 gbe_fork 所有配置选项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SteamAppConfig {
    /// 当前分支名称
    #[serde(rename = "branch_name")]
    pub branch_name: String,
    /// 是否为 Beta 分支
    #[serde(rename = "is_beta_branch", default)]
    pub is_beta_branch: bool,
    /// 应用路径映射（DLC 安装路径）
    #[serde(rename = "app_paths", default)]
    pub app_paths: HashMap<String, String>,
    /// DLC 配置
    pub dlcs: DlcConfig,
    /// Steam Input 控制器配置
    #[serde(rename = "controller", default)]
    pub controller: AppControllerConfig,
    /// 云存档配置
    #[serde(rename = "cloud_saves", default)]
    pub cloud_saves: CloudSavesConfig,
}

/// Steam Input 控制器配置
/// [app::controller] 段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppControllerConfig {
    /// Steam Input 开关
    #[serde(rename = "steam_input", default)]
    pub steam_input: Option<bool>,
    /// 控制器类型：XBOX360 / PS4 / PS5 / SWITCH
    #[serde(rename = "type", default)]
    pub r#type: Option<String>,
}

/// 云存档配置
/// [app::cloud_save::general] 段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudSavesConfig {
    /// 是否启用云存档
    #[serde(rename = "enabled", default)]
    pub enabled: Option<bool>,
    /// 自动创建默认目录
    #[serde(rename = "create_default_dir", default)]
    pub create_default_dir: Option<bool>,
    /// 自动创建特定目录
    #[serde(rename = "create_specific_dirs", default)]
    pub create_specific_dirs: Option<bool>,
    /// Windows 云存档路径列表
    #[serde(rename = "windows_dirs", default)]
    pub windows_dirs: Vec<String>,
    /// Linux 云存档路径列表
    #[serde(rename = "linux_dirs", default)]
    pub linux_dirs: Vec<String>,
}

/// DLC 配置
/// gbe_fork 格式: configs.app.ini 中 [app::dlcs] 段
/// 支持前端简化格式: dlc_list 为 DLC ID 列表, depot_ids 为 Depot ID 列表
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DlcConfig {
    #[serde(rename = "unlock_all")]
    pub unlock_all: bool,
    /// 单个 DLC 列表 (gbe_fork 格式: appid = enabled # name)
    #[serde(rename = "individual_dlcs")]
    pub individual_dlcs: Vec<IndividualDlc>,
    /// DLC ID 列表（简化格式，每行一个 ID）
    #[serde(rename = "dlc_list", default)]
    pub dlc_list: Vec<String>,
    /// Depot IDs 列表（极少数游戏需要）
    #[serde(rename = "depot_ids", default)]
    pub depot_ids: Vec<String>,
    /// DLC 安装路径映射（极少数游戏需要，格式: appid => path）
    #[serde(rename = "dlc_paths", default)]
    pub dlc_paths: HashMap<String, String>,
}

/// 单个 DLC 条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualDlc {
    #[serde(rename = "app_id")]
    pub app_id: String,
    pub name: String,
    pub enabled: bool,
}

impl SteamAppConfig {
    pub fn default_config() -> Self {
        Self {
            branch_name: "public".to_string(),
            is_beta_branch: false,
            app_paths: HashMap::new(),
            dlcs: DlcConfig {
                unlock_all: true,
                individual_dlcs: vec![],
                dlc_list: vec![],
                depot_ids: vec![],
                dlc_paths: HashMap::new(),
            },
            controller: AppControllerConfig {
                steam_input: Some(true),
                r#type: Some("XBOX360".to_string()),
            },
            cloud_saves: CloudSavesConfig::default(),
        }
    }

    pub fn to_ini(&self) -> String {
        let mut result = format!(
            "[app::general]\nbranch_name = {}\nis_beta_branch = {}\n\n",
            self.branch_name,
            self.is_beta_branch as i32
        );

        // [app::paths] 段
        result.push_str("[app::paths]\n");
        let has_paths = !self.app_paths.is_empty() || !self.dlcs.dlc_paths.is_empty();
        if !has_paths {
            result.push_str("# No custom paths\n");
        } else {
            for (key, value) in &self.dlcs.dlc_paths {
                result.push_str(&format!("{} = {}\n", key, value));
            }
            for (key, value) in &self.app_paths {
                result.push_str(&format!("{} = {}\n", key, value));
            }
        }
        result.push('\n');

        // [app::dlcs] 段
        result.push_str(&format!(
            "[app::dlcs]\nunlock_all = {}\n",
            self.dlcs.unlock_all as i32
        ));
        // gbe_fork 格式: appid=DLC Name (直接在 [app::dlcs] 段内，没有 enabled 前缀)
        if !self.dlcs.individual_dlcs.is_empty() {
            for dlc in &self.dlcs.individual_dlcs {
                result.push_str(&format!("{} = {}\n", dlc.app_id, dlc.name));
            }
        }
        result.push('\n');

        // [app::controller] 段
        result.push_str("[app::controller]\n");
        if let Some(steam_input) = self.controller.steam_input {
            result.push_str(&format!("steam_input = {}\n", steam_input as i32));
        }
        if let Some(ref rtype) = self.controller.r#type {
            result.push_str(&format!("type = {}\n", rtype));
        }
        result.push('\n');

        // [app::cloud_save::general] 段
        result.push_str("[app::cloud_save::general]\n");
        if let Some(enabled) = self.cloud_saves.enabled {
            result.push_str(&format!("enabled = {}\n", enabled as i32));
        }
        if let Some(create_default) = self.cloud_saves.create_default_dir {
            result.push_str(&format!("create_default_dir = {}\n", create_default as i32));
        }
        if let Some(create_specific) = self.cloud_saves.create_specific_dirs {
            result.push_str(&format!("create_specific_dirs = {}\n", create_specific as i32));
        }
        result.push('\n');

        // [app::cloud_save::win] 段
        result.push_str("[app::cloud_save::win]\n");
        if self.cloud_saves.windows_dirs.is_empty() {
            result.push_str("# No Windows cloud save paths\n");
        } else {
            for (i, dir) in self.cloud_saves.windows_dirs.iter().enumerate() {
                result.push_str(&format!("dir{} = {}\n", i + 1, dir));
            }
        }
        result.push('\n');

        // [app::cloud_save::linux] 段
        result.push_str("[app::cloud_save::linux]\n");
        if self.cloud_saves.linux_dirs.is_empty() {
            result.push_str("# No Linux cloud save paths\n");
        } else {
            for (i, dir) in self.cloud_saves.linux_dirs.iter().enumerate() {
                result.push_str(&format!("dir{} = {}\n", i + 1, dir));
            }
        }
        result.push('\n');

        result
    }
}

/// configs.overlay.ini - 覆盖层配置文件
/// 100% 实现 gbe_fork 所有配置选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayConfig {
    /// 启用实验性覆盖层
    #[serde(rename = "enable_experimental_overlay")]
    pub enable_experimental_overlay: bool,
    /// Hook 延迟（秒）
    #[serde(rename = "hook_delay_sec", default)]
    pub hook_delay_sec: Option<u32>,
    /// 渲染器检测超时（秒）
    #[serde(rename = "renderer_detector_timeout_sec", default)]
    pub renderer_detector_timeout_sec: Option<u32>,
    /// 热键组合（gbe_fork 格式: shift + tab）
    #[serde(rename = "overlay_hotkey", default)]
    pub overlay_hotkey: String,
    /// FPS 平均窗口
    #[serde(rename = "fps_averaging_window", default)]
    pub fps_averaging_window: Option<f32>,
    /// 通知与功能开关
    #[serde(default)]
    pub notifications: OverlayNotifications,
    /// 外观设置
    #[serde(default)]
    pub appearance: OverlayAppearance,
    /// 性能设置
    #[serde(default)]
    pub performance: OverlayPerformance,
    /// 功能开关
    #[serde(default)]
    pub features: OverlayFeatures,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

/// 通知与功能开关配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayNotifications {
    /// 禁用成就通知
    #[serde(rename = "disable_achievement_notification", default)]
    pub disable_achievement_notification: bool,
    /// 禁用好友通知
    #[serde(rename = "disable_friend_notification", default)]
    pub disable_friend_notification: bool,
    /// 禁用成就进度
    #[serde(rename = "disable_achievement_progress", default)]
    pub disable_achievement_progress: bool,
    /// 禁用所有警告
    #[serde(rename = "disable_warning_any", default)]
    pub disable_warning_any: bool,
    /// 禁用 Bad AppID 警告
    #[serde(rename = "disable_warning_bad_appid", default)]
    pub disable_warning_bad_appid: bool,
    /// 禁用本地存档警告
    #[serde(rename = "disable_warning_local_save", default)]
    pub disable_warning_local_save: bool,
    /// 上传成就图标到 GPU
    #[serde(rename = "upload_achievements_icons_to_gpu", default)]
    pub upload_achievements_icons_to_gpu: bool,
    /// 始终显示用户信息
    #[serde(rename = "overlay_always_show_user_info", default)]
    pub overlay_always_show_user_info: bool,
    /// 始终显示 FPS
    #[serde(rename = "overlay_always_show_fps", default)]
    pub overlay_always_show_fps: bool,
    /// 始终显示帧时间
    #[serde(rename = "overlay_always_show_frametime", default)]
    pub overlay_always_show_frametime: bool,
    /// 始终显示游玩时间
    #[serde(rename = "overlay_always_show_playtime", default)]
    pub overlay_always_show_playtime: bool,
    /// 成就通知时长（秒）
    #[serde(rename = "notification_duration_achievement", default)]
    pub notification_duration_achievement: Option<i32>,
    /// 邀请通知时长（秒）
    #[serde(rename = "notification_duration_invitation", default)]
    pub notification_duration_invitation: Option<i32>,
    /// 聊天通知时长（秒）
    #[serde(rename = "notification_duration_chat", default)]
    pub notification_duration_chat: Option<i32>,
    /// 进度通知时长（秒）
    #[serde(rename = "notification_duration_progress", default)]
    pub notification_duration_progress: Option<i32>,
}

impl Default for OverlayNotifications {
    fn default() -> Self {
        Self {
            disable_achievement_notification: false,
            disable_friend_notification: false,
            disable_achievement_progress: false,
            disable_warning_any: false,
            disable_warning_bad_appid: false,
            disable_warning_local_save: false,
            upload_achievements_icons_to_gpu: true,
            overlay_always_show_user_info: false,
            overlay_always_show_fps: false,
            overlay_always_show_frametime: false,
            overlay_always_show_playtime: false,
            notification_duration_achievement: None,
            notification_duration_invitation: None,
            notification_duration_chat: None,
            notification_duration_progress: None,
        }
    }
}

/// 外观设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayAppearance {
    /// 字体覆盖
    #[serde(rename = "Font_Override", default)]
    pub font_override: Option<String>,
    /// 字体大小
    #[serde(rename = "Font_Size", default)]
    pub font_size: Option<i32>,
    /// 字体字间距 X
    #[serde(rename = "Font_Glyph_Extra_Spacing_x", default)]
    pub font_glyph_extra_spacing_x: Option<f32>,
    /// 字体字间距 Y
    #[serde(rename = "Font_Glyph_Extra_Spacing_y", default)]
    pub font_glyph_extra_spacing_y: Option<f32>,
    /// 图标大小
    #[serde(rename = "Icon_Size", default)]
    pub icon_size: Option<i32>,
    /// 主题
    pub theme: String,
    /// 透明度
    pub opacity: f32,
    /// 缩放
    pub scale: f32,
    /// 模糊
    pub blur: bool,
    /// 通知圆角
    #[serde(rename = "Notification_Rounding", default)]
    pub notification_rounding: Option<i32>,
    /// 通知边距 X
    #[serde(rename = "Notification_Margin_x", default)]
    pub notification_margin_x: Option<i32>,
    /// 通知边距 Y
    #[serde(rename = "Notification_Margin_y", default)]
    pub notification_margin_y: Option<i32>,
    /// 通知背景色 R
    #[serde(rename = "Notification_R", default)]
    pub notification_r: Option<i32>,
    /// 通知背景色 G
    #[serde(rename = "Notification_G", default)]
    pub notification_g: Option<i32>,
    /// 通知背景色 B
    #[serde(rename = "Notification_B", default)]
    pub notification_b: Option<i32>,
    /// 通知背景色 A
    #[serde(rename = "Notification_A", default)]
    pub notification_a: Option<i32>,
    /// 成就解锁日期格式
    #[serde(rename = "Achievement_Unlock_Datetime_Format", default)]
    pub achievement_unlock_datetime_format: Option<String>,
    /// 背景色 R
    #[serde(rename = "Background_R", default)]
    pub background_r: Option<i32>,
    /// 背景色 G
    #[serde(rename = "Background_G", default)]
    pub background_g: Option<i32>,
    /// 背景色 B
    #[serde(rename = "Background_B", default)]
    pub background_b: Option<i32>,
    /// 背景色 A
    #[serde(rename = "Background_A", default)]
    pub background_a: Option<i32>,
    /// 元素色 R
    #[serde(rename = "Element_R", default)]
    pub element_r: Option<i32>,
    /// 元素色 G
    #[serde(rename = "Element_G", default)]
    pub element_g: Option<i32>,
    /// 元素色 B
    #[serde(rename = "Element_B", default)]
    pub element_b: Option<i32>,
    /// 元素色 A
    #[serde(rename = "Element_A", default)]
    pub element_a: Option<i32>,
    /// 元素悬停色 R
    #[serde(rename = "ElementHovered_R", default)]
    pub element_hovered_r: Option<i32>,
    /// 元素悬停色 G
    #[serde(rename = "ElementHovered_G", default)]
    pub element_hovered_g: Option<i32>,
    /// 元素悬停色 B
    #[serde(rename = "ElementHovered_B", default)]
    pub element_hovered_b: Option<i32>,
    /// 元素悬停色 A
    #[serde(rename = "ElementHovered_A", default)]
    pub element_hovered_a: Option<i32>,
    /// 元素激活色 R
    #[serde(rename = "ElementActive_R", default)]
    pub element_active_r: Option<i32>,
    /// 元素激活色 G
    #[serde(rename = "ElementActive_G", default)]
    pub element_active_g: Option<i32>,
    /// 元素激活色 B
    #[serde(rename = "ElementActive_B", default)]
    pub element_active_b: Option<i32>,
    /// 元素激活色 A
    #[serde(rename = "ElementActive_A", default)]
    pub element_active_a: Option<i32>,
    /// 成就通知位置
    #[serde(rename = "PosAchievement", default)]
    pub pos_achievement: Option<String>,
    /// 邀请通知位置
    #[serde(rename = "PosInvitation", default)]
    pub pos_invitation: Option<String>,
    /// 聊天消息位置
    #[serde(rename = "PosChatMsg", default)]
    pub pos_chat_msg: Option<String>,
    /// 统计背景色 R
    #[serde(rename = "Stats_Background_R", default)]
    pub stats_background_r: Option<i32>,
    /// 统计背景色 G
    #[serde(rename = "Stats_Background_G", default)]
    pub stats_background_g: Option<i32>,
    /// 统计背景色 B
    #[serde(rename = "Stats_Background_B", default)]
    pub stats_background_b: Option<i32>,
    /// 统计背景色 A
    #[serde(rename = "Stats_Background_A", default)]
    pub stats_background_a: Option<i32>,
    /// 统计文字色 R
    #[serde(rename = "Stats_Text_R", default)]
    pub stats_text_r: Option<i32>,
    /// 统计文字色 G
    #[serde(rename = "Stats_Text_G", default)]
    pub stats_text_g: Option<i32>,
    /// 统计文字色 B
    #[serde(rename = "Stats_Text_B", default)]
    pub stats_text_b: Option<i32>,
    /// 统计文字色 A
    #[serde(rename = "Stats_Text_A", default)]
    pub stats_text_a: Option<i32>,
    /// 统计位置 X
    #[serde(rename = "Stats_Pos_x", default)]
    pub stats_pos_x: Option<i32>,
    /// 统计位置 Y
    #[serde(rename = "Stats_Pos_y", default)]
    pub stats_pos_y: Option<i32>,
}

impl Default for OverlayAppearance {
    fn default() -> Self {
        Self {
            font_override: None,
            font_size: None,
            font_glyph_extra_spacing_x: None,
            font_glyph_extra_spacing_y: None,
            icon_size: None,
            theme: "dark".to_string(),
            opacity: 0.95,
            scale: 1.0,
            blur: true,
            notification_rounding: None,
            notification_margin_x: None,
            notification_margin_y: None,
            notification_r: None,
            notification_g: None,
            notification_b: None,
            notification_a: None,
            achievement_unlock_datetime_format: None,
            background_r: None,
            background_g: None,
            background_b: None,
            background_a: None,
            element_r: None,
            element_g: None,
            element_b: None,
            element_a: None,
            element_hovered_r: None,
            element_hovered_g: None,
            element_hovered_b: None,
            element_hovered_a: None,
            element_active_r: None,
            element_active_g: None,
            element_active_b: None,
            element_active_a: None,
            pos_achievement: None,
            pos_invitation: None,
            pos_chat_msg: None,
            stats_background_r: None,
            stats_background_g: None,
            stats_background_b: None,
            stats_background_a: None,
            stats_text_r: None,
            stats_text_g: None,
            stats_text_b: None,
            stats_text_a: None,
            stats_pos_x: None,
            stats_pos_y: None,
        }
    }
}

/// 性能设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayPerformance {
    /// 硬件加速
    #[serde(rename = "hardware_acceleration")]
    pub hardware_acceleration: bool,
    /// FPS 限制
    #[serde(rename = "fps_limit")]
    pub fps_limit: i32,
    /// 低性能模式
    #[serde(rename = "low_performance_mode")]
    pub low_performance_mode: bool,
}

impl Default for OverlayPerformance {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            fps_limit: 60,
            low_performance_mode: false,
        }
    }
}

/// 功能开关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayFeatures {
    /// 成就
    pub achievements: bool,
    /// 好友
    pub friends: bool,
    /// 聊天
    pub chat: bool,
    /// 浏览器
    pub browser: bool,
    /// 设置
    pub settings: bool,
}

impl Default for OverlayFeatures {
    fn default() -> Self {
        Self {
            achievements: true,
            friends: true,
            chat: true,
            browser: true,
            settings: true,
        }
    }
}

impl OverlayConfig {
    pub fn default_config() -> Self {
        Self {
            enable_experimental_overlay: false,
            hook_delay_sec: None,
            renderer_detector_timeout_sec: None,
            overlay_hotkey: "shift + tab".to_string(),
            fps_averaging_window: None,
            notifications: OverlayNotifications::default(),
            appearance: OverlayAppearance::default(),
            performance: OverlayPerformance::default(),
            features: OverlayFeatures::default(),
        }
    }

    pub fn to_ini(&self) -> String {
        let mut result = String::new();

        // [overlay::general] 段
        result.push_str("[overlay::general]\n");
        result.push_str(&format!("enable_experimental_overlay = {}\n", self.enable_experimental_overlay as i32));
        if let Some(hook_delay) = self.hook_delay_sec {
            result.push_str(&format!("hook_delay_sec = {}\n", hook_delay));
        }
        if let Some(timeout) = self.renderer_detector_timeout_sec {
            result.push_str(&format!("renderer_detector_timeout_sec = {}\n", timeout));
        }
        if let Some(avg_window) = self.fps_averaging_window {
            result.push_str(&format!("fps_averaging_window = {:.1}\n", avg_window));
        }
        result.push('\n');

        // [overlay::hotkeys] 段
        result.push_str("[overlay::hotkeys]\n");
        result.push_str(&format!("key_combo = {}\n", self.overlay_hotkey));
        result.push('\n');

        // [overlay::notifications] 段
        result.push_str("[overlay::notifications]\n");
        let n = &self.notifications;
        result.push_str(&format!("disable_achievement_notification = {}\n", n.disable_achievement_notification as i32));
        result.push_str(&format!("disable_friend_notification = {}\n", n.disable_friend_notification as i32));
        result.push_str(&format!("disable_achievement_progress = {}\n", n.disable_achievement_progress as i32));
        result.push_str(&format!("disable_warning_any = {}\n", n.disable_warning_any as i32));
        result.push_str(&format!("disable_warning_bad_appid = {}\n", n.disable_warning_bad_appid as i32));
        result.push_str(&format!("disable_warning_local_save = {}\n", n.disable_warning_local_save as i32));
        result.push_str(&format!("upload_achievements_icons_to_gpu = {}\n", n.upload_achievements_icons_to_gpu as i32));
        result.push_str(&format!("overlay_always_show_user_info = {}\n", n.overlay_always_show_user_info as i32));
        result.push_str(&format!("overlay_always_show_fps = {}\n", n.overlay_always_show_fps as i32));
        result.push_str(&format!("overlay_always_show_frametime = {}\n", n.overlay_always_show_frametime as i32));
        result.push_str(&format!("overlay_always_show_playtime = {}\n", n.overlay_always_show_playtime as i32));
        if let Some(v) = n.notification_duration_achievement { result.push_str(&format!("notification_duration_achievement = {}\n", v)); }
        if let Some(v) = n.notification_duration_invitation { result.push_str(&format!("notification_duration_invitation = {}\n", v)); }
        if let Some(v) = n.notification_duration_chat { result.push_str(&format!("notification_duration_chat = {}\n", v)); }
        if let Some(v) = n.notification_duration_progress { result.push_str(&format!("notification_duration_progress = {}\n", v)); }
        result.push('\n');

        // [overlay::appearance] 段
        result.push_str("[overlay::appearance]\n");
        let a = &self.appearance;
        if let Some(ref v) = a.font_override { result.push_str(&format!("Font_Override = {}\n", v)); }
        if let Some(v) = a.font_size { result.push_str(&format!("Font_Size = {}\n", v)); }
        if let Some(v) = a.font_glyph_extra_spacing_x { result.push_str(&format!("Font_Glyph_Extra_Spacing_x = {:.1}\n", v)); }
        if let Some(v) = a.font_glyph_extra_spacing_y { result.push_str(&format!("Font_Glyph_Extra_Spacing_y = {:.1}\n", v)); }
        if let Some(v) = a.icon_size { result.push_str(&format!("Icon_Size = {}\n", v)); }
        result.push_str(&format!("theme = {}\n", a.theme));
        result.push_str(&format!("opacity = {:.2}\n", a.opacity));
        result.push_str(&format!("scale = {:.2}\n", a.scale));
        result.push_str(&format!("blur = {}\n", a.blur as i32));
        if let Some(v) = a.notification_rounding { result.push_str(&format!("Notification_Rounding = {}\n", v)); }
        if let Some(v) = a.notification_margin_x { result.push_str(&format!("Notification_Margin_x = {}\n", v)); }
        if let Some(v) = a.notification_margin_y { result.push_str(&format!("Notification_Margin_y = {}\n", v)); }
        if let Some(v) = a.notification_r { result.push_str(&format!("Notification_R = {}\n", v)); }
        if let Some(v) = a.notification_g { result.push_str(&format!("Notification_G = {}\n", v)); }
        if let Some(v) = a.notification_b { result.push_str(&format!("Notification_B = {}\n", v)); }
        if let Some(v) = a.notification_a { result.push_str(&format!("Notification_A = {}\n", v)); }
        if let Some(ref v) = a.achievement_unlock_datetime_format { result.push_str(&format!("Achievement_Unlock_Datetime_Format = {}\n", v)); }
        if let Some(v) = a.background_r { result.push_str(&format!("Background_R = {}\n", v)); }
        if let Some(v) = a.background_g { result.push_str(&format!("Background_G = {}\n", v)); }
        if let Some(v) = a.background_b { result.push_str(&format!("Background_B = {}\n", v)); }
        if let Some(v) = a.background_a { result.push_str(&format!("Background_A = {}\n", v)); }
        if let Some(v) = a.element_r { result.push_str(&format!("Element_R = {}\n", v)); }
        if let Some(v) = a.element_g { result.push_str(&format!("Element_G = {}\n", v)); }
        if let Some(v) = a.element_b { result.push_str(&format!("Element_B = {}\n", v)); }
        if let Some(v) = a.element_a { result.push_str(&format!("Element_A = {}\n", v)); }
        if let Some(v) = a.element_hovered_r { result.push_str(&format!("ElementHovered_R = {}\n", v)); }
        if let Some(v) = a.element_hovered_g { result.push_str(&format!("ElementHovered_G = {}\n", v)); }
        if let Some(v) = a.element_hovered_b { result.push_str(&format!("ElementHovered_B = {}\n", v)); }
        if let Some(v) = a.element_hovered_a { result.push_str(&format!("ElementHovered_A = {}\n", v)); }
        if let Some(v) = a.element_active_r { result.push_str(&format!("ElementActive_R = {}\n", v)); }
        if let Some(v) = a.element_active_g { result.push_str(&format!("ElementActive_G = {}\n", v)); }
        if let Some(v) = a.element_active_b { result.push_str(&format!("ElementActive_B = {}\n", v)); }
        if let Some(v) = a.element_active_a { result.push_str(&format!("ElementActive_A = {}\n", v)); }
        if let Some(ref v) = a.pos_achievement { result.push_str(&format!("PosAchievement = {}\n", v)); }
        if let Some(ref v) = a.pos_invitation { result.push_str(&format!("PosInvitation = {}\n", v)); }
        if let Some(ref v) = a.pos_chat_msg { result.push_str(&format!("PosChatMsg = {}\n", v)); }
        if let Some(v) = a.stats_background_r { result.push_str(&format!("Stats_Background_R = {}\n", v)); }
        if let Some(v) = a.stats_background_g { result.push_str(&format!("Stats_Background_G = {}\n", v)); }
        if let Some(v) = a.stats_background_b { result.push_str(&format!("Stats_Background_B = {}\n", v)); }
        if let Some(v) = a.stats_background_a { result.push_str(&format!("Stats_Background_A = {}\n", v)); }
        if let Some(v) = a.stats_text_r { result.push_str(&format!("Stats_Text_R = {}\n", v)); }
        if let Some(v) = a.stats_text_g { result.push_str(&format!("Stats_Text_G = {}\n", v)); }
        if let Some(v) = a.stats_text_b { result.push_str(&format!("Stats_Text_B = {}\n", v)); }
        if let Some(v) = a.stats_text_a { result.push_str(&format!("Stats_Text_A = {}\n", v)); }
        if let Some(v) = a.stats_pos_x { result.push_str(&format!("Stats_Pos_x = {}\n", v)); }
        if let Some(v) = a.stats_pos_y { result.push_str(&format!("Stats_Pos_y = {}\n", v)); }
        result.push('\n');

        // [overlay::performance] 段
        let p = &self.performance;
        result.push_str("[overlay::performance]\n");
        result.push_str(&format!("hardware_acceleration = {}\n", p.hardware_acceleration as i32));
        result.push_str(&format!("fps_limit = {}\n", p.fps_limit));
        result.push_str(&format!("low_performance_mode = {}\n", p.low_performance_mode as i32));
        result.push('\n');

        // [overlay::features] 段
        let f = &self.features;
        result.push_str("[overlay::features]\n");
        result.push_str(&format!("achievements = {}\n", f.achievements as i32));
        result.push_str(&format!("friends = {}\n", f.friends as i32));
        result.push_str(&format!("chat = {}\n", f.chat as i32));
        result.push_str(&format!("browser = {}\n", f.browser as i32));
        result.push_str(&format!("settings = {}\n", f.settings as i32));
        result.push('\n');

        result
    }
}

// ============================================
// 2. 成就系统配置
// ============================================

/// 成就数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub description: String,
    pub hidden: bool,
    pub icon: Option<String>,
    #[serde(rename = "iconGray")]
    pub icon_gray: Option<String>,
}

/// achievements.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AchievementsConfig {
    pub enabled: bool,
    #[serde(rename = "showNotifications")]
    pub show_notifications: bool,
    pub achievements: Vec<Achievement>,
}

impl AchievementsConfig {
    #[allow(dead_code)]
    pub fn default_config() -> Self {
        Self {
            enabled: true,
            show_notifications: true,
            achievements: vec![],
        }
    }
}

// ============================================
// 3. 统计数据配置
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatItem {
    pub name: String,
    #[serde(rename = "type")]
    pub stat_type: String, // "int", "float", "avgrate", "achievements"
    #[serde(rename = "defaultValue")]
    pub default_value: f64,
    #[serde(rename = "minValue")]
    pub min_value: Option<f64>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<f64>,
    #[serde(rename = "windowSize")]
    pub window_size: Option<i32>,
}

/// stats.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsConfig {
    pub enabled: bool,
    pub stats: Vec<StatItem>,
}

impl StatsConfig {
    #[allow(dead_code)]
    pub fn default_config() -> Self {
        Self {
            enabled: true,
            stats: vec![],
        }
    }
}

// ============================================
// 4. 物品库存配置
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProperty {
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameItem {
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub quantity: i32,
    pub properties: Option<Vec<ItemProperty>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub icon: Option<String>,
    pub stackable: bool,
    #[serde(rename = "maxStackSize")]
    pub max_stack_size: i32,
}

/// items.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemsConfig {
    pub enabled: bool,
    #[serde(rename = "itemDefinitions")]
    pub item_definitions: Vec<ItemDefinition>,
    #[serde(rename = "initialItems")]
    pub initial_items: Vec<GameItem>,
}

impl ItemsConfig {
    #[allow(dead_code)]
    pub fn default_config() -> Self {
        Self {
            enabled: true,
            item_definitions: vec![],
            initial_items: vec![],
        }
    }
}

// ============================================
// 5. 创意工坊模组配置
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModFile {
    pub name: String,
    pub size: i64,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkshopMod {
    #[serde(rename = "publishedFileId")]
    pub published_file_id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "authorId")]
    pub author_id: Option<String>,
    #[serde(rename = "authorName")]
    pub author_name: Option<String>,
    #[serde(rename = "timeCreated")]
    pub time_created: Option<String>,
    #[serde(rename = "timeUpdated")]
    pub time_updated: Option<String>,
    #[serde(rename = "previewImage")]
    pub preview_image: Option<String>,
    pub files: Vec<ModFile>,
    pub tags: Option<Vec<String>>,
    pub visibility: String, // "public", "friends", "private"
}

/// mods.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModsConfig {
    pub enabled: bool,
    #[serde(rename = "subscribedMods")]
    pub subscribed_mods: Vec<WorkshopMod>,
    #[serde(rename = "autoUpdate")]
    pub auto_update: bool,
}

impl ModsConfig {
    #[allow(dead_code)]
    pub fn default_config() -> Self {
        Self {
            enabled: true,
            subscribed_mods: vec![],
            auto_update: false,
        }
    }
}

// ============================================
// 6. 排行榜配置
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i32,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    pub username: String,
    pub score: i64,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "sortMethod")]
    pub sort_method: String, // "asc", "desc"
    #[serde(rename = "displayType")]
    pub display_type: String, // "numeric", "seconds", "milliseconds"
    pub entries: Vec<LeaderboardEntry>,
}

/// leaderboards.txt (实际使用JSON格式存储)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LeaderboardsConfig {
    pub enabled: bool,
    pub leaderboards: Vec<Leaderboard>,
}

impl LeaderboardsConfig {
    pub fn default_config() -> Self {
        Self {
            enabled: true,
            leaderboards: vec![],
        }
    }

    /// 转换为 gbe_fork leaderboards.txt 格式
    /// 格式: LEADERBOARD_NAME=sort method=display type
    pub fn to_txt(&self) -> String {
        let mut result = String::new();
        for lb in &self.leaderboards {
            // gbe_fork 格式: NAME=sort=display
            result.push_str(&format!(
                "{}={}={}\n",
                lb.name, lb.sort_method, lb.display_type
            ));
        }
        result
    }
}

// ============================================
// 7. 控制器配置
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerBinding {
    pub action: String,
    pub button: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerDeadzone {
    #[serde(rename = "leftStick")]
    pub left_stick: f32,
    #[serde(rename = "rightStick")]
    pub right_stick: f32,
    #[serde(rename = "leftTrigger")]
    pub left_trigger: f32,
    #[serde(rename = "rightTrigger")]
    pub right_trigger: f32,
}

impl Default for ControllerDeadzone {
    fn default() -> Self {
        Self {
            left_stick: 0.1,
            right_stick: 0.1,
            left_trigger: 0.1,
            right_trigger: 0.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerRumble {
    pub enabled: bool,
    pub intensity: f32,
}

impl Default for ControllerRumble {
    fn default() -> Self {
        Self {
            enabled: true,
            intensity: 0.8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CustomGlyphs {
    pub enabled: bool,
    pub path: Option<String>,
}


/// controller配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerConfig {
    pub enabled: bool,
    #[serde(rename = "controllerType")]
    pub controller_type: String, // "xbox", "playstation", "nintendo", "generic"
    pub bindings: Vec<ControllerBinding>,
    pub deadzone: ControllerDeadzone,
    pub rumble: ControllerRumble,
    #[serde(rename = "customGlyphs")]
    pub custom_glyphs: CustomGlyphs,
}

impl Default for ControllerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            controller_type: "xbox".to_string(),
            bindings: vec![],
            deadzone: ControllerDeadzone::default(),
            rumble: ControllerRumble::default(),
            custom_glyphs: CustomGlyphs::default(),
        }
    }
}

// ============================================
// 8. ColdClientLoader 配置
// ============================================

/// ColdClientLoader 配置文件 (coldclientloader.ini)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColdClientLoaderConfig {
    pub enabled: bool,
    /// 注入模式: "direct" (直接注入) 或 "launcher" (启动器模式)
    #[serde(rename = "injectionMode")]
    pub injection_mode: String,
    /// 额外 DLL 列表
    #[serde(rename = "extraDlls")]
    pub extra_dlls: Vec<String>,
    /// 游戏启动参数
    #[serde(rename = "launchArgs")]
    pub launch_args: String,
    /// 游戏主程序路径
    #[serde(rename = "exePath")]
    pub exe_path: Option<String>,
    /// 工作目录
    #[serde(rename = "workingDir")]
    pub working_dir: Option<String>,
}

impl Default for ColdClientLoaderConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

impl ColdClientLoaderConfig {
    pub fn default_config() -> Self {
        Self {
            enabled: false,
            injection_mode: "direct".to_string(),
            extra_dlls: vec![],
            launch_args: String::new(),
            exe_path: None,
            working_dir: None,
        }
    }

    /// 序列化为 INI 格式
    pub fn to_ini(&self) -> String {
        let mut result = String::new();
        result.push_str("[loader]\n");
        result.push_str(&format!("enabled = {}\n", self.enabled as i32));
        result.push_str(&format!("injection_mode = {}\n", self.injection_mode));
        result.push_str(&format!("launch_args = {}\n", self.launch_args));
        
        if let Some(ref path) = self.exe_path {
            result.push_str(&format!("exe_path = {}\n", path));
        }
        
        if let Some(ref dir) = self.working_dir {
            result.push_str(&format!("working_dir = {}\n", dir));
        }
        
        if !self.extra_dlls.is_empty() {
            result.push_str("\n[extra_dlls]\n");
            for (i, dll) in self.extra_dlls.iter().enumerate() {
                result.push_str(&format!("dll{} = {}\n", i + 1, dll));
            }
        }
        
        result
    }
}

// ============================================
// 9. lobby_connect 配置
// ============================================

/// lobby_connect 配置文件 (lobby_connect.ini)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyConnectConfig {
    pub enabled: bool,
    /// 自动加入大厅
    #[serde(rename = "autoJoin")]
    pub auto_join: bool,
    /// 目标大厅 ID
    #[serde(rename = "targetLobbyId")]
    pub target_lobby_id: String,
    /// 大厅密码
    pub password: String,
    /// 自动重连
    #[serde(rename = "autoReconnect")]
    pub auto_reconnect: bool,
    /// 重连间隔(秒)
    #[serde(rename = "reconnectInterval")]
    pub reconnect_interval: i32,
}

impl Default for LobbyConnectConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

impl LobbyConnectConfig {
    pub fn default_config() -> Self {
        Self {
            enabled: false,
            auto_join: false,
            target_lobby_id: String::new(),
            password: String::new(),
            auto_reconnect: false,
            reconnect_interval: 5,
        }
    }

    /// 序列化为 INI 格式
    pub fn to_ini(&self) -> String {
        format!(
            r#"[lobby_connect]
enabled = {}
auto_join = {}
target_lobby_id = {}
password = {}
auto_reconnect = {}
reconnect_interval = {}
"#,
            self.enabled as i32,
            self.auto_join as i32,
            self.target_lobby_id,
            self.password,
            self.auto_reconnect as i32,
            self.reconnect_interval
        )
    }
}

// ============================================
// 10. 完整配置集合（简化版）
// ============================================

/// 完整的Steam设置配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct CompleteSteamSettings {
    pub main: MainConfig,
    pub user: UserConfig,
    pub app: SteamAppConfig,
    pub overlay: OverlayConfig,
    pub achievements: AchievementsConfig,
    pub stats: StatsConfig,
    pub items: ItemsConfig,
    pub mods: ModsConfig,
    pub leaderboards: LeaderboardsConfig,
    pub controller: ControllerConfig,
    pub cold_client_loader: ColdClientLoaderConfig,
    pub lobby_connect: LobbyConnectConfig,
}

impl CompleteSteamSettings {
    #[allow(dead_code)]
    pub fn default_config() -> Self {
        Self {
            main: MainConfig::default_config(),
            user: UserConfig::default_config(),
            app: SteamAppConfig::default_config(),
            overlay: OverlayConfig::default_config(),
            achievements: AchievementsConfig::default_config(),
            stats: StatsConfig::default_config(),
            items: ItemsConfig::default_config(),
            mods: ModsConfig::default_config(),
            leaderboards: LeaderboardsConfig::default_config(),
            controller: ControllerConfig::default(),
            cold_client_loader: ColdClientLoaderConfig::default_config(),
            lobby_connect: LobbyConnectConfig::default_config(),
        }
    }
}
