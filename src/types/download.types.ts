/**
 * 下载模块类型定义
 * 用于游戏本体下载功能
 */

/**
 * 通用下载任务信息
 */
export interface DownloadTask {
  /** 任务ID */
  id: string
  /** 下载URL */
  url: string
  /** 保存路径 */
  savePath: string
  /** 任务状态 */
  status: 'pending' | 'downloading' | 'paused' | 'completed' | 'error'
  /** 下载进度（0-100） */
  progress: number
}

/**
 * 单个depot的下载进度信息（前端列表展示）
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
  /** depot 说明文字：下载中的当前文件 / 完成提示 / 失败时的错误日志 */
  message?: string
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
 * 引擎单个 depot 的实时进度（Rust engine::progress::DepotProgress 镜像）
 * 注意：doneBytes / totalBytes / percent 是「整场下载」的全局字节汇总
 * （引擎按全部活跃 depot 的字节总数计算），depotId / currentFile 仅标识
 * 当前正在处理哪个 depot 的哪个文件。整体进度条直接用 percent 即可。
 */
export interface EngineDepotProgress {
  /** Steam depot 编号 */
  depotId: number
  /** 全局已下载字节数 */
  doneBytes: number
  /** 全局总字节数 */
  totalBytes: number
  /** 完成百分比 0.0 ~ 100.0（全局加权） */
  percent: number
  /** 当前正在下载的文件名（resume 跳过补发的 100% 事件为空串） */
  currentFile: string
  /** depot 状态：downloading / completed / error（前端据此切换徽章与配色） */
  status: 'downloading' | 'completed' | 'error'
  /** 人类可读说明：下载中的当前文件 / 完成提示 / 失败时的错误日志 */
  message: string
}

/**
 * 引擎整体运行状态（Rust engine::progress::EngineStatus 镜像）
 * 供页面刷新/进入时经 get_download_status 快照兜底拉取
 */
export interface EngineStatus {
  /** 游戏 ID（对应 AppID） */
  gameId: string
  /** 各 depot 进度列表 */
  depots: EngineDepotProgress[]
  /** 引擎是否仍在运行 */
  running: boolean
}

/**
 * 进度推送事件 payload（event: depot-download-progress）
 */
export interface DownloadProgressEvent {
  /** 游戏 ID（对应 AppID），支持多游戏并行互不干扰 */
  gameId: string
  /** 引擎单条进度 */
  progress: EngineDepotProgress
}

/**
 * 下载结束事件 payload（event: depot-download-finished）
 * 成功（全部 depot 落盘 + SHA-1 校验通过）时 success=true，
 * 取消/失败时 success=false，前端据此收尾展示
 */
export interface DownloadFinishedEvent {
  /** 游戏 ID（对应 AppID） */
  gameId: string
  /** 是否全部 depot 校验通过 */
  success: boolean
  /** 结束原因描述 */
  message: string
}

/**
 * 下载结果
 */
export interface DownloadResult {
  /** 是否成功 */
  success: boolean
  /** 消息 */
  message: string
  /** 外部进程 ID（引擎模式恒为 null，仅保留兼容旧调用方） */
  processId?: number
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
