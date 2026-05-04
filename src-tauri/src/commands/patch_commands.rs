// 补丁注入命令
// 处理免Steam补丁注入相关的IPC调用
// 100% 实现 gbe_fork 所有配置文件的读写支持

use crate::models::steam_config::*;
use crate::utils::resource_utils::get_resource_dir as get_resource_dir_util;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

// ============================================
// 基础命令
// ============================================

/// 获取资源目录（IPC命令版本）
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

// ============================================
// Steamless 脱壳
// ============================================

/// 使用 Steamless 脱壳游戏主程序
#[tauri::command]
pub async fn unpack_game_exe(
    app: AppHandle,
    game_exe_path: String,
) -> Result<UnpackResult, String> {
    use std::process::Command;

    let resource_dir = get_resource_dir(app)?;
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

    if let Some(ext) = target_exe.extension() {
        if ext.to_string_lossy().to_lowercase() != "exe" {
            return Err(format!("选择的文件不是 .exe 文件: {}", game_exe_path));
        }
    } else {
        return Err("选择的文件没有扩展名".to_string());
    }

    let target_exe_str = target_exe.to_string_lossy();

    let output = Command::new(&steamless_path)
        .arg("--quiet")
        .arg(&*target_exe_str)
        .output()
        .map_err(|e| format!("运行 Steamless 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        let unpacked_exe = target_exe.with_extension("exe.unpacked.exe");
        let unpacked_path = if unpacked_exe.exists() {
            let backup_path = target_exe.with_extension("exe.bak");
            std::fs::rename(target_exe, &backup_path)
                .map_err(|e| format!("备份原文件失败: {}", e))?;
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

// ============================================
// 基础配置应用
// ============================================

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

    let resource_dir = get_resource_dir(app)?;
    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err(format!("游戏路径不存在: {}", game_path));
    }

    let steam_settings_dir = game_dir.join("steam_settings");
    let mode = emulator_mode.unwrap_or(0);

    // 第1步: 复制 steam_settings.EXAMPLE 文件夹
    let example_dir = Path::new(&resource_dir).join("gbe_fork").join("steam_settings.EXAMPLE");
    
    if example_dir.exists() {
        if steam_settings_dir.exists() {
            fs::remove_dir_all(&steam_settings_dir)
                .await
                .map_err(|e| format!("删除旧 steam_settings 失败: {}", e))?;
        }

        copy_dir_recursive(&example_dir, &steam_settings_dir)
            .await
            .map_err(|e| format!("复制 steam_settings.EXAMPLE 失败: {}", e))?;

        rename_example_files(&steam_settings_dir)
            .await
            .map_err(|e| format!("重命名示例文件失败: {}", e))?;
    }

    // 第2步: 判断游戏架构
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

    // 第3步: 根据模式处理 DLL
    if mode == 0 {
        // 标准模式: 替换 steam_api.dll
        let api_dll_name = if is_64bit { "steam_api64.dll" } else { "steam_api.dll" };
        let original_api_path = game_dir.join(&api_dll_name);

        if !original_api_path.exists() {
            return Err(format!(
                "未找到 {}！\n\n可能原因：\n1. 选择的游戏目录不正确\n2. 游戏使用 steamclient.dll 而非 steam_api.dll\n3. 请先尝试「高级模式】",
                api_dll_name
            ));
        }

        // 备份原 DLL
        let backup_path = game_dir.join(format!("{}.bak", api_dll_name));
        if !backup_path.exists() {
            fs::copy(&original_api_path, &backup_path)
                .await
                .map_err(|e| format!("备份原 DLL 失败: {}", e))?;
        }

        // 生成 steam_interfaces.txt
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

        // 替换 steam_api.dll
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
        // 高级模式: 替换 steamclient.dll + steam_api.dll
        
        let client_dll_name = if is_64bit { "steamclient64.dll" } else { "steamclient.dll" };
        let original_client_path = game_dir.join(&client_dll_name);

        if !original_client_path.exists() {
            return Err(format!(
                "未找到 {}！\n\n可能原因：\n1. 选择的游戏目录不正确\n2. 该游戏不支持高级模式\n3. 请尝试「标准模式】",
                client_dll_name
            ));
        }

        let backup_path = game_dir.join(format!("{}.bak", client_dll_name));
        if !backup_path.exists() {
            fs::copy(&original_client_path, &backup_path)
                .await
                .map_err(|e| format!("备份 steamclient 失败: {}", e))?;
        }

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

        // 同步替换 steam_api.dll（实验版）
        let api_dll_name = if is_64bit { "steam_api64.dll" } else { "steam_api.dll" };
        let api_source = if is_64bit {
            "gbe_fork/experimental/x64/steam_api64.dll"
        } else {
            "gbe_fork/experimental/x32/steam_api.dll"
        };

        let api_source_path = Path::new(&resource_dir).join(api_source);
        let api_target_path = game_dir.join(&api_dll_name);

        if api_source_path.exists() {
            if api_target_path.exists() {
                let api_backup_path = game_dir.join(format!("{}.bak", api_dll_name));
                if !api_backup_path.exists() {
                    fs::copy(&api_target_path, &api_backup_path)
                        .await
                        .map_err(|e| format!("备份 steam_api 失败: {}", e))?;
                }
            }

            fs::copy(&api_source_path, &api_target_path)
                .await
                .map_err(|e| format!("复制 steam_api 失败: {}", e))?;
        } else {
            return Err(format!("源 steam_api 文件不存在: {}", api_source_path.display()));
        }
    }

    // 第4步: 双路径写入 steam_appid.txt
    let appid_path_settings = steam_settings_dir.join("steam_appid.txt");
    let mut appid_file_settings = fs::File::create(&appid_path_settings)
        .await
        .map_err(|e| format!("创建 steam_settings/steam_appid.txt 失败: {}", e))?;
    appid_file_settings.write_all(steam_app_id.as_bytes())
        .await
        .map_err(|e| format!("写入 steam_settings/steam_appid.txt 失败: {}", e))?;

    let appid_path_root = game_dir.join("steam_appid.txt");
    let mut appid_file_root = fs::File::create(&appid_path_root)
        .await
        .map_err(|e| format!("创建根目录 steam_appid.txt 失败: {}", e))?;
    appid_file_root.write_all(steam_app_id.as_bytes())
        .await
        .map_err(|e| format!("写入根目录 steam_appid.txt 失败: {}", e))?;

    // 第5步: 写入基础配置文件
    let main_config = MainConfig::default_config();
    let main_config_path = steam_settings_dir.join("configs.main.ini");
    let main_config_content = main_config.to_ini();
    
    let mut main_file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    main_file.write_all(main_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    let user_config = UserConfig::default_config();
    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config_content = user_config.to_ini();
    
    let mut user_file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    user_file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    let app_config = SteamAppConfig::default_config();
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let app_config_content = app_config.to_ini();
    
    let mut app_file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    app_file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    // 写入 custom_broadcasts.txt
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let broadcasts_content = "192.168.1.0/24\n192.168.0.0/24\n10.0.0.0/24\n";
    
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

// ============================================
// Overlay 配置
// ============================================

/// 保存 Overlay 配置
#[tauri::command]
pub async fn save_overlay_config(
    game_path: String,
    config: OverlayConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let overlay_config_path = steam_settings_dir.join("configs.overlay.ini");
    let overlay_config_content = config.to_ini();

    let mut file = fs::File::create(&overlay_config_path)
        .await
        .map_err(|e| format!("创建 configs.overlay.ini 失败: {}", e))?;
    file.write_all(overlay_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.overlay.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "Overlay 配置已保存".to_string(),
    })
}

/// 加载 Overlay 配置
#[tauri::command]
pub async fn load_overlay_config(
    game_path: String,
) -> Result<ConfigLoadResult<OverlayConfig>, String> {
    use tokio::fs;

    let overlay_config_path = Path::new(&game_path).join("steam_settings").join("configs.overlay.ini");

    if !overlay_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&overlay_config_path)
        .await
        .map_err(|e| format!("读取 configs.overlay.ini 失败: {}", e))?;

    // 解析 INI 内容
    let config = parse_overlay_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

fn parse_overlay_ini(content: &str) -> Result<OverlayConfig, String> {
    let mut config = OverlayConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "overlay" => {
                    match key {
                        "enable_experimental_overlay" => config.enable_experimental_overlay = value == "1" || value.to_lowercase() == "true",
                        "enabled" => config.enable_experimental_overlay = value == "1" || value.to_lowercase() == "true",
                        "hotkey" => config.hotkey = value.to_string(),
                        _ => {}
                    }
                }
                "overlay::notifications" => {
                    match key {
                        "achievement" => config.notifications.achievement = value == "1",
                        "friend" => config.notifications.friend = value == "1",
                        "message" => config.notifications.message = value == "1",
                        "duration" => config.notifications.duration = value.parse().unwrap_or(5),
                        "position" => config.notifications.position = value.to_string(),
                        _ => {}
                    }
                }
                "overlay::appearance" => {
                    match key {
                        "theme" => config.appearance.theme = value.to_string(),
                        "opacity" => config.appearance.opacity = value.parse().unwrap_or(0.95),
                        "scale" => config.appearance.scale = value.parse().unwrap_or(1.0),
                        "blur" => config.appearance.blur = value == "1",
                        _ => {}
                    }
                }
                "overlay::performance" => {
                    match key {
                        "hardware_acceleration" => config.performance.hardware_acceleration = value == "1",
                        "fps_limit" => config.performance.fps_limit = value.parse().unwrap_or(60),
                        "low_performance_mode" => config.performance.low_performance_mode = value == "1",
                        _ => {}
                    }
                }
                "overlay::features" => {
                    match key {
                        "achievements" => config.features.achievements = value == "1",
                        "friends" => config.features.friends = value == "1",
                        "chat" => config.features.chat = value == "1",
                        "browser" => config.features.browser = value == "1",
                        "settings" => config.features.settings = value == "1",
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 成就配置
// ============================================

/// 保存成就配置
#[tauri::command]
pub async fn save_achievements_config(
    game_path: String,
    config: AchievementsConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let achievements_path = steam_settings_dir.join("achievements.json");
    let achievements_json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化成就配置失败: {}", e))?;

    let mut file = fs::File::create(&achievements_path)
        .await
        .map_err(|e| format!("创建 achievements.json 失败: {}", e))?;
    file.write_all(achievements_json.as_bytes())
        .await
        .map_err(|e| format!("写入 achievements.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "成就配置已保存".to_string(),
    })
}

/// 加载成就配置
#[tauri::command]
pub async fn load_achievements_config(
    game_path: String,
) -> Result<ConfigLoadResult<AchievementsConfig>, String> {
    use tokio::fs;

    let achievements_path = Path::new(&game_path).join("steam_settings").join("achievements.json");

    if !achievements_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&achievements_path)
        .await
        .map_err(|e| format!("读取 achievements.json 失败: {}", e))?;

    let config: AchievementsConfig = serde_json::from_str(&content)
        .map_err(|e| format!("解析 achievements.json 失败: {}", e))?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 从文件导入成就配置
#[tauri::command]
pub async fn import_achievements_from_file(
    file_path: String,
) -> Result<ImportResult<Achievement>, String> {
    use tokio::fs;

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取文件失败: {}", e))?;

    // 尝试解析为成就配置
    let achievements: Vec<Achievement> = serde_json::from_str(&content)
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let count = achievements.len();

    Ok(ImportResult {
        success: true,
        data: achievements,
        message: format!("成功导入 {} 个成就", count),
    })
}

/// 导出成就配置
#[tauri::command]
pub async fn export_achievements_config(
    game_path: String,
) -> Result<ExportResult, String> {
    use tokio::fs;

    let achievements_path = Path::new(&game_path).join("steam_settings").join("achievements.json");

    if !achievements_path.exists() {
        return Ok(ExportResult {
            success: false,
            data: None,
            message: "成就配置文件不存在".to_string(),
        });
    }

    let content = fs::read_to_string(&achievements_path)
        .await
        .map_err(|e| format!("读取 achievements.json 失败: {}", e))?;

    Ok(ExportResult {
        success: true,
        data: Some(content),
        message: "导出成功".to_string(),
    })
}

// ============================================
// 统计配置
// ============================================

/// 保存统计配置
#[tauri::command]
pub async fn save_stats_config(
    game_path: String,
    config: StatsConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let stats_path = steam_settings_dir.join("stats.json");
    let stats_json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化统计配置失败: {}", e))?;

    let mut file = fs::File::create(&stats_path)
        .await
        .map_err(|e| format!("创建 stats.json 失败: {}", e))?;
    file.write_all(stats_json.as_bytes())
        .await
        .map_err(|e| format!("写入 stats.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "统计配置已保存".to_string(),
    })
}

/// 加载统计配置
#[tauri::command]
pub async fn load_stats_config(
    game_path: String,
) -> Result<ConfigLoadResult<StatsConfig>, String> {
    use tokio::fs;

    let stats_path = Path::new(&game_path).join("steam_settings").join("stats.json");

    if !stats_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&stats_path)
        .await
        .map_err(|e| format!("读取 stats.json 失败: {}", e))?;

    let config: StatsConfig = serde_json::from_str(&content)
        .map_err(|e| format!("解析 stats.json 失败: {}", e))?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

// ============================================
// 物品配置
// ============================================

/// 保存物品配置
#[tauri::command]
pub async fn save_items_config(
    game_path: String,
    config: ItemsConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let items_path = steam_settings_dir.join("items.json");
    let items_json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化物品配置失败: {}", e))?;

    let mut file = fs::File::create(&items_path)
        .await
        .map_err(|e| format!("创建 items.json 失败: {}", e))?;
    file.write_all(items_json.as_bytes())
        .await
        .map_err(|e| format!("写入 items.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "物品配置已保存".to_string(),
    })
}

/// 加载物品配置
#[tauri::command]
pub async fn load_items_config(
    game_path: String,
) -> Result<ConfigLoadResult<ItemsConfig>, String> {
    use tokio::fs;

    let items_path = Path::new(&game_path).join("steam_settings").join("items.json");

    if !items_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&items_path)
        .await
        .map_err(|e| format!("读取 items.json 失败: {}", e))?;

    let config: ItemsConfig = serde_json::from_str(&content)
        .map_err(|e| format!("解析 items.json 失败: {}", e))?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

// ============================================
// 模组配置
// ============================================

/// 保存模组配置
#[tauri::command]
pub async fn save_mods_config(
    game_path: String,
    config: ModsConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let mods_path = steam_settings_dir.join("mods.json");
    let mods_json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化模组配置失败: {}", e))?;

    let mut file = fs::File::create(&mods_path)
        .await
        .map_err(|e| format!("创建 mods.json 失败: {}", e))?;
    file.write_all(mods_json.as_bytes())
        .await
        .map_err(|e| format!("写入 mods.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "模组配置已保存".to_string(),
    })
}

/// 加载模组配置
#[tauri::command]
pub async fn load_mods_config(
    game_path: String,
) -> Result<ConfigLoadResult<ModsConfig>, String> {
    use tokio::fs;

    let mods_path = Path::new(&game_path).join("steam_settings").join("mods.json");

    if !mods_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&mods_path)
        .await
        .map_err(|e| format!("读取 mods.json 失败: {}", e))?;

    let config: ModsConfig = serde_json::from_str(&content)
        .map_err(|e| format!("解析 mods.json 失败: {}", e))?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

// ============================================
// 排行榜配置
// ============================================

/// 保存排行榜配置
#[tauri::command]
pub async fn save_leaderboards_config(
    game_path: String,
    config: LeaderboardsConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let leaderboards_path = steam_settings_dir.join("leaderboards.txt");
    let leaderboards_content = config.to_txt();

    let mut file = fs::File::create(&leaderboards_path)
        .await
        .map_err(|e| format!("创建 leaderboards.txt 失败: {}", e))?;
    file.write_all(leaderboards_content.as_bytes())
        .await
        .map_err(|e| format!("写入 leaderboards.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "排行榜配置已保存".to_string(),
    })
}

/// 加载排行榜配置
#[tauri::command]
pub async fn load_leaderboards_config(
    game_path: String,
) -> Result<ConfigLoadResult<LeaderboardsConfig>, String> {
    use tokio::fs;

    let leaderboards_path = Path::new(&game_path).join("steam_settings").join("leaderboards.txt");

    if !leaderboards_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&leaderboards_path)
        .await
        .map_err(|e| format!("读取 leaderboards.txt 失败: {}", e))?;

    let config = parse_leaderboards_txt(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

fn parse_leaderboards_txt(content: &str) -> Result<LeaderboardsConfig, String> {
    let mut config = LeaderboardsConfig::default_config();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 {
            config.leaderboards.push(Leaderboard {
                name: parts[0].to_string(),
                display_name: parts[1].to_string(),
                sort_method: parts[2].to_string(),
                display_type: parts[3].to_string(),
                entries: vec![],
            });
        }
    }

    Ok(config)
}

// ============================================
// 控制器配置
// ============================================

/// 保存控制器配置
#[tauri::command]
pub async fn save_controller_config(
    game_path: String,
    config: ControllerConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let controller_dir = Path::new(&game_path).join("steam_settings").join("controller");
    fs::create_dir_all(&controller_dir)
        .await
        .map_err(|e| format!("创建 controller 目录失败: {}", e))?;

    // 保存配置文件
    let config_path = Path::new(&game_path).join("steam_settings").join("configs.controller.ini");
    let config_content = format!(
        r#"[controller]
enabled = {}
type = {}

[controller::deadzone]
left_stick = {:.2}
right_stick = {:.2}
left_trigger = {:.2}
right_trigger = {:.2}

[controller::rumble]
enabled = {}
intensity = {:.2}

[controller::custom_glyphs]
enabled = {}
"#,
        config.enabled as i32,
        config.controller_type,
        config.deadzone.left_stick,
        config.deadzone.right_stick,
        config.deadzone.left_trigger,
        config.deadzone.right_trigger,
        config.rumble.enabled as i32,
        config.rumble.intensity,
        config.custom_glyphs.enabled as i32
    );

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 configs.controller.ini 失败: {}", e))?;
    file.write_all(config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.controller.ini 失败: {}", e))?;

    // 保存按键绑定
    let bindings_path = controller_dir.join("bindings.txt");
    let mut bindings_content = String::new();
    for binding in &config.bindings {
        bindings_content.push_str(&format!("{} = {} # {}\n", 
            binding.action, 
            binding.button,
            binding.description.as_deref().unwrap_or("")
        ));
    }

    let mut bindings_file = fs::File::create(&bindings_path)
        .await
        .map_err(|e| format!("创建 bindings.txt 失败: {}", e))?;
    bindings_file.write_all(bindings_content.as_bytes())
        .await
        .map_err(|e| format!("写入 bindings.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "控制器配置已保存".to_string(),
    })
}

/// 加载控制器配置
#[tauri::command]
pub async fn load_controller_config(
    game_path: String,
) -> Result<ConfigLoadResult<ControllerConfig>, String> {
    use tokio::fs;

    let config_path = Path::new(&game_path).join("steam_settings").join("configs.controller.ini");

    if !config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取 configs.controller.ini 失败: {}", e))?;

    let config = parse_controller_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

fn parse_controller_ini(content: &str) -> Result<ControllerConfig, String> {
    let mut config = ControllerConfig::default();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "controller" => {
                    match key {
                        "enabled" => config.enabled = value == "1",
                        "type" => config.controller_type = value.to_string(),
                        _ => {}
                    }
                }
                "controller::deadzone" => {
                    match key {
                        "left_stick" => config.deadzone.left_stick = value.parse().unwrap_or(0.1),
                        "right_stick" => config.deadzone.right_stick = value.parse().unwrap_or(0.1),
                        "left_trigger" => config.deadzone.left_trigger = value.parse().unwrap_or(0.1),
                        "right_trigger" => config.deadzone.right_trigger = value.parse().unwrap_or(0.1),
                        _ => {}
                    }
                }
                "controller::rumble" => {
                    match key {
                        "enabled" => config.rumble.enabled = value == "1",
                        "intensity" => config.rumble.intensity = value.parse().unwrap_or(0.8),
                        _ => {}
                    }
                }
                "controller::custom_glyphs" => {
                    match key {
                        "enabled" => config.custom_glyphs.enabled = value == "1",
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 用户配置
// ============================================

/// 保存用户配置
#[tauri::command]
pub async fn save_user_config(
    game_path: String,
    config: UserConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config_content = config.to_ini();

    let mut file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "用户配置已保存".to_string(),
    })
}

/// 加载用户配置
#[tauri::command]
pub async fn load_user_config(
    game_path: String,
) -> Result<ConfigLoadResult<UserConfig>, String> {
    use tokio::fs;

    let user_config_path = Path::new(&game_path).join("steam_settings").join("configs.user.ini");

    if !user_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&user_config_path)
        .await
        .map_err(|e| format!("读取 configs.user.ini 失败: {}", e))?;

    let config = parse_user_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

fn parse_user_ini(content: &str) -> Result<UserConfig, String> {
    let mut config = UserConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            if current_section == "user" {
                match key {
                    "username" => config.username = value.to_string(),
                    "language" => config.language = value.to_string(),
                    "save_path" => config.save_path = value.to_string(),
                    "avatar_path" => config.avatar_path = Some(value.to_string()),
                    "use_default_avatar" => config.use_default_avatar = value == "1",
                    "saves_folder_name" => config.saves_folder_name = Some(value.to_string()),
                    "local_save_path" => config.local_save_path = Some(value.to_string()),
                    "ticket" => config.ticket = Some(value.to_string()),
                    _ => {}
                }
            }
        }
    }

    Ok(config)
}

// ============================================
// 主配置
// ============================================

/// 保存主配置
#[tauri::command]
pub async fn save_main_config(
    game_path: String,
    config: MainConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let main_config_path = steam_settings_dir.join("configs.main.ini");
    let main_config_content = config.to_ini();

    let mut file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    file.write_all(main_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "主配置已保存".to_string(),
    })
}

/// 加载主配置
#[tauri::command]
pub async fn load_main_config(
    game_path: String,
) -> Result<ConfigLoadResult<MainConfig>, String> {
    use tokio::fs;

    let main_config_path = Path::new(&game_path).join("steam_settings").join("configs.main.ini");

    if !main_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&main_config_path)
        .await
        .map_err(|e| format!("读取 configs.main.ini 失败: {}", e))?;

    let config = parse_main_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

fn parse_main_ini(content: &str) -> Result<MainConfig, String> {
    let mut config = MainConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "main::general" => {
                    match key {
                        "new_app_ticket" => config.new_app_ticket = value == "1",
                        "gc_token" => config.gc_token = value == "1",
                        "block_unknown_clients" => config.block_unknown_clients = value == "1",
                        "steam_deck" => config.steam_deck = value == "1",
                        "enable_account_avatar" => config.enable_account_avatar = value == "1",
                        "enable_voice_chat" => config.enable_voice_chat = value == "1",
                        "immediate_gameserver_stats" => config.immediate_gameserver_stats = value == "1",
                        "matchmaking_server_list_actual_type" => config.matchmaking_server_list_actual_type = value == "1",
                        "matchmaking_server_details_via_source_query" => config.matchmaking_server_details_via_source_query = value == "1",
                        "crash_printer_location" => config.crash_printer_location = Some(value.to_string()),
                        _ => {}
                    }
                }
                "main::stats" => {
                    match key {
                        "disable_leaderboards_create_unknown" => config.disable_leaderboards_create_unknown = value == "1",
                        "allow_unknown_stats" => config.allow_unknown_stats = value == "1",
                        "stat_achievement_progress_functionality" => config.stat_achievement_progress_functionality = value == "1",
                        "save_only_higher_stat_achievement_progress" => config.save_only_higher_stat_achievement_progress = value == "1",
                        "paginated_achievements_icons" => config.paginated_achievements_icons = value.parse().unwrap_or(10),
                        "record_playtime" => config.record_playtime = value == "1",
                        _ => {}
                    }
                }
                "main::connectivity" => {
                    match key {
                        "disable_lan_only" => config.disable_lan_only = value == "1",
                        "disable_networking" => config.disable_networking = value == "1",
                        "listen_port" => config.listen_port = value.parse().unwrap_or(47584),
                        "offline" => config.offline = value == "1",
                        "disable_sharing_stats_with_gameserver" => config.disable_sharing_stats_with_gameserver = value == "1",
                        "disable_source_query" => config.disable_source_query = value == "1",
                        "share_leaderboards_over_network" => config.share_leaderboards_over_network = value == "1",
                        "disable_lobby_creation" => config.disable_lobby_creation = value == "1",
                        "download_steamhttp_requests" => config.download_steamhttp_requests = value == "1",
                        _ => {}
                    }
                }
                "main::misc" => {
                    match key {
                        "achievements_bypass" => config.achievements_bypass = value == "1",
                        "force_steamhttp_success" => config.force_steamhttp_success = value == "1",
                        "disable_steamoverlaygameid_env_var" => config.disable_steamoverlaygameid_env_var = value == "1",
                        "enable_steam_preowned_ids" => config.enable_steam_preowned_ids = value == "1",
                        "steam_game_stats_reports_dir" => config.steam_game_stats_reports_dir = Some(value.to_string()),
                        "free_weekend" => config.free_weekend = value == "1",
                        "use_32bit_inventory_item_ids" => config.use_32bit_inventory_item_ids = value == "1",
                        _ => {}
                    }
                }
                "extra_dlls" => {
                    if key.starts_with("dll") {
                        config.extra_dlls.push(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 局域网联机配置
// ============================================

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
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

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
            _ => String::new(),
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

/// 加载局域网联机配置
#[tauri::command]
pub async fn load_lan_multiplayer_config(
    game_path: String,
) -> Result<serde_json::Value, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    
    // 读取 custom_broadcasts.txt
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let custom_broadcasts = if broadcasts_path.exists() {
        let content = fs::read_to_string(&broadcasts_path)
            .await
            .map_err(|e| format!("读取 custom_broadcasts.txt 失败: {}", e))?;
        content.lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    // 读取 auto_accept_invite.txt
    let auto_accept_path = steam_settings_dir.join("auto_accept_invite.txt");
    let (auto_accept, whitelist) = if auto_accept_path.exists() {
        let content = fs::read_to_string(&auto_accept_path)
            .await
            .map_err(|e| format!("读取 auto_accept_invite.txt 失败: {}", e))?;
        let lines: Vec<String> = content.lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if lines.is_empty() {
            ("none".to_string(), vec![])
        } else if lines.len() == 1 && lines[0] == "*" {
            ("all".to_string(), vec![])
        } else {
            ("whitelist".to_string(), lines)
        }
    } else {
        ("none".to_string(), vec![])
    };

    Ok(serde_json::json!({
        "exists": broadcasts_path.exists() || auto_accept_path.exists(),
        "enabled": true,
        "customBroadcasts": custom_broadcasts,
        "autoAcceptInvite": auto_accept,
        "whitelist": whitelist,
        "listenPort": 47584
    }))
}

// ============================================
// DLC 配置
// ============================================

/// 保存 DLC 配置
#[tauri::command]
pub async fn save_dlc_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    // 读取现有的 app 配置
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let mut app_config = if app_config_path.exists() {
        let content = fs::read_to_string(&app_config_path)
            .await
            .map_err(|e| format!("读取 configs.app.ini 失败: {}", e))?;
        parse_app_ini(&content)?
    } else {
        SteamAppConfig::default_config()
    };

    // 更新 DLC 配置
    if let Some(unlock_all) = config.get("unlockAll").and_then(|v| v.as_bool()) {
        app_config.dlcs.unlock_all = unlock_all;
    }

    if let Some(dlcs) = config.get("individualDlcs").and_then(|v| v.as_array()) {
        app_config.dlcs.individual_dlcs = dlcs
            .iter()
            .filter_map(|v| {
                Some(IndividualDlc {
                    app_id: v.get("appId")?.as_str()?.to_string(),
                    name: v.get("name")?.as_str()?.to_string(),
                    enabled: v.get("enabled")?.as_bool()?,
                })
            })
            .collect();
    }

    // 保存配置
    let app_config_content = app_config.to_ini();
    let mut file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "DLC 配置已保存".to_string(),
    })
}

fn parse_app_ini(content: &str) -> Result<SteamAppConfig, String> {
    let mut config = SteamAppConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "app" => {
                    if key == "branch_name" {
                        config.branch_name = value.to_string();
                    }
                }
                "app::dlcs" => {
                    if key == "unlock_all" {
                        config.dlcs.unlock_all = value == "1";
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 应用补丁
// ============================================

/// 应用补丁 - 解压7z文件并复制到游戏目录
#[tauri::command]
pub async fn apply_patch(
    app: AppHandle,
    patch_source_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    let resource_dir = get_resource_dir_util(&app)?;

    let patch_lower = patch_source_path.to_lowercase();
    let patch_relative_path = if patch_lower.starts_with("resources/") {
        &patch_source_path[10..]
    } else if patch_lower.starts_with("resources\\") {
        &patch_source_path[11..]
    } else {
        &patch_source_path
    };

    let patch_file_name = if patch_relative_path.ends_with(".7z") {
        patch_relative_path.to_string()
    } else {
        format!("{}.7z", patch_relative_path)
    };

    let patch_file_path = Path::new(&resource_dir).join(&patch_file_name);
    let target_path = Path::new(&game_path);

    if !patch_file_path.exists() {
        return Err(format!("补丁源路径不存在: {}", patch_file_path.display()));
    }

    if !target_path.exists() {
        return Err(format!("游戏目标路径不存在: {}", game_path));
    }

    let temp_dir = std::env::temp_dir().join(format!("steam_tool_patch_{}", std::process::id()));
    let temp_path = temp_dir.clone();

    if temp_path.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
    }
    tokio::fs::create_dir_all(&temp_path)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    match extract_7z(&patch_file_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    let mut backed_up_files: Vec<String> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    match copy_dir_with_backup(&temp_path, target_path, &mut backed_up_files, &mut copied_files, &mut errors).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }

    let _ = tokio::fs::remove_dir_all(&temp_path).await;

    Ok(ApplyPatchResult {
        success: errors.is_empty(),
        backed_up_files,
        copied_files,
        errors,
    })
}

/// 从用户选择的压缩包文件应用补丁
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

    let temp_dir = std::env::temp_dir().join(format!("steam_tool_patch_{}", std::process::id()));
    let temp_path = temp_dir.clone();

    if temp_path.exists() {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
    }
    tokio::fs::create_dir_all(&temp_path)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    match extract_7z(&archive_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    let mut backed_up_files: Vec<String> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    match copy_dir_with_backup(&temp_path, target_path, &mut backed_up_files, &mut copied_files, &mut errors).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }

    let _ = tokio::fs::remove_dir_all(&temp_path).await;

    Ok(ApplyPatchResult {
        success: errors.is_empty(),
        backed_up_files,
        copied_files,
        errors,
    })
}

/// 补丁应用结果
#[derive(serde::Serialize)]
pub struct ApplyPatchResult {
    pub success: bool,
    pub backed_up_files: Vec<String>,
    pub copied_files: Vec<String>,
    pub errors: Vec<String>,
}

// ============================================
// 辅助函数
// ============================================

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

// ============================================
// 其他 gbe_fork 配置文件支持
// ============================================

/// 保存 installed_app_ids.txt
#[tauri::command]
pub async fn save_installed_app_ids(
    game_path: String,
    app_ids: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("installed_app_ids.txt");
    let content = app_ids.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 installed_app_ids.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 installed_app_ids.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "已安装应用ID已保存".to_string(),
    })
}

/// 加载 installed_app_ids.txt
#[tauri::command]
pub async fn load_installed_app_ids(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("installed_app_ids.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 installed_app_ids.txt 失败: {}", e))?;

    let app_ids: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(app_ids)
}

/// 保存 subscribed_groups.txt
#[tauri::command]
pub async fn save_subscribed_groups(
    game_path: String,
    group_ids: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("subscribed_groups.txt");
    let content = group_ids.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 subscribed_groups.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 subscribed_groups.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "订阅群组已保存".to_string(),
    })
}

/// 加载 subscribed_groups.txt
#[tauri::command]
pub async fn load_subscribed_groups(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("subscribed_groups.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 subscribed_groups.txt 失败: {}", e))?;

    let group_ids: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(group_ids)
}

/// 保存 purchased_keys.txt
#[tauri::command]
pub async fn save_purchased_keys(
    game_path: String,
    keys: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("purchased_keys.txt");
    let content = keys.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 purchased_keys.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 purchased_keys.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "购买密钥已保存".to_string(),
    })
}

/// 加载 purchased_keys.txt
#[tauri::command]
pub async fn load_purchased_keys(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("purchased_keys.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 purchased_keys.txt 失败: {}", e))?;

    let keys: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(keys)
}

/// 保存 supported_languages.txt
#[tauri::command]
pub async fn save_supported_languages(
    game_path: String,
    languages: Vec<String>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("supported_languages.txt");
    let content = languages.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 supported_languages.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 supported_languages.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "支持语言已保存".to_string(),
    })
}

/// 加载 supported_languages.txt
#[tauri::command]
pub async fn load_supported_languages(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("supported_languages.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 supported_languages.txt 失败: {}", e))?;

    let languages: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(languages)
}

/// 保存 depots.txt
#[tauri::command]
pub async fn save_depots(
    game_path: String,
    depots: Vec<serde_json::Value>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("depots.txt");
    let mut lines = vec![];
    
    for depot in depots {
        if let (Some(id), Some(manifest)) = (
            depot.get("depotId").and_then(|v| v.as_str()),
            depot.get("manifestId").and_then(|v| v.as_str())
        ) {
            lines.push(format!("{} {}", id, manifest));
        }
    }

    let content = lines.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 depots.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 depots.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "Depot 配置已保存".to_string(),
    })
}

/// 加载 depots.txt
#[tauri::command]
pub async fn load_depots(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("depots.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 depots.txt 失败: {}", e))?;

    let mut depots = vec![];
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            depots.push(serde_json::json!({
                "depotId": parts[0],
                "manifestId": parts[1]
            }));
        }
    }

    Ok(depots)
}

/// 保存 branches.json
#[tauri::command]
pub async fn save_branches(
    game_path: String,
    branches: Vec<serde_json::Value>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("branches.json");
    let json = serde_json::to_string_pretty(&branches)
        .map_err(|e| format!("序列化 branches 失败: {}", e))?;

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 branches.json 失败: {}", e))?;
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| format!("写入 branches.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "分支配置已保存".to_string(),
    })
}

/// 加载 branches.json
#[tauri::command]
pub async fn load_branches(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("branches.json");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 branches.json 失败: {}", e))?;

    let branches: Vec<serde_json::Value> = serde_json::from_str(&content)
        .map_err(|e| format!("解析 branches.json 失败: {}", e))?;

    Ok(branches)
}

/// 保存 gc.json (游戏协调器)
#[tauri::command]
pub async fn save_game_coordinator(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("gc.json");
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化 gc 配置失败: {}", e))?;

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 gc.json 失败: {}", e))?;
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| format!("写入 gc.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "游戏协调器配置已保存".to_string(),
    })
}

/// 加载 gc.json
#[tauri::command]
pub async fn load_game_coordinator(
    game_path: String,
) -> Result<serde_json::Value, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("gc.json");

    if !file_path.exists() {
        return Ok(serde_json::json!({}));
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 gc.json 失败: {}", e))?;

    let config: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 gc.json 失败: {}", e))?;

    Ok(config)
}

// ============================================
// 订阅群组（公会）配置
// ============================================

/// 保存 subscribed_groups_clans.txt
#[tauri::command]
pub async fn save_subscribed_groups_clans(
    game_path: String,
    groups: Vec<serde_json::Value>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("subscribed_groups_clans.txt");
    let mut lines = vec![];

    for group in groups {
        let group_id = group.get("groupId").and_then(|v| v.as_str()).unwrap_or("");
        let name = group.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let tag = group.get("tag").and_then(|v| v.as_str()).unwrap_or("");

        if !group_id.is_empty() {
            lines.push(format!("{}\t\t{}\t\t{}", group_id, name, tag));
        }
    }

    let content = lines.join("\n");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 subscribed_groups_clans.txt 失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 subscribed_groups_clans.txt 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "订阅群组（公会）配置已保存".to_string(),
    })
}

/// 加载 subscribed_groups_clans.txt
#[tauri::command]
pub async fn load_subscribed_groups_clans(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("subscribed_groups_clans.txt");

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 subscribed_groups_clans.txt 失败: {}", e))?;

    let mut groups = vec![];
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').filter(|s| !s.is_empty()).collect();
        if parts.len() >= 3 {
            groups.push(serde_json::json!({
                "groupId": parts[0].trim(),
                "name": parts[1].trim(),
                "tag": parts[2].trim()
            }));
        } else if parts.len() == 1 {
            groups.push(serde_json::json!({
                "groupId": parts[0].trim(),
                "name": "",
                "tag": ""
            }));
        }
    }

    Ok(groups)
}

// ============================================
// 默认物品配置
// ============================================

/// 保存 default_items.json
#[tauri::command]
pub async fn save_default_items(
    game_path: String,
    items: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_path = steam_settings_dir.join("default_items.json");
    let json = serde_json::to_string_pretty(&items)
        .map_err(|e| format!("序列化 default_items 失败: {}", e))?;

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 default_items.json 失败: {}", e))?;
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| format!("写入 default_items.json 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "默认物品配置已保存".to_string(),
    })
}

/// 加载 default_items.json
#[tauri::command]
pub async fn load_default_items(
    game_path: String,
) -> Result<serde_json::Value, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("default_items.json");

    if !file_path.exists() {
        return Ok(serde_json::json!({}));
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 default_items.json 失败: {}", e))?;

    let items: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 default_items.json 失败: {}", e))?;

    Ok(items)
}

// ============================================
// 音效配置
// ============================================

/// 保存音效文件
#[tauri::command]
pub async fn save_sound_file(
    game_path: String,
    sound_type: String,
    file_data: Vec<u8>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let sounds_dir = Path::new(&game_path).join("steam_settings").join("sounds");
    fs::create_dir_all(&sounds_dir)
        .await
        .map_err(|e| format!("创建 sounds 目录失败: {}", e))?;

    let file_name = match sound_type.as_str() {
        "achievement" => "overlay_achievement_notification.wav",
        "friend" => "overlay_friend_notification.wav",
        "message" => "overlay_message_notification.wav",
        _ => return Err(format!("未知的音效类型: {}", sound_type)),
    };

    let file_path = sounds_dir.join(file_name);

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建音效文件失败: {}", e))?;
    file.write_all(&file_data)
        .await
        .map_err(|e| format!("写入音效文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: format!("{} 音效已保存", sound_type),
    })
}

/// 加载音效文件
#[tauri::command]
pub async fn load_sound_file(
    game_path: String,
    sound_type: String,
) -> Result<Option<Vec<u8>>, String> {
    use tokio::fs;

    let file_name = match sound_type.as_str() {
        "achievement" => "overlay_achievement_notification.wav",
        "friend" => "overlay_friend_notification.wav",
        "message" => "overlay_message_notification.wav",
        _ => return Err(format!("未知的音效类型: {}", sound_type)),
    };

    let file_path = Path::new(&game_path).join("steam_settings").join("sounds").join(file_name);

    if !file_path.exists() {
        return Ok(None);
    }

    let data = fs::read(&file_path)
        .await
        .map_err(|e| format!("读取音效文件失败: {}", e))?;

    Ok(Some(data))
}

/// 检查音效文件是否存在
#[tauri::command]
pub async fn check_sound_file_exists(
    game_path: String,
    sound_type: String,
) -> Result<bool, String> {
    let file_name = match sound_type.as_str() {
        "achievement" => "overlay_achievement_notification.wav",
        "friend" => "overlay_friend_notification.wav",
        "message" => "overlay_message_notification.wav",
        _ => return Err(format!("未知的音效类型: {}", sound_type)),
    };

    let file_path = Path::new(&game_path).join("steam_settings").join("sounds").join(file_name);
    Ok(file_path.exists())
}

// ============================================
// 头像配置
// ============================================

/// 保存头像文件
#[tauri::command]
pub async fn save_avatar(
    game_path: String,
    avatar_type: String,
    file_data: Vec<u8>,
    extension: String,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let file_name = match avatar_type.as_str() {
        "account" => format!("account_avatar.{}", extension),
        "default" => format!("account_avatar_default.{}", extension),
        _ => return Err(format!("未知的头像类型: {}", avatar_type)),
    };

    let file_path = steam_settings_dir.join(&file_name);

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建头像文件失败: {}", e))?;
    file.write_all(&file_data)
        .await
        .map_err(|e| format!("写入头像文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "头像已保存".to_string(),
    })
}

/// 加载头像文件
#[tauri::command]
pub async fn load_avatar(
    game_path: String,
    avatar_type: String,
) -> Result<Option<(Vec<u8>, String)>, String> {
    use tokio::fs;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");

    let base_name = match avatar_type.as_str() {
        "account" => "account_avatar",
        "default" => "account_avatar_default",
        _ => return Err(format!("未知的头像类型: {}", avatar_type)),
    };

    for ext in ["png", "jpg", "jpeg"] {
        let file_path = steam_settings_dir.join(format!("{}.{}", base_name, ext));
        if file_path.exists() {
            let data = fs::read(&file_path)
                .await
                .map_err(|e| format!("读取头像文件失败: {}", e))?;
            return Ok(Some((data, ext.to_string())));
        }
    }

    Ok(None)
}

/// 检查头像文件是否存在
#[tauri::command]
pub async fn check_avatar_exists(
    game_path: String,
    avatar_type: String,
) -> Result<bool, String> {
    let steam_settings_dir = Path::new(&game_path).join("steam_settings");

    let base_name = match avatar_type.as_str() {
        "account" => "account_avatar",
        "default" => "account_avatar_default",
        _ => return Err(format!("未知的头像类型: {}", avatar_type)),
    };

    for ext in ["png", "jpg", "jpeg"] {
        let file_path = steam_settings_dir.join(format!("{}.{}", base_name, ext));
        if file_path.exists() {
            return Ok(true);
        }
    }

    Ok(false)
}

// ============================================
// SteamHTTP 配置
// ============================================

/// 保存 SteamHTTP 响应文件
#[tauri::command]
pub async fn save_steam_http_response(
    game_path: String,
    domain: String,
    path: String,
    content: String,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let http_dir = Path::new(&game_path).join("steam_settings").join("http").join(&domain);
    fs::create_dir_all(&http_dir)
        .await
        .map_err(|e| format!("创建 http 目录失败: {}", e))?;

    let file_path = http_dir.join(&path);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("创建父目录失败: {}", e))?;
    }

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建 HTTP 响应文件失败: {}", e))?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入 HTTP 响应文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "HTTP 响应已保存".to_string(),
    })
}

/// 加载 SteamHTTP 响应文件
#[tauri::command]
pub async fn load_steam_http_response(
    game_path: String,
    domain: String,
    path: String,
) -> Result<Option<String>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path)
        .join("steam_settings")
        .join("http")
        .join(&domain)
        .join(&path);

    if !file_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("读取 HTTP 响应文件失败: {}", e))?;

    Ok(Some(content))
}

/// 列出所有 SteamHTTP 配置
#[tauri::command]
pub async fn list_steam_http_configs(
    game_path: String,
) -> Result<Vec<serde_json::Value>, String> {
    use tokio::fs;

    let http_dir = Path::new(&game_path).join("steam_settings").join("http");

    if !http_dir.exists() {
        return Ok(vec![]);
    }

    let mut configs = vec![];
    let mut entries = fs::read_dir(&http_dir)
        .await
        .map_err(|e| format!("读取 http 目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("读取条目失败: {}", e))? {
        let domain = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        if path.is_dir() {
            // 使用 spawn_blocking 来执行同步的递归函数
            let paths = tokio::task::spawn_blocking(move || list_http_paths(&path))
                .await
                .map_err(|e| format!("列出 HTTP 路径失败: {:?}", e))??;
            configs.push(serde_json::json!({
                "domain": domain,
                "paths": paths
            }));
        }
    }

    Ok(configs)
}

fn list_http_paths(dir: &Path) -> Result<Vec<String>, String> {
    use std::fs;

    let mut paths = vec![];

    let entries = fs::read_dir(dir)
        .map_err(|e| format!("读取目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
        let path = entry.path();
        if path.is_file() {
            paths.push(entry.file_name().to_string_lossy().to_string());
        } else if path.is_dir() {
            let sub_paths = list_http_paths(&path)?;
            for sub_path in sub_paths {
                paths.push(format!("{}/{}", entry.file_name().to_string_lossy(), sub_path));
            }
        }
    }

    Ok(paths)
}

// ============================================
// 字体配置
// ============================================

/// 保存字体文件
#[tauri::command]
pub async fn save_font_file(
    game_path: String,
    font_name: String,
    file_data: Vec<u8>,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let fonts_dir = Path::new(&game_path).join("steam_settings").join("fonts");
    fs::create_dir_all(&fonts_dir)
        .await
        .map_err(|e| format!("创建 fonts 目录失败: {}", e))?;

    let file_path = fonts_dir.join(&font_name);

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| format!("创建字体文件失败: {}", e))?;
    file.write_all(&file_data)
        .await
        .map_err(|e| format!("写入字体文件失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "字体文件已保存".to_string(),
    })
}

/// 加载字体文件
#[tauri::command]
pub async fn load_font_file(
    game_path: String,
    font_name: String,
) -> Result<Option<Vec<u8>>, String> {
    use tokio::fs;

    let file_path = Path::new(&game_path).join("steam_settings").join("fonts").join(&font_name);

    if !file_path.exists() {
        return Ok(None);
    }

    let data = fs::read(&file_path)
        .await
        .map_err(|e| format!("读取字体文件失败: {}", e))?;

    Ok(Some(data))
}

/// 列出所有字体文件
#[tauri::command]
pub async fn list_font_files(
    game_path: String,
) -> Result<Vec<String>, String> {
    use tokio::fs;

    let fonts_dir = Path::new(&game_path).join("steam_settings").join("fonts");

    if !fonts_dir.exists() {
        return Ok(vec![]);
    }

    let mut files = vec![];
    let mut entries = fs::read_dir(&fonts_dir)
        .await
        .map_err(|e| format!("读取 fonts 目录失败: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("读取条目失败: {}", e))? {
        let path = entry.path();
        if path.is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".ttf") || name.ends_with(".otf") {
                files.push(name);
            }
        }
    }

    Ok(files)
}

/// 重命名 .EXAMPLE 文件和文件夹
async fn rename_example_files(dir: &Path) -> Result<(), String> {
    use tokio::fs;

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
            let new_name = &file_name_str[..file_name_str.len() - 8];
            let new_path = dir.join(new_name);
            entries_to_rename.push((path.clone(), new_path));
        } else if path.is_dir() {
            sub_dirs.push(path);
        }
    }

    for sub_dir in sub_dirs {
        Box::pin(rename_example_files(&sub_dir)).await?;
    }

    for (old_path, new_path) in entries_to_rename {
        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| format!("重命名失败: {} -> {}: {}", old_path.display(), new_path.display(), e))?;
    }

    Ok(())
}

/// 使用 sevenz-rust 解压7z文件
async fn extract_7z(
    archive_path: &Path,
    output_dir: &Path,
) -> Result<(), String> {
    use tokio::time::{timeout, Duration};

    let result = timeout(
        Duration::from_secs(300),
        async {
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

    let mut entries = fs::read_dir(src).await
        .map_err(|e| format!("无法读取源目录: {}", e))?;

    while let Some(entry) = entries.next_entry().await
        .map_err(|e| format!("读取目录条目失败: {}", e))? {

        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if src_path.is_dir() {
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
            if dst_path.exists() {
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

/// 备份文件
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

        if backup_number > 100 {
            return Err("无法找到可用的备份文件名".to_string());
        }
    };

    fs::rename(file_path, &backup_path).await
        .map_err(|e| format!("重命名文件失败: {}", e))?;

    Ok(backup_path)
}

/// 获取补丁说明
#[tauri::command]
pub async fn get_patch_readme(
    app: AppHandle,
    game_id: String,
    patch_type: i32,
) -> Result<String, String> {
    let resource_dir = get_resource_dir(app)?;
    
    let readme_subdir = match patch_type {
        0 => "免_steam",
        1 => "steam_联机",
        2 => "steam_联机",
        3 => "D_加密虚拟机",
        4 => "epic_联机",
        _ => "免_steam",
    };
    
    let readme_path = Path::new(&resource_dir)
        .join("Readme")
        .join(readme_subdir)
        .join(format!("{}.txt", game_id));
    
    if !readme_path.exists() {
        return Ok(String::new());
    }
    
    match tokio::fs::read_to_string(&readme_path).await {
        Ok(content) => Ok(content),
        Err(_) => Ok(String::new()),
    }
}

// ============================================
// 通用结果类型
// ============================================

/// 配置保存结果
#[derive(serde::Serialize)]
pub struct ConfigSaveResult {
    pub success: bool,
    pub message: String,
}

/// 配置加载结果
#[derive(serde::Serialize)]
pub struct ConfigLoadResult<T> {
    pub exists: bool,
    pub config: Option<T>,
}

/// 导入结果
#[derive(serde::Serialize)]
pub struct ImportResult<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub message: String,
}

/// 导出结果
#[derive(serde::Serialize)]
pub struct ExportResult {
    pub success: bool,
    pub data: Option<String>,
    pub message: String,
}

/// 关闭应用程序
#[tauri::command]
pub fn close_application() -> Result<(), String> {
    std::process::exit(0);
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

/// 写入文本文件
#[tauri::command]
pub async fn write_text_file(path: String, content: String) -> Result<(), String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    // 确保父目录存在
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let mut file = fs::File::create(&path)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;
    
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

/// 打开虚拟化环境配置教程视频
/// 使用系统默认应用打开 resources/D加密虚拟化（虚拟机）环境搭建教程.mp4
#[tauri::command]
pub async fn open_virtualization_tutorial(app: AppHandle) -> Result<(), String> {
    use std::process::Command;
    
    // 获取程序根目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();
    
    // 构建视频文件路径
    let video_path = exe_dir.join("resources").join("D加密虚拟化（虚拟机）环境搭建教程.mp4");
    
    // 检查文件是否存在
    if !video_path.exists() {
        return Err("视频文件不存在".to_string());
    }
    
    // 使用系统默认应用打开视频
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", video_path.to_string_lossy().as_ref()])
            .spawn()
            .map_err(|e| format!("打开视频失败: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&video_path)
            .spawn()
            .map_err(|e| format!("打开视频失败: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&video_path)
            .spawn()
            .map_err(|e| format!("打开视频失败: {}", e))?;
    }
    
    Ok(())
}
