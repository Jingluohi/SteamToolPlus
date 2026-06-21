// 文件操作模块
// 包含目录递归复制、示例文件重命名、游戏EXE备份等功能

use std::path::Path;

// ============================================
// 文件操作函数
// ============================================

/// 递归复制目录（异步版本）
/// 将源目录中的所有文件和子目录复制到目标目录
pub(crate) async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
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
/// 支持三种格式:
/// 1. name.EXAMPLE -> name (文件夹或没有扩展名的文件)
/// 2. name.EXAMPLE.ext -> name.ext (有扩展名的文件，如 configs.main.EXAMPLE.ini)
/// 3. name_EXAMPLE.ext -> name.ext (下划线分隔的格式，如 configs.main_EXAMPLE.ini)
pub(crate) async fn rename_example_files(dir: &Path) -> Result<(), String> {
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

        // 格式1: name.EXAMPLE (以 .EXAMPLE 结尾)
        if let Some(new_name) = file_name_str.strip_suffix(".EXAMPLE") {
            let new_path = dir.join(new_name);
            entries_to_rename.push((path.clone(), new_path));
        }
        // 格式2: name.EXAMPLE.ext (中间有 .EXAMPLE.)
        else if let Some(pos) = file_name_str.find(".EXAMPLE.") {
            let new_name = format!("{}{}", &file_name_str[..pos], &file_name_str[pos + 8..]);
            let new_path = dir.join(&new_name);
            entries_to_rename.push((path.clone(), new_path));
        }
        // 格式3: name_EXAMPLE.ext (中间有 _EXAMPLE.)
        else if let Some(pos) = file_name_str.find("_EXAMPLE.") {
            let new_name = format!("{}{}", &file_name_str[..pos], &file_name_str[pos + 8..]);
            let new_path = dir.join(&new_name);
            entries_to_rename.push((path.clone(), new_path));
        }
        else if path.is_dir() {
            sub_dirs.push(path);
        }
    }

    // 递归处理子目录
    for sub_dir in sub_dirs {
        Box::pin(rename_example_files(&sub_dir)).await?;
    }

    // 执行重命名操作
    for (old_path, new_path) in entries_to_rename {
        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| format!("重命名失败: {} -> {}: {}", old_path.display(), new_path.display(), e))?;
    }

    Ok(())
}

/// 备份游戏exe文件
/// 将 .exe 文件备份为 .exe.bak，如果备份已存在则跳过
#[tauri::command]
pub fn backup_game_exe(exe_path: String) -> Result<super::common::BackupResult, String> {
    use std::fs;

    use super::common::BackupResult;

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
