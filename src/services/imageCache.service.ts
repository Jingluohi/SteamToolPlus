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

/** 缓存条目结构 */
interface CacheEntry {
  url: string
  lastAccessed: number
}

// 图片缓存映射表：key = gameId, value = 缓存条目
const coverImageCache = new Map<string, CacheEntry>()
const libraryImageCache = new Map<string, CacheEntry>()

/**
 * LRU 淘汰：当缓存超过最大限制时，移除最久未访问的条目
 */
function evictLRU(cache: Map<string, CacheEntry>): void {
  if (cache.size <= MAX_CACHE_SIZE) return

  let oldestKey: string | null = null
  let oldestTime = Infinity

  for (const [key, entry] of cache.entries()) {
    if (entry.lastAccessed < oldestTime) {
      oldestTime = entry.lastAccessed
      oldestKey = key
    }
  }

  if (oldestKey) {
    cache.delete(oldestKey)
  }
}

/**
 * 获取缓存条目（自动更新访问时间）
 */
function getCachedEntry(cache: Map<string, CacheEntry>, key: string): string | null {
  const entry = cache.get(key)
  if (!entry) return null

  // 更新访问时间
  entry.lastAccessed = Date.now()
  return entry.url
}

/**
 * 设置缓存条目（带 LRU 淘汰）
 */
function setCacheEntry(cache: Map<string, CacheEntry>, key: string, url: string): void {
  evictLRU(cache)
  cache.set(key, {
    url,
    lastAccessed: Date.now()
  })
}

/**
 * 通用图片缓存获取函数
 */
async function getCachedImage(
  cache: Map<string, CacheEntry>,
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

  if (!filePath) {
    return ''
  }

  // 转换为 asset URL 并缓存
  const assetUrl = convertFileSrc(filePath)
  setCacheEntry(cache, gameId, assetUrl)

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
