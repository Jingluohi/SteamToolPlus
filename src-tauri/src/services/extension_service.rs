// 扩展服务
// 实现扩展系统的业务逻辑

use crate::models::{
    Extension, ExtensionListResponse, ExtensionLoadStatus, ExtensionManifest,
    InstallExtensionRequest,
};
use crate::services::seven_z_service::{SevenZService, SevenZServiceTrait};
use crate::utils::file_utils;
use std::sync::{Arc, Mutex};

/// 扩展服务接口
pub trait ExtensionServiceTrait: Send + Sync {
    /// 获取所有扩展
    fn get_extensions(&self) -> ExtensionListResponse;
    /// 安装扩展
    fn install_extension(&self, request: InstallExtensionRequest) -> Result<Extension, String>;
    /// 卸载扩展
    fn uninstall_extension(&self, id: &str) -> Result<(), String>;
    /// 启用/禁用扩展
    fn toggle_extension(&self, id: &str, enabled: bool) -> Result<Extension, String>;
    /// 重新加载扩展
    fn reload_extension(&self, id: &str) -> Result<Extension, String>;
    /// 获取扩展清单
    fn get_manifest(&self, id: &str) -> Option<ExtensionManifest>;
}

/// 扩展服务实现
pub struct ExtensionService {
    /// 扩展列表
    extensions: Mutex<Vec<Extension>>,
    /// 扩展目录路径
    extensions_path: String,
}

impl ExtensionService {
    /// 创建新的扩展服务实例
    pub fn new() -> Self {
        let extensions_path = "extensions".to_string();
        let extensions = Self::load_extensions(&extensions_path);

        Self {
            extensions: Mutex::new(extensions),
            extensions_path,
        }
    }

    /// 加载所有扩展
    fn load_extensions(path: &str) -> Vec<Extension> {
        let mut extensions = Vec::new();

        let ext_dir = std::path::Path::new(path);
        if !ext_dir.exists() {
            // 创建扩展目录
            let _ = std::fs::create_dir_all(ext_dir);
            return extensions;
        }

        // 扫描扩展目录
        let entries = match std::fs::read_dir(ext_dir) {
            Ok(entries) => entries,
            Err(_) => return extensions,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // 尝试读取manifest.json
                let manifest_path = path.join("manifest.json");
                let manifest_path_str = manifest_path.to_string_lossy().to_string();
                if let Ok(manifest) = file_utils::read_json_file::<ExtensionManifest>(&manifest_path_str) {
                    let extension = Extension {
                        manifest,
                        path: path.to_string_lossy().to_string(),
                        enabled: false,
                        install_time: chrono::Local::now().to_rfc3339(),
                        update_time: chrono::Local::now().to_rfc3339(),
                        load_status: ExtensionLoadStatus::Unloaded,
                        error: None,
                    };
                    extensions.push(extension);
                }
            }
        }

        extensions
    }

    /// 解压7z扩展包
    fn extract_extension_package(&self, package_path: &str) -> Result<String, String> {
        let service = SevenZService::new();
        let file_name = std::path::Path::new(package_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("无效的文件名")?;

        let extract_path = format!("{}/{}", self.extensions_path, file_name);
        service.extract(package_path, &extract_path)?;

        Ok(extract_path)
    }
}

impl ExtensionServiceTrait for ExtensionService {
    /// 获取所有扩展
    fn get_extensions(&self) -> ExtensionListResponse {
        let extensions = self.extensions.lock().unwrap();
        let total = extensions.len();
        ExtensionListResponse {
            extensions: extensions.clone(),
            total,
        }
    }

    /// 安装扩展
    fn install_extension(&self, request: InstallExtensionRequest) -> Result<Extension, String> {
        // 验证文件是否存在
        if !std::path::Path::new(&request.package_path).exists() {
            return Err("扩展包文件不存在".to_string());
        }

        // 解压扩展包
        let extract_path = self.extract_extension_package(&request.package_path)?;

        // 读取manifest.json
        let manifest_path = format!("{}/manifest.json", extract_path);
        let manifest = file_utils::read_json_file::<ExtensionManifest>(&manifest_path)?;

        // 检查扩展是否已存在
        let mut extensions = self.extensions.lock().unwrap();
        if extensions.iter().any(|e| e.manifest.id == manifest.id) {
            return Err("扩展已存在".to_string());
        }

        // 创建扩展实例
        let extension = Extension {
            manifest,
            path: extract_path,
            enabled: false,
            install_time: chrono::Local::now().to_rfc3339(),
            update_time: chrono::Local::now().to_rfc3339(),
            load_status: ExtensionLoadStatus::Unloaded,
            error: None,
        };

        extensions.push(extension.clone());

        Ok(extension)
    }

    /// 卸载扩展
    fn uninstall_extension(&self, id: &str) -> Result<(), String> {
        let mut extensions = self.extensions.lock().unwrap();
        let pos = extensions
            .iter()
            .position(|e| e.manifest.id == id)
            .ok_or("扩展不存在")?;

        let extension = &extensions[pos];

        // 删除扩展目录
        let ext_path = std::path::Path::new(&extension.path);
        if ext_path.exists() {
            std::fs::remove_dir_all(ext_path).map_err(|e| e.to_string())?;
        }

        extensions.remove(pos);

        Ok(())
    }

    /// 启用/禁用扩展
    fn toggle_extension(&self, id: &str, enabled: bool) -> Result<Extension, String> {
        let mut extensions = self.extensions.lock().unwrap();
        let extension = extensions
            .iter_mut()
            .find(|e| e.manifest.id == id)
            .ok_or("扩展不存在")?;

        extension.enabled = enabled;
        extension.load_status = if enabled {
            ExtensionLoadStatus::Loaded
        } else {
            ExtensionLoadStatus::Disabled
        };
        extension.update_time = chrono::Local::now().to_rfc3339();

        Ok(extension.clone())
    }

    /// 重新加载扩展
    fn reload_extension(&self, id: &str) -> Result<Extension, String> {
        let mut extensions = self.extensions.lock().unwrap();
        let extension = extensions
            .iter_mut()
            .find(|e| e.manifest.id == id)
            .ok_or("扩展不存在")?;

        extension.load_status = ExtensionLoadStatus::Loading;

        // 重新读取manifest
        let manifest_path = format!("{}/manifest.json", extension.path);
        match file_utils::read_json_file::<ExtensionManifest>(&manifest_path) {
            Ok(manifest) => {
                extension.manifest = manifest;
                extension.load_status = ExtensionLoadStatus::Loaded;
                extension.error = None;
            }
            Err(e) => {
                extension.load_status = ExtensionLoadStatus::Failed;
                extension.error = Some(e);
            }
        }

        extension.update_time = chrono::Local::now().to_rfc3339();

        Ok(extension.clone())
    }

    /// 获取扩展清单
    fn get_manifest(&self, id: &str) -> Option<ExtensionManifest> {
        let extensions = self.extensions.lock().unwrap();
        extensions
            .iter()
            .find(|e| e.manifest.id == id)
            .map(|e| e.manifest.clone())
    }
}

impl Default for ExtensionService {
    fn default() -> Self {
        Self::new()
    }
}
