/**
 * 背景图片类型定义
 */

/** 背景显示模式 */
export type BackgroundMode = 'single' | 'slideshow' | 'random'

/** 切换动画效果 */
export type TransitionEffect = 'fade' | 'slide' | 'zoom' | 'none'

/** 页面类型 */
export type PageType = 'library' | 'browse' | 'download' | 'patch' | 'settings' | 'about'

/** 背景文件配置 */
export interface BackgroundFile {
  /** 文件ID */
  id: string
  /** 文件名 */
  filename: string
  /** 文件路径 */
  path: string
  /** 添加时间 */
  addedTime: string
  /** 是否启用 */
  enabled: boolean
  /** 排序顺序 */
  order: number
}

/** 主题模式 */
export type ThemeMode = 'light' | 'dark'

/** 页面背景配置 */
export interface PageBackgroundConfig {
  /** 页面类型 */
  pageType: PageType
  /** 页面名称 */
  pageName: string
  /** 是否启用背景 */
  enabled: boolean
  /** 显示模式 */
  mode: BackgroundMode
  /** 当前使用的背景文件ID (兼容旧版本) */
  currentFileId: string | null
  /** 浅色模式背景文件ID列表 */
  lightFileIds: string[]
  /** 深色模式背景文件ID列表 */
  darkFileIds: string[]
  /** 轮播间隔 */
  interval: number
  /** 切换动画 */
  transitionEffect: TransitionEffect
  /** 该页面使用的背景文件ID列表 (兼容旧版本) */
  fileIds: string[]
  /** 背景模糊强度 */
  blurStrength: number
  /** 背景暗化程度 */
  darkness: number
}

/** 背景设置配置 */
export interface BackgroundConfig {
  /** 全局背景文件列表 */
  files: BackgroundFile[]
  /** 各页面背景配置 */
  pageConfigs: PageBackgroundConfig[]
  /** 默认配置（用于新页面） */
  defaultConfig: Omit<PageBackgroundConfig, 'pageType' | 'pageName'>
}

/** 页面配置映射 */
export const PAGE_CONFIG_MAP: Record<PageType, { name: string; icon: string }> = {
  library: { name: '游戏库', icon: '🎮' },
  browse: { name: '浏览', icon: '🔍' },
  download: { name: '下载', icon: '⬇️' },
  patch: { name: '补丁', icon: '🔧' },
  settings: { name: '设置', icon: '⚙️' },
  about: { name: '关于', icon: 'ℹ️' }
}

/** 默认页面背景配置 */
export const DEFAULT_PAGE_CONFIG: Omit<PageBackgroundConfig, 'pageType' | 'pageName'> = {
  enabled: true,
  mode: 'single',
  currentFileId: null,
  lightFileIds: [],
  darkFileIds: [],
  interval: 10000,
  transitionEffect: 'fade',
  fileIds: [],
  blurStrength: 0,
  darkness: 0.15
}

/** 默认背景配置 */
export const DEFAULT_BACKGROUND_CONFIG: BackgroundConfig = {
  files: [],
  pageConfigs: [
    { ...DEFAULT_PAGE_CONFIG, pageType: 'library', pageName: '游戏库' },
    { ...DEFAULT_PAGE_CONFIG, pageType: 'browse', pageName: '浏览' },
    { ...DEFAULT_PAGE_CONFIG, pageType: 'download', pageName: '下载' },
    { ...DEFAULT_PAGE_CONFIG, pageType: 'patch', pageName: '补丁' },
    { ...DEFAULT_PAGE_CONFIG, pageType: 'settings', pageName: '设置' },
    { ...DEFAULT_PAGE_CONFIG, pageType: 'about', pageName: '关于' }
  ],
  defaultConfig: DEFAULT_PAGE_CONFIG
}

/** 预设的切换间隔选项 */
export const INTERVAL_OPTIONS = [
  { label: '5秒', value: 5000 },
  { label: '10秒', value: 10000 },
  { label: '30秒', value: 30000 },
  { label: '1分钟', value: 60000 },
  { label: '5分钟', value: 300000 },
  { label: '10分钟', value: 600000 }
]

/** 预设的动画效果选项 */
export const TRANSITION_EFFECT_OPTIONS = [
  { label: '淡入淡出', value: 'fade' },
  { label: '滑动', value: 'slide' },
  { label: '缩放', value: 'zoom' },
  { label: '无动画', value: 'none' }
]

/** 预设的模糊强度选项 */
export const BLUR_STRENGTH_OPTIONS = [
  { label: '无', value: 0 },
  { label: '轻微', value: 5 },
  { label: '中等', value: 10 },
  { label: '较强', value: 20 },
  { label: '强', value: 40 }
]

/** 预设的暗化程度选项 */
export const DARKNESS_OPTIONS = [
  { label: '无', value: 0 },
  { label: '轻微', value: 0.2 },
  { label: '中等', value: 0.4 },
  { label: '较强', value: 0.6 },
  { label: '强', value: 0.8 }
]

/** 支持的图片文件扩展名 */
export const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'gif']
