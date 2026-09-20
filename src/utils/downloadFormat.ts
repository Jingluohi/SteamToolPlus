/**
 * downloadFormat.ts - 下载相关通用格式化工具
 *
 * 下载列表页、游戏详情页（下载进度组件）共用的速度 / 大小 / 剩余时间格式化，
 * 避免各页面重复实现同一套逻辑。
 */

/**
 * 字节/秒 → 速度文本（单位统一为十进制 MB/s：1 MB = 1000 × 1000 字节）。
 * ≥100 MB/s 取整，其余保留 2 位小数（0.01 起步，避免 <0.01 显示 0）。
 * 传入 0 或负数返回空串（调用方自行处理"获取进度中…"等占位）。
 */
export function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec <= 0) return ''
  const mbps = bytesPerSec / (1000 * 1000)
  return mbps >= 100 ? `${Math.round(mbps)} MB/s` : `${mbps.toFixed(2)} MB/s`
}

/**
 * 字节 → 可读大小文本（1 KB = 1024 B，用于已下载/总量的进度展示）。
 */
export function bytesText(bytes: number): string {
  if (!bytes || bytes <= 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let i = 0
  let v = bytes
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024
    i++
  }
  return `${v.toFixed(v >= 100 ? 0 : 1)} ${units[i]}`
}

/**
 * 剩余下载时间：按当前速度估算 (totalBytes - doneBytes) / speed。
 * speed <= 0（尚未捕获到采样）或剩余字节无意义时返回空串。
 */
export function formatRemaining(
  speed: number,
  doneBytes: number,
  totalBytes: number
): string {
  if (!speed || speed <= 0) return ''
  const remainBytes = totalBytes - doneBytes
  if (remainBytes <= 0) return ''
  const seconds = remainBytes / speed
  if (seconds < 60) return `剩余 ${Math.ceil(seconds)} 秒`
  if (seconds < 3600) return `剩余 ${Math.ceil(seconds / 60)} 分钟`
  const hours = Math.floor(seconds / 3600)
  const mins = Math.ceil((seconds % 3600) / 60)
  return `剩余 ${hours} 小时 ${mins} 分钟`
}