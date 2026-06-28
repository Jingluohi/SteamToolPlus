/**
 * manifest_commands.rs - 清单入库命令模块
 * 提供清单文件扫描、解压、VDF转Lua、复制到Steam目录等功能
 */
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use serde_json::json;
use crate::services::config_service::ConfigServiceTrait;
use crate::utils::resource_utils::get_resource_dir;
use tauri::AppHandle;

/// 扫描文件夹中的清单文件
/// 递归查找.lua、.manifest、.vdf文件，支持嵌套1-2层目录
#[tauri::command]
pub fn scan_manifest_folder(folder_path: String) -> Result<serde_json::Value, String> {
    let mut lua_files = Vec::new();
    let mut manifest_files = Vec::new();
    let mut vdf_files = Vec::new();

    fn scan_directory(dir: &Path, lua: &mut Vec<String>, manifest: &mut Vec<String>, vdf: &mut Vec<String>, depth: usize) -> Result<(), String> {
        if depth > 2 {
            return Ok(());
        }

        let entries = fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                scan_directory(&path, lua, manifest, vdf, depth + 1)?;
            } else if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                let path_str = path.to_string_lossy().to_string();

                match ext.as_str() {
                    "lua" => lua.push(path_str),
                    "manifest" => manifest.push(path_str),
                    "vdf" => vdf.push(path_str),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    scan_directory(Path::new(&folder_path), &mut lua_files, &mut manifest_files, &mut vdf_files, 0)?;

    Ok(json!({
        "luaFiles": lua_files,
        "manifestFiles": manifest_files,
        "vdfFiles": vdf_files
    }))
}

/// 解压压缩包到临时目录
/// 支持7z、zip格式
#[tauri::command]
pub fn extract_archive(archive_path: String) -> Result<String, String> {
    use std::process::Command;

    let archive_path = Path::new(&archive_path);
    let file_stem = archive_path
        .file_stem()
        .ok_or("无法获取文件名")?
        .to_string_lossy();

    // 创建临时解压目录
    let temp_dir = std::env::temp_dir().join(format!("steam_tool_manifest_{}", file_stem));
    fs::create_dir_all(&temp_dir).map_err(|e| format!("创建临时目录失败: {}", e))?;

    let temp_dir_str = temp_dir.to_string_lossy().to_string();

    // 判断压缩包类型并使用对应工具解压
    let ext = archive_path
        .extension()
        .ok_or("无法获取文件扩展名")?
        .to_string_lossy()
        .to_lowercase();

    match ext.as_str() {
        "7z" => {
            // 使用 sevenz-rust 库解压
            sevenz_rust::decompress_file(archive_path, &temp_dir)
                .map_err(|e| format!("解压7z文件失败: {:?}", e))?;
        }
        "zip" => {
            // 使用PowerShell解压zip文件（隐藏窗口）
            #[cfg(target_os = "windows")]
            use std::os::windows::process::CommandExt;
            #[cfg(target_os = "windows")]
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            let mut cmd = Command::new("powershell");
            #[cfg(target_os = "windows")]
            cmd.creation_flags(CREATE_NO_WINDOW);
            let output = cmd
                .args([
                    "-Command",
                    &format!("Expand-Archive -Path '{}' -DestinationPath '{}' -Force", archive_path.to_string_lossy(), temp_dir_str)
                ])
                .output()
                .map_err(|e| format!("解压失败: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("解压失败: {}", stderr));
            }
        }
        _ => return Err("不支持的压缩格式，仅支持7z和zip".to_string()),
    };

    Ok(temp_dir_str)
}

/// 解析VDF内容，提取depot信息
fn parse_vdf_content(content: &str) -> Vec<(String, String)> {
    let mut depots = Vec::new();
    let mut current_depot: Option<String> = None;
    // 将正则表达式移到循环外部，避免每次迭代都重新编译
    let re = Regex::new(r#""DecryptionKey"\s+"([a-f0-9]+)""#).unwrap();

    for line in content.lines() {
        let trimmed = line.trim();

        // 匹配 depot_id 行: "123456"
        if trimmed.starts_with('"') && !trimmed.contains("DecryptionKey") && !trimmed.contains("depots") {
            if let Some(end_quote) = trimmed[1..].find('"') {
                let depot_id = &trimmed[1..=end_quote];
                if depot_id.chars().all(|c| c.is_ascii_digit()) {
                    current_depot = Some(depot_id.to_string());
                }
            }
        }

        // 匹配 DecryptionKey 行
        if trimmed.contains("DecryptionKey") {
            if let Some(cap) = re.captures(trimmed) {
                let key = cap[1].to_string();
                if let Some(ref depot_id) = current_depot {
                    depots.push((depot_id.clone(), key));
                    current_depot = None;
                }
            }
        }
    }

    depots
}

/// 从目录中的.manifest文件提取manifest ID
fn extract_manifest_ids(dir: &Path, _depots: &[(String, String)]) -> std::collections::HashMap<String, String> {
    let mut manifest_map = std::collections::HashMap::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "manifest" {
                    if let Some(filename) = path.file_stem() {
                        let name = filename.to_string_lossy().to_string();
                        if let Some(underscore_pos) = name.find('_') {
                            let depot_id = &name[..underscore_pos];
                            let manifest_id = &name[underscore_pos + 1..];
                            manifest_map.insert(depot_id.to_string(), manifest_id.to_string());
                        }
                    }
                }
            }
        }
    }

    manifest_map
}

/// 生成Lua格式内容
fn generate_lua(
    main_app_id: u64,
    depots: &[(String, String)],
    manifest_map: &std::collections::HashMap<String, String>
) -> String {
    let mut lines = Vec::new();

    // 添加主App ID
    lines.push(format!("addappid({})", main_app_id));

    // 添加带密钥的depot
    for (depot_id, key) in depots {
        lines.push(format!("addappid({},0,\"{}\")", depot_id, key));
    }

    // 添加setManifestid
    for (depot_id, _) in depots {
        if let Some(manifest_id) = manifest_map.get(depot_id) {
            lines.push(format!("setManifestid({},\"{}\")", depot_id, manifest_id));
        }
    }

    lines.join("\n")
}

/// 将VDF文件转换为Lua文件
pub fn convert_vdf_to_lua_internal(vdf_path: &Path) -> Result<PathBuf, String> {
    // 读取VDF文件内容
    let content = fs::read_to_string(vdf_path).map_err(|e| format!("读取VDF文件失败: {}", e))?;

    // 解析VDF文件
    let depots = parse_vdf_content(&content);

    if depots.is_empty() {
        return Err("VDF文件中未找到depot信息".to_string());
    }

    // 计算主App ID
    let first_depot_id = depots[0].0.parse::<u64>().unwrap_or(0);
    let main_app_id = if first_depot_id > 0 { first_depot_id - 1 } else { 0 };

    // 从同目录提取manifest ID
    let parent = vdf_path.parent().unwrap_or(Path::new("."));
    let manifest_map = extract_manifest_ids(parent, &depots);

    // 生成Lua内容
    let lua_content = generate_lua(main_app_id, &depots, &manifest_map);

    // 生成输出文件路径
    let output_path = parent.join(format!("{}.lua", main_app_id));

    // 写入Lua文件
    fs::write(&output_path, lua_content).map_err(|e| format!("写入Lua文件失败: {}", e))?;

    Ok(output_path)
}

/// 导入清单到Steam
/// 将Lua文件复制到Steam/config/stplug-in/，Manifest文件复制到Steam/config/depotcache/
#[tauri::command]
pub fn import_manifest_to_steam(
    steam_path: String,
    lua_files: Vec<String>,
    manifest_files: Vec<String>,
    vdf_files: Vec<String>
) -> Result<serde_json::Value, String> {
    let steam_path = Path::new(&steam_path);

    // 检查Steam路径
    if !steam_path.exists() {
        return Err("Steam路径不存在".to_string());
    }

    // 构建目标路径
    let stplugin_dir = steam_path.join("config").join("stplug-in");
    let depotcache_dir = steam_path.join("config").join("depotcache");

    // 确保目标目录存在
    fs::create_dir_all(&stplugin_dir).map_err(|e| format!("创建stplug-in目录失败: {}", e))?;
    fs::create_dir_all(&depotcache_dir).map_err(|e| format!("创建depotcache目录失败: {}", e))?;

    let mut imported_lua = 0;
    let mut imported_manifest = 0;
    let mut converted_vdf = 0;

    // 处理VDF文件（转换为Lua）
    let mut converted_lua_files = Vec::new();
    for vdf_file in &vdf_files {
        let vdf_path = Path::new(vdf_file);
        match convert_vdf_to_lua_internal(vdf_path) {
            Ok(lua_path) => {
                converted_lua_files.push(lua_path.to_string_lossy().to_string());
                converted_vdf += 1;
            }
            Err(e) => {
                log::warn!("转换VDF文件失败 {}: {}", vdf_file, e);
            }
        }
    }

    // 合并原始Lua文件和转换后的Lua文件
    let all_lua_files: Vec<String> = lua_files.into_iter()
        .chain(converted_lua_files)
        .collect();

    // 复制Lua文件
    let mut lua_errors = Vec::new();
    for lua_file in &all_lua_files {
        let source = Path::new(lua_file);
        if let Some(filename) = source.file_name() {
            let dest = stplugin_dir.join(filename);
            match fs::copy(source, &dest) {
                Ok(_) => {
                    imported_lua += 1;
                    log::info!("已复制Lua文件: {} -> {}", lua_file, dest.display());
                }
                Err(e) => {
                    let err_msg = format!("复制Lua文件失败 {}: {}", lua_file, e);
                    log::error!("{}", err_msg);
                    lua_errors.push(err_msg);
                }
            }
        }
    }

    // 复制Manifest文件
    let mut manifest_errors = Vec::new();
    for manifest_file in &manifest_files {
        let source = Path::new(manifest_file);
        if let Some(filename) = source.file_name() {
            let dest = depotcache_dir.join(filename);
            match fs::copy(source, &dest) {
                Ok(_) => {
                    imported_manifest += 1;
                    log::info!("已复制Manifest文件: {} -> {}", manifest_file, dest.display());
                }
                Err(e) => {
                    let err_msg = format!("复制Manifest文件失败 {}: {}", manifest_file, e);
                    log::error!("{}", err_msg);
                    manifest_errors.push(err_msg);
                }
            }
        }
    }

    // 检查是否有错误
    if !lua_errors.is_empty() || !manifest_errors.is_empty() {
        let mut error_msg = String::from("清单入库部分失败:\n");
        for err in &lua_errors {
            error_msg.push_str(&format!("- {}\n", err));
        }
        for err in &manifest_errors {
            error_msg.push_str(&format!("- {}\n", err));
        }
        return Err(error_msg);
    }

    // 检查是否导入了任何文件
    if imported_lua == 0 && imported_manifest == 0 {
        return Err("没有成功导入任何文件，请检查文件是否存在".to_string());
    }

    Ok(json!({
        "success": true,
        "message": "清单入库完成",
        "importedLua": imported_lua,
        "importedManifest": imported_manifest,
        "convertedVdf": converted_vdf
    }))
}

/// 重启Steam
/// 1. 结束steam.exe进程
/// 2. 启动Steam
#[tauri::command]
pub fn restart_steam() -> Result<serde_json::Value, String> {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    // 从配置中读取Steam路径
    let config = crate::services::ConfigService::new();
    let app_config = config.get_config();
    let steam_path_str = app_config.game_dirs.steam_path
        .ok_or("未配置Steam路径，请前往全局设置配置")?;

    let steam_path = Path::new(&steam_path_str);

    // 检查Steam路径
    if !steam_path.exists() {
        return Err("Steam路径不存在".to_string());
    }

    // Steam.exe路径
    let steam_exe = steam_path.join("steam.exe");
    if !steam_exe.exists() {
        return Err("未找到steam.exe，请确认Steam路径正确".to_string());
    }

    // 1. 结束steam.exe进程
    #[cfg(target_os = "windows")]
    let kill_output = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new("taskkill")
            .args(["/F", "/IM", "steam.exe"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
    };
    
    #[cfg(not(target_os = "windows"))]
    let kill_output = Command::new("taskkill")
        .args(&["/F", "/IM", "steam.exe"])
        .output();

    match kill_output {
        Ok(output) => {
            if output.status.success() {
                log::info!("已结束steam.exe进程");
            } else {
                // 进程可能本来就不存在，继续执行
                log::info!("steam.exe进程未运行或结束失败，继续启动");
            }
        }
        Err(e) => {
            log::warn!("结束steam.exe进程时出错: {}", e);
        }
    }

    // 等待1秒确保进程完全结束
    thread::sleep(Duration::from_secs(1));

    // 2. 启动Steam（隐藏窗口）
    let steam_exe_str = steam_exe.to_string_lossy().to_string();
    #[cfg(target_os = "windows")]
    let spawn_result = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new(&steam_exe_str)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
    };
    #[cfg(not(target_os = "windows"))]
    let spawn_result = Command::new(&steam_exe_str).spawn();

    match spawn_result {
        Ok(_) => {
            log::info!("Steam启动成功");
            Ok(json!({
                "success": true,
                "message": "Steam重启成功"
            }))
        }
        Err(e) => {
            log::error!("启动Steam失败: {}", e);
            Err(format!("启动Steam失败: {}", e))
        }
    }
}

/// 检查游戏清单文件是否存在
/// 用于游戏详情页判断【入库Steam】按钮是否可用
#[tauri::command]
pub fn check_game_manifest_exists(app: AppHandle, game_id: String) -> Result<serde_json::Value, String> {
    // 获取资源目录路径
    let resource_dir = get_resource_dir(&app)?;
    // 构建manifest目录路径
    let manifest_dir = resource_dir.join("manifest").join(&game_id);

    // 检查目录是否存在
    if !manifest_dir.exists() {
        return Ok(json!({
            "exists": false,
            "hasLua": false,
            "hasVdf": false,
            "hasManifest": false
        }));
    }

    // 扫描文件
    let mut has_lua = false;
    let mut has_vdf = false;
    let mut has_manifest = false;

    if let Ok(entries) = fs::read_dir(&manifest_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                match ext.as_str() {
                    "lua" => has_lua = true,
                    "vdf" => has_vdf = true,
                    "manifest" => has_manifest = true,
                    _ => {}
                }
            }
        }
    }

    // 按钮可用条件：有lua或有vdf
    let can_import = has_lua || has_vdf;

    Ok(json!({
        "exists": manifest_dir.exists(),
        "hasLua": has_lua,
        "hasVdf": has_vdf,
        "hasManifest": has_manifest,
        "canImport": can_import
    }))
}

/// 导入游戏清单到Steam（用于游戏详情页）
/// 从 resources/manifest/{game_id}/ 读取清单文件并导入
#[tauri::command]
pub fn import_game_manifest_to_steam(app: AppHandle, game_id: String) -> Result<serde_json::Value, String> {
    // 从配置中读取Steam路径
    let config = crate::services::ConfigService::new();
    let app_config = config.get_config();
    let steam_path = app_config.game_dirs.steam_path
        .ok_or("未配置Steam路径，请前往全局设置配置")?;

    // 获取资源目录路径
    let resource_dir = get_resource_dir(&app)?;
    // 构建manifest目录路径
    let manifest_dir = resource_dir.join("manifest").join(&game_id);

    // 检查目录是否存在
    if !manifest_dir.exists() {
        return Err("未找到游戏清单目录".to_string());
    }

    // 扫描文件
    let mut lua_files = Vec::new();
    let mut manifest_files = Vec::new();
    let mut vdf_files = Vec::new();

    if let Ok(entries) = fs::read_dir(&manifest_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
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
    }

    // 检查是否有lua或vdf
    if lua_files.is_empty() && vdf_files.is_empty() {
        return Err("未找到.lua或.vdf文件".to_string());
    }

    // 调用通用的导入函数
    import_manifest_to_steam(steam_path, lua_files, manifest_files, vdf_files)
}

/// 首次使用清单入库配置
/// 打开SteamTools和示例文件夹，供用户完成初始化
#[tauri::command]
pub fn setup_manifest_import_first_time(app: AppHandle) -> Result<serde_json::Value, String> {
    use std::process::Command;

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    log::info!("资源目录: {}", resource_dir.display());

    // SteamTools路径
    let steamtools_exe = resource_dir.join("SteamTools").join("SteamTools.exe");
    log::info!("SteamTools路径: {}", steamtools_exe.display());

    // 示例文件夹路径
    let example_folder = resource_dir.join("第一次使用清单入库请将这个文件夹中的内容拖入_steamtools_的图标");
    log::info!("示例文件夹路径: {}", example_folder.display());

    // 检查SteamTools是否存在
    if !steamtools_exe.exists() {
        log::error!("未找到SteamTools.exe: {}", steamtools_exe.display());
        return Err(format!("未找到SteamTools.exe，路径: {}", steamtools_exe.display()));
    }

    // 检查示例文件夹是否存在
    if !example_folder.exists() {
        log::error!("未找到示例文件夹: {}", example_folder.display());
        return Err(format!("未找到示例文件夹，路径: {}", example_folder.display()));
    }

    // 启动SteamTools - 使用Command直接启动
    let steamtools_path_str = steamtools_exe.to_string_lossy().to_string();
    log::info!("正在启动SteamTools: {}", steamtools_path_str);

    #[cfg(target_os = "windows")]
    let steam_result = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new(&steamtools_path_str)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
    };

    #[cfg(not(target_os = "windows"))]
    let steam_result = Command::new(&steamtools_path_str).spawn();

    match steam_result {
        Ok(_) => {
            log::info!("已启动SteamTools: {}", steamtools_exe.display());
        }
        Err(e) => {
            log::error!("启动SteamTools失败: {}", e);
            return Err(format!("启动SteamTools失败: {}", e));
        }
    }

    // 打开示例文件夹 - 使用explorer.exe
    let example_folder_str = example_folder.to_string_lossy().to_string();
    log::info!("正在打开示例文件夹: {}", example_folder_str);

    #[cfg(target_os = "windows")]
    let folder_result = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new("explorer.exe")
            .arg(&example_folder_str)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
    };

    #[cfg(not(target_os = "windows"))]
    let folder_result = Command::new("xdg-open")
        .arg(&example_folder_str)
        .spawn();

    match folder_result {
        Ok(_) => {
            log::info!("已打开示例文件夹: {}", example_folder.display());
        }
        Err(e) => {
            log::error!("打开示例文件夹失败: {}", e);
            // 不返回错误，因为SteamTools已经启动了
        }
    }

    Ok(json!({
        "success": true,
        "message": "SteamTools和示例文件夹已打开"
    }))
}

/// 打开SteamTools
/// 供用户手动启动SteamTools
#[tauri::command]
pub fn open_steamtools(app: AppHandle) -> Result<serde_json::Value, String> {
    use std::process::Command;

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    let steamtools_exe = resource_dir.join("SteamTools").join("SteamTools.exe");

    // 检查SteamTools是否存在
    if !steamtools_exe.exists() {
        return Err(format!("未找到SteamTools.exe，路径: {}", steamtools_exe.display()));
    }

    // 启动SteamTools
    let steamtools_path_str = steamtools_exe.to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    let result = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new(&steamtools_path_str)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
    };

    #[cfg(not(target_os = "windows"))]
    let result = Command::new(&steamtools_path_str).spawn();

    match result {
        Ok(_) => {
            log::info!("已启动SteamTools: {}", steamtools_exe.display());
            Ok(json!({
                "success": true,
                "message": "SteamTools已启动"
            }))
        }
        Err(e) => {
            log::error!("启动SteamTools失败: {}", e);
            Err(format!("启动SteamTools失败: {}", e))
        }
    }
}

/// 打开示例文件夹
/// 供用户手动打开示例文件夹
#[tauri::command]
pub fn open_example_folder(app: AppHandle) -> Result<serde_json::Value, String> {
    use std::process::Command;

    // 获取资源目录
    let resource_dir = get_resource_dir(&app)?;
    let example_folder = resource_dir.join("第一次使用清单入库请将这个文件夹中的内容拖入_steamtools_的图标");

    // 检查示例文件夹是否存在
    if !example_folder.exists() {
        return Err(format!("未找到示例文件夹，路径: {}", example_folder.display()));
    }

    // 打开示例文件夹
    let example_folder_str = example_folder.to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    let result = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new("explorer.exe")
            .arg(&example_folder_str)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
    };

    #[cfg(not(target_os = "windows"))]
    let result = Command::new("xdg-open")
        .arg(&example_folder_str)
        .spawn();

    match result {
        Ok(_) => {
            log::info!("已打开示例文件夹: {}", example_folder.display());
            Ok(json!({
                "success": true,
                "message": "示例文件夹已打开"
            }))
        }
        Err(e) => {
            log::error!("打开示例文件夹失败: {}", e);
            Err(format!("打开示例文件夹失败: {}", e))
        }
    }
}

/// 在指定深度内查找包含 .lua 或 .vdf 文件的文件夹
/// 返回最浅层的匹配文件夹路径，优先返回同一层的结果
fn find_manifest_folder(dir: &Path, max_depth: usize) -> Option<PathBuf> {
    fn search(current: &Path, depth: usize, max_depth: usize) -> Option<PathBuf> {
        if depth > max_depth {
            return None;
        }

        // 先检查当前目录是否包含目标文件
        let has_manifest_file = fs::read_dir(current)
            .ok()?
            .flatten()
            .filter(|e| e.path().is_file())
            .any(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| {
                        let ext = ext.to_lowercase();
                        ext == "lua" || ext == "vdf"
                    })
                    .unwrap_or(false)
            });

        if has_manifest_file {
            return Some(current.to_path_buf());
        }

        // 再递归检查子目录
        if let Ok(entries) = fs::read_dir(current) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(found) = search(&path, depth + 1, max_depth) {
                        return Some(found);
                    }
                }
            }
        }

        None
    }

    search(dir, 0, max_depth)
}

/// 将清单 7z 压缩包解压到 resources/manifest/{game_id}/ 目录
/// 用于游戏详情页下载游戏本体时，手动选择清单压缩包并解压
/// 解压流程：
/// 1. 先将压缩包解压到临时目录
/// 2. 在临时目录的 2 层子目录内搜索包含 .lua 或 .vdf 的文件夹
/// 3. 将找到的文件夹内容复制到 resources/manifest/{game_id}/
/// 4. 清理临时目录
/// 5. 若 2 层内未找到 .lua 或 .vdf，返回错误并清理临时目录
#[tauri::command]
pub fn extract_manifest_archive(
    app: AppHandle,
    archive_path: String,
    game_id: String,
) -> Result<String, String> {
    let resource_dir = get_resource_dir(&app)?;
    let target_dir = resource_dir.join("manifest").join(&game_id);

    // 如果目标目录已存在，先清理旧文件
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)
            .map_err(|e| format!("清理旧清单目录失败: {}", e))?;
    }

    // 创建目标目录
    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("创建清单目录失败: {}", e))?;

    // 创建临时解压目录
    let archive = Path::new(&archive_path);
    let file_stem = archive
        .file_stem()
        .ok_or("无法获取文件名")?
        .to_string_lossy();
    let temp_dir = std::env::temp_dir()
        .join(format!("steam_tool_manifest_{}_{}", game_id, file_stem));

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)
            .map_err(|e| format!("清理旧临时目录失败: {}", e))?;
    }
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("创建临时解压目录失败: {}", e))?;

    // 使用 sevenz-rust 解压 7z 文件到临时目录
    sevenz_rust::decompress_file(archive, &temp_dir)
        .map_err(|e| format!("解压清单压缩包失败: {:?}", e))?;

    // 在 2 层以内查找包含 .lua 或 .vdf 的文件夹
    let source_dir = match find_manifest_folder(&temp_dir, 2) {
        Some(dir) => dir,
        None => {
            // 未找到清单文件，清理临时目录和目标目录
            if let Err(e) = fs::remove_dir_all(&temp_dir) {
                log::warn!("清理临时解压目录失败: {}", e);
            }
            if let Err(e) = fs::remove_dir_all(&target_dir) {
                log::warn!("清理空目标目录失败: {}", e);
            }
            return Err("未在压缩包中找到.lua或.vdf文件".to_string());
        }
    };

    // 将定位到的文件夹内容复制到目标目录
    copy_dir_all(&source_dir, &target_dir)
        .map_err(|e| format!("复制清单文件到目标目录失败: {}", e))?;

    // 清理临时解压目录
    if let Err(e) = fs::remove_dir_all(&temp_dir) {
        log::warn!("清理临时解压目录失败: {}", e);
    }

    log::info!(
        "已将清单从 {} 移动到 {}",
        source_dir.display(),
        target_dir.display()
    );

    Ok(target_dir.to_string_lossy().to_string())
}

/// 递归复制源目录到目标目录
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// 将用户选择的清单文件夹复制到 resources/manifest/{game_id}/ 目录
/// 用于已解压的清单文件夹场景
#[tauri::command]
pub fn copy_folder_to_manifest(
    app: AppHandle,
    source_path: String,
    game_id: String,
) -> Result<String, String> {
    let resource_dir = get_resource_dir(&app)?;
    let target_dir = resource_dir.join("manifest").join(&game_id);

    // 如果目标目录已存在，先清理旧文件
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)
            .map_err(|e| format!("清理旧清单目录失败: {}", e))?;
    }

    // 创建目标目录并复制内容
    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("创建清单目录失败: {}", e))?;

    copy_dir_all(Path::new(&source_path), &target_dir)
        .map_err(|e| format!("复制清单文件夹失败: {}", e))?;

    log::info!(
        "已将清单文件夹 {} 复制到 {}",
        source_path,
        target_dir.display()
    );

    Ok(target_dir.to_string_lossy().to_string())
}
