/**
 * extension.api.ts - 扩展系统相关的API调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { Extension, ExtensionListResponse } from '../types/extension.types'

/**
 * 获取已安装的扩展列表
 */
export async function getExtensions(): Promise<ExtensionListResponse> {
  return invoke<ExtensionListResponse>('get_extensions')
}

/**
 * 安装扩展
 * @param packagePath - 扩展包文件路径(.7z格式)
 */
export async function installExtension(packagePath: string): Promise<Extension> {
  return invoke<Extension>('install_extension', { packagePath })
}

/**
 * 卸载扩展
 * @param extensionId - 扩展ID
 */
export async function uninstallExtension(extensionId: string): Promise<void> {
  return invoke<void>('uninstall_extension', { extensionId })
}

/**
 * 启用/禁用扩展
 * @param extensionId - 扩展ID
 * @param enabled - 是否启用
 * @returns 更新后的扩展信息
 */
export async function toggleExtension(extensionId: string, enabled: boolean): Promise<Extension> {
  return invoke<Extension>('toggle_extension', { extensionId, enabled })
}

/**
 * 重新加载扩展
 * @param extensionId - 扩展ID
 */
export async function reloadExtension(extensionId: string): Promise<Extension> {
  return invoke<Extension>('reload_extension', { extensionId })
}
