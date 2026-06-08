// 工具集成模块
// 处理 ColdClientLoader、lobby_connect 配置以及 generate_interfaces 工具调用

use crate::models::steam_config::*;
use std::path::Path;

// ============================================
// generate_interfaces 工具函数
// ============================================

/// 运行 generate_interfaces 工具生成 steam_interfaces.txt
/// 该函数在 apply_steam_patch_basic 中被调用，用于从原 steam_api.dll 中提取接口列表
pub(crate) async fn generate_interfaces(
    resource_dir: &str,
    original_api_path: &Path,
    game_dir: &Path,
    steam_settings_dir: &Path,
) -> Result<(), String> {
    use std::process::Command;

    // 判断使用 32 位还是 64 位工具
    let is_64bit = original_api_path
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase().contains("64"))
        .unwrap_or(false);

    let tool_name = if is_64bit {
        "tools/generate_interfaces/generate_interfaces_x64.exe"
    } else {
        "tools/generate_interfaces/generate_interfaces_x32.exe"
    };
    let tool_path = Path::new(resource_dir).join("gbe_fork").join(tool_name);

    if tool_path.exists() {
        let tool_path_clone = tool_path.clone();
        let original_api_path_clone = original_api_path.to_path_buf();
        let game_dir_clone = game_dir.to_path_buf();
        let steam_settings_dir_clone = steam_settings_dir.to_path_buf();

        let result = tokio::task::spawn_blocking(move || {
            #[cfg(target_os = "windows")]
            use std::os::windows::process::CommandExt;
            #[cfg(target_os = "windows")]
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            let mut cmd = Command::new(&tool_path_clone);
            #[cfg(target_os = "windows")]
            cmd.creation_flags(CREATE_NO_WINDOW);
            cmd.arg(&original_api_path_clone)
                .current_dir(&game_dir_clone)
                .output()
        }).await.map_err(|e| format!("运行 generate_interfaces 失败: {}", e))?;

        if let Ok(output) = result {
            if output.status.success() {
                let generated_txt = game_dir.join("steam_interfaces.txt");
                if generated_txt.exists() {
                    let target_txt = steam_settings_dir_clone.join("steam_interfaces.txt");
                    tokio::fs::rename(&generated_txt, &target_txt)
                        .await
                        .map_err(|e| format!("移动 steam_interfaces.txt 失败: {}", e))?;
                }
            }
        }
    }

    Ok(())
}

// ============================================
// ColdClientLoader 配置
// ============================================

/// 保存 ColdClientLoader 配置
#[tauri::command]
pub async fn save_coldclient_config(
    game_path: String,
    config: ColdClientLoaderConfig,
) -> Result<super::common::ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    use super::common::ConfigSaveResult;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let config_path = steam_settings_dir.join("coldclientloader.ini");
    let config_content = config.to_ini();

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 coldclientloader.ini 失败: {}", e))?;
    file.write_all(config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 coldclientloader.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "ColdClientLoader 配置已保存".to_string(),
    })
}

/// 加载 ColdClientLoader 配置
#[tauri::command]
pub async fn load_coldclient_config(
    game_path: String,
) -> Result<super::common::ConfigLoadResult<ColdClientLoaderConfig>, String> {
    use tokio::fs;

    use super::common::ConfigLoadResult;

    let config_path = Path::new(&game_path).join("steam_settings").join("coldclientloader.ini");

    if !config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取 coldclientloader.ini 失败: {}", e))?;

    let config = parse_coldclient_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析 ColdClientLoader INI 内容
fn parse_coldclient_ini(content: &str) -> Result<ColdClientLoaderConfig, String> {
    let mut config = ColdClientLoaderConfig::default_config();
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
                "loader" => {
                    match key {
                        "enabled" => config.enabled = value == "1",
                        "injection_mode" => config.injection_mode = value.to_string(),
                        "launch_args" => config.launch_args = value.to_string(),
                        "exe_path" => config.exe_path = Some(value.to_string()),
                        "working_dir" => config.working_dir = Some(value.to_string()),
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
// lobby_connect 配置
// ============================================

/// 保存 lobby_connect 配置
#[tauri::command]
pub async fn save_lobby_connect_config(
    game_path: String,
    config: LobbyConnectConfig,
) -> Result<super::common::ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    use super::common::ConfigSaveResult;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let config_path = steam_settings_dir.join("lobby_connect.ini");
    let config_content = config.to_ini();

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 lobby_connect.ini 失败: {}", e))?;
    file.write_all(config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 lobby_connect.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "lobby_connect 配置已保存".to_string(),
    })
}

/// 加载 lobby_connect 配置
#[tauri::command]
pub async fn load_lobby_connect_config(
    game_path: String,
) -> Result<super::common::ConfigLoadResult<LobbyConnectConfig>, String> {
    use tokio::fs;

    use super::common::ConfigLoadResult;

    let config_path = Path::new(&game_path).join("steam_settings").join("lobby_connect.ini");

    if !config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&config_path)
        .await
        .map_err(|e| format!("读取 lobby_connect.ini 失败: {}", e))?;

    let config = parse_lobby_connect_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析 lobby_connect INI 内容
fn parse_lobby_connect_ini(content: &str) -> Result<LobbyConnectConfig, String> {
    let mut config = LobbyConnectConfig::default_config();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "enabled" => config.enabled = value == "1",
                "auto_join" => config.auto_join = value == "1",
                "target_lobby_id" => config.target_lobby_id = value.to_string(),
                "password" => config.password = value.to_string(),
                "auto_reconnect" => config.auto_reconnect = value == "1",
                "reconnect_interval" => config.reconnect_interval = value.parse().unwrap_or(5),
                _ => {}
            }
        }
    }

    Ok(config)
}
