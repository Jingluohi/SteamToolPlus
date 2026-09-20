// Patch 页面状态持久化模型
// 用于在程序运行期间记忆免 Steam 补丁注入页面的用户输入

use serde::{Deserialize, Serialize};

/// Patch 页面持久化状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchState {
    /// 模拟器引擎模式（单机构建 / 局域网联机构建）：
    /// - `regular`：单机（仅替换 steam_api.dll，无 Overlay/局域网）
    /// - `experimental`：局域网联机 - 功能版 steam_api（experimental 构建，支持 Overlay/局域网）
    /// - `steamclient`：局域网联机 - ColdClientLoader 注入 steamclient（steamclient_experimental，支持 Overlay/局域网）
    pub emulator_mode: String,
    /// 游戏文件夹路径
    pub game_path: String,
    /// 游戏主程序 exe 路径
    pub game_exe_path: String,
    /// Steam AppID
    pub steam_app_id: String,
    /// 基础配置是否已应用
    pub basic_config_applied: bool,
    /// 脱壳是否成功
    pub unpack_success: bool,
    /// 是否使用 Steam 存档路径
    /// 勾选后应用基础配置时，将 configs.user.ini 的 local_save_path 指向
    /// <Steam>\userdata\<账户ID>（存档写入 <Steam>\userdata\<账户ID>\<AppID>\remote，与 Steam 一致），
    /// 并同时写入真实 SteamID64（account_steamid）
    #[serde(default)]
    pub use_steam_save_path: bool,
}

impl Default for PatchState {
    fn default() -> Self {
        Self {
            emulator_mode: "regular".to_string(),
            game_path: String::new(),
            game_exe_path: String::new(),
            steam_app_id: String::new(),
            basic_config_applied: false,
            unpack_success: false,
            use_steam_save_path: false,
        }
    }
}
