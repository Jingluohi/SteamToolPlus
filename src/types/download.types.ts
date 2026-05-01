/**
 * 下载模块类型定义
 * 用于游戏本体下载功能
 */

/**
 * 单个depot的下载进度信息
 */
export interface DepotProgress {
  /** Depot ID */
  depotId: string
  /** 下载百分比 */
  percentage: number
  /** 已下载文件数 */
  downloadedFiles: number
  /** 总文件数 */
  totalFiles: number
  /** 状态 */
  status: 'pending' | 'downloading' | 'completed' | 'error'
}

/**
 * 整体下载进度信息
 */
export interface DownloadProgress {
  /** 总Depot数量 */
  totalDepots: number
  /** 已完成Depot数量 */
  completedDepots: number
  /** 总体百分比 */
  overallPercentage: number
  /** Depot列表 */
  depots: DepotProgress[]
  /** 是否完成 */
  isComplete: boolean
}

/**
 * 清单文件夹读取结果
 */
export interface ManifestFolderResult {
  /** JSON文件列表 */
  jsonFiles: string[]
  /** VDF文件列表 */
  vdfFiles: string[]
  /** Manifest文件列表 */
  manifestFiles: string[]
}

/**
 * 下载结果
 */
export interface DownloadResult {
  /** 是否成功 */
  success: boolean
  /** 消息 */
  message: string
}

/**
 * 进度文件信息
 */
export interface ProgressFileInfo {
  /** 文件名 */
  name: string
  /** 文件路径 */
  path: string
  /** 进度百分比 (0-100) */
  progress: number
  /** Depot ID */
  depot_id: string
}

/**
 * 目录项信息
 */
export interface DirEntry {
  /** 名称 */
  name: string
  /** 路径 */
  path: string
  /** 是否目录 */
  is_dir: boolean
}

/**
 * 批量游戏信息
 */
export interface BatchGame {
  /** 游戏ID */
  id: string
  /** 游戏名称 */
  name: string
  /** 路径 */
  path: string
  /** 是否选中 */
  selected: boolean
  /** 状态 */
  status: 'pending' | 'downloading' | 'completed' | 'error'
}

/**
 * 下载日志条目
 */
export interface DownloadLog {
  /** 时间 */
  time: string
  /** 消息 */
  message: string
  /** 类型 */
  type: 'info' | 'success' | 'error' | 'warning'
}

/**
 * 下载模式
 */
export type DownloadMode = 'single' | 'batch'
