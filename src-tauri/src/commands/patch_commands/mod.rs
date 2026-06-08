// 补丁注入命令模块
// 100% 实现 gbe_fork 所有配置文件的读写支持
// 
// 模块结构：
// - common: 公共类型定义和工具函数
// - file_ops: 文件操作（复制目录、重命名、备份等）
// - steamless: Steamless 脱壳功能
// - patch_apply: 补丁应用（DLL 复制、steam_settings 生成）
// - config_core: 核心配置（main、user、app、overlay）
// - game_features: 游戏功能配置（成就、统计、物品、模组、排行榜、控制器）
// - misc_config: 杂项配置（DLC、depots、branches、groups、languages 等）
// - tools: 工具集成（ColdClientLoader、lobby_connect、generate_interfaces）

mod common;
mod file_ops;
mod steamless;
mod patch_apply;
mod config_core;
mod game_features;
mod misc_config;
mod tools;

// 重新导出所有公开命令
pub use common::*;
pub use file_ops::*;
pub use steamless::*;
pub use patch_apply::*;
pub use config_core::*;
pub use game_features::*;
pub use misc_config::*;
pub use tools::*;
