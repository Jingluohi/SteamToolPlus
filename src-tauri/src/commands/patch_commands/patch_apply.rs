// 补丁应用模块
// 应用 Steam 补丁基础配置：复制 DLL、生成 steam_settings、写入配置文件等

use crate::models::steam_config::*;
use std::path::Path;
use tauri::AppHandle;

/// 检查游戏目录中是否存在 steam_api.dll 或 steam_api64.dll
/// 标准模式下应用基础配置前的预检查
#[tauri::command]
pub async fn check_steam_dll_exists(
    game_path: String,
) -> Result<CheckDllResult, String> {
    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err(format!("游戏路径不存在: {}", game_path));
    }

    // 检查两种架构的 DLL 是否存在
    let dll_32 = game_dir.join("steam_api.dll");
    let dll_64 = game_dir.join("steam_api64.dll");

    if dll_32.exists() {
        return Ok(CheckDllResult {
            found: true,
            dll_path: dll_32.to_string_lossy().to_string(),
        });
    }

    if dll_64.exists() {
        return Ok(CheckDllResult {
            found: true,
            dll_path: dll_64.to_string_lossy().to_string(),
        });
    }

    // 都没找到
    Ok(CheckDllResult {
        found: false,
        dll_path: String::new(),
    })
}

/// DLL 检查结果
pub struct CheckDllResult {
    pub found: bool,
    pub dll_path: String,
}

impl serde::Serialize for CheckDllResult {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CheckDllResult", 2)?;
        state.serialize_field("found", &self.found)?;
        state.serialize_field("dllPath", &self.dll_path)?;
        state.end()
    }
}

/// 应用 Steam 补丁基础配置
/// 这是补丁注入的核心函数，执行以下操作：
/// 1. 复制 steam_settings.EXAMPLE 文件夹并重命名示例文件
/// 2. 判断游戏架构（32位/64位）
/// 3. 根据模式（标准/高级）替换对应的 DLL 文件
/// 4. 生成 steam_interfaces.txt（如可能）
/// 5. 写入 steam_appid.txt 到双路径
/// 6. 写入基础配置文件（main、user、app、overlay）
/// 7. 写入 custom_broadcasts.txt
/// 8. 高级模式额外复制 GameOverlayRenderer DLL
#[tauri::command]
pub async fn apply_steam_patch_basic(
    app: AppHandle,
    game_path: String,
    _game_id: String,
    steam_app_id: String,
    use_experimental: bool,
    emulator_mode: Option<i32>,
) -> Result<super::common::BasicConfigResult, String> {
    use super::common::{get_resource_dir, BasicConfigResult};
    use super::file_ops::{copy_dir_recursive, rename_example_files};
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

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
        let original_api_path = game_dir.join(api_dll_name);

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
        super::tools::generate_interfaces(&resource_dir, &original_api_path, game_dir, &steam_settings_dir).await?;

        // 替换 steam_api.dll
        let source_dll = if is_64bit {
            if use_experimental {
                "gbe_fork/experimental/x64/steam_api64.dll"
            } else {
                "gbe_fork/regular/x64/steam_api64.dll"
            }
        } else if use_experimental {
            "gbe_fork/experimental/x32/steam_api.dll"
        } else {
            "gbe_fork/regular/x32/steam_api.dll"
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
        let original_client_path = game_dir.join(client_dll_name);

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
        let api_target_path = game_dir.join(api_dll_name);

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
    // 如果 steam_settings 已存在，先加载已有配置并保留

    // 写入 configs.main.ini
    let main_config_path = steam_settings_dir.join("configs.main.ini");
    let main_config = if main_config_path.exists() {
        let content = fs::read_to_string(&main_config_path)
            .await
            .map_err(|e| format!("读取已有 main 配置失败: {}", e))?;
        super::config_core::parse_main_ini(&content)?
    } else {
        MainConfig::default_config()
    };
    let main_config_content = main_config.to_ini();

    let mut main_file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    main_file.write_all(main_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    // 写入 configs.user.ini
    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config = if user_config_path.exists() {
        let content = fs::read_to_string(&user_config_path)
            .await
            .map_err(|e| format!("读取已有 user 配置失败: {}", e))?;
        super::config_core::parse_user_ini(&content)?
    } else {
        UserConfig::default_config()
    };
    let user_config_content = user_config.to_ini();

    let mut user_file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    user_file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    // 写入 configs.app.ini
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let app_config = if app_config_path.exists() {
        let content = fs::read_to_string(&app_config_path)
            .await
            .map_err(|e| format!("读取已有 app 配置失败: {}", e))?;
        super::config_core::parse_app_ini(&content)?
    } else {
        SteamAppConfig::default_config()
    };
    let app_config_content = app_config.to_ini();

    let mut app_file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    app_file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    // 写入 configs.overlay.ini
    let overlay_config_path = steam_settings_dir.join("configs.overlay.ini");
    let overlay_config = if overlay_config_path.exists() {
        let content = fs::read_to_string(&overlay_config_path)
            .await
            .map_err(|e| format!("读取已有 overlay 配置失败: {}", e))?;
        super::config_core::parse_overlay_ini(&content)?
    } else {
        OverlayConfig::default_config()
    };
    let overlay_config_content = overlay_config.to_ini();

    let mut overlay_file = fs::File::create(&overlay_config_path)
        .await
        .map_err(|e| format!("创建 configs.overlay.ini 失败: {}", e))?;
    overlay_file.write_all(overlay_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.overlay.ini 失败: {}", e))?;

    // 写入 custom_broadcasts.txt
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let broadcasts_content = "192.168.1.0/24\n192.168.0.0/24\n10.0.0.0/24\n";

    let mut broadcasts_file = fs::File::create(&broadcasts_path)
        .await
        .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
    broadcasts_file.write_all(broadcasts_content.as_bytes())
        .await
        .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;

    // 第6步: 高级模式额外复制 GameOverlayRenderer DLL
    if mode == 1 {
        let overlay_dll_name = if is_64bit { "GameOverlayRenderer64.dll" } else { "GameOverlayRenderer.dll" };
        let overlay_source = if is_64bit {
            "gbe_fork/steamclient_experimental/GameOverlayRenderer64.dll"
        } else {
            "gbe_fork/steamclient_experimental/GameOverlayRenderer.dll"
        };

        let overlay_source_path = Path::new(&resource_dir).join(overlay_source);
        let overlay_target_path = game_dir.join(overlay_dll_name);

        if overlay_source_path.exists() {
            if overlay_target_path.exists() {
                let overlay_backup_path = game_dir.join(format!("{}.bak", overlay_dll_name));
                if !overlay_backup_path.exists() {
                    fs::copy(&overlay_target_path, &overlay_backup_path)
                        .await
                        .map_err(|e| format!("备份 {} 失败: {}", overlay_dll_name, e))?;
                }
            }

            fs::copy(&overlay_source_path, &overlay_target_path)
                .await
                .map_err(|e| format!("复制 {} 失败: {}", overlay_dll_name, e))?;
        }
    }

    Ok(BasicConfigResult {
        success: true,
        message: "基础配置已应用".to_string(),
    })
}
