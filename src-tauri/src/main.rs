// Steam Tool Plus - Tauri 主入口
// 禁止控制台输出，优化性能

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    steam_tool_plus_lib::run();
}
