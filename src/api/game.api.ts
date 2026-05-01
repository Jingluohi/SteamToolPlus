/**
 * game.api.ts - 游戏库相关的API调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { Game } from '../types/game.types'

/**
 * 获取游戏列表
 */
export async function getGames(): Promise<Game[]> {
  return invoke<Game[]>('get_games')
}

/**
 * 获取单个游戏详情
 */
export async function getGame(id: string): Promise<Game | null> {
  return invoke<Game | null>('get_game', { id })
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
 * 获取游戏库背景图片路径
 * 从 resources/pic/库 目录加载
 */
export async function getGameLibraryImage(gameId: string): Promise<string> {
  return invoke<string>('get_game_library_image', { gameId })
}

/**
 * 从配置文件加载游戏数据
 */
export async function loadGamesConfigFromFile(): Promise<Game[]> {
  return invoke<Game[]>('load_games_config_from_file')
}
