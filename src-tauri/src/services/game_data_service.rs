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
/// 具有容错机制：如果主文件损坏，自动尝试从备份恢复
pub async fn load_game_data(_app: AppHandle) -> Result<GameDataCollection, String> {
    // 尝试从备份恢复
    let _ = restore_game_data_from_backup();

    let data_path = get_game_data_path()?;
    let backup_path = data_path.with_extension("backup");

    // 如果文件不存在，返回空集合
    if !data_path.exists() {
        return Ok(GameDataCollection::default());
    }

    // 尝试读取主文件
    let content = match fs::read_to_string(&data_path).await {
        Ok(content) => content,
        Err(e) => {
            // 主文件读取失败，尝试从备份恢复
            if backup_path.exists() {
                match fs::read_to_string(&backup_path).await {
                    Ok(backup_content) => {
                        // 备份读取成功，尝试恢复
                        let _ = fs::copy(&backup_path, &data_path).await;
                        backup_content
                    }
                    Err(_) => {
                        return Err(format!("读取游戏数据文件失败: {}", e));
                    }
                }
            } else {
                return Err(format!("读取游戏数据文件失败: {}", e));
            }
        }
    };

    // 尝试解析JSON
    let collection: GameDataCollection = match serde_json::from_str(&content) {
        Ok(collection) => collection,
        Err(_e) => {
            // 主文件解析失败，尝试从备份恢复
            if backup_path.exists() {
                match fs::read_to_string(&backup_path).await {
                    Ok(backup_content) => {
                        match serde_json::from_str(&backup_content) {
                            Ok(backup_collection) => {
                                // 备份解析成功，恢复备份
                                let _ = fs::copy(&backup_path, &data_path).await;
                                backup_collection
                            }
                            Err(_) => {
                                // 备份也损坏，返回空集合并记录错误
                                return Ok(GameDataCollection::default());
                            }
                        }
                    }
                    Err(_) => {
                        // 备份读取失败，返回空集合
                        return Ok(GameDataCollection::default());
                    }
                }
            } else {
                // 没有备份，返回空集合
                return Ok(GameDataCollection::default());
            }
        }
    };

    Ok(collection)
}

/// 保存游戏数据
/// 使用原子写入机制：先写入临时文件，成功后再重命名为正式文件
/// 这样即使程序崩溃，也不会留下损坏的正式文件
pub async fn save_game_data(_app: AppHandle, collection: &GameDataCollection) -> Result<(), String> {
    ensure_data_dir_exists().await?;

    let data_path = get_game_data_path()?;
    
    // 创建临时文件路径（在同一目录下，确保重命名是原子操作）
    let temp_path = data_path.with_extension("tmp");
    let backup_path = data_path.with_extension("backup");

    // 序列化为JSON
    let content = serde_json::to_string_pretty(collection)
        .map_err(|e| format!("序列化游戏数据失败: {}", e))?;

    // 步骤1：如果存在旧的临时文件，先删除
    if temp_path.exists() {
        let _ = fs::remove_file(&temp_path).await;
    }

    // 步骤2：写入临时文件
    let mut file = fs::File::create(&temp_path)
        .await
        .map_err(|e| format!("创建临时游戏数据文件失败: {}", e))?;

    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入临时游戏数据失败: {}", e))?;

    // 步骤3：确保数据完全写入磁盘（fsync）
    file.sync_all()
        .await
        .map_err(|e| format!("同步游戏数据到磁盘失败: {}", e))?;

    // 步骤4：如果存在旧的正式文件，先备份
    if data_path.exists() {
        let _ = fs::rename(&data_path, &backup_path).await;
    }

    // 步骤5：原子重命名临时文件为正式文件
    fs::rename(&temp_path, &data_path)
        .await
        .map_err(|e| {
            // 重命名失败，尝试从备份恢复
            if backup_path.exists() {
                let _ = std::fs::rename(&backup_path, &data_path);
            }
            format!("保存游戏数据失败: {}", e)
        })?;

    // 步骤6：删除备份文件（保存成功后才删除）
    if backup_path.exists() {
        let _ = fs::remove_file(&backup_path).await;
    }

    // 步骤7：同步到备份目录
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

/// 下载完成后收尾处理
/// 扫描下载目录，自动定位游戏主程序并标记为已安装
pub async fn finalize_download(app: AppHandle, game_id: String) -> Result<GameData, String> {
    let mut collection = load_game_data(app.clone()).await?;

    // 获取游戏数据
    let game = collection.games.get(&game_id)
        .cloned()
        .ok_or_else(|| format!("游戏不存在: {}", game_id))?;

    // 确定要扫描的根目录：优先使用下载路径
    let scan_root = game.download_path.as_ref()
        .filter(|p| !p.is_empty() && std::path::Path::new(p).exists())
        .or_else(|| Some(&game.install_path))
        .filter(|p| !p.is_empty() && std::path::Path::new(p).exists())
        .cloned()
        .ok_or_else(|| "未找到有效的下载目录".to_string())?;

    // 扫描目录寻找游戏主程序
    let best_exe = find_best_game_exe(&scan_root, &game.game_name);

    // 更新游戏数据
    if let Some(game) = collection.games.get_mut(&game_id) {
        game.download_status = "completed".to_string();
        game.download_progress = 100;

        if let Some((exe_path, _exe_dir)) = best_exe {
            game.exe_path = exe_path;
            // install_path 保持为下载/安装根目录，与 exe 所在目录区分开
            game.install_path = scan_root.clone();
            game.is_installed = true;
        } else {
            // 即使没有找到 exe，只要目录存在也标记为已安装
            // 用户可以后续手动编辑 exe 路径
            game.install_path = scan_root.clone();
            game.is_installed = true;
        }

        game.last_played = Some(chrono::Local::now().to_rfc3339());
        collection.last_updated = chrono::Local::now().to_rfc3339();
    }

    // 保存更新后的数据
    save_game_data(app, &collection).await?;

    // 返回更新后的游戏数据
    collection.games.get(&game_id)
        .cloned()
        .ok_or_else(|| format!("游戏不存在: {}", game_id))
}

/// 在指定目录中查找最合适的游戏主程序
/// 返回 (exe 完整路径, 所在目录) 或 None
pub fn find_best_game_exe(root: &str, game_name: &str) -> Option<(String, String)> {
    let root_path = std::path::Path::new(root);
    if !root_path.exists() || !root_path.is_dir() {
        return None;
    }

    // 使用栈进行非递归扫描，限制最大深度为 2
    let mut candidates: Vec<(std::path::PathBuf, usize, u64)> = Vec::new();
    let mut dirs_to_scan: Vec<(std::path::PathBuf, usize)> = vec![(root_path.to_path_buf(), 0)];

    while let Some((current_dir, depth)) = dirs_to_scan.pop() {
        let entries = match std::fs::read_dir(&current_dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                // 检查是否为 .exe 文件
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case("exe") {
                        // 排除明显的辅助程序
                        let file_name = path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        if is_helper_exe(&file_name) {
                            continue;
                        }

                        // 获取文件大小
                        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                        candidates.push((path, depth, size));
                    }
                }
            } else if path.is_dir() && depth < 2 {
                dirs_to_scan.push((path, depth + 1));
            }
        }
    }

    if candidates.is_empty() {
        return None;
    }

    // 标准化游戏名称用于匹配
    let normalized_game_name = normalize_name(game_name);

    // 按优先级排序：深度浅优先，文件名匹配游戏名优先，文件大优先
    candidates.sort_by(|a, b| {
        // 首先按深度排序
        let depth_cmp = a.1.cmp(&b.1);
        if depth_cmp != std::cmp::Ordering::Equal {
            return depth_cmp;
        }

        // 同一深度下，文件名匹配游戏名的优先
        let a_name = a.0.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        let b_name = b.0.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        let a_match = normalize_name(&a_name) == normalized_game_name ||
                      normalized_game_name.len() > 2 && a_name.contains(&normalized_game_name);
        let b_match = normalize_name(&b_name) == normalized_game_name ||
                      normalized_game_name.len() > 2 && b_name.contains(&normalized_game_name);

        let match_cmp = b_match.cmp(&a_match);
        if match_cmp != std::cmp::Ordering::Equal {
            return match_cmp;
        }

        // 最后按文件大小降序
        b.2.cmp(&a.2)
    });

    // 返回最佳候选
    candidates.into_iter().next().map(|(path, _, _)| {
        let exe_path = path.to_string_lossy().to_string();
        let install_path = path.parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| root.to_string());
        (exe_path, install_path)
    })
}

/// 判断 exe 是否为辅助程序（非游戏主程序）
fn is_helper_exe(file_name: &str) -> bool {
    let helper_keywords = [
        "launcher", "setup", "install", "uninstall", "config", "settings",
        "crash", "report", "updater", "patch", "redist", "vcredist", "directx",
        "steam", "epic", "origin", "uplay", "cef", "webhelper", "bootstrap",
        "unitycrashhandler", "dump", "feedback", "repair", "verify",
    ];

    helper_keywords.iter().any(|keyword| file_name.contains(keyword))
}

/// 标准化名称：移除非字母数字字符并转小写，用于模糊匹配
fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}


