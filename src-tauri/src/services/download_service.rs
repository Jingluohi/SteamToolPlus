// 下载服务
// 实现游戏本体下载的业务逻辑

use std::path::Path;
use tauri::AppHandle;

/// 清单文件夹读取结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct ManifestFolderResult {
    #[serde(rename = "jsonFiles")]
    pub json_files: Vec<String>,
    #[serde(rename = "vdfFiles")]
    pub vdf_files: Vec<String>,
    #[serde(rename = "manifestFiles")]
    pub manifest_files: Vec<String>,
}

/// 下载结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadResult {
    pub success: bool,
    pub message: String,
}

/// 进度文件信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProgressFileInfo {
    pub name: String,
    pub path: String,
    /// 进度百分比 (0-100)
    pub progress: u32,
    /// Depot ID
    pub depot_id: String,
}

/// 目录项信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    #[serde(rename = "is_dir")]
    pub is_dir: bool,
    pub size: Option<u64>,
}

/// 下载服务接口
pub trait DownloadServiceTrait: Send + Sync {
    /// 读取清单文件夹
    fn read_manifest_folder(&self, folder_path: &str) -> Result<ManifestFolderResult, String>;
    /// 读取文本文件
    fn read_text_file(&self, file_path: &str) -> Result<String, String>;
    /// 读取JSON文件
    fn read_json_file(&self, file_path: &str) -> Result<serde_json::Value, String>;
    /// 获取可用盘符
    fn get_available_drive(&self) -> Result<String, String>;
    /// 获取清单路径
    fn get_manifest_path(&self, app: &AppHandle, game_id: &str) -> Result<String, String>;
    /// 启动游戏下载
    fn start_game_download(
        &self,
        app: &AppHandle,
        manifest_path: &str,
        download_path: &str,
    ) -> Result<DownloadResult, String>;
    /// 获取下载进度文件
    fn get_download_progress_files(&self) -> Result<Vec<ProgressFileInfo>, String>;
    /// 读取目录
    fn read_directory(&self, path: &str) -> Result<Vec<DirEntry>, String>;
    /// 删除文件
    fn delete_file(&self, file_path: &str) -> Result<(), String>;
    /// 关闭系统
    fn shutdown_system(&self) -> Result<(), String>;
    /// 获取游戏的 depot 列表
    fn get_game_depots(&self, app: &AppHandle, game_id: &str) -> Result<Vec<String>, String>;
    /// 检查并清理已完成的下载
    /// 当游戏的所有 depot 都下载完成后，静默删除对应的进度 JSON 文件
    fn check_and_cleanup_completed_downloads(&self, app: &AppHandle) -> Result<(), String>;
    /// 停止下载进程
    /// 终止 ddv20.exe 进程
    fn stop_download(&self) -> Result<(), String>;
}

/// 下载服务实现
pub struct DownloadService;

impl DownloadService {
    /// 创建新的下载服务实例
    pub fn new() -> Self {
        Self
    }

    /// 解析进度文件名
    /// 格式: "{百分比}% - {depotId}.json"
    /// 返回: (进度百分比, depot ID)
    fn parse_progress_filename(file_name: &str) -> (u32, String) {
        // 移除 .json 后缀
        let name_without_ext = file_name.trim_end_matches(".json");

        // 分割 "100% - 3357651"
        if let Some(pos) = name_without_ext.find("% - ") {
            let progress_str = &name_without_ext[..pos];
            let depot_id_str = &name_without_ext[pos + 4..];

            let progress = progress_str.parse::<u32>().unwrap_or(0);
            let depot_id = depot_id_str.to_string();

            return (progress, depot_id);
        }

        (0, String::new())
    }

    /// 获取资源目录路径
    fn get_resource_dir(&self, _app: &AppHandle) -> Result<std::path::PathBuf, String> {
        let exe_dir = std::env::current_exe()
            .map_err(|e| format!("无法获取程序路径: {}", e))?
            .parent()
            .ok_or("无法获取程序所在目录")?
            .to_path_buf();

        let resources_path = exe_dir.join("resources");
        if resources_path.exists() && resources_path.is_dir() {
            return Ok(resources_path);
        }

        if let Some(target_dir) = exe_dir.parent() {
            let project_resources = target_dir.parent().unwrap_or(target_dir).join("resources");
            if project_resources.exists() && project_resources.is_dir() {
                return Ok(project_resources);
            }
        }

        Err("无法找到资源目录".to_string())
    }
}

impl DownloadServiceTrait for DownloadService {
    /// 读取清单文件夹内容
    fn read_manifest_folder(&self, folder_path: &str) -> Result<ManifestFolderResult, String> {
        let path = Path::new(folder_path);

        if !path.exists() {
            return Err("文件夹不存在".to_string());
        }

        if !path.is_dir() {
            return Err("路径不是文件夹".to_string());
        }

        let mut json_files = Vec::new();
        let mut vdf_files = Vec::new();
        let mut manifest_files = Vec::new();

        let entries = std::fs::read_dir(path).map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
            let file_path = entry.path();

            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    let path_str = file_path.to_string_lossy().to_string();

                    match ext.as_str() {
                        "json" => json_files.push(path_str),
                        "vdf" => vdf_files.push(path_str),
                        "manifest" => manifest_files.push(path_str),
                        _ => {}
                    }
                }
            }
        }

        Ok(ManifestFolderResult {
            json_files,
            vdf_files,
            manifest_files,
        })
    }

    /// 读取文本文件内容
    fn read_text_file(&self, file_path: &str) -> Result<String, String> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err("文件不存在".to_string());
        }

        std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))
    }

    /// 读取JSON文件内容
    fn read_json_file(&self, file_path: &str) -> Result<serde_json::Value, String> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err("文件不存在".to_string());
        }

        let content =
            std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;
        let json: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

        Ok(json)
    }

    /// 获取可用的游戏盘符
    fn get_available_drive(&self) -> Result<String, String> {
        let d_drive = Path::new("D:\\");
        if d_drive.exists() {
            Ok("D:".to_string())
        } else {
            Ok("C:".to_string())
        }
    }

    /// 获取游戏的清单文件夹路径
    fn get_manifest_path(&self, app: &AppHandle, game_id: &str) -> Result<String, String> {
        let resource_dir = self.get_resource_dir(app)?;
        let manifest_dir = resource_dir.join("manifest").join(game_id);

        if manifest_dir.exists() && manifest_dir.is_dir() {
            Ok(manifest_dir.to_string_lossy().to_string())
        } else {
            Ok("".to_string())
        }
    }

    /// 启动游戏下载
    fn start_game_download(
        &self,
        app: &AppHandle,
        manifest_path: &str,
        download_path: &str,
    ) -> Result<DownloadResult, String> {
        let resource_dir = self.get_resource_dir(app)?;
        let ddv20_path = resource_dir.join("ddv20.exe");

        if !ddv20_path.exists() {
            return Err(format!("ddv20.exe 不存在: {}", ddv20_path.display()));
        }

        // 检查下载路径的父目录是否存在，不存在则创建
        let download_dir = Path::new(download_path);
        if let Some(parent) = download_dir.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("创建下载目录失败: {}", e))?;
            }
        }

        // 在 Windows 上使用 start 命令在新终端窗口中运行 ddv20.exe
        // 这样用户可以看到下载进度
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            let ddv20_path_str = ddv20_path.to_string_lossy().replace('\\', "/");
            let download_path_str = download_path.replace('\\', "/");
            let manifest_path_str = manifest_path.replace('\\', "/");

            // 使用 start 命令在新窗口中运行 ddv20
            // start "" "ddv20.exe路径" -lu China --use-http -o "下载路径" app -p "清单路径"
            let child = Command::new("cmd")
                .arg("/c")
                .arg("start")
                .arg("")
                .arg(&ddv20_path_str)
                .arg("-lu")
                .arg("China")
                .arg("--use-http")
                .arg("-o")
                .arg(&download_path_str)
                .arg("app")
                .arg("-p")
                .arg(&manifest_path_str)
                .spawn()
                .map_err(|e| format!("启动下载进程失败: {}", e))?;

            Ok(DownloadResult {
                success: true,
                message: format!("下载进程已启动 (PID: {:?})", child.id()),
            })
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("当前仅支持 Windows 系统".to_string())
        }
    }

    /// 获取下载进度文件列表
    /// 扫描程序根目录下的 {百分比}% - {depotId}.json 文件
    fn get_download_progress_files(&self) -> Result<Vec<ProgressFileInfo>, String> {
        let exe_dir = std::env::current_exe()
            .map_err(|e| format!("无法获取程序路径: {}", e))?
            .parent()
            .ok_or("无法获取程序所在目录")?
            .to_path_buf();

        let mut progress_files = Vec::new();

        let entries = std::fs::read_dir(&exe_dir).map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
            let file_path = entry.path();

            if file_path.is_file() {
                if let Some(file_name) = file_path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    // 匹配进度文件格式: "{百分比}% - {depotId}.json"
                    if file_name_str.ends_with(".json") && file_name_str.contains("% - ") {
                        // 解析文件名获取进度和 depot ID
                        // 格式: "100% - 3357651.json"
                        let (progress, depot_id) = Self::parse_progress_filename(&file_name_str);
                        progress_files.push(ProgressFileInfo {
                            name: file_name_str.to_string(),
                            path: file_path.to_string_lossy().to_string(),
                            progress,
                            depot_id,
                        });
                    }
                }
            }
        }

        Ok(progress_files)
    }

    /// 读取目录内容
    fn read_directory(&self, path: &str) -> Result<Vec<DirEntry>, String> {
        let dir_path = Path::new(path);

        if !dir_path.exists() {
            return Err("目录不存在".to_string());
        }

        if !dir_path.is_dir() {
            return Err("路径不是目录".to_string());
        }

        let mut entries = Vec::new();

        for entry in std::fs::read_dir(dir_path).map_err(|e| format!("读取目录失败: {}", e))? {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let metadata = entry
                .metadata()
                .map_err(|e| format!("获取元数据失败: {}", e))?;

            let size = if metadata.is_file() {
                Some(metadata.len())
            } else {
                None
            };

            entries.push(DirEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                is_dir: metadata.is_dir(),
                size,
            });
        }

        Ok(entries)
    }

    /// 删除文件
    fn delete_file(&self, file_path: &str) -> Result<(), String> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err("文件不存在".to_string());
        }

        std::fs::remove_file(path).map_err(|e| format!("删除文件失败: {}", e))?;

        Ok(())
    }

    /// 关闭系统
    fn shutdown_system(&self) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            Command::new("shutdown")
                .args(&["/s", "/t", "0"])
                .spawn()
                .map_err(|e| format!("执行关机命令失败: {}", e))?;

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("当前仅支持 Windows 系统".to_string())
        }
    }

    /// 获取游戏的 depot 列表
    /// 通过扫描 manifest 文件夹中的 .manifest 文件
    fn get_game_depots(&self, app: &AppHandle, game_id: &str) -> Result<Vec<String>, String> {
        let resource_dir = self.get_resource_dir(app)?;
        let manifest_dir = resource_dir.join("manifest").join(game_id);

        if !manifest_dir.exists() || !manifest_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut depots = Vec::new();
        let entries = std::fs::read_dir(&manifest_dir)
            .map_err(|e| format!("读取 manifest 目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
            let file_path = entry.path();

            if file_path.is_file() {
                if let Some(file_name) = file_path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    // manifest 文件名格式: {depotId}_{hash}.manifest
                    if file_name_str.ends_with(".manifest") {
                        // 解析 depot ID
                        if let Some(pos) = file_name_str.find('_') {
                            let depot_id = &file_name_str[..pos];
                            depots.push(depot_id.to_string());
                        }
                    }
                }
            }
        }

        Ok(depots)
    }

    /// 检查并清理已完成的下载
    /// 当游戏的所有 depot 都下载完成后（进度为 100%），静默删除对应的进度 JSON 文件
    fn check_and_cleanup_completed_downloads(&self, app: &AppHandle) -> Result<(), String> {
        // 获取所有进度文件
        let progress_files = self.get_download_progress_files()?;
        
        if progress_files.is_empty() {
            return Ok(());
        }

        // 按游戏分组整理进度文件
        // 首先获取所有游戏 ID
        let resource_dir = self.get_resource_dir(app)?;
        let manifest_dir = resource_dir.join("manifest");
        
        if !manifest_dir.exists() || !manifest_dir.is_dir() {
            return Ok(());
        }

        // 遍历所有游戏
        let game_entries = std::fs::read_dir(&manifest_dir)
            .map_err(|e| format!("读取 manifest 目录失败: {}", e))?;

        for game_entry in game_entries {
            let game_entry = match game_entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let game_path = game_entry.path();

            if !game_path.is_dir() {
                continue;
            }

            let game_id = game_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            if game_id.is_empty() {
                continue;
            }

            // 获取该游戏的所有 depot
            let depots = match self.get_game_depots(app, game_id) {
                Ok(d) => d,
                Err(_) => continue,
            };
            if depots.is_empty() {
                continue;
            }

            // 检查该游戏的每个 depot 是否都有 100% 的进度文件
            let mut all_completed = true;
            let mut game_progress_files = Vec::new();

            for depot_id in &depots {
                // 查找该 depot 的进度文件
                let depot_progress: Vec<&ProgressFileInfo> = progress_files.iter()
                    .filter(|p| p.depot_id == *depot_id && p.progress == 100)
                    .collect();

                if depot_progress.is_empty() {
                    // 该 depot 没有 100% 的进度文件
                    all_completed = false;
                    break;
                } else {
                    // 记录该 depot 的进度文件路径
                    for p in depot_progress {
                        game_progress_files.push(p.path.clone());
                    }
                }
            }

            // 如果所有 depot 都完成了，静默删除进度文件
            if all_completed && !game_progress_files.is_empty() {
                for file_path in game_progress_files {
                    let _ = self.delete_file(&file_path);
                }
            }
        }

        Ok(())
    }

    /// 停止下载进程
    /// 终止 ddv20.exe 进程
    fn stop_download(&self) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // 使用 taskkill 命令终止 ddv20.exe 进程
            let output = Command::new("taskkill")
                .args(&["/F", "/IM", "ddv20.exe"])
                .output()
                .map_err(|e| format!("执行终止进程命令失败: {}", e))?;

            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.contains("not found") || stderr.contains("找不到") {
                    Err("ddv20.exe 进程未运行".to_string())
                } else {
                    Err(format!("终止进程失败: {}", stderr))
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("当前仅支持 Windows 系统".to_string())
        }
    }
}

impl Default for DownloadService {
    fn default() -> Self {
        Self::new()
    }
}
