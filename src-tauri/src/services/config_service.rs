// 配置服务
// 实现应用程序配置的管理

use crate::models::{AppConfig, UpdateConfigRequest, WindowConfig};
use crate::utils::config_path_utils;
use crate::utils::file_utils;
use std::sync::Mutex;

/// 配置文件名
const CONFIG_FILENAME: &str = "config.json";

/// 配置服务接口
pub trait ConfigServiceTrait: Send + Sync {
    /// 获取当前配置
    fn get_config(&self) -> AppConfig;
    /// 更新配置
    fn update_config(&self, request: UpdateConfigRequest) -> Result<AppConfig, String>;
    /// 重置配置到默认值
    fn reset_config(&self) -> AppConfig;
    /// 保存配置到文件
    fn save_config(&self) -> Result<(), String>;
    /// 更新窗口配置
    fn update_window_config(&self, config: WindowConfig) -> Result<(), String>;
}

/// 配置服务实现
pub struct ConfigService {
    /// 配置数据
    config: Mutex<AppConfig>,
}

impl ConfigService {
    /// 创建新的配置服务实例
    pub fn new() -> Self {
        // 尝试从备份恢复配置
        let _ = config_path_utils::restore_from_backup(CONFIG_FILENAME);

        let config = Self::load_config();

        Self {
            config: Mutex::new(config),
        }
    }

    /// 获取运行时配置文件路径
    fn get_config_path() -> Result<String, String> {
        let path = config_path_utils::get_runtime_config_path(CONFIG_FILENAME)?;
        Ok(path.to_string_lossy().to_string())
    }

    /// 从文件加载配置
    fn load_config() -> AppConfig {
        // 尝试获取运行时配置路径
        let config_path = match Self::get_config_path() {
            Ok(path) => path,
            Err(_) => return AppConfig::default(),
        };

        match file_utils::read_json_file::<AppConfig>(&config_path) {
            Ok(config) => config,
            Err(_) => {
                // 文件不存在或读取失败，使用默认配置
                let default_config = AppConfig::default();
                // 尝试保存默认配置
                let _ = Self::save_config_internal(&default_config);
                default_config
            }
        }
    }

    /// 保存配置到文件（内部方法）
    fn save_config_internal(config: &AppConfig) -> Result<(), String> {
        let config_path = Self::get_config_path()?;

        // 确保运行时配置目录存在
        config_path_utils::ensure_runtime_config_dir()?;

        // 保存到运行时目录
        file_utils::write_json_file(&config_path, config)?;

        // 同步到备份目录
        config_path_utils::sync_to_backup(CONFIG_FILENAME)?;

        Ok(())
    }
}

impl ConfigServiceTrait for ConfigService {
    /// 获取当前配置
    fn get_config(&self) -> AppConfig {
        let config = self.config.lock().unwrap();
        config.clone()
    }

    /// 更新配置
    fn update_config(&self, request: UpdateConfigRequest) -> Result<AppConfig, String> {
        let mut config = self.config.lock().unwrap();

        // 更新各个配置项
        if let Some(window) = request.window {
            config.window = window;
        }
        if let Some(theme) = request.theme {
            config.theme = theme;
        }
        if let Some(game_dirs) = request.game_dirs {
            config.game_dirs = game_dirs;
        }
        if let Some(launch) = request.launch {
            config.launch = launch;
        }

        let config_clone = config.clone();
        drop(config);

        // 保存到文件
        Self::save_config_internal(&config_clone)?;

        Ok(config_clone)
    }

    /// 重置配置到默认值
    fn reset_config(&self) -> AppConfig {
        let default_config = AppConfig::default();
        let mut config = self.config.lock().unwrap();
        *config = default_config.clone();
        drop(config);

        // 保存默认配置
        let _ = Self::save_config_internal(&default_config);

        default_config
    }

    /// 保存配置到文件
    fn save_config(&self) -> Result<(), String> {
        let config = self.config.lock().unwrap();
        Self::save_config_internal(&config)
    }

    /// 更新窗口配置
    fn update_window_config(&self, window_config: WindowConfig) -> Result<(), String> {
        let mut config = self.config.lock().unwrap();
        config.window = window_config;
        let config_clone = config.clone();
        drop(config);

        Self::save_config_internal(&config_clone)
    }
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}
