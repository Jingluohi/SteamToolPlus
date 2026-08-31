// 游戏数据管理命令
// 处理游戏数据的增删改查

use std::collections::HashSet;
use sysinfo::{Pid, ProcessesToUpdate, System};
use tauri::AppHandle;
use crate::services::game_data_service::{self, GameData};

/// 获取所有游戏数据
#[tauri::command]
pub async fn get_all_games_data(app: AppHandle) -> Result<Vec<GameData>, String> {
    game_data_service::get_all_games(app).await
}

/// 获取单个游戏数据
#[tauri::command]
pub async fn get_game_data(app: AppHandle, game_id: String) -> Result<Option<GameData>, String> {
    game_data_service::get_game(app, game_id).await
}

/// 添加或更新游戏数据
#[tauri::command]
pub async fn upsert_game_data(app: AppHandle, game: GameData) -> Result<(), String> {
    game_data_service::upsert_game(app, game).await
}

/// 删除游戏数据
#[tauri::command]
pub async fn remove_game_data(app: AppHandle, game_id: String) -> Result<(), String> {
    game_data_service::remove_game(app, game_id).await
}

/// 更新下载状态
#[tauri::command]
pub async fn update_game_download_status(
    app: AppHandle,
    game_id: String,
    status: String,
    progress: u32,
) -> Result<(), String> {
    game_data_service::update_download_status(app, game_id, status, progress).await
}

/// 更新游戏时长
#[tauri::command]
pub async fn update_game_play_time(
    app: AppHandle,
    game_id: String,
    additional_minutes: u64,
) -> Result<(), String> {
    game_data_service::update_play_time(app, game_id, additional_minutes).await
}

/// 删除游戏目录
/// 卸载游戏时使用，删除游戏安装目录但保留存档，同时清理下载日志目录
#[tauri::command]
pub async fn delete_game_directory(
    path: String,
    save_path: Option<String>,
    game_id: Option<String>,
) -> Result<(), String> {
    use std::fs;

    let path_obj = std::path::Path::new(&path);

    // 如果路径不存在，直接返回成功（用户可能已经手动删除了文件）
    // 这样前端可以继续更新游戏状态为未安装
    if !path_obj.exists() {
        return Ok(());
    }

    // 安全检查：确保路径是目录
    if !path_obj.is_dir() {
        return Err("路径不是目录".to_string());
    }

    // 如果有存档路径，先备份存档
    let save_backup: Option<(String, std::path::PathBuf)> = if let Some(ref save) = save_path {
        let save_obj = std::path::Path::new(save);
        if save_obj.exists() && save_obj.is_dir() {
            // 创建临时备份目录
            let temp_dir = std::env::temp_dir().join(format!("steamtool_save_backup_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()));

            // 复制存档到临时目录
            if let Err(e) = copy_dir_all(save_obj, &temp_dir) {
                eprintln!("备份存档失败: {}", e);
                None
            } else {
                Some((save.clone(), temp_dir))
            }
        } else {
            None
        }
    } else {
        None
    };

    // 删除游戏目录及其所有内容
    fs::remove_dir_all(&path)
        .map_err(|e| format!("删除游戏目录失败: {}", e))?;

    // 恢复存档
    if let Some((original_save_path, backup_path)) = save_backup {
        let save_obj = std::path::Path::new(&original_save_path);

        // 确保存档目录存在
        if let Some(parent) = save_obj.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // 从备份恢复存档
        if let Err(e) = copy_dir_all(&backup_path, save_obj) {
            eprintln!("恢复存档失败: {}", e);
        }

        // 清理临时备份
        let _ = fs::remove_dir_all(&backup_path);
    }

    // 如果提供了游戏ID，清理下载日志目录 %APPDATA%/SteamToolPlus/log/{game_id}
    if let Some(gid) = game_id {
        let log_dir = crate::utils::config_path_utils::get_appdata_dir()?
            .join("log")
            .join(&gid);
        if log_dir.exists() {
            if let Err(e) = fs::remove_dir_all(&log_dir) {
                eprintln!("清理下载日志目录失败 ({}): {}", log_dir.display(), e);
            }
        }
    }

    Ok(())
}

/// 递归复制目录
fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    use std::fs;

    if !dst.exists() {
        fs::create_dir_all(dst)
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    for entry in fs::read_dir(src).map_err(|e| format!("读取目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)
                .map_err(|e| format!("复制文件失败: {}", e))?;
        }
    }

    Ok(())
}

/// 打开游戏目录
/// 使用 Windows 资源管理器打开游戏安装目录
#[tauri::command]
pub async fn open_game_directory(path: String) -> Result<(), String> {
    use std::process::Command;
    
    let path_obj = std::path::Path::new(&path);
    
    // 安全检查：确保路径存在且是目录
    if !path_obj.exists() {
        return Err("游戏目录不存在".to_string());
    }
    
    if !path_obj.is_dir() {
        return Err("路径不是目录".to_string());
    }
    
    // 使用 explorer.exe 打开目录（隐藏窗口）
    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;
    #[cfg(target_os = "windows")]
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut cmd = Command::new("explorer.exe");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.arg(&path)
        .spawn()
        .map_err(|e| format!("打开游戏目录失败: {}", e))?;
    
    Ok(())
}

/// 检查游戏是否存在
#[tauri::command]
pub async fn check_game_exists(app: AppHandle, game_id: String) -> Result<bool, String> {
    game_data_service::game_exists(app, game_id).await
}

/// 检查所有已安装游戏目录，将目录缺失的游戏重置为未安装
#[tauri::command]
pub async fn reset_missing_installed_games(app: AppHandle) -> Result<Vec<GameData>, String> {
    game_data_service::reset_missing_installed_games(app).await
}

/// 导入自定义游戏
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn import_custom_game(
    app: AppHandle,
    game_name: String,
    chinese_name: String,
    install_path: String,
    exe_path: String,
    save_path: Option<String>,
    cover_path: Option<String>,
    steam_game_id: Option<String>,
) -> Result<GameData, String> {
    use chrono::Local;
    
    // 生成游戏ID（使用时间戳）
    let game_id = format!("custom_{}", Local::now().timestamp_millis());
    
    let game = GameData {
        game_id: game_id.clone(),
        game_name,
        chinese_name,
        game_type: "imported".to_string(),
        install_path,
        exe_path,
        save_path,
        cover_path,
        steam_game_id,
        is_favorite: false,
        is_installed: true,
        play_time: 0,
        last_played: None,
        added_date: Local::now().to_rfc3339(),
        download_status: "completed".to_string(),
        download_progress: 100,
        download_path: None,
    };
    
    game_data_service::upsert_game(app, game.clone()).await?;
    
    Ok(game)
}

/// 启动游戏并记录时间
/// 模仿 Steam 的检测方式：
/// 1. 启动游戏并记录初始 PID
/// 2. 等待游戏窗口出现（很多游戏启动器会先显示窗口）
/// 3. 持续监控游戏目录下的所有进程
/// 4. 检测进程树中的子进程
/// 返回游戏进程ID
#[tauri::command]
pub async fn launch_game_with_tracking(
    app: AppHandle,
    game_id: String,
) -> Result<u32, String> {
    use std::process::Command;
    use std::thread;
    use std::time::{Duration, Instant};
    
    // 获取游戏数据
    let game = game_data_service::get_game(app.clone(), game_id.clone()).await?
        .ok_or("游戏不存在")?;
    
    // 检查exe文件是否存在
    let exe_path = std::path::Path::new(&game.exe_path);
    if !exe_path.exists() {
        return Err(format!("游戏主程序不存在: {}", game.exe_path));
    }
    
    // 更新最后游玩时间
    let mut game_clone = game.clone();
    game_clone.last_played = Some(chrono::Local::now().to_rfc3339());
    game_data_service::upsert_game(app.clone(), game_clone).await?;
    
    // 启动游戏
    let _start_time = Instant::now();
    
    let mut child = Command::new(&game.exe_path)
        .current_dir(&game.install_path)
        .spawn()
        .map_err(|e| format!("启动游戏失败: {}", e))?;
    
    // 获取进程ID
    let initial_pid = child.id();
    
    // 获取exe文件名和安装目录（用于进程监控）
    let exe_name = exe_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    let install_path = game.install_path.clone();
    let _exe_path_str = game.exe_path.clone();
    
    // 在后台线程中监控游戏进程
    // 使用 std::thread 承载长时间阻塞的轮询，避免占用 tokio worker 线程
    let app_handle = app.clone();
    let game_id_clone = game_id.clone();
    // 获取当前异步运行时句柄，供后台线程复用，避免创建新的 Runtime
    let rt_handle = tokio::runtime::Handle::current();

    thread::spawn(move || {
        // 等待初始进程结束（很多游戏启动器会立即退出）
        let _ = child.wait();

        // 游戏启动时间
        let game_start_time = Instant::now();

        // 复用同一个 sysinfo System 实例做进程监控，
        // 替代原先每 2~3 秒 spawn wmic/tasklist 子进程的做法，消除周期性 CPU 尖峰
        let mut system = System::new();
        let mut known_pids: HashSet<u32> = HashSet::new();
        let mut known_names: HashSet<String> = HashSet::new();

        // 动态获取进程快照 - 多次尝试捕获游戏进程（处理启动器延迟启动的情况）
        let max_attempts = 10; // 最多尝试10次
        let attempt_interval = Duration::from_secs(2); // 每2秒尝试一次

        for _attempt in 1..=max_attempts {
            thread::sleep(attempt_interval);

            system.refresh_processes(ProcessesToUpdate::All, true);
            let snapshot = get_game_processes_snapshot(&system, &install_path, &exe_name, initial_pid);

            if !snapshot.is_empty() {
                for (pid, name) in snapshot {
                    known_pids.insert(pid);
                    if !name.is_empty() {
                        known_names.insert(name);
                    }
                }
                break;
            }
        }

        // 持续监控，直到确认游戏完全关闭
        let mut last_seen = Instant::now();
        let max_wait = Duration::from_secs(15); // 最多等待15秒确认进程完全关闭
        let check_interval = Duration::from_secs(3); // 每3秒检查一次

        loop {
            system.refresh_processes(ProcessesToUpdate::All, true);
            // 检查游戏是否还在运行（使用动态检测，不依赖初始快照）
            let is_running = game_process_is_running(
                &system, &install_path, &exe_name, initial_pid, &known_pids, &known_names,
            );

            if is_running {
                // 游戏还在运行，更新最后看到时间
                last_seen = Instant::now();
                thread::sleep(check_interval);
                continue;
            }

            // 游戏进程不在运行，检查是否已经过了足够时间确认完全关闭
            let elapsed = last_seen.elapsed();
            if elapsed >= max_wait {
                break;
            }

            // 继续等待确认
            thread::sleep(Duration::from_secs(1));
        }

        // 计算游玩时长（精确到分钟）
        let elapsed = game_start_time.elapsed();
        let minutes = elapsed.as_secs() / 60;

        // 记录游玩时长（即使不到1分钟也记录，但只记录大于0的）
        if elapsed.as_secs() > 0 {
            // 复用当前 tokio runtime 句柄执行异步保存，避免创建新 Runtime 带来的开销
            rt_handle.block_on(async {
                // 更新总游玩时长
                let _ = game_data_service::update_play_time(
                    app_handle.clone(),
                    game_id_clone.clone(),
                    minutes
                ).await;

                // 更新最后游玩时间
                if let Ok(Some(mut game)) = game_data_service::get_game(app_handle.clone(), game_id_clone.clone()).await {
                    game.last_played = Some(chrono::Local::now().to_rfc3339());
                    let _ = game_data_service::upsert_game(app_handle, game).await;
                }
            });
        }
    });

    Ok(initial_pid)
}

/// 获取游戏进程快照
/// 返回 (PID, 进程名) 的列表（基于 sysinfo 内存枚举，不 spawn 子进程）
fn get_game_processes_snapshot(
    system: &System,
    dir_path: &str,
    exe_name: &str,
    initial_pid: u32,
) -> Vec<(u32, String)> {
    let mut processes = Vec::new();
    let dir_lower = dir_path.to_lowercase();
    let exe_lower = exe_name.to_lowercase();
    let exe_base = exe_lower.replace(".exe", "");

    for process in system.processes().values() {
        let pid = process.pid().as_u32();
        let proc_name = process.name().to_string_lossy().to_lowercase();

        // 检查是否是我们启动的初始进程
        if pid == initial_pid {
            if !processes.iter().any(|(p, _)| *p == pid) {
                processes.push((pid, proc_name.clone()));
            }
            continue;
        }

        // 检查进程是否在游戏目录下
        if let Some(exe) = process.exe() {
            let exe_path_lower = exe.to_string_lossy().to_lowercase();
            if exe_path_lower.starts_with(&dir_lower) {
                processes.push((pid, proc_name.clone()));
                continue;
            }
        }

        // 检查进程名是否匹配原始exe名（有些游戏会改名）
        if proc_name == exe_lower || (!exe_base.is_empty() && proc_name.contains(&exe_base)) {
            // 避免重复添加
            if !processes.iter().any(|(p, _)| *p == pid) {
                processes.push((pid, proc_name.clone()));
            }
        }
    }

    processes
}

/// 动态检查游戏是否还在运行（基于 sysinfo，替代原 wmic/tasklist 子进程枚举）
/// 检测逻辑与原 check_game_running_dynamic 等价：
/// 1. 初始进程（可能是启动器）
/// 2. 之前探测到的游戏进程（pid/名称）
/// 3. 游戏安装目录下的进程
/// 4. 进程名匹配原始 exe 名
/// 5. 进程名包含游戏名提示 / 目录名
fn game_process_is_running(
    system: &System,
    dir_path: &str,
    exe_name: &str,
    initial_pid: u32,
    known_pids: &HashSet<u32>,
    known_names: &HashSet<String>,
) -> bool {
    let dir_lower = dir_path.to_lowercase();
    let exe_lower = exe_name.to_lowercase();
    let game_name_hint = exe_lower
        .replace("launcher.exe", "")
        .replace(".exe", "")
        .trim()
        .to_string();
    let dir_name = std::path::Path::new(dir_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    for process in system.processes().values() {
        let pid = process.pid().as_u32();

        // 跳过当前程序自身
        if pid == std::process::id() {
            continue;
        }
        let proc_name = process.name().to_string_lossy().to_lowercase();

        // 1. 初始进程（可能是启动器）
        if pid == initial_pid {
            return true;
        }

        // 2. 之前探测到的游戏进程
        if known_pids.contains(&pid) || known_names.contains(&proc_name) {
            return true;
        }

        // 3. 游戏安装目录下的进程
        if let Some(exe) = process.exe() {
            if exe.to_string_lossy().to_lowercase().starts_with(&dir_lower) {
                return true;
            }
        }

        // 4. 进程名匹配原始 exe 名
        if proc_name == exe_lower {
            return true;
        }

        // 5. 进程名包含游戏名提示
        if !game_name_hint.is_empty() && proc_name.contains(&game_name_hint) {
            return true;
        }

        // 6. 进程名包含游戏目录名
        if !dir_name.is_empty() && proc_name.contains(&dir_name) {
            return true;
        }
    }

    false
}

/// 检查特定PID的进程是否还在运行（基于 sysinfo）
fn is_process_running(pid: u32) -> bool {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    system.process(Pid::from_u32(pid)).is_some()
}

/// 关闭游戏进程
/// 关闭指定PID的进程及其所有子进程
#[tauri::command]
pub async fn close_game_process(pid: u32) -> Result<(), String> {
    use std::process::Command;
    
    // 首先尝试使用 /T 参数关闭进程树（包括子进程）
    #[cfg(target_os = "windows")]
    let _output = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
    };
    
    #[cfg(not(target_os = "windows"))]
    let _output = Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/T", "/F"])
        .output();
    
    // 即使 taskkill 返回错误，也可能部分成功，所以不直接返回错误
    // 而是继续检查进程是否还在运行
    
    // 等待一小段时间让进程结束
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // 检查进程是否还在运行
    if is_process_running(pid) {
        // 如果还在运行，尝试再次强制关闭
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            let _ = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .creation_flags(CREATE_NO_WINDOW)
                .output();
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            let _ = Command::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/F"])
                .output();
        }
        
        // 再次等待
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        // 如果仍然运行，返回错误
        if is_process_running(pid) {
            return Err("无法关闭游戏进程，请手动关闭".to_string());
        }
    }
    
    Ok(())
}

/// 检查游戏进程是否还在运行
/// 返回 (是否运行中, 当前会话已玩分钟数)
#[tauri::command]
pub async fn check_game_process_status(
    app: AppHandle,
    game_id: String,
    start_time_secs: u64,
) -> Result<(bool, u64), String> {
    // 获取游戏数据
    let game = game_data_service::get_game(app, game_id).await?
        .ok_or("游戏不存在")?;
    
    // 获取exe文件名
    let exe_path = std::path::Path::new(&game.exe_path);
    let exe_name = exe_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 检查进程是否还在运行
    let is_running = is_game_process_running_simple(&exe_name);
    
    // 计算当前会话已玩时长（分钟）
    let elapsed_secs = start_time_secs; // 前端传过来的已运行秒数
    let minutes = elapsed_secs / 60;
    
    Ok((is_running, minutes))
}

/// 简化版检查游戏进程是否还在运行（用于前端轮询，基于 sysinfo）
fn is_game_process_running_simple(exe_name: &str) -> bool {
    if exe_name.is_empty() {
        return false;
    }

    let exe_lower = exe_name.to_lowercase();
    let exe_base = exe_lower.replace(".exe", "");

    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);

    for process in system.processes().values() {
        let process_name = process.name().to_string_lossy().to_lowercase();
        let process_base = process_name.replace(".exe", "");

        // 检查进程名是否匹配（支持部分匹配）
        if process_name == exe_lower
            || process_name.starts_with(&exe_base)
            || exe_lower.starts_with(&process_base)
        {
            return true;
        }
    }

    false
}

/// 更新游戏数据（用于编辑功能）
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn update_game_data(
    app: AppHandle,
    game_id: String,
    game_name: Option<String>,
    chinese_name: Option<String>,
    install_path: Option<String>,
    exe_path: Option<String>,
    save_path: Option<String>,
    cover_path: Option<String>,
    steam_game_id: Option<String>,
) -> Result<GameData, String> {
    // 获取现有游戏数据
    let mut game = game_data_service::get_game(app.clone(), game_id.clone()).await?
        .ok_or("游戏不存在")?;
    
    // 更新字段
    if let Some(name) = game_name {
        game.game_name = name;
    }
    if let Some(name) = chinese_name {
        game.chinese_name = name;
    }
    if let Some(path) = install_path {
        game.install_path = path;
    }
    if let Some(path) = exe_path {
        game.exe_path = path;
    }
    if let Some(path) = save_path {
        game.save_path = Some(path);
    }
    if let Some(path) = cover_path {
        game.cover_path = Some(path);
    }
    if let Some(id) = steam_game_id {
        game.steam_game_id = Some(id);
    }
    
    // 保存更新
    game_data_service::upsert_game(app, game.clone()).await?;
    
    Ok(game)
}

/// 切换游戏收藏状态
#[tauri::command]
pub async fn toggle_game_favorite(
    app: AppHandle,
    game_id: String,
) -> Result<GameData, String> {
    game_data_service::toggle_game_favorite(app, game_id).await
}

/// 下载完成后收尾处理
/// 扫描下载目录，自动定位游戏主程序并标记为已安装
#[tauri::command]
pub async fn finalize_game_download(
    app: AppHandle,
    game_id: String,
) -> Result<GameData, String> {
    game_data_service::finalize_download(app, game_id).await
}

/// 删除指定游戏的下载日志缓存目录
/// 路径: %APPDATA%/SteamToolPlus/log/{game_id}/
/// 当游戏目录被删除或重置时调用，清理对应的下载进度缓存文件
#[tauri::command]
pub fn delete_game_log_cache(game_id: String) -> Result<(), String> {
    use std::fs;
    let log_dir = crate::utils::config_path_utils::get_appdata_dir()?
        .join("log")
        .join(&game_id);
    if log_dir.exists() {
        fs::remove_dir_all(&log_dir)
            .map_err(|e| format!("删除游戏日志缓存失败 ({}): {}", log_dir.display(), e))?;
    }
    Ok(())
}
