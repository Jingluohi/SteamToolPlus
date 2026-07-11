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
 * - 删除 Windows 文件系统禁止字符和常见危险字符
 * - 保留中英文、数字、空格及常用连接符
 * - 连续非法字符替换为单个空格
 * - 结果为空时使用 "game" 兜底，避免创建空目录名
 * @param name - 原始游戏名称
 * @returns 处理后的文件夹名称
 */
export function sanitizeGameFolderName(name: string): string {
  const sanitized = name
    .replace(/[<>:"/\\|?*!']/g, '')
    .replace(/[^\u4e00-\u9fa5a-zA-Z0-9\s\-_]+/g, ' ')
    .trim()

  return sanitized || 'game'
}
