/**
 * config.api.ts - 配置管理相关的API调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { AppConfig } from '../types/config.types'

/**
 * 获取应用配置
 */
export async function getConfig(): Promise<AppConfig> {
  return invoke<AppConfig>('get_config')
}

/**
 * 保存应用配置
 */
export async function saveConfig(config: AppConfig): Promise<void> {
  return invoke<void>('save_config', { config })
}

/**
 * 重置配置为默认值
 */
export async function resetConfig(): Promise<AppConfig> {
  return invoke<AppConfig>('reset_config')
}
