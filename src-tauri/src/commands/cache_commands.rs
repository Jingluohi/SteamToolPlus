// 缓存命令
// 处理缓存相关的IPC调用

use crate::AppState;
use tauri::State;

/// 添加手动导入过清单的游戏ID到缓存
#[tauri::command]
pub fn add_imported_manifest_game_id(
    state: State<AppState>,
    game_id: String,
) -> Result<(), String> {
    state.cache_service.add_imported_manifest_game_id(&game_id)
}

/// 获取缓存数据
#[tauri::command]
pub fn get_cache(state: State<AppState>) -> Result<crate::models::CacheData, String> {
    state.cache_service.get_cache()
}

/// 清除导入过的清单缓存
/// 返回成功删除的文件夹数量
#[tauri::command]
pub fn clear_imported_manifest_cache(
    state: State<AppState>,
    app: tauri::AppHandle,
) -> Result<usize, String> {
    state.cache_service.clear_imported_manifest_cache(&app)
}
