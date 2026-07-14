// 假入库游戏管理服务
// 管理通过本程序入库到 Steam 的游戏记录，支持安全删除

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::config_path_utils;
use crate::utils::file_utils;

/// 假入库游戏记录文件名
const FAKE_IMPORTED_GAMES_FILENAME: &str = "fake_imported_games.json";

/// 假入库游戏信息（返回给前端）
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FakeImportedGameInfo {
    /// 游戏 AppID
    pub app_id: u32,
    /// 显示名称：如果在 games_config.json 中匹配到则使用中文名，否则使用 AppID 字符串
    pub display_name: String,
    /// 是否在 games_config.json 中存在
    pub in_config: bool,
}

/// 假入库游戏记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct FakeImportedGamesRecord {
    /// 已入库游戏的 AppID 列表
    app_ids: Vec<u32>,
}

impl Default for FakeImportedGamesRecord {
    fn default() -> Self {
        Self { app_ids: Vec::new() }
    }
}

/// 获取假入库游戏记录文件路径
fn get_record_path() -> Result<PathBuf, String> {
    config_path_utils::get_runtime_config_path(FAKE_IMPORTED_GAMES_FILENAME)
}

/// 读取记录文件
fn load_record() -> Result<FakeImportedGamesRecord, String> {
    let path = get_record_path()?;

    if !path.exists() {
        return Ok(FakeImportedGamesRecord::default());
    }

    match file_utils::read_json_file::<FakeImportedGamesRecord>(&path.to_string_lossy()) {
        Ok(record) => Ok(record),
        Err(e) => {
            log::error!("读取假入库游戏记录失败: {}，将使用空记录", e);
            Ok(FakeImportedGamesRecord::default())
        }
    }
}

/// 保存记录文件
fn save_record(record: &FakeImportedGamesRecord) -> Result<(), String> {
    let path = get_record_path()?;
    config_path_utils::ensure_runtime_config_dir()?;
    file_utils::write_json_file(&path.to_string_lossy(), record)?;
    let _ = config_path_utils::sync_to_backup(FAKE_IMPORTED_GAMES_FILENAME);
    Ok(())
}

/// 加载 games_config.json 中的游戏名称映射
/// 返回 app_id 到中文名的映射
fn load_game_names() -> HashMap<u32, String> {
    let mut result = HashMap::new();

    // 尝试从程序根目录读取 games_config.json
    if let Ok(exe_dir) = config_path_utils::get_exe_dir() {
        let config_path = exe_dir.join("resources").join("games_config.json");
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(games) = serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                    for game in games {
                        if let (Some(game_id), Some(chinese_name)) = (
                            game.get("game_id").and_then(|v| v.as_str()),
                            game.get("chinese_name").and_then(|v| v.as_str()),
                        ) {
                            if let Ok(app_id) = game_id.parse::<u32>() {
                                result.insert(app_id, chinese_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

/// 获取所有假入库游戏信息
pub fn get_fake_imported_games() -> Result<Vec<FakeImportedGameInfo>, String> {
    let record = load_record()?;
    let name_map = load_game_names();

    let games: Vec<FakeImportedGameInfo> = record
        .app_ids
        .iter()
        .map(|&app_id| {
            let in_config = name_map.contains_key(&app_id);
            let display_name = name_map
                .get(&app_id)
                .cloned()
                .unwrap_or_else(|| app_id.to_string());

            FakeImportedGameInfo {
                app_id,
                display_name,
                in_config,
            }
        })
        .collect();

    Ok(games)
}

/// 添加一个假入库游戏记录
pub fn add_fake_imported_game(app_id: u32) -> Result<(), String> {
    if app_id == 0 {
        return Err("AppID 不能为 0".to_string());
    }

    let mut record = load_record()?;

    if !record.app_ids.contains(&app_id) {
        record.app_ids.push(app_id);
        // 保持排序，便于展示和比较
        record.app_ids.sort();
        save_record(&record)?;
    }

    Ok(())
}

/// 移除一个假入库游戏记录
/// 注意：此方法只从记录中移除，不清理 Steam 文件
pub fn remove_fake_imported_game_record(app_id: u32) -> Result<(), String> {
    let mut record = load_record()?;
    record.app_ids.retain(|&id| id != app_id);
    save_record(&record)
}

/// 检查 Steam 库中是否存在该游戏的正版购买记录
/// 通过检查 appmanifest_{app_id}.acf 文件是否存在来判断
pub fn has_real_steam_ownership(steam_path: &str, app_id: u32) -> bool {
    let manifest_path = Path::new(steam_path)
        .join("steamapps")
        .join(format!("appmanifest_{}.acf", app_id));

    manifest_path.exists()
}

/// 删除 OpenSteamTool 为该游戏写入的文件
/// 包括：Steam/config/lua/{app_id}.lua 和相关 depotcache 文件
/// 返回删除的文件列表
pub fn cleanup_fake_imported_files(
    steam_path: &str,
    app_id: u32,
) -> Result<Vec<String>, String> {
    let mut deleted_files = Vec::new();
    let steam_path = Path::new(steam_path);

    // 1. 删除 Lua 文件
    let lua_file = steam_path.join("config").join("lua").join(format!("{}.lua", app_id));
    if lua_file.exists() {
        match fs::remove_file(&lua_file) {
            Ok(_) => {
                log::info!("已删除假入库 Lua 文件: {}", lua_file.display());
                deleted_files.push(lua_file.to_string_lossy().to_string());
            }
            Err(e) => {
                log::warn!("删除 Lua 文件失败 {}: {}", lua_file.display(), e);
            }
        }
    }

    // 2. 删除 depotcache 中可能相关的 manifest 文件
    // manifest 文件名通常与 app_id 相关，格式不固定，这里删除包含 app_id 的文件
    let depotcache_dir = steam_path.join("config").join("depotcache");
    if depotcache_dir.exists() {
        if let Ok(entries) = fs::read_dir(&depotcache_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    // 匹配文件名中包含 app_id 的 manifest 文件
                    if file_name.contains(&app_id.to_string()) {
                        match fs::remove_file(entry.path()) {
                            Ok(_) => {
                                log::info!("已删除假入库 manifest 文件: {}", entry.path().display());
                                deleted_files.push(entry.path().to_string_lossy().to_string());
                            }
                            Err(e) => {
                                log::warn!("删除 manifest 文件失败 {}: {}", entry.path().display(), e);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(deleted_files)
}
