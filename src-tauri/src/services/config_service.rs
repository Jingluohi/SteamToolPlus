// 配置服务
// 实现应用程序配置的管理

use crate::models::{AppConfig, UpdateConfigRequest, WindowConfig};
use crate::utils::file_utils;
use std::sync::Mutex;

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
    /// 配置文件路径
    config_path: String,
}

impl ConfigService {
    /// 创建新的配置服务实例
    pub fn new() -> Self {
        let config_path = "config/config.json".to_string();
        let config = Self::load_config(&config_path);

        Self {
            config: Mutex::new(config),
            config_path,
        }
    }

    /// 从文件加载配置
    fn load_config(path: &str) -> AppConfig {
        match file_utils::read_json_file::<AppConfig>(path) {
            Ok(config) => config,
            Err(_) => {
                // 文件不存在或读取失败，使用默认配置
                let default_config = AppConfig::default();
                // 尝试保存默认配置
                let _ = file_utils::write_json_file(path, &default_config);
                default_config
            }
        }
    }

    /// 保存配置到文件
    fn save_config_internal(&self, config: &AppConfig) -> Result<(), String> {
        file_utils::write_json_file(&self.config_path, config)
    }
}

use std::sync::Arc;

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
        if let Some(extension) = request.extension {
            config.extension = extension;
        }
        if let Some(security) = request.security {
            config.security = security;
        }

        let config_clone = config.clone();
        drop(config);

        // 保存到文件
        self.save_config_internal(&config_clone)?;

        Ok(config_clone)
    }

    /// 重置配置到默认值
    fn reset_config(&self) -> AppConfig {
        let default_config = AppConfig::default();
        let mut config = self.config.lock().unwrap();
        *config = default_config.clone();
        drop(config);

        // 保存默认配置
        let _ = self.save_config_internal(&default_config);

        default_config
    }

    /// 保存配置到文件
    fn save_config(&self) -> Result<(), String> {
        let config = self.config.lock().unwrap();
        self.save_config_internal(&config)
    }

    /// 更新窗口配置
    fn update_window_config(&self, window_config: WindowConfig) -> Result<(), String> {
        let mut config = self.config.lock().unwrap();
        config.window = window_config;
        let config_clone = config.clone();
        drop(config);

        self.save_config_internal(&config_clone)
    }
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}
