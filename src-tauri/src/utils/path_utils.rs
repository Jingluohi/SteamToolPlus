// 路径工具
// 提供路径处理相关的通用函数

use std::path::Path;

/// 获取文件名（不含扩展名）
pub fn get_file_stem(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// 获取文件扩展名
pub fn get_file_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
}

/// 获取父目录路径
pub fn get_parent_dir(path: &str) -> Option<String> {
    Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
}

/// 拼接路径
pub fn join_paths(base: &str, relative: &str) -> String {
    Path::new(base)
        .join(relative)
        .to_string_lossy()
        .to_string()
}

/// 检查路径是否为绝对路径
pub fn is_absolute(path: &str) -> bool {
    Path::new(path).is_absolute()
}

/// 规范化路径
pub fn normalize_path(path: &str) -> String {
    Path::new(path)
        .canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string())
}

/// 获取相对路径
pub fn get_relative_path(base: &str, target: &str) -> Option<String> {
    let base_path = Path::new(base);
    let target_path = Path::new(target);

    target_path
        .strip_prefix(base_path)
        .ok()
        .map(|p| p.to_string_lossy().to_string())
}

/// 检查路径是否在指定目录下
pub fn is_path_in_dir(path: &str, dir: &str) -> bool {
    let path = Path::new(path);
    let dir = Path::new(dir);

    path.starts_with(dir)
}
