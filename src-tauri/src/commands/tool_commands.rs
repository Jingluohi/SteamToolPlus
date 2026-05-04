/**
 * tool_commands.rs - 工具命令模块
 * 提供Lua转VDF和Steam封面下载功能的命令
 */

use std::fs;
use std::path::Path;
use regex::Regex;

/// 读取文件内容
#[tauri::command]
pub fn read_file_content(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 获取文件夹中的所有Lua文件
#[tauri::command]
pub fn get_lua_files_in_folder(folder: String) -> Result<Vec<String>, String> {
    let mut lua_files = Vec::new();
    
    fn scan_directory(dir: &Path, files: &mut Vec<String>) -> Result<(), String> {
        let entries = fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                scan_directory(&path, files)?;
            } else if let Some(ext) = path.extension() {
                if ext == "lua" {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(())
    }
    
    scan_directory(Path::new(&folder), &mut lua_files)?;
    Ok(lua_files)
}

/// 转换Lua文件到VDF
#[tauri::command]
pub fn convert_lua_to_vdf(file_path: String) -> Result<serde_json::Value, String> {
    // 读取Lua文件内容
    let content = fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;
    
    // 解析Lua文件，提取depot信息
    let depots = parse_lua_content(&content);
    
    if depots.is_empty() {
        return Ok(serde_json::json!({
            "success": false,
            "message": "未找到任何depot信息",
            "depotCount": 0
        }));
    }
    
    // 生成VDF内容
    let vdf_content = generate_vdf(&depots);
    
    // 生成输出文件路径 - 固定保存为 config.vdf
    let path = Path::new(&file_path);
    let parent = path.parent().unwrap_or(Path::new("."));
    let output_path = parent.join("config.vdf");
    
    // 写入VDF文件
    fs::write(&output_path, vdf_content).map_err(|e| format!("写入文件失败: {}", e))?;
    
    Ok(serde_json::json!({
        "success": true,
        "message": format!("成功转换 {} 个depot", depots.len()),
        "depotCount": depots.len(),
        "outputPath": output_path.to_string_lossy().to_string()
    }))
}

/// 解析Lua内容，提取depot信息
fn parse_lua_content(content: &str) -> Vec<(String, String)> {
    let mut depots = Vec::new();
    
    // 正则匹配 addappid(depot_id, 1, "decryption_key") 格式
    let re = Regex::new(r#"addappid\s*\(\s*(\d+)\s*,\s*\d+\s*,\s*"([a-f0-9]+)"\s*\)"#).unwrap();
    
    for cap in re.captures_iter(content) {
        let depot_id = cap[1].to_string();
        let key = cap[2].to_string();
        depots.push((depot_id, key));
    }
    
    depots
}

/// 生成VDF格式内容
fn generate_vdf(depots: &[(String, String)]) -> String {
    let mut lines = vec!["\"depots\"".to_string(), "{".to_string()];
    
    for (depot_id, key) in depots {
        lines.push(format!("\t\"{}\"", depot_id));
        lines.push("\t{".to_string());
        lines.push(format!("\t\t\"DecryptionKey\" \"{}\"", key));
        lines.push("\t}".to_string());
    }
    
    lines.push("}".to_string());
    lines.push("".to_string());
    
    lines.join("\n")
}

/// 获取文件夹中的所有VDF文件
#[tauri::command]
pub fn get_vdf_files_in_folder(folder: String) -> Result<Vec<String>, String> {
    let mut vdf_files = Vec::new();
    
    fn scan_directory(dir: &Path, files: &mut Vec<String>) -> Result<(), String> {
        let entries = fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                scan_directory(&path, files)?;
            } else if let Some(ext) = path.extension() {
                if ext == "vdf" {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(())
    }
    
    scan_directory(Path::new(&folder), &mut vdf_files)?;
    Ok(vdf_files)
}

/// 转换VDF文件到Lua
#[tauri::command]
pub fn convert_vdf_to_lua(file_path: String) -> Result<serde_json::Value, String> {
    // 读取VDF文件内容
    let content = fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;
    
    // 解析VDF文件，提取depot信息
    let depots = parse_vdf_content(&content);
    
    if depots.is_empty() {
        return Ok(serde_json::json!({
            "success": false,
            "message": "未找到任何depot信息",
            "depotCount": 0
        }));
    }
    
    // 获取第一个depot_id作为主App ID的基础
    let first_depot_id = depots[0].0.parse::<u64>().unwrap_or(0);
    let main_app_id = if first_depot_id > 0 { first_depot_id - 1 } else { 0 };
    
    // 从同目录的.manifest文件提取manifest ID
    let path = Path::new(&file_path);
    let parent = path.parent().unwrap_or(Path::new("."));
    let manifest_map = extract_manifest_ids(parent, &depots);
    
    // 生成Lua内容
    let lua_content = generate_lua(main_app_id, &depots, &manifest_map);
    
    // 生成输出文件路径 - 固定保存为 {main_app_id}.lua
    let output_path = parent.join(format!("{}.lua", main_app_id));
    
    // 写入Lua文件
    fs::write(&output_path, lua_content).map_err(|e| format!("写入文件失败: {}", e))?;
    
    Ok(serde_json::json!({
        "success": true,
        "message": format!("成功转换 {} 个depot", depots.len()),
        "depotCount": depots.len(),
        "mainAppId": main_app_id,
        "outputPath": output_path.to_string_lossy().to_string()
    }))
}

/// 解析VDF内容，提取depot信息
fn parse_vdf_content(content: &str) -> Vec<(String, String)> {
    let mut depots = Vec::new();
    let mut current_depot: Option<String> = None;
    
    // 按行解析VDF
    for line in content.lines() {
        let trimmed = line.trim();
        
        // 匹配 depot_id 行: "123456" 或 "depot_id"
        if trimmed.starts_with('"') && !trimmed.contains("DecryptionKey") && !trimmed.contains("depots") {
            if let Some(end_quote) = trimmed[1..].find('"') {
                let depot_id = &trimmed[1..=end_quote];
                // 检查是否为纯数字（depot_id）
                if depot_id.chars().all(|c| c.is_ascii_digit()) {
                    current_depot = Some(depot_id.to_string());
                }
            }
        }
        
        // 匹配 DecryptionKey 行: "DecryptionKey" "key"
        if trimmed.contains("DecryptionKey") {
            let re = Regex::new(r#""DecryptionKey"\s+"([a-f0-9]+)""#).unwrap();
            if let Some(cap) = re.captures(trimmed) {
                let key = cap[1].to_string();
                if let Some(ref depot_id) = current_depot {
                    depots.push((depot_id.clone(), key));
                    current_depot = None;
                }
            }
        }
    }
    
    depots
}

/// 从目录中的.manifest文件提取manifest ID
fn extract_manifest_ids(dir: &Path, _depots: &[(String, String)]) -> std::collections::HashMap<String, String> {
    let mut manifest_map = std::collections::HashMap::new();
    
    // 读取目录中的所有文件
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "manifest" {
                    // 解析文件名: {depot_id}_{manifest_id}.manifest
                    if let Some(filename) = path.file_stem() {
                        let name = filename.to_string_lossy().to_string();
                        if let Some(underscore_pos) = name.find('_') {
                            let depot_id = &name[..underscore_pos];
                            let manifest_id = &name[underscore_pos + 1..];
                            manifest_map.insert(depot_id.to_string(), manifest_id.to_string());
                        }
                    }
                }
            }
        }
    }
    
    manifest_map
}

/// 生成Lua格式内容
fn generate_lua(
    main_app_id: u64,
    depots: &[(String, String)],
    manifest_map: &std::collections::HashMap<String, String>
) -> String {
    let mut lines = Vec::new();

    // 添加主App ID（无密钥）- 第一个depot_id - 1
    lines.push(format!("addappid({})", main_app_id));

    // 添加带密钥的depot
    for (depot_id, key) in depots {
        lines.push(format!("addappid({},0,\"{}\")", depot_id, key));
    }

    // 添加setManifestid
    for (depot_id, _) in depots {
        if let Some(manifest_id) = manifest_map.get(depot_id) {
            lines.push(format!("setManifestid({},\"{}\")", depot_id, manifest_id));
        }
    }

    lines.join("\n")
}

/// 下载Steam封面
/// 支持多种Steam CDN地址，与 steam_cover_downloader.py 使用相同的API和保存逻辑
#[tauri::command]
pub async fn download_steam_cover(
    game_id: String,
    size_type: String,
    size_desc: String,
    width: i32,
    height: i32,
    output_path: Option<String>,
) -> Result<serde_json::Value, String> {
    // 根据图片类型确定文件扩展名和路径
    let (path, ext) = if size_type == "logo" {
        ("logo.png".to_string(), "png")
    } else {
        (format!("{}.jpg", size_type), "jpg")
    };
    
    // 构建多个CDN URL列表（按优先级排序）
    // 注意：不同的CDN可能返回不同的图片版本
    let urls = vec![
        format!("https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/{}/{}", game_id, path),
        format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/{}", game_id, path),
        format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/{}", game_id, path),
    ];
    
    // 构建输出路径 - 与Python脚本一致：创建尺寸子文件夹
    let output_dir = output_path.unwrap_or_else(|| "./downloads".to_string());
    let size_folder = format!("{}x{}", width, height);
    let size_dir = Path::new(&output_dir).join(&size_folder);
    let output_file = size_dir.join(format!("{}.{}", game_id, ext));
    
    // 确保输出目录存在
    fs::create_dir_all(&size_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    
    // 尝试所有CDN地址
    let mut last_error = String::new();
    let output_file_str = output_file.to_string_lossy().to_string();
    
    for url in urls {
        match download_image(&url, &output_file_str).await {
            Ok(()) => {
                return Ok(serde_json::json!({
                    "success": true,
                    "message": format!("下载成功: {}", size_desc),
                    "filePath": output_file.to_string_lossy().to_string()
                }));
            }
            Err(e) => {
                last_error = e;
                continue;
            }
        }
    }
    
    // 所有CDN都失败了
    Ok(serde_json::json!({
        "success": false,
        "message": format!("下载失败: {}", last_error)
    }))
}

/// 下载单个图片
async fn download_image(url: &str, output_file: &str) -> Result<(), String> {
    // 设置请求头，模拟浏览器请求
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "image/webp,image/apng,image/*,*/*;q=0.8")
        .header("Referer", "https://store.steampowered.com/")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;
    
    if response.status().is_success() {
        // 检查内容类型是否为图片
        let content_type = response.headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        
        if content_type.contains("image") || content_type.contains("application/octet-stream") {
            let bytes = response.bytes().await
                .map_err(|e| format!("读取响应失败: {}", e))?;
            
            fs::write(output_file, bytes)
                .map_err(|e| format!("保存文件失败: {}", e))?;
            
            Ok(())
        } else {
            Err(format!("非图片内容({})", content_type))
        }
    } else {
        Err(format!("HTTP错误: {}", response.status()))
    }
}
