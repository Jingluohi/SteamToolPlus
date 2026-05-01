// 图片服务
// 提供优化的图片加载和缓存功能，降低GPU消耗

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// 图片缓存 - 使用LRU策略限制内存占用
/// 缓存图片的文件路径而不是base64数据，避免内存占用过高
static IMAGE_CACHE: Lazy<Mutex<HashMap<String, ImageCacheEntry>>> = Lazy::new(|| {
    Mutex::new(HashMap::with_capacity(100))
});

/// 最大缓存条目数
const MAX_CACHE_SIZE: usize = 50;

/// 图片缓存条目
#[derive(Clone)]
struct ImageCacheEntry {
    /// 图片文件路径
    pub path: PathBuf,
    /// 图片格式 (jpg, png, webp)
    pub format: String,
    /// 最后访问时间戳（用于LRU淘汰）
    pub last_accessed: std::time::Instant,
}

/// 图片加载结果
pub struct ImageLoadResult {
    /// 图片文件路径
    pub path: PathBuf,
    /// MIME类型
    pub mime_type: String,
    /// 是否需要缩略图
    pub needs_thumbnail: bool,
}

/// 查找游戏封面图片
/// 优先返回文件路径而不是base64数据，让前端使用文件路径加载
/// 这样可以利用浏览器的图片缓存机制，减少GPU内存占用
///
/// # 参数
/// * `cover_dir` - 封面图片目录
/// * `game_id` - 游戏ID
///
/// # 返回值
/// * `Ok(Some(ImageLoadResult))` - 找到图片
/// * `Ok(None)` - 图片不存在
/// * `Err(String)` - 错误信息
pub fn find_game_cover(cover_dir: &Path, game_id: &str) -> Result<Option<ImageLoadResult>, String> {
    // 验证游戏ID
    if game_id.is_empty() || !game_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("无效的游戏ID".to_string());
    }

    // 构建缓存键
    let cache_key = format!("{}", game_id);

    // 检查缓存
    {
        let mut cache = IMAGE_CACHE.lock().map_err(|e| format!("缓存锁定失败: {}", e))?;

        // 如果缓存已满，清理最旧的条目
        if cache.len() >= MAX_CACHE_SIZE {
            cleanup_cache(&mut cache);
        }

        // 检查缓存中是否存在
        if let Some(entry) = cache.get_mut(&cache_key) {
            // 更新访问时间
            entry.last_accessed = std::time::Instant::now();

            // 检查文件是否仍然存在
            if entry.path.exists() {
                return Ok(Some(ImageLoadResult {
                    path: entry.path.clone(),
                    mime_type: format!("image/{}", entry.format),
                    needs_thumbnail: false,
                }));
            } else {
                // 文件已删除，从缓存中移除
                cache.remove(&cache_key);
            }
        }
    }

    // 尝试不同格式的图片
    let formats = [("jpg", "jpeg"), ("png", "png"), ("webp", "webp")];

    for (ext, mime_ext) in &formats {
        let image_path = cover_dir.join(format!("{}.{}", game_id, ext));

        if image_path.exists() {
            // 添加到缓存
            let mut cache = IMAGE_CACHE.lock().map_err(|e| format!("缓存锁定失败: {}", e))?;
            cache.insert(
                cache_key,
                ImageCacheEntry {
                    path: image_path.clone(),
                    format: mime_ext.to_string(),
                    last_accessed: std::time::Instant::now(),
                },
            );

            return Ok(Some(ImageLoadResult {
                path: image_path,
                mime_type: format!("image/{}", mime_ext),
                needs_thumbnail: false,
            }));
        }
    }

    // 没有找到图片
    Ok(None)
}

/// 清理缓存 - 移除最旧的条目
fn cleanup_cache(cache: &mut HashMap<String, ImageCacheEntry>) {
    // 找到最旧的条目
    let oldest_key = cache
        .iter()
        .min_by_key(|(_, entry)| entry.last_accessed)
        .map(|(key, _)| key.clone());

    if let Some(key) = oldest_key {
        cache.remove(&key);
    }
}

/// 清除图片缓存
pub fn clear_image_cache() -> Result<(), String> {
    let mut cache = IMAGE_CACHE.lock().map_err(|e| format!("缓存锁定失败: {}", e))?;
    cache.clear();
    Ok(())
}

/// 获取缓存统计信息
pub fn get_cache_stats() -> Result<(usize, usize), String> {
    let cache = IMAGE_CACHE.lock().map_err(|e| format!("缓存锁定失败: {}", e))?;
    Ok((cache.len(), MAX_CACHE_SIZE))
}

/// 将图片转换为指定尺寸的缩略图（可选功能）
/// 如果图片尺寸过大，可以生成缩略图来减少GPU内存占用
///
/// # 参数
/// * `image_path` - 原图路径
/// * `max_width` - 最大宽度
/// * `max_height` - 最大高度
///
/// # 返回值
/// * `Ok(PathBuf)` - 缩略图路径（如果不需要缩略图则返回原图路径）
/// * `Err(String)` - 错误信息
#[allow(dead_code)]
pub async fn create_thumbnail_if_needed(
    image_path: &Path,
    max_width: u32,
    max_height: u32,
) -> Result<PathBuf, String> {
    // 注意：这里可以实现图片缩放逻辑
    // 由于添加 image 库会增加编译时间和体积，这里先返回原图路径
    // 如果需要缩略图功能，可以添加 image crate

    // 检查图片文件大小，如果超过阈值则建议生成缩略图
    let metadata = tokio::fs::metadata(image_path)
        .await
        .map_err(|e| format!("读取文件元数据失败: {}", e))?;

    let file_size = metadata.len();
    const MAX_FILE_SIZE: u64 = 1024 * 1024; // 1MB

    if file_size > MAX_FILE_SIZE {
        // 图片文件较大，建议生成缩略图
        // 这里可以实现缩略图生成逻辑
        // 暂时返回原图路径
    }

    Ok(image_path.to_path_buf())
}

/// 获取图片文件路径（转换为URL可用的格式）
/// 在Tauri中，可以使用 convertFileSrc 将本地文件路径转换为安全的URL
///
/// # 参数
/// * `image_path` - 图片文件路径
///
/// # 返回值
/// * `String` - 转换后的路径字符串
pub fn get_image_url_path(image_path: &Path) -> String {
    // 返回文件路径，前端使用 convertFileSrc 转换为安全URL
    image_path.to_string_lossy().to_string()
}
