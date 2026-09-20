/**
 * extractPlay.api.ts - 解压即玩相关的API调用
 * 负责从 Gitee 仓库实时拉取解压即玩（夸克网盘二维码）图片
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 解压即玩图片查询结果
 */
export interface ExtractPlayImageInfo {
  /** 该游戏是否已有解压即玩图片 */
  exists: boolean
  /** 本地缓存图片路径（exists 为 true 时提供） */
  path: string | null
}

/**
 * 获取指定游戏的解压即玩图片
 * 后端会优先复用本会话已下载的本地缓存，否则实时从 Gitee 下载。
 * @param gameId 游戏ID
 */
export async function getExtractPlayImage(gameId: string): Promise<ExtractPlayImageInfo> {
  return invoke<ExtractPlayImageInfo>('get_extract_play_image', { gameId })
}
