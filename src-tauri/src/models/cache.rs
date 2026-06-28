// 缓存数据模型
// 定义应用程序缓存相关的数据结构

use serde::{Deserialize, Serialize};

/// 缓存数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheData {
    /// 手动导入过清单文件的游戏ID列表
    pub imported_manifest_game_ids: Vec<String>,
}

impl Default for CacheData {
    fn default() -> Self {
        Self {
            imported_manifest_game_ids: Vec::new(),
        }
    }
}
