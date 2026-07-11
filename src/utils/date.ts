/**
 * 日期工具函数
 */

/**
 * 格式化日期
 * @param date 日期对象或字符串
 * @param format 格式模板
 * @returns 格式化后的日期字符串
 */
export function formatDate(date: Date | string, format: string = 'YYYY-MM-DD'): string {
  const d = typeof date === 'string' ? new Date(date) : date

  // 防御无效日期，避免显示 NaN
  if (isNaN(d.getTime())) {
    return ''
  }

  const year = d.getFullYear()
  const month = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  const hours = String(d.getHours()).padStart(2, '0')
  const minutes = String(d.getMinutes()).padStart(2, '0')
  const seconds = String(d.getSeconds()).padStart(2, '0')

  return format
    .replace('YYYY', String(year))
    .replace('MM', month)
    .replace('DD', day)
    .replace('HH', hours)
    .replace('mm', minutes)
    .replace('ss', seconds)
}

/**
 * 格式化时长为中文可读格式
 * @param seconds 秒数
 * @returns 中文时长字符串（如：1小时30分）
 */
export function formatDurationCN(seconds: number): string {
  // 防御非法输入
  if (!isFinite(seconds) || seconds <= 0) return '0秒'

  const totalSeconds = Math.floor(seconds)
  if (totalSeconds < 60) return `${totalSeconds}秒`

  const hours = Math.floor(totalSeconds / 3600)
  const mins = Math.floor((totalSeconds % 3600) / 60)
  const secs = totalSeconds % 60

  if (hours > 0) {
    return mins > 0 ? `${hours}小时${mins}分` : `${hours}小时`
  }
  return secs > 0 ? `${mins}分${secs}秒` : `${mins}分`
}
