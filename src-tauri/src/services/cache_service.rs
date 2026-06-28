// 缓存服务
// 管理 cache.json 的读写与缓存清理逻辑

use crate::models::CacheData;
use crate::utils::{config_path_utils, file_utils, resource_utils};
use std::sync::Mutex;
use tauri::AppHandle;

/// 缓存文件名
const CACHE_FILENAME: &str = "cache.json";

/// 缓存服务接口
pub trait CacheServiceTrait: Send + Sync {
    /// 添加手动导入过清单的游戏ID
    fn add_imported_manifest_game_id(&self, game_id: &str) -> Result<(), String>;
    /// 获取缓存数据
    fn get_cache(&self) -> Result<CacheData, String>;
    /// 清除导入过的清单缓存，删除对应的 resources/manifest/{game_id} 文件夹
    fn clear_imported_manifest_cache(&self, app: &AppHandle) -> Result<usize, String>;
}

/// 缓存服务实现
pub struct CacheService {
    cache: Mutex<CacheData>,
}

impl CacheService {
    /// 创建新的缓存服务实例
    pub fn new() -> Self {
        let cache = Self::load_cache();
        Self {
            cache: Mutex::new(cache),
        }
    }

    /// 获取运行时缓存文件路径
    fn get_cache_path() -> Result<String, String> {
        let path = config_path_utils::get_runtime_config_path(CACHE_FILENAME)?;
        Ok(path.to_string_lossy().to_string())
    }

    /// 从文件加载缓存
    fn load_cache() -> CacheData {
        match Self::get_cache_path() {
            Ok(path) => file_utils::read_json_file::<CacheData>(&path).unwrap_or_default(),
            Err(_) => CacheData::default(),
        }
    }

    /// 保存缓存到文件
    fn save_cache(&self, cache: &CacheData) -> Result<(), String> {
        let path = Self::get_cache_path()?;
        config_path_utils::ensure_runtime_config_dir()?;
        file_utils::write_json_file(&path, cache)
    }
}

impl CacheServiceTrait for CacheService {
    /// 添加手动导入过清单的游戏ID
    /// 如果已存在则不去重添加，保持记录顺序
    fn add_imported_manifest_game_id(&self, game_id: &str) -> Result<(), String> {
        let mut cache = self.cache.lock().map_err(|e| format!("缓存锁被污染: {}", e))?;

        if !cache.imported_manifest_game_ids.contains(&game_id.to_string()) {
            cache.imported_manifest_game_ids.push(game_id.to_string());
            let cache_clone = cache.clone();
            drop(cache);
            self.save_cache(&cache_clone)?;
        }

        Ok(())
    }

    /// 获取缓存数据
    fn get_cache(&self) -> Result<CacheData, String> {
        let cache = self.cache.lock().map_err(|e| format!("缓存锁被污染: {}", e))?;
        Ok(cache.clone())
    }

    /// 清除导入过的清单缓存
    /// 删除 resources/manifest/{game_id} 下所有记录的文件夹，并清空 cache.json
    fn clear_imported_manifest_cache(&self, app: &AppHandle) -> Result<usize, String> {
        let resource_dir = resource_utils::get_resource_dir(app)?;
        let manifest_dir = resource_dir.join("manifest");

        let game_ids = {
            let cache = self.cache.lock().map_err(|e| format!("缓存锁被污染: {}", e))?;
            cache.imported_manifest_game_ids.clone()
        };

        let mut deleted_count = 0usize;
        for game_id in &game_ids {
            let game_manifest_dir = manifest_dir.join(game_id);
            if game_manifest_dir.exists() {
                match std::fs::remove_dir_all(&game_manifest_dir) {
                    Ok(_) => {
                        deleted_count += 1;
                        log::info!("已删除清单缓存目录: {}", game_manifest_dir.display());
                    }
                    Err(e) => {
                        log::warn!("删除清单缓存目录失败 {}: {}", game_manifest_dir.display(), e);
                    }
                }
            }
        }

        // 清空记录并保存
        {
            let mut cache = self.cache.lock().map_err(|e| format!("缓存锁被污染: {}", e))?;
            cache.imported_manifest_game_ids.clear();
            let cache_clone = cache.clone();
            drop(cache);
            self.save_cache(&cache_clone)?;
        }

        Ok(deleted_count)
    }
}

impl Default for CacheService {
    fn default() -> Self {
        Self::new()
    }
}
