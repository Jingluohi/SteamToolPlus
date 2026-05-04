use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use uuid::Uuid;
use crate::utils::config_path_utils;

/// 背景配置文件名
const BACKGROUND_CONFIG_FILENAME: &str = "background.json";

/// 背景显示模式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackgroundMode {
    Single,
    Slideshow,
    Random,
}

impl Default for BackgroundMode {
    fn default() -> Self {
        BackgroundMode::Single
    }
}

/// 切换动画效果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransitionEffect {
    Fade,
    Slide,
    Zoom,
    None,
}

impl Default for TransitionEffect {
    fn default() -> Self {
        TransitionEffect::Fade
    }
}

/// 页面类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PageType {
    Library,
    /// 兼容旧版 background.json 中的 `extension` 页面类型
    #[serde(alias = "extension")]
    Browse,
    Download,
    Patch,
    Settings,
    About,
}

/// 背景文件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundFile {
    pub id: String,
    pub filename: String,
    pub path: String,
    #[serde(rename = "addedTime")]
    pub added_time: String,
    pub enabled: bool,
    pub order: i32,
}

/// 页面背景配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageBackgroundConfig {
    pub page_type: PageType,
    pub page_name: String,
    pub enabled: bool,
    pub mode: BackgroundMode,
    pub current_file_id: Option<String>,
    /// 浅色模式背景文件ID列表
    #[serde(default)]
    pub light_file_ids: Vec<String>,
    /// 深色模式背景文件ID列表
    #[serde(default)]
    pub dark_file_ids: Vec<String>,
    pub interval: u64,
    pub transition_effect: TransitionEffect,
    /// 兼容旧版本的文件ID列表
    #[serde(default)]
    pub file_ids: Vec<String>,
    pub blur_strength: i32,
    pub darkness: f32,
}

impl Default for PageBackgroundConfig {
    fn default() -> Self {
        Self {
            page_type: PageType::Library,
            page_name: "游戏库".to_string(),
            enabled: true,
            mode: BackgroundMode::Single,
            current_file_id: None,
            light_file_ids: vec![],
            dark_file_ids: vec![],
            interval: 10000,
            transition_effect: TransitionEffect::Fade,
            file_ids: vec![],
            blur_strength: 0,
            darkness: 0.3,
        }
    }
}

/// 背景设置配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundConfig {
    pub files: Vec<BackgroundFile>,
    pub page_configs: Vec<PageBackgroundConfig>,
    pub default_config: PageBackgroundConfig,
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        Self {
            files: vec![],
            page_configs: vec![
                PageBackgroundConfig { page_type: PageType::Library, page_name: "游戏库".to_string(), ..Default::default() },
                PageBackgroundConfig { page_type: PageType::Browse, page_name: "浏览".to_string(), ..Default::default() },
                PageBackgroundConfig { page_type: PageType::Download, page_name: "下载".to_string(), ..Default::default() },
                PageBackgroundConfig { page_type: PageType::Patch, page_name: "补丁".to_string(), ..Default::default() },
                PageBackgroundConfig { page_type: PageType::Settings, page_name: "设置".to_string(), ..Default::default() },
                PageBackgroundConfig { page_type: PageType::About, page_name: "关于".to_string(), ..Default::default() },
            ],
            default_config: PageBackgroundConfig::default(),
        }
    }
}

/// 获取背景文件存储目录（%appdata%/SteamToolPlus/resources/pic/background）
fn get_background_dir() -> Result<PathBuf, String> {
    let appdata_dir = config_path_utils::get_appdata_dir()?;
    let background_dir = appdata_dir.join("resources").join("pic").join("background");
    
    // 确保目录存在
    if !background_dir.exists() {
        fs::create_dir_all(&background_dir)
            .map_err(|e| format!("创建背景目录失败: {}", e))?;
    }
    
    Ok(background_dir)
}

/// 获取程序根目录下的默认背景图片目录
fn get_default_background_dir() -> Result<PathBuf, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();
    Ok(exe_dir.join("resources").join("pic").join("background"))
}

/// 读取背景配置
#[tauri::command]
pub async fn get_background_config() -> Result<BackgroundConfig, String> {
    // 尝试从备份恢复
    let _ = config_path_utils::restore_from_backup(BACKGROUND_CONFIG_FILENAME);

    let config_path = config_path_utils::get_runtime_config_path(BACKGROUND_CONFIG_FILENAME)
        .map_err(|e| format!("获取配置路径失败: {}", e))?;

    // 如果配置文件不存在，尝试初始化默认背景
    if !config_path.exists() {
        return init_default_background().await;
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;

    let mut config: BackgroundConfig = serde_json::from_str(&content)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;
    
    // 更新文件路径为绝对路径（因为路径可能包含旧的位置）
    let background_dir = get_background_dir()?;
    for file in &mut config.files {
        // 只保留文件名，重新构建完整路径
        let filename = PathBuf::from(&file.path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| file.filename.clone());
        file.path = background_dir.join(&filename).to_string_lossy().to_string();
    }

    Ok(config)
}

/// 初始化默认背景
/// 当 background.json 不存在时，从程序根目录复制默认图片到 %appdata%，并生成配置
/// 深色模式使用 default.png，浅色模式使用 default2.png
async fn init_default_background() -> Result<BackgroundConfig, String> {
    let default_dir = get_default_background_dir()?;
    let background_dir = get_background_dir()?;
    
    // 创建默认配置
    let mut config = BackgroundConfig::default();
    
    // 定义深色和浅色模式的默认背景文件名
    let dark_default = "default.png";
    let light_default = "default2.png";
    
    // 支持的图片格式
    let _image_extensions = ["jpg", "jpeg", "png", "webp", "bmp", "gif"];
    
    // 如果默认目录存在，处理默认背景图片
    if default_dir.exists() {
        let mut dark_file_id: Option<String> = None;
        let mut light_file_id: Option<String> = None;
        let mut order = 0;
        
        // 首先处理深色模式默认背景 (default.png)
        let dark_path = default_dir.join(dark_default);
        if dark_path.exists() {
            let target_path = background_dir.join(dark_default);
            if let Err(e) = fs::copy(&dark_path, &target_path) {
                eprintln!("复制深色默认背景图片失败: {} - {}", dark_default, e);
            } else {
                let id = Uuid::new_v4().to_string();
                let file = BackgroundFile {
                    id: id.clone(),
                    filename: dark_default.to_string(),
                    path: target_path.to_string_lossy().to_string(),
                    added_time: chrono::Local::now().to_rfc3339(),
                    enabled: true,
                    order,
                };
                config.files.push(file);
                dark_file_id = Some(id);
                order += 1;
            }
        }
        
        // 然后处理浅色模式默认背景 (default2.png)
        let light_path = default_dir.join(light_default);
        if light_path.exists() {
            let target_path = background_dir.join(light_default);
            if let Err(e) = fs::copy(&light_path, &target_path) {
                eprintln!("复制浅色默认背景图片失败: {} - {}", light_default, e);
            } else {
                let id = Uuid::new_v4().to_string();
                let file = BackgroundFile {
                    id: id.clone(),
                    filename: light_default.to_string(),
                    path: target_path.to_string_lossy().to_string(),
                    added_time: chrono::Local::now().to_rfc3339(),
                    enabled: true,
                    order,
                };
                config.files.push(file);
                light_file_id = Some(id);
            }
        }
        
        // 为所有页面配置设置背景
        for page_config in &mut config.page_configs {
            page_config.enabled = true;
            
            // 设置深色模式背景
            if let Some(ref id) = dark_file_id {
                page_config.dark_file_ids.push(id.clone());
                page_config.file_ids.push(id.clone());
                if page_config.current_file_id.is_none() {
                    page_config.current_file_id = Some(id.clone());
                }
            }
            
            // 设置浅色模式背景
            if let Some(ref id) = light_file_id {
                page_config.light_file_ids.push(id.clone());
                // 如果深色背景不存在，也添加到通用file_ids
                if dark_file_id.is_none() {
                    page_config.file_ids.push(id.clone());
                }
                if page_config.current_file_id.is_none() {
                    page_config.current_file_id = Some(id.clone());
                }
            }
        }
        
        // 保存配置
        if !config.files.is_empty() {
            save_background_config(config.clone()).await?;
        }
    }

    Ok(config)
}

/// 保存背景配置
#[tauri::command]
pub async fn save_background_config(
    config: BackgroundConfig,
) -> Result<(), String> {
    // 确保运行时配置目录存在
    config_path_utils::ensure_runtime_config_dir()
        .map_err(|e| format!("确保配置目录存在失败: {}", e))?;

    let config_path = config_path_utils::get_runtime_config_path(BACKGROUND_CONFIG_FILENAME)
        .map_err(|e| format!("获取配置路径失败: {}", e))?;

    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;

    fs::write(&config_path, content)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    // 同步到备份目录
    config_path_utils::sync_to_backup(BACKGROUND_CONFIG_FILENAME)
        .map_err(|e| format!("同步到备份失败: {}", e))?;

    Ok(())
}

/// 添加背景文件（图片）
#[tauri::command]
pub async fn add_background_file(
    file_path: String,
) -> Result<BackgroundFile, String> {
    let background_dir = get_background_dir()?;

    // 确保目录存在
    if !background_dir.exists() {
        fs::create_dir_all(&background_dir)
            .map_err(|e| format!("创建背景目录失败: {}", e))?;
    }

    // 获取源文件信息
    let source_path = PathBuf::from(&file_path);
    let extension = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg")
        .to_lowercase();

    // 获取原始文件名（不含扩展名）
    let original_stem = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("background");

    // 生成唯一ID
    let id = Uuid::new_v4().to_string();

    // 保持原文件名，如果存在同名文件则添加序号
    let mut filename = format!("{}.{}", original_stem, extension);
    let mut target_path = background_dir.join(&filename);
    let mut counter = 1;

    // 检查文件名是否已存在，存在则添加序号
    while target_path.exists() {
        filename = format!("{}_{}.{}", original_stem, counter, extension);
        target_path = background_dir.join(&filename);
        counter += 1;
    }

    // 复制文件
    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("复制文件失败: {}", e))?;

    // 获取当前配置以确定排序
    let mut config = get_background_config().await.unwrap_or_default();
    let order = config.files.len() as i32;

    let file = BackgroundFile {
        id: id.clone(),
        filename: filename.clone(),
        path: target_path.to_string_lossy().to_string(),
        added_time: chrono::Local::now().to_rfc3339(),
        enabled: true,
        order,
    };

    // 添加到配置
    config.files.push(file.clone());

    save_background_config(config).await?;

    Ok(file)
}

/// 删除背景文件
#[tauri::command]
pub async fn remove_background_file(
    file_id: String,
) -> Result<(), String> {
    let mut config = get_background_config().await.unwrap_or_default();

    // 找到并删除文件
    if let Some(index) = config.files.iter().position(|f| f.id == file_id) {
        let file = config.files.remove(index);

        // 删除文件
        let _ = fs::remove_file(&file.path);

        // 更新排序
        for (i, f) in config.files.iter_mut().enumerate() {
            f.order = i as i32;
        }

        // 从所有页面配置中移除该文件
        for page_config in &mut config.page_configs {
            page_config.light_file_ids.retain(|id| id != &file_id);
            page_config.dark_file_ids.retain(|id| id != &file_id);
            page_config.file_ids.retain(|id| id != &file_id);
            // 更新 current_file_id
            let all_ids: Vec<String> = page_config.light_file_ids.iter()
                .chain(page_config.dark_file_ids.iter())
                .chain(page_config.file_ids.iter())
                .cloned()
                .collect();
            if page_config.current_file_id.as_ref() == Some(&file_id) {
                page_config.current_file_id = all_ids.first().cloned();
            }
        }

        save_background_config(config).await?;
    }

    Ok(())
}

/// 扫描背景文件目录，自动添加新文件
#[tauri::command]
pub async fn scan_background_files() -> Result<Vec<BackgroundFile>, String> {
    let background_dir = get_background_dir()?;

    // 确保目录存在
    if !background_dir.exists() {
        fs::create_dir_all(&background_dir)
            .map_err(|e| format!("创建背景目录失败: {}", e))?;
    }

    let mut config = get_background_config().await.unwrap_or_default();
    let mut new_files = Vec::new();

    // 支持的图片格式
    let image_extensions = ["jpg", "jpeg", "png", "webp", "bmp", "gif"];

    // 读取目录中的所有文件
    let entries = fs::read_dir(&background_dir)
        .map_err(|e| format!("读取背景目录失败: {}", e))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();

                    if image_extensions.contains(&ext.as_str()) {
                        let filename = path.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();

                        // 检查是否已存在
                        let exists = config.files.iter().any(|f| f.filename == filename);

                        if !exists {
                            let id = Uuid::new_v4().to_string();
                            let file = BackgroundFile {
                                id: id.clone(),
                                filename: filename.clone(),
                                path: path.to_string_lossy().to_string(),
                                added_time: chrono::Local::now().to_rfc3339(),
                                enabled: true,
                                order: config.files.len() as i32 + new_files.len() as i32,
                            };

                            new_files.push(file);
                        }
                    }
                }
            }
        }
    }

    // 添加新文件到配置
    if !new_files.is_empty() {
        config.files.extend(new_files.clone());
        save_background_config(config).await?;
    }

    Ok(new_files)
}

// 兼容旧版命令的别名
#[tauri::command]
pub async fn add_background_image(
    file_path: String,
) -> Result<BackgroundFile, String> {
    add_background_file(file_path).await
}

#[tauri::command]
pub async fn remove_background_image(
    file_id: String,
) -> Result<(), String> {
    remove_background_file(file_id).await
}

#[tauri::command]
pub async fn scan_background_images() -> Result<Vec<BackgroundFile>, String> {
    scan_background_files().await
}

/// 重置背景配置
/// 删除 background.json 配置文件，恢复到初始状态
#[tauri::command]
pub async fn reset_background_config() -> Result<(), String> {
    // 获取配置文件路径
    let config_path = config_path_utils::get_runtime_config_path(BACKGROUND_CONFIG_FILENAME)
        .map_err(|e| format!("获取配置路径失败: {}", e))?;
    
    // 如果配置文件存在，删除它
    if config_path.exists() {
        fs::remove_file(&config_path)
            .map_err(|e| format!("删除配置文件失败: {}", e))?;
    }
    
    // 同时删除备份文件
    let backup_path = config_path_utils::get_backup_config_path(BACKGROUND_CONFIG_FILENAME)
        .map_err(|e| format!("获取备份路径失败: {}", e))?;
    if backup_path.exists() {
        let _ = fs::remove_file(&backup_path);
    }
    
    Ok(())
}
