/**
 * error-message.service.ts - 错误提示转换服务
 * 将后端错误转换为用户友好的中文提示
 */

/**
 * 错误类型映射表
 * 将常见错误关键词转换为友好提示
 */
const ERROR_MESSAGES: Record<string, string> = {
  // 文件操作错误
  'FileError': '文件操作失败，请检查文件权限或路径是否正确',
  'ConfigError': '配置解析失败，请检查配置格式',
  'PathError': '路径验证失败，请使用有效的路径',
  'SteamError': 'Steam操作失败，请检查Steam是否正常运行',
  'ParamError': '参数错误，请检查输入内容',
  
  // 常见具体错误
  '无法找到Steam安装路径': '请前往全局设置手动配置Steam路径',
  'Steam路径不存在': 'Steam路径无效，请重新配置',
  '未找到steam.exe': 'Steam未安装或路径不正确',
  '写入Lua文件失败': '无法写入配置文件，请检查权限',
  '创建目录失败': '无法创建目录，请检查磁盘空间和权限',
  '无法读取清单文件': '清单文件格式不正确或已损坏',
  '解压失败': '压缩包解压失败，请检查文件是否完整',
  '扫描失败': '文件扫描失败，请检查路径是否正确',
}

/**
 * 转换错误消息为友好提示
 * @param error - 原始错误消息
 * @returns 转换后的友好提示文本
 */
export function toFriendlyMessage(error: string): string {
  // 检查是否有直接匹配
  if (ERROR_MESSAGES[error]) {
    return ERROR_MESSAGES[error]
  }
  
  // 检查是否包含关键词
  for (const [key, msg] of Object.entries(ERROR_MESSAGES)) {
    if (error.includes(key)) {
      return msg
    }
  }
  
  // 返回原始错误（但添加友好前缀）
  return `操作失败: ${error}`
}

/**
 * 显示友好的错误提示
 * @param error - 错误对象，可以是字符串、Error对象或其他类型
 */
export function showError(error: unknown): void {
  // 根据错误类型提取消息
  const message = typeof error === 'string' 
    ? toFriendlyMessage(error)
    : error instanceof Error 
      ? toFriendlyMessage(error.message)
      : '操作失败，请稍后重试'
  
  // 使用浏览器原生 alert 显示提示
  alert(message)
}