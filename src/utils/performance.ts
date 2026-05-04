/**
 * 性能优化工具函数
 */

/**
 * 防抖函数
 * @param fn 要执行的函数
 * @param delay 延迟时间（毫秒）
 * @returns 防抖后的函数
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timer: ReturnType<typeof setTimeout> | null = null
  
  return function (this: unknown, ...args: Parameters<T>) {
    if (timer) {
      clearTimeout(timer)
    }
    timer = setTimeout(() => {
      fn.apply(this, args)
    }, delay)
  }
}

/**
 * 节流函数
 * @param fn 要执行的函数
 * @param limit 限制时间（毫秒）
 * @returns 节流后的函数
 */
export function throttle<T extends (...args: unknown[]) => unknown>(
  fn: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle = false
  
  return function (this: unknown, ...args: Parameters<T>) {
    if (!inThrottle) {
      fn.apply(this, args)
      inThrottle = true
      setTimeout(() => {
        inThrottle = false
      }, limit)
    }
  }
}

/**
 * 延迟执行
 * @param ms 延迟毫秒数
 * @returns Promise
 */
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

/**
 * 分批处理数组
 * @param array 数组
 * @param batchSize 批次大小
 * @param processor 处理函数
 */
export async function batchProcess<T>(
  array: T[],
  batchSize: number,
  processor: (item: T) => Promise<void>
): Promise<void> {
  for (let i = 0; i < array.length; i += batchSize) {
    const batch = array.slice(i, i + batchSize)
    await Promise.all(batch.map(processor))
  }
}

/**
 * 测量函数执行时间
 * @param fn 要测量的函数
 * @param label 标签
 */
export function measurePerformance<T>(fn: () => T, label: string): T {
  const start = performance.now()
  const result = fn()
  const end = performance.now()
  // 性能测量结果不输出到控制台
  return result
}

/**
 * 异步测量函数执行时间
 * @param fn 要测量的异步函数
 * @param label 标签
 */
export async function measureAsyncPerformance<T>(
  fn: () => Promise<T>,
  label: string
): Promise<T> {
  const start = performance.now()
  const result = await fn()
  const end = performance.now()
  // 性能测量结果不输出到控制台
  return result
}
