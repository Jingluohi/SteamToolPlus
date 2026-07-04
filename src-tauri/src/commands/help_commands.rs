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

/// 赞助图片数据（编译时嵌入到 exe 中）
/// 使用 include_bytes! 宏将 weixin.jpg 嵌入到程序内部
static SPONSOR_IMAGE_BYTES: &[u8] = include_bytes!("../../icons/weixin.jpg");

/// 获取赞助图片的 Base64 编码
/// 将嵌入到 exe 中的图片数据转为 Base64 字符串，前端可直接使用
#[tauri::command]
pub fn get_sponsor_image_base64() -> Result<String, String> {
    // 将嵌入的图片数据编码为 Base64
    use base64::Engine;
    let base64_string = base64::engine::general_purpose::STANDARD.encode(SPONSOR_IMAGE_BYTES);
    Ok(format!("data:image/jpeg;base64,{}", base64_string))
}

/// 清单下载二维码图片数据（编译时嵌入到 exe 中）
/// 使用 include_bytes! 宏将 qingdan.png 嵌入到程序内部
static QINGDAN_IMAGE_BYTES: &[u8] = include_bytes!("../../icons/qingdan.png");

/// 获取清单下载二维码图片的 Base64 编码
/// 将嵌入到 exe 中的图片数据转为 Base64 字符串，前端可直接使用
#[tauri::command]
pub fn get_qingdan_image_base64() -> Result<String, String> {
    // 将嵌入的图片数据编码为 Base64
    use base64::Engine;
    let base64_string = base64::engine::general_purpose::STANDARD.encode(QINGDAN_IMAGE_BYTES);
    Ok(format!("data:image/png;base64,{}", base64_string))
}
