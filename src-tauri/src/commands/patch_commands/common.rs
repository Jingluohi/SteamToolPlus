// 公共类型定义和工具函数
// 包含所有模块共享的结果类型和基础命令

use crate::utils::resource_utils::get_resource_dir as get_resource_dir_util;
use std::collections::HashSet;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Emitter};

// ============================================
// 常量定义
// ============================================

/// 「未设置默认浏览器」错误前缀
/// 前后端共用同一错误码约定（与项目 MANIFEST_PATCH_ERROR_PREFIX 风格一致）：
/// Rust 侧 open_external_link 检测到系统未设默认浏览器时返回以此前缀开头的错误，
/// 前端据此识别后弹出程序内友好提示，而不是暴露 Windows 原生「找不到应用程序」弹窗。
const NO_DEFAULT_BROWSER_PREFIX: &str = "E_NO_DEFAULT_BROWSER";

/// Gitee 仓库配置（Readme 云端更新源，与解压即玩图片同一仓库）
const GITEE_OWNER: &str = "Jingluohi";
const GITEE_REPO: &str = "steam-tool-plus";
/// Gitee 默认分支
const GITEE_BRANCH: &str = "master";

/// Readme 在 Gitee 仓库中的目录
const README_REMOTE_DIR: &str = "Readme";

/// 已检查过云端更新的 Readme 文档集合（会话级去重）
/// key 格式: "{patch_type}_{game_id}"，同一文档本次运行内只检查一次
static CHECKED_README_KEYS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

/// 获取会话级去重集合
fn checked_readme_keys() -> &'static Mutex<HashSet<String>> {
    CHECKED_README_KEYS.get_or_init(|| Mutex::new(HashSet::new()))
}

/// 补丁类型对应的 Readme 子目录名
fn readme_subdir_name(patch_type: i32) -> &'static str {
    match patch_type {
        0 => "免_steam",
        1 => "局域网联机",
        2 => "steam联机",
        3 => "D_加密虚拟机",
        4 => "epic_联机",
        5 => "xbox_联机",
        _ => "免_steam",
    }
}

// ============================================
// 结果类型定义
// ============================================

/// 备份结果
#[derive(serde::Serialize)]
pub struct BackupResult {
    pub success: bool,
    pub message: String,
}

/// 脱壳结果
#[derive(serde::Serialize)]
pub struct UnpackResult {
    pub success: bool,
    pub message: String,
    pub unpacked_path: Option<String>,
}

/// 基础配置结果
#[derive(serde::Serialize)]
pub struct BasicConfigResult {
    pub success: bool,
    pub message: String,
}

/// 配置保存结果
#[derive(serde::Serialize)]
pub struct ConfigSaveResult {
    pub success: bool,
    pub message: String,
}

/// 配置加载结果（泛型）
#[derive(serde::Serialize)]
pub struct ConfigLoadResult<T> {
    pub exists: bool,
    pub config: Option<T>,
}

/// 导入结果（泛型）
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

/// 补丁应用结果
#[derive(serde::Serialize)]
pub struct ApplyPatchResult {
    pub success: bool,
    pub backed_up_files: Vec<String>,
    pub copied_files: Vec<String>,
    pub errors: Vec<String>,
}

/// 误把「清单文件包」当补丁应用时返回的错误码前缀
/// 前端据此识别并弹出专门的提示弹窗（提示去网盘下载真正的补丁）
const MANIFEST_PATCH_ERROR_PREFIX: &str = "E_MANIFEST_PATCH_DETECTED";

/// 补丁压缩包内容扫描深度限制
/// - dll/exe（补丁文件特征）：检索最多 5 层
/// - 清单文件特征（lua/vdf/manifest/txt）：检索最多 3 层（与下载/导入清单的扫描深度语义一致）
const MAX_PATCH_FILE_SCAN_DEPTH: u32 = 5;
const MAX_MANIFEST_FILE_SCAN_DEPTH: u32 = 3;

// ============================================
// 通用配置读写辅助函数
// ============================================

use crate::utils::file_utils;
use serde::{de::DeserializeOwned, Serialize};

/// 保存 steam_settings 目录下的 JSON 配置文件
pub async fn save_steam_settings_json<T: Serialize>(
    game_path: &str,
    file_name: &str,
    config: &T,
) -> Result<(), String> {
    let path = Path::new(game_path).join("steam_settings").join(file_name);
    file_utils::write_json_file_async(&path, config).await
}

/// 加载 steam_settings 目录下的 JSON 配置文件
pub async fn load_steam_settings_json<T: DeserializeOwned>(
    game_path: &str,
    file_name: &str,
) -> Result<Option<T>, String> {
    let path = Path::new(game_path).join("steam_settings").join(file_name);

    if !path.exists() {
        return Ok(None);
    }

    let config = file_utils::read_json_file_async(&path).await?;
    Ok(Some(config))
}

// ============================================
// 基础命令
// ============================================

/// 获取资源目录（IPC命令版本）
#[tauri::command]
pub fn get_resource_dir(app: AppHandle) -> Result<String, String> {
    let path = get_resource_dir_util(&app)?;
    Ok(path.to_string_lossy().to_string())
}

/// 规范化资源相对路径
/// 移除用户传入路径中可能包含的 "resources/" 或 "Resources\\" 前缀，使其统一为相对于资源目录的路径
pub fn normalize_resource_relative_path(path: &str) -> &str {
    let path_lower = path.to_lowercase();
    if path_lower.starts_with("resources/") {
        &path[10..]
    } else if path_lower.starts_with("resources\\") {
        &path[11..]
    } else {
        path
    }
}

/// 检查路径是否存在
#[tauri::command]
pub fn path_exists(path: String) -> Result<bool, String> {
    Ok(Path::new(&path).exists())
}

/// 获取补丁说明
/// 根据游戏ID和补丁类型读取对应的 Readme 文本
/// 同时后台检查 Gitee 云端是否有更新版本，有则下载替换本地文件并推送事件
#[tauri::command]
pub async fn get_patch_readme(
    app: AppHandle,
    game_id: String,
    patch_type: i32,
) -> Result<String, String> {
    let resource_dir = get_resource_dir(app.clone())?;

    let readme_subdir = readme_subdir_name(patch_type);

    let readme_path = Path::new(&resource_dir)
        .join("Readme")
        .join(readme_subdir)
        .join(&format!("{}.txt", game_id));

    // 会话级去重：同一文档本次运行内只检查一次云端更新
    let key = format!("{}_{}", patch_type, game_id);
    {
        let mut checked = checked_readme_keys()
            .lock()
            .map_err(|_| "检查锁获取失败".to_string())?;
        if !checked.contains(&key) {
            checked.insert(key);
            // 后台异步检查云端更新，不阻塞本次读取返回
            let app_clone = app.clone();
            let readme_path_clone = readme_path.clone();
            let game_id_clone = game_id.clone();
            tokio::spawn(async move {
                check_readme_remote_update(
                    app_clone,
                    game_id_clone,
                    patch_type,
                    readme_subdir,
                    readme_path_clone,
                )
                .await;
            });
        }
    }

    if !readme_path.exists() {
        return Ok(String::new());
    }

    match tokio::fs::read_to_string(&readme_path).await {
        Ok(content) => Ok(content),
        Err(_) => Ok(String::new()),
    }
}

/// 后台检查 Readme 云端更新并替换本地文件
/// 流程：Gitee raw 下载同路径 txt → 与本地内容对比 → 不同则覆盖写回原位 → 推送事件通知前端刷新
/// 任何失败（404/网络错误/写入失败）都静默忽略，保持本地内容不变
async fn check_readme_remote_update(
    app: AppHandle,
    game_id: String,
    patch_type: i32,
    readme_subdir: &str,
    local_path: std::path::PathBuf,
) {
    // 构建 Gitee raw 下载地址（中文路径由 url 库自动百分号编码）
    let url = format!(
        "https://gitee.com/{}/{}/raw/{}/{}/{}/{}.txt",
        GITEE_OWNER, GITEE_REPO, GITEE_BRANCH, README_REMOTE_DIR, readme_subdir, game_id
    );

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(_) => return, // 静默失败
    };

    // 下载远程文本
    let remote_text = match client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().as_u16() == 404 {
                return; // 云端无该文档，静默
            }
            if !resp.status().is_success() {
                return; // 其它非成功状态，静默
            }
            match resp.text().await {
                Ok(text) => text,
                Err(_) => return, // 读取响应失败，静默
            }
        }
        Err(_) => return, // 网络失败，静默
    };

    // 读取本地现有内容，与远程对比（本地不存在时视为空内容）
    let local_text = match tokio::fs::read_to_string(&local_path).await {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    // 内容一致则无需替换；不一致（或本地原本为空）则覆盖写回原位文件
    if remote_text == local_text {
        return;
    }

    if tokio::fs::write(&local_path, &remote_text).await.is_err() {
        return; // 写入失败，忽略本次更新，保持本地旧内容
    }

    // 推送事件通知前端刷新显示（前端静默刷新，不展示任何提示）
    let _ = app.emit(
        "readme-updated",
        serde_json::json!({ "patch_type": patch_type, "game_id": game_id }),
    );
}

// ============================================
// 通用命令
// ============================================

/// 关闭应用程序
#[tauri::command]
pub fn close_application() -> Result<(), String> {
    std::process::exit(0);
}

/// 打开外部链接
/// 使用 CREATE_NO_WINDOW 标志防止闪烁终端窗口
///
/// 打开前先检测系统默认浏览器（仅对 http/https 链接）：
/// 未设置默认浏览器时不调用 ShellExecute —— 因为系统未设默认浏览器时，
/// Windows 会自行弹出「标题为 URL、内容为找不到应用程序」的原生对话框，
/// 该弹窗由系统进程弹出，程序无法拦截或关闭，只能在调用之前拦截。
/// 检测失败时返回带 NO_DEFAULT_BROWSER_PREFIX 前缀的错误，由前端识别后弹程序内友好提示。
#[tauri::command]
pub fn open_external_link(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // 仅对 Web 链接做默认浏览器检测（mailto:/ms-settings: 等协议不走默认浏览器）
        if is_web_url(&url) && !has_default_browser() {
            return Err(format!(
                "{NO_DEFAULT_BROWSER_PREFIX} 系统未设置默认浏览器，请先到系统设置中设置"
            ));
        }

        use std::os::windows::process::CommandExt;
        use std::process::Command;

        const CREATE_NO_WINDOW: u32 = 0x08000000;

        Command::new("cmd")
            .args(["/c", "start", "", &url])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("打开链接失败: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开链接失败: {}", e))?;
    }

    Ok(())
}

/// 判断是否为 Web 链接（http/https 协议）
/// scheme 按 RFC 区分大小写不敏感，故统一转小写再比较
#[cfg(target_os = "windows")]
fn is_web_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

/// 检测系统是否已设置可用的默认浏览器（仅 Windows）
///
/// 检测原理：用 AssocQueryStringW 查询 https/http 协议的「打开」命令
/// （即默认浏览器在处理该协议时实际执行的命令行），
/// 再从命令行解析出可执行文件路径并校验其真实存在。
/// 只有「命令能查到」且「exe 真实存在」才视为已设置默认浏览器。
#[cfg(target_os = "windows")]
fn has_default_browser() -> bool {
    /// 从 AssocQueryString 返回的打开命令行中解析出可执行文件路径，并校验其真实存在
    /// 命令行形如 `"C:\...\msedge.exe" --profile-directory="Default" "%1"`：
    /// 若以引号开头，取第一段引号内的内容；否则取第一个空白前的内容
    fn command_exe_exists(command: &str) -> bool {
        let cmd = command.trim();
        let exe = if let Some(rest) = cmd.strip_prefix('"') {
            rest.split('"').next().map(String::from)
        } else {
            cmd.split_whitespace().next().map(String::from)
        };
        exe.map(|p| Path::new(&p).is_file()).unwrap_or(false)
    }

    // https 与 http 任一协议存在「指向真实可执行文件」的默认打开命令，即视为已设置默认浏览器
    ["https", "http"].iter().any(|proto| {
        assoc_query_command(proto)
            .map(|cmd| command_exe_exists(&cmd))
            .unwrap_or(false)
    })
}

/// 用 AssocQueryStringW 查询指定协议（如 https）的默认「打开」命令
/// ASSOCF_VERIFY 标志会让 API 校验关联的可执行文件真实存在（关联残留但 exe 已删除时会失败）
#[cfg(target_os = "windows")]
fn assoc_query_command(protocol: &str) -> Option<String> {
    use windows::core::{PCWSTR, PWSTR};
    use windows::Win32::UI::Shell::{AssocQueryStringW, ASSOCF_VERIFY, ASSOCSTR_COMMAND};

    // 输入参数需要以 NUL 结尾的宽字符串（PCWSTR）
    let proto_wide: Vec<u16> = protocol.encode_utf16().chain(std::iter::once(0)).collect();
    let open_wide: Vec<u16> = "open".encode_utf16().chain(std::iter::once(0)).collect();

    // 固定缓冲区足以容纳浏览器命令行（进程路径 + 参数）；仅用于判断存在性，截断不影响结果
    let mut buf = [0u16; 1024];
    let mut size = buf.len() as u32;

    unsafe {
        let hr = AssocQueryStringW(
            ASSOCF_VERIFY,
            ASSOCSTR_COMMAND,
            PCWSTR(proto_wide.as_ptr()),
            PCWSTR(open_wide.as_ptr()),
            Some(PWSTR(buf.as_mut_ptr())),
            &mut size,
        );

        if hr.is_ok() {
            // pcchout 输出为写入的字符数（含结尾 NUL），裁剪掉结尾 NUL 后还原为 String
            let mut len = (size as usize).min(buf.len());
            if len > 0 && buf[len - 1] == 0 {
                len -= 1;
            }
            let text = String::from_utf16_lossy(&buf[..len]).trim().to_string();
            return Some(text);
        }
    }

    None
}

/// 写入文本文件
#[tauri::command]
pub async fn write_text_file(path: String, content: String) -> Result<(), String> {
    use tokio::io::AsyncWriteExt;

    // 确保父目录存在
    if let Some(parent) = Path::new(&path).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let mut file = tokio::fs::File::create(&path)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;

    file.write_all(content.as_bytes())
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

/// 打开虚拟化环境配置教程视频
/// 使用系统默认应用打开 resources/D_加密虚拟化（虚拟机）环境搭建教程.mp4
#[tauri::command]
pub async fn open_virtualization_tutorial(_app: AppHandle) -> Result<(), String> {
    use std::process::Command;

    // 获取程序根目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();

    // 构建视频文件路径
    let video_path = exe_dir.join("resources").join("D_加密虚拟化（虚拟机）环境搭建教程.mp4");

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

// ============================================
// 补丁应用命令
// ============================================

/// 应用补丁 - 解压7z文件并复制到游戏目录
#[tauri::command]
pub async fn apply_patch(
    app: AppHandle,
    patch_source_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    let resource_dir = get_resource_dir_util(&app)?;

    let patch_relative_path = normalize_resource_relative_path(&patch_source_path);

    let patch_file_name = if patch_relative_path.ends_with(".7z") {
        patch_relative_path.to_string()
    } else {
        format!("{}.7z", patch_relative_path)
    };

    let patch_file_path = Path::new(&resource_dir).join(&patch_file_name);

    if !patch_file_path.exists() {
        return Err(format!("补丁源路径不存在: {}", patch_file_path.display()));
    }

    apply_patch_internal(&patch_file_path, &game_path).await
}

/// 从用户选择的压缩包文件应用补丁
#[tauri::command]
pub async fn apply_patch_from_file(
    archive_path: String,
    game_path: String,
) -> Result<ApplyPatchResult, String> {
    let archive_path = Path::new(&archive_path);

    if !archive_path.exists() {
        return Err(format!("补丁文件不存在: {}", archive_path.display()));
    }

    apply_patch_internal(archive_path, &game_path).await
}

/// 应用补丁的内部通用实现
/// 解压 7z 压缩包到临时目录，再递归复制到游戏目录，并备份已存在的文件
async fn apply_patch_internal(
    archive_path: &Path,
    game_path: &str,
) -> Result<ApplyPatchResult, String> {
    let target_path = Path::new(game_path);

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

    match extract_7z(archive_path, &temp_path).await {
        Ok(_) => {}
        Err(e) => {
            let _ = tokio::fs::remove_dir_all(&temp_path).await;
            return Err(format!("解压补丁失败: {}", e));
        }
    }

    // ===== 解压后、复制前：检测是否把「清单文件包」误当补丁应用 =====
    // 优先看 dll/exe：只要包含补丁文件特征，一律按补丁处理（即使误把清单文件打包进补丁也不误拦）
    let mut has_patch_file = false;
    let mut has_manifest_file = false;
    if let Err(e) = scan_patch_archive_contents(&temp_path, 0, &mut has_patch_file, &mut has_manifest_file)
    {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
        return Err(e);
    }
    // 没有 dll/exe 却包含清单文件特征 → 判定为清单文件包，拒绝应用，不写入任何文件
    if !has_patch_file && has_manifest_file {
        let _ = tokio::fs::remove_dir_all(&temp_path).await;
        return Err(format!(
            "{}|该压缩包内包含 Steam 清单文件（.lua/.vdf/.manifest 等），看起来是「清单文件包」而不是游戏补丁，已取消应用，未写入任何文件。请到补丁页面点击网盘下载按钮，下载真正的补丁后再来应用。",
            MANIFEST_PATCH_ERROR_PREFIX
        ));
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

/// 使用 zesven 解压7z文件
async fn extract_7z(
    archive_path: &Path,
    output_dir: &Path,
) -> Result<(), String> {
    use tokio::time::{timeout, Duration};

    let archive_path = archive_path.to_path_buf();
    let output_dir = output_dir.to_path_buf();

    let result = timeout(
        Duration::from_secs(300),
        async move {
            tokio::task::spawn_blocking(move || {
                let mut archive = zesven::Archive::open_path(&archive_path)
                    .map_err(|e| format!("打开7z补丁文件失败: {}", e))?;
                archive
                    .extract(&output_dir, (), &zesven::ExtractOptions::default())
                    .map_err(|e| format!("解压7z补丁文件失败: {}", e))
            })
            .await
            .map_err(|e| format!("解压任务异常: {}", e))?
        }
    ).await;

    match result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("解压操作超时（超过5分钟）".to_string()),
    }
}

/// 扫描补丁压缩包解压后的内容
/// 一次性递归遍历，收集两类信号（深度计数方式与现有清单扫描一致，根目录为第0层）：
/// - `has_patch_file`：是否包含 .dll/.exe（补丁文件特征，检索最多 5 层）
/// - `has_manifest_file`：是否包含 .lua/.vdf/.manifest/.txt（清单文件包特征，检索最多 3 层）
/// 只要存在 dll/exe 就按补丁处理；没有 dll/exe 但存在清单文件特征时，判定为误把清单当补丁。
fn scan_patch_archive_contents(
    dir: &Path,
    current_depth: u32,
    has_patch_file: &mut bool,
    has_manifest_file: &mut bool,
) -> Result<(), String> {
    // 达到补丁文件特征的最大检索深度（5层）则停止继续下钻
    if current_depth >= MAX_PATCH_FILE_SCAN_DEPTH {
        return Ok(());
    }

    let entries = std::fs::read_dir(dir).map_err(|e| format!("读取解压目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // 递归调用时深度+1
            scan_patch_archive_contents(&path, current_depth + 1, has_patch_file, has_manifest_file)?;
        } else if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();

            match ext.as_str() {
                // dll/exe：补丁文件特征，5 层内任意深度都算补丁
                "dll" | "exe" => *has_patch_file = true,
                // lua/vdf/manifest/txt：清单文件包特征，仅统计 3 层以内（可能附带说明 txt）
                "lua" | "vdf" | "manifest" | "txt"
                    if current_depth < MAX_MANIFEST_FILE_SCAN_DEPTH =>
                {
                    *has_manifest_file = true;
                }
                _ => {}
            }
        }
    }

    Ok(())
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

/// 备份文件（自动递增 .bak 后缀）
async fn backup_file(file_path: &Path) -> Result<std::path::PathBuf, String> {
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
