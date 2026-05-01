/**
 * 扩展相关类型定义
 */

/**
 * 扩展清单
 */
export interface ExtensionManifest {
  /** 扩展唯一ID */
  id: string
  /** 扩展名称 */
  name: string
  /** 版本号 */
  version: string
  /** 作者 */
  author: string
  /** 描述 */
  description: string
  /** 入口文件 */
  main: string
  /** 图标文件 */
  icon?: string
  /** 权限声明 */
  permissions: string[]
  /** 兼容的程序版本 */
  compatibleVersions: string[]
  /** 扩展配置 */
  config?: Record<string, unknown>
}

/**
 * 扩展加载状态
 */
export type ExtensionLoadStatus = 'unloaded' | 'loading' | 'loaded' | 'failed' | 'disabled'

/**
 * 扩展信息
 */
export interface Extension {
  /** 扩展清单 */
  manifest: ExtensionManifest
  /** 扩展路径 */
  path: string
  /** 是否已启用 */
  enabled: boolean
  /** 安装时间 */
  installTime: string
  /** 最后更新时间 */
  updateTime: string
  /** 加载状态 */
  loadStatus: ExtensionLoadStatus
  /** 错误信息 */
  error?: string
}

/**
 * 扩展列表响应
 */
export interface ExtensionListResponse {
  /** 扩展列表 */
  extensions: Extension[]
  /** 总数 */
  total: number
}

/**
 * 安装扩展请求
 */
export interface InstallExtensionRequest {
  /** 扩展包路径 */
  packagePath: string
}

/**
 * 扩展权限
 */
export interface ExtensionPermission {
  /** 权限ID */
  id: string
  /** 权限名称 */
  name: string
  /** 权限描述 */
  description: string
  /** 是否危险权限 */
  dangerous: boolean
  /** 是否已授予 */
  granted: boolean
}

/**
 * 扩展API调用请求
 */
export interface ExtensionApiRequest {
  /** 扩展ID */
  extensionId: string
  /** API名称 */
  api: string
  /** 参数 */
  params: Record<string, unknown>
}

/**
 * 扩展API调用响应
 */
export interface ExtensionApiResponse {
  /** 是否成功 */
  success: boolean
  /** 返回数据 */
  data?: Record<string, unknown>
  /** 错误信息 */
  error?: string
}
