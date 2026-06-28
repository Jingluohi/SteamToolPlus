// OpenSteamTool 内核服务
// 提供Steam路径检测、内核DLL安装卸载、Lua生成、manifest复制、Steam重启等功能

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::AppHandle;

use crate::services::config_service::{ConfigService, ConfigServiceTrait};
use crate::utils::resource_utils::get_resource_dir;

/// OpenSteamTool DLL文件名列表
const OPENSTEAMTOOL_DLLS: &[&str] = &["dwmapi.dll", "xinput1_4.dll", "OpenSteamTool.dll"];

/// Lua配置目标子目录
const STEAM_LUA_DIR: &str = "config/lua";
/// Manifest缓存目标子目录
const STEAM_DEPOTCACHE_DIR: &str = "config/depotcache";

/// Steam注册表路径（用于读取Steam安装路径）
#[cfg(target_os = "windows")]
const STEAM_REGISTRY_PATH: &str = r"Software\Valve\Steam";
#[cfg(target_os = "windows")]
const STEAM_PATH_VALUE: &str = "SteamPath";

/// OpenSteamTool通用导入选项
#[derive(Debug, Clone)]
pub struct OpenSteamToolImportOptions {
    /// Steam安装路径
    pub steam_path: String,
    /// 游戏AppID
    pub app_id: u32,
    /// 游戏名称（用于日志展示）
    pub game_name: String,
    /// Lua脚本内容
    pub lua_content: String,
    /// 需要复制的manifest文件路径列表（可选）
    pub manifest_files: Vec<String>,
    /// 是否自动安装内核DLL（首次使用）
    pub install_kernel: bool,
    /// 是否重启Steam
    pub restart_steam: bool,
    /// 是否启用高级模式（写注册表等）
    pub advanced_mode: bool,
}

/// OpenSteamTool导入结果
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSteamToolImportResult {
    pub success: bool,
    pub message: String,
    pub kernel_installed: bool,
    pub lua_written: bool,
    pub manifest_copied: usize,
    pub steam_restarted: bool,
    pub advanced_enabled: bool,
}

/// 获取Steam安装路径
/// 优先级：1.用户配置 2.注册表读取 3.常见默认路径
pub fn detect_steam_path(config_steam_path: Option<&str>) -> Result<String, String> {
    // 1. 优先使用用户配置的Steam路径
    if let Some(path) = config_steam_path {
        if !path.is_empty() && Path::new(path).exists() {
            log::info!("使用用户配置的Steam路径: {}", path);
            return Ok(path.to_string());
        }
    }

    // 2. 从注册表读取
    #[cfg(target_os = "windows")]
    {
        match read_steam_path_from_registry() {
            Ok(path) => {
                if Path::new(&path).exists() {
                    log::info!("从注册表读取到Steam路径: {}", path);
                    return Ok(path);
                }
            }
            Err(e) => {
                log::warn!("从注册表读取Steam路径失败: {}", e);
            }
        }
    }

    // 3. 尝试常见默认路径
    let default_paths = [
        r"C:\Program Files (x86)\Steam",
        r"C:\Program Files\Steam",
    ];
    for path in &default_paths {
        if Path::new(path).exists() {
            log::info!("使用默认Steam路径: {}", path);
            return Ok(path.to_string());
        }
    }

    Err("无法找到Steam安装路径，请前往全局设置手动配置".to_string())
}

/// 从Windows注册表读取Steam安装路径
#[cfg(target_os = "windows")]
fn read_steam_path_from_registry() -> Result<String, String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let steam_key = hkcu
        .open_subkey(STEAM_REGISTRY_PATH)
        .map_err(|e| format!("打开Steam注册表项失败: {}", e))?;
    let steam_path: String = steam_key
        .get_value(STEAM_PATH_VALUE)
        .map_err(|e| format!("读取SteamPath注册表值失败: {}", e))?;

    Ok(steam_path)
}

#[cfg(not(target_os = "windows"))]
fn read_steam_path_from_registry() -> Result<String, String> {
    Err("非Windows系统不支持从注册表读取Steam路径".to_string())
}

/// 检查内核DLL是否已安装到Steam目录
pub fn is_kernel_installed(steam_path: &str) -> bool {
    let steam_path = Path::new(steam_path);
    if !steam_path.exists() {
        return false;
    }

    OPENSTEAMTOOL_DLLS.iter().all(|dll_name| {
        steam_path.join(dll_name).exists()
    })
}

/// 安装OpenSteamTool内核DLL到Steam目录
/// 从 resources/open 目录动态读取DLL并复制
pub fn install_kernel(app: &AppHandle, steam_path: &str) -> Result<(), String> {
    let steam_path = Path::new(steam_path);
    if !steam_path.exists() {
        return Err("Steam路径不存在".to_string());
    }

    let resource_dir = get_resource_dir(app)?;
    let source_dir = resource_dir.join("open");

    if !source_dir.exists() {
        return Err(format!("未找到OpenSteamTool DLL目录: {}", source_dir.display()));
    }

    for dll_name in OPENSTEAMTOOL_DLLS {
        let source = source_dir.join(dll_name);
        let dest = steam_path.join(dll_name);

        if !source.exists() {
            return Err(format!("未找到DLL文件: {}", source.display()));
        }

        // 如果目标已存在，先备份原文件
        if dest.exists() {
            let backup = steam_path.join(format!("{}.backup", dll_name));
            if !backup.exists() {
                fs::copy(&dest, &backup).map_err(|e| {
                    format!("备份 {} 失败: {}", dll_name, e)
                })?;
                log::info!("已备份原文件: {} -> {}", dest.display(), backup.display());
            }
        }

        fs::copy(&source, &dest).map_err(|e| {
            format!("复制 {} 到Steam目录失败: {}", dll_name, e)
        })?;
        log::info!("已安装内核DLL: {} -> {}", source.display(), dest.display());
    }

    Ok(())
}

/// 卸载OpenSteamTool内核DLL
pub fn uninstall_kernel(steam_path: &str) -> Result<(), String> {
    let steam_path = Path::new(steam_path);
    if !steam_path.exists() {
        return Err("Steam路径不存在".to_string());
    }

    for dll_name in OPENSTEAMTOOL_DLLS {
        let dest = steam_path.join(dll_name);
        if dest.exists() {
            fs::remove_file(&dest).map_err(|e| {
                format!("删除 {} 失败: {}", dll_name, e)
            })?;
            log::info!("已卸载内核DLL: {}", dest.display());
        }

        // 尝试恢复原文件
        let backup = steam_path.join(format!("{}.backup", dll_name));
        if backup.exists() {
            fs::rename(&backup, &dest).map_err(|e| {
                format!("恢复 {} 备份失败: {}", dll_name, e)
            })?;
            log::info!("已恢复原文件: {}", dest.display());
        }
    }

    Ok(())
}

/// 将Lua脚本写入Steam/config/lua/{app_id}.lua
pub fn write_lua_file(steam_path: &str, app_id: u32, lua_content: &str) -> Result<PathBuf, String> {
    let steam_path = Path::new(steam_path);
    let lua_dir = steam_path.join(STEAM_LUA_DIR);

    fs::create_dir_all(&lua_dir).map_err(|e| {
        format!("创建Lua目录失败: {}", e)
    })?;

    let lua_file = lua_dir.join(format!("{}.lua", app_id));
    fs::write(&lua_file, lua_content).map_err(|e| {
        format!("写入Lua文件失败: {}", e)
    })?;

    log::info!("已写入Lua文件: {}", lua_file.display());
    Ok(lua_file)
}

/// 复制manifest文件到Steam/config/depotcache/
pub fn copy_manifests(steam_path: &str, manifest_files: &[String]) -> Result<usize, String> {
    if manifest_files.is_empty() {
        return Ok(0);
    }

    let steam_path = Path::new(steam_path);
    let depotcache_dir = steam_path.join(STEAM_DEPOTCACHE_DIR);

    fs::create_dir_all(&depotcache_dir).map_err(|e| {
        format!("创建depotcache目录失败: {}", e)
    })?;

    let mut copied = 0;
    for manifest_file in manifest_files {
        let source = Path::new(manifest_file);
        if !source.exists() {
            log::warn!("manifest文件不存在，跳过: {}", source.display());
            continue;
        }

        if let Some(filename) = source.file_name() {
            let dest = depotcache_dir.join(filename);
            match fs::copy(source, &dest) {
                Ok(_) => {
                    copied += 1;
                    log::info!("已复制manifest: {} -> {}", source.display(), dest.display());
                }
                Err(e) => {
                    log::error!("复制manifest失败 {}: {}", source.display(), e);
                }
            }
        }
    }

    Ok(copied)
}

/// 重启Steam客户端
pub fn restart_steam(steam_path: &str) -> Result<(), String> {
    let steam_path = Path::new(steam_path);
    let steam_exe = steam_path.join("steam.exe");

    if !steam_exe.exists() {
        return Err("未找到steam.exe".to_string());
    }

    // 结束steam.exe进程
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", "steam.exe"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("pkill")
            .arg("steam")
            .output();
    }

    thread::sleep(Duration::from_secs(2));

    // 启动Steam
    let steam_exe_str = steam_exe.to_string_lossy().to_string();
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new(&steam_exe_str)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("启动Steam失败: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new(&steam_exe_str)
            .spawn()
            .map_err(|e| format!("启动Steam失败: {}", e))?;
    }

    log::info!("Steam已重启: {}", steam_exe_str);
    Ok(())
}

/// 从Lua脚本内容中提取AppID
/// 支持 addappid(appid) 格式
fn extract_app_id_from_lua(lua_content: &str) -> Option<u32> {
    use regex::Regex;

    let re = Regex::new(r"addappid\s*\(\s*(\d+)\s*").ok()?;
    re.captures(lua_content)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
}

/// 通用OpenSteamTool入库方法
pub fn import_with_opensteamtool(
    app: &AppHandle,
    options: OpenSteamToolImportOptions,
) -> Result<OpenSteamToolImportResult, String> {
    let steam_path = detect_steam_path(Some(&options.steam_path))?;

    // 1. 安装内核（如果需要）
    let mut kernel_installed = is_kernel_installed(&steam_path);
    if options.install_kernel && !kernel_installed {
        install_kernel(app, &steam_path)?;
        kernel_installed = true;

        // 更新配置：记录内核已安装
        if let Ok(config_service) = std::panic::catch_unwind(|| ConfigService::new()) {
            let mut current_config = config_service.get_config();
            current_config.opensteamtool.kernel_installed = true;
            let _ = config_service.update_config(crate::models::UpdateConfigRequest {
                window: None,
                theme: None,
                game_dirs: None,
                launch: None,
                opensteamtool: Some(current_config.opensteamtool),
            });
        }
    }

    // 2. 确定AppID
    let app_id = if options.app_id > 0 {
        options.app_id
    } else {
        extract_app_id_from_lua(&options.lua_content)
            .ok_or("无法从Lua内容中提取AppID，请提供有效的app_id")?
    };

    // 3. 写入Lua文件
    let lua_file = write_lua_file(&steam_path, app_id, &options.lua_content)?;
    let lua_written = lua_file.exists();

    // 4. 复制manifest文件（如果有）
    let manifest_copied = copy_manifests(&steam_path, &options.manifest_files)?;

    // 5. 高级模式：写注册表（Denuvo ticket等）
    let mut advanced_enabled = false;
    if options.advanced_mode {
        advanced_enabled = true;
        log::info!("高级模式已启用，将写入注册表（功能占位）");
        // TODO: 根据需求实现具体的ticket/token写入逻辑
        // 当前版本仅做占位，避免误操作
    }

    // 6. 重启Steam（如果需要）
    let mut steam_restarted = false;
    if options.restart_steam {
        restart_steam(&steam_path)?;
        steam_restarted = true;
    }

    let message = format!(
        "OpenSteamTool入库完成: {} (AppID: {}), Lua已写入, manifest复制{}个, 内核{}, Steam{}",
        options.game_name,
        app_id,
        manifest_copied,
        if kernel_installed { "已安装" } else { "未安装" },
        if steam_restarted { "已重启" } else { "未重启" }
    );

    Ok(OpenSteamToolImportResult {
        success: true,
        message,
        kernel_installed,
        lua_written,
        manifest_copied,
        steam_restarted,
        advanced_enabled,
    })
}

/// 获取资源目录下的OpenSteamTool DLL信息
pub fn get_kernel_dll_info(app: &AppHandle) -> Result<Vec<(String, bool)>, String> {
    let resource_dir = get_resource_dir(app)?;
    let source_dir = resource_dir.join("open");

    let mut result = Vec::new();
    for dll_name in OPENSTEAMTOOL_DLLS {
        let source = source_dir.join(dll_name);
        result.push((dll_name.to_string(), source.exists()));
    }

    Ok(result)
}
