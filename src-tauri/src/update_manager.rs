// SteamToolPlus 自更新管理器
// 负责：实例冲突检测、7z 升级补丁下载/解压/应用、exe 自更新、资源热更新

use futures_util::StreamExt;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{ProcessesToUpdate, System};
use tauri::Emitter;

/// 主程序可执行文件名
/// 发布版本统一使用 SteamToolPlus.exe，所有涉及 exe 更新的逻辑都以此为准
const EXE_NAME: &str = "SteamToolPlus.exe";

/// 在目录中递归查找指定名称的文件（不区分大小写）
/// 用于从解压后的补丁包中定位主程序 exe，兼容不同打包路径
fn find_file_case_insensitive(dir: &Path, target: &str) -> Option<PathBuf> {
    let target_lower = target.to_lowercase();
    let entries = fs::read_dir(dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.to_lowercase() == target_lower)
                .unwrap_or(false)
            {
                return Some(path);
            }
        } else if path.is_dir() {
            if let Some(found) = find_file_case_insensitive(&path, target) {
                return Some(found);
            }
        }
    }

    None
}

/// 资源版本信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceVersion {
    pub version: String,
    pub description: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub has_exe_update: bool,
}

/// 远程资源更新信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteResourceInfo {
    pub version: String,
    pub patch_url: String,
    pub description: Option<String>,
    pub has_exe_update: bool,
}

/// 应用版本信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppVersionInfo {
    pub app_version: String,
    pub resource_version: String,
}

/// 更新检查结果
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCheckResult {
    pub has_update: bool,
    pub local_app_version: String,
    pub local_resource_version: String,
    pub remote_resource_version: String,
    pub remote_description: Option<String>,
    pub has_exe_update: bool,
    pub patch_url: String,
}

/// 实例重启请求文件路径
fn instance_control_path() -> PathBuf {
    crate::utils::config_path_utils::get_appdata_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("instance-control.json")
}

/// 获取当前 exe 所在目录
fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .expect("无法获取当前 exe 路径")
        .parent()
        .expect("无法获取 exe 所在目录")
        .to_path_buf()
}

/// 获取当前 exe 完整路径
fn get_exe_path() -> PathBuf {
    std::env::current_exe().expect("无法获取当前 exe 路径")
}

/// 读取本地资源版本
pub fn read_local_resource_version() -> Option<ResourceVersion> {
    let version_path = get_exe_dir().join("resources").join("version.json");
    if !version_path.exists() {
        return None;
    }

    fs::read_to_string(&version_path)
        .ok()
        .and_then(|content| serde_json::from_str::<ResourceVersion>(&content).ok())
}

/// 比较两个版本号（格式：yyyy.m.d.n）
/// 返回 true 如果 remote > local
fn is_remote_newer(local: &str, remote: &str) -> bool {
    let parse = |s: &str| {
        s.split('.')
            .filter_map(|part| part.parse::<u32>().ok())
            .collect::<Vec<_>>()
    };

    let local_parts = parse(local);
    let remote_parts = parse(remote);

    for i in 0..local_parts.len().max(remote_parts.len()) {
        let l = local_parts.get(i).copied().unwrap_or(0);
        let r = remote_parts.get(i).copied().unwrap_or(0);
        match r.cmp(&l) {
            std::cmp::Ordering::Greater => return true,
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Equal => continue,
        }
    }

    false
}

/// 在程序启动前处理实例冲突
/// 如果检测到同名进程但路径不同，发送重启请求并等待旧进程退出
pub fn handle_instance_conflicts() {
    let current_exe = match get_exe_path().canonicalize() {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::All, true);

    let current_pid = std::process::id();
    let exe_name = current_exe
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(EXE_NAME);

    let conflicting_pids: Vec<u32> = system
        .processes_by_exact_name(exe_name.as_ref())
        .filter_map(|process| {
            let pid = process.pid().as_u32();
            if pid == current_pid {
                return None;
            }

            let process_exe = process
                .exe()
                .and_then(|p| p.canonicalize().ok());

            match process_exe {
                Some(other_exe) if other_exe != current_exe => Some(pid),
                _ => None,
            }
        })
        .collect();

    if conflicting_pids.is_empty() {
        return;
    }

    log::info!(
        "检测到 {} 个路径不同的旧实例正在运行，准备请求其退出",
        conflicting_pids.len()
    );

    // 写入重启请求文件
    let control_data = serde_json::json!({
        "action": "restart",
        "new_path": current_exe.to_string_lossy().to_string(),
        "pids": conflicting_pids,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    if let Err(e) = fs::write(instance_control_path(), control_data.to_string()) {
        log::error!("写入实例控制文件失败: {}", e);
        return;
    }

    // 等待旧进程退出，最多 10 秒
    let start = Instant::now();
    let timeout = Duration::from_secs(10);

    loop {
        system.refresh_processes(ProcessesToUpdate::All, true);
        let still_running: Vec<u32> = conflicting_pids
            .iter()
            .copied()
            .filter(|pid| system.process(sysinfo::Pid::from_u32(*pid)).is_some())
            .collect();

        if still_running.is_empty() {
            log::info!("旧实例已全部退出");
            break;
        }

        if start.elapsed() >= timeout {
            log::warn!("旧实例未在 10 秒内退出，强制结束: {:?}", still_running);
            for pid in still_running {
                if let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) {
                    let _ = process.kill();
                }
            }
            thread::sleep(Duration::from_millis(500));
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }

    // 清理控制文件
    let _ = fs::remove_file(instance_control_path());
}

/// 启动后台线程，监听来自新实例的重启请求
pub fn spawn_instance_control_watcher(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let control_path = instance_control_path();
        let current_exe = get_exe_path();
        let mut has_emitted = false;

        loop {
            thread::sleep(Duration::from_millis(500));

            if !control_path.exists() {
                has_emitted = false;
                continue;
            }

            let content = match fs::read_to_string(&control_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let data: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let action = data["action"].as_str().unwrap_or("");
            let new_path = data["new_path"].as_str().unwrap_or("").to_string();

            if action != "restart" || new_path.is_empty() {
                continue;
            }

            // 检查请求是否指向当前进程
            let requested_path = match PathBuf::from(&new_path).canonicalize() {
                Ok(p) => p,
                Err(_) => continue,
            };
            let current_path = match current_exe.canonicalize() {
                Ok(p) => p,
                Err(_) => continue,
            };

            if requested_path != current_path {
                // 这个重启请求是给另一个旧实例的，忽略
                continue;
            }

            if !has_emitted {
                log::info!("收到新实例的重启请求，新路径: {}", new_path);

                // 通知前端显示重启提示
                let _ = app_handle.emit(
                    "instance-restart-request",
                    serde_json::json!({ "new_path": new_path }),
                );

                has_emitted = true;
            }

            // 不删除控制文件，由新实例在旧进程退出后清理
        }
    });
}

/// 检查当前进程是否收到来自新实例的重启请求
/// 返回新实例路径，如果没有则返回 None
#[tauri::command]
pub fn check_instance_restart_request() -> Option<String> {
    let control_path = instance_control_path();
    if !control_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&control_path).ok()?;
    let data: serde_json::Value = serde_json::from_str(&content).ok()?;

    let action = data["action"].as_str()?;
    let new_path = data["new_path"].as_str()?;

    if action != "restart" || new_path.is_empty() {
        return None;
    }

    // 检查请求是否指向当前进程
    let requested_path = PathBuf::from(new_path).canonicalize().ok()?;
    let current_path = get_exe_path().canonicalize().ok()?;

    if requested_path != current_path {
        return None;
    }

    Some(new_path.to_string())
}

/// 获取远程资源版本信息
/// 从 Gitee Release 列表中查找最新的 resources-v* 标签，
/// 然后下载该 Release 中的 version.json 作为版本信息。
async fn fetch_remote_resource_info() -> Result<RemoteResourceInfo, String> {
    let owner = "Jingluohi";
    let repo = "steam-tool-plus";
    let releases_url = format!(
        "https://gitee.com/api/v5/repos/{}/{}/releases",
        owner, repo
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    // 1. 获取 Release 列表
    let response = client
        .get(&releases_url)
        .send()
        .await
        .map_err(|e| format!("请求 Gitee Release 列表失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Gitee Release 列表返回错误: {}", response.status()));
    }

    let releases: Vec<serde_json::Value> = response
        .json()
        .await
        .map_err(|e| format!("解析 Release 列表失败: {}", e))?;

    // 2. 筛选 resources-v* 标签并找出最新版本
    const TAG_PREFIX: &str = "resources-v";
    let mut latest_tag: Option<String> = None;
    let mut latest_version = "0.0.0.0".to_string();

    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        if let Some(version) = tag.strip_prefix(TAG_PREFIX) {
            if is_remote_newer(&latest_version, version) {
                latest_version = version.to_string();
                latest_tag = Some(tag.to_string());
            }
        }
    }

    let tag = latest_tag.ok_or("未找到资源更新 Release（标签格式应为 resources-v*）")?;

    // 3. 从最新 Release 下载 version.json
    let version_url = format!(
        "https://gitee.com/{}/{}/releases/download/{}/version.json",
        owner, repo, tag
    );

    let version_resp = client
        .get(&version_url)
        .send()
        .await
        .map_err(|e| format!("下载 version.json 失败: {}", e))?;

    if !version_resp.status().is_success() {
        return Err(format!("version.json 下载失败: {}", version_resp.status()));
    }

    let version: ResourceVersion = version_resp
        .json()
        .await
        .map_err(|e| format!("解析 version.json 失败: {}", e))?;

    // 4. 补丁下载地址固定为同一 Release 中的 升级补丁.7z
    let patch_url = format!(
        "https://gitee.com/{}/{}/releases/download/{}/升级补丁.7z",
        owner, repo, tag
    );

    Ok(RemoteResourceInfo {
        version: version.version,
        patch_url,
        description: version.description,
        has_exe_update: version.has_exe_update,
    })
}

/// 下载文件到指定路径，并实时推送下载进度事件
/// app_handle 用于向前端发送 download-progress 事件：{ url, downloaded, total, percent }
async fn download_file_with_progress(
    url: &str,
    dest: &Path,
    app_handle: Option<&tauri::AppHandle>,
) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("下载文件失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("下载文件失败，HTTP 状态码: {}", response.status()));
    }

    let total_size = response
        .content_length()
        .unwrap_or(0);

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    let mut buffer = Vec::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| format!("读取下载数据失败: {}", e))?;
        downloaded += chunk.len() as u64;
        buffer.extend_from_slice(&chunk);

        if let Some(handle) = app_handle {
            let percent = if total_size > 0 {
                (downloaded as f64 / total_size as f64 * 100.0) as u32
            } else {
                0
            };
            let _ = handle.emit(
                "download-progress",
                serde_json::json!({
                    "url": url,
                    "downloaded": downloaded,
                    "total": total_size,
                    "percent": percent,
                }),
            );
        }
    }

    fs::write(dest, buffer).map_err(|e| format!("保存下载文件失败: {}", e))?;

    Ok(())
}

/// 获取当前应用版本和资源版本
#[tauri::command]
pub fn get_app_versions() -> Result<AppVersionInfo, String> {
    let app_version = env!("CARGO_PKG_VERSION").to_string();
    let resource_version = match read_local_resource_version() {
        Some(v) => v.version,
        None => "未设置".to_string(),
    };

    Ok(AppVersionInfo {
        app_version,
        resource_version,
    })
}

/// 检查是否存在可用更新
#[tauri::command]
pub async fn check_for_update() -> Result<UpdateCheckResult, String> {
    let local_version = match read_local_resource_version() {
        Some(v) => v,
        None => {
            return Err("未找到本地 resources/version.json，无法检查更新".to_string());
        }
    };

    let remote_info = fetch_remote_resource_info().await?;
    let has_update = is_remote_newer(&local_version.version, &remote_info.version);

    Ok(UpdateCheckResult {
        has_update,
        local_app_version: env!("CARGO_PKG_VERSION").to_string(),
        local_resource_version: local_version.version,
        remote_resource_version: remote_info.version,
        remote_description: remote_info.description,
        has_exe_update: remote_info.has_exe_update,
        patch_url: remote_info.patch_url,
    })
}

/// 应用更新
/// 下载并应用 7z 补丁，包含 exe 时执行自更新并重启；仅资源更新时返回成功
#[tauri::command]
pub async fn apply_update(
    app_handle: tauri::AppHandle,
    patch_url: String,
    has_exe_update: bool,
) -> Result<bool, String> {
    let exe_dir = get_exe_dir();
    let temp_dir = std::env::temp_dir().join("steam_tool_updates");
    let _ = fs::create_dir_all(&temp_dir);

    let patch_path = temp_dir.join("升级补丁.7z");

    // 下载补丁（带进度）
    if let Err(e) = download_file_with_progress(&patch_url, &patch_path, Some(&app_handle)).await {
        log::error!("下载升级补丁失败: {}", e);
        let _ = app_handle.emit("resource-update-error", serde_json::json!({ "error": e }));
        return Err(e);
    }

    // 应用补丁
    let (patch_has_exe, exe_update_source, patch_temp_dir) = match apply_patch_archive(&patch_path, &exe_dir).await {
        Ok(result) => result,
        Err(e) => {
            log::error!("应用升级补丁失败: {}", e);
            let _ = app_handle.emit("resource-update-error", serde_json::json!({ "error": e }));
            return Err(e);
        }
    };

    // 实际是否包含 exe 以补丁内文件为准，但参数 has_exe_update 用于决定后续是否重启
    let really_has_exe = patch_has_exe || has_exe_update;

    // 更新本地 version.json
    let remote_info = fetch_remote_resource_info().await?;
    let new_version = ResourceVersion {
        version: remote_info.version.clone(),
        description: remote_info.description.clone(),
        updated_at: Some(chrono::Local::now().format("%Y-%m-%d").to_string()),
        has_exe_update: really_has_exe,
    };

    let version_path = exe_dir.join("resources").join("version.json");
    if let Err(e) = fs::write(
        &version_path,
        serde_json::to_string_pretty(&new_version).unwrap_or_default(),
    ) {
        log::error!("更新本地 version.json 失败: {}", e);
    }

    // 删除补丁文件
    let _ = fs::remove_file(&patch_path);

    if really_has_exe {
        if let Some(new_exe_source) = exe_update_source {
            if let Err(e) = apply_exe_update(&exe_dir, &new_exe_source) {
                log::error!("应用 exe 更新失败: {}", e);
                let _ = fs::remove_dir_all(&patch_temp_dir);
                return Err(e);
            }
        }

        let _ = fs::remove_dir_all(&patch_temp_dir);
        cleanup_old_backups(&exe_dir);

        let exe_path = get_exe_path();
        if let Err(e) = restart_application(&exe_path) {
            log::error!("重启应用程序失败: {}", e);
            return Err(e);
        }

        return Ok(true);
    }

    // 不含 exe 更新，清理补丁临时目录
    let _ = fs::remove_dir_all(&patch_temp_dir);

    let _ = app_handle.emit(
        "resource-update-finished",
        serde_json::json!({
            "version": remote_info.version,
            "has_exe_update": false,
        }),
    );

    Ok(false)
}

/// 应用 7z 升级补丁
/// 返回是否包含 exe 更新、新 exe 的临时路径、临时目录路径
async fn apply_patch_archive(
    patch_path: &Path,
    exe_dir: &Path,
) -> Result<(bool, Option<PathBuf>, PathBuf), String> {
    let temp_dir = std::env::temp_dir().join(format!(
        "steam_tool_patch_{}",
        chrono::Utc::now().timestamp_millis()
    ));

    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("创建补丁临时目录失败: {}", e))?;

    // 解压补丁
    let patch_path = patch_path.to_path_buf();
    let temp_dir_clone = temp_dir.clone();
    let extract_result = tokio::task::spawn_blocking(move || {
        let mut archive = zesven::Archive::open_path(&patch_path)
            .map_err(|e| format!("打开补丁文件失败: {}", e))?;
        archive
            .extract(&temp_dir_clone, (), &zesven::ExtractOptions::default())
            .map_err(|e| format!("解压补丁失败: {}", e))
    })
    .await
    .map_err(|e| format!("解压任务异常: {}", e))?;

    if let Err(e) = extract_result {
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(e);
    }

    // 判断补丁中是否包含 exe
    // 递归查找，不区分大小写，兼容不同打包路径下的 SteamToolPlus.exe
    let patch_exe = find_file_case_insensitive(&temp_dir, EXE_NAME);
    let has_exe_update = patch_exe.is_some();

    // 备份 games_config.json（如果存在）
    let games_config_src = exe_dir.join("resources").join("games_config.json");
    let games_config_backup = exe_dir.join("resources").join("games_config.json.bak");
    if games_config_src.exists() {
        fs::copy(&games_config_src, &games_config_backup)
            .map_err(|e| format!("备份 games_config.json 失败: {}", e))?;
    }

    // 复制资源文件到 exe 目录（exe 单独处理，不在这里复制）
    copy_patch_files(&temp_dir, exe_dir)?;

    Ok((has_exe_update, patch_exe, temp_dir))
}

/// 递归复制补丁文件到目标目录
fn copy_patch_files(source: &Path, target: &Path) -> Result<(), String> {
    for entry in fs::read_dir(source).map_err(|e| format!("读取补丁目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取补丁目录项失败: {}", e))?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dest_path = target.join(&file_name);

        // 跳过主程序 exe，exe 更新由调用方单独处理
        // 使用不区分大小写匹配，兼容历史补丁中的不同命名
        if file_name
            .to_string_lossy()
            .to_lowercase()
            .eq(&EXE_NAME.to_lowercase())
        {
            continue;
        }

        if src_path.is_dir() {
            fs::create_dir_all(&dest_path)
                .map_err(|e| format!("创建目标目录失败: {}", e))?;
            copy_patch_files(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)
                .map_err(|e| format!("复制文件 {} 失败: {}", src_path.display(), e))?;
        }
    }

    Ok(())
}

/// 执行 exe 自更新
/// new_exe_source 是补丁中解压出的新 exe 路径
fn apply_exe_update(exe_dir: &Path, new_exe_source: &Path) -> Result<(), String> {
    if !new_exe_source.exists() {
        return Err(format!(
            "新 exe 文件不存在: {}",
            new_exe_source.display()
        ));
    }

    let current_exe = get_exe_path();
    let current_exe_name = current_exe
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(EXE_NAME);
    let old_exe = exe_dir.join(format!("{}.old", current_exe_name));

    // 删除已存在的旧备份（理论上不应存在，但防御性处理）
    if old_exe.exists() {
        let _ = fs::remove_file(&old_exe);
    }

    // 重命名当前运行的 exe 为 .old
    fs::rename(&current_exe, &old_exe)
        .map_err(|e| format!("重命名旧 exe 失败: {}", e))?;

    // 复制新 exe 到当前运行位置
    fs::copy(new_exe_source, &current_exe)
        .map_err(|e| format!("复制新 exe 失败: {}", e))?;

    Ok(())
}

/// 清理旧的 .old 备份文件
fn cleanup_old_backups(exe_dir: &Path) {
    // 清理当前 exe 对应的 .old 备份
    let current_exe = get_exe_path();
    let current_exe_name = current_exe
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(EXE_NAME);
    let old_exe = exe_dir.join(format!("{}.old", current_exe_name));
    if old_exe.exists() {
        let _ = fs::remove_file(&old_exe);
    }

    // 同时清理历史命名遗留的 .old 文件
    let legacy_old_exe = exe_dir.join(format!("{}.old", EXE_NAME));
    if legacy_old_exe != old_exe && legacy_old_exe.exists() {
        let _ = fs::remove_file(&legacy_old_exe);
    }
}

/// 重启应用程序
fn restart_application(exe_path: &Path) -> Result<(), String> {
    Command::new(exe_path)
        .spawn()
        .map_err(|e| format!("启动新实例失败: {}", e))?;

    std::process::exit(0);
}


