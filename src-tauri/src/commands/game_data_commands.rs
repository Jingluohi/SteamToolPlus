// 游戏数据管理命令
// 处理游戏数据的增删改查

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
/// 卸载游戏时使用，删除游戏安装目录但保留存档
#[tauri::command]
pub async fn delete_game_directory(
    path: String,
    save_path: Option<String>,
) -> Result<(), String> {
    use std::fs;

    let path_obj = std::path::Path::new(&path);

    // 安全检查：确保路径存在且是目录
    if !path_obj.exists() {
        return Err("游戏目录不存在".to_string());
    }

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
    
    // 使用 explorer.exe 打开目录
    Command::new("explorer.exe")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("打开游戏目录失败: {}", e))?;
    
    Ok(())
}

/// 检查游戏是否存在
#[tauri::command]
pub async fn check_game_exists(app: AppHandle, game_id: String) -> Result<bool, String> {
    game_data_service::game_exists(app, game_id).await
}

/// 导入自定义游戏
#[tauri::command]
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
    let app_handle = app.clone();
    let game_id_clone = game_id.clone();

    thread::spawn(move || {
        // 等待初始进程结束（很多游戏启动器会立即退出）
        let _ = child.wait();

        // 游戏启动时间
        let game_start_time = Instant::now();

        // 动态获取进程快照 - 多次尝试捕获游戏进程（处理启动器延迟启动的情况）
        let mut game_processes = Vec::new();
        let max_attempts = 10; // 最多尝试10次
        let attempt_interval = Duration::from_secs(2); // 每2秒尝试一次

        for attempt in 1..=max_attempts {
            thread::sleep(attempt_interval);

            // 获取当前进程快照
            let current_processes = get_game_processes_snapshot(&install_path, &exe_name, initial_pid);

            if !current_processes.is_empty() {
                game_processes = current_processes;
                break;
            }

            // 如果还没找到进程且还没达到最大尝试次数，继续等待
            if attempt < max_attempts {
                continue;
            }
        }

        // 持续监控，直到确认游戏完全关闭
        let mut last_seen = Instant::now();
        let max_wait = Duration::from_secs(15); // 最多等待15秒确认进程完全关闭
        let check_interval = Duration::from_secs(3); // 每3秒检查一次

        loop {
            // 检查游戏是否还在运行（使用动态检测，不依赖初始快照）
            let is_running = check_game_running_dynamic(&install_path, &exe_name, initial_pid, &game_processes);

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
            // 使用 tokio runtime 执行异步操作
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
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
/// 返回 (PID, 进程名) 的列表
fn get_game_processes_snapshot(dir_path: &str, exe_name: &str, initial_pid: u32) -> Vec<(u32, String)> {
    use std::process::Command;
    
    let mut processes = Vec::new();
    let dir_lower = dir_path.to_lowercase();
    let exe_lower = exe_name.to_lowercase();
    
    // 使用 wmic 获取所有进程的详细信息
    let output = Command::new("wmic")
        .args(&["process", "get", "ProcessId,ExecutablePath,Name", "/format:csv"])
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            for line in stdout.lines() {
                // CSV格式: Node,ExecutablePath,Name,ProcessId
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 4 {
                    let exe_path = parts[1].trim().trim_matches('"');
                    let proc_name = parts[2].trim().trim_matches('"').to_lowercase();
                    let pid_str = parts[3].trim().trim_matches('"');
                    
                    if let Ok(pid_num) = pid_str.parse::<u32>() {
                        // 检查是否是我们启动的初始进程
                        if pid_num == initial_pid {
                            processes.push((pid_num, proc_name.clone()));
                            continue;
                        }
                        
                        // 检查进程是否在游戏目录下
                        if !exe_path.is_empty() {
                            let exe_path_lower = exe_path.to_lowercase();
                            if exe_path_lower.starts_with(&dir_lower) {
                                processes.push((pid_num, proc_name.clone()));
                                continue;
                            }
                        }
                        
                        // 检查进程名是否匹配原始exe名（有些游戏会改名）
                        if proc_name == exe_lower || 
                           proc_name.contains(&exe_lower.replace(".exe", "")) {
                            // 避免重复添加
                            if !processes.iter().any(|(p, _)| *p == pid_num) {
                                processes.push((pid_num, proc_name.clone()));
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[SteamTool] 获取进程列表失败: {}", e);
        }
    }
    
    processes
}

/// 检查游戏是否还在运行
/// 模仿 Steam 的检测逻辑：
/// 1. 检查原始进程是否还在
/// 2. 检查游戏目录下是否有相关进程
/// 3. 检查已知的游戏进程是否还在
fn check_game_running(
    dir_path: &str, 
    exe_name: &str, 
    initial_pid: u32,
    known_processes: &[(u32, String)]
) -> bool {
    use std::process::Command;
    
    let dir_lower = dir_path.to_lowercase();
    let exe_lower = exe_name.to_lowercase();
    
    // 使用 tasklist 获取所有进程（比 wmic 更快）
    let output = Command::new("tasklist")
        .args(&["/FO", "CSV", "/NH"])
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let found_any = false;
            
            for line in stdout.lines() {
                // CSV格式: "进程名","PID","会话名","会话#","内存使用"
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let proc_name = parts[0].trim().trim_matches('"').to_lowercase();
                    let pid_str = parts[1].trim().trim_matches('"');
                    
                    if let Ok(pid_num) = pid_str.parse::<u32>() {
                        // 1. 检查是否是初始进程
                        if pid_num == initial_pid {
                            return true;
                        }
                        
                        // 2. 检查是否在已知的游戏进程列表中
                        if known_processes.iter().any(|(p, n)| *p == pid_num || n == &proc_name) {
                            return true;
                        }
                        
                        // 3. 检查进程名是否匹配原始exe名
                        if proc_name == exe_lower {
                            return true;
                        }
                        
                        // 4. 检查进程名是否包含游戏名称（如 "The Last of Us"）
                        let game_name_hint = exe_lower.replace("launcher.exe", "")
                            .replace(".exe", "")
                            .trim()
                            .to_string();
                        if !game_name_hint.is_empty() && proc_name.contains(&game_name_hint) {
                            return true;
                        }
                    }
                }
            }
            
            // 如果 tasklist 没有找到，尝试使用 wmic 检查游戏目录下的进程
            if !found_any {
                return check_processes_in_directory_wmic(&dir_lower);
            }
            
            false
        }
        Err(_) => {
            // tasklist 失败，回退到 wmic
            check_processes_in_directory_wmic(&dir_lower)
        }
    }
}

/// 使用 wmic 检查游戏目录下的进程
fn check_processes_in_directory_wmic(dir_lower: &str) -> bool {
    use std::process::Command;

    let output = Command::new("wmic")
        .args(&["process", "get", "ExecutablePath", "/format:csv"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let exe_path = parts[1].trim().trim_matches('"');
                    if !exe_path.is_empty() {
                        let exe_lower = exe_path.to_lowercase();
                        if exe_lower.starts_with(dir_lower) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        Err(_) => false
    }
}

/// 动态检查游戏是否还在运行
/// 与 check_game_running 不同，这个函数会实时扫描游戏目录下的所有进程
/// 不依赖初始进程快照，适用于启动器延迟启动游戏的情况
fn check_game_running_dynamic(
    dir_path: &str,
    exe_name: &str,
    initial_pid: u32,
    known_processes: &[(u32, String)]
) -> bool {
    use std::process::Command;

    let dir_lower = dir_path.to_lowercase();
    let exe_lower = exe_name.to_lowercase();

    // 首先检查初始进程是否还在运行
    if is_process_running(initial_pid) {
        return true;
    }

    // 使用 wmic 获取所有进程的详细信息（包括可执行路径）
    let output = Command::new("wmic")
        .args(&["process", "get", "ProcessId,ExecutablePath,Name", "/format:csv"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines() {
                // CSV格式: Node,ExecutablePath,Name,ProcessId
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 4 {
                    let exe_path = parts[1].trim().trim_matches('"');
                    let proc_name = parts[2].trim().trim_matches('"').to_lowercase();
                    let pid_str = parts[3].trim().trim_matches('"');

                    if let Ok(pid_num) = pid_str.parse::<u32>() {
                        // 跳过系统进程和当前检查进程
                        if pid_num == std::process::id() {
                            continue;
                        }

                        // 1. 检查是否在已知的游戏进程列表中
                        if known_processes.iter().any(|(p, n)| *p == pid_num || n == &proc_name) {
                            return true;
                        }

                        // 2. 检查进程是否在游戏目录下（这是最关键的检测）
                        if !exe_path.is_empty() {
                            let exe_path_lower = exe_path.to_lowercase();
                            if exe_path_lower.starts_with(&dir_lower) {
                                return true;
                            }
                        }

                        // 3. 检查进程名是否匹配原始exe名
                        if proc_name == exe_lower {
                            return true;
                        }

                        // 4. 检查进程名是否包含游戏名称（如 "The Last of Us"）
                        let game_name_hint = exe_lower.replace("launcher.exe", "")
                            .replace(".exe", "")
                            .trim()
                            .to_string();
                        if !game_name_hint.is_empty() && proc_name.contains(&game_name_hint) {
                            return true;
                        }

                        // 5. 检查进程名是否包含游戏目录名（有些游戏exe名和目录名相关）
                        let dir_name = std::path::Path::new(dir_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        if !dir_name.is_empty() && proc_name.contains(&dir_name) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        Err(_) => {
            // wmic 失败，回退到 tasklist
            check_game_running(dir_path, exe_name, initial_pid, known_processes)
        }
    }
}

/// 检查特定PID的进程是否还在运行
fn is_process_running(pid: u32) -> bool {
    use std::process::Command;
    
    let output = Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"])
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // 如果输出包含PID，说明进程还在运行
            stdout.contains(&pid.to_string())
        }
        Err(_) => false
    }
}

/// 关闭游戏进程
/// 关闭指定PID的进程及其所有子进程
#[tauri::command]
pub async fn close_game_process(pid: u32) -> Result<(), String> {
    use std::process::Command;
    
    // 首先尝试使用 /T 参数关闭进程树（包括子进程）
    let _output = Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/T", "/F"])
        .output()
        .map_err(|e| format!("执行关闭命令失败: {}", e))?;
    
    // 即使 taskkill 返回错误，也可能部分成功，所以不直接返回错误
    // 而是继续检查进程是否还在运行
    
    // 等待一小段时间让进程结束
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // 检查进程是否还在运行
    if is_process_running(pid) {
        // 如果还在运行，尝试再次强制关闭
        let _ = Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .output();
        
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

/// 简化版检查游戏进程是否还在运行（用于前端轮询）
fn is_game_process_running_simple(exe_name: &str) -> bool {
    use std::process::Command;
    
    if exe_name.is_empty() {
        return false;
    }
    
    // 使用 tasklist 查找进程
    let output = Command::new("tasklist")
        .args(&["/FO", "CSV", "/NH"])
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                // CSV格式: "进程名","PID","会话名","会话#","内存使用"
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let process_name = parts[0].trim().trim_matches('"').to_lowercase();
                    // 检查进程名是否匹配（支持部分匹配）
                    if process_name == exe_name || 
                       process_name.starts_with(&exe_name.replace(".exe", "")) ||
                       exe_name.starts_with(&process_name.replace(".exe", "")) {
                        return true;
                    }
                }
            }
            false
        }
        Err(_) => false
    }
}

/// 更新游戏数据（用于编辑功能）
#[tauri::command]
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
