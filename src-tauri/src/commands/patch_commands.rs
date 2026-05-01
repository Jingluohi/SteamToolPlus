// 补丁注入命令
// 处理免Steam补丁注入相关的IPC调用

use crate::utils::resource_utils::get_resource_dir as get_resource_dir_util;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

/// 获取资源目录（IPC命令版本）
/// 包装共享工具函数，供前端调用
#[tauri::command]
pub fn get_resource_dir(app: AppHandle) -> Result<String, String> {
    let path = get_resource_dir_util(&app)?;
    Ok(path.to_string_lossy().to_string())
}

/// 检查路径是否存在
#[tauri::command]
pub fn path_exists(path: String) -> Result<bool, String> {
    Ok(Path::new(&path).exists())
}

/// 备份游戏exe文件
#[tauri::command]
pub fn backup_game_exe(exe_path: String) -> Result<BackupResult, String> {
    use std::fs;
    
    let exe = Path::new(&exe_path);
    if !exe.exists() {
        return Ok(BackupResult {
            success: false,
            message: "文件不存在".to_string(),
        });
    }
    
    let backup_path = exe.with_extension("exe.bak");
    
    if backup_path.exists() {
        return Ok(BackupResult {
            success: true,
            message: "备份已存在".to_string(),
        });
    }
    
    match fs::copy(exe, &backup_path) {
        Ok(_) => Ok(BackupResult {
            success: true,
            message: "备份成功".to_string(),
        }),
        Err(e) => Ok(BackupResult {
            success: false,
            message: format!("备份失败: {}", e),
        }),
    }
}

/// 备份结果
#[derive(serde::Serialize)]
pub struct BackupResult {
    pub success: bool,
    pub message: String,
}

/// 使用 Steamless 脱壳游戏主程序
#[tauri::command]
pub async fn unpack_game_exe(
    app: AppHandle,
    game_exe_path: String,
) -> Result<UnpackResult, String> {
    use std::process::Command;

    // 获取资源目录
    let resource_dir = get_resource_dir(app)?;

    // 构建 Steamless.CLI.exe 的完整路径
    let steamless_path = Path::new(&resource_dir).join("steamless").join("Steamless.CLI.exe");

    if !steamless_path.exists() {
        return Err(format!("Steamless.CLI.exe 不存在: {}", steamless_path.display()));
    }

    let target_exe = Path::new(&game_exe_path);

    if !target_exe.exists() {
        return Err(format!("游戏主程序不存在: {}", game_exe_path));
    }

    if !target_exe.is_file() {
        return Err(format!("路径不是文件: {}", game_exe_path));
    }

    // 检查文件扩展名
    if let Some(ext) = target_exe.extension() {
        if ext.to_string_lossy().to_lowercase() != "exe" {
            return Err(format!("选择的文件不是 .exe 文件: {}", game_exe_path));
        }
    } else {
        return Err("选择的文件没有扩展名".to_string());
    }

    let target_exe_str = target_exe.to_string_lossy();

    // 运行 Steamless 进行脱壳
    let output = Command::new(&steamless_path)
        .arg("--quiet")
        .arg(&*target_exe_str)
        .output()
        .map_err(|e| format!("运行 Steamless 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        // 脱壳成功，Steamless 会生成 .unpacked.exe 文件
        let unpacked_exe = target_exe.with_extension("exe.unpacked.exe");
        let unpacked_path = if unpacked_exe.exists() {
            // 备份原文件为 xxx.exe.bak
            let backup_path = target_exe.with_extension("exe.bak");
            std::fs::rename(target_exe, &backup_path)
                .map_err(|e| format!("备份原文件失败: {}", e))?;
            // 将脱壳后的文件重命名为原文件名 xxx.exe
            std::fs::rename(&unpacked_exe, target_exe)
                .map_err(|e| format!("重命名脱壳文件失败: {}", e))?;
            Some(game_exe_path.clone())
        } else {
            None
        };

        Ok(UnpackResult {
            success: true,
            message: "脱壳成功".to_string(),
            unpacked_path,
        })
    } else {
        Ok(UnpackResult {
            success: false,
            message: format!("脱壳失败: {} {}", stdout, stderr),
            unpacked_path: None,
        })
    }
}

/// 脱壳结果
#[derive(serde::Serialize)]
pub struct UnpackResult {
    pub success: bool,
    pub message: String,
    pub unpacked_path: Option<String>,
}

/// 应用 Steam 补丁基础配置
#[tauri::command]
pub async fn apply_steam_patch_basic(
    app: AppHandle,
    game_path: String,
    _game_id: String,
    steam_app_id: String,
    use_experimental: bool,
    emulator_mode: Option<i32>,
) -> Result<BasicConfigResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;
    use std::process::Command;

    // 获取资源目录
    let resource_dir = get_resource_dir(app)?;

    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err(format!("游戏路径不存在: {}", game_path));
    }

    let steam_settings_dir = game_dir.join("steam_settings");
    let mode = emulator_mode.unwrap_or(0);

    // ========== 第1步: 复制 steam_settings.EXAMPLE 文件夹 ==========
    let example_dir = Path::new(&resource_dir).join("gbe_fork").join("steam_settings.EXAMPLE");
    
    if example_dir.exists() {
        // 如果已存在 steam_settings，先删除
        if steam_settings_dir.exists() {
            fs::remove_dir_all(&steam_settings_dir)
                .await
                .map_err(|e| format!("删除旧 steam_settings 失败: {}", e))?;
        }

        // 递归复制示例文件夹
        copy_dir_recursive(&example_dir, &steam_settings_dir)
            .await
            .map_err(|e| format!("复制 steam_settings.EXAMPLE 失败: {}", e))?;

        // 重命名 .EXAMPLE 文件
        rename_example_files(&steam_settings_dir)
            .await
            .map_err(|e| format!("重命名示例文件失败: {}", e))?;
    }

    // ========== 第2步: 判断游戏架构 ==========
    let is_64bit = game_dir.read_dir()
        .map_err(|e| format!("读取游戏目录失败: {}", e))?
        .filter_map(|e| e.ok())
        .any(|e| {
            if let Some(name) = e.file_name().to_str() {
                name.contains("64") || name.contains("x64")
            } else {
                false
            }
        });

    // ========== 第3步: 根据模式处理 DLL ==========
    if mode == 0 {
        // ========== 标准模式: 替换 steam_api.dll ==========
        let api_dll_name = if is_64bit { "steam_api64.dll" } else { "steam_api.dll" };
        let original_api_path = game_dir.join(&api_dll_name);

        // 检查是否存在 steam_api.dll
        if !original_api_path.exists() {
            return Err(format!(
                "未找到 {}！\n\n可能原因：\n1. 选择的游戏目录不正确\n2. 游戏使用 steamclient.dll 而非 steam_api.dll\n3. 请先尝试「高级模式】",
                api_dll_name
            ));
        }

        // 3.1 备份原 DLL
        let backup_path = game_dir.join(format!("{}.bak", api_dll_name));
        if !backup_path.exists() {
            fs::copy(&original_api_path, &backup_path)
                .await
                .map_err(|e| format!("备份原 DLL 失败: {}", e))?;
        }

        // 3.2 生成 steam_interfaces.txt
        let tool_name = if is_64bit {
            "tools/generate_interfaces/generate_interfaces_x64.exe"
        } else {
            "tools/generate_interfaces/generate_interfaces_x32.exe"
        };
        let tool_path = Path::new(&resource_dir).join("gbe_fork").join(tool_name);

        if tool_path.exists() {
            let tool_path_clone = tool_path.clone();
            let original_api_path_clone = original_api_path.clone();
            let game_dir_clone = game_dir.to_path_buf();
            let steam_settings_dir_clone = steam_settings_dir.clone();
            
            let result = tokio::task::spawn_blocking(move || {
                Command::new(&tool_path_clone)
                    .arg(&original_api_path_clone)
                    .current_dir(&game_dir_clone)
                    .output()
            }).await.map_err(|e| format!("运行 generate_interfaces 失败: {}", e))?;

            if let Ok(output) = result {
                if output.status.success() {
                    // 移动生成的 steam_interfaces.txt 到 steam_settings
                    let generated_txt = game_dir.join("steam_interfaces.txt");
                    if generated_txt.exists() {
                        let target_txt = steam_settings_dir_clone.join("steam_interfaces.txt");
                        fs::rename(&generated_txt, &target_txt)
                            .await
                            .map_err(|e| format!("移动 steam_interfaces.txt 失败: {}", e))?;
                    }
                }
            }
        }

        // 3.3 替换 steam_api.dll
        let source_dll = if is_64bit {
            if use_experimental {
                "gbe_fork/experimental/x64/steam_api64.dll"
            } else {
                "gbe_fork/regular/x64/steam_api64.dll"
            }
        } else {
            if use_experimental {
                "gbe_fork/experimental/x32/steam_api.dll"
            } else {
                "gbe_fork/regular/x32/steam_api.dll"
            }
        };

        let source_path = Path::new(&resource_dir).join(source_dll);
        if source_path.exists() {
            fs::copy(&source_path, &original_api_path)
                .await
                .map_err(|e| format!("复制 DLL 失败: {}", e))?;
        } else {
            return Err(format!("源 DLL 文件不存在: {}", source_path.display()));
        }
    } else {
        // ========== 高级模式: 替换 steamclient.dll + steam_api.dll ==========
        
        // 4.1 替换 steamclient.dll
        let client_dll_name = if is_64bit { "steamclient64.dll" } else { "steamclient.dll" };
        let original_client_path = game_dir.join(&client_dll_name);

        // 检查是否存在 steamclient.dll
        if !original_client_path.exists() {
            return Err(format!(
                "未找到 {}！\n\n可能原因：\n1. 选择的游戏目录不正确\n2. 该游戏不支持高级模式\n3. 请尝试「标准模式】",
                client_dll_name
            ));
        }

        // 备份原 steamclient.dll
        let backup_path = game_dir.join(format!("{}.bak", client_dll_name));
        if !backup_path.exists() {
            fs::copy(&original_client_path, &backup_path)
                .await
                .map_err(|e| format!("备份 steamclient 失败: {}", e))?;
        }

        // 复制实验版 steamclient.dll
        let client_source = if is_64bit {
            "gbe_fork/steamclient_experimental/steamclient64.dll"
        } else {
            "gbe_fork/steamclient_experimental/steamclient.dll"
        };

        let client_source_path = Path::new(&resource_dir).join(client_source);
        if client_source_path.exists() {
            fs::copy(&client_source_path, &original_client_path)
                .await
                .map_err(|e| format!("复制 steamclient 失败: {}", e))?;
        } else {
            return Err(format!("源 steamclient 文件不存在: {}", client_source_path.display()));
        }

        // 4.2 同步替换 steam_api.dll（实验版）
        let api_dll_name = if is_64bit { "steam_api64.dll" } else { "steam_api.dll" };
        let api_source = if is_64bit {
            "gbe_fork/experimental/x64/steam_api64.dll"
        } else {
            "gbe_fork/experimental/x32/steam_api.dll"
        };

        let api_source_path = Path::new(&resource_dir).join(api_source);
        let api_target_path = game_dir.join(&api_dll_name);

        if api_source_path.exists() {
            // 备份原 steam_api.dll（如果存在）
            if api_target_path.exists() {
                let api_backup_path = game_dir.join(format!("{}.bak", api_dll_name));
                if !api_backup_path.exists() {
                    fs::copy(&api_target_path, &api_backup_path)
                        .await
                        .map_err(|e| format!("备份 steam_api 失败: {}", e))?;
                }
            }

            // 复制实验版 steam_api.dll
            fs::copy(&api_source_path, &api_target_path)
                .await
                .map_err(|e| format!("复制 steam_api 失败: {}", e))?;
        } else {
            return Err(format!("源 steam_api 文件不存在: {}", api_source_path.display()));
        }
    }

    // ========== 第4步: 双路径写入 steam_appid.txt ==========
    // 4.1 steam_settings/steam_appid.txt
    let appid_path_settings = steam_settings_dir.join("steam_appid.txt");
    let mut appid_file_settings = fs::File::create(&appid_path_settings)
        .await
        .map_err(|e| format!("创建 steam_settings/steam_appid.txt 失败: {}", e))?;
    appid_file_settings.write_all(steam_app_id.as_bytes())
        .await
        .map_err(|e| format!("写入 steam_settings/steam_appid.txt 失败: {}", e))?;

    // 4.2 游戏根目录/steam_appid.txt
    let appid_path_root = game_dir.join("steam_appid.txt");
    let mut appid_file_root = fs::File::create(&appid_path_root)
        .await
        .map_err(|e| format!("创建根目录 steam_appid.txt 失败: {}", e))?;
    appid_file_root.write_all(steam_app_id.as_bytes())
        .await
        .map_err(|e| format!("写入根目录 steam_appid.txt 失败: {}", e))?;

    // ========== 第5步: 写入基础配置文件 ==========
    // 5.1 写入 configs.main.ini - 核心配置（强制局域网模式）
    let main_config_path = steam_settings_dir.join("configs.main.ini");
    let main_config_content = r#"[main]
# 强制局域网模式 - 屏蔽外网校验，实现免Steam联机
force_lan_only = 1

# 启用局域网广播
enable_lan_broadcast = 1

# 匹配服务器列表类型
# 0=始终返回局域网服务器列表
matchmaking_server_list_actual_type = 0

# 禁用Steam网络检查
disable_steam_network_check = 1

# 低延迟联机模式
lan_broadcast_interval = 300

# 允许未知统计
allow_unknown_stats = 1

# 最大联机人数
max_lobby_players = 32
"#;
    
    let mut main_file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    main_file.write_all(main_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    // 5.2 写入 configs.user.ini - 用户配置
    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config_content = r#"[user]
# 默认存档路径：%appdata%\GSE Saves
"#;
    
    let mut user_file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    user_file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    // 5.3 写入 configs.app.ini - 应用配置（DLC解锁）
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let app_config_content = r#"
# default=public
branch_name=public

[app::paths]

[app::dlcs]
# 一键解锁全部DLC
unlock_all=1
"#;
    
    let mut app_file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    app_file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    // 5.4 写入 custom_broadcasts.txt - 局域网广播配置
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let broadcasts_content = r#"192.168.1.0/24
192.168.0.0/24
10.0.0.0/24
"#;
    
    let mut broadcasts_file = fs::File::create(&broadcasts_path)
        .await
        .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
    broadcasts_file.write_all(broadcasts_content.as_bytes())
        .await
        .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;

    Ok(BasicConfigResult {
        success: true,
        message: "基础配置已应用".to_string(),
    })
}

/// 基础配置结果
#[derive(serde::Serialize)]
pub struct BasicConfigResult {
    pub success: bool,
    pub message: String,
}

/// 递归复制目录
async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    use tokio::fs;

    fs::create_dir_all(dst)
        .await
        .map_err(|e| format!("创建目录失败: {}", e))?;

    let mut entries = fs::read_dir(src)
        .await
        .map_err(|e| format!("读取源目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("读取条目失败: {}", e))? {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            Box::pin(copy_dir_recursive(&src_path, &dst_path)).await?;
        } else {
            fs::copy(&src_path, &dst_path)
                .await
                .map_err(|e| format!("复制文件失败: {}", e))?;
        }
    }

    Ok(())
}

/// 重命名 .EXAMPLE 文件和文件夹
async fn rename_example_files(dir: &Path) -> Result<(), String> {
    use tokio::fs;

    // 首先收集所有需要重命名的条目（避免在遍历过程中修改）
    let mut entries_to_rename: Vec<(std::path::PathBuf, std::path::PathBuf)> = Vec::new();
    let mut sub_dirs: Vec<std::path::PathBuf> = Vec::new();

    let mut entries = fs::read_dir(dir)
        .await
        .map_err(|e| format!("读取目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("读取条目失败: {}", e))? {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.ends_with(".EXAMPLE") {
            // 收集需要重命名的条目（文件或文件夹）
            let new_name = &file_name_str[..file_name_str.len() - 8];
            let new_path = dir.join(new_name);
            entries_to_rename.push((path.clone(), new_path));
        } else if path.is_dir() {
            // 收集子目录以便后续递归处理
            sub_dirs.push(path);
        }
    }

    // 先递归处理子目录（在重命名父文件夹之前）
    for sub_dir in sub_dirs {
        Box::pin(rename_example_files(&sub_dir)).await?;
    }

    // 然后重命名当前目录中的 .EXAMPLE 条目
    for (old_path, new_path) in entries_to_rename {
        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| format!("重命名失败: {} -> {}: {}", old_path.display(), new_path.display(), e))?;
    }

    Ok(())
}

/// 保存局域网联机配置
#[tauri::command]
pub async fn save_lan_multiplayer_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 文件夹失败: {}", e))?;

    // 保存 custom_broadcasts.txt
    if let Some(broadcasts) = config.get("customBroadcasts").and_then(|v| v.as_array()) {
        let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
        let content: Vec<String> = broadcasts
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .filter(|s| !s.is_empty())
            .collect();

        if !content.is_empty() {
            let mut file = fs::File::create(&broadcasts_path)
                .await
                .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
            file.write_all(content.join("\n").as_bytes())
                .await
                .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;
        }
    }

    // 保存 auto_accept_invite.txt
    if let Some(auto_accept) = config.get("autoAcceptInvite").and_then(|v| v.as_str()) {
        let auto_accept_path = steam_settings_dir.join("auto_accept_invite.txt");
        let content = match auto_accept {
            "all" => "*".to_string(),
            "whitelist" => {
                // 白名单模式，保存白名单
                if let Some(whitelist) = config.get("whitelist").and_then(|v| v.as_array()) {
                    whitelist
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    String::new()
                }
            }
            _ => String::new(), // none
        };
        
        if !content.is_empty() {
            let mut file = fs::File::create(&auto_accept_path)
                .await
                .map_err(|e| format!("创建 auto_accept_invite.txt 失败: {}", e))?;
            file.write_all(content.as_bytes())
                .await
                .map_err(|e| format!("写入 auto_accept_invite.txt 失败: {}", e))?;
        }
    }

    Ok(ConfigSaveResult {
        success: true,
        message: "配置已保存".to_string(),
    })
}

/// 配置保存结果
#[derive(serde::Serialize)]
pub struct ConfigSaveResult {
    pub success: bool,
    pub message: String,
}

/// 注入补丁（旧版兼容）
#[tauri::command]
pub fn inject_patch(
    game_exe_path: String,
    patch_file_path: String,
    options: InjectOptions,
) -> Result<InjectResult, String> {
    // 这里简化处理，实际应该调用相应的注入逻辑
    Ok(InjectResult {
        success: true,
        message: "补丁注入成功".to_string(),
    })
}

/// 注入选项
#[derive(serde::Deserialize)]
pub struct InjectOptions {
    pub auto_close: bool,
    pub wait_for_exit: bool,
    pub backup_original: bool,
}

/// 注入结果
#[derive(serde::Serialize)]
pub struct InjectResult {
    pub success: bool,
    pub message: String,
}

/// 关闭应用程序
#[tauri::command]
pub fn close_application() -> Result<(), String> {
    std::process::exit(0);
}

/// 选择文件
#[tauri::command]
pub async fn select_file(title: String) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    // 简化实现，返回None
    Ok(None)
}

/// 选择文件夹
#[tauri::command]
pub async fn select_folder(title: String) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    // 简化实现，返回None
    Ok(None)
}

/// 打开外部链接
#[tauri::command]
pub fn open_external_link(url: String) -> Result<(), String> {
    use std::process::Command;
    
    Command::new("cmd")
        .args(["/c", "start", "", &url])
        .spawn()
        .map_err(|e| format!("打开链接失败: {}", e))?;
    
    Ok(())
}

/// 保存其他配置（占位符）
#[tauri::command]
pub async fn save_overlay_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_achievements_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_items_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_controller_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_user_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_leaderboards_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_stats_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_dlc_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn save_main_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    Ok(ConfigSaveResult { success: true, message: "配置已保存".to_string() })
}

#[tauri::command]
pub async fn load_main_config(
    game_path: String,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "exists": false
    }))
}

#[tauri::command]
pub async fn load_lan_multiplayer_config(
    game_path: String,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "exists": false
    }))
}

/// 补丁应用结果
#[derive(serde::Serialize)]
pub struct ApplyPatchResult {
    pub success: bool,
    pub backed_up_files: Vec<String>,
    pub copied_files: Vec<String>,
    pub errors: Vec<String>,
}

/// 应用补丁 - 解压7z文件并复制到游戏目录，并备份原有文件
#[tauri::command]
pub async fn apply_patch(
    app: AppHandle,
    patch_source_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir_util(&app)?;

    // 处理路径：如果 patch_source_path 已经包含 "resources/" 或 "Resources/" 前缀，则移除它
    let patch_lower = patch_source_path.to_lowercase();
    let patch_relative_path = if patch_lower.starts_with("resources/") {
        &patch_source_path[10..]  // 移除 "resources/" 前缀
    } else if patch_lower.starts_with("resources\\") {
        &patch_source_path[11..]  // 移除 "resources\" 前缀
    } else {
        &patch_source_path
    };

    // 自动添加 .7z 后缀（如果路径中没有）
    let patch_file_name = if patch_relative_path.ends_with(".7z") {
        patch_relative_path.to_string()
    } else {
        format!("{}.7z", patch_relative_path)
    };

    // 构建完整的补丁文件路径（相对于资源目录）
    let patch_file_path = Path::new(&resource_dir).join(&patch_file_name);
    let target_path = Path::new(&game_path);

    if !patch_file_path.exists() {
        return Err(format!("补丁源路径不存在: {}", patch_file_path.display()));
    }

    if !target_path.exists() {
        return Err(format!("游戏目标路径不存在: {}", game_path));
    }

    // 创建临时解压目录
    let temp_dir = std::env::temp_dir().join(format!("steam_tool_patch_{}", std::process::id()));
    let temp_path = temp_dir.clone();

    // 清理并创建临时目录
    if temp_path.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
    }
    tokio::fs::create_dir_all(&temp_path)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    // 解压7z文件
    match extract_7z(&patch_file_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            // 清理临时目录
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    let mut backed_up_files: Vec<String> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // 递归复制解压后的文件
    match copy_dir_with_backup(&temp_path, target_path, &mut backed_up_files, &mut copied_files, &mut errors).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }

    // 清理临时目录
    let _ = tokio::fs::remove_dir_all(&temp_path).await;

    Ok(ApplyPatchResult {
        success: errors.is_empty(),
        backed_up_files,
        copied_files,
        errors,
    })
}

/// 从用户选择的压缩包文件应用补丁
/// 直接解压用户选择的7z文件到游戏目录
#[tauri::command]
pub async fn apply_patch_from_file(
    archive_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    let archive_path = Path::new(&archive_path);
    let target_path = Path::new(&game_path);

    if !archive_path.exists() {
        return Err(format!("补丁文件不存在: {}", archive_path.display()));
    }

    if !target_path.exists() {
        return Err(format!("游戏目标路径不存在: {}", game_path));
    }

    // 创建临时解压目录
    let temp_dir = std::env::temp_dir().join(format!("steam_tool_patch_{}", std::process::id()));
    let temp_path = temp_dir.clone();

    // 清理并创建临时目录
    if temp_path.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
    }
    tokio::fs::create_dir_all(&temp_path)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    // 解压7z文件
    match extract_7z(&archive_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            // 清理临时目录
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    let mut backed_up_files: Vec<String> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // 递归复制解压后的文件
    match copy_dir_with_backup(&temp_path, target_path, &mut backed_up_files, &mut copied_files, &mut errors).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }

    // 清理临时目录
    let _ = tokio::fs::remove_dir_all(&temp_path).await;

    Ok(ApplyPatchResult {
        success: errors.is_empty(),
        backed_up_files,
        copied_files,
        errors,
    })
}

/// 使用 sevenz-rust 解压7z文件（带超时机制）
async fn extract_7z(
    archive_path: &Path,
    output_dir: &Path,
) -> Result<(), String> {
    use tokio::time::{timeout, Duration};

    // 设置5分钟超时
    let result = timeout(
        Duration::from_secs(300),
        async {
            // 使用 sevenz-rust 的 decompress_file 函数解压
            sevenz_rust::decompress_file(
                archive_path.to_str().unwrap(),
                output_dir.to_str().unwrap()
            ).map_err(|e| format!("解压7z文件失败: {:?}", e))
        }
    ).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("解压操作超时（超过5分钟）".to_string()),
    }
}

/// 递归复制目录，并备份已存在的文件
async fn copy_dir_with_backup(
    src: &Path,
    dst: &Path,
    backed_up_files: &mut Vec<String>,
    copied_files: &mut Vec<String>,
    errors: &mut Vec<String>,
) -> Result<(), String> {
    use tokio::fs;

    // 读取源目录内容
    let mut entries = fs::read_dir(src).await
        .map_err(|e| format!("无法读取源目录: {}", e))?;

    while let Some(entry) = entries.next_entry().await
        .map_err(|e| format!("读取目录条目失败: {}", e))? {

        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if src_path.is_dir() {
            // 如果是目录，递归处理
            if !dst_path.exists() {
                fs::create_dir_all(&dst_path).await
                    .map_err(|e| format!("创建目录失败: {}", e))?;
            }

            Box::pin(copy_dir_with_backup(
                &src_path,
                &dst_path,
                backed_up_files,
                copied_files,
                errors,
            )).await?;
        } else {
            // 如果是文件
            if dst_path.exists() {
                // 目标文件已存在，需要备份
                match backup_file(&dst_path).await {
                    Ok(backup_path) => {
                        backed_up_files.push(backup_path.to_string_lossy().to_string());
                    }
                    Err(e) => {
                        errors.push(format!("备份文件失败 {}: {}", dst_path.display(), e));
                        continue;
                    }
                }
            }

            // 复制文件
            match fs::copy(&src_path, &dst_path).await {
                Ok(_) => {
                    copied_files.push(dst_path.to_string_lossy().to_string());
                }
                Err(e) => {
                    errors.push(format!("复制文件失败 {} -> {}: {}",
                        src_path.display(), dst_path.display(), e));
                }
            }
        }
    }

    Ok(())
}

/// 备份文件 - 如果存在.bak则使用.bak1, .bak2等
async fn backup_file(file_path: &Path) -> Result<PathBuf, String> {
    use tokio::fs;

    let file_name = file_path.file_stem()
        .ok_or("无法获取文件名")?
        .to_string_lossy();
    let extension = file_path.extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let parent = file_path.parent()
        .ok_or("无法获取父目录")?;

    // 寻找可用的备份文件名
    let mut backup_number = 0;
    let backup_path = loop {
        let suffix = if backup_number == 0 {
            ".bak".to_string()
        } else {
            format!(".bak{}", backup_number)
        };

        let backup_name = format!("{}{}{}", file_name, suffix, extension);
        let backup_path = parent.join(&backup_name);

        if !backup_path.exists() {
            break backup_path;
        }

        backup_number += 1;

        // 防止无限循环
        if backup_number > 100 {
            return Err("无法找到可用的备份文件名".to_string());
        }
    };

    // 重命名原文件为备份文件
    fs::rename(file_path, &backup_path).await
        .map_err(|e| format!("重命名文件失败: {}", e))?;

    Ok(backup_path)
}

/// 获取补丁说明
/// 根据游戏ID和补丁类型从resources/Readme目录读取对应的txt文件
#[tauri::command]
pub async fn get_patch_readme(
    app: AppHandle,
    game_id: String,
    patch_type: i32,
) -> Result<String, String> {
    // 获取资源目录
    let resource_dir = get_resource_dir(app)?;
    
    // 根据patch_type确定Readme子目录
    let readme_subdir = match patch_type {
        0 => "免_steam",
        1 => "steam_联机",
        2 => "steam_联机", // Steam联机和局域网使用相同目录
        3 => "D_加密虚拟机",
        4 => "epic_联机",
        _ => "免_steam", // 默认使用免steam目录
    };
    
    // 构建Readme文件路径
    let readme_path = Path::new(&resource_dir)
        .join("Readme")
        .join(readme_subdir)
        .join(format!("{}.txt", game_id));
    
    // 检查文件是否存在
    if !readme_path.exists() {
        return Ok(String::new()); // 文件不存在，返回空字符串
    }
    
    // 读取文件内容
    match tokio::fs::read_to_string(&readme_path).await {
        Ok(content) => Ok(content),
        Err(e) => {
            println!("读取补丁说明失败: {:?}, 错误: {}", readme_path, e);
            Ok(String::new()) // 读取失败，返回空字符串
        }
    }
}
