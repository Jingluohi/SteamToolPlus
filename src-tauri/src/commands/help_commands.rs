// 帮助命令
// 处理程序内置图片（赞助二维码、清单二维码）的嵌入式资源命令

/// 微信支付赞助图片数据（编译时嵌入到 exe 中）
/// 使用 include_bytes! 宏将 weixin.jpg 嵌入到程序内部
static WEIXIN_IMAGE_BYTES: &[u8] = include_bytes!("../../icons/weixin.jpg");

/// 支付宝赞助图片数据（编译时嵌入到 exe 中）
/// 使用 include_bytes! 宏将 alipay.jpg 嵌入到程序内部
static ALIPAY_IMAGE_BYTES: &[u8] = include_bytes!("../../icons/alipay.jpg");

/// 获取微信支付赞助图片的 Base64 编码
/// 将嵌入到 exe 中的图片数据转为 Base64 字符串，前端可直接使用
#[tauri::command]
pub fn get_weixin_image_base64() -> Result<String, String> {
    // 将嵌入的图片数据编码为 Base64
    use base64::Engine;
    let base64_string = base64::engine::general_purpose::STANDARD.encode(WEIXIN_IMAGE_BYTES);
    Ok(format!("data:image/jpeg;base64,{}", base64_string))
}

/// 获取支付宝赞助图片的 Base64 编码
/// 将嵌入到 exe 中的图片数据转为 Base64 字符串，前端可直接使用
#[tauri::command]
pub fn get_alipay_image_base64() -> Result<String, String> {
    // 将嵌入的图片数据编码为 Base64
    use base64::Engine;
    let base64_string = base64::engine::general_purpose::STANDARD.encode(ALIPAY_IMAGE_BYTES);
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
