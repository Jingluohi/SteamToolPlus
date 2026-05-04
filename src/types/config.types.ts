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
  /** 主题模式：dark/light/auto */
  mode: 'dark' | 'light' | 'auto'
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
}
