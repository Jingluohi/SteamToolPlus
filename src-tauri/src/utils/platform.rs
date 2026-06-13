// 跨平台工具函数
// 消除多处重复的跨平台命令执行代码

#![allow(dead_code)]

use std::path::Path;
use std::process::Command;
use std::io::{Read, Seek, SeekFrom};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

/// 使用系统默认程序打开文件或URL
/// 跨平台实现：Windows 用 cmd /c start，macOS 用 open，Linux 用 xdg-open
pub fn open_with_default_app(path_or_url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/c", "start", "", path_or_url])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("打开失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path_or_url)
            .spawn()
            .map_err(|e| format!("打开失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path_or_url)
            .spawn()
            .map_err(|e| format!("打开失败: {}", e))?;
    }

    Ok(())
}

/// 创建隐藏窗口的 Command（Windows 专用）
#[cfg(target_os = "windows")]
pub fn create_hidden_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

/// 创建隐藏窗口的 Command（非 Windows 平台直接返回普通 Command）
#[cfg(not(target_os = "windows"))]
pub fn create_hidden_command(program: &str) -> Command {
    Command::new(program)
}

/// 使用 taskkill 关闭进程（仅 Windows）
#[cfg(target_os = "windows")]
pub fn kill_process(process_name: &str) -> Result<(), String> {
    Command::new("taskkill")
        .args(["/F", "/IM", process_name])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("关闭进程失败: {}", e))?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn kill_process(_process_name: &str) -> Result<(), String> {
    Ok(())
}

/// 检测文件是否包含 64 位 PE 头
/// 通过读取 PE 文件头中的 Machine 字段判断架构，比文件名匹配更可靠
pub fn is_pe_64bit(exe_path: &Path) -> Result<bool, String> {
    use std::fs;

    let mut file = fs::File::open(exe_path)
        .map_err(|e| format!("无法打开文件: {}", e))?;

    // 读取 DOS 头（前 64 字节）
    let mut dos_header = [0u8; 64];
    file.read_exact(&mut dos_header)
        .map_err(|e| format!("读取 DOS 头失败: {}", e))?;

    // 验证 DOS 签名 "MZ"
    if dos_header[0] != b'M' || dos_header[1] != b'Z' {
        return Err("不是有效的 PE 文件".to_string());
    }

    // 获取 PE 签名偏移量（DOS 头偏移 0x3C 处）
    let pe_offset = u32::from_le_bytes([
        dos_header[0x3C],
        dos_header[0x3D],
        dos_header[0x3E],
        dos_header[0x3F],
    ]) as usize;

    // 读取 PE 签名 + COFF 头（4 字节签名 + 20 字节 COFF 头）
    let mut pe_header = [0u8; 24];
    file.seek(SeekFrom::Start(pe_offset as u64))
        .map_err(|e| format!("定位 PE 头失败: {}", e))?;
    file.read_exact(&mut pe_header)
        .map_err(|e| format!("读取 PE 头失败: {}", e))?;

    // 验证 PE 签名 "PE\0\0"
    if pe_header[0] != b'P' || pe_header[1] != b'E' || pe_header[2] != 0 || pe_header[3] != 0 {
        return Err("不是有效的 PE 文件".to_string());
    }

    // Machine 字段在 COFF 头的前 2 字节（PE 签名偏移 4 字节处 = pe_header[4..6]）
    let machine = u16::from_le_bytes([pe_header[4], pe_header[5]]);

    // IMAGE_FILE_MACHINE_AMD64 = 0x8664
    // IMAGE_FILE_MACHINE_I386 = 0x14C
    Ok(machine == 0x8664)
}

/// 检测游戏目录的架构（通过扫描 exe 文件）
/// 返回 true = 64位, false = 32位
/// 通过实际读取 PE 文件头判断，比文件名包含 "64"/"x64" 更可靠
pub fn detect_game_architecture(game_dir: &Path) -> bool {
    if !game_dir.exists() || !game_dir.is_dir() {
        return false;
    }

    // 扫描目录中的 .exe 文件
    let entries = match std::fs::read_dir(game_dir) {
        Ok(entries) => entries,
        Err(_) => return false,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("exe") {
                    if let Ok(is_64) = is_pe_64bit(&path) {
                        if is_64 {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}