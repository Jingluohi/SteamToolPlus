// 游戏库服务
// 实现游戏库的所有业务逻辑

use crate::models::{
    AddGameRequest, Game, GameFilter, GameListResponse, GameSortBy, LaunchGameResult,
    ScanGamesRequest, UpdateGameRequest,
};
use crate::services::ConfigServiceTrait;
use crate::utils::file_utils;
use std::sync::{Arc, Mutex};

/// 游戏服务接口
pub trait GameServiceTrait: Send + Sync {
    /// 获取所有游戏
    fn get_games(&self, filter: Option<GameFilter>, sort: Option<GameSortBy>) -> GameListResponse;
    /// 根据ID获取游戏
    fn get_game_by_id(&self, id: &str) -> Option<Game>;
    /// 添加游戏
    fn add_game(&self, request: AddGameRequest) -> Result<Game, String>;
    /// 更新游戏
    fn update_game(&self, request: UpdateGameRequest) -> Result<Game, String>;
    /// 删除游戏
    fn delete_game(&self, id: &str) -> Result<(), String>;
    /// 启动游戏
    fn launch_game(&self, id: &str) -> LaunchGameResult;
    /// 扫描游戏目录
    fn scan_games_directory(&self, request: ScanGamesRequest) -> Result<Vec<Game>, String>;
    /// 导入Steam游戏
    fn import_steam_games(&self) -> Result<Vec<Game>, String>;
    /// 保存游戏数据
    fn save_games(&self) -> Result<(), String>;
}

/// 游戏服务实现
pub struct GameService {
    /// 游戏数据存储
    games: Mutex<Vec<Game>>,
    /// 数据文件路径
    data_path: String,
    /// 配置服务引用
    config_service: Arc<dyn ConfigServiceTrait>,
}

impl GameService {
    /// 创建新的游戏服务实例
    pub fn new(config_service: Arc<dyn ConfigServiceTrait>) -> Self {
        let data_path = "data/games.json".to_string();
        let games = Self::load_games(&data_path);

        Self {
            games: Mutex::new(games),
            data_path,
            config_service,
        }
    }

    /// 从文件加载游戏数据
    fn load_games(path: &str) -> Vec<Game> {
        match file_utils::read_json_file::<Vec<Game>>(path) {
            Ok(games) => games,
            Err(_) => {
                // 文件不存在或读取失败，返回空列表
                Vec::new()
            }
        }
    }

    /// 保存游戏数据到文件
    fn save_games_internal(&self, games: &[Game]) -> Result<(), String> {
        file_utils::write_json_file(&self.data_path, &games.to_vec())
    }
}

impl GameServiceTrait for GameService {
    /// 获取所有游戏，支持筛选和排序
    fn get_games(&self, filter: Option<GameFilter>, sort: Option<GameSortBy>) -> GameListResponse {
        let games = self.games.lock().unwrap();
        let mut filtered: Vec<Game> = games.clone();

        // 应用筛选条件
        if let Some(filter) = filter {
            if let Some(search) = filter.search {
                let search_lower = search.to_lowercase();
                filtered.retain(|g| {
                    g.name.to_lowercase().contains(&search_lower)
                        || g.tags.iter().any(|t| t.to_lowercase().contains(&search_lower))
                        || g.publisher.to_lowercase().contains(&search_lower)
                });
            }
            if let Some(installed) = filter.installed {
                filtered.retain(|g| g.is_installed == installed);
            }
            if let Some(favorite) = filter.favorite {
                filtered.retain(|g| g.is_favorite == favorite);
            }
            if let Some(tags) = filter.tags {
                filtered.retain(|g| tags.iter().any(|t| g.tags.contains(t)));
            }
            if let Some(publisher) = filter.publisher {
                filtered.retain(|g| g.publisher == publisher);
            }
        }

        // 应用排序
        if let Some(sort) = sort {
            match sort {
                GameSortBy::Name => {
                    filtered.sort_by(|a, b| a.name.cmp(&b.name));
                }
                GameSortBy::LastPlayed => {
                    filtered.sort_by(|a, b| b.last_play_time.cmp(&a.last_play_time));
                }
                GameSortBy::PlayTime => {
                    filtered.sort_by(|a, b| b.total_play_time.cmp(&a.total_play_time));
                }
                GameSortBy::InstallDate => {
                    filtered.sort_by(|a, b| b.create_time.cmp(&a.create_time));
                }
                GameSortBy::ReleaseDate => {
                    filtered.sort_by(|a, b| b.release_date.cmp(&a.release_date));
                }
            }
        }

        let total = filtered.len();
        GameListResponse {
            games: filtered,
            total,
        }
    }

    /// 根据ID获取游戏
    fn get_game_by_id(&self, id: &str) -> Option<Game> {
        let games = self.games.lock().unwrap();
        games.iter().find(|g| g.id == id).cloned()
    }

    /// 添加新游戏
    fn add_game(&self, request: AddGameRequest) -> Result<Game, String> {
        let mut game = Game::new(request.name, request.exe_path);

        // 设置可选字段
        if let Some(cover) = request.cover_path {
            game.cover_path = Some(cover);
        }
        if let Some(params) = request.launch_params {
            game.launch_params = params;
        }
        if let Some(publisher) = request.publisher {
            game.publisher = publisher;
        }
        if let Some(release_date) = request.release_date {
            game.release_date = release_date;
        }
        if let Some(tags) = request.tags {
            game.tags = tags;
        }

        // 验证exe文件是否存在
        if !std::path::Path::new(&game.exe_path).exists() {
            return Err("游戏可执行文件不存在".to_string());
        }

        // 保存到列表
        let mut games = self.games.lock().unwrap();
        games.push(game.clone());

        // 持久化到文件
        drop(games);
        self.save_games()?;

        Ok(game)
    }

    /// 更新游戏信息
    fn update_game(&self, request: UpdateGameRequest) -> Result<Game, String> {
        let mut games = self.games.lock().unwrap();
        let game = games
            .iter_mut()
            .find(|g| g.id == request.id)
            .ok_or("游戏不存在")?;

        // 更新字段
        if let Some(name) = request.name {
            game.name = name;
        }
        if let Some(cover) = request.cover_path {
            game.cover_path = Some(cover);
        }
        if let Some(params) = request.launch_params {
            game.launch_params = params;
        }
        if let Some(publisher) = request.publisher {
            game.publisher = publisher;
        }
        if let Some(release_date) = request.release_date {
            game.release_date = release_date;
        }
        if let Some(tags) = request.tags {
            game.tags = tags;
        }
        if let Some(is_favorite) = request.is_favorite {
            game.is_favorite = is_favorite;
        }

        let game_clone = game.clone();
        drop(games);
        self.save_games()?;

        Ok(game_clone)
    }

    /// 删除游戏
    fn delete_game(&self, id: &str) -> Result<(), String> {
        let mut games = self.games.lock().unwrap();
        let pos = games.iter().position(|g| g.id == id).ok_or("游戏不存在")?;
        games.remove(pos);

        drop(games);
        self.save_games()?;

        Ok(())
    }

    /// 启动游戏
    fn launch_game(&self, id: &str) -> LaunchGameResult {
        let game = match self.get_game_by_id(id) {
            Some(g) => g,
            None => {
                return LaunchGameResult {
                    success: false,
                    error: Some("游戏不存在".to_string()),
                    pid: None,
                }
            }
        };

        // 检查exe文件是否存在
        if !std::path::Path::new(&game.exe_path).exists() {
            return LaunchGameResult {
                success: false,
                error: Some("游戏可执行文件不存在".to_string()),
                pid: None,
            };
        }

        // 使用Tauri shell API启动游戏
        // 这里返回成功，实际启动由前端调用shell.open处理
        LaunchGameResult {
            success: true,
            error: None,
            pid: None,
        }
    }

    /// 扫描游戏目录
    fn scan_games_directory(&self, request: ScanGamesRequest) -> Result<Vec<Game>, String> {
        let path = std::path::Path::new(&request.directory);
        if !path.exists() || !path.is_dir() {
            return Err("目录不存在或不是有效目录".to_string());
        }

        let mut found_games = Vec::new();

        // 扫描目录中的exe文件
        let entries = std::fs::read_dir(path).map_err(|e| e.to_string())?;

        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    if ext.eq_ignore_ascii_case("exe") {
                        let file_name = file_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("Unknown");

                        let game = Game::new(
                            file_name.to_string(),
                            file_path.to_string_lossy().to_string(),
                        );
                        found_games.push(game);
                    }
                }
            } else if request.recursive && file_path.is_dir() {
                // 递归扫描子目录
                let sub_request = ScanGamesRequest {
                    directory: file_path.to_string_lossy().to_string(),
                    recursive: true,
                };
                if let Ok(sub_games) = self.scan_games_directory(sub_request) {
                    found_games.extend(sub_games);
                }
            }
        }

        Ok(found_games)
    }

    /// 导入Steam游戏
    fn import_steam_games(&self) -> Result<Vec<Game>, String> {
        // 获取Steam安装路径
        let config = self.config_service.get_config();
        let steam_path = config
            .game_dirs
            .steam_path
            .ok_or("未配置Steam安装路径")?;

        // Steam库文件夹列表
        let library_folders_path = std::path::Path::new(&steam_path)
            .join("steamapps")
            .join("libraryfolders.vdf");

        if !library_folders_path.exists() {
            return Err("未找到Steam库配置文件".to_string());
        }

        let mut imported_games = Vec::new();

        // 解析libraryfolders.vdf获取所有库路径
        // 简化实现：直接扫描steamapps/common目录
        let steamapps_path = std::path::Path::new(&steam_path).join("steamapps");
        let common_path = steamapps_path.join("common");

        if common_path.exists() {
            let entries = std::fs::read_dir(&common_path).map_err(|e| e.to_string())?;

            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let game_name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Unknown");

                    // 查找exe文件
                    if let Ok(exe_path) = find_game_exe(&path) {
                        let game = Game::new(game_name.to_string(), exe_path);
                        imported_games.push(game);
                    }
                }
            }
        }

        Ok(imported_games)
    }

    /// 保存游戏数据
    fn save_games(&self) -> Result<(), String> {
        let games = self.games.lock().unwrap();
        self.save_games_internal(&games)
    }
}

/// 在目录中查找游戏exe文件
fn find_game_exe(dir: &std::path::Path) -> Result<String, String> {
    let entries = std::fs::read_dir(dir).map_err(|e| e.to_string())?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("exe") {
                    return Ok(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Err("未找到可执行文件".to_string())
}
