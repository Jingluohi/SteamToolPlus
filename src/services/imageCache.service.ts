/**
 * imageCache.service.ts - 全局图片缓存服务
 *
 * 管理两类图片资源的加载与生命周期：
 * 1. 游戏封面：后端返回 data URL（data:image/jpeg;base64,...），
 *    data URL 不走 HTTP 缓存（asset:// 会被 WebView2 当作 HTTP 资源缓存导致解码位图累积），
 *    用 LRU 缓存 data URL 字符串避免翻回旧页重复 IPC
 * 2. 游戏库背景图：走 asset:// 协议 + convertFileSrc，使用 LRU 缓存 URL
 *
 * 另提供图片刷新/释放信号，用于窗口隐藏（释放）与恢复（重载）时的统一调度
 */

import { convertFileSrc } from '@tauri-apps/api/core'
import { getGameLibraryImage, getGameCoverBytes } from '../api/game.api'
import { ref } from 'vue'

/** 最大缓存条目数 */
const MAX_CACHE_SIZE = 100

/** 刷新信号 - 当窗口从隐藏恢复时触发所有图片组件清空后重新加载 */
export const imageRefreshSignal = ref(0)

/** 释放信号 - 当窗口隐藏到托盘时触发所有图片组件清空（不重新加载） */
export const imageReleaseSignal = ref(0)

// 库背景图 asset URL 缓存：key = gameId, value = asset URL（LRU 淘汰）
const libraryImageCache = new Map<string, string>()

// 封面 data URL 缓存：key = gameId, value = data URL（LRU 淘汰）
// 缓存 data URL 字符串，避免翻回旧页时重复 IPC 读取文件
const coverUrlCache = new Map<string, string>()

/**
 * 获取缓存条目（LRU：命中后移动到最新位置）
 * 时间复杂度 O(1)
 */
function getCachedEntry<T>(cache: Map<string, T>, key: string): T | null {
  const value = cache.get(key)
  if (value === undefined) return null

  // 重新设置以更新其在 Map 中的顺序（最新使用）
  cache.delete(key)
  cache.set(key, value)
  return value
}

/**
 * 设置缓存条目（带 O(1) LRU 淘汰）
 */
function setCacheEntry<T>(cache: Map<string, T>, key: string, value: T): void {
  if (cache.has(key)) {
    cache.delete(key)
  } else if (cache.size >= MAX_CACHE_SIZE) {
    // 淘汰最久未访问的条目（Map 的第一个 key）
    const oldestKey = cache.keys().next().value
    if (oldestKey !== undefined) {
      cache.delete(oldestKey)
    }
  }

  cache.set(key, value)
}

/**
 * 通用图片缓存获取函数（供库背景图使用）
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
 * 获取游戏封面图片 data URL（带 LRU 缓存）
 * data URL 不走 HTTP 缓存，翻页/卸载后解码位图可被回收，避免内存累积
 * @param gameId 游戏ID
 * @returns data URL，图片不存在时返回空字符串
 */
export async function getCachedCoverImage(gameId: string): Promise<string> {
  // 优先从缓存获取
  const cached = getCachedEntry(coverUrlCache, gameId)
  if (cached) {
    return cached
  }

  // 缓存未命中，从后端获取 data URL
  const dataUrl = await getGameCoverBytes(gameId)
  if (!dataUrl) {
    return ''
  }

  setCacheEntry(coverUrlCache, gameId, dataUrl)
  return dataUrl
}

/**
 * 获取游戏库背景图片 URL（带缓存）
 * @param gameId 游戏ID
 */
export async function getCachedLibraryImage(gameId: string): Promise<string> {
  return getCachedImage(libraryImageCache, getGameLibraryImage, gameId)
}

/**
 * 清空所有图片缓存（窗口隐藏到托盘时调用）
 */
export function clearAllImageCaches(): void {
  libraryImageCache.clear()
  coverUrlCache.clear()
}

/**
 * 触发图片刷新信号
 * 通知所有使用图片缓存的组件清空后重新加载图片（用于窗口恢复显示）
 */
export function triggerImageRefresh(): void {
  imageRefreshSignal.value++
}

/**
 * 触发图片释放信号
 * 通知所有使用图片缓存的组件清空图片引用（不重新加载），用于窗口隐藏到托盘时释放显存
 */
export function triggerImageRelease(): void {
  imageReleaseSignal.value++
}