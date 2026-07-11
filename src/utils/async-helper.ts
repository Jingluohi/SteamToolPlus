/**
 * async-helper.ts - 异步操作工具函数
 * 提供安全的异步调用包装，统一错误处理
 */

/**
 * 安全地执行异步操作，自动捕获错误并返回 null
 * @param fn - 要执行的异步函数
 * @returns 操作结果，失败时返回 null
 */
export async function safeAsync<T>(fn: () => Promise<T>): Promise<T | null> {
  try {
    return await fn()
  } catch {
    return null
  }
}

