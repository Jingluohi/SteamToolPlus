/**
 * error-handler.ts - 错误处理工具函数
 * 提供统一的错误处理和日志记录功能
 */

/**
 * 错误处理选项
 */
export interface ErrorHandlerOptions {
  /** 是否静默处理（不抛出错误） */
  silent?: boolean
  /** 自定义错误消息 */
  message?: string
  /** 是否记录日志 */
  log?: boolean
}

/**
 * 标准化的API响应
 */
export interface ApiResponse<T = unknown> {
  success: boolean
  data?: T
  error?: string
}

/**
 * 处理错误并返回标准化错误消息
 * @param error - 捕获的错误对象
 * @param context - 错误上下文信息
 * @param options - 处理选项
 * @returns 标准化的错误消息
 */
export function handleError(
  error: unknown,
  context: string,
  options: ErrorHandlerOptions = {}
): string {
  const { silent = false, message, log = true } = options

  // 提取错误消息
  const errorMessage = message || (error instanceof Error ? error.message : String(error))

  // 记录日志
  if (log) {
    console.error(`[${context}] ${errorMessage}`, error)
  }

  // 非静默模式下抛出错误
  if (!silent) {
    throw new Error(errorMessage)
  }

  return errorMessage
}

/**
 * 包装异步函数，统一处理错误
 * @param fn - 要包装的异步函数
 * @param context - 错误上下文
 * @param options - 错误处理选项
 * @returns 包装后的函数
 */
export function withErrorHandling<T extends (...args: unknown[]) => Promise<R>, R>(
  fn: T,
  context: string,
  options: ErrorHandlerOptions = {}
): (...args: Parameters<T>) => Promise<R> {
  return async (...args: Parameters<T>): Promise<R> => {
    try {
      return await fn(...args)
    } catch (error) {
      handleError(error, context, options)
      throw error
    }
  }
}

/**
 * 创建带加载状态和错误处理的store action包装器
 * @param loadingRef - 加载状态ref
 * @param errorRef - 错误状态ref
 * @returns action包装函数
 */
export function createStoreActionWrapper(
  loadingRef: { value: boolean },
  errorRef: { value: string | null }
) {
  return async function wrapAction<T>(
    action: () => Promise<T>,
    context: string
  ): Promise<T | undefined> {
    loadingRef.value = true
    errorRef.value = null

    try {
      return await action()
    } catch (err) {
      errorRef.value = err instanceof Error ? err.message : `${context}失败`
      console.error(`${context}失败:`, err)
      return undefined
    } finally {
      loadingRef.value = false
    }
  }
}

/**
 * 安全的JSON解析
 * @param jsonString - JSON字符串
 * @param defaultValue - 解析失败时的默认值
 * @returns 解析结果或默认值
 */
export function safeJsonParse<T>(jsonString: string, defaultValue: T): T {
  try {
    return JSON.parse(jsonString) as T
  } catch {
    return defaultValue
  }
}

/**
 * 断言函数，用于类型收窄
 * @param condition - 断言条件
 * @param message - 断言失败时的错误消息
 */
export function assert(condition: unknown, message: string): asserts condition {
  if (!condition) {
    throw new Error(message)
  }
}
