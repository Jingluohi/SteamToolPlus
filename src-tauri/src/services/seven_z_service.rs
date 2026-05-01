// 7z解压服务
// 实现7z压缩包的解压功能

use sevenz_rust::SevenZReader;
use std::fs::File;

/// 7z服务接口
pub trait SevenZServiceTrait: Send + Sync {
    /// 解压7z文件
    fn extract(&self, archive_path: &str, output_path: &str) -> Result<(), String>;
    /// 获取压缩包内容列表
    fn list_contents(&self, archive_path: &str) -> Result<Vec<String>, String>;
}

/// 7z服务实现
pub struct SevenZService;

impl SevenZService {
    /// 创建新的7z服务实例
    pub fn new() -> Self {
        Self
    }
}

impl SevenZServiceTrait for SevenZService {
    /// 解压7z文件到指定目录
    fn extract(&self, archive_path: &str, output_path: &str) -> Result<(), String> {
        // 验证文件是否存在
        if !std::path::Path::new(archive_path).exists() {
            return Err("压缩文件不存在".to_string());
        }

        // 创建输出目录
        let output_dir = std::path::Path::new(output_path);
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| format!("创建目录失败: {}", e))?;
        }

        // 打开7z文件并解压
        SevenZReader::open(archive_path, sevenz_rust::Password::empty())
            .map_err(|e| format!("打开7z文件失败: {:?}", e))?
            .for_each_entries(|entry, reader| {
                let path = output_dir.join(&entry.name());

                // 创建父目录
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                // 写入文件内容（如果不是目录）
                if !entry.is_directory() {
                    let mut file = match File::create(&path) {
                        Ok(f) => f,
                        Err(_) => return Ok(true),
                    };

                    let mut buffer = vec![0u8; entry.size() as usize];
                    if reader.read_exact(&mut buffer).is_ok() {
                        let _ = std::io::Write::write_all(&mut file, &buffer);
                    }
                }

                Ok(true)
            })
            .map_err(|e| format!("解压失败: {:?}", e))?;

        Ok(())
    }

    /// 获取压缩包内容列表
    fn list_contents(&self, archive_path: &str) -> Result<Vec<String>, String> {
        // 验证文件是否存在
        if !std::path::Path::new(archive_path).exists() {
            return Err("压缩文件不存在".to_string());
        }

        // 打开7z文件并读取内容列表
        let mut contents = Vec::new();
        SevenZReader::open(archive_path, sevenz_rust::Password::empty())
            .map_err(|e| format!("打开7z文件失败: {:?}", e))?
            .for_each_entries(|entry, _| {
                contents.push(entry.name().to_string());
                Ok(true)
            })
            .map_err(|e| format!("读取文件列表失败: {:?}", e))?;

        Ok(contents)
    }
}

impl Default for SevenZService {
    fn default() -> Self {
        Self::new()
    }
}
