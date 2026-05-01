/**
 * 窗口相关类型定义
 */

/**
 * 窗口状态
 */
export interface WindowState {
  /** 窗口宽度 */
  width: number
  /** 窗口高度 */
  height: number
  /** 窗口位置X */
  x: number
  /** 窗口位置Y */
  y: number
  /** 是否最大化 */
  maximized: boolean
  /** 是否全屏 */
  fullscreen: boolean
}

/**
 * 窗口操作结果
 */
export interface WindowOperationResult {
  /** 是否成功 */
  success: boolean
  /** 错误信息 */
  error?: string
}

/**
 * 设置窗口大小请求
 */
export interface SetWindowSizeRequest {
  /** 宽度 */
  width: number
  /** 高度 */
  height: number
}

/**
 * 设置窗口位置请求
 */
export interface SetWindowPositionRequest {
  /** X坐标 */
  x: number
  /** Y坐标 */
  y: number
}

/**
 * 导航菜单项
 */
export interface NavMenuItem {
  /** 菜单ID */
  id: string
  /** 菜单名称 */
  name: string
  /** 菜单路径 */
  path: string
  /** 图标 */
  icon?: string
  /** 子菜单 */
  children?: NavMenuItem[]
}
