// OpenSteamTool 内核相关命令
// 提供前端调用的IPC命令，用于管理OpenSteamTool内核和游戏入库

use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

use crate::commands::manifest_commands::convert_vdf_to_lua_internal;
use crate::services::config_service::{ConfigService, ConfigServiceTrait};
use crate::services::fake_imported_service::add_fake_imported_game;
use crate::services::opensteamtool_service::{
    clean_steamtools_residuals, detect_steam_path, generate_opensteamtool_toml,
    get_kernel_dll_info, install_kernel, is_kernel_installed, is_steam_running,
    uninstall_kernel, import_with_opensteamtool, OpenSteamToolImportOptions,
    OpenSteamToolImportResult, SteamToolsCleanResult,
};
use crate::utils::resource_utils::get_resource_dir;

/// 获取Steam安装路径
/// 优先读取用户配置，其次从注册表读取，最后尝试默认路径
#[tauri::command]
pub fn get_steam_path() -> Result<String, String> {
    let config_steam_path = ConfigService::new().get_config().game_dirs.steam_path;
    detect_steam_path(config_steam_path.as_deref())
}

/// 检测Steam安装路径（不依赖用户配置）
#[tauri::command]
pub fn detect_steam_path_auto() -> Result<String, String> {
    detect_steam_path(None)
}

/// 安装OpenSteamTool内核DLL
#[tauri::command]
pub fn install_opensteamtool_kernel(app: AppHandle, steam_path: String) -> Result<serde_json::Value, String> {
    install_kernel(&app, &steam_path)?;

    // 更新配置
    let config_service = ConfigService::new();
    let mut config = config_service.get_config();
    config.opensteamtool.kernel_installed = true;
    let _ = config_service.update_config(crate::models::UpdateConfigRequest {
        window: None,
        theme: None,
        game_dirs: None,
        launch: None,
        opensteamtool: Some(config.opensteamtool),
    });

    Ok(serde_json::json!({
        "success": true,
        "message": "OpenSteamTool内核安装成功"
    }))
}

/// 卸载OpenSteamTool内核DLL
#[tauri::command]
pub fn uninstall_opensteamtool_kernel(steam_path: String) -> Result<serde_json::Value, String> {
    uninstall_kernel(&steam_path)?;

    // 更新配置
    let config_service = ConfigService::new();
    let mut config = config_service.get_config();
    config.opensteamtool.kernel_installed = false;
    let _ = config_service.update_config(crate::models::UpdateConfigRequest {
        window: None,
        theme: None,
        game_dirs: None,
        launch: None,
        opensteamtool: Some(config.opensteamtool),
    });

    Ok(serde_json::json!({
        "success": true,
        "message": "OpenSteamTool内核卸载成功"
    }))
}

/// 检查OpenSteamTool内核是否已安装
#[tauri::command]
pub fn check_opensteamtool_kernel_installed(steam_path: String) -> Result<serde_json::Value, String> {
    let installed = is_kernel_installed(&steam_path);
    Ok(serde_json::json!({
        "installed": installed
    }))
}

/// 获取OpenSteamTool DLL资源信息
#[tauri::command]
pub fn get_opensteamtool_dll_info(app: AppHandle) -> Result<serde_json::Value, String> {
    let dll_info = get_kernel_dll_info(&app)?;
    Ok(serde_json::json!({
        "dlls": dll_info
    }))
}

/// 通用OpenSteamTool入库命令
/// 适用于清单入库和游戏详情页入库
#[tauri::command]
pub fn import_with_opensteamtool_command(
    app: AppHandle,
    steam_path: String,
    app_id: u32,
    game_name: String,
    lua_content: String,
    manifest_files: Option<Vec<String>>,
    install_kernel: Option<bool>,
    restart_steam: Option<bool>,
    advanced_mode: Option<bool>,
    hot_reload: Option<bool>,
) -> Result<OpenSteamToolImportResult, String> {
    let options = OpenSteamToolImportOptions {
        steam_path,
        app_id,
        game_name,
        lua_content,
        manifest_files: manifest_files.unwrap_or_default(),
        install_kernel: install_kernel.unwrap_or(true),
        restart_steam: restart_steam.unwrap_or(true),
        advanced_mode: advanced_mode.unwrap_or(false),
        hot_reload: hot_reload.unwrap_or(true),
    };

    import_with_opensteamtool(&app, options)
}

/// 递归扫描清单文件
/// 扫描目录及其子目录（最多2层），收集.lua、.manifest、.vdf文件路径
fn scan_manifest_files_recursive(
    dir: &Path,
    lua_files: &mut Vec<String>,
    manifest_files: &mut Vec<String>,
    vdf_files: &mut Vec<String>,
    depth: usize,
) -> Result<(), String> {
    if depth > 2 {
        return Ok(());
    }

    let entries = fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            scan_manifest_files_recursive(&path, lua_files, manifest_files, vdf_files, depth + 1)?;
        } else if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            let path_str = path.to_string_lossy().to_string();

            match ext.as_str() {
                "lua" => lua_files.push(path_str),
                "manifest" => manifest_files.push(path_str),
                "vdf" => vdf_files.push(path_str),
                _ => {}
            }
        }
    }

    Ok(())
}

/// 尝试将VDF文件转换为Lua文件
/// 返回转换后的Lua文件路径列表
///
/// # 参数
/// - `vdf_files`: VDF文件路径列表
/// - `access_token`: 可选的访问令牌，用于下载受保护的游戏/DLC
/// - `stats_steam_id`: 可选的成就数据SteamID
/// - `lock_version`: 是否锁定版本（生成 setManifestid）
fn convert_vdf_files_to_lua(
    vdf_files: &[String],
    access_token: Option<&str>,
    stats_steam_id: Option<&str>,
    lock_version: bool,
) -> Vec<String> {
    let mut converted_lua_files = Vec::new();

    for vdf_file in vdf_files {
        let vdf_path = PathBuf::from(vdf_file);
        match convert_vdf_to_lua_internal(&vdf_path, access_token, stats_steam_id, lock_version) {
            Ok(lua_path) => {
                converted_lua_files.push(lua_path.to_string_lossy().to_string());
            }
            Err(e) => {
                log::warn!("转换VDF文件失败 {}: {}", vdf_file, e);
            }
        }
    }

    converted_lua_files
}

/// 从游戏清单目录使用OpenSteamTool入库
/// 用于游戏详情页：自动读取 resources/manifest/{game_id}/ 下的文件
#[tauri::command]
pub fn import_game_with_opensteamtool(
    app: AppHandle,
    steam_path: String,
    game_id: String,
    game_name: String,
    app_id: u32,
    advanced_mode: Option<bool>,
    hot_reload: Option<bool>,
    access_token: Option<String>,
    stats_steam_id: Option<String>,
    lock_version: Option<bool>,
) -> Result<OpenSteamToolImportResult, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    let manifest_dir = resource_dir.join("manifest").join(&game_id);

    if !manifest_dir.exists() {
        return Err(format!("未找到游戏清单目录: {}", manifest_dir.display()));
    }

    // 递归扫描文件
    let mut lua_files = Vec::new();
    let mut manifest_files = Vec::new();
    let mut vdf_files = Vec::new();

    scan_manifest_files_recursive(&manifest_dir, &mut lua_files, &mut manifest_files, &mut vdf_files, 0)?;

    // 检查是否有lua或vdf
    if lua_files.is_empty() && vdf_files.is_empty() {
        return Err("未找到.lua或.vdf文件".to_string());
    }

    // 没有Lua但有VDF时，自动转换VDF为Lua
    if lua_files.is_empty() && !vdf_files.is_empty() {
        let converted = convert_vdf_files_to_lua(
            &vdf_files,
            access_token.as_deref(),
            stats_steam_id.as_deref(),
            lock_version.unwrap_or(false),
        );
        lua_files.extend(converted);
    }

    // 读取第一个Lua文件内容
    let lua_content = if let Some(lua_file) = lua_files.first() {
        fs::read_to_string(lua_file).map_err(|e| format!("读取Lua文件失败: {}", e))?
    } else {
        return Err("未找到Lua文件，OpenSteamTool内核模式需要Lua文件".to_string());
    };

    let options = OpenSteamToolImportOptions {
        steam_path,
        app_id,
        game_name,
        lua_content,
        manifest_files,
        install_kernel: true,
        restart_steam: true,
        advanced_mode: advanced_mode.unwrap_or(false),
        hot_reload: hot_reload.unwrap_or(true),
    };

    let result = import_with_opensteamtool(&app, options);

    // 入库成功后记录到假入库游戏列表
    if result.is_ok() {
        let _ = add_fake_imported_game(app_id);
    }

    result
}

/// 从清单文件夹使用OpenSteamTool入库
/// 用于清单入库页面
#[tauri::command]
pub fn import_manifest_with_opensteamtool(
    app: AppHandle,
    steam_path: String,
    folder_path: String,
    game_name: Option<String>,
    app_id: Option<u32>,
    advanced_mode: Option<bool>,
    hot_reload: Option<bool>,
    access_token: Option<String>,
    stats_steam_id: Option<String>,
    lock_version: Option<bool>,
) -> Result<OpenSteamToolImportResult, String> {
    let folder_path = Path::new(&folder_path);

    if !folder_path.exists() {
        return Err("清单文件夹不存在".to_string());
    }

    // 递归扫描文件
    let mut lua_files = Vec::new();
    let mut manifest_files = Vec::new();
    let mut vdf_files = Vec::new();

    scan_manifest_files_recursive(folder_path, &mut lua_files, &mut manifest_files, &mut vdf_files, 0)?;

    if lua_files.is_empty() && vdf_files.is_empty() {
        return Err("未找到.lua或.vdf文件".to_string());
    }

    // 没有Lua但有VDF时，自动转换VDF为Lua
    if lua_files.is_empty() && !vdf_files.is_empty() {
        let converted = convert_vdf_files_to_lua(
            &vdf_files,
            access_token.as_deref(),
            stats_steam_id.as_deref(),
            lock_version.unwrap_or(false),
        );
        lua_files.extend(converted);
    }

    // 读取Lua内容
    let lua_content = if let Some(lua_file) = lua_files.first() {
        fs::read_to_string(lua_file).map_err(|e| format!("读取Lua文件失败: {}", e))?
    } else {
        return Err("未找到Lua文件，OpenSteamTool内核模式需要Lua文件".to_string());
    };

    // 提取AppID
    let detected_app_id = if let Some(id) = app_id {
        id
    } else {
        extract_app_id_from_lua(&lua_content)
            .ok_or("无法从Lua内容中提取AppID，请手动提供")?
    };

    let game_name_value = game_name.unwrap_or_else(|| format!("Game_{}", detected_app_id));

    let options = OpenSteamToolImportOptions {
        steam_path,
        app_id: detected_app_id,
        game_name: game_name_value,
        lua_content,
        manifest_files,
        install_kernel: true,
        restart_steam: true,
        advanced_mode: advanced_mode.unwrap_or(false),
        hot_reload: hot_reload.unwrap_or(true),
    };

    let result = import_with_opensteamtool(&app, options);

    // 入库成功后记录到假入库游戏列表
    if result.is_ok() {
        let _ = add_fake_imported_game(detected_app_id);
    }

    result
}

/// 检测Steam是否正在运行
#[tauri::command]
pub fn check_steam_running() -> Result<serde_json::Value, String> {
    let running = is_steam_running();
    Ok(serde_json::json!({
        "running": running
    }))
}

/// 生成默认 opensteamtool.toml 配置文件到 Steam 根目录
#[tauri::command]
pub fn generate_opensteamtool_config(steam_path: String) -> Result<serde_json::Value, String> {
    let toml_path = generate_opensteamtool_toml(&steam_path)?;
    Ok(serde_json::json!({
        "success": true,
        "path": toml_path.to_string_lossy().to_string(),
        "message": "opensteamtool.toml 生成成功"
    }))
}

/// 清理 SteamTools 残留文件和注册表项
#[tauri::command]
pub fn clean_steamtools_residuals_command(steam_path: String) -> Result<SteamToolsCleanResult, String> {
    clean_steamtools_residuals(&steam_path)
}

/// 从Lua内容中提取AppID
fn extract_app_id_from_lua(lua_content: &str) -> Option<u32> {
    use regex::Regex;

    let re = Regex::new(r"addappid\s*\(\s*(\d+)\s*").ok()?;
    re.captures(lua_content)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
}
