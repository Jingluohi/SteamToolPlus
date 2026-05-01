// 安全沙箱实现
// 提供扩展的安全隔离环境

use crate::security::permission::PermissionManager;

/// 安全沙箱
pub struct SecuritySandbox {
    /// 扩展ID
    extension_id: String,
    /// 权限管理器
    permission_manager: PermissionManager,
}

impl SecuritySandbox {
    /// 创建新的安全沙箱
    pub fn new(extension_id: String) -> Self {
        Self {
            extension_id,
            permission_manager: PermissionManager::new(),
        }
    }

    /// 验证操作
    pub fn validate_operation(&self, operation: &str) -> bool {
        match operation {
            "read_file" => self.permission_manager.has_permission(&self.extension_id, "fs.read"),
            "write_file" => self.permission_manager.has_permission(&self.extension_id, "fs.write"),
            "read_game" => self.permission_manager.has_permission(&self.extension_id, "game.read"),
            "write_game" => self.permission_manager.has_permission(&self.extension_id, "game.write"),
            "launch_game" => self.permission_manager.has_permission(&self.extension_id, "game.launch"),
            "network" => self.permission_manager.has_permission(&self.extension_id, "network"),
            _ => false,
        }
    }

    /// 验证路径访问
    pub fn validate_path(&self, path: &str) -> bool {
        // 禁止访问系统敏感目录
        let blocked_paths = [
            "C:\\Windows",
            "C:\\Program Files",
            "C:\\Program Files (x86)",
            "C:\\Users\\All Users",
            "C:\\ProgramData",
        ];

        for blocked in &blocked_paths {
            if path.starts_with(blocked) {
                return false;
            }
        }

        // 只允许访问程序目录下的文件
        if !path.starts_with("data") && !path.starts_with("config") && !path.starts_with("extensions") {
            return false;
        }

        true
    }
}
