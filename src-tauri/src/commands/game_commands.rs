// 游戏库命令
// 处理游戏库相关的IPC调用

use crate::models::{
    AddGameRequest, Game, GameConfigData, GameFilter, GameListResponse, GameSortBy,
    ScanGamesRequest, UpdateGameRequest,
};
use crate::utils::resource_utils::get_resource_dir;
use crate::AppState;

/// 获取所有游戏
#[tauri::command]
pub fn get_games(
    state: tauri::State<AppState>,
    filter: Option<GameFilter>,
    sort: Option<GameSortBy>,
) -> GameListResponse {
    state.game_service.get_games(filter, sort)
}

/// 根据ID获取游戏
#[tauri::command]
pub fn get_game_by_id(state: tauri::State<AppState>, id: String) -> Option<Game> {
    state.game_service.get_game_by_id(&id)
}

/// 添加游戏
#[tauri::command]
pub fn add_game(
    state: tauri::State<AppState>,
    request: AddGameRequest,
) -> Result<Game, String> {
    state.game_service.add_game(request)
}

/// 更新游戏
#[tauri::command]
pub fn update_game(
    state: tauri::State<AppState>,
    request: UpdateGameRequest,
) -> Result<Game, String> {
    state.game_service.update_game(request)
}

/// 删除游戏
#[tauri::command]
pub fn delete_game(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    state.game_service.delete_game(&id)
}

/// 扫描游戏目录
#[tauri::command]
pub fn scan_games_directory(
    state: tauri::State<AppState>,
    request: ScanGamesRequest,
) -> Result<Vec<Game>, String> {
    state.game_service.scan_games_directory(request)
}

/// 导入Steam游戏
#[tauri::command]
pub fn import_steam_games(state: tauri::State<AppState>) -> Result<Vec<Game>, String> {
    state.game_service.import_steam_games()
}

/// 从resources/games_config.json动态加载游戏配置
#[tauri::command]
pub async fn load_games_config_from_file(app: tauri::AppHandle) -> Result<Vec<GameConfigData>, String> {
    // 获取资源目录路径
    let resource_dir = get_resource_dir(&app)?;
    let config_path = resource_dir.join("games_config.json");
    
    // 检查文件是否存在
    if !config_path.exists() {
        return Err(format!("游戏配置文件不存在: {}", config_path.display()));
    }
    
    // 读取文件内容
    let content = tokio::fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取游戏配置文件失败: {}", e))?;
    
    // 解析JSON
    let games_config: Vec<GameConfigData> = serde_json::from_str(&content)
        .map_err(|e| format!("解析游戏配置文件失败: {}", e))?;
    
    Ok(games_config)
}

/// 获取游戏封面图片路径（从resources/pic/Game_Cover目录）
/// 返回本地文件路径，前端使用 convertFileSrc 转换为 asset URL
/// 这样可以避免 base64 编码的内存开销
#[tauri::command]
pub async fn get_game_cover_image(app: tauri::AppHandle, game_id: String) -> Result<String, String> {
    // 验证游戏ID，防止路径遍历攻击
    if game_id.is_empty() || !game_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("无效的游戏ID".to_string());
    }

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    let cover_dir = resource_dir.join("pic").join("Game_Cover");

    // 尝试不同格式的图片
    let jpg_path = cover_dir.join(format!("{}.jpg", game_id));
    let png_path = cover_dir.join(format!("{}.png", game_id));
    let webp_path = cover_dir.join(format!("{}.webp", game_id));

    // 返回存在的图片路径，使用 / 分隔符
    if jpg_path.exists() {
        Ok(jpg_path.to_string_lossy().replace('\\', "/"))
    } else if png_path.exists() {
        Ok(png_path.to_string_lossy().replace('\\', "/"))
    } else if webp_path.exists() {
        Ok(webp_path.to_string_lossy().replace('\\', "/"))
    } else {
        Ok(String::new()) // 图片不存在，返回空字符串
    }
}

/// 从配置文件中读取额外的游戏安装路径
/// 配置文件路径: resources/game_paths.json
/// 格式: ["D:\\Games", "E:\\SteamGames"]
fn get_extra_install_paths(app: &tauri::AppHandle) -> Vec<String> {
    let resource_dir = match get_resource_dir(app) {
        Ok(dir) => dir,
        Err(_) => return Vec::new(),
    };
    
    let config_path = resource_dir.join("game_paths.json");
    if !config_path.exists() {
        return Vec::new();
    }
    
    match std::fs::read_to_string(&config_path) {
        Ok(content) => {
            match serde_json::from_str::<Vec<String>>(&content) {
                Ok(paths) => paths,
                Err(_) => Vec::new(),
            }
        }
        Err(_) => Vec::new(),
    }
}

/// 检查游戏是否已安装
/// 通过检查游戏安装目录是否存在来判断
#[tauri::command]
pub async fn check_game_installed(
    game_id: String,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    // 验证游戏ID
    if game_id.is_empty() || !game_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("无效的游戏ID".to_string());
    }
    
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    
    // 检查游戏安装目录：resources/game/{game_id}
    let game_install_dir = resource_dir.join("game").join(&game_id);
    
    // 如果目录存在且包含游戏文件，则认为已安装
    if game_install_dir.exists() && game_install_dir.is_dir() {
        // 检查目录中是否有可执行文件
        let entries = std::fs::read_dir(&game_install_dir)
            .map_err(|e| format!("读取游戏目录失败: {}", e))?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "exe" {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }
    
    // 从配置文件读取额外的安装路径并检查
    let extra_paths = get_extra_install_paths(&app);
    for base_path in &extra_paths {
        let path = std::path::Path::new(base_path).join(&game_id);
        if path.exists() && path.is_dir() {
            let entries = std::fs::read_dir(&path)
                .map_err(|e| format!("读取游戏目录失败: {}", e))?;
            
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        if let Some(ext) = file_path.extension() {
                            if ext == "exe" {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(false)
}

/// 启动游戏
#[tauri::command]
pub async fn launch_game(game_id: String, app: tauri::AppHandle) -> Result<(), String> {
    // 验证游戏ID
    if game_id.is_empty() || !game_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("无效的游戏ID".to_string());
    }
    
    // 首先检查游戏是否已安装
    let is_installed = check_game_installed(game_id.clone(), app.clone()).await?;
    if !is_installed {
        return Err("游戏未安装".to_string());
    }
    
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    
    // 尝试找到游戏可执行文件
    let game_install_dir = resource_dir.join("game").join(&game_id);
    let mut exe_path: Option<std::path::PathBuf> = None;
    
    if game_install_dir.exists() {
        let entries = std::fs::read_dir(&game_install_dir)
            .map_err(|e| format!("读取游戏目录失败: {}", e))?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "exe" {
                            exe_path = Some(path);
                            break;
                        }
                    }
                }
            }
        }
    }
    
    // 如果在resources/game/{game_id}中没找到，尝试配置文件中定义的额外路径
    if exe_path.is_none() {
        let extra_paths = get_extra_install_paths(&app);
        for base_path in &extra_paths {
            let path = std::path::Path::new(base_path).join(&game_id);
            if path.exists() && path.is_dir() {
                let entries = std::fs::read_dir(&path)
                    .map_err(|e| format!("读取游戏目录失败: {}", e))?;
                
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path();
                        if file_path.is_file() {
                            if let Some(ext) = file_path.extension() {
                                if ext == "exe" {
                                    exe_path = Some(file_path);
                                    break;
                                }
                            }
                        }
                    }
                }
                if exe_path.is_some() {
                    break;
                }
            }
        }
    }
    
    // 启动游戏
    if let Some(exe) = exe_path {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            Command::new(&exe)
                .spawn()
                .map_err(|e| format!("启动游戏失败: {}", e))?;
        }
        
        Ok(())
    } else {
        Err("未找到游戏可执行文件".to_string())
    }
}

/// 检查补丁文件是否存在
/// 返回补丁文件的完整路径，如果不存在则返回空字符串
#[tauri::command]
pub async fn check_patch_file_exists(
    patch_source_path: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    
    // 处理路径：如果 patch_source_path 已经包含 "resources/" 或 "Resources/" 前缀，则移除它
    let patch_lower = patch_source_path.to_lowercase();
    let patch_relative_path = if patch_lower.starts_with("resources/") {
        &patch_source_path[10..]  // 移除 "resources/" 前缀
    } else if patch_lower.starts_with("resources\\") {
        &patch_source_path[11..]  // 移除 "resources\" 前缀
    } else {
        &patch_source_path
    };
    
    // 自动添加 .7z 后缀（如果路径中没有）
    let patch_file_name = if patch_relative_path.ends_with(".7z") {
        patch_relative_path.to_string()
    } else {
        format!("{}.7z", patch_relative_path)
    };
    
    // 构建完整的补丁文件路径（相对于资源目录）
    let patch_file_path = resource_dir.join(&patch_file_name);
    
    if patch_file_path.exists() && patch_file_path.is_file() {
        Ok(patch_file_path.to_string_lossy().to_string())
    } else {
        Ok(String::new())  // 文件不存在，返回空字符串
    }
}

/// 获取游戏封面路径
/// 用于库页面加载游戏封面
#[tauri::command]
pub async fn get_game_cover_path(app: tauri::AppHandle, game_id: String) -> Result<String, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    let cover_dir = resource_dir.join("pic").join("Game_Cover");
    
    // 尝试不同格式的图片
    let extensions = ["jpg", "png", "webp", "jpeg"];
    for ext in &extensions {
        let cover_path = cover_dir.join(format!("{}.{}", game_id, ext));
        if cover_path.exists() {
            return Ok(cover_path.to_string_lossy().replace('\\', "/"));
        }
    }
    
    Ok(String::new()) // 封面不存在
}

/// 获取游戏库背景图片路径（从resources/pic/库目录）
/// 用于库页面加载游戏背景大图
#[tauri::command]
pub async fn get_game_library_image(app: tauri::AppHandle, game_id: String) -> Result<String, String> {
    // 验证游戏ID，防止路径遍历攻击
    // 允许纯数字ID或custom_前缀的ID
    if game_id.is_empty() {
        return Err("无效的游戏ID".to_string());
    }
    
    // 检查ID是否合法：纯数字 或 custom_开头后跟数字
    let is_valid = if game_id.starts_with("custom_") {
        game_id[7..].chars().all(|c| c.is_ascii_digit())
    } else {
        game_id.chars().all(|c| c.is_ascii_digit())
    };
    
    if !is_valid {
        return Err("无效的游戏ID".to_string());
    }

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    let library_dir = resource_dir.join("pic").join("库");

    // 尝试不同格式的图片
    let extensions = ["jpg", "png", "webp", "jpeg"];
    for ext in &extensions {
        let image_path = library_dir.join(format!("{}.{}", game_id, ext));
        if image_path.exists() {
            // 将路径转换为使用 / 分隔符，确保 convertFileSrc 能正确处理
            let path_str = image_path.to_string_lossy().replace('\\', "/");
            return Ok(path_str);
        }
    }

    Ok(String::new()) // 图片不存在，返回空字符串
}
