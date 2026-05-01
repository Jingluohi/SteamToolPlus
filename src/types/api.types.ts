/**
 * IPC接口相关类型定义
 */

/**
 * API响应包装
 */
export interface ApiResponse<T = unknown> {
  /** 是否成功 */
  success: boolean
  /** 返回数据 */
  data?: T
  /** 错误信息 */
  error?: string
}

/**
 * 分页请求参数
 */
export interface PaginationParams {
  /** 页码 */
  page: number
  /** 每页数量 */
  pageSize: number
}

/**
 * 分页响应数据
 */
export interface PaginationResponse<T> {
  /** 数据列表 */
  list: T[]
  /** 总数 */
  total: number
  /** 页码 */
  page: number
  /** 每页数量 */
  pageSize: number
  /** 总页数 */
  totalPages: number
}

/**
 * 排序参数
 */
export interface SortParams {
  /** 排序字段 */
  field: string
  /** 排序方向 */
  order: 'asc' | 'desc'
}

/**
 * 文件信息
 */
export interface FileInfo {
  /** 文件名称 */
  name: string
  /** 文件路径 */
  path: string
  /** 文件大小 */
  size: number
  /** 修改时间 */
  modifiedTime: string
  /** 是否目录 */
  isDirectory: boolean
}

/**
 * 进度信息
 */
export interface ProgressInfo {
  /** 当前进度 */
  current: number
  /** 总进度 */
  total: number
  /** 进度百分比 */
  percentage: number
  /** 状态信息 */
  message?: string
}
