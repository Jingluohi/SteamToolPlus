/**
 * manifest_commands.rs - 清单入库命令模块
 * 提供清单文件扫描、解压、VDF转Lua、复制到资源目录等功能
 */
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use serde_json::json;
use crate::services::config_service::ConfigServiceTrait;
use crate::utils::resource_utils::get_resource_dir;
use tauri::AppHandle;

/// 最大递归扫描深度限制，防止无限递归或目录层级过深
const MAX_SCAN_DEPTH: u32 = 10;

/// 扫描文件夹中的清单文件
/// 递归查找.lua、.manifest、.vdf文件，最大扫描深度为 MAX_SCAN_DEPTH
#[tauri::command]
pub fn scan_manifest_folder(folder_path: String) -> Result<serde_json::Value, String> {
    let mut lua_files = Vec::new();
    let mut manifest_files = Vec::new();
    let mut vdf_files = Vec::new();

    /// 内部递归扫描函数
    /// # 参数
    /// - `dir`: 当前扫描目录
    /// - `lua`: lua文件列表
    /// - `manifest`: manifest文件列表
    /// - `vdf`: vdf文件列表
    /// - `current_depth`: 当前递归深度
    fn scan_directory(
        dir: &Path,
        lua: &mut Vec<String>,
        manifest: &mut Vec<String>,
        vdf: &mut Vec<String>,
        current_depth: u32,
    ) -> Result<(), String> {
        // 检查是否达到最大扫描深度
        if current_depth >= MAX_SCAN_DEPTH {
            log::warn!("达到最大扫描深度 {}, 停止扫描: {}", MAX_SCAN_DEPTH, dir.display());
            return Ok(());
        }

        let entries = fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // 递归调用时深度+1
                scan_directory(&path, lua, manifest, vdf, current_depth + 1)?;
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

    // 从深度0开始扫描
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
    let re = Regex::new(r#""DecryptionKey"\s+"([a-fA-F0-9]+)""#).unwrap();

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

/// 生成Lua格式内容
///
/// # 参数
/// - `main_app_id`: 主游戏AppID
/// - `depots`: depot列表，每项为(depot_id, decryption_key)
/// - `manifest_map`: depot_id到manifest_id的映射（仅在lock_version=true时使用）
/// - `access_token`: 可选的访问令牌，用于下载受保护的游戏/DLC
/// - `stats_steam_id`: 可选的SteamID，用于拉取该账号的成就数据
/// - `lock_version`: 是否锁定版本（生成 setManifestid）
///
/// # 说明
/// 当 `lock_version=true` 时生成 `setManifestid`，强制锁定 depot 到特定 manifest，
/// 适用于限定版本补丁。否则允许 Steam 自动更新。
fn generate_lua(
    main_app_id: u64,
    depots: &[(String, String)],
    manifest_map: &std::collections::HashMap<String, String>,
    access_token: Option<&str>,
    stats_steam_id: Option<&str>,
    lock_version: bool,
) -> String {
    let mut lines = Vec::new();

    // 添加主App ID
    lines.push(format!("addappid({})", main_app_id));

    // 添加带密钥的depot
    for (depot_id, key) in depots {
        lines.push(format!("addappid({},0,\"{}\")", depot_id, key));
    }

    // 添加访问令牌（用于受保护的游戏/DLC）
    if let Some(token) = access_token {
        let token = token.trim();
        if !token.is_empty() {
            lines.push(format!("addtoken({},\"{}\")", main_app_id, token));
        }
    }

    // 锁定版本：生成 setManifestid 强制锁定 depot 到特定 manifest
    // 仅在 lock_version=true 且存在 manifest_map 时生效
    if lock_version {
        for (depot_id, _) in depots {
            if let Some(manifest_id) = manifest_map.get(depot_id) {
                lines.push(format!("setManifestid({},\"{}\")", depot_id, manifest_id));
            }
        }
    }

    // 添加成就数据SteamID
    if let Some(steam_id) = stats_steam_id {
        let steam_id = steam_id.trim();
        if !steam_id.is_empty() {
            lines.push(format!("setStat({},\"{}\")", main_app_id, steam_id));
        }
    }

    lines.join("\n")
}

/// 从目录中的 .manifest 文件提取 manifest ID
///
/// # 参数
/// - `dir`: 包含 manifest 文件的目录
/// - `depots`: depot列表，用于筛选相关depot的manifest
///
/// # 返回
/// depot_id 到 manifest_id 的映射
fn extract_manifest_ids(dir: &Path, _depots: &[(String, String)]) -> std::collections::HashMap<String, String> {
    let mut manifest_map = std::collections::HashMap::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "manifest" {
                    // 解析文件名: {depot_id}_{manifest_id}.manifest
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

/// 将VDF文件转换为Lua文件
///
/// # 参数
/// - `vdf_path`: VDF文件路径
/// - `access_token`: 可选的访问令牌
/// - `stats_steam_id`: 可选的成就数据SteamID
/// - `lock_version`: 是否锁定版本（生成 setManifestid）
pub fn convert_vdf_to_lua_internal(
    vdf_path: &Path,
    access_token: Option<&str>,
    stats_steam_id: Option<&str>,
    lock_version: bool,
) -> Result<PathBuf, String> {
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

    // 从同目录提取 manifest ID（仅当需要锁定版本时才使用）
    let parent = vdf_path.parent().unwrap_or(Path::new("."));
    let manifest_map = if lock_version {
        extract_manifest_ids(parent, &depots)
    } else {
        std::collections::HashMap::new()
    };

    // 生成Lua内容
    let lua_content = generate_lua(main_app_id, &depots, &manifest_map, access_token, stats_steam_id, lock_version);

    // 生成输出文件路径
    let output_path = parent.join(format!("{}.lua", main_app_id));

    // 写入Lua文件
    fs::write(&output_path, lua_content).map_err(|e| format!("写入Lua文件失败: {}", e))?;

    Ok(output_path)
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

/// 在指定深度内查找包含 .lua 或 .vdf 文件的文件夹
/// 返回最浅层的匹配文件夹路径，优先返回同一层的结果
/// # 参数
/// - `dir`: 起始扫描目录
/// - `max_depth`: 最大扫描深度（将被限制为不超过 MAX_SCAN_DEPTH）
fn find_manifest_folder(dir: &Path, max_depth: usize) -> Option<PathBuf> {
    /// 内部递归搜索函数
    /// # 参数
    /// - `current`: 当前扫描目录
    /// - `current_depth`: 当前递归深度
    /// - `limit_depth`: 最大扫描深度限制
    fn search(current: &Path, current_depth: u32, limit_depth: u32) -> Option<PathBuf> {
        // 检查是否达到最大扫描深度
        if current_depth >= limit_depth {
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
                    // 递归调用时深度+1
                    if let Some(found) = search(&path, current_depth + 1, limit_depth) {
                        return Some(found);
                    }
                }
            }
        }

        None
    }

    // 将传入的 max_depth 限制为不超过 MAX_SCAN_DEPTH
    let limit_depth = std::cmp::min(max_depth as u32, MAX_SCAN_DEPTH);
    search(dir, 0, limit_depth)
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

    // 将定位到的文件夹内容复制到目标目录（从深度0开始）
    copy_dir_all(&source_dir, &target_dir, 0)
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
/// # 参数
/// - `src`: 源目录路径
/// - `dst`: 目标目录路径
/// - `current_depth`: 当前递归深度（内部使用）
fn copy_dir_all(src: &Path, dst: &Path, current_depth: u32) -> std::io::Result<()> {
    // 检查是否达到最大扫描深度
    if current_depth >= MAX_SCAN_DEPTH {
        log::warn!("达到最大复制深度 {}, 停止复制: {}", MAX_SCAN_DEPTH, src.display());
        return Ok(());
    }

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            // 递归调用时深度+1
            copy_dir_all(&src_path, &dst_path, current_depth + 1)?;
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

    // 创建目标目录并复制内容（从深度0开始）
    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("创建清单目录失败: {}", e))?;

    copy_dir_all(Path::new(&source_path), &target_dir, 0)
        .map_err(|e| format!("复制清单文件夹失败: {}", e))?;

    log::info!(
        "已将清单文件夹 {} 复制到 {}",
        source_path,
        target_dir.display()
    );

    Ok(target_dir.to_string_lossy().to_string())
}
