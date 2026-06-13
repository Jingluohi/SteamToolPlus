// 通用配置文件管理器
// 提供统一的配置文件保存/加载接口，消除 patch_commands 中大量重复的 save/load 函数模板
// 当前已用于 misc_config 模块，后续可扩展至 game_features、config_core 等模块

#![allow(dead_code)]

use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};

/// 配置文件管理器
/// 封装了 JSON/INI/Text 三种格式文件的 save/load 操作
/// 所有方法自动处理目录创建、路径拼接
pub struct ConfigFileManager {
    base_dir: PathBuf,
}

impl ConfigFileManager {
    /// 创建一个新的文件管理器，操作路径为 {game_path}/steam_settings/
    pub fn new(game_path: &str) -> Self {
        Self {
            base_dir: Path::new(game_path).join("steam_settings"),
        }
    }

    /// 带子目录的文件管理器
    pub fn with_subdir(game_path: &str, subdir: &str) -> Self {
        Self {
            base_dir: Path::new(game_path).join("steam_settings").join(subdir),
        }
    }

    /// 获取完整的文件路径（相对于 steam_settings）
    fn file_path(&self, filename: &str) -> PathBuf {
        self.base_dir.join(filename)
    }

    /// 确保 steam_settings 目录存在
    async fn ensure_dir(&self) -> Result<(), String> {
        tokio::fs::create_dir_all(&self.base_dir)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))
    }

    /// 保存 JSON 配置
    pub async fn save_json<T: Serialize>(
        &self,
        filename: &str,
        config: &T,
        success_msg: &str,
    ) -> Result<ConfigSaveResult, String> {
        use tokio::io::AsyncWriteExt;

        self.ensure_dir().await?;
        let path = self.file_path(filename);
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("序列化 {} 失败: {}", filename, e))?;

        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(|e| format!("创建 {} 失败: {}", filename, e))?;
        file.write_all(json.as_bytes())
            .await
            .map_err(|e| format!("写入 {} 失败: {}", filename, e))?;

        Ok(ConfigSaveResult {
            success: true,
            message: success_msg.to_string(),
        })
    }

    /// 加载 JSON 配置
    pub async fn load_json<T: DeserializeOwned>(
        &self,
        filename: &str,
    ) -> Result<ConfigLoadResult<T>, String> {
        let path = self.file_path(filename);
        if !path.exists() {
            return Ok(ConfigLoadResult {
                exists: false,
                config: None,
            });
        }

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| format!("读取 {} 失败: {}", filename, e))?;

        let config: T = serde_json::from_str(&content)
            .map_err(|e| format!("解析 {} 失败: {}", filename, e))?;

        Ok(ConfigLoadResult {
            exists: true,
            config: Some(config),
        })
    }

    /// 保存文本文件（每行一个条目）
    pub async fn save_text(
        &self,
        filename: &str,
        items: &[String],
        success_msg: &str,
    ) -> Result<ConfigSaveResult, String> {
        use tokio::io::AsyncWriteExt;

        self.ensure_dir().await?;
        let path = self.file_path(filename);
        let content = items.join("\n");

        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(|e| format!("创建 {} 失败: {}", filename, e))?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|e| format!("写入 {} 失败: {}", filename, e))?;

        Ok(ConfigSaveResult {
            success: true,
            message: success_msg.to_string(),
        })
    }

    /// 加载文本文件（每行一个条目）
    pub async fn load_text(&self, filename: &str) -> Result<Vec<String>, String> {
        let path = self.file_path(filename);
        if !path.exists() {
            return Ok(vec![]);
        }

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| format!("读取 {} 失败: {}", filename, e))?;

        Ok(content
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }

    /// 保存 INI 文件
    pub async fn save_ini(
        &self,
        filename: &str,
        content: &str,
        success_msg: &str,
    ) -> Result<ConfigSaveResult, String> {
        use tokio::io::AsyncWriteExt;

        self.ensure_dir().await?;
        let path = self.file_path(filename);

        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(|e| format!("创建 {} 失败: {}", filename, e))?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|e| format!("写入 {} 失败: {}", filename, e))?;

        Ok(ConfigSaveResult {
            success: true,
            message: success_msg.to_string(),
        })
    }

    /// 加载 INI 文件内容（原始字符串）
    pub async fn load_ini_raw(&self, filename: &str) -> Result<Option<String>, String> {
        let path = self.file_path(filename);
        if !path.exists() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| format!("读取 {} 失败: {}", filename, e))?;

        Ok(Some(content))
    }

    /// 保存二进制文件
    pub async fn save_binary(
        &self,
        relative_path: &str,
        data: &[u8],
        success_msg: &str,
    ) -> Result<ConfigSaveResult, String> {
        use tokio::io::AsyncWriteExt;

        let path = self.base_dir.join(relative_path);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }

        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(|e| format!("创建文件失败: {}", e))?;
        file.write_all(data)
            .await
            .map_err(|e| format!("写入文件失败: {}", e))?;

        Ok(ConfigSaveResult {
            success: true,
            message: success_msg.to_string(),
        })
    }

    /// 加载二进制文件
    pub async fn load_binary(&self, relative_path: &str) -> Result<Option<Vec<u8>>, String> {
        let path = self.base_dir.join(relative_path);
        if !path.exists() {
            return Ok(None);
        }

        let data = tokio::fs::read(&path)
            .await
            .map_err(|e| format!("读取文件失败: {}", e))?;

        Ok(Some(data))
    }

    /// 检查文件是否存在
    pub async fn file_exists(&self, relative_path: &str) -> bool {
        self.base_dir.join(relative_path).exists()
    }
}

// ============================================
// 辅助类型（用于 common.rs）
// ============================================

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