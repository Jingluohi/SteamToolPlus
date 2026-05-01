// 补丁服务
// 实现免Steam补丁注入的业务逻辑

use std::path::Path;
use tauri::AppHandle;

/// 备份结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackupResult {
    pub success: bool,
    pub message: String,
}

/// 注入结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct InjectResult {
    pub success: bool,
    pub message: String,
}

/// 注入选项
#[derive(Debug, Clone, serde::Deserialize)]
pub struct InjectOptions {
    pub auto_close: bool,
    pub wait_for_exit: bool,
    pub backup_original: bool,
}

/// 文件过滤器
#[derive(Debug, Clone, serde::Deserialize)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// 补丁服务接口
pub trait PatchServiceTrait: Send + Sync {
    /// 获取资源目录
    fn get_resource_dir(&self, app: &AppHandle) -> Result<String, String>;
    /// 检查路径是否存在
    fn path_exists(&self, path: &str) -> Result<bool, String>;
    /// 备份游戏exe文件
    fn backup_game_exe(&self, exe_path: &str) -> Result<BackupResult, String>;
    /// 注入补丁
    fn inject_patch(
        &self,
        game_exe_path: &str,
        patch_file_path: &str,
        options: InjectOptions,
    ) -> Result<InjectResult, String>;
    /// 关闭应用程序
    fn close_application(&self) -> Result<(), String>;
    /// 选择文件
    fn select_file(
        &self,
        title: &str,
        filters: Vec<FileFilter>,
    ) -> impl std::future::Future<Output = Result<Option<String>, String>> + Send;
    /// 选择文件夹
    fn select_folder(
        &self,
        title: &str,
    ) -> impl std::future::Future<Output = Result<Option<String>, String>> + Send;
    /// 打开外部链接
    fn open_external_link(&self, url: &str) -> Result<(), String>;
}

/// 补丁服务实现
pub struct PatchService;

impl PatchService {
    /// 创建新的补丁服务实例
    pub fn new() -> Self {
        Self
    }
}

impl PatchServiceTrait for PatchService {
    /// 获取资源目录路径
    fn get_resource_dir(&self, app: &AppHandle) -> Result<String, String> {
        let exe_dir = std::env::current_exe()
            .map_err(|e| format!("无法获取程序路径: {}", e))?
            .parent()
            .ok_or("无法获取程序所在目录")?
            .to_path_buf();

        let resources_path = exe_dir.join("resources");
        if resources_path.exists() && resources_path.is_dir() {
            return Ok(resources_path.to_string_lossy().to_string());
        }

        if let Some(target_dir) = exe_dir.parent() {
            let project_resources = target_dir.parent().unwrap_or(target_dir).join("resources");
            if project_resources.exists() && project_resources.is_dir() {
                return Ok(project_resources.to_string_lossy().to_string());
            }
        }

        Err("无法找到资源目录".to_string())
    }

    /// 检查路径是否存在
    fn path_exists(&self, path: &str) -> Result<bool, String> {
        let path = Path::new(path);
        Ok(path.exists())
    }

    /// 备份游戏exe文件
    fn backup_game_exe(&self, exe_path: &str) -> Result<BackupResult, String> {
        let path = Path::new(exe_path);

        if !path.exists() {
            return Ok(BackupResult {
                success: false,
                message: "游戏exe文件不存在".to_string(),
            });
        }

        // 创建备份文件路径
        let backup_path = path.with_extension("exe.backup");

        // 如果备份文件已存在，先删除
        if backup_path.exists() {
            std::fs::remove_file(&backup_path)
                .map_err(|e| format!("删除旧备份文件失败: {}", e))?;
        }

        // 复制文件
        std::fs::copy(path, &backup_path)
            .map_err(|e| format!("备份文件失败: {}", e))?;

        Ok(BackupResult {
            success: true,
            message: format!("备份已创建: {}", backup_path.display()),
        })
    }

    /// 注入补丁
    fn inject_patch(
        &self,
        game_exe_path: &str,
        patch_file_path: &str,
        _options: InjectOptions,
    ) -> Result<InjectResult, String> {
        let game_path = Path::new(game_exe_path);
        let patch_path = Path::new(patch_file_path);

        if !game_path.exists() {
            return Ok(InjectResult {
                success: false,
                message: "游戏exe文件不存在".to_string(),
            });
        }

        if !patch_path.exists() {
            return Ok(InjectResult {
                success: false,
                message: "补丁文件不存在".to_string(),
            });
        }

        // 获取游戏目录
        let game_dir = game_path
            .parent()
            .ok_or("无法获取游戏目录")?;

        // 获取补丁文件名
        let patch_file_name = patch_path
            .file_name()
            .ok_or("无法获取补丁文件名")?
            .to_string_lossy();

        // 目标补丁路径（复制到游戏目录）
        let target_patch_path = game_dir.join(&*patch_file_name);

        // 如果目标位置已存在同名文件，先删除
        if target_patch_path.exists() {
            std::fs::remove_file(&target_patch_path)
                .map_err(|e| format!("删除旧补丁文件失败: {}", e))?;
        }

        // 复制补丁文件到游戏目录
        std::fs::copy(patch_path, &target_patch_path)
            .map_err(|e| format!("复制补丁文件失败: {}", e))?;

        // 创建steam_api64.dll（模拟Steam API）
        // 实际实现中，这里应该使用gbe_fork的DLL注入逻辑
        let steam_api_path = game_dir.join("steam_api64.dll");
        if steam_api_path.exists() {
            // 备份原始的steam_api64.dll
            let steam_api_backup = game_dir.join("steam_api64.dll.backup");
            if !steam_api_backup.exists() {
                std::fs::copy(&steam_api_path, &steam_api_backup)
                    .map_err(|e| format!("备份steam_api64.dll失败: {}", e))?;
            }

            // 删除原始的steam_api64.dll
            std::fs::remove_file(&steam_api_path)
                .map_err(|e| format!("删除原始steam_api64.dll失败: {}", e))?;
        }

        // 将补丁文件重命名为steam_api64.dll
        std::fs::rename(&target_patch_path, &steam_api_path)
            .map_err(|e| format!("重命名补丁文件失败: {}", e))?;

        Ok(InjectResult {
            success: true,
            message: format!("补丁注入成功！补丁文件已复制到: {}", steam_api_path.display()),
        })
    }

    /// 关闭应用程序
    fn close_application(&self) -> Result<(), String> {
        std::process::exit(0);
    }

    /// 选择文件
    async fn select_file(
        &self,
        title: &str,
        filters: Vec<FileFilter>,
    ) -> Result<Option<String>, String> {
        use tauri_plugin_dialog::DialogExt;

        // 这里需要通过AppHandle来获取dialog插件
        // 由于无法直接访问AppHandle，我们返回一个错误提示
        // 实际实现中需要在命令层处理
        Err(format!("请使用前端dialog API选择文件: {}", title))
    }

    /// 选择文件夹
    async fn select_folder(&self, title: &str) -> Result<Option<String>, String> {
        Err(format!("请使用前端dialog API选择文件夹: {}", title))
    }

    /// 打开外部链接
    fn open_external_link(&self, url: &str) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            Command::new("cmd")
                .args(&["/c", "start", "", url])
                .spawn()
                .map_err(|e| format!("打开链接失败: {}", e))?;

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("当前仅支持 Windows 系统".to_string())
        }
    }
}

impl Default for PatchService {
    fn default() -> Self {
        Self::new()
    }
}
