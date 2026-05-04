// 游戏数据管理服务
// 管理已下载和导入的游戏数据，存储在 %APPDATA%/SteamTool Plus/config/game.json
// 备份存储在程序根目录的 config/game.json

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tauri::AppHandle;
use crate::utils::config_path_utils;

/// 游戏数据文件名
const GAME_DATA_FILENAME: &str = "game.json";

/// 游戏数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    /// 游戏ID
    pub game_id: String,
    /// 游戏名称（英文）
    pub game_name: String,
    /// 游戏中文名
    pub chinese_name: String,
    /// 游戏类型：downloaded(下载的) 或 imported(导入的)
    pub game_type: String,
    /// 游戏安装路径
    pub install_path: String,
    /// 游戏主程序exe路径
    pub exe_path: String,
    /// 游戏存档路径（可选）
    pub save_path: Option<String>,
    /// 游戏封面路径（可选）
    pub cover_path: Option<String>,
    /// Steam游戏ID（可选）
    pub steam_game_id: Option<String>,
    /// 是否收藏
    #[serde(default)]
    pub is_favorite: bool,
    /// 是否已安装（用于库页面显示控制）
    #[serde(default = "default_true")]
    pub is_installed: bool,
    /// 总游玩时长（分钟）
    pub play_time: u64,
    /// 最后游玩日期（ISO格式）
    pub last_played: Option<String>,
    /// 添加日期（ISO格式）
    pub added_date: String,
    /// 下载状态：idle, downloading, paused, completed, error
    pub download_status: String,
    /// 下载进度（0-100）
    pub download_progress: u32,
    /// 下载路径（仅下载的游戏）
    pub download_path: Option<String>,
}

/// 默认值为 true（用于 is_installed 字段）
fn default_true() -> bool {
    true
}

/// 游戏数据集合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataCollection {
    /// 游戏数据映射
    pub games: HashMap<String, GameData>,
    /// 最后更新时间
    pub last_updated: String,
}

impl Default for GameDataCollection {
    fn default() -> Self {
        Self {
            games: HashMap::new(),
            last_updated: chrono::Local::now().to_rfc3339(),
        }
    }
}

/// 获取游戏数据文件路径（运行时目录）
fn get_game_data_path() -> Result<std::path::PathBuf, String> {
    config_path_utils::get_runtime_config_path(GAME_DATA_FILENAME)
}

/// 确保数据目录存在
async fn ensure_data_dir_exists() -> Result<(), String> {
    config_path_utils::ensure_runtime_config_dir()
}

/// 从备份恢复游戏数据
pub fn restore_game_data_from_backup() -> Result<bool, String> {
    config_path_utils::restore_from_backup(GAME_DATA_FILENAME)
}

/// 同步游戏数据到备份
pub fn sync_game_data_to_backup() -> Result<(), String> {
    config_path_utils::sync_to_backup(GAME_DATA_FILENAME)
}

/// 加载游戏数据
pub async fn load_game_data(_app: AppHandle) -> Result<GameDataCollection, String> {
    // 尝试从备份恢复
    let _ = restore_game_data_from_backup();

    let data_path = get_game_data_path()?;

    // 如果文件不存在，返回空集合
    if !data_path.exists() {
        return Ok(GameDataCollection::default());
    }

    // 读取文件内容
    let content = fs::read_to_string(&data_path)
        .await
        .map_err(|e| format!("读取游戏数据文件失败: {}", e))?;

    // 解析JSON
    let collection: GameDataCollection = serde_json::from_str(&content)
        .map_err(|e| format!("解析游戏数据失败: {}", e))?;

    Ok(collection)
}

/// 保存游戏数据
pub async fn save_game_data(_app: AppHandle, collection: &GameDataCollection) -> Result<(), String> {
    ensure_data_dir_exists().await?;

    let data_path = get_game_data_path()?;

    // 序列化为JSON
    let content = serde_json::to_string_pretty(collection)
        .map_err(|e| format!("序列化游戏数据失败: {}", e))?;

    // 写入文件
    let mut file = fs::File::create(&data_path)
        .await
        .map_err(|e| format!("创建游戏数据文件失败: {}", e))?;

    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入游戏数据失败: {}", e))?;

    // 同步到备份
    sync_game_data_to_backup()?;

    Ok(())
}

/// 添加或更新游戏
pub async fn upsert_game(app: AppHandle, game: GameData) -> Result<(), String> {
    let mut collection = load_game_data(app.clone()).await?;

    // 更新最后更新时间
    collection.last_updated = chrono::Local::now().to_rfc3339();

    // 插入或更新游戏数据
    collection.games.insert(game.game_id.clone(), game);

    // 保存
    save_game_data(app, &collection).await?;

    Ok(())
}

/// 删除游戏
pub async fn remove_game(app: AppHandle, game_id: String) -> Result<(), String> {
    let mut collection = load_game_data(app.clone()).await?;

    collection.games.remove(&game_id);
    collection.last_updated = chrono::Local::now().to_rfc3339();

    save_game_data(app, &collection).await?;

    Ok(())
}

/// 获取单个游戏数据
pub async fn get_game(app: AppHandle, game_id: String) -> Result<Option<GameData>, String> {
    let collection = load_game_data(app).await?;
    Ok(collection.games.get(&game_id).cloned())
}

/// 获取所有游戏数据
pub async fn get_all_games(app: AppHandle) -> Result<Vec<GameData>, String> {
    let collection = load_game_data(app).await?;
    Ok(collection.games.values().cloned().collect())
}

/// 更新下载状态
pub async fn update_download_status(
    app: AppHandle,
    game_id: String,
    status: String,
    progress: u32,
) -> Result<(), String> {
    let mut collection = load_game_data(app.clone()).await?;

    if let Some(game) = collection.games.get_mut(&game_id) {
        game.download_status = status;
        game.download_progress = progress;
        game.last_played = Some(chrono::Local::now().to_rfc3339());
        collection.last_updated = chrono::Local::now().to_rfc3339();

        save_game_data(app, &collection).await?;
    }

    Ok(())
}

/// 更新游戏时长
pub async fn update_play_time(
    app: AppHandle,
    game_id: String,
    additional_minutes: u64,
) -> Result<(), String> {
    let mut collection = load_game_data(app.clone()).await?;

    if let Some(game) = collection.games.get_mut(&game_id) {
        game.play_time += additional_minutes;
        game.last_played = Some(chrono::Local::now().to_rfc3339());
        collection.last_updated = chrono::Local::now().to_rfc3339();

        save_game_data(app, &collection).await?;
    }

    Ok(())
}

/// 检查游戏是否已存在
pub async fn game_exists(app: AppHandle, game_id: String) -> Result<bool, String> {
    let collection = load_game_data(app).await?;
    Ok(collection.games.contains_key(&game_id))
}

/// 切换游戏收藏状态
pub async fn toggle_game_favorite(app: AppHandle, game_id: String) -> Result<GameData, String> {
    let mut collection = load_game_data(app.clone()).await?;

    // 先检查游戏是否存在
    if !collection.games.contains_key(&game_id) {
        return Err(format!("游戏不存在: {}", game_id));
    }

    // 切换收藏状态
    if let Some(game) = collection.games.get_mut(&game_id) {
        game.is_favorite = !game.is_favorite;
        collection.last_updated = chrono::Local::now().to_rfc3339();
    }

    // 获取更新后的游戏数据副本
    let updated_game = collection.games.get(&game_id).cloned()
        .ok_or_else(|| format!("游戏不存在: {}", game_id))?;

    // 保存更新后的数据
    save_game_data(app, &collection).await?;

    // 返回更新后的游戏数据
    Ok(updated_game)
}


