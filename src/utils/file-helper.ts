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
 * 安全化游戏文件夹名称（用于下载目录）
 * 规则：
 * - 删除 ! 和 ' 字符
 * - 连续的非英文字母数字空格字符替换为单个空格
 * - 保留英文字母和数字
 * @param name - 原始游戏名称
 * @returns 处理后的文件夹名称
 */
export function sanitizeGameFolderName(name: string): string {
  return name
    .replace(/[!']/g, '')
    .replace(/[^a-zA-Z0-9\s]+/g, ' ')
    .trim()
}
