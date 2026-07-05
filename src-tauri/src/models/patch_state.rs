// Patch 页面状态持久化模型
// 用于在程序运行期间记忆免 Steam 补丁注入页面的用户输入

use serde::{Deserialize, Serialize};

/// Patch 页面持久化状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchState {
    /// 模拟器模式：0=标准模式，1=高级模式
    pub emulator_mode: i32,
    /// 是否使用实验版 DLL（仅在标准模式下有效）
    pub use_experimental: bool,
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
}

impl Default for PatchState {
    fn default() -> Self {
        Self {
            emulator_mode: 0,
            use_experimental: false,
            game_path: String::new(),
            game_exe_path: String::new(),
            steam_app_id: String::new(),
            basic_config_applied: false,
            unpack_success: false,
        }
    }
}
