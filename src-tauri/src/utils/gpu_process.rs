// GPU 进程管理模块（Windows 专用）
//
// WebView2/Chromium 的图片解码位图存放在独立的 msedgewebview2.exe GPU 进程中，
// 前端无论怎么移除 <img>、清空 src，Chromium 的 GPU discardable 纹理缓存都不会立即归还。
//
// 唯一能"立即彻底释放 GPU 显存"的手段，是利用 Chromium 的容错机制：
// 结束 GPU 进程（--type=gpu-process）后，browser 进程会检测到 GPU 子进程退出并自动重启，
// 显存随之归零、图片由前端重新上传恢复。渲染进程（JS/DOM 状态）不受影响。

use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

/// 结束本应用的 WebView2 GPU 进程，强制释放全部 GPU 显存
///
/// 实现要点：
/// 1. 枚举所有 msedgewebview2.exe 进程；
/// 2. 仅匹配其中命令行含 --type=gpu-process 的进程；
/// 3. 通过父进程链追溯，只结束属于当前主程序（SteamToolPlus.exe）的进程，
///    避免误伤系统上其他 WebView2 应用（Teams、Outlook 等）的 GPU 进程。
#[cfg(target_os = "windows")]
pub fn kill_gpu_process() {
    let mut system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    system.refresh_processes(ProcessesToUpdate::All, true);

    // 当前主进程 PID（SteamToolPlus.exe）
    let self_pid = Pid::from_u32(std::process::id());

    // 收集属于本应用、且为 GPU 类型的子进程
    let mut gpu_pids: Vec<Pid> = Vec::new();
    for (pid, process) in system.processes() {
        // 只处理 WebView2 进程
        if process.name().to_string_lossy().to_ascii_lowercase() != "msedgewebview2.exe" {
            continue;
        }
        // 仅处理 GPU 类型子进程
        let is_gpu = process
            .cmd()
            .iter()
            .any(|arg| arg.to_string_lossy().contains("--type=gpu-process"));
        if !is_gpu {
            continue;
        }
        // 通过父进程链确认属于本应用
        if belongs_to_self(&system, *pid, self_pid) {
            gpu_pids.push(*pid);
        }
    }

    // 结束 GPU 进程，释放全部 GPU 显存
    for pid in gpu_pids {
        if let Some(process) = system.process(pid) {
            if process.kill() {
                log::info!("已结束 WebView2 GPU 进程以释放显存: {:?}", pid);
            }
        }
    }
}

/// 沿父进程链向上追溯，判断目标进程是否属于当前主程序
/// 最多向上追溯 5 层，并防止自引用死循环
#[cfg(target_os = "windows")]
fn belongs_to_self(system: &System, start_pid: Pid, self_pid: Pid) -> bool {
    let mut current = Some(start_pid);
    let mut depth = 0u32;

    while let Some(pid) = current {
        if pid == self_pid {
            return true;
        }
        if depth >= 5 {
            break;
        }
        let parent = system.process(pid).and_then(|p| p.parent());
        if parent == Some(pid) {
            // 自引用，防止死循环
            break;
        }
        current = parent;
        depth += 1;
    }

    false
}

// 非 Windows 平台空实现，保证跨平台可编译
#[cfg(not(target_os = "windows"))]
pub fn kill_gpu_process() {}