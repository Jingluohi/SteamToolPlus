// Patch 页面状态持久化命令
// 在程序运行期间记忆免 Steam 补丁注入页面的基础状态

use crate::models::PatchState;
use crate::utils::{config_path_utils, file_utils};

const PATCH_STATE_FILENAME: &str = "patch_state.json";

/// 加载 Patch 页面状态
#[tauri::command]
pub async fn load_patch_state() -> Result<PatchState, String> {
    // 尝试从备份恢复
    let _ = config_path_utils::restore_from_backup(PATCH_STATE_FILENAME);

    let path = config_path_utils::get_runtime_config_path(PATCH_STATE_FILENAME)?;

    if !path.exists() {
        return Ok(PatchState::default());
    }

    match file_utils::read_json_file::<PatchState>(&path.to_string_lossy()) {
        Ok(state) => Ok(state),
        Err(_) => Ok(PatchState::default()),
    }
}

/// 保存 Patch 页面状态
#[tauri::command]
pub async fn save_patch_state(state: PatchState) -> Result<(), String> {
    let path = config_path_utils::get_runtime_config_path(PATCH_STATE_FILENAME)?;

    config_path_utils::ensure_runtime_config_dir()?;
    file_utils::write_json_file(&path.to_string_lossy(), &state)?;

    // 同步到备份目录
    let _ = config_path_utils::sync_to_backup(PATCH_STATE_FILENAME);

    Ok(())
}
