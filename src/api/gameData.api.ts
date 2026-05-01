/**
 * 游戏数据管理API
 * 管理已下载和导入的游戏数据
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 游戏数据接口
 */
export interface GameData {
  /** 游戏ID */
  game_id: string
  /** 游戏名称（英文） */
  game_name: string
  /** 游戏中文名 */
  chinese_name: string
  /** 游戏类型：downloaded(下载的) 或 imported(导入的) */
  game_type: 'downloaded' | 'imported'
  /** 游戏安装路径 */
  install_path: string
  /** 游戏主程序exe路径 */
  exe_path: string
  /** 游戏存档路径（可选） */
  save_path?: string
  /** 游戏封面路径（可选） */
  cover_path?: string
  /** Steam游戏ID（可选） */
  steam_game_id?: string
  /** 是否收藏 */
  is_favorite: boolean
  /** 是否已安装（用于库页面显示控制） */
  is_installed: boolean
  /** 总游玩时长（分钟） */
  play_time: number
  /** 最后游玩日期（ISO格式） */
  last_played?: string
  /** 添加日期（ISO格式） */
  added_date: string
  /** 下载状态：idle, downloading, paused, completed, error */
  download_status: 'idle' | 'downloading' | 'paused' | 'completed' | 'error' | ''
  /** 下载进度（0-100） */
  download_progress: number
  /** 下载路径（仅下载的游戏） */
  download_path?: string
}

/**
 * 获取所有游戏数据
 */
export async function getAllGamesData(): Promise<GameData[]> {
  return invoke<GameData[]>('get_all_games_data')
}

/**
 * 获取单个游戏数据
 */
export async function getGameData(gameId: string): Promise<GameData | null> {
  return invoke<GameData | null>('get_game_data', { gameId })
}

/**
 * 添加或更新游戏数据
 */
export async function upsertGameData(game: GameData): Promise<void> {
  return invoke<void>('upsert_game_data', { game })
}

/**
 * 删除游戏数据
 */
export async function removeGameData(gameId: string): Promise<void> {
  return invoke<void>('remove_game_data', { gameId })
}

/**
 * 更新下载状态
 */
export async function updateGameDownloadStatus(
  gameId: string,
  status: string,
  progress: number
): Promise<void> {
  return invoke<void>('update_game_download_status', { gameId, status, progress })
}

/**
 * 更新游戏时长
 */
export async function updateGamePlayTime(
  gameId: string,
  additionalMinutes: number
): Promise<void> {
  return invoke<void>('update_game_play_time', { gameId, additionalMinutes })
}

/**
 * 检查游戏是否存在
 */
export async function checkGameExists(gameId: string): Promise<boolean> {
  return invoke<boolean>('check_game_exists', { gameId })
}

/**
 * 导入自定义游戏
 */
export async function importCustomGame(
  gameName: string,
  chineseName: string,
  installPath: string,
  exePath: string,
  savePath?: string,
  coverPath?: string,
  steamGameId?: string
): Promise<GameData> {
  return invoke<GameData>('import_custom_game', {
    gameName,
    chineseName,
    installPath,
    exePath,
    savePath,
    coverPath,
    steamGameId
  })
}

/**
 * 启动游戏并记录时间
 * 返回进程ID
 */
export async function launchGameWithTracking(gameId: string): Promise<number> {
  return invoke<number>('launch_game_with_tracking', { gameId })
}

/**
 * 关闭游戏进程
 */
export async function closeGameProcess(pid: number): Promise<void> {
  return invoke<void>('close_game_process', { pid })
}

/**
 * 检查游戏进程状态
 * 返回 [是否运行中, 当前会话已玩分钟数]
 */
export async function checkGameProcessStatus(
  gameId: string,
  startTimeSecs: number
): Promise<[boolean, number]> {
  return invoke<[boolean, number]>('check_game_process_status', { 
    gameId, 
    startTimeSecs 
  })
}

/**
 * 更新游戏数据（编辑功能）
 */
export async function updateGameData(
  gameId: string,
  data: {
    game_name?: string
    chinese_name?: string
    install_path?: string
    exe_path?: string
    save_path?: string
    cover_path?: string
    steam_game_id?: string
  }
): Promise<GameData> {
  return invoke<GameData>('update_game_data', {
    gameId,
    gameName: data.game_name,
    chineseName: data.chinese_name,
    installPath: data.install_path,
    exePath: data.exe_path,
    savePath: data.save_path,
    coverPath: data.cover_path,
    steamGameId: data.steam_game_id
  })
}

/**
 * 切换游戏收藏状态
 */
export async function toggleGameFavorite(gameId: string): Promise<GameData> {
  return invoke<GameData>('toggle_game_favorite', { gameId })
}

/**
 * 格式化游戏时长
 */
export function formatPlayTime(minutes: number): string {
  if (minutes === 0) return '0 分钟'
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours > 0) {
    return `${hours} 小时 ${mins} 分钟`
  }
  return `${mins} 分钟`
}

/**
 * 格式化日期
 */
export function formatDate(dateString?: string): string {
  if (!dateString) return '从未'
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}
