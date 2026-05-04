// 配置路径工具
// 管理配置文件的存储路径，支持运行时路径和备份路径

use std::path::PathBuf;

/// 获取程序所在目录
pub fn get_exe_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录".to_string())
        .map(|p| p.to_path_buf())
}

/// 获取 AppData 目录路径
pub fn get_appdata_dir() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA")
        .map_err(|_| "无法获取 APPDATA 环境变量".to_string())?;
    Ok(PathBuf::from(appdata).join("SteamToolPlus"))
}

/// 获取运行时配置目录（AppData）
pub fn get_runtime_config_dir() -> Result<PathBuf, String> {
    let appdata_dir = get_appdata_dir()?;
    Ok(appdata_dir.join("config"))
}

/// 获取备份配置目录（程序根目录）
pub fn get_backup_config_dir() -> Result<PathBuf, String> {
    let exe_dir = get_exe_dir()?;
    Ok(exe_dir.join("config"))
}

/// 获取运行时配置文件路径
pub fn get_runtime_config_path(filename: &str) -> Result<PathBuf, String> {
    let config_dir = get_runtime_config_dir()?;
    Ok(config_dir.join(filename))
}

/// 获取备份配置文件路径
pub fn get_backup_config_path(filename: &str) -> Result<PathBuf, String> {
    let config_dir = get_backup_config_dir()?;
    Ok(config_dir.join(filename))
}

/// 确保运行时配置目录存在
pub fn ensure_runtime_config_dir() -> Result<(), String> {
    let config_dir = get_runtime_config_dir()?;
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("创建运行时配置目录失败: {}", e))?;
    }
    Ok(())
}

/// 确保备份配置目录存在
pub fn ensure_backup_config_dir() -> Result<(), String> {
    let config_dir = get_backup_config_dir()?;
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("创建备份配置目录失败: {}", e))?;
    }
    Ok(())
}

/// 从备份恢复配置到运行时目录
/// 如果运行时配置不存在但备份存在，则复制备份到运行时目录
pub fn restore_from_backup(filename: &str) -> Result<bool, String> {
    let runtime_path = get_runtime_config_path(filename)?;
    let backup_path = get_backup_config_path(filename)?;

    // 如果运行时配置已存在，不需要恢复
    if runtime_path.exists() {
        return Ok(false);
    }

    // 如果备份存在，复制到运行时目录
    if backup_path.exists() {
        ensure_runtime_config_dir()?;
        std::fs::copy(&backup_path, &runtime_path)
            .map_err(|e| format!("从备份恢复配置失败: {}", e))?;
        return Ok(true);
    }

    Ok(false)
}

/// 同步配置到备份目录
/// 将运行时配置复制到备份目录
pub fn sync_to_backup(filename: &str) -> Result<(), String> {
    let runtime_path = get_runtime_config_path(filename)?;
    let backup_path = get_backup_config_path(filename)?;

    // 如果运行时配置不存在，不执行同步
    if !runtime_path.exists() {
        return Ok(());
    }

    ensure_backup_config_dir()?;
    std::fs::copy(&runtime_path, &backup_path)
        .map_err(|e| format!("同步配置到备份失败: {}", e))?;

    Ok(())
}
