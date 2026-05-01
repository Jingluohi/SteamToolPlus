// 权限管理
// 定义和管理扩展权限

use std::collections::HashMap;

/// 权限定义
#[derive(Debug, Clone)]
pub struct Permission {
    /// 权限ID
    pub id: String,
    /// 权限名称
    pub name: String,
    /// 权限描述
    pub description: String,
    /// 是否危险权限
    pub dangerous: bool,
}

/// 权限管理器
pub struct PermissionManager {
    /// 权限定义表
    permissions: HashMap<String, Permission>,
    /// 扩展权限配置
    extension_permissions: HashMap<String, Vec<String>>,
}

impl PermissionManager {
    /// 创建新的权限管理器
    pub fn new() -> Self {
        let mut permissions = HashMap::new();
        
        // 注册内置权限
        permissions.insert("ui".to_string(), Permission {
            id: "ui".to_string(),
            name: "UI扩展".to_string(),
            description: "允许扩展修改UI界面".to_string(),
            dangerous: false,
        });
        
        permissions.insert("game.read".to_string(), Permission {
            id: "game.read".to_string(),
            name: "读取游戏库".to_string(),
            description: "允许扩展读取游戏库数据".to_string(),
            dangerous: false,
        });
        
        permissions.insert("game.write".to_string(), Permission {
            id: "game.write".to_string(),
            name: "修改游戏库".to_string(),
            description: "允许扩展修改游戏库数据".to_string(),
            dangerous: true,
        });
        
        permissions.insert("game.launch".to_string(), Permission {
            id: "game.launch".to_string(),
            name: "启动游戏".to_string(),
            description: "允许扩展启动游戏".to_string(),
            dangerous: false,
        });
        
        permissions.insert("storage".to_string(), Permission {
            id: "storage".to_string(),
            name: "存储访问".to_string(),
            description: "允许扩展访问本地存储".to_string(),
            dangerous: false,
        });
        
        permissions.insert("fs.read".to_string(), Permission {
            id: "fs.read".to_string(),
            name: "文件读取".to_string(),
            description: "允许扩展读取文件".to_string(),
            dangerous: false,
        });
        
        permissions.insert("fs.write".to_string(), Permission {
            id: "fs.write".to_string(),
            name: "文件写入".to_string(),
            description: "允许扩展写入文件".to_string(),
            dangerous: true,
        });
        
        permissions.insert("network".to_string(), Permission {
            id: "network".to_string(),
            name: "网络访问".to_string(),
            description: "允许扩展进行网络请求".to_string(),
            dangerous: true,
        });

        Self {
            permissions,
            extension_permissions: HashMap::new(),
        }
    }

    /// 获取权限定义
    pub fn get_permission(&self, id: &str) -> Option<&Permission> {
        self.permissions.get(id)
    }

    /// 获取所有权限
    pub fn get_all_permissions(&self) -> Vec<&Permission> {
        self.permissions.values().collect()
    }

    /// 设置扩展权限
    pub fn set_extension_permissions(&mut self, extension_id: &str, permissions: Vec<String>) {
        self.extension_permissions.insert(extension_id.to_string(), permissions);
    }

    /// 获取扩展权限
    pub fn get_extension_permissions(&self, extension_id: &str) -> Vec<String> {
        self.extension_permissions
            .get(extension_id)
            .cloned()
            .unwrap_or_default()
    }

    /// 检查扩展是否有权限
    pub fn has_permission(&self, extension_id: &str, permission: &str) -> bool {
        self.extension_permissions
            .get(extension_id)
            .map(|perms| perms.contains(&permission.to_string()))
            .unwrap_or(false)
    }

    /// 添加扩展权限
    pub fn grant_permission(&mut self, extension_id: &str, permission: &str) {
        let perms = self.extension_permissions
            .entry(extension_id.to_string())
            .or_insert_with(Vec::new);
        
        if !perms.contains(&permission.to_string()) {
            perms.push(permission.to_string());
        }
    }

    /// 移除扩展权限
    pub fn revoke_permission(&mut self, extension_id: &str, permission: &str) {
        if let Some(perms) = self.extension_permissions.get_mut(extension_id) {
            perms.retain(|p| p != permission);
        }
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}
