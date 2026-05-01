/**
 * 路径工具函数
 */

/**
 * 获取文件名
 * @param path 文件路径
 * @returns 文件名
 */
export function getFileName(path: string): string {
  const parts = path.split(/[\\/]/)
  return parts[parts.length - 1] || ''
}

/**
 * 获取文件扩展名
 * @param path 文件路径
 * @returns 扩展名（不含点）
 */
export function getFileExtension(path: string): string {
  const fileName = getFileName(path)
  const index = fileName.lastIndexOf('.')
  return index === -1 ? '' : fileName.slice(index + 1).toLowerCase()
}

/**
 * 获取父目录路径
 * @param path 文件路径
 * @returns 父目录路径
 */
export function getParentDir(path: string): string {
  const index = path.lastIndexOf('\\')
  if (index === -1) {
    const index2 = path.lastIndexOf('/')
    return index2 === -1 ? '' : path.slice(0, index2)
  }
  return path.slice(0, index)
}

/**
 * 拼接路径
 * @param paths 路径片段
 * @returns 拼接后的路径
 */
export function joinPath(...paths: string[]): string {
  return paths
    .map(p => p.replace(/[\\/]+$/, ''))
    .join('/')
    .replace(/\\/g, '/')
}

/**
 * 规范化路径
 * @param path 路径
 * @returns 规范化后的路径
 */
export function normalizePath(path: string): string {
  return path.replace(/\\/g, '/')
}

/**
 * 检查是否为绝对路径
 * @param path 路径
 * @returns 是否为绝对路径
 */
export function isAbsolutePath(path: string): boolean {
  return /^([a-zA-Z]:[\\/]|\\|\/)/.test(path)
}
