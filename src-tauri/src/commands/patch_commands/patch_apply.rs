// 补丁应用模块
// 应用 Steam 补丁基础配置：复制 DLL、生成 steam_settings、写入配置文件等

use crate::models::steam_config::*;
use std::path::Path;
use tauri::AppHandle;

// ============================================
// 补丁资源路径常量
// 所有相对路径均基于 resources 目录，禁止在业务代码中硬编码路径
// ============================================

/// gbe_fork 资源目录名
const GBE_FORK_DIR: &str = "gbe_fork";
/// 示例配置目录名
const STEAM_SETTINGS_EXAMPLE_DIR: &str = "steam_settings.EXAMPLE";
/// 实验版 steam_api 目录
const EXPERIMENTAL_DIR: &str = "experimental";
/// 稳定版 steam_api 目录
const REGULAR_DIR: &str = "regular";
/// 64 位子目录
const X64_DIR: &str = "x64";
/// 32 位子目录
const X32_DIR: &str = "x32";
/// steamclient 实验版目录
const STEAMCLIENT_EXPERIMENTAL_DIR: &str = "steamclient_experimental";

/// 标准模式 64 位 steam_api64.dll 资源路径
fn regular_x64_steam_api_dll() -> String {
    format!("{}/{}/{}/steam_api64.dll", GBE_FORK_DIR, REGULAR_DIR, X64_DIR)
}

/// 标准模式 32 位 steam_api.dll 资源路径
fn regular_x32_steam_api_dll() -> String {
    format!("{}/{}/{}/steam_api.dll", GBE_FORK_DIR, REGULAR_DIR, X32_DIR)
}

/// 实验版 64 位 steam_api64.dll 资源路径
fn experimental_x64_steam_api_dll() -> String {
    format!("{}/{}/{}/steam_api64.dll", GBE_FORK_DIR, EXPERIMENTAL_DIR, X64_DIR)
}

/// 实验版 32 位 steam_api.dll 资源路径
fn experimental_x32_steam_api_dll() -> String {
    format!("{}/{}/{}/steam_api.dll", GBE_FORK_DIR, EXPERIMENTAL_DIR, X32_DIR)
}

/// 高级模式 64 位 steamclient64.dll 资源路径
fn steamclient_experimental_x64_dll() -> String {
    format!("{}/{}/steamclient64.dll", GBE_FORK_DIR, STEAMCLIENT_EXPERIMENTAL_DIR)
}

/// 高级模式 32 位 steamclient.dll 资源路径
fn steamclient_experimental_x32_dll() -> String {
    format!("{}/{}/steamclient.dll", GBE_FORK_DIR, STEAMCLIENT_EXPERIMENTAL_DIR)
}

/// 高级模式 64 位 GameOverlayRenderer64.dll 资源路径
fn game_overlay_renderer_x64_dll() -> String {
    format!("{}/{}/GameOverlayRenderer64.dll", GBE_FORK_DIR, STEAMCLIENT_EXPERIMENTAL_DIR)
}

/// 高级模式 32 位 GameOverlayRenderer.dll 资源路径
fn game_overlay_renderer_x32_dll() -> String {
    format!("{}/{}/GameOverlayRenderer.dll", GBE_FORK_DIR, STEAMCLIENT_EXPERIMENTAL_DIR)
}

/// 高级模式 64 位 ColdClientLoader 启动器资源路径
fn steamclient_loader_x64_exe() -> String {
    format!("{}/{}/steamclient_loader_x64.exe", GBE_FORK_DIR, STEAMCLIENT_EXPERIMENTAL_DIR)
}

/// 高级模式 32 位 ColdClientLoader 启动器资源路径
fn steamclient_loader_x32_exe() -> String {
    format!("{}/{}/steamclient_loader_x32.exe", GBE_FORK_DIR, STEAMCLIENT_EXPERIMENTAL_DIR)
}

/// 根据架构获取 ColdClientLoader 启动器资源路径
fn get_steamclient_loader_exe_path(is_64bit: bool) -> String {
    if is_64bit {
        steamclient_loader_x64_exe()
    } else {
        steamclient_loader_x32_exe()
    }
}

/// 根据架构获取标准模式 steam_api DLL 资源路径
fn get_standard_mode_api_dll_path(is_64bit: bool, use_experimental: bool) -> String {
    if is_64bit {
        if use_experimental {
            experimental_x64_steam_api_dll()
        } else {
            regular_x64_steam_api_dll()
        }
    } else if use_experimental {
        experimental_x32_steam_api_dll()
    } else {
        regular_x32_steam_api_dll()
    }
}


/// 应用基础配置时写入 custom_broadcasts.txt
const DEFAULT_CUSTOM_BROADCASTS: &str = "192.168.1.0/24\n192.168.0.0/24\n10.0.0.0/24\n";

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
/// 3. 根据模式（标准/高级）处理 DLL 文件
///    - 标准模式：替换 steam_api.dll
///    - 高级模式：保留原 steam_api.dll，通过 ColdClientLoader 注入 steamclient.dll
/// 4. 生成 steam_interfaces.txt（标准模式下）
/// 5. 写入 steam_appid.txt 到双路径
/// 6. 写入基础配置文件（main、user、app、overlay）
/// 7. 写入 custom_broadcasts.txt
/// 8. 高级模式复制 ColdClientLoader 相关文件并生成 ColdClientLoader.ini
#[tauri::command]
pub async fn apply_steam_patch_basic(
    app: AppHandle,
    game_path: String,
    _game_id: String,
    steam_app_id: String,
    use_experimental: bool,
    emulator_mode: Option<i32>,
    game_exe_path: Option<String>,
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
    let example_dir = Path::new(&resource_dir).join(GBE_FORK_DIR).join(STEAM_SETTINGS_EXAMPLE_DIR);

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
    // 优先根据实际存在的 steam_api/steamclient DLL 判断，避免文件名包含 "64" 导致误判
    let has_api_64 = game_dir.join("steam_api64.dll").exists();
    let has_api_32 = game_dir.join("steam_api.dll").exists();
    let has_client_64 = game_dir.join("steamclient64.dll").exists();
    let has_client_32 = game_dir.join("steamclient.dll").exists();

    let is_64bit = if has_api_64 || has_client_64 {
        true
    } else if has_api_32 || has_client_32 {
        false
    } else {
        // 两种 DLL 都不存在时，兜底检查目录名中是否包含 x64/64 字样
        game_dir
            .read_dir()
            .map_err(|e| format!("读取游戏目录失败: {}", e))?
            .filter_map(|e| e.ok())
            .any(|e| {
                e.file_name()
                    .to_str()
                    .map(|name| name.contains("x64") || name == "64")
                    .unwrap_or(false)
            })
    };

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
        let source_dll = get_standard_mode_api_dll_path(is_64bit, use_experimental);
        let source_path = Path::new(&resource_dir).join(&source_dll);
        if source_path.exists() {
            fs::copy(&source_path, &original_api_path)
                .await
                .map_err(|e| format!("复制 DLL 失败: {}", e))?;
        } else {
            return Err(format!("源 DLL 文件不存在: {}", source_path.display()));
        }
    } else {
        // 高级模式: 保留原 steam_api.dll，通过 ColdClientLoader 注入 steamclient.dll
        let exe_path = game_exe_path.ok_or(
            "高级模式需要指定游戏主程序路径（.exe），请在选择游戏文件夹后，再选择游戏主程序。".to_string(),
        )?;

        setup_coldclient_loader(&resource_dir, game_dir, &steam_app_id, &exe_path, is_64bit).await?;
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
    // 注意：基础配置应用时不应保留示例目录中的示例值，始终使用程序默认值生成
    let main_config_path = steam_settings_dir.join("configs.main.ini");
    let main_config = MainConfig::default_config();
    let main_config_content = main_config.to_ini();

    let mut main_file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    main_file.write_all(main_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    // 写入 configs.user.ini
    // 基础配置应用时使用程序默认值，避免示例目录中的示例值被保留
    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config = UserConfig::default_config();
    let user_config_content = user_config.to_ini();

    let mut user_file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    user_file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    // 写入 configs.app.ini
    // 基础配置应用时使用程序默认值，确保 DLC 默认解锁全部且不带示例值
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let app_config = SteamAppConfig::default_config();
    let app_config_content = app_config.to_ini();

    let mut app_file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    app_file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    // 写入 configs.overlay.ini
    // 基础配置应用时使用程序默认值，避免示例目录中的示例值被保留
    // 高级模式（ColdClientLoader）已复制并注入 GameOverlayRenderer，默认启用 Overlay
    let overlay_config_path = steam_settings_dir.join("configs.overlay.ini");
    let mut overlay_config = OverlayConfig::default_config();
    if mode == 1 {
        overlay_config.enable_experimental_overlay = true;
    }
    let overlay_config_content = overlay_config.to_ini();

    let mut overlay_file = fs::File::create(&overlay_config_path)
        .await
        .map_err(|e| format!("创建 configs.overlay.ini 失败: {}", e))?;
    overlay_file.write_all(overlay_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.overlay.ini 失败: {}", e))?;

    // 写入 custom_broadcasts.txt
    let broadcasts_path = steam_settings_dir.join("custom_broadcasts.txt");
    let mut broadcasts_file = fs::File::create(&broadcasts_path)
        .await
        .map_err(|e| format!("创建 custom_broadcasts.txt 失败: {}", e))?;
    broadcasts_file.write_all(DEFAULT_CUSTOM_BROADCASTS.as_bytes())
        .await
        .map_err(|e| format!("写入 custom_broadcasts.txt 失败: {}", e))?;

    Ok(BasicConfigResult {
        success: true,
        message: "基础配置已应用".to_string(),
    })
}

// ============================================
// ColdClientLoader 高级模式 setup
// 保留原 steam_api.dll，复制 steamclient/overlay/loader 并生成 ColdClientLoader.ini
// ============================================

/// 高级模式：设置 ColdClientLoader
/// 1. 校验原 steam_api.dll 存在（这是该模式的前提）
/// 2. 复制 steamclient.dll / steamclient64.dll（推荐同时保留两种架构）
/// 3. 复制 GameOverlayRenderer.dll / GameOverlayRenderer64.dll
/// 4. 复制对应架构的 steamclient_loader.exe
/// 5. 生成 ColdClientLoader.ini
async fn setup_coldclient_loader(
    resource_dir: &str,
    game_dir: &Path,
    steam_app_id: &str,
    game_exe_path: &str,
    is_64bit: bool,
) -> Result<(), String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let api_dll_name = if is_64bit { "steam_api64.dll" } else { "steam_api.dll" };
    let original_api_path = game_dir.join(api_dll_name);

    if !original_api_path.exists() {
        return Err(format!(
            "未找到 {}！\n\n高级模式（ColdClientLoader）需要游戏原本就包含 steam_api(64).dll，\n请确认选择的游戏目录正确，或尝试「标准模式」。",
            api_dll_name
        ));
    }

    // 需要复制到游戏目录的文件列表（源相对路径 -> 目标文件名）
    let files_to_copy: Vec<(String, String)> = vec![
        (steamclient_experimental_x32_dll(), "steamclient.dll".to_string()),
        (steamclient_experimental_x64_dll(), "steamclient64.dll".to_string()),
        (game_overlay_renderer_x32_dll(), "GameOverlayRenderer.dll".to_string()),
        (game_overlay_renderer_x64_dll(), "GameOverlayRenderer64.dll".to_string()),
    ];

    for (source_relative, target_name) in files_to_copy {
        let source_path = Path::new(resource_dir).join(&source_relative);
        let target_path = game_dir.join(&target_name);

        if !source_path.exists() {
            return Err(format!("源文件不存在: {}", source_path.display()));
        }

        // 若目标已存在则先备份（如游戏自带 steamclient）
        if target_path.exists() {
            let backup_path = game_dir.join(format!("{}.bak", target_name));
            if !backup_path.exists() {
                fs::copy(&target_path, &backup_path)
                    .await
                    .map_err(|e| format!("备份 {} 失败: {}", target_name, e))?;
            }
        }

        fs::copy(&source_path, &target_path)
            .await
            .map_err(|e| format!("复制 {} 失败: {}", target_name, e))?;
    }

    // 复制对应架构的 loader，统一重命名为 steamclient_loader.exe
    let loader_source_relative = get_steamclient_loader_exe_path(is_64bit);
    let loader_source_path = Path::new(resource_dir).join(&loader_source_relative);
    let loader_target_path = game_dir.join("steamclient_loader.exe");

    if !loader_source_path.exists() {
        return Err(format!("源文件不存在: {}", loader_source_path.display()));
    }

    if loader_target_path.exists() {
        let loader_backup_path = game_dir.join("steamclient_loader.exe.bak");
        if !loader_backup_path.exists() {
            fs::copy(&loader_target_path, &loader_backup_path)
                .await
                .map_err(|e| format!("备份 steamclient_loader.exe 失败: {}", e))?;
        }
    }

    fs::copy(&loader_source_path, &loader_target_path)
        .await
        .map_err(|e| format!("复制 steamclient_loader.exe 失败: {}", e))?;

    // 生成 ColdClientLoader.ini
    let exe_relative = compute_relative_path(game_dir, Path::new(game_exe_path));
    let coldclient_config = ColdClientLoaderConfig {
        enabled: true,
        exe: exe_relative.unwrap_or_else(|_| game_exe_path.to_string()),
        exe_run_dir: String::new(),
        exe_command_line: String::new(),
        app_id: steam_app_id.to_string(),
        steam_client_dll: "steamclient.dll".to_string(),
        steam_client64_dll: "steamclient64.dll".to_string(),
        force_inject_steam_client: true,
        force_inject_game_overlay_renderer: true,
        dlls_to_inject_folder: String::new(),
        ignore_injection_error: true,
        ignore_loader_arch_difference: false,
        persistence_mode: 0,
        resume_by_debugger: false,
    };

    let config_path = game_dir.join("ColdClientLoader.ini");
    let config_content = coldclient_config.to_ini();

    let mut file = fs::File::create(&config_path)
        .await
        .map_err(|e| format!("创建 ColdClientLoader.ini 失败: {}", e))?;
    file.write_all(config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 ColdClientLoader.ini 失败: {}", e))?;

    Ok(())
}

/// 计算 target 相对于 base 的相对路径
/// 若无法计算（例如不在同一盘符）则返回错误，调用方应回退到绝对路径
fn compute_relative_path(base: &Path, target: &Path) -> Result<String, String> {
    let base = base
        .canonicalize()
        .map_err(|e| format!("canonicalize base 失败: {}", e))?;
    let target = target
        .canonicalize()
        .map_err(|e| format!("canonicalize target 失败: {}", e))?;

    target
        .strip_prefix(&base)
        .map(|p| p.to_string_lossy().to_string().replace('\\', "/"))
        .map_err(|_| "target 不在 base 目录下".to_string())
}

// ============================================
// 还原游戏文件
// 删除补丁生成的文件/目录，并将 .bak 备份复原
// ============================================

/// 补丁可能新增或替换的文件名列表
/// 用于在不存在对应 .bak 时清理残留文件
const PATCH_GENERATED_FILES: &[&str] = &[
    "steam_api.dll",
    "steam_api64.dll",
    "steamclient.dll",
    "steamclient64.dll",
    "GameOverlayRenderer.dll",
    "GameOverlayRenderer64.dll",
    "steamclient_loader.exe",
    "steamclient_loader_x32.exe",
    "steamclient_loader_x64.exe",
    "ColdClientLoader.ini",
    "steam_appid.txt",
];

/// 还原结果
#[derive(serde::Serialize)]
pub struct RestoreFilesResult {
    pub success: bool,
    pub restored_files: Vec<String>,
    pub removed_files: Vec<String>,
    pub removed_directories: Vec<String>,
    pub skipped_files: Vec<String>,
    pub message: String,
}

/// 还原游戏文件
/// 1. 删除 steam_settings 目录
/// 2. 删除 steam_appid.txt
/// 3. 删除补丁新增且无对应 .bak 的文件
/// 4. 将 .bak 备份文件去掉 .bak 后缀复原
#[tauri::command]
pub async fn restore_game_files(game_path: String) -> Result<RestoreFilesResult, String> {
    use tokio::fs;

    let game_dir = Path::new(&game_path);
    if !game_dir.exists() {
        return Err(format!("游戏路径不存在: {}", game_path));
    }

    let mut restored_files: Vec<String> = Vec::new();
    let mut removed_files: Vec<String> = Vec::new();
    let mut removed_directories: Vec<String> = Vec::new();
    let mut skipped_files: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // 1. 删除 steam_settings 目录
    let steam_settings_dir = game_dir.join("steam_settings");
    if steam_settings_dir.exists() {
        match fs::remove_dir_all(&steam_settings_dir).await {
            Ok(_) => removed_directories.push("steam_settings".to_string()),
            Err(e) => errors.push(format!("删除 steam_settings 失败: {}", e)),
        }
    }

    // 2. 删除补丁新增且无对应 .bak 的文件
    for file_name in PATCH_GENERATED_FILES {
        let file_path = game_dir.join(file_name);
        let backup_path = game_dir.join(format!("{}.bak", file_name));

        // 如果存在对应的 .bak，则后续统一处理，这里跳过
        if backup_path.exists() {
            continue;
        }

        if file_path.exists() {
            match fs::remove_file(&file_path).await {
                Ok(_) => removed_files.push(file_name.to_string()),
                Err(e) => errors.push(format!("删除 {} 失败: {}", file_name, e)),
            }
        }
    }

    // 3. 扫描并恢复所有 .bak 文件
    let entries = match fs::read_dir(game_dir).await {
        Ok(entries) => entries,
        Err(e) => return Err(format!("读取游戏目录失败: {}", e)),
    };

    let mut entries_stream = entries;
    while let Ok(Some(entry)) = entries_stream.next_entry().await {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // 只处理 .bak 结尾的文件
        if let Some(original_name) = file_name_str.strip_suffix(".bak") {
            let original_path = game_dir.join(original_name);

            // 如果原始文件存在，先删除原始文件
            if original_path.exists() {
                if let Err(e) = fs::remove_file(&original_path).await {
                    errors.push(format!("删除原文件 {} 失败: {}", original_name, e));
                    skipped_files.push(file_name_str.to_string());
                    continue;
                }
            }

            // 将 .bak 重命名为原始文件名
            match fs::rename(&path, &original_path).await {
                Ok(_) => restored_files.push(original_name.to_string()),
                Err(e) => {
                    errors.push(format!("恢复 {} 失败: {}", file_name_str, e));
                    skipped_files.push(file_name_str.to_string());
                }
            }
        }
    }

    // 构建结果消息
    let message = if errors.is_empty() {
        "游戏文件已还原".to_string()
    } else {
        format!("还原完成，但出现 {} 个错误: {}", errors.len(), errors.join("; "))
    };

    Ok(RestoreFilesResult {
        success: errors.is_empty(),
        restored_files,
        removed_files,
        removed_directories,
        skipped_files,
        message,
    })
}
