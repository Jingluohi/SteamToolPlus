/**
 * imageCache.service.ts - 全局图片缓存服务
 * 
 * 解决多个页面/组件同时加载同一张图片时的资源竞争问题
 * 通过缓存 convertFileSrc 生成的 URL，避免重复转换和内存浪费
 */

import { convertFileSrc } from '@tauri-apps/api/core'
import { getGameCoverImage, getGameLibraryImage } from '../api/game.api'

// 图片缓存映射表：key = gameId, value = 转换后的 asset URL
const coverImageCache = new Map<string, string>()
const libraryImageCache = new Map<string, string>()

/**
 * 获取游戏封面图片 URL（带缓存）
 * @param gameId 游戏ID
 * @returns 转换后的 asset URL，如果图片不存在返回空字符串
 */
export async function getCachedCoverImage(gameId: string): Promise<string> {
  // 先检查缓存
  if (coverImageCache.has(gameId)) {
    return coverImageCache.get(gameId)!
  }

  // 缓存未命中，调用后端获取文件路径
  const filePath = await getGameCoverImage(gameId)
  
  if (!filePath) {
    return ''
  }

  // 转换为 asset URL 并缓存
  const assetUrl = convertFileSrc(filePath)
  coverImageCache.set(gameId, assetUrl)
  
  return assetUrl
}

/**
 * 获取游戏库背景图片 URL（带缓存）
 * @param gameId 游戏ID
 * @returns 转换后的 asset URL，如果图片不存在返回空字符串
 */
export async function getCachedLibraryImage(gameId: string): Promise<string> {
  // 先检查缓存
  if (libraryImageCache.has(gameId)) {
    return libraryImageCache.get(gameId)!
  }

  // 缓存未命中，调用后端获取文件路径
  const filePath = await getGameLibraryImage(gameId)
  
  if (!filePath) {
    return ''
  }

  // 转换为 asset URL 并缓存
  const assetUrl = convertFileSrc(filePath)
  libraryImageCache.set(gameId, assetUrl)
  
  return assetUrl
}

/**
 * 预加载多个游戏的封面图片到缓存
 * @param gameIds 游戏ID数组
 */
export async function preloadCoverImages(gameIds: string[]): Promise<void> {
  const promises = gameIds.map(id => getCachedCoverImage(id))
  await Promise.all(promises)
}

/**
 * 预加载多个游戏的库背景图片到缓存
 * @param gameIds 游戏ID数组
 */
export async function preloadLibraryImages(gameIds: string[]): Promise<void> {
  const promises = gameIds.map(id => getCachedLibraryImage(id))
  await Promise.all(promises)
}

/**
 * 清空封面图片缓存
 */
export function clearCoverImageCache(): void {
  coverImageCache.clear()
}

/**
 * 清空库背景图片缓存
 */
export function clearLibraryImageCache(): void {
  libraryImageCache.clear()
}

/**
 * 获取缓存统计信息（用于调试）
 */
export function getCacheStats(): { cover: number; library: number } {
  return {
    cover: coverImageCache.size,
    library: libraryImageCache.size
  }
}
