/**
 * 背景图片管理API
 * 提供背景图片的上传、删除、配置管理等功能
 */

import { invoke } from '@tauri-apps/api/core'
import { convertFileSrc } from '@tauri-apps/api/core'
import type { BackgroundConfig, BackgroundFile, PageType, ThemeMode } from '../types/background.types'

/**
 * 获取背景配置
 */
export async function getBackgroundConfig(): Promise<BackgroundConfig> {
  return invoke<BackgroundConfig>('get_background_config')
}

/**
 * 保存背景配置
 */
export async function saveBackgroundConfig(config: BackgroundConfig): Promise<void> {
  return invoke<void>('save_background_config', { config })
}

/**
 * 添加背景文件（图片）
 * @param filePath 文件路径
 */
export async function addBackgroundFile(filePath: string): Promise<BackgroundFile> {
  return invoke<BackgroundFile>('add_background_file', { filePath })
}

/**
 * 删除背景文件
 * @param fileId 文件ID
 */
export async function removeBackgroundFile(fileId: string): Promise<void> {
  return invoke<void>('remove_background_file', { fileId })
}

/**
 * 获取背景文件的完整URL
 * @param filePath 文件路径
 */
export async function getBackgroundFileUrl(filePath: string): Promise<string> {
  return convertFileSrc(filePath)
}

/**
 * 获取指定页面的背景配置
 * @param config 背景配置
 * @param pageType 页面类型
 */
export function getPageBackgroundConfig(config: BackgroundConfig, pageType: PageType) {
  return config.pageConfigs.find(p => p.pageType === pageType)
}

/**
 * 获取当前系统主题模式
 */
export function getSystemThemeMode(): ThemeMode {
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

/**
 * 获取页面启用的背景文件列表
 * @param config 背景配置
 * @param pageType 页面类型
 * @param themeMode 主题模式，不传则自动检测系统主题
 */
export function getPageBackgroundFiles(
  config: BackgroundConfig,
  pageType: PageType,
  themeMode?: ThemeMode
): BackgroundFile[] {
  const pageConfig = getPageBackgroundConfig(config, pageType)
  if (!pageConfig || !pageConfig.enabled) {
    return []
  }

  // 确定使用哪个主题模式的文件列表
  const mode = themeMode || getSystemThemeMode()
  const fileIds = mode === 'light' ? pageConfig.lightFileIds : pageConfig.darkFileIds

  // 如果新格式的文件列表为空，尝试兼容旧格式的 fileIds
  const targetFileIds = fileIds.length > 0 ? fileIds : pageConfig.fileIds

  return targetFileIds
    .map(id => config.files.find(f => f.id === id))
    .filter((f): f is BackgroundFile => f !== undefined && f.enabled)
    .sort((a, b) => a.order - b.order)
}

/**
 * 获取当前应该显示的背景文件URL
 * @param config 背景配置
 * @param pageType 页面类型
 * @param currentIndex 当前轮播索引
 */
export async function getCurrentBackgroundUrl(
  config: BackgroundConfig,
  pageType: PageType,
  currentIndex: number = 0
): Promise<string | null> {
  const pageFiles = getPageBackgroundFiles(config, pageType)

  if (pageFiles.length === 0) {
    return null
  }

  const pageConfig = getPageBackgroundConfig(config, pageType)
  let targetFile: BackgroundFile

  if (pageConfig?.mode === 'single') {
    // 单张模式
    targetFile = pageFiles.find(f => f.id === pageConfig.currentFileId) || pageFiles[0]
  } else {
    // 轮播或随机模式
    const index = currentIndex % pageFiles.length
    targetFile = pageFiles[index]
  }

  try {
    return await getBackgroundFileUrl(targetFile.path)
  } catch (err) {
    return null
  }
}

/**
 * 获取下一张背景图片的索引
 * @param config 背景配置
 * @param pageType 页面类型
 * @param currentIndex 当前索引
 */
export function getNextBackgroundIndex(
  config: BackgroundConfig,
  pageType: PageType,
  currentIndex: number
): number {
  const pageFiles = getPageBackgroundFiles(config, pageType)
  const enabledCount = pageFiles.length

  if (enabledCount === 0) {
    return 0
  }

  const pageConfig = getPageBackgroundConfig(config, pageType)

  if (pageConfig?.mode === 'random') {
    // 随机模式
    let nextIndex = Math.floor(Math.random() * enabledCount)
    while (nextIndex === currentIndex && enabledCount > 1) {
      nextIndex = Math.floor(Math.random() * enabledCount)
    }
    return nextIndex
  } else {
    // 轮播模式
    return (currentIndex + 1) % enabledCount
  }
}

/**
 * 扫描背景文件目录，自动添加新文件
 */
export async function scanBackgroundFiles(): Promise<BackgroundFile[]> {
  return invoke<BackgroundFile[]>('scan_background_files')
}

// 兼容旧API的别名
export const addBackgroundImage = addBackgroundFile
export const removeBackgroundImage = removeBackgroundFile
export const getBackgroundImageUrl = getBackgroundFileUrl
export const scanBackgroundImages = scanBackgroundFiles
