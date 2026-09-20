// 假入库游戏管理命令
// 提供前端调用的 IPC 接口，管理通过本程序入库的 Steam 游戏

use crate::services::fake_imported_service::{
    add_fake_imported_game, cleanup_fake_imported_files, get_fake_imported_games,
    has_real_steam_ownership, remove_fake_imported_game_record, FakeImportedGameInfo,
};
use crate::services::opensteamtool_service::detect_steam_path;

/// 获取所有假入库游戏列表
#[tauri::command]
pub fn get_fake_imported_games_command() -> Result<Vec<FakeImportedGameInfo>, String> {
    get_fake_imported_games()
}

/// 添加一个假入库游戏记录
/// 通常在 OpenSteamTool 入库成功后自动调用
#[tauri::command]
pub fn add_fake_imported_game_command(app_id: u32) -> Result<serde_json::Value, String> {
    add_fake_imported_game(app_id)?;
    Ok(serde_json::json!({
        "success": true,
        "message": format!("已记录假入库游戏: {}", app_id)
    }))
}

/// 移除假入库游戏
/// 安全策略：
/// 1. 检查 Steam 库中是否存在 appmanifest_{app_id}.acf（该游戏当前已安装，可能为假入库安装产生）
/// 2. 如果存在，拒绝删除并提示用户先在 Steam 中卸载
/// 3. 如果不存在，删除 OpenSteamTool 写入的 Lua 和 manifest 文件，并从记录中移除
#[tauri::command]
pub fn remove_fake_imported_game_command(
    app_id: u32,
    steam_path: Option<String>,
) -> Result<serde_json::Value, String> {
    // 1. 获取 Steam 路径
    let steam_path = match detect_steam_path(steam_path.as_deref()) {
        Ok(path) => path,
        Err(e) => {
            return Err(format!("无法获取 Steam 路径: {}，请先在全局设置中配置 Steam 路径", e));
        }
    };

    // 2. 安全检查：Steam 库中是否仍存在该游戏的安装记录
    // 假入库游戏在 Steam 中安装后同样会生成 appmanifest_{app_id}.acf，
    // 此时若删除入库文件，游戏会变为「未购买」且无法游玩，因此必须让用户先卸载
    if has_real_steam_ownership(&steam_path, app_id) {
        return Ok(serde_json::json!({
            "success": false,
            "message": format!(
                "无法删除 AppID {}：该游戏当前已在 Steam 库中安装。若删除入库，游戏将显示为「未购买」且无法游玩。如确需删除入库，请先在 Steam 库中卸载该游戏，卸载后即可删除",
                app_id
            ),
            "hasRealOwnership": true
        }));
    }

    // 3. 清理 OpenSteamTool 写入的文件
    let deleted_files = cleanup_fake_imported_files(&steam_path, app_id)?;

    // 4. 从记录中移除
    remove_fake_imported_game_record(app_id)?;

    Ok(serde_json::json!({
        "success": true,
        "message": format!("已移除假入库游戏: {}，清理文件 {} 个", app_id, deleted_files.len()),
        "deletedFiles": deleted_files
    }))
}
