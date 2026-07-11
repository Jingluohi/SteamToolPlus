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

/// 常见 Steam 默认安装路径（作为最后兜底）
const DEFAULT_STEAM_PATHS: &[&str] = &[
    r"C:\Program Files (x86)\Steam",
    r"C:\Program Files\Steam",
];

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
    /// 是否使用热加载（Steam运行时不重启，依赖文件监视自动加载新Lua）
    pub hot_reload: bool,
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

    // 2. 从注册表读取（只读，符合绿色便携规则）
    #[cfg(target_os = "windows")]
    {
        match read_steam_path_from_registry() {
            Ok(path) => {
                if Path::new(&path).exists() {
                    log::info!("从注册表读取到Steam路径: {}", path);
                    return Ok(path.to_string());
                }
            }
            Err(e) => {
                log::warn!("从注册表读取Steam路径失败: {}", e);
            }
        }
    }

    // 3. 尝试常见默认路径
    for path in DEFAULT_STEAM_PATHS {
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

/// 检测Steam是否正在运行
/// 通过 tasklist 检查 steam.exe 进程是否存在
pub fn is_steam_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let output = Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq steam.exe", "/NH"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.contains("steam.exe");
        }
    }

    false
}

/// 生成默认 opensteamtool.toml 配置文件到 Steam 根目录
/// 按官方 example 生成，使用默认的 opensteamtool manifest 源和中国友好的 wudrm 备用源
pub fn generate_opensteamtool_toml(steam_path: &str) -> Result<PathBuf, String> {
    let steam_path = Path::new(steam_path);
    if !steam_path.exists() {
        return Err("Steam路径不存在".to_string());
    }

    let toml_content = r#"# opensteamtool.toml — OpenSteamTool configuration
# Place at: <Steam>/opensteamtool.toml
# This file is loaded at startup and hot-reloaded after changes.

[log]
# Log verbosity for all log files (Debug build only).
# Valid: trace, debug, info, warn, error
level = "info"

[manifest]
# Upstream API for depot manifest request codes.
# "opensteamtool" -> https://manifest.opensteamtool.com/{gid}
# "wudrm"         -> http://gmrc.wudrm.com/manifest/{gid} (recommended for China users)
# "steamrun"      -> https://manifest.steam.run/api/manifest/{gid}
url = "wudrm"

# HTTP timeouts for manifest requests (milliseconds).
timeout_resolve_ms = 5000
timeout_connect_ms = 5000
timeout_send_ms    = 10000
timeout_recv_ms    = 10000

[lua]
# Additional Lua config directories (optional).
# paths = []

[stats]
# Query https://stats.opensteamtool.com/{appid} when no Lua setStat override exists.
# Priority: setStat > stats API when enabled and valid > hardcoded preset SteamID.
enable_api = true

[inject]
# Optional library injection into game processes.
enabled = false
# library_x64 = "OpenSteamTool.GameHook.x64.dll"
# library_x86 = "OpenSteamTool.GameHook.x86.dll"

[remote]
# Optional metadata mirror. Leave unset to use GitHub with jsDelivr fallback.
# Custom mirror template must include {channel}, {component}, and {sha256}.
# url_template = "https://your.server/{channel}/{component}/{sha256}.toml"
# url_template = "https://fast.jsdelivr.net/gh/OpenSteam001/steam-monitor@{channel}/{component}/{sha256}.toml"
"#;

    let toml_path = steam_path.join("opensteamtool.toml");
    fs::write(&toml_path, toml_content)
        .map_err(|e| format!("写入 opensteamtool.toml 失败: {}", e))?;

    log::info!("已生成 opensteamtool.toml: {}", toml_path.display());
    Ok(toml_path)
}

/// SteamTools 残留清理结果
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamToolsCleanResult {
    pub success: bool,
    pub removed_files: Vec<String>,
    pub removed_dirs: Vec<String>,
    /// 已自动删除的 SteamTools 相关注册表项
    pub removed_registry_keys: Vec<String>,
    pub message: String,
}

/// 已知 SteamTools 相关文件名
const KNOWN_STEAMTOOLS_FILES: &[&str] = &[
    "steamtools.dll",
    "SteamTools.dll",
    "stplug-in.dll",
    "stplugin.dll",
    "SteamToolsLoader.dll",
    "steamclient64.dll.backup.steamtools",
    "steamclient.dll.backup.steamtools",
];

/// 清理 SteamTools 残留文件和注册表项
/// 包括：Steam/config/stplug-in/ 目录、已知的 SteamTools DLL、相关注册表项
pub fn clean_steamtools_residuals(steam_path: &str) -> Result<SteamToolsCleanResult, String> {
    let steam_path = Path::new(steam_path);
    if !steam_path.exists() {
        return Err("Steam路径不存在".to_string());
    }

    let mut removed_files = Vec::new();
    let mut removed_dirs = Vec::new();
    let mut removed_registry_keys = Vec::new();

    // 1. 清理 SteamTools 专用的 stplug-in 目录
    let stplugin_dir = steam_path.join("config").join("stplug-in");
    if stplugin_dir.exists() {
        match fs::remove_dir_all(&stplugin_dir) {
            Ok(_) => removed_dirs.push(stplugin_dir.to_string_lossy().to_string()),
            Err(e) => log::warn!("清理 stplug-in 目录失败: {}", e),
        }
    }

    // 2. 清理已知的 SteamTools DLL 文件（在 Steam 根目录）
    for file_name in KNOWN_STEAMTOOLS_FILES {
        let file_path = steam_path.join(file_name);
        if file_path.exists() {
            match fs::remove_file(&file_path) {
                Ok(_) => removed_files.push(file_path.to_string_lossy().to_string()),
                Err(e) => log::warn!("删除 SteamTools 文件失败 {}: {}", file_path.display(), e),
            }
        }
    }

    // 3. 清理可能残留的 SteamTools 注册表项（仅 Windows）
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
        use winreg::RegKey;

        let steamtools_registry_paths = [
            (HKEY_CURRENT_USER, r"Software\Valve\Steam\SteamTools"),
            (HKEY_CURRENT_USER, r"Software\SteamTools"),
            (HKEY_LOCAL_MACHINE, r"Software\Valve\Steam\SteamTools"),
            (HKEY_LOCAL_MACHINE, r"Software\SteamTools"),
        ];

        for (hkey, path) in &steamtools_registry_paths {
            let root = RegKey::predef(*hkey);
            if root.open_subkey(path).is_ok() {
                match root.delete_subkey_all(path) {
                    Ok(_) => removed_registry_keys.push(format!("{:?}\\{}", hkey, path)),
                    Err(e) => log::warn!("删除注册表项失败 {}: {}", path, e),
                }
            }
        }
    }

    let total_removed = removed_files.len() + removed_dirs.len() + removed_registry_keys.len();
    let message = if total_removed > 0 {
        format!("已清理 {} 处 SteamTools 残留", total_removed)
    } else {
        "未检测到 SteamTools 残留".to_string()
    };

    log::info!("{}", message);

    Ok(SteamToolsCleanResult {
        success: true,
        removed_files,
        removed_dirs,
        removed_registry_keys,
        message,
    })
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

    // 5. 高级模式：不再自动写注册表，仅做占位记录
    let advanced_enabled = options.advanced_mode;
    if advanced_enabled {
        log::info!("高级模式已启用，但不会自动写入系统注册表");
    }

    // 6. 重启Steam（如果需要）
    // 热加载模式下：只要Steam正在运行就不重启，依赖OpenSteamTool的文件监视自动加载新Lua
    let mut steam_restarted = false;
    let should_restart = options.restart_steam && !options.hot_reload;
    if should_restart {
        if is_steam_running() {
            restart_steam(&steam_path)?;
            steam_restarted = true;
        } else {
            log::info!("Steam未运行，跳过重启步骤");
        }
    } else if options.hot_reload {
        log::info!("热加载模式已启用，Steam保持运行，OpenSteamTool将自动重新加载Lua配置");
    }

    let message = format!(
        "OpenSteamTool入库完成: {} (AppID: {}), Lua已写入, manifest复制{}个, 内核{}, Steam{}",
        options.game_name,
        app_id,
        manifest_copied,
        if kernel_installed { "已安装" } else { "未安装" },
        if steam_restarted { "已重启" } else if options.hot_reload { "热加载" } else { "未重启" }
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
