// 解压即玩图片命令
// 负责从 Gitee 仓库实时拉取解压即玩（夸克网盘二维码）图片，
// 下载到本地临时缓存供前端显示，并在程序退出时统一清理。

use crate::utils::config_path_utils;
use std::path::PathBuf;

/// Gitee 图片仓库配置（写死，复用程序更新仓库）
const GITEE_OWNER: &str = "Jingluohi";
const GITEE_REPO: &str = "steam-tool-plus";
/// Gitee 默认分支（程序更新走 Release，与分支文件互相独立）
const GITEE_BRANCH: &str = "master";

/// 解压即玩图片在仓库中的目录
const IMAGES_DIR: &str = "images/Quark_QR";
/// 图片文件扩展名
const IMAGE_EXT: &str = "png";

/// 本地临时缓存目录，相对 %APPDATA%/SteamToolPlus 的子目录
const CACHE_SUBDIR: &str = "cache/extract_play";

/// 单次图片查询结果
#[derive(serde::Serialize)]
pub struct ExtractPlayImageInfo {
    /// 该游戏是否已有解压即玩图片
    pub exists: bool,
    /// 本地缓存图片路径（exists 为 true 时提供）
    pub path: Option<String>,
}

/// 获取解压即玩图片本地临时缓存目录
/// 返回 %APPDATA%/SteamToolPlus/cache/extract_play/
fn get_cache_dir() -> Result<PathBuf, String> {
    Ok(config_path_utils::get_appdata_dir()?.join(CACHE_SUBDIR))
}

/// 获取指定游戏的图片本地缓存路径
fn get_cached_image_path(game_id: &str) -> Result<PathBuf, String> {
    Ok(get_cache_dir()?.join(format!("{}.{}", game_id, IMAGE_EXT)))
}

/// 解压即玩图片的远程原始地址
fn get_remote_image_url(game_id: &str) -> String {
    format!(
        "https://gitee.com/{}/{}/raw/{}/{}/{}.{}",
        GITEE_OWNER, GITEE_REPO, GITEE_BRANCH, IMAGES_DIR, game_id, IMAGE_EXT
    )
}

/// 下载文件到指定路径
/// 仅用于下载图片，若 HTTP 返回 404 表示仓库中尚未上传该图片
async fn download_image(url: &str, dest: &PathBuf) -> Result<bool, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("下载解压即玩图片失败: {}", e))?;

    // 404 表示仓库中还没有该游戏的图片
    if response.status().as_u16() == 404 {
        return Ok(false);
    }
    if !response.status().is_success() {
        return Err(format!(
            "下载解压即玩图片失败，HTTP 状态码: {}",
            response.status()
        ));
    }

    // 确保响应确实是图片（防止 Gitee 返回错误页被当作图片保存）
    let is_image = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.starts_with("image/"))
        .unwrap_or(false);

    if !is_image {
        return Err("下载的解压即玩图片内容不是有效图片".to_string());
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("读取图片数据失败: {}", e))?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建图片缓存目录失败: {}", e))?;
    }
    std::fs::write(dest, &bytes).map_err(|e| format!("写入图片缓存失败: {}", e))?;

    Ok(true)
}

/// 获取指定游戏的解压即玩图片
/// 优先复用本会话已下载的本地缓存；否则实时从 Gitee 下载。
/// 返回 exists=false 表示仓库中尚未上传该游戏图片（前端显示"暂未上传"）。
#[tauri::command]
pub async fn get_extract_play_image(game_id: String) -> Result<ExtractPlayImageInfo, String> {
    // 1. 若本会话已下载过，直接复用本地缓存，避免重复请求
    let cache_path = get_cached_image_path(&game_id)?;
    if cache_path.exists() {
        return Ok(ExtractPlayImageInfo {
            exists: true,
            path: Some(cache_path.to_string_lossy().to_string()),
        });
    }

    // 2. 实时从 Gitee 下载
    let url = get_remote_image_url(&game_id);
    match download_image(&url, &cache_path).await {
        Ok(true) => Ok(ExtractPlayImageInfo {
            exists: true,
            path: Some(cache_path.to_string_lossy().to_string()),
        }),
        Ok(false) => Ok(ExtractPlayImageInfo {
            exists: false,
            path: None,
        }),
        Err(e) => Err(e),
    }
}

/// 清理解压即玩图片本地临时缓存
/// 在程序退出时调用，删除 %APPDATA%/SteamToolPlus/cache/extract_play/ 目录。
pub fn clear_extract_play_cache() -> Result<(), String> {
    let cache_dir = get_cache_dir()?;
    if cache_dir.exists() {
        std::fs::remove_dir_all(&cache_dir)
            .map_err(|e| format!("清理解压即玩图片缓存失败: {}", e))?;
    }
    Ok(())
}
