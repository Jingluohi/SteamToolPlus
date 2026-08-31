// 游戏库命令
// 处理游戏库相关的IPC调用

use crate::models::{
    AddGameRequest, Game, GameConfigData, GameFilter, GameListResponse, GameSortBy,
    ScanGamesRequest, UpdateGameRequest,
};
use crate::commands::patch_commands::common::normalize_resource_relative_path;
use crate::utils::resource_utils::get_resource_dir;
use crate::AppState;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

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
    let resource_dir = get_resource_dir(&app)
        .map_err(|e| format!("获取资源目录失败: {}", e))?;
    
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

/// 封面索引缓存：game_id -> 封面文件完整路径（正斜杠分隔）
/// 进程内只扫描一次 Game_Cover 目录，后续查询 O(1)，避免逐张图片做文件系统探测
static COVER_INDEX_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

/// 扫描 resources/pic/Game_Cover 目录，构建 game_id -> 文件路径 的索引
/// 浏览页封面仅存在 .jpg 格式，只索引 jpg 文件，避免无谓的格式探测
fn build_cover_index(app: &tauri::AppHandle) -> HashMap<String, String> {
    let mut index = HashMap::new();

    // 获取资源目录失败时返回空索引，不影响主流程
    let resource_dir = match get_resource_dir(app) {
        Ok(dir) => dir,
        Err(_) => return index,
    };

    let cover_dir = resource_dir.join("pic").join("Game_Cover");

    // 目录不存在或不是目录时直接返回空索引
    if !cover_dir.is_dir() {
        return index;
    }

    // 遍历目录，仅索引 .jpg 文件；文件名（去扩展名）即游戏 ID
    if let Ok(entries) = std::fs::read_dir(&cover_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            // 仅处理 jpg 扩展名（大小写不敏感）
            let is_jpg = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("jpg"))
                .unwrap_or(false);
            if !is_jpg {
                continue;
            }

            // 文件名去掉扩展名即游戏 ID
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                index.insert(stem.to_string(), path.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    index
}

/// 获取封面索引（懒加载：仅在首次调用时扫描一次目录）
fn get_cover_index(app: &tauri::AppHandle) -> std::sync::MutexGuard<'_, HashMap<String, String>> {
    COVER_INDEX_CACHE
        .get_or_init(|| Mutex::new(build_cover_index(app)))
        .lock()
        .unwrap()
}

/// 获取游戏封面图片路径（从resources/pic/Game_Cover目录）
/// 返回本地文件路径，前端使用 convertFileSrc 转换为 asset URL
/// 查询走内存索引，O(1) 且无文件系统探测
#[tauri::command]
pub async fn get_game_cover_image(app: tauri::AppHandle, game_id: String) -> Result<String, String> {
    // 验证游戏ID，防止路径遍历攻击
    if !crate::utils::file_utils::is_game_id_safe(&game_id) {
        return Err("无效的游戏ID".to_string());
    }

    // 从内存索引查询封面路径，不存在时返回空字符串
    let index = get_cover_index(&app);
    Ok(index.get(&game_id).cloned().unwrap_or_default())
}

/// 获取游戏封面图片（data URL）
/// 返回 "data:image/jpeg;base64,xxx" 格式，前端直接赋给 img.src，
/// data URL 不走 HTTP 缓存（asset:// 会被 WebView2 当作 HTTP 资源缓存导致解码位图累积）
#[tauri::command]
pub fn get_game_cover_bytes(app: tauri::AppHandle, game_id: String) -> Result<String, String> {
    use base64::Engine;

    // 校验游戏 ID，防止路径遍历
    if !crate::utils::file_utils::is_game_id_safe(&game_id) {
        return Ok(String::new());
    }

    // 在块内从内存索引查询封面路径，拿到后立即释放锁
    let path = {
        let index = get_cover_index(&app);
        index.get(&game_id).cloned()
    };

    // 图片不存在时返回空字符串
    let Some(path) = path else {
        return Ok(String::new());
    };

    // 读取文件字节并编码为 data URL
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let base64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:image/jpeg;base64,{}", base64))
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
            serde_json::from_str::<Vec<String>>(&content).unwrap_or_default()
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
        
        for entry in entries.flatten() {
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
    
    // 从配置文件读取额外的安装路径并检查
    let extra_paths = get_extra_install_paths(&app);
    for base_path in &extra_paths {
        let path = std::path::Path::new(base_path).join(&game_id);
        if path.exists() && path.is_dir() {
            let entries = std::fs::read_dir(&path)
                .map_err(|e| format!("读取游戏目录失败: {}", e))?;
            
            for entry in entries.flatten() {
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
        
        for entry in entries.flatten() {
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
    
    // 如果在resources/game/{game_id}中没找到，尝试配置文件中定义的额外路径
    if exe_path.is_none() {
        let extra_paths = get_extra_install_paths(&app);
        for base_path in &extra_paths {
            let path = std::path::Path::new(base_path).join(&game_id);
            if path.exists() && path.is_dir() {
                let entries = std::fs::read_dir(&path)
                    .map_err(|e| format!("读取游戏目录失败: {}", e))?;
                
                for entry in entries.flatten() {
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
                if exe_path.is_some() {
                    break;
                }
            }
        }
    }
    
    // 启动游戏（隐藏窗口）
    if let Some(exe) = exe_path {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            Command::new(&exe)
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .map_err(|e| format!("启动游戏失败: {}", e))?;
        }

        #[cfg(not(target_os = "windows"))]
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
    
    // 规范化资源相对路径，移除可能存在的 "resources/" 或 "Resources\\" 前缀
    let patch_relative_path = normalize_resource_relative_path(&patch_source_path);
    
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
    let is_valid = if let Some(suffix) = game_id.strip_prefix("custom_") {
        suffix.chars().all(|c| c.is_ascii_digit())
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

/// 从resources/successfully-tested.json加载测试成功的游戏列表
#[tauri::command]
pub async fn load_tested_games_config(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    // 获取资源目录路径
    let resource_dir = get_resource_dir(&app)
        .map_err(|e| format!("获取资源目录失败: {}", e))?;
    
    let config_path = resource_dir.join("successfully-tested.json");
    
    // 检查文件是否存在
    if !config_path.exists() {
        return Err(format!("测试游戏配置文件不存在: {}", config_path.display()));
    }
    
    // 读取文件内容
    let content = tokio::fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取测试游戏配置文件失败: {}", e))?;
    
    // 解析JSON
    let config: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析测试游戏配置文件失败: {}", e))?;
    
    Ok(config)
}
