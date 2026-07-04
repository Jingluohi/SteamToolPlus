// Denuvo 授权管理命令
// 提供从注册表提取 D 加密授权、修改 SteamID、写回注册表的能力
// 所有数据保存到 %APPDATA%/SteamToolPlus/denuvo_auth/ 目录

use std::fs;
use std::path::PathBuf;

use crate::models::{
    ActiveSteamUserInfo, DenuvoAuthEntry, DenuvoAuthListItem,
};
use crate::utils::config_path_utils::get_appdata_dir;

/// 注册表中 Steam 每个 App 的授权信息根路径
#[cfg(target_os = "windows")]
const STEAM_APPS_REGISTRY_PATH_PREFIX: &str = r"Software\Valve\Steam\Apps";

/// 注册表中 Steam 当前活动用户路径
#[cfg(target_os = "windows")]
const STEAM_ACTIVE_PROCESS_PATH: &str = r"Software\Valve\Steam\ActiveProcess";

/// D 加密授权备份目录名
const DENUVO_AUTH_DIR_NAME: &str = "denuvo_auth";

/// 备份条目文件名后缀
const BACKUP_FILE_EXTENSION: &str = "json";

/// 将 AppID 转换为注册表子项路径
#[cfg(target_os = "windows")]
fn app_registry_path(app_id: u32) -> String {
    format!("{}\\{}", STEAM_APPS_REGISTRY_PATH_PREFIX, app_id)
}

/// 将 Universe 名称转换为数值
fn universe_to_number(universe: &str) -> u8 {
    match universe.to_lowercase().as_str() {
        "public" => 1,
        "beta" => 2,
        "internal" => 3,
        "dev" => 4,
        _ => 1,
    }
}

/// 根据 AccountID 与 Universe 构造 64 位 SteamID
fn build_steam_id_64(account_id: u32, universe: &str) -> String {
    let universe_num = universe_to_number(universe) as u64;
    // SteamID64 布局：universe(8bit) | account type(4bit, individual=1) | instance(20bit, default=1) | account id(32bit)
    let steam_id_64 = (universe_num << 56) | (1u64 << 52) | (1u64 << 32) | (account_id as u64);
    steam_id_64.to_string()
}

/// 获取 D 加密授权备份目录
fn get_denuvo_auth_dir() -> Result<PathBuf, String> {
    let appdata_dir = get_appdata_dir()?;
    let dir = appdata_dir.join(DENUVO_AUTH_DIR_NAME);
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("创建 D 加密备份目录失败: {}", e))?;
    }
    Ok(dir)
}

/// 获取某个 AppID 的备份文件路径
fn get_backup_file_path(app_id: u32) -> Result<PathBuf, String> {
    let dir = get_denuvo_auth_dir()?;
    Ok(dir.join(format!("{}.{}", app_id, BACKUP_FILE_EXTENSION)))
}

/// 将二进制数据编码为十六进制字符串
fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

/// 将十六进制字符串解码为二进制数据
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let clean_hex = hex.replace(|c: char| c.is_whitespace(), "");
    if clean_hex.len() % 2 != 0 {
        return Err("十六进制字符串长度必须为偶数".to_string());
    }
    let mut bytes = Vec::with_capacity(clean_hex.len() / 2);
    for chunk in clean_hex.as_bytes().chunks(2) {
        let chunk_str = std::str::from_utf8(chunk)
            .map_err(|e| format!("无效的十六进制字符: {}", e))?;
        let byte = u8::from_str_radix(chunk_str, 16)
            .map_err(|e| format!("解析十六进制失败: {}", e))?;
        bytes.push(byte);
    }
    Ok(bytes)
}

/// 获取当前 Steam 活动用户信息
#[tauri::command]
pub fn get_active_steam_user() -> Result<ActiveSteamUserInfo, String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let active_process_key = hkcu
            .open_subkey(STEAM_ACTIVE_PROCESS_PATH)
            .map_err(|e| format!("打开 Steam ActiveProcess 注册表项失败: {}", e))?;

        let account_id: u32 = active_process_key
            .get_value("ActiveUser")
            .map_err(|e| format!("读取 ActiveUser 失败: {}", e))?;
        let universe: String = active_process_key
            .get_value("Universe")
            .map_err(|e| format!("读取 Universe 失败: {}", e))?;

        if account_id == 0 {
            return Err("当前没有 Steam 账号处于活动状态".to_string());
        }

        let steam_id_64 = build_steam_id_64(account_id, &universe);

        Ok(ActiveSteamUserInfo {
            account_id,
            universe,
            steam_id_64,
        })
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("非 Windows 系统不支持读取 Steam 注册表".to_string())
    }
}

/// 从注册表读取指定 AppID 的 D 加密授权信息
#[tauri::command]
pub fn read_denuvo_auth_from_registry(app_id: u32) -> Result<DenuvoAuthEntry, String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let app_key = hkcu
            .open_subkey(&app_registry_path(app_id))
            .map_err(|e| format!("打开 App {} 注册表项失败: {}", app_id, e))?;

        let steam_id: Result<String, _> = app_key.get_value("SteamID");
        let app_ticket = app_key.get_raw_value("AppTicket")
            .ok()
            .filter(|rv| rv.vtype == winreg::enums::REG_BINARY)
            .map(|rv| bytes_to_hex(&rv.bytes));
        let e_ticket = app_key.get_raw_value("ETicket")
            .ok()
            .filter(|rv| rv.vtype == winreg::enums::REG_BINARY)
            .map(|rv| bytes_to_hex(&rv.bytes));

        Ok(DenuvoAuthEntry {
            app_id,
            game_name: String::new(),
            steam_id: steam_id.ok(),
            app_ticket_hex: app_ticket,
            e_ticket_hex: e_ticket,
            backup_time: None,
        })
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("非 Windows 系统不支持读取 Steam 注册表".to_string())
    }
}

/// 将 D 加密授权信息写入注册表
#[tauri::command]
pub fn write_denuvo_auth_to_registry(
    app_id: u32,
    steam_id: Option<String>,
    app_ticket_hex: Option<String>,
    e_ticket_hex: Option<String>,
) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (app_key, _) = hkcu
            .create_subkey_with_flags(&app_registry_path(app_id), KEY_SET_VALUE)
            .map_err(|e| format!("创建/打开 App {} 注册表项失败: {}", app_id, e))?;

        if let Some(steam_id) = steam_id {
            app_key
                .set_value("SteamID", &steam_id)
                .map_err(|e| format!("写入 SteamID 失败: {}", e))?;
        }

        if let Some(hex) = app_ticket_hex {
            let bytes = hex_to_bytes(&hex)?;
            let reg_value = winreg::RegValue {
                vtype: winreg::enums::REG_BINARY,
                bytes,
            };
            app_key
                .set_raw_value("AppTicket", &reg_value)
                .map_err(|e| format!("写入 AppTicket 失败: {}", e))?;
        }

        if let Some(hex) = e_ticket_hex {
            let bytes = hex_to_bytes(&hex)?;
            let reg_value = winreg::RegValue {
                vtype: winreg::enums::REG_BINARY,
                bytes,
            };
            app_key
                .set_raw_value("ETicket", &reg_value)
                .map_err(|e| format!("写入 ETicket 失败: {}", e))?;
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("非 Windows 系统不支持写入 Steam 注册表".to_string())
    }
}

/// 备份指定 AppID 的 D 加密授权到程序数据目录
#[tauri::command]
pub fn backup_denuvo_auth(
    app_id: u32,
    game_name: String,
) -> Result<DenuvoAuthEntry, String> {
    let mut entry = read_denuvo_auth_from_registry(app_id)?;

    entry.game_name = game_name;
    entry.backup_time = Some(chrono::Local::now().to_rfc3339());

    let file_path = get_backup_file_path(app_id)?;
    let json = serde_json::to_string_pretty(&entry)
        .map_err(|e| format!("序列化备份数据失败: {}", e))?;
    fs::write(&file_path, json)
        .map_err(|e| format!("写入备份文件失败: {}", e))?;

    Ok(entry)
}

/// 列出所有已备份的 D 加密授权
#[tauri::command]
pub fn list_denuvo_auth_backups() -> Result<Vec<DenuvoAuthListItem>, String> {
    let dir = get_denuvo_auth_dir()?;
    let mut items = Vec::new();

    let entries = fs::read_dir(&dir)
        .map_err(|e| format!("读取备份目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) != Some(BACKUP_FILE_EXTENSION) {
            continue;
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| format!("读取备份文件失败: {}", e))?;
        let auth: DenuvoAuthEntry = serde_json::from_str(&content)
            .map_err(|e| format!("解析备份文件失败: {}", e))?;

        items.push(DenuvoAuthListItem {
            app_id: auth.app_id,
            game_name: auth.game_name,
            has_steam_id: auth.steam_id.is_some() && !auth.steam_id.as_ref().unwrap().is_empty(),
            has_app_ticket: auth.app_ticket_hex.is_some() && !auth.app_ticket_hex.as_ref().unwrap().is_empty(),
            has_e_ticket: auth.e_ticket_hex.is_some() && !auth.e_ticket_hex.as_ref().unwrap().is_empty(),
            backup_time: auth.backup_time,
        });
    }

    items.sort_by(|a, b| a.app_id.cmp(&b.app_id));
    Ok(items)
}

/// 加载指定 AppID 的备份详情
#[tauri::command]
pub fn load_denuvo_auth_backup(app_id: u32) -> Result<DenuvoAuthEntry, String> {
    let file_path = get_backup_file_path(app_id)?;
    if !file_path.exists() {
        return Err(format!("未找到 AppID {} 的备份", app_id));
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取备份文件失败: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("解析备份文件失败: {}", e))
}

/// 保存用户编辑的 D 加密授权条目（仅更新本地备份文件，不写注册表）
#[tauri::command]
pub fn save_denuvo_auth_entry(entry: DenuvoAuthEntry) -> Result<(), String> {
    let file_path = get_backup_file_path(entry.app_id)?;
    let json = serde_json::to_string_pretty(&entry)
        .map_err(|e| format!("序列化备份数据失败: {}", e))?;
    fs::write(&file_path, json)
        .map_err(|e| format!("写入备份文件失败: {}", e))?;
    Ok(())
}

/// 删除指定 AppID 的本地备份
#[tauri::command]
pub fn delete_denuvo_auth_backup(app_id: u32) -> Result<(), String> {
    let file_path = get_backup_file_path(app_id)?;
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| format!("删除备份文件失败: {}", e))?;
    }
    Ok(())
}

/// 将本地备份的 D 加密授权写回注册表
#[tauri::command]
pub fn apply_denuvo_auth_backup(app_id: u32) -> Result<(), String> {
    let entry = load_denuvo_auth_backup(app_id)?;
    write_denuvo_auth_to_registry(
        app_id,
        entry.steam_id,
        entry.app_ticket_hex,
        entry.e_ticket_hex,
    )
}
