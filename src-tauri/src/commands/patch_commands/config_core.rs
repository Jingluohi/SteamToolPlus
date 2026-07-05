// 核心配置模块
// 处理 Overlay、Main、User、App 配置的保存、加载和解析

use crate::models::steam_config::*;
use std::path::Path;
use super::common::{ConfigSaveResult, ConfigLoadResult};

// ============================================
// Overlay 配置
// ============================================

/// 保存 Overlay 配置
/// 接收前端 JSON 格式，只更新提供的字段，保留已有数据
#[tauri::command]
pub async fn save_overlay_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use crate::models::steam_config::*;
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    // 读取现有的 overlay 配置，保留已有数据
    let overlay_config_path = steam_settings_dir.join("configs.overlay.ini");
    let mut overlay_config = if overlay_config_path.exists() {
        let content = fs::read_to_string(&overlay_config_path)
            .await
            .map_err(|e| format!("读取 configs.overlay.ini 失败: {}", e))?;
        parse_overlay_ini(&content)?
    } else {
        OverlayConfig::default_config()
    };

    // 更新顶层字段
    if let Some(v) = config.get("enableExperimentalOverlay").and_then(|v| v.as_bool()) {
        overlay_config.enable_experimental_overlay = v;
    }
    if let Some(v) = config.get("hookDelaySec").and_then(|v| v.as_u64()) {
        overlay_config.hook_delay_sec = Some(v as u32);
    }
    if let Some(v) = config.get("rendererDetectorTimeoutSec").and_then(|v| v.as_u64()) {
        overlay_config.renderer_detector_timeout_sec = Some(v as u32);
    }
    if let Some(v) = config.get("overlayHotkey").and_then(|v| v.as_str()) {
        overlay_config.overlay_hotkey = v.to_string();
    }
    if let Some(v) = config.get("fpsAveragingWindow").and_then(|v| v.as_f64()) {
        overlay_config.fps_averaging_window = Some(v as f32);
    }

    // 更新 notifications
    if let Some(notifications) = config.get("notifications") {
        let n = &mut overlay_config.notifications;
        if let Some(v) = notifications.get("disableAchievementNotification").and_then(|v| v.as_bool()) {
            n.disable_achievement_notification = v;
        }
        if let Some(v) = notifications.get("disableFriendNotification").and_then(|v| v.as_bool()) {
            n.disable_friend_notification = v;
        }
        if let Some(v) = notifications.get("disableAchievementProgress").and_then(|v| v.as_bool()) {
            n.disable_achievement_progress = v;
        }
        if let Some(v) = notifications.get("disableWarningAny").and_then(|v| v.as_bool()) {
            n.disable_warning_any = v;
        }
        if let Some(v) = notifications.get("disableWarningBadAppid").and_then(|v| v.as_bool()) {
            n.disable_warning_bad_appid = v;
        }
        if let Some(v) = notifications.get("disableWarningLocalSave").and_then(|v| v.as_bool()) {
            n.disable_warning_local_save = v;
        }
        if let Some(v) = notifications.get("uploadAchievementsIconsToGpu").and_then(|v| v.as_bool()) {
            n.upload_achievements_icons_to_gpu = v;
        }
        if let Some(v) = notifications.get("overlayAlwaysShowUserInfo").and_then(|v| v.as_bool()) {
            n.overlay_always_show_user_info = v;
        }
        if let Some(v) = notifications.get("overlayAlwaysShowFps").and_then(|v| v.as_bool()) {
            n.overlay_always_show_fps = v;
        }
        if let Some(v) = notifications.get("overlayAlwaysShowFrametime").and_then(|v| v.as_bool()) {
            n.overlay_always_show_frametime = v;
        }
        if let Some(v) = notifications.get("overlayAlwaysShowPlaytime").and_then(|v| v.as_bool()) {
            n.overlay_always_show_playtime = v;
        }
        if let Some(v) = notifications.get("notificationDurationAchievement").and_then(|v| v.as_i64()) {
            n.notification_duration_achievement = Some(v as i32);
        }
        if let Some(v) = notifications.get("notificationDurationInvitation").and_then(|v| v.as_i64()) {
            n.notification_duration_invitation = Some(v as i32);
        }
        if let Some(v) = notifications.get("notificationDurationChat").and_then(|v| v.as_i64()) {
            n.notification_duration_chat = Some(v as i32);
        }
        if let Some(v) = notifications.get("notificationDurationProgress").and_then(|v| v.as_i64()) {
            n.notification_duration_progress = Some(v as i32);
        }
    }

    // 更新 appearance
    if let Some(appearance) = config.get("appearance") {
        let a = &mut overlay_config.appearance;
        if let Some(v) = appearance.get("fontOverride").and_then(|v| v.as_str()) {
            a.font_override = Some(v.to_string());
        }
        if let Some(v) = appearance.get("fontSize").and_then(|v| v.as_i64()) {
            a.font_size = Some(v as i32);
        }
        if let Some(v) = appearance.get("fontGlyphExtraSpacingX").and_then(|v| v.as_f64()) {
            a.font_glyph_extra_spacing_x = Some(v as f32);
        }
        if let Some(v) = appearance.get("fontGlyphExtraSpacingY").and_then(|v| v.as_f64()) {
            a.font_glyph_extra_spacing_y = Some(v as f32);
        }
        if let Some(v) = appearance.get("iconSize").and_then(|v| v.as_i64()) {
            a.icon_size = Some(v as i32);
        }
        if let Some(v) = appearance.get("theme").and_then(|v| v.as_str()) {
            a.theme = v.to_string();
        }
        if let Some(v) = appearance.get("opacity").and_then(|v| v.as_f64()) {
            a.opacity = v as f32;
        }
        if let Some(v) = appearance.get("scale").and_then(|v| v.as_f64()) {
            a.scale = v as f32;
        }
        if let Some(v) = appearance.get("blur").and_then(|v| v.as_bool()) {
            a.blur = v;
        }
        if let Some(v) = appearance.get("notificationRounding").and_then(|v| v.as_i64()) {
            a.notification_rounding = Some(v as i32);
        }
        if let Some(v) = appearance.get("notificationMarginX").and_then(|v| v.as_i64()) {
            a.notification_margin_x = Some(v as i32);
        }
        if let Some(v) = appearance.get("notificationMarginY").and_then(|v| v.as_i64()) {
            a.notification_margin_y = Some(v as i32);
        }
        if let Some(v) = appearance.get("notificationR").and_then(|v| v.as_i64()) {
            a.notification_r = Some(v as i32);
        }
        if let Some(v) = appearance.get("notificationG").and_then(|v| v.as_i64()) {
            a.notification_g = Some(v as i32);
        }
        if let Some(v) = appearance.get("notificationB").and_then(|v| v.as_i64()) {
            a.notification_b = Some(v as i32);
        }
        if let Some(v) = appearance.get("notificationA").and_then(|v| v.as_i64()) {
            a.notification_a = Some(v as i32);
        }
        if let Some(v) = appearance.get("achievementUnlockDatetimeFormat").and_then(|v| v.as_str()) {
            a.achievement_unlock_datetime_format = Some(v.to_string());
        }
        if let Some(v) = appearance.get("backgroundR").and_then(|v| v.as_i64()) {
            a.background_r = Some(v as i32);
        }
        if let Some(v) = appearance.get("backgroundG").and_then(|v| v.as_i64()) {
            a.background_g = Some(v as i32);
        }
        if let Some(v) = appearance.get("backgroundB").and_then(|v| v.as_i64()) {
            a.background_b = Some(v as i32);
        }
        if let Some(v) = appearance.get("backgroundA").and_then(|v| v.as_i64()) {
            a.background_a = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementR").and_then(|v| v.as_i64()) {
            a.element_r = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementG").and_then(|v| v.as_i64()) {
            a.element_g = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementB").and_then(|v| v.as_i64()) {
            a.element_b = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementA").and_then(|v| v.as_i64()) {
            a.element_a = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementHoveredR").and_then(|v| v.as_i64()) {
            a.element_hovered_r = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementHoveredG").and_then(|v| v.as_i64()) {
            a.element_hovered_g = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementHoveredB").and_then(|v| v.as_i64()) {
            a.element_hovered_b = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementHoveredA").and_then(|v| v.as_i64()) {
            a.element_hovered_a = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementActiveR").and_then(|v| v.as_i64()) {
            a.element_active_r = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementActiveG").and_then(|v| v.as_i64()) {
            a.element_active_g = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementActiveB").and_then(|v| v.as_i64()) {
            a.element_active_b = Some(v as i32);
        }
        if let Some(v) = appearance.get("elementActiveA").and_then(|v| v.as_i64()) {
            a.element_active_a = Some(v as i32);
        }
        if let Some(v) = appearance.get("posAchievement").and_then(|v| v.as_str()) {
            a.pos_achievement = Some(v.to_string());
        }
        if let Some(v) = appearance.get("posInvitation").and_then(|v| v.as_str()) {
            a.pos_invitation = Some(v.to_string());
        }
        if let Some(v) = appearance.get("posChatMsg").and_then(|v| v.as_str()) {
            a.pos_chat_msg = Some(v.to_string());
        }
        if let Some(v) = appearance.get("statsBackgroundR").and_then(|v| v.as_i64()) {
            a.stats_background_r = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsBackgroundG").and_then(|v| v.as_i64()) {
            a.stats_background_g = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsBackgroundB").and_then(|v| v.as_i64()) {
            a.stats_background_b = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsBackgroundA").and_then(|v| v.as_i64()) {
            a.stats_background_a = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsTextR").and_then(|v| v.as_i64()) {
            a.stats_text_r = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsTextG").and_then(|v| v.as_i64()) {
            a.stats_text_g = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsTextB").and_then(|v| v.as_i64()) {
            a.stats_text_b = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsTextA").and_then(|v| v.as_i64()) {
            a.stats_text_a = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsPosX").and_then(|v| v.as_i64()) {
            a.stats_pos_x = Some(v as i32);
        }
        if let Some(v) = appearance.get("statsPosY").and_then(|v| v.as_i64()) {
            a.stats_pos_y = Some(v as i32);
        }
    }

    // 更新 performance
    if let Some(performance) = config.get("performance") {
        let p = &mut overlay_config.performance;
        if let Some(v) = performance.get("hardwareAcceleration").and_then(|v| v.as_bool()) {
            p.hardware_acceleration = v;
        }
        if let Some(v) = performance.get("fpsLimit").and_then(|v| v.as_i64()) {
            p.fps_limit = v as i32;
        }
        if let Some(v) = performance.get("lowPerformanceMode").and_then(|v| v.as_bool()) {
            p.low_performance_mode = v;
        }
    }

    // 更新 features
    if let Some(features) = config.get("features") {
        let f = &mut overlay_config.features;
        if let Some(v) = features.get("achievements").and_then(|v| v.as_bool()) {
            f.achievements = v;
        }
        if let Some(v) = features.get("friends").and_then(|v| v.as_bool()) {
            f.friends = v;
        }
        if let Some(v) = features.get("chat").and_then(|v| v.as_bool()) {
            f.chat = v;
        }
        if let Some(v) = features.get("browser").and_then(|v| v.as_bool()) {
            f.browser = v;
        }
        if let Some(v) = features.get("settings").and_then(|v| v.as_bool()) {
            f.settings = v;
        }
    }

    let overlay_config_content = overlay_config.to_ini();
    let mut file = fs::File::create(&overlay_config_path)
        .await
        .map_err(|e| format!("创建 configs.overlay.ini 失败: {}", e))?;
    file.write_all(overlay_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.overlay.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "Overlay 配置已保存".to_string(),
    })
}

/// 加载 Overlay 配置
#[tauri::command]
pub async fn load_overlay_config(
    game_path: String,
) -> Result<ConfigLoadResult<OverlayConfig>, String> {
    use tokio::fs;

    let overlay_config_path = Path::new(&game_path).join("steam_settings").join("configs.overlay.ini");

    if !overlay_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&overlay_config_path)
        .await
        .map_err(|e| format!("读取 configs.overlay.ini 失败: {}", e))?;

    // 解析 INI 内容
    let config = parse_overlay_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析 Overlay INI 内容
/// 支持 gbe_fork 格式: [overlay::general]、[overlay::hotkeys]、[overlay::notifications]、[overlay::appearance]、[overlay::performance]、[overlay::features]
pub fn parse_overlay_ini(content: &str) -> Result<OverlayConfig, String> {
    let mut config = OverlayConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            let is_one = value == "1" || value.to_lowercase() == "true";

            match current_section.as_str() {
                // 兼容旧的 [overlay] 段和新的 [overlay::general] 段
                "overlay" | "overlay::general" => {
                    match key {
                        "enable_experimental_overlay" | "enabled" => {
                            config.enable_experimental_overlay = is_one;
                        }
                        "hook_delay_sec" => {
                            config.hook_delay_sec = value.parse().ok();
                        }
                        "renderer_detector_timeout_sec" => {
                            config.renderer_detector_timeout_sec = value.parse().ok();
                        }
                        "fps_averaging_window" => {
                            config.fps_averaging_window = value.parse().ok();
                        }
                        _ => {}
                    }
                }
                // [overlay::hotkeys] 段
                "overlay::hotkeys" => {
                    match key {
                        "key_combo" | "hotkey" | "overlay_toggle" => {
                            config.overlay_hotkey = value.to_string();
                        }
                        _ => {}
                    }
                }
                // [overlay::notifications] 段
                "overlay::notifications" => {
                    let n = &mut config.notifications;
                    match key {
                        // 旧格式兼容
                        "achievement" => n.disable_achievement_notification = !is_one,
                        "friend" => n.disable_friend_notification = !is_one,
                        "message" => {} // 旧格式忽略
                        "duration" => {
                            n.notification_duration_achievement = value.parse().ok();
                            n.notification_duration_invitation = value.parse().ok();
                        }
                        "position" => {} // 旧格式忽略
                        // 新格式
                        "disable_achievement_notification" => n.disable_achievement_notification = is_one,
                        "disable_friend_notification" => n.disable_friend_notification = is_one,
                        "disable_achievement_progress" => n.disable_achievement_progress = is_one,
                        "disable_warning_any" => n.disable_warning_any = is_one,
                        "disable_warning_bad_appid" => n.disable_warning_bad_appid = is_one,
                        "disable_warning_local_save" => n.disable_warning_local_save = is_one,
                        "upload_achievements_icons_to_gpu" => n.upload_achievements_icons_to_gpu = is_one,
                        "overlay_always_show_user_info" => n.overlay_always_show_user_info = is_one,
                        "overlay_always_show_fps" => n.overlay_always_show_fps = is_one,
                        "overlay_always_show_frametime" => n.overlay_always_show_frametime = is_one,
                        "overlay_always_show_playtime" => n.overlay_always_show_playtime = is_one,
                        "notification_duration_achievement" => n.notification_duration_achievement = value.parse().ok(),
                        "notification_duration_invitation" => n.notification_duration_invitation = value.parse().ok(),
                        "notification_duration_chat" => n.notification_duration_chat = value.parse().ok(),
                        "notification_duration_progress" => n.notification_duration_progress = value.parse().ok(),
                        _ => {}
                    }
                }
                // [overlay::appearance] 段
                "overlay::appearance" => {
                    let a = &mut config.appearance;
                    match key {
                        "theme" => a.theme = value.to_string(),
                        "opacity" => a.opacity = value.parse().unwrap_or(0.95),
                        "scale" => a.scale = value.parse().unwrap_or(1.0),
                        "blur" => a.blur = is_one,
                        "Font_Override" => a.font_override = Some(value.to_string()),
                        "Font_Size" => a.font_size = value.parse().ok(),
                        "Font_Glyph_Extra_Spacing_x" => a.font_glyph_extra_spacing_x = value.parse().ok(),
                        "Font_Glyph_Extra_Spacing_y" => a.font_glyph_extra_spacing_y = value.parse().ok(),
                        "Icon_Size" => a.icon_size = value.parse().ok(),
                        "Notification_Rounding" => a.notification_rounding = value.parse().ok(),
                        "Notification_Margin_x" => a.notification_margin_x = value.parse().ok(),
                        "Notification_Margin_y" => a.notification_margin_y = value.parse().ok(),
                        "Notification_R" => a.notification_r = value.parse().ok(),
                        "Notification_G" => a.notification_g = value.parse().ok(),
                        "Notification_B" => a.notification_b = value.parse().ok(),
                        "Notification_A" => a.notification_a = value.parse().ok(),
                        "Achievement_Unlock_Datetime_Format" => a.achievement_unlock_datetime_format = Some(value.to_string()),
                        "Background_R" => a.background_r = value.parse().ok(),
                        "Background_G" => a.background_g = value.parse().ok(),
                        "Background_B" => a.background_b = value.parse().ok(),
                        "Background_A" => a.background_a = value.parse().ok(),
                        "Element_R" => a.element_r = value.parse().ok(),
                        "Element_G" => a.element_g = value.parse().ok(),
                        "Element_B" => a.element_b = value.parse().ok(),
                        "Element_A" => a.element_a = value.parse().ok(),
                        "ElementHovered_R" => a.element_hovered_r = value.parse().ok(),
                        "ElementHovered_G" => a.element_hovered_g = value.parse().ok(),
                        "ElementHovered_B" => a.element_hovered_b = value.parse().ok(),
                        "ElementHovered_A" => a.element_hovered_a = value.parse().ok(),
                        "ElementActive_R" => a.element_active_r = value.parse().ok(),
                        "ElementActive_G" => a.element_active_g = value.parse().ok(),
                        "ElementActive_B" => a.element_active_b = value.parse().ok(),
                        "ElementActive_A" => a.element_active_a = value.parse().ok(),
                        "PosAchievement" => a.pos_achievement = Some(value.to_string()),
                        "PosInvitation" => a.pos_invitation = Some(value.to_string()),
                        "PosChatMsg" => a.pos_chat_msg = Some(value.to_string()),
                        "Stats_Background_R" => a.stats_background_r = value.parse().ok(),
                        "Stats_Background_G" => a.stats_background_g = value.parse().ok(),
                        "Stats_Background_B" => a.stats_background_b = value.parse().ok(),
                        "Stats_Background_A" => a.stats_background_a = value.parse().ok(),
                        "Stats_Text_R" => a.stats_text_r = value.parse().ok(),
                        "Stats_Text_G" => a.stats_text_g = value.parse().ok(),
                        "Stats_Text_B" => a.stats_text_b = value.parse().ok(),
                        "Stats_Text_A" => a.stats_text_a = value.parse().ok(),
                        "Stats_Pos_x" => a.stats_pos_x = value.parse().ok(),
                        "Stats_Pos_y" => a.stats_pos_y = value.parse().ok(),
                        _ => {}
                    }
                }
                // [overlay::performance] 段
                "overlay::performance" => {
                    let p = &mut config.performance;
                    match key {
                        "hardware_acceleration" => p.hardware_acceleration = is_one,
                        "fps_limit" => p.fps_limit = value.parse().unwrap_or(60),
                        "low_performance_mode" => p.low_performance_mode = is_one,
                        _ => {}
                    }
                }
                // [overlay::features] 段
                "overlay::features" => {
                    let f = &mut config.features;
                    match key {
                        "achievements" => f.achievements = is_one,
                        "friends" => f.friends = is_one,
                        "chat" => f.chat = is_one,
                        "browser" => f.browser = is_one,
                        "settings" => f.settings = is_one,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 主配置
// ============================================

/// 保存主配置（接收原始 INI 字符串，直接写入文件）
/// 前端发送格式: { mainIni: "[main]\nforce_lan_only = 1\n..." }
#[tauri::command]
pub async fn save_main_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let main_ini = config
        .get("mainIni")
        .and_then(|v| v.as_str())
        .ok_or("缺少 mainIni 字段")?;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let main_config_path = steam_settings_dir.join("configs.main.ini");

    let mut file = fs::File::create(&main_config_path)
        .await
        .map_err(|e| format!("创建 configs.main.ini 失败: {}", e))?;
    file.write_all(main_ini.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.main.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "主配置已保存".to_string(),
    })
}

/// 加载主配置（返回原始 INI 字符串，供前端文本编辑器使用）
/// 当文件不存在时，返回默认 INI 内容，确保单独窗口与完整管理器的默认值一致
#[tauri::command]
pub async fn load_main_config(
    game_path: String,
) -> Result<serde_json::Value, String> {
    use tokio::fs;

    let main_config_path = Path::new(&game_path).join("steam_settings").join("configs.main.ini");

    if !main_config_path.exists() {
        let default_content = MainConfig::default_config().to_ini();
        return Ok(serde_json::json!({
            "exists": false,
            "content": default_content
        }));
    }

    let content = fs::read_to_string(&main_config_path)
        .await
        .map_err(|e| format!("读取 configs.main.ini 失败: {}", e))?;

    Ok(serde_json::json!({
        "exists": true,
        "content": content
    }))
}

/// 解析主配置 INI 内容
#[allow(dead_code)]
pub fn parse_main_ini(content: &str) -> Result<MainConfig, String> {
    let mut config = MainConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match current_section.as_str() {
                "main::general" => {
                    match key {
                        "new_app_ticket" => config.new_app_ticket = value == "1",
                        "gc_token" => config.gc_token = value == "1",
                        "block_unknown_clients" => config.block_unknown_clients = value == "1",
                        "steam_deck" => config.steam_deck = value == "1",
                        "enable_account_avatar" => config.enable_account_avatar = value == "1",
                        "enable_voice_chat" => config.enable_voice_chat = value == "1",
                        "immediate_gameserver_stats" => config.immediate_gameserver_stats = value == "1",
                        "matchmaking_server_list_actual_type" => config.matchmaking_server_list_actual_type = value == "1",
                        "matchmaking_server_details_via_source_query" => config.matchmaking_server_details_via_source_query = value == "1",
                        "crash_printer_location" => config.crash_printer_location = Some(value.to_string()),
                        _ => {}
                    }
                }
                "main::stats" => {
                    match key {
                        "disable_leaderboards_create_unknown" => config.disable_leaderboards_create_unknown = value == "1",
                        "allow_unknown_stats" => config.allow_unknown_stats = value == "1",
                        "stat_achievement_progress_functionality" => config.stat_achievement_progress_functionality = value == "1",
                        "save_only_higher_stat_achievement_progress" => config.save_only_higher_stat_achievement_progress = value == "1",
                        "paginated_achievements_icons" => config.paginated_achievements_icons = value.parse().unwrap_or(10),
                        "record_playtime" => config.record_playtime = value == "1",
                        _ => {}
                    }
                }
                "main::connectivity" => {
                    match key {
                        "disable_lan_only" => config.disable_lan_only = value == "1",
                        "disable_networking" => config.disable_networking = value == "1",
                        "listen_port" => config.listen_port = value.parse().unwrap_or(47584),
                        "offline" => config.offline = value == "1",
                        "disable_sharing_stats_with_gameserver" => config.disable_sharing_stats_with_gameserver = value == "1",
                        "disable_source_query" => config.disable_source_query = value == "1",
                        "share_leaderboards_over_network" => config.share_leaderboards_over_network = value == "1",
                        "disable_lobby_creation" => config.disable_lobby_creation = value == "1",
                        "download_steamhttp_requests" => config.download_steamhttp_requests = value == "1",
                        _ => {}
                    }
                }
                "main::misc" => {
                    match key {
                        "achievements_bypass" => config.achievements_bypass = value == "1",
                        "force_steamhttp_success" => config.force_steamhttp_success = value == "1",
                        "disable_steamoverlaygameid_env_var" => config.disable_steamoverlaygameid_env_var = value == "1",
                        "enable_steam_preowned_ids" => config.enable_steam_preowned_ids = value == "1",
                        "steam_game_stats_reports_dir" => config.steam_game_stats_reports_dir = Some(value.to_string()),
                        "free_weekend" => config.free_weekend = value == "1",
                        "use_32bit_inventory_item_ids" => config.use_32bit_inventory_item_ids = value == "1",
                        _ => {}
                    }
                }
                "extra_dlls" => {
                    if key.starts_with("dll") {
                        config.extra_dlls.push(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// 用户配置
// ============================================

/// 保存用户配置
#[tauri::command]
pub async fn save_user_config(
    game_path: String,
    config: UserConfig,
) -> Result<ConfigSaveResult, String> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    let user_config_path = steam_settings_dir.join("configs.user.ini");
    let user_config_content = config.to_ini();

    let mut file = fs::File::create(&user_config_path)
        .await
        .map_err(|e| format!("创建 configs.user.ini 失败: {}", e))?;
    file.write_all(user_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.user.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "用户配置已保存".to_string(),
    })
}

/// 加载用户配置
#[tauri::command]
pub async fn load_user_config(
    game_path: String,
) -> Result<ConfigLoadResult<UserConfig>, String> {
    use tokio::fs;

    let user_config_path = Path::new(&game_path).join("steam_settings").join("configs.user.ini");

    if !user_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&user_config_path)
        .await
        .map_err(|e| format!("读取 configs.user.ini 失败: {}", e))?;

    let config = parse_user_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析用户配置 INI 内容
/// 支持 gbe_fork 格式: [user::general] 和 [user::saves] 两个段
pub fn parse_user_ini(content: &str) -> Result<UserConfig, String> {
    let mut config = UserConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // 兼容旧的 [user] 段和新的 [user::general] 段
            match current_section.as_str() {
                "user" | "user::general" => {
                    match key {
                        "username" | "account_name" => config.username = value.to_string(),
                        "account_steamid" => config.account_steamid = Some(value.to_string()),
                        "language" => config.language = value.to_string(),
                        "ip_country" => config.ip_country = Some(value.to_string()),
                        "ticket" => config.ticket = Some(value.to_string()),
                        "alt_steamid" => config.alt_steamid = Some(value.to_string()),
                        "alt_steamid_count" => config.alt_steamid_count = value.parse().ok(),
                        _ => {}
                    }
                }
                "user::saves" => {
                    match key {
                        "saves_folder_name" => {
                            if !value.is_empty() {
                                config.saves_folder_name = Some(value.to_string());
                            }
                        }
                        "local_save_path" => {
                            if !value.is_empty() {
                                config.local_save_path = Some(value.to_string());
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}

// ============================================
// ============================================
// 应用配置
// ============================================

/// 保存应用配置
/// 接收前端简化格式: { branchName, appPaths, dlcs: { unlockAll, dlcList, depotIds, dlcPaths } }
#[tauri::command]
pub async fn save_app_config(
    game_path: String,
    config: serde_json::Value,
) -> Result<ConfigSaveResult, String> {
    use crate::models::steam_config::*;
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let steam_settings_dir = Path::new(&game_path).join("steam_settings");
    fs::create_dir_all(&steam_settings_dir)
        .await
        .map_err(|e| format!("创建 steam_settings 目录失败: {}", e))?;

    // 读取现有的 app 配置，保留已有数据
    let app_config_path = steam_settings_dir.join("configs.app.ini");
    let mut app_config = if app_config_path.exists() {
        let content = fs::read_to_string(&app_config_path)
            .await
            .map_err(|e| format!("读取 configs.app.ini 失败: {}", e))?;
        parse_app_ini(&content)?
    } else {
        SteamAppConfig::default_config()
    };

    // 更新分支名称
    if let Some(branch_name) = config.get("branchName").and_then(|v| v.as_str()) {
        app_config.branch_name = branch_name.to_string();
    }

    // 更新 Beta 分支标志
    if let Some(is_beta) = config.get("isBetaBranch").and_then(|v| v.as_bool()) {
        app_config.is_beta_branch = is_beta;
    }

    // 更新 Steam Input 控制器配置
    if let Some(controller) = config.get("controller") {
        if let Some(steam_input) = controller.get("steamInput").and_then(|v| v.as_bool()) {
            app_config.controller.steam_input = Some(steam_input);
        }
        if let Some(controller_type) = controller.get("type").and_then(|v| v.as_str()) {
            app_config.controller.r#type = Some(controller_type.to_string());
        }
    }

    // 更新云存档配置
    if let Some(cloud_saves) = config.get("cloudSaves") {
        if let Some(enabled) = cloud_saves.get("enabled").and_then(|v| v.as_bool()) {
            app_config.cloud_saves.enabled = Some(enabled);
        }
        if let Some(create_default) = cloud_saves.get("createDefaultDir").and_then(|v| v.as_bool()) {
            app_config.cloud_saves.create_default_dir = Some(create_default);
        }
        if let Some(create_specific) = cloud_saves.get("createSpecificDirs").and_then(|v| v.as_bool()) {
            app_config.cloud_saves.create_specific_dirs = Some(create_specific);
        }
        // Windows 云存档路径
        if let Some(win_dirs) = cloud_saves.get("windowsDirs").and_then(|v| v.as_array()) {
            app_config.cloud_saves.windows_dirs.clear();
            for dir in win_dirs {
                if let Some(dir_str) = dir.as_str() {
                    app_config.cloud_saves.windows_dirs.push(dir_str.to_string());
                }
            }
        }
        // Linux 云存档路径
        if let Some(linux_dirs) = cloud_saves.get("linuxDirs").and_then(|v| v.as_array()) {
            app_config.cloud_saves.linux_dirs.clear();
            for dir in linux_dirs {
                if let Some(dir_str) = dir.as_str() {
                    app_config.cloud_saves.linux_dirs.push(dir_str.to_string());
                }
            }
        }
    }

    // 更新 DLC 配置
    if let Some(dlcs) = config.get("dlcs") {
        // unlockAll
        if let Some(unlock_all) = dlcs.get("unlockAll").and_then(|v| v.as_bool()) {
            app_config.dlcs.unlock_all = unlock_all;
        }

        // dlcList (每行一个 ID 或 appid=Name 的字符串)
        if let Some(dlc_list_str) = dlcs.get("dlcList").and_then(|v| v.as_str()) {
            // 保存原始行到 dlc_list（保留名称格式）
            app_config.dlcs.dlc_list = dlc_list_str
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            // 同时更新 individual_dlcs（用于 to_ini 输出）
            app_config.dlcs.individual_dlcs.clear();
            for line in &app_config.dlcs.dlc_list {
                // 解析 "appid=Name" 或 "appid" 格式
                let (app_id, name) = if let Some(eq_pos) = line.find('=') {
                    let id = line[..eq_pos].trim().to_string();
                    let nm = line[eq_pos + 1..].trim().to_string();
                    (id, nm)
                } else {
                    (line.clone(), format!("DLC {}", line))
                };
                if !app_id.is_empty() {
                    app_config.dlcs.individual_dlcs.push(IndividualDlc {
                        app_id,
                        name,
                        enabled: true,
                    });
                }
            }
        }

        // depotIds (每行一个 ID 的字符串)
        if let Some(depot_ids_str) = dlcs.get("depotIds").and_then(|v| v.as_str()) {
            app_config.dlcs.depot_ids = depot_ids_str
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // dlcPaths (格式: appid=相对路径，每行一个)
        if let Some(dlc_paths_str) = dlcs.get("dlcPaths").and_then(|v| v.as_str()) {
            app_config.dlcs.dlc_paths.clear();
            for line in dlc_paths_str.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some(eq_pos) = line.find('=') {
                    let app_id = line[..eq_pos].trim().to_string();
                    let path = line[eq_pos + 1..].trim().to_string();
                    if !app_id.is_empty() && !path.is_empty() {
                        app_config.dlcs.dlc_paths.insert(app_id, path);
                    }
                }
            }
        }
    }

    let app_config_content = app_config.to_ini();
    let mut file = fs::File::create(&app_config_path)
        .await
        .map_err(|e| format!("创建 configs.app.ini 失败: {}", e))?;
    file.write_all(app_config_content.as_bytes())
        .await
        .map_err(|e| format!("写入 configs.app.ini 失败: {}", e))?;

    Ok(ConfigSaveResult {
        success: true,
        message: "应用配置已保存".to_string(),
    })
}

/// 加载应用配置
#[tauri::command]
pub async fn load_app_config(
    game_path: String,
) -> Result<ConfigLoadResult<SteamAppConfig>, String> {
    use tokio::fs;

    let app_config_path = Path::new(&game_path).join("steam_settings").join("configs.app.ini");

    if !app_config_path.exists() {
        return Ok(ConfigLoadResult {
            exists: false,
            config: None,
        });
    }

    let content = fs::read_to_string(&app_config_path)
        .await
        .map_err(|e| format!("读取 configs.app.ini 失败: {}", e))?;

    let config = parse_app_ini(&content)?;

    Ok(ConfigLoadResult {
        exists: true,
        config: Some(config),
    })
}

/// 解析应用配置 INI 内容（公开版本，供 DLC 配置模块使用）
pub fn parse_app_ini(content: &str) -> Result<SteamAppConfig, String> {
    parse_app_ini_inner(content)
}

/// 解析应用配置 INI 内容（内部实现）
fn parse_app_ini_inner(content: &str) -> Result<SteamAppConfig, String> {
    let mut config = SteamAppConfig::default_config();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            let is_one = value == "1" || value.to_lowercase() == "true";

            match current_section.as_str() {
                // 兼容旧的 [app] 段和新的 [app::general] 段
                "app" | "app::general" => {
                    match key {
                        "branch_name" => config.branch_name = value.to_string(),
                        "is_beta_branch" => config.is_beta_branch = is_one,
                        _ => {}
                    }
                }
                "app::paths" => {
                    if !value.is_empty() && !value.starts_with('#') {
                        config.app_paths.insert(key.to_string(), value.to_string());
                    }
                }
                "app::dlcs" => {
                    if key == "unlock_all" {
                        config.dlcs.unlock_all = is_one;
                    } else if !key.starts_with('#') && !key.is_empty() {
                        // gbe_fork 格式: "appid = DLC Name" (value 就是名称)
                        // 或者 "appid = " (空名称)
                        let name = if value.is_empty() {
                            format!("DLC {}", key)
                        } else {
                            value.to_string()
                        };
                        config.dlcs.individual_dlcs.push(IndividualDlc {
                            app_id: key.to_string(),
                            name,
                            enabled: true, // gbe_fork 格式没有 enabled 字段，列出的都是启用的
                        });
                    }
                }
                // [app::controller] 段
                "app::controller" => {
                    match key {
                        "steam_input" => config.controller.steam_input = Some(is_one),
                        "type" => config.controller.r#type = Some(value.to_string()),
                        _ => {}
                    }
                }
                // [app::cloud_save::general] 段
                "app::cloud_save::general" => {
                    match key {
                        "enabled" => config.cloud_saves.enabled = Some(is_one),
                        "create_default_dir" => config.cloud_saves.create_default_dir = Some(is_one),
                        "create_specific_dirs" => config.cloud_saves.create_specific_dirs = Some(is_one),
                        _ => {}
                    }
                }
                // [app::cloud_save::win] 段
                "app::cloud_save::win" => {
                    if !value.is_empty() && !value.starts_with('#') {
                        config.cloud_saves.windows_dirs.push(value.to_string());
                    }
                }
                // [app::cloud_save::linux] 段
                "app::cloud_save::linux" => {
                    if !value.is_empty() && !value.starts_with('#') {
                        config.cloud_saves.linux_dirs.push(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    Ok(config)
}
