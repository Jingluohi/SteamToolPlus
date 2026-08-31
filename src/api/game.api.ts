/**
 * game.api.ts - 游戏库相关的API调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { GameConfigData, GameListResponse } from '../types/game.types'

/**
 * 获取游戏列表
 */
export async function getGames(): Promise<GameListResponse> {
  return invoke<GameListResponse>('get_games')
}

/**
 * 启动游戏
 */
export async function launchGame(gameId: string): Promise<void> {
  return invoke<void>('launch_game', { gameId })
}

/**
 * 获取游戏封面图片路径
 */
export async function getGameCoverImage(gameId: string): Promise<string> {
  return invoke<string>('get_game_cover_image', { gameId })
}

/**
 * 获取游戏封面图片（data URL）
 * 后端返回 "data:image/jpeg;base64,xxx" 格式，前端直接赋给 img.src
 */
export async function getGameCoverBytes(gameId: string): Promise<string> {
  return invoke<string>('get_game_cover_bytes', { gameId })
}

/**
 * 获取游戏库背景图片路径
 * 从 resources/pic/库 目录加载
 */
export async function getGameLibraryImage(gameId: string): Promise<string> {
  return invoke<string>('get_game_library_image', { gameId })
}

/**
 * 触发一次 GPU discardable 缓存回收（翻页时调用）
 * 短暂降低 WebView2 内存目标等级，丢弃旧页纹理，降低 GPU 内存峰值
 */
export async function flushGpuCache(): Promise<void> {
  return invoke<void>('flush_gpu_cache')
}

/**
 * 从配置文件加载游戏数据
 */
export async function loadGamesConfigFromFile(): Promise<GameConfigData[]> {
  return invoke<GameConfigData[]>('load_games_config_from_file')
}
