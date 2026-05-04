/**
 * 游戏相关类型定义
 */

/**
 * 游戏数据结构
 */
export interface Game {
  /** 唯一ID */
  id: string
  /** 游戏名称 */
  name: string
  /** 游戏exe文件路径 */
  exePath: string
  /** 封面图路径 */
  coverPath?: string
  /** 启动参数 */
  launchParams: string
  /** 发行商 */
  publisher: string
  /** 发行日期 */
  releaseDate: string
  /** 标签/分类 */
  tags: string[]
  /** 是否安装 */
  isInstalled: boolean
  /** 是否收藏 */
  isFavorite: boolean
  /** 总游玩时长，单位秒 */
  totalPlayTime: number
  /** 最近游玩时间 */
  lastPlayTime?: string
  /** 添加时间 */
  createTime: string
}

/**
 * 游戏筛选条件
 */
export interface GameFilter {
  /** 搜索关键词 */
  search?: string
  /** 是否已安装 */
  installed?: boolean
  /** 是否收藏 */
  favorite?: boolean
  /** 标签筛选 */
  tags?: string[]
  /** 发行商筛选 */
  publisher?: string
}

/**
 * 游戏排序方式
 */
export type GameSortBy = 'name' | 'lastPlayed' | 'playTime' | 'installDate' | 'releaseDate'

/**
 * 游戏列表响应
 */
export interface GameListResponse {
  /** 游戏列表 */
  games: Game[]
  /** 总数 */
  total: number
}

/**
 * 添加游戏请求
 */
export interface AddGameRequest {
  /** 游戏名称 */
  name: string
  /** 游戏exe路径 */
  exePath: string
  /** 封面图路径（可选） */
  coverPath?: string
  /** 启动参数 */
  launchParams?: string
  /** 发行商 */
  publisher?: string
  /** 发行日期 */
  releaseDate?: string
  /** 标签 */
  tags?: string[]
}

/**
 * 更新游戏请求
 */
export interface UpdateGameRequest {
  /** 游戏ID */
  id: string
  /** 游戏名称 */
  name?: string
  /** 封面图路径 */
  coverPath?: string
  /** 启动参数 */
  launchParams?: string
  /** 发行商 */
  publisher?: string
  /** 发行日期 */
  releaseDate?: string
  /** 标签 */
  tags?: string[]
  /** 是否收藏 */
  isFavorite?: boolean
}

/**
 * 扫描游戏目录请求
 */
export interface ScanGamesRequest {
  /** 要扫描的目录路径 */
  directory: string
  /** 是否递归扫描子目录 */
  recursive: boolean
}

/**
 * 游戏启动结果
 */
export interface LaunchGameResult {
  /** 是否成功 */
  success: boolean
  /** 错误信息 */
  error?: string
  /** 进程ID */
  pid?: number
}

/**
 * 游戏视图类型
 */
export type GameViewType = 'grid' | 'list'

/**
 * 网盘下载链接配置
 */
export interface DownloadUrlConfig {
  /** 网盘来源标识: baidu=百度网盘, thunder=迅雷网盘, lanzou=蓝奏云 */
  source: string
  /** 下载链接URL */
  url: string
  /** 提取码/密码（可选） */
  pwd?: string | null
}

/**
 * 游戏标签配置（来自games_config.json）
 */
export interface GameTagConfig {
  /** 补丁类型：0=免Steam，1=局域网联机，2=Steam联机，3=D加密虚拟机，4=Epic联机 */
  patch_type: number
  /** 补丁源路径（可选，如果不提供则自动生成） */
  patch_source_path?: string
  /** 下载链接（兼容旧版单链接格式，新版请使用 download_urls） */
  download_url?: string
  /** 多网盘下载链接列表（新版格式，优先使用） */
  download_urls?: DownloadUrlConfig[]
}

/**
 * 获取补丁源路径
 * 如果配置中提供了路径则使用，否则根据补丁类型和游戏ID自动生成
 */
export function getPatchSourcePath(tag: GameTagConfig, gameId: string): string {
  // 如果配置中提供了路径，直接使用
  if (tag.patch_source_path && tag.patch_source_path.trim() !== '') {
    return tag.patch_source_path
  }

  // 根据补丁类型自动生成路径
  const patchTypeFolder: Record<number, string> = {
    0: '免_steam',
    1: '局域网联机',
    2: 'steam_联机',
    3: 'D_加密虚拟机',
    4: 'epic_联机'
  }

  const folder = patchTypeFolder[tag.patch_type] || '其他'
  return `Resources/crack/${folder}/${gameId}`
}

/**
 * 游戏配置数据（来自games_config.json）
 */
export interface GameConfigData {
  /** 游戏ID（Steam App ID） */
  game_id: string
  /** 英文游戏名称 */
  game_name: string
  /** 中文游戏名称 */
  chinese_name: string
  /** 是否可下载 */
  downloadable: boolean
  /** 补丁标签列表 */
  tags: GameTagConfig[]
  /** 是否已安装（运行时检测） */
  is_installed?: boolean
  /** 安装路径 */
  install_path?: string
  /** 最后运行时间 */
  last_played?: string
  /** 游戏时长（分钟） */
  play_time?: number
}

/**
 * 补丁类型映射
 */
export const PATCH_TYPE_MAP: Record<number, { name: string; description: string }> = {
  0: { name: '免Steam补丁', description: '无需Steam即可运行游戏' },
  1: { name: '局域网联机补丁', description: '支持局域网联机游玩' },
  2: { name: 'Steam联机补丁', description: '支持Steam平台联机' },
  3: { name: 'D加密虚拟机补丁', description: '用于D加密游戏的虚拟机补丁' },
  4: { name: 'Epic联机补丁', description: '支持Epic平台联机' }
}
