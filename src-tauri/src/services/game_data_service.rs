// 游戏数据管理服务
// 管理已下载和导入的游戏数据，存储在 %APPDATA%/SteamTool Plus/config/game.json
// 备份存储在程序根目录的 config/game.json
//
// 性能优化：使用全局内存缓存（RwLock）+ 脏标记 + 后台合并落盘。
// 所有读写操作直接操作内存，不再每次读写都完整加载/序列化整个 game.json 文件，
// 从而消除高频操作（下载进度更新、游戏时长更新）带来的重复磁盘 IO 与 CPU 序列化开销。

use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Once, RwLock};
use std::time::Duration;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
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

/// 已完成下载历史条目（下载记录页"已完成"Tab，持久化在 game.json，
/// 清理系统缓存不影响；字段名与前端 not serialize 一致）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadHistoryEntry {
    /// 游戏 ID
    pub game_id: String,
    /// 游戏名（中文 + 英文，前端写入）
    pub game_name: String,
    /// 完成时间戳（毫秒）
    pub completed_at: i64,
    /// 本次下载总字节数
    pub total_bytes: u64,
}

/// 游戏数据集合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataCollection {
    /// 游戏数据映射
    pub games: HashMap<String, GameData>,
    /// 已完成下载历史（下载记录页"已完成"Tab；不随系统缓存清理而丢失）
    #[serde(default)]
    pub download_history: Vec<DownloadHistoryEntry>,
    /// 最后更新时间
    pub last_updated: String,
}

impl Default for GameDataCollection {
    fn default() -> Self {
        Self {
            games: HashMap::new(),
            download_history: Vec::new(),
            last_updated: chrono::Local::now().to_rfc3339(),
        }
    }
}

/// 全局游戏数据内存缓存
/// 进程内只从磁盘加载一次，后续读操作直接读内存，写操作改内存后由后台线程合并落盘
static GAME_DATA_CACHE: Lazy<RwLock<GameDataCollection>> =
    Lazy::new(|| RwLock::new(load_game_data_from_disk()));

/// 脏标记：true 表示内存中有尚未落盘的更改
static DIRTY: AtomicBool = AtomicBool::new(false);

/// 后台落盘线程的启动保护（保证只启动一次）
static FLUSH_WORKER: Once = Once::new();

/// 获取游戏数据文件路径（运行时目录）
fn get_game_data_path() -> Result<std::path::PathBuf, String> {
    config_path_utils::get_runtime_config_path(GAME_DATA_FILENAME)
}

/// 从备份恢复游戏数据
pub fn restore_game_data_from_backup() -> Result<bool, String> {
    config_path_utils::restore_from_backup(GAME_DATA_FILENAME)
}

/// 同步游戏数据到备份
pub fn sync_game_data_to_backup() -> Result<(), String> {
    config_path_utils::sync_to_backup(GAME_DATA_FILENAME)
}

/// 从磁盘加载游戏数据（同步版本，仅在进程启动时调用一次）
/// 具有容错机制：主文件读取/解析失败时自动尝试从备份恢复，均失败则返回空集合
fn load_game_data_from_disk() -> GameDataCollection {
    let _ = restore_game_data_from_backup();

    let data_path = match get_game_data_path() {
        Ok(p) => p,
        Err(_) => return GameDataCollection::default(),
    };

    if !data_path.exists() {
        return GameDataCollection::default();
    }

    let backup_path = data_path.with_extension("backup");

    // 读取主文件
    let content = match std::fs::read_to_string(&data_path) {
        Ok(content) => content,
        Err(_) => {
            // 主文件读取失败，尝试从备份恢复
            if backup_path.exists() {
                if let Ok(backup_content) = std::fs::read_to_string(&backup_path) {
                    let _ = std::fs::copy(&backup_path, &data_path);
                    return serde_json::from_str(&backup_content).unwrap_or_default();
                }
            }
            return GameDataCollection::default();
        }
    };

    // 解析主文件
    match serde_json::from_str(&content) {
        Ok(collection) => collection,
        Err(_) => {
            // 主文件解析失败，尝试从备份恢复
            if backup_path.exists() {
                if let Ok(backup_content) = std::fs::read_to_string(&backup_path) {
                    if let Ok(backup_collection) = serde_json::from_str(&backup_content) {
                        let _ = std::fs::copy(&backup_path, &data_path);
                        return backup_collection;
                    }
                }
            }
            GameDataCollection::default()
        }
    }
}

/// 将内存中的集合持久化到磁盘
/// 使用流式序列化 + 临时文件 + 原子重命名 + fsync，
/// 避免 to_string 产生大字符串，同时保证崩溃时不会留下损坏的正式文件
fn persist_game_data(collection: &GameDataCollection) -> Result<(), String> {
    let _ = config_path_utils::ensure_runtime_config_dir();

    let data_path = get_game_data_path()?;
    let temp_path = data_path.with_extension("tmp");
    let backup_path = data_path.with_extension("backup");

    // 删除旧的临时文件
    if temp_path.exists() {
        let _ = std::fs::remove_file(&temp_path);
    }

    // 流式写入临时文件
    {
        let file = std::fs::File::create(&temp_path)
            .map_err(|e| format!("创建临时游戏数据文件失败: {}", e))?;
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, collection)
            .map_err(|e| format!("序列化游戏数据失败: {}", e))?;
        writer
            .flush()
            .map_err(|e| format!("写入临时游戏数据失败: {}", e))?;
        writer
            .get_ref()
            .sync_all()
            .map_err(|e| format!("同步游戏数据到磁盘失败: {}", e))?;
    }

    // 备份旧文件
    if data_path.exists() {
        let _ = std::fs::rename(&data_path, &backup_path);
    }

    // 原子重命名临时文件为正式文件
    if let Err(e) = std::fs::rename(&temp_path, &data_path) {
        // 重命名失败时从备份恢复，避免丢失旧数据
        if backup_path.exists() {
            let _ = std::fs::rename(&backup_path, &data_path);
        }
        return Err(format!("保存游戏数据失败: {}", e));
    }

    // 删除备份文件（保存成功后才删除）
    if backup_path.exists() {
        let _ = std::fs::remove_file(&backup_path);
    }

    // 同步到备份目录
    let _ = sync_game_data_to_backup();

    Ok(())
}

/// 确保后台落盘线程已启动（仅在首次写操作时调用）
fn ensure_flush_worker() {
    FLUSH_WORKER.call_once(|| {
        let _ = std::thread::Builder::new()
            .name("game-data-flush".to_string())
            .spawn(|| loop {
                // 每 500ms 检查一次脏标记，合并该时间窗口内的多次写入
                std::thread::sleep(Duration::from_millis(500));
                flush_if_dirty();
            });
    });
}

/// 标记内存数据为脏，并确保后台落盘线程已启动
fn mark_dirty() {
    DIRTY.store(true, Ordering::SeqCst);
    ensure_flush_worker();
}

/// 若存在脏数据则立即落盘（供后台线程与退出前调用）
fn flush_if_dirty() {
    if !DIRTY.swap(false, Ordering::SeqCst) {
        return;
    }
    let snapshot = GAME_DATA_CACHE.read().unwrap().clone();
    let _ = persist_game_data(&snapshot);
}

/// 同步刷新缓存到磁盘
/// 在程序退出前调用，确保最后的数据变更不丢失（后台线程最多延迟 500ms）
pub fn flush_now() {
    flush_if_dirty();
}

/// 获取所有游戏数据
pub async fn get_all_games(_app: AppHandle) -> Result<Vec<GameData>, String> {
    let cache = GAME_DATA_CACHE.read().unwrap();
    Ok(cache.games.values().cloned().collect())
}

/// 获取已完成下载历史
pub fn get_download_history() -> Result<Vec<DownloadHistoryEntry>, String> {
    let cache = GAME_DATA_CACHE.read().unwrap();
    Ok(cache.download_history.clone())
}

/// 追加（去重后置顶）已完成下载历史，保留最近 50 条
pub fn append_download_history(entries: Vec<DownloadHistoryEntry>) -> Result<(), String> {
    let mut cache = GAME_DATA_CACHE.write().unwrap();
    for entry in entries {
        cache
            .download_history
            .retain(|e| e.game_id != entry.game_id);
        cache.download_history.insert(0, entry);
    }
    if cache.download_history.len() > 50 {
        cache.download_history.truncate(50);
    }
    cache.last_updated = chrono::Local::now().to_rfc3339();
    drop(cache);
    mark_dirty();
    Ok(())
}

/// 移除指定游戏 ID 的已完成下载历史
pub fn remove_download_history(game_ids: &[String]) -> Result<(), String> {
    let mut cache = GAME_DATA_CACHE.write().unwrap();
    cache
        .download_history
        .retain(|e| !game_ids.contains(&e.game_id));
    cache.last_updated = chrono::Local::now().to_rfc3339();
    drop(cache);
    mark_dirty();
    Ok(())
}

/// 获取单个游戏数据
pub async fn get_game(_app: AppHandle, game_id: String) -> Result<Option<GameData>, String> {
    let cache = GAME_DATA_CACHE.read().unwrap();
    Ok(cache.games.get(&game_id).cloned())
}

/// 添加或更新游戏数据
pub async fn upsert_game(_app: AppHandle, game: GameData) -> Result<(), String> {
    {
        let mut cache = GAME_DATA_CACHE.write().unwrap();
        cache.last_updated = chrono::Local::now().to_rfc3339();
        cache.games.insert(game.game_id.clone(), game);
    }
    mark_dirty();
    Ok(())
}

/// 删除游戏数据
pub async fn remove_game(_app: AppHandle, game_id: String) -> Result<(), String> {
    {
        let mut cache = GAME_DATA_CACHE.write().unwrap();
        cache.games.remove(&game_id);
        cache.last_updated = chrono::Local::now().to_rfc3339();
    }
    mark_dirty();
    Ok(())
}

/// 更新游戏下载状态
pub async fn update_download_status(
    _app: AppHandle,
    game_id: String,
    status: String,
    progress: u32,
) -> Result<(), String> {
    {
        let mut cache = GAME_DATA_CACHE.write().unwrap();
        if let Some(game) = cache.games.get_mut(&game_id) {
            game.download_status = status;
            game.download_progress = progress;
            cache.last_updated = chrono::Local::now().to_rfc3339();
        }
    }
    mark_dirty();
    Ok(())
}

/// 更新游戏时长
pub async fn update_play_time(
    _app: AppHandle,
    game_id: String,
    additional_minutes: u64,
) -> Result<(), String> {
    {
        let mut cache = GAME_DATA_CACHE.write().unwrap();
        if let Some(game) = cache.games.get_mut(&game_id) {
            game.play_time += additional_minutes;
            game.last_played = Some(chrono::Local::now().to_rfc3339());
            cache.last_updated = chrono::Local::now().to_rfc3339();
        }
    }
    mark_dirty();
    Ok(())
}

/// 检查游戏是否已存在
pub async fn game_exists(_app: AppHandle, game_id: String) -> Result<bool, String> {
    let cache = GAME_DATA_CACHE.read().unwrap();
    Ok(cache.games.contains_key(&game_id))
}

/// 切换游戏收藏状态
pub async fn toggle_game_favorite(_app: AppHandle, game_id: String) -> Result<GameData, String> {
    {
        let mut cache = GAME_DATA_CACHE.write().unwrap();
        if !cache.games.contains_key(&game_id) {
            return Err(format!("游戏不存在: {}", game_id));
        }
        if let Some(game) = cache.games.get_mut(&game_id) {
            game.is_favorite = !game.is_favorite;
            cache.last_updated = chrono::Local::now().to_rfc3339();
        }
    }
    mark_dirty();

    let cache = GAME_DATA_CACHE.read().unwrap();
    cache
        .games
        .get(&game_id)
        .cloned()
        .ok_or_else(|| format!("游戏不存在: {}", game_id))
}

/// 下载完成后收尾处理
/// 扫描下载目录，自动定位游戏主程序并标记为已安装
pub async fn finalize_download(_app: AppHandle, game_id: String) -> Result<GameData, String> {
    // 先取出游戏副本，避免在扫描目录期间持有写锁
    let game = {
        let cache = GAME_DATA_CACHE.read().unwrap();
        cache
            .games
            .get(&game_id)
            .cloned()
            .ok_or_else(|| format!("游戏不存在: {}", game_id))?
    };

    // 确定要扫描的根目录：优先使用下载路径
    let scan_root = game
        .download_path
        .as_ref()
        .filter(|p| !p.is_empty() && std::path::Path::new(p).exists())
        .or_else(|| Some(&game.install_path))
        .filter(|p| !p.is_empty() && std::path::Path::new(p).exists())
        .cloned()
        .ok_or_else(|| "未找到有效的下载目录".to_string())?;

    // 扫描目录寻找游戏主程序（CPU 密集，不持锁）
    let best_exe = find_best_game_exe(&scan_root, &game.game_name);

    // 更新游戏数据
    {
        let mut cache = GAME_DATA_CACHE.write().unwrap();
        if let Some(game) = cache.games.get_mut(&game_id) {
            game.download_status = "completed".to_string();
            game.download_progress = 100;
            // 下载完成才算"已下载"（game_type 此前在下载开始阶段不标注，
            // 避免没下完/暂停/失败时游戏库误显示"已下载"）
            game.game_type = "downloaded".to_string();

            if let Some((exe_path, _exe_dir)) = best_exe {
                game.exe_path = exe_path;
                game.install_path = scan_root.clone();
                game.is_installed = true;
            } else {
                // 即使没有找到 exe，只要目录存在也标记为已安装
                game.install_path = scan_root.clone();
                game.is_installed = true;
            }

            cache.last_updated = chrono::Local::now().to_rfc3339();
        }
    }
    mark_dirty();

    let cache = GAME_DATA_CACHE.read().unwrap();
    cache
        .games
        .get(&game_id)
        .cloned()
        .ok_or_else(|| format!("游戏不存在: {}", game_id))
}

/// 检查所有已安装游戏的目录是否存在，将目录缺失的游戏重置为未安装
/// 并清理对应的下载日志缓存目录，返回被重置的游戏列表
/// 供前端库页面加载游戏列表后一次性调用，消除逐个游戏的 IPC 往返
pub async fn reset_missing_installed_games(_app: AppHandle) -> Result<Vec<GameData>, String> {
    use std::collections::HashSet;

    // 先收集需要检查的游戏（不持锁做磁盘 IO）
    let to_check: Vec<(String, String)> = {
        let cache = GAME_DATA_CACHE.read().unwrap();
        cache
            .games
            .iter()
            .filter(|(_, g)| g.is_installed && !g.install_path.is_empty())
            .map(|(id, g)| (id.clone(), g.install_path.clone()))
            .collect()
    };

    // 检查目录是否存在（不持锁）
    let missing_ids: HashSet<String> = to_check
        .iter()
        .filter(|(_, path)| !std::path::Path::new(path).is_dir())
        .map(|(id, _)| id.clone())
        .collect();

    if missing_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 批量重置状态
    let mut reset_games = Vec::new();
    {
        let mut cache = GAME_DATA_CACHE.write().unwrap();
        for id in &missing_ids {
            if let Some(game) = cache.games.get_mut(id) {
                game.is_installed = false;
                game.install_path = String::new();
                game.exe_path = String::new();
                game.download_status = String::new();
                game.download_progress = 0;
                reset_games.push(game.clone());
            }
        }
        cache.last_updated = chrono::Local::now().to_rfc3339();
    }
    mark_dirty();

    // 清理下载日志缓存目录
    if let Ok(root) = config_path_utils::get_appdata_dir() {
        for id in &missing_ids {
            let log_dir = root.join("log").join(id);
            if log_dir.exists() {
                let _ = std::fs::remove_dir_all(&log_dir);
            }
        }
    }

    Ok(reset_games)
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
        let depth_cmp = a.1.cmp(&b.1);
        if depth_cmp != std::cmp::Ordering::Equal {
            return depth_cmp;
        }

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