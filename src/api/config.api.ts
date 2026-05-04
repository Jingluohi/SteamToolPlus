/**
 * config.api.ts - 配置管理相关的API调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { AppConfig, ThemeConfig, WindowConfig, GameDirConfig, LaunchConfig } from '../types/config.types'

/**
 * 获取应用配置
 */
export async function getConfig(): Promise<AppConfig> {
  return invoke<AppConfig>('get_config')
}

/**
 * 更新应用配置（部分更新）
 */
export async function updateConfig(request: {
  window?: WindowConfig
  theme?: ThemeConfig
  gameDirs?: GameDirConfig
  launch?: LaunchConfig
}): Promise<AppConfig> {
  // 后端使用 camelCase，直接发送
  return invoke<AppConfig>('update_config', { request })
}

/**
 * 保存完整应用配置
 */
export async function saveConfig(config: AppConfig): Promise<void> {
  // 使用 update_config 命令，将所有配置作为 request 发送
  // 后端使用 camelCase，直接发送
  await invoke<AppConfig>('update_config', { request: config })
}

/**
 * 重置配置为默认值
 */
export async function resetConfig(): Promise<AppConfig> {
  return invoke<AppConfig>('reset_config')
}
