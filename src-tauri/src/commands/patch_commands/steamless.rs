// Steamless 脱壳模块
// 使用 Steamless.CLI.exe 对游戏主程序进行脱壳处理

use std::path::Path;
use tauri::AppHandle;

/// 使用 Steamless 脱壳游戏主程序
/// 运行 Steamless.CLI.exe 对指定的 .exe 文件进行脱壳，
/// 脱壳成功后自动备份原文件并替换为脱壳后的文件
#[tauri::command]
pub async fn unpack_game_exe(
    app: AppHandle,
    game_exe_path: String,
) -> Result<super::common::UnpackResult, String> {
    use std::process::Command;

    use super::common::{get_resource_dir, UnpackResult};

    let resource_dir = get_resource_dir(app)?;
    let steamless_path = Path::new(&resource_dir).join("steamless").join("Steamless.CLI.exe");

    if !steamless_path.exists() {
        return Err(format!("Steamless.CLI.exe 不存在: {}", steamless_path.display()));
    }

    let target_exe = Path::new(&game_exe_path);

    if !target_exe.exists() {
        return Err(format!("游戏主程序不存在: {}", game_exe_path));
    }

    if !target_exe.is_file() {
        return Err(format!("路径不是文件: {}", game_exe_path));
    }

    if let Some(ext) = target_exe.extension() {
        if ext.to_string_lossy().to_lowercase() != "exe" {
            return Err(format!("选择的文件不是 .exe 文件: {}", game_exe_path));
        }
    } else {
        return Err("选择的文件没有扩展名".to_string());
    }

    let target_exe_str = target_exe.to_string_lossy();

    // 运行 Steamless（隐藏窗口）
    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;
    #[cfg(target_os = "windows")]
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut cmd = Command::new(&steamless_path);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let output = cmd
        .arg("--quiet")
        .arg(&*target_exe_str)
        .output()
        .map_err(|e| format!("运行 Steamless 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        let unpacked_exe = target_exe.with_extension("exe.unpacked.exe");
        let unpacked_path = if unpacked_exe.exists() {
            let backup_path = target_exe.with_extension("exe.bak");
            std::fs::rename(target_exe, &backup_path)
                .map_err(|e| format!("备份原文件失败: {}", e))?;
            std::fs::rename(&unpacked_exe, target_exe)
                .map_err(|e| format!("重命名脱壳文件失败: {}", e))?;
            Some(game_exe_path.clone())
        } else {
            None
        };

        Ok(UnpackResult {
            success: true,
            message: "脱壳成功".to_string(),
            unpacked_path,
        })
    } else {
        Ok(UnpackResult {
            success: false,
            message: format!("脱壳失败: {} {}", stdout, stderr),
            unpacked_path: None,
        })
    }
}
