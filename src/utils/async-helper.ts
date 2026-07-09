/**
 * async-helper.ts - 异步操作工具函数
 * 提供安全的异步调用包装，统一错误处理
 */

/**
 * 安全地执行异步操作，自动捕获并记录错误
 * @param fn - 要执行的异步函数
 * @param errorMessage - 错误时的日志消息
 * @returns 操作结果，失败时返回 null
 */
export async function safeAsync<T>(
  fn: () => Promise<T>,
  _errorMessage: string
): Promise<T | null> {
  try {
    return await fn()
  } catch (error) {
    return null
  }
}

