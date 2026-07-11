/**
 * imageCache.service.ts - 全局图片缓存服务
 *
 * 解决多个页面/组件同时加载同一张图片时的资源竞争问题
 * 通过缓存 convertFileSrc 生成的 URL，避免重复转换和内存浪费
 * 采用 LRU 淘汰策略，限制最大缓存数量防止内存无限增长
 */

import { convertFileSrc } from '@tauri-apps/api/core'
import { getGameCoverImage, getGameLibraryImage } from '../api/game.api'
import { ref } from 'vue'

/** 最大缓存条目数 */
const MAX_CACHE_SIZE = 100

/** 刷新信号 - 当窗口从隐藏恢复时触发所有图片组件重新加载 */
export const imageRefreshSignal = ref(0)

// 图片缓存映射表：key = gameId, value = asset URL
// 利用 JavaScript Map 的插入顺序实现 O(1) 的 LRU：
// - 访问命中时先 delete 再 set，将条目移到最新位置
// - 超出容量时删除 Map 的第一个 key（最久未访问）
const coverImageCache = new Map<string, string>()
const libraryImageCache = new Map<string, string>()

/**
 * 获取缓存条目（LRU：命中后移动到最新位置）
 * 时间复杂度 O(1)
 */
function getCachedEntry(cache: Map<string, string>, key: string): string | null {
  const url = cache.get(key)
  if (url === undefined) return null

  // 重新设置以更新其在 Map 中的顺序（最新使用）
  cache.delete(key)
  cache.set(key, url)
  return url
}

/**
 * 设置缓存条目（带 O(1) LRU 淘汰）
 */
function setCacheEntry(cache: Map<string, string>, key: string, url: string): void {
  // 如果已存在，先删除以更新顺序
  if (cache.has(key)) {
    cache.delete(key)
  } else if (cache.size >= MAX_CACHE_SIZE) {
    // 淘汰最久未访问的条目（Map 的第一个 key）
    const oldestKey = cache.keys().next().value
    if (oldestKey !== undefined) {
      cache.delete(oldestKey)
    }
  }

  cache.set(key, url)
}

/**
 * 通用图片缓存获取函数
 */
async function getCachedImage(
  cache: Map<string, string>,
  fetchPath: (id: string) => Promise<string>,
  gameId: string
): Promise<string> {
  // 先检查缓存
  const cached = getCachedEntry(cache, gameId)
  if (cached) {
    return cached
  }

  // 缓存未命中，调用后端获取文件路径
  const filePath = await fetchPath(gameId)

  // 图片不存在时直接返回空字符串，不入缓存，避免后续补图后仍显示占位
  if (!filePath) {
    return ''
  }

  // 转换为 asset URL 并缓存
  const assetUrl = convertFileSrc(filePath)
  if (assetUrl) {
    setCacheEntry(cache, gameId, assetUrl)
  }

  return assetUrl
}

/**
 * 获取游戏封面图片 URL（带缓存）
 * @param gameId 游戏ID
 * @returns 转换后的 asset URL，如果图片不存在返回空字符串
 */
export async function getCachedCoverImage(gameId: string): Promise<string> {
  return getCachedImage(coverImageCache, getGameCoverImage, gameId)
}

/**
 * 获取游戏库背景图片 URL（带缓存）
 * @param gameId 游戏ID
 * @returns 转换后的 asset URL，如果图片不存在返回空字符串
 */
export async function getCachedLibraryImage(gameId: string): Promise<string> {
  return getCachedImage(libraryImageCache, getGameLibraryImage, gameId)
}

/**
 * 清空所有图片缓存
 */
export function clearAllImageCaches(): void {
  coverImageCache.clear()
  libraryImageCache.clear()
}

/**
 * 触发图片刷新信号
 * 通知所有使用图片缓存的组件重新加载图片
 */
export function triggerImageRefresh(): void {
  imageRefreshSignal.value++
}
