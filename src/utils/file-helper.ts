/**
 * file-helper.ts - 文件路径处理工具函数
 */

/**
 * 从完整路径中提取文件名
 * @param path - 文件路径
 * @returns 文件名
 */
export function getFileName(path: string): string {
  const parts = path.split(/[\\/]/)
  return parts[parts.length - 1] || path
}

/**
 * 从路径中提取文件夹名称（不含斜杠）
 * @param path - 路径
 * @returns 文件夹名称
 */
export function getFolderName(path: string): string {
  const parts = path.split(/[\\/]/).filter(Boolean)
  return parts[parts.length - 1] || path
}

/**
 * 安全化文件夹名称（移除非法字符）
 * @param name - 原始名称
 * @returns 安全的文件夹名称
 */
export function sanitizeFolderName(name: string): string {
  return name.replace(/[\\/:*?"<>|]/g, '_')
}
