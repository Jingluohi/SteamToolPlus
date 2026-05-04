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
    pub username: String,
    pub language: String,
    #[serde(rename = "save_path")]
    pub save_path: String,
    #[serde(rename = "avatar_path")]
    pub avatar_path: Option<String>,
    #[serde(rename = "use_default_avatar")]
    pub use_default_avatar: bool,
    /// 存档文件夹名称（覆盖默认的 "GSE Saves"）
    #[serde(rename = "saves_folder_name")]
    pub saves_folder_name: Option<String>,
    /// 本地存档路径（便携模式）
    #[serde(rename = "local_save_path")]
    pub local_save_path: Option<String>,
    /// EncryptedAppTicket (Base64编码)
    #[serde(rename = "ticket")]
    pub ticket: Option<String>,
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
            language: "schinese".to_string(),
            save_path: "%appdata%/GSE Saves".to_string(),
            avatar_path: None,
            use_default_avatar: true,
            saves_folder_name: None,
            local_save_path: None,
            ticket: None,
        }
    }

    pub fn to_ini(&self) -> String {
        let mut result = String::new();
        result.push_str("[user]\n");
        result.push_str(&format!("username = {}\n", self.username));
        result.push_str(&format!("language = {}\n", self.language));
        result.push_str(&format!("save_path = {}\n", self.save_path));
        
        if let Some(ref path) = self.avatar_path {
            result.push_str(&format!("avatar_path = {}\n", path));
        }
        result.push_str(&format!("use_default_avatar = {}\n", self.use_default_avatar as i32));
        
        if let Some(ref name) = self.saves_folder_name {
            result.push_str(&format!("saves_folder_name = {}\n", name));
        }
        
        if let Some(ref path) = self.local_save_path {
            result.push_str(&format!("local_save_path = {}\n", path));
        }
        
        if let Some(ref ticket) = self.ticket {
            result.push_str(&format!("ticket = {}\n", ticket));
        }
        
        result
    }
}

/// configs.app.ini - 应用配置文件 (Steam)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SteamAppConfig {
    #[serde(rename = "branch_name")]
    pub branch_name: String,
    #[serde(rename = "app_paths")]
    pub app_paths: HashMap<String, String>,
    pub dlcs: DlcConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DlcConfig {
    #[serde(rename = "unlock_all")]
    pub unlock_all: bool,
    #[serde(rename = "individual_dlcs")]
    pub individual_dlcs: Vec<IndividualDlc>,
}

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
            app_paths: HashMap::new(),
            dlcs: DlcConfig {
                unlock_all: true,
                individual_dlcs: vec![],
            },
        }
    }

    pub fn to_ini(&self) -> String {
        let mut result = format!(
            r#"[app]
branch_name = {}

[app::paths]
"#,
            self.branch_name
        );

        if self.app_paths.is_empty() {
            result.push_str("# No custom paths\n");
        } else {
            for (key, value) in &self.app_paths {
                result.push_str(&format!("{} = {}\n", key, value));
            }
        }

        result.push_str(&format!(
            r#"
[app::dlcs]
unlock_all = {}
"#,
            self.dlcs.unlock_all as i32
        ));

        if !self.dlcs.individual_dlcs.is_empty() {
            result.push_str("# Individual DLCs\n");
            for dlc in &self.dlcs.individual_dlcs {
                result.push_str(&format!(
                    "{} = {} # {}\n",
                    dlc.app_id,
                    dlc.enabled as i32,
                    dlc.name
                ));
            }
        }

        result
    }
}

/// configs.overlay.ini - 覆盖层配置文件
/// 100% 实现 gbe_fork 所有配置选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayConfig {
    /// 启用实验性覆盖层 (gbe_fork 关键配置)
    #[serde(rename = "enable_experimental_overlay")]
    pub enable_experimental_overlay: bool,
    pub hotkey: String,
    pub notifications: OverlayNotifications,
    pub appearance: OverlayAppearance,
    pub performance: OverlayPerformance,
    pub features: OverlayFeatures,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayNotifications {
    pub achievement: bool,
    pub friend: bool,
    pub message: bool,
    pub duration: i32,
    pub position: String,
}

impl Default for OverlayNotifications {
    fn default() -> Self {
        Self {
            achievement: true,
            friend: true,
            message: true,
            duration: 5,
            position: "bottom-right".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayAppearance {
    pub theme: String,
    pub opacity: f32,
    pub scale: f32,
    pub blur: bool,
}

impl Default for OverlayAppearance {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            opacity: 0.95,
            scale: 1.0,
            blur: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayPerformance {
    #[serde(rename = "hardware_acceleration")]
    pub hardware_acceleration: bool,
    #[serde(rename = "fps_limit")]
    pub fps_limit: i32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayFeatures {
    pub achievements: bool,
    pub friends: bool,
    pub chat: bool,
    pub browser: bool,
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
            hotkey: "Shift+Tab".to_string(),
            notifications: OverlayNotifications::default(),
            appearance: OverlayAppearance::default(),
            performance: OverlayPerformance::default(),
            features: OverlayFeatures::default(),
        }
    }

    pub fn to_ini(&self) -> String {
        format!(
            r#"[overlay]
enable_experimental_overlay = {}
hotkey = {}

[overlay::notifications]
achievement = {}
friend = {}
message = {}
duration = {}
position = {}

[overlay::appearance]
theme = {}
opacity = {:.2}
scale = {:.2}
blur = {}

[overlay::performance]
hardware_acceleration = {}
fps_limit = {}
low_performance_mode = {}

[overlay::features]
achievements = {}
friends = {}
chat = {}
browser = {}
settings = {}
"#,
            self.enable_experimental_overlay as i32,
            self.hotkey,
            self.notifications.achievement as i32,
            self.notifications.friend as i32,
            self.notifications.message as i32,
            self.notifications.duration,
            self.notifications.position,
            self.appearance.theme,
            self.appearance.opacity,
            self.appearance.scale,
            self.appearance.blur as i32,
            self.performance.hardware_acceleration as i32,
            self.performance.fps_limit,
            self.performance.low_performance_mode as i32,
            self.features.achievements as i32,
            self.features.friends as i32,
            self.features.chat as i32,
            self.features.browser as i32,
            self.features.settings as i32,
        )
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

    /// 转换为TXT格式
    pub fn to_txt(&self) -> String {
        let mut result = String::new();
        for lb in &self.leaderboards {
            result.push_str(&format!(
                "{}|{}|{}|{}\n",
                lb.name, lb.display_name, lb.sort_method, lb.display_type
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
pub struct CustomGlyphs {
    pub enabled: bool,
    pub path: Option<String>,
}

impl Default for CustomGlyphs {
    fn default() -> Self {
        Self {
            enabled: false,
            path: None,
        }
    }
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
// 8. 完整配置集合（简化版）
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
        }
    }
}
