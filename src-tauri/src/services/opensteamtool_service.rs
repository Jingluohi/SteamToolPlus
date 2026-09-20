// OpenSteamTool 内核服务
// 提供Steam路径检测、内核DLL安装卸载、Lua生成、manifest复制、Steam重启等功能

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

use crate::services::fake_imported_service::cleanup_fake_imported_files;
use crate::utils::resource_utils::get_resource_dir;

/// OpenSteamTool DLL文件名列表
const OPENSTEAMTOOL_DLLS: &[&str] = &["dwmapi.dll", "xinput1_4.dll", "OpenSteamTool.dll"];

/// Lua配置目标子目录
const STEAM_LUA_DIR: &str = "config/lua";

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
    /// Lua脚本内容（主游戏）
    pub lua_content: String,
    /// 额外的Lua文件（DLC/创意工坊等），每项为(入库到的app_id, Lua内容)
    /// OpenSteamTool 会加载 config/lua 目录下的所有 .lua，
    /// 因此清单包内的主游戏、每个DLC、创意工坊depot各自独立写入，全部保留
    pub additional_lua_files: Vec<(u32, String)>,
    /// 需一并复制到 Steam/depotcache/（Steam 安装根目录）的清单文件路径（锁定版本时必须有本地 manifest，
    /// 否则 OpenSteamTool 只能从远程 manifest 服务取请求码，取不到就回退到最新版）
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

/// 去除 Windows verbatim 路径前缀 `\\?\`，使保存和显示的路径更简洁
fn normalize_windows_path(path: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        if let Some(stripped) = path.strip_prefix(r"\\?\") {
            return stripped.to_string();
        }
    }
    path.to_string()
}

/// 验证给定路径是否为有效的 Steam 安装目录
/// 必须包含 steam.exe 文件才算有效
pub fn validate_steam_path(steam_path: &str) -> Result<String, String> {
    if steam_path.is_empty() {
        return Err("Steam 路径不能为空".to_string());
    }

    let path = Path::new(steam_path);
    if !path.exists() {
        return Err(format!("路径不存在: {}", steam_path));
    }

    if !path.is_dir() {
        return Err(format!("路径不是目录: {}", steam_path));
    }

    // 检查 steam.exe 是否存在（支持大小写不敏感）
    let steam_exe = path.join("steam.exe");
    if steam_exe.exists() {
        return Ok(normalize_windows_path(steam_path));
    }

    // 兼容 Steam 安装目录下的 steam 目录（部分用户选择上层目录）
    let nested_steam_exe = path.join("Steam").join("steam.exe");
    if nested_steam_exe.exists() {
        return Ok(normalize_windows_path(
            &path.join("Steam").to_string_lossy(),
        ));
    }

    Err(format!(
        "选择失败，请选择 Steam 的安装路径（包含 steam.exe）：{}",
        steam_path
    ))
}

/// 获取Steam安装路径
/// 优先级：1.用户配置 2.注册表读取 3.常见默认路径
/// 返回前会验证路径下是否存在 steam.exe
pub fn detect_steam_path(config_steam_path: Option<&str>) -> Result<String, String> {
    // 1. 优先使用用户配置的Steam路径
    if let Some(path) = config_steam_path {
        if !path.is_empty() {
            match validate_steam_path(path) {
                Ok(valid_path) => {
                    log::info!("使用用户配置并验证通过的Steam路径: {}", valid_path);
                    return Ok(valid_path);
                }
                Err(e) => {
                    log::warn!("用户配置的Steam路径验证失败: {}", e);
                }
            }
        }
    }

    // 2. 从注册表读取（只读，符合绿色便携规则）
    #[cfg(target_os = "windows")]
    {
        match read_steam_path_from_registry() {
            Ok(path) => {
                match validate_steam_path(&path) {
                    Ok(valid_path) => {
                        log::info!("从注册表读取并验证通过的Steam路径: {}", valid_path);
                        return Ok(valid_path);
                    }
                    Err(e) => {
                        log::warn!("注册表读取的Steam路径验证失败: {}", e);
                    }
                }
            }
            Err(e) => {
                log::warn!("从注册表读取Steam路径失败: {}", e);
            }
        }
    }

    // 3. 尝试常见默认路径
    for path in DEFAULT_STEAM_PATHS {
        if let Ok(valid_path) = validate_steam_path(path) {
            log::info!("使用默认并验证通过的Steam路径: {}", valid_path);
            return Ok(valid_path);
        }
    }

    Err("无法找到有效的 Steam 安装路径（需要包含 steam.exe），请前往全局设置手动配置".to_string())
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

// ============================================
// 当前 Steam 登录账号解析（用于「使用 Steam 存档路径」功能）
// 与 OpenSteamTool 的 Denuvo 授权使用同一套账号提取机制：
//   1. 优先读取注册表 HKCU\Software\Valve\Steam\ActiveProcess\ActiveUser（当前活动账号）
//   2. 失败则解析 <Steam>\config\loginusers.vdf 中 mostrecent=1 的最近登录账号
// Steam 云端存档目录为 <Steam>\userdata\<32位账号ID>\<AppID>\remote
// ============================================

/// Steam 活动进程注册表键路径（Steam 客户端运行时/关闭后均可能残留，读取需容错）
#[cfg(target_os = "windows")]
const STEAM_ACTIVE_PROCESS_KEY: &str = r"Software\Valve\Steam\ActiveProcess";

/// 解析当前 Steam 登录账号
/// 返回 (Steam安装路径, 32位账号ID, SteamID64)
pub fn resolve_steam_account_id() -> Result<(String, u32, u64), String> {
    let steam_path = detect_steam_path(None)?;

    // 1. 首选：注册表 ActiveProcess\ActiveUser（Steam 当前活动账号）
    #[cfg(target_os = "windows")]
    {
        if let Some(account_id) = read_active_process_user()? {
            let universe = read_active_process_universe().unwrap_or(1u64);
            let steamid64 = (universe << 56) | (1u64 << 52) | (account_id as u64);
            log::info!("从注册表 ActiveProcess 解析到当前账号: account_id={}, steamid64={}", account_id, steamid64);
            return Ok((steam_path, account_id, steamid64));
        }
    }

    // 2. 兜底：解析 loginusers.vdf 的最近登录账号
    let (account_id, steamid64) = resolve_account_from_loginusers(&steam_path)?;
    log::info!("从 loginusers.vdf 解析到最近登录账号: account_id={}, steamid64={}", account_id, steamid64);
    Ok((steam_path, account_id, steamid64))
}

/// 从注册表读取 Steam 当前活动账号（32位账号ID），无记录时返回 None
#[cfg(target_os = "windows")]
fn read_active_process_user() -> Result<Option<u32>, String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let active_key = match hkcu.open_subkey(STEAM_ACTIVE_PROCESS_KEY) {
        Ok(key) => key,
        Err(_) => return Ok(None),
    };

    match active_key.get_value::<u32, _>("ActiveUser") {
        Ok(value) if value != 0 => Ok(Some(value)),
        _ => Ok(None),
    }
}

#[cfg(not(target_os = "windows"))]
fn read_active_process_user() -> Result<Option<u32>, String> {
    Ok(None)
}

/// 从注册表读取 Steam 活动账号所属的 universe（默认公网 universe=1）
#[cfg(target_os = "windows")]
fn read_active_process_universe() -> Option<u64> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let active_key = hkcu.open_subkey(STEAM_ACTIVE_PROCESS_KEY).ok()?;
    let value: String = active_key.get_value("Universe").ok()?;
    value.trim().parse().ok()
}

#[cfg(not(target_os = "windows"))]
fn read_active_process_universe() -> Option<u64> {
    None
}

/// 从 loginusers.vdf 解析最近登录（mostrecent=1）的账号
/// 返回 (32位账号ID, SteamID64)
fn resolve_account_from_loginusers(steam_path: &str) -> Result<(u32, u64), String> {
    let vdf_path = Path::new(steam_path).join("config").join("loginusers.vdf");
    let content = fs::read_to_string(&vdf_path)
        .map_err(|_| "未找到 loginusers.vdf，请先在 Steam 客户端登录一次账号".to_string())?;

    // 简单 VDF 解析：记录花括号深度，账号块位于深度 1~2
    // 深度 1 的键行为账号 SteamID64，深度 2 的键行为账号属性
    let mut depth: usize = 0;
    let mut current_steamid: Option<u64> = None;
    let mut current_account_id: Option<u32> = None;
    let mut current_most_recent = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.contains('{') {
            depth += 1;
            continue;
        }
        if trimmed.contains('}') {
            // 结束一个账号块：mostrecent=1 即最近登录账号
            if depth == 2 && current_most_recent {
                if let Some(steamid64) = current_steamid {
                    // AccountID 字段缺失时用 steamid64 低 32 位换算
                    let account_id = current_account_id.unwrap_or((steamid64 & 0xFFFF_FFFF) as u32);
                    return Ok((account_id, steamid64));
                }
            }
            current_steamid = None;
            current_account_id = None;
            current_most_recent = false;
            depth = depth.saturating_sub(1);
            continue;
        }

        let Some((key, value)) = extract_vdf_pair(trimmed) else { continue };
        if depth == 1 && value.is_none()
            && key.chars().all(|c| c.is_ascii_digit())
            && key.len() >= 14
        {
            // 账号块键：SteamID64
            if let Ok(id) = key.parse::<u64>() {
                current_steamid = Some(id);
            }
        } else if depth == 2 {
            match key.as_str() {
                "mostrecent" => current_most_recent = value.as_deref() == Some("1"),
                "AccountID" => {
                    if let Some(v) = value.as_deref().and_then(|v| v.parse::<u32>().ok()) {
                        current_account_id = Some(v);
                    }
                }
                _ => {}
            }
        }
    }

    Err("loginusers.vdf 中未找到最近登录的账号，请先在 Steam 客户端登录账号".to_string())
}

/// 提取 VDF 行中的键值对，格式 `"key" "value"`；键行（无值）时 value 为 None
fn extract_vdf_pair(line: &str) -> Option<(String, Option<String>)> {
    if !line.starts_with('"') {
        return None;
    }
    let mut parts = line.split('"').filter(|s| !s.is_empty());
    let key = parts.next()?.to_string();
    let value = parts.next().map(|s| s.to_string());
    Some((key, value))
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
    // 正则静态化，编译一次复用，避免每次调用重复编译
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"addappid\s*\(\s*(\d+)\s*").unwrap());
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

        // 更新配置：记录内核已安装（复用 AppState 中的共享配置服务）
        let config_service = app.state::<crate::AppState>().config_service.clone();
        let mut current_config = config_service.get_config();
        current_config.opensteamtool.kernel_installed = true;
        let _ = config_service.update_config(crate::models::UpdateConfigRequest {
            window: None,
            theme: None,
            game_dirs: None,
            launch: None,
            opensteamtool: Some(current_config.opensteamtool),
            record_button: None,
        });
    }

    // 2. 确定AppID
    let app_id = if options.app_id > 0 {
        options.app_id
    } else {
        extract_app_id_from_lua(&options.lua_content)
            .ok_or("无法从Lua内容中提取AppID，请提供有效的app_id")?
    };

    // 3. 清理以前假入库时可能残留的 Lua 文件（同一 AppID）
    // 避免旧 Lua 内容干扰新的入库配置
    if let Ok(deleted) = cleanup_fake_imported_files(&steam_path, app_id) {
        for file in deleted {
            log::info!("入库前已清理残留 Lua: {}", file);
        }
    }

    // 4. 写入Lua文件
    // 主游戏写入 {app_id}.lua
    let lua_file = write_lua_file(&steam_path, app_id, &options.lua_content)?;
    let mut lua_written = lua_file.exists();

    // 4.1 写入额外的Lua文件（DLC/创意工坊等）
    // OpenSteamTool 会加载 config/lua 目录下的所有 .lua，
    // 因此每个文件按各自提取的 app_id 命名写入，确保创意工坊密钥、DLC 等全部入库生效
    for (extra_app_id, extra_content) in &options.additional_lua_files {
        if *extra_app_id == 0 || *extra_app_id == app_id {
            continue;
        }
        match write_lua_file(&steam_path, *extra_app_id, extra_content) {
            Ok(f) => {
                lua_written = lua_written && f.exists();
                log::info!("已写入附加Lua文件: {}", f.display());
            }
            Err(e) => {
                log::warn!("写入附加Lua文件失败 appid={}: {}", extra_app_id, e);
            }
        }
    }

    // 4. 复制清单(.manifest)文件到 Steam/depotcache/（Steam 安装根目录，不是 config/depotcache）
    // setManifestid 只是把 depot 的 GID 补丁为锁定值，Steam 之后仍需要清单的实际内容。
    // OpenSteamTool 会向远程 manifest 服务(http/manifest.opensteamtool.com 等)取请求码，
    // 若该服务不可达或不认识该 GID，请求码为 0，Steam 会回退到最新版。
    // 因此必须在 depotcache 里放一份本地 {depotId}_{manifestId}.manifest，让 Steam 直接读本地清单，
    // 锁定版本才能真正生效。
    // Steam 的 depot manifest 缓存目录位于安装根目录的 depotcache/：Steam 在下载/校验游戏时
    // 会从该目录读取 {depotId}_{manifestId}.manifest。放在 config/depotcache 下 Steam 读不到，
    // 会转而向远程请求锁定 GID 的清单，接口加密后必然失败（表现为"无互联网连接"）。
    let mut manifest_copied = 0usize;
    if !options.manifest_files.is_empty() {
        let depotcache_dir = Path::new(&steam_path).join("depotcache");
        fs::create_dir_all(&depotcache_dir).map_err(|e| {
            format!("创建 depotcache 目录失败: {}", e)
        })?;

        // 清理同一批 depot 的旧清单：用户替换清单后，depotcache 里可能残留旧 manifestId 的
        // {depotId}_*.manifest。depotcache 是 Steam 共享缓存（含其它游戏清单），不能整体清空，
        // 因此只删除本次入库 depot 对应的旧文件，再写入新清单，避免过期清单堆积。
        let new_depot_ids: HashSet<String> = options
            .manifest_files
            .iter()
            .filter_map(|f| {
                Path::new(f)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|name| name.split('_').next().map(|id| id.to_string()))
            })
            .collect();

        if let Ok(entries) = fs::read_dir(&depotcache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("manifest") {
                    continue;
                }
                let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) else { continue };
                let Some(depot_id) = file_name.split('_').next() else { continue };
                if new_depot_ids.contains(depot_id) {
                    match fs::remove_file(&path) {
                        Ok(_) => log::info!("已清理旧清单文件: {}", path.display()),
                        Err(e) => log::warn!("清理旧清单文件失败 {}: {}", path.display(), e),
                    }
                }
            }
        }

        for manifest_file in &options.manifest_files {
            let src = PathBuf::from(manifest_file);
            if !src.is_file() {
                continue;
            }
            let Some(file_name) = src.file_name() else { continue };
            let dest = depotcache_dir.join(file_name);
            match fs::copy(&src, &dest) {
                Ok(_) => {
                    manifest_copied += 1;
                    log::info!("已复制清单文件: {}", dest.display());
                }
                Err(e) => {
                    log::warn!("复制清单文件失败 {}: {}", src.display(), e);
                }
            }
        }
    }

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
        "OpenSteamTool入库完成: {} (AppID: {}), Lua已写入, 内核{}, Steam{}",
        options.game_name,
        app_id,
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
