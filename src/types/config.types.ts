/**
 * 配置相关类型定义
 */

/**
 * 应用程序配置
 */
export interface AppConfig {
  /** 窗口配置 */
  window: WindowConfig
  /** 主题配置 */
  theme: ThemeConfig
  /** 游戏目录配置 */
  gameDirs: GameDirConfig
  /** 启动配置 */
  launch: LaunchConfig
  /** OpenSteamTool内核配置 */
  opensteamtool: OpenSteamToolConfig
  /** 下载记录按钮配置 */
  recordButton: RecordButtonConfig
}

/**
 * 下载记录按钮配置
 * 无下载任务时右下角「下载记录」悬浮按钮的位置（距窗口右/下边缘的 px 距离），
 * 用户可自由拖动，位置持久化到 config.json。
 */
export interface RecordButtonConfig {
  /** 距离窗口右边缘的距离（px） */
  right: number
  /** 距离窗口底部边缘的距离（px） */
  bottom: number
}

/**
 * 窗口配置
 */
export interface WindowConfig {
  /** 窗口宽度 */
  width: number
  /** 窗口高度 */
  height: number
  /** 是否最大化 */
  maximized: boolean
  /** 是否全屏 */
  fullscreen: boolean
  /** 窗口位置X */
  posX?: number
  /** 窗口位置Y */
  posY?: number
}

/**
 * 主题配置
 */
export interface ThemeConfig {
  /**
   * 主题模式：
   * - dark: 深色（图片）
   * - light: 浅色（图片）
   * - black: 黑色（纯色）
   * - white: 白色（纯色）
   * - auto: 跟随系统（图片）
   * - auto-solid: 跟随系统（纯色）
   */
  mode: 'dark' | 'light' | 'black' | 'white' | 'auto' | 'auto-solid'
  /** 是否使用系统主题 */
  followSystem: boolean
  /** 自定义主题变量 */
  customVars?: Record<string, string>
}

/**
 * 游戏目录配置
 */
export interface GameDirConfig {
  /** Steam安装路径 */
  steamPath?: string
  /** 游戏默认下载路径 */
  defaultDownloadPath?: string
  /** 封面图存储路径 */
  coversPath: string
}

/**
 * 启动配置
 */
export interface LaunchConfig {
  /** 程序启动后最小化到托盘 */
  startMinimizedToTray: boolean
  /** 关闭程序后隐藏在托盘（默认开启） */
  hideToTrayOnClose: boolean
  /** 启动前检查游戏文件 */
  verifyBeforeLaunch: boolean
  /** 清单入库功能是否已完成首次初始化 */
  manifestImportInitialized: boolean
  /**
   * 是否使用进程内 Rust 新下载引擎。
   * false（默认）= 外部 ddv20.exe 下载（稳定）；true = 新引擎（不稳定，容易出错）
   */
  useNewEngine: boolean
  /** 分块并发数：单个下载任务同时从 CDN 拉取的数据块数量（仅新引擎生效，默认 32） */
  chunkConcurrency: number
  /** 全局并发上限：所有下载任务在途分块总数封顶（仅新引擎生效，默认 96） */
  globalConcurrency: number
  /** 并发 depot 数：同一游戏内同时下载的 depot 数量（仅新引擎生效，默认 2） */
  concurrentDepots: number
  /** 文件句柄缓存上限：写盘 LRU 保留的已打开文件句柄数（仅新引擎生效，默认 512） */
  fileHandleCache: number
}

/**
 * OpenSteamTool内核配置
 */
export interface OpenSteamToolConfig {
  /** 内核DLL是否已安装到Steam目录 */
  kernelInstalled: boolean
  /** 是否启用高级模式（写注册表等） */
  advancedMode: boolean
  /** 是否使用热加载入库（Steam运行时不重启，依赖OpenSteamTool文件监视自动加载） */
  hotReload: boolean
}

/**
 * 更新配置请求
 */
export interface UpdateConfigRequest {
  /** 窗口配置更新 */
  window?: WindowConfig
  /** 主题配置更新 */
  theme?: ThemeConfig
  /** 游戏目录配置更新 */
  gameDirs?: GameDirConfig
  /** 启动配置更新 */
  launch?: LaunchConfig
  /** OpenSteamTool内核配置更新 */
  opensteamtool?: OpenSteamToolConfig
  /** 下载记录按钮配置更新 */
  recordButton?: RecordButtonConfig
}
