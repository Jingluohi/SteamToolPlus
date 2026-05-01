// 帮助命令
// 处理帮助相关的IPC调用，如读取使用说明文档

use crate::utils::file_utils;

/// 读取使用说明文档内容
/// 从程序根目录下的 resources/README.md 读取
#[tauri::command]
pub fn read_readme_file() -> Result<String, String> {
    // 使用相对路径读取 README.md 文件
    // 程序会从根目录下的 resources 文件夹中读取
    let readme_path = "resources/README.md";
    
    file_utils::read_text_file(readme_path)
}

/// 检查使用说明文件是否存在
#[tauri::command]
pub fn check_readme_exists() -> bool {
    let readme_path = "resources/README.md";
    file_utils::file_exists(readme_path)
}
